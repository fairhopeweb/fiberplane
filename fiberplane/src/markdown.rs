use crate::{
    protocols::formatting::{Annotation, AnnotationWithOffset, Formatting},
    text_util::{char_count, char_slice},
};
use lazy_static::lazy_static;
use regex::{Matches, Regex, RegexBuilder};

/// Characters that trigger Markdown formatting.
pub const MARKERS: &[char] = &['*', '_', '`', '~', ':'];

const START_BOLD_ANNOTATIONS: &[Annotation] = &[Annotation::StartBold];
const START_BOLD_AND_ITALICS_ANNOTATIONS: &[Annotation] =
    &[Annotation::StartBold, Annotation::StartItalics];
const START_CODE_ANNOTATIONS: &[Annotation] = &[Annotation::StartCode];
const START_HIGHLIGHT_ANNOTATIONS: &[Annotation] = &[Annotation::StartHighlight];
const START_ITALICS_ANNOTATIONS: &[Annotation] = &[Annotation::StartItalics];
const START_STRIKETHROUGH_ANNOTATIONS: &[Annotation] = &[Annotation::StartStrikethrough];

// TODO: Some of the punctuation categories are incomplete or omitted entirely,
//       which may be problematic for less common scripts.
const UNICODE_PUNCTUATION: &[char] = &[
    // https://www.compart.com/en/unicode/category/Pf
    '\u{00bb}', '\u{2019}', '\u{201d}', '\u{203a}', '\u{2e03}', '\u{2e05}', '\u{2e0a}', '\u{2e0d}',
    '\u{2e1d}', '\u{2e21}', // https://www.compart.com/en/unicode/category/Pi
    '\u{00ab}', '\u{2018}', '\u{201b}', '\u{201c}', '\u{201f}', '\u{2039}', '\u{2e02}', '\u{2e04}',
    '\u{2e09}', '\u{2e0c}', '\u{2e1c}', '\u{2e20}',
    // https://www.compart.com/en/unicode/category/Po (TODO, incomplete!)
    '\u{00a1}', '\u{00a7}', '\u{00b6}', '\u{00b7}', '\u{00bf}',
    // https://www.compart.com/en/unicode/category/Ps (TODO, incomplete!)
    '\u{201a}', '\u{201e}',
];

/// Finds URLs in the given string.
pub fn find_urls(text: &str) -> Matches<'static, '_> {
    lazy_static! {
        // https://stackoverflow.com/a/3809435
        // (note it's simplified from the StackOverflow answer based on our linting suggestions)
        static ref LINK_REGEX: Regex =
            RegexBuilder::new(r"https?://(www\.)?[\w#%+.:=@~-]{1,256}\.[\d()A-Za-z]{1,6}\b[\w#%&()+./:=?@~-]*")
            .size_limit(33554432)
            .build()
            .unwrap();
    }

    LINK_REGEX.find_iter(text)
}

