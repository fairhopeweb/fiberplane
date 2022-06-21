use super::is_annotation_included_in_formatting;
use crate::protocols::{formatting::Annotation, operations::ReplaceCellsOperation};

pub(crate) fn is_annotation_included_in_old_cell_formatting(
    annotation: &Annotation,
    offset: u32,
    operation: &ReplaceCellsOperation,
    cell_id: &str,
) -> bool {
    if let Some(old_cell) = operation
        .old_cells
        .iter()
        .find(|old_cell| old_cell.id() == cell_id)
    {
        old_cell
            .formatting()
            .map(|old_formatting| {
                is_annotation_included_in_formatting(annotation, offset, old_formatting)
            })
            .unwrap_or_default()
    } else {
        false
    }
}
