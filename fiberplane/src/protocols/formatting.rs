use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

/// Formatting to be applied in order to turn plain-text into rich-text.
///
/// The vector consists of tuples, each containing a character offset and an
/// annotation. The vector must be sorted by offset (the order of annotations at
/// the same offset is undefined).
pub type Formatting = Vec<AnnotationWithOffset>;

/// Newtype representing `(offset, Annotation)` tuples.
///
/// Used inside the `Formatting` vector.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::formatting")]
#[serde(rename_all = "camelCase")]
pub struct AnnotationWithOffset {
    pub offset: u32,
    #[serde(flatten)]
    pub annotation: Annotation,
}

impl AnnotationWithOffset {
    pub fn new(offset: u32, annotation: Annotation) -> Self {
        Self { offset, annotation }
    }

    /// Translates the offset of the annotation with the given delta.
    pub fn translate(&self, delta: i64) -> Self {
        Self {
            offset: (self.offset as i64 + delta) as u32,
            annotation: self.annotation.clone(),
        }
    }
}

/// A rich-text annotation.
///
/// Annotations are typically found inside a `Formatting` vector.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::formatting")]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Annotation {
    StartBold,
    EndBold,
    StartCode,
    EndCode,
    StartHighlight,
    EndHighlight,
    StartItalics,
    EndItalics,
    #[serde(rename_all = "camelCase")]
    StartLink {
        url: String,
    },
    EndLink,
    Mention(Mention),
    StartStrikethrough,
    EndStrikethrough,
    StartUnderline,
    EndUnderline,
}

impl Annotation {
    /// Returns the opposite of an annotation for the purpose of toggling the
    /// formatting.
    ///
    /// Returns `None` if the annotation is not part of a pair, or if the
    /// formatting cannot be toggled without more information.
    pub fn toggle_opposite(&self) -> Option<Annotation> {
        match self {
            Annotation::StartBold => Some(Annotation::EndBold),
            Annotation::EndBold => Some(Annotation::StartBold),
            Annotation::StartCode => Some(Annotation::EndCode),
            Annotation::EndCode => Some(Annotation::StartCode),
            Annotation::StartHighlight => Some(Annotation::EndHighlight),
            Annotation::EndHighlight => Some(Annotation::StartHighlight),
            Annotation::StartItalics => Some(Annotation::EndItalics),
            Annotation::EndItalics => Some(Annotation::StartItalics),
            Annotation::StartLink { .. } => Some(Annotation::EndLink),
            Annotation::EndLink => None,
            Annotation::Mention(_) => None,
            Annotation::StartStrikethrough => Some(Annotation::EndStrikethrough),
            Annotation::EndStrikethrough => Some(Annotation::StartStrikethrough),
            Annotation::StartUnderline => Some(Annotation::EndUnderline),
            Annotation::EndUnderline => Some(Annotation::StartUnderline),
        }
    }
}

/// A struct that represents all the formatting that is active at any given
/// character offset.
#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::formatting")]
#[serde(rename_all = "camelCase")]
pub struct ActiveFormatting {
    pub bold: bool,
    pub code: bool,
    pub highlight: bool,
    pub italics: bool,
    pub link: Option<String>,
    pub mention: Option<Mention>,
    pub strikethrough: bool,
    pub underline: bool,
}

impl ActiveFormatting {
    /// Returns a list of annotations that should be inserted to activate
    /// this formatting compared to a reference formatting.
    pub fn annotations_for_toggled_formatting(&self, reference: &Self) -> Vec<Annotation> {
        let mut annotations = Vec::new();
        if self.bold != reference.bold {
            annotations.push(if self.bold {
                Annotation::StartBold
            } else {
                Annotation::EndBold
            });
        }
        if self.code != reference.code {
            annotations.push(if self.code {
                Annotation::StartCode
            } else {
                Annotation::EndCode
            });
        }
        if self.highlight != reference.highlight {
            annotations.push(if self.highlight {
                Annotation::StartHighlight
            } else {
                Annotation::EndHighlight
            });
        }
        if self.italics != reference.italics {
            annotations.push(if self.italics {
                Annotation::StartItalics
            } else {
                Annotation::EndItalics
            });
        }
        if self.link != reference.link {
            annotations.push(if let Some(url) = self.link.as_ref() {
                Annotation::StartLink { url: url.clone() }
            } else {
                Annotation::EndLink
            });
        }
        if self.mention != reference.mention {
            if let Some(mention) = self.mention.as_ref() {
                annotations.push(Annotation::Mention(mention.clone()))
            }
        }
        if self.strikethrough != reference.strikethrough {
            annotations.push(if self.strikethrough {
                Annotation::StartStrikethrough
            } else {
                Annotation::EndStrikethrough
            });
        }
        if self.underline != reference.underline {
            annotations.push(if self.underline {
                Annotation::StartUnderline
            } else {
                Annotation::EndUnderline
            });
        }
        annotations
    }