/// Returns a formatting vector from parsing the given text with Markdown.
pub fn formatting_from_markdown(text: &str) -> Formatting {
    let mut formatting = Formatting::default();

    for link in find_urls(text) {
        // URLs may contain parenthesis, but it's common for users to put a URL
        // *inside* them. So if the matched URL ends with a closing parenthesis,
        // but we there's no opening parenthesis, we strip it off:
        let mut url = link.as_str();
        if url.ends_with(')') && !url.contains('(') {
            url = &url[0..url.len() - 1];
        }

        let start_index = link.start();
        let start_offset = char_count(&text[..start_index]);
        let end_index = link.end();
        let end_offset = start_offset + char_count(&text[start_index..end_index]);
        insert_annotations(
            &mut formatting,
            &[Annotation::StartLink {
                url: url.to_owned(),
            }],
            start_offset,
            end_offset,
        );
    }

    // See: https://spec.commonmark.org/0.29/#delimiter-run
    let mut current_delimiter_run: Option<(usize, char)> = None;
    let mut open_delimiter_run: Option<(usize, &str)> = None;

    let mut skip = false;
    // We use null characters ('\0') to represent both boundaries of the text.
    for (i, c) in text.chars().chain(['\0']).enumerate() {
        if skip {
            skip = false; // Resume processing on the next character.
        } else if let Some((delimiter_start_offset, delimiter_run_char)) = current_delimiter_run {
            if c != delimiter_run_char {
                let prev_char = if delimiter_start_offset > 0 {
                    text.chars()
                        .nth(delimiter_start_offset - 1)
                        .unwrap_or_default()
                } else {
                    '\0'
                };

                if open_delimiter_run.is_none() {
                    if is_left_flanking_delimiter_run(prev_char, c) {
                        // https://spec.commonmark.org/0.29/#can-open-emphasis
                        let can_open = delimiter_run_char == '*'
                            || !is_right_flanking_delimiter_run(prev_char, c)
                            || is_punctuation(prev_char);
                        if can_open {
                            let delimiter_run = char_slice(text, delimiter_start_offset, i);
                            open_delimiter_run = Some((delimiter_start_offset, delimiter_run));
                        }
                    }
                } else if is_right_flanking_delimiter_run(prev_char, c) {
                    if let Some((run_start_offset, run)) = open_delimiter_run {
                        if run == char_slice(text, delimiter_start_offset, i) {
                            // https://spec.commonmark.org/0.29/#can-close-emphasis
                            let can_close = delimiter_run_char == '*'
                                || !is_left_flanking_delimiter_run(prev_char, c)
                                || is_punctuation(c);
                            if can_close {
                                if let Some(annotations) = start_annotations_for_delimiter_run(run)
                                {
                                    insert_annotations(
                                        &mut formatting,
                                        annotations,
                                        run_start_offset as u32,
                                        i as u32,
                                    );
                                }

                                open_delimiter_run = None;
                            }
                        }
                    }
                }

                current_delimiter_run = None;
            }
        } else if MARKERS.contains(&c) {
            let start_new_run = if let Some((_, current_run)) = open_delimiter_run {
                if current_run == "`" || current_run == "::" {
                    // Code blocks and highlights are not processed internally,
                    // so we only accept a run that may close the open one:
                    current_run
                        .chars()
                        .next()
                        .map(|char| char == c)
                        .unwrap_or_default()
                } else {
                    true
                }
            } else {
                true
            };
            if start_new_run {
                current_delimiter_run = Some((i, c));
            }
        } else if c == '`' || c == '~' {
        } else if c == '\\' {
            skip = true;
        }
    }

    formatting.sort_by(|a, b| a.offset.cmp(&b.offset));
    formatting
}

fn insert_annotations(
    formatting: &mut Formatting,
    annotations: &[Annotation],
    start: u32,
    end: u32,
) {
    for annotation in annotations {
        let maybe_end_annotation = annotation.toggle_opposite();
        formatting.push(AnnotationWithOffset::new(start, annotation.clone()));
        if let Some(end_annotation) = maybe_end_annotation {
            formatting.push(AnnotationWithOffset::new(end, end_annotation));
        }
    }
}

pub fn start_annotations_for_delimiter_run(open_emphasis: &str) -> Option<&'static [Annotation]> {
    match open_emphasis {
        "`" => Some(START_CODE_ANNOTATIONS),
        "*" | "_" => Some(START_ITALICS_ANNOTATIONS),
        "~" => Some(START_STRIKETHROUGH_ANNOTATIONS),
        "**" | "__" => Some(START_BOLD_ANNOTATIONS),
        "***" | "___" => Some(START_BOLD_AND_ITALICS_ANNOTATIONS),
        "::" => Some(START_HIGHLIGHT_ANNOTATIONS),
        _ => None,
    }
}

// https://spec.commonmark.org/0.29/#ascii-punctuation-character
fn is_ascii_punctuation(c: char) -> bool {
    ('\u{0021}'..='\u{002f}').contains(&c)
        || ('\u{003a}'..='\u{0040}').contains(&c)
        || ('\u{005b}'..='\u{0060}').contains(&c)
        || ('\u{007b}'..='\u{007e}').contains(&c)
}

