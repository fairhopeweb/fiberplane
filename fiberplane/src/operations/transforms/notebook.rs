use super::TransformOperationState;
use crate::protocols::operations::*;

pub(crate) fn transform_update_notebook_time_range_operation(
    _: &dyn TransformOperationState,
    successor: &UpdateNotebookTimeRangeOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    if matches!(predecessor, Operation::UpdateNotebookTimeRange(_)) {
        // Cannot converge. Drop successor:
        None
    } else {
        Some(Operation::UpdateNotebookTimeRange(successor.clone()))
    }
}

pub(crate) fn transform_update_notebook_title_operation(
    _: &dyn TransformOperationState,
    successor: &UpdateNotebookTitleOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    if matches!(predecessor, Operation::UpdateNotebookTitle(_)) {
        // Cannot converge. Drop successor:
        None
    } else {
        Some(Operation::UpdateNotebookTitle(successor.clone()))
    }
}