    /// Returns whether the given annotation is active in this struct.
    pub fn contains(&self, annotation: &Annotation) -> bool {
        match annotation {
            Annotation::StartBold => self.bold,
            Annotation::EndBold => !self.bold,
            Annotation::StartCode => self.code,
            Annotation::EndCode => !self.code,
            Annotation::StartHighlight => self.highlight,
            Annotation::EndHighlight => !self.highlight,
            Annotation::StartItalics => self.italics,
            Annotation::EndItalics => !self.italics,
            Annotation::StartLink { .. } => self.link.is_some(),
            Annotation::EndLink => self.link.is_none(),
            Annotation::Mention(_) => self.mention.is_some(),
            Annotation::StartStrikethrough => self.strikethrough,
            Annotation::EndStrikethrough => !self.strikethrough,
            Annotation::StartUnderline => self.underline,
            Annotation::EndUnderline => !self.underline,
        }
    }
}

/// Annotation for the mention of a user.
///
/// Mentions do not have a start and end offset. Instead, they occur at the
/// start offset only and are expected to run up to the end of the name of
/// the mentioned user. If however, for unforeseen reasons, the plain text
/// being annotated does not align with the name inside the mention, the
/// mention will stop at the first non-matching character. Mentions for
/// which the first character of the name does not align must be ignored in
/// their entirety.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::formatting")]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub name: String,
    pub user_id: String,
}

/// Finds the first index at which an annotation can be found for the given
/// offset, or the next existing offset in case the exact offset cannot be
/// found.
///
/// Returns the length of the range if no annotation for the offset can be
/// found.
pub fn first_annotation_index_for_offset(range: &[AnnotationWithOffset], offset: u32) -> usize {
    let mut index = annotation_insertion_index(range, offset);
    // Make sure we return the first in case of multiple hits:
    while index > 0 && range[index - 1].offset == offset {
        index -= 1;
    }

    index
}

#[test]
fn test_first_annotation_index_for_offset() {
    let formatting = vec![
        AnnotationWithOffset::new(30, Annotation::StartBold),
        AnnotationWithOffset::new(30, Annotation::StartItalics),
        AnnotationWithOffset::new(94, Annotation::EndBold),
        AnnotationWithOffset::new(94, Annotation::EndItalics),
    ];

    assert_eq!(first_annotation_index_for_offset(&formatting, 10), 0);
    assert_eq!(first_annotation_index_for_offset(&formatting, 30), 0);
    assert_eq!(first_annotation_index_for_offset(&formatting, 31), 2);
    assert_eq!(first_annotation_index_for_offset(&formatting, 94), 2);
    assert_eq!(first_annotation_index_for_offset(&formatting, 95), 4);
}

/// Finds the first index at which an annotation can be found for an offset
/// higher than the given offset.
///
/// Returns the length of the range if no annotations for higher offsets can be
/// found.
pub fn first_annotation_index_beyond_offset(range: &[AnnotationWithOffset], offset: u32) -> usize {
    let mut index = annotation_insertion_index(range, offset);
    // Make sure we step over any potential hits:
    while index < range.len() && range[index].offset == offset {
        index += 1;
    }

    index
}

#[test]
fn test_first_annotation_index_beyond_offset() {
    let formatting = vec![
        AnnotationWithOffset::new(30, Annotation::StartBold),
        AnnotationWithOffset::new(30, Annotation::StartItalics),
        AnnotationWithOffset::new(94, Annotation::EndBold),
        AnnotationWithOffset::new(94, Annotation::EndItalics),
    ];

    assert_eq!(first_annotation_index_beyond_offset(&formatting, 10), 0);
    assert_eq!(first_annotation_index_beyond_offset(&formatting, 30), 2);
    assert_eq!(first_annotation_index_beyond_offset(&formatting, 31), 2);
    assert_eq!(first_annotation_index_beyond_offset(&formatting, 94), 4);
    assert_eq!(first_annotation_index_beyond_offset(&formatting, 95), 4);
}

/// Finds the correct insertion index for an annotation at the given offset
/// inside of a range.
pub fn annotation_insertion_index(range: &[AnnotationWithOffset], offset: u32) -> usize {
    match range.binary_search_by(|annotation| annotation.offset.cmp(&offset)) {
        Ok(index) => index,
        Err(insertion_index) => insertion_index,
    }
}

/// Translates all offsets in a range of formatting annotations with the given
/// delta.
#[must_use]
pub fn translate(range: &[AnnotationWithOffset], delta: i64) -> Formatting {
    range
        .iter()
        .map(|annotation| annotation.translate(delta))
        .collect()
}
