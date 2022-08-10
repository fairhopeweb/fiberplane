use fiberplane::protocols::core::{DividerCell, TextCell};

use super::*;

#[test]
fn formatting_basic() {
    let content = "some normal, some bold, and some italicized text";
    let formatting = vec![
        AnnotationWithOffset {
            annotation: Annotation::StartBold,
            offset: 13,
        },
        AnnotationWithOffset {
            annotation: Annotation::EndBold,
            offset: 24,
        },
        AnnotationWithOffset {
            annotation: Annotation::StartItalics,
            offset: 24,
        },
        AnnotationWithOffset {
            annotation: Annotation::EndItalics,
            offset: 43,
        },
    ];
    let actual = format_content(content, Some(formatting));
    // alternative: "fmt.raw('some normal, ').bold('some bold, ').italics('and some italicized text')"
    assert_eq!(
        actual,
        "['some normal, ', fmt.bold(['some bold, ']), fmt.italics(['and some italicized']), ' text']"
    );
}

#[test]
fn formatting_nested() {
    let content = "some normal, some bold, and some bold italicized text";
    let formatting = vec![
        AnnotationWithOffset {
            annotation: Annotation::StartBold,
            offset: 13,
        },
        AnnotationWithOffset {
            annotation: Annotation::StartItalics,
            offset: 24,
        },
        AnnotationWithOffset {
            annotation: Annotation::EndItalics,
            offset: 48,
        },
        AnnotationWithOffset {
            annotation: Annotation::EndBold,
            offset: 48,
        },
    ];
    let actual = format_content(content, Some(formatting));
    assert_eq!(actual, "['some normal, ', fmt.bold(['some bold, ', fmt.italics(['and some bold italicized'])]), ' text']");
}

#[test]
fn format_link() {
    let content = "see here for more";
    let formatting = vec![
        AnnotationWithOffset {
            annotation: Annotation::StartLink {
                url: "https://example.com/more".to_string(),
            },
            offset: 4,
        },
        AnnotationWithOffset {
            annotation: Annotation::EndLink,
            offset: 8,
        },
    ];
    let actual = format_content(content, Some(formatting));
    assert_eq!(
        actual,
        "['see ', fmt.link(url='https://example.com/more', content=['here']), ' for more']"
    );
}

#[test]
fn format_unclosed() {
    let content = "some normal, some bold";
    let formatting = vec![AnnotationWithOffset {
        annotation: Annotation::StartBold,
        offset: 13,
    }];
    let actual = format_content(content, Some(formatting));
    assert_eq!(actual, "['some normal, ', fmt.bold(['some bold'])]");
}

#[test]
fn print_text_cell() {
    let mut writer = CodeWriter::new();
    print_cell(
        &mut writer,
        Cell::Text(TextCell {
            id: "c1".to_owned(),
            content: "I'm a text cell".to_owned(),
            formatting: Some(Formatting::default()),
            read_only: None,
        }),
    );
    assert_eq!(writer.to_string(), "c.text(\"I'm a text cell\"),\n");
}

#[test]
fn print_divider_cell() {
    let mut writer = CodeWriter::new();
    print_cell(
        &mut writer,
        Cell::Divider(DividerCell {
            id: "c2".to_owned(),
            read_only: None,
        }),
    );
    assert_eq!(writer.to_string(), "c.divider(),\n");
}
