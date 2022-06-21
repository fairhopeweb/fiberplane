use crate::protocols::formatting::{Annotation, Formatting};

pub(crate) fn is_annotation_included_in_formatting(
    annotation: &Annotation,
    offset: u32,
    formatting: &Formatting,
) -> bool {
    formatting
        .iter()
        .any(|a| a.offset == offset && &a.annotation == annotation)
}