// https://spec.commonmark.org/0.29/#left-flanking-delimiter-run
pub fn is_left_flanking_delimiter_run(prev_char: char, next_char: char) -> bool {
    if next_char == '\0' || is_unicode_white_space(next_char) {
        return false;
    }

    if !is_punctuation(next_char) {
        return true;
    }

    prev_char == '\0' || is_unicode_white_space(prev_char) || is_punctuation(prev_char)
}

// https://spec.commonmark.org/0.29/#right-flanking-delimiter-run
pub fn is_right_flanking_delimiter_run(prev_char: char, next_char: char) -> bool {
    if prev_char == '\0' || is_unicode_white_space(prev_char) {
        return false;
    }

    if !is_punctuation(prev_char) {
        return true;
    }

    next_char == '\0' || is_unicode_white_space(next_char) || is_punctuation(next_char)
}

// https://spec.commonmark.org/0.29/#punctuation-character
pub fn is_punctuation(c: char) -> bool {
    is_ascii_punctuation(c) || UNICODE_PUNCTUATION.contains(&c)
}

// https://spec.commonmark.org/0.29/#unicode-whitespace-character
fn is_unicode_white_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\u{000b}' | '\r' | ' '
        // https://www.compart.com/en/unicode/category/Zs
        | '\u{00a0}' | '\u{1680}' | '\u{2000}' | '\u{2001}' | '\u{2002}' | '\u{2003}' | '\u{2004}'
        | '\u{2005}' | '\u{2006}' | '\u{2007}' | '\u{2008}' | '\u{2009}' | '\u{200a}' | '\u{202f}'
        | '\u{205f}' | '\u{3000}'
    )
}

#[cfg(test)]
mod tests {
    use super::formatting_from_markdown;
    use crate::protocols::formatting::{Annotation, AnnotationWithOffset};
    use pretty_assertions::assert_eq;

    struct TestCase {
        text: &'static str,
        expected_formatting: &'static [AnnotationWithOffset],
    }

    #[test]
    fn test_markdown_formatting() {
        let test_cases = &[
            TestCase {
                text: "here",
                expected_formatting: &[],
            },
            TestCase {
                text: "*italic*",
                expected_formatting: &[
                    AnnotationWithOffset {
                        annotation: Annotation::StartItalics,
                        offset: 0,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndItalics,
                        offset: 8,
                    },
                ],
            },
            TestCase {
                text: "**bold**",
                expected_formatting: &[
                    AnnotationWithOffset {
                        annotation: Annotation::StartBold,
                        offset: 0,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndBold,
                        offset: 8,
                    },
                ],
            },
            TestCase {
                text: "`code`",
                expected_formatting: &[
                    AnnotationWithOffset {
                        annotation: Annotation::StartCode,
                        offset: 0,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndCode,
                        offset: 6,
                    },
                ],
            },
            TestCase {
                text: "`code with *bold*`",
                expected_formatting: &[
                    AnnotationWithOffset {
                        annotation: Annotation::StartCode,
                        offset: 0,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndCode,
                        offset: 18,
                    },
                ],
            },
            TestCase {
                text: "***bold and italic***`",
                expected_formatting: &[
                    AnnotationWithOffset {
                        annotation: Annotation::StartBold,
                        offset: 0,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::StartItalics,
                        offset: 0,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndBold,
                        offset: 21,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndItalics,
                        offset: 21,
                    },
                ],
            },
            TestCase {
                text: "::highlight::",
                expected_formatting: &[
                    AnnotationWithOffset {
                        annotation: Annotation::StartHighlight,
                        offset: 0,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndHighlight,
                        offset: 13,
                    },
                ],
            },
        ];

        for test_case in test_cases {
            assert_eq!(
                formatting_from_markdown(test_case.text),
                test_case.expected_formatting,
                "Unexpected formatting for \"{}\"",
                test_case.text
            )
        }
    }
}
