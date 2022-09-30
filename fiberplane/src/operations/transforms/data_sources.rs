use super::TransformOperationState;
use crate::protocols::operations::*;

pub(crate) fn transform_set_selected_data_source_operation(
    _: &dyn TransformOperationState,
    successor: &SetSelectedDataSourceOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    match predecessor {
        // If the previous operation set a different data source for the same type,
        // they win because they got their change in first
        Operation::SetSelectedDataSource(predecessor)
            if predecessor.provider_type == successor.provider_type =>
        {
            None
        }
        _ => Some(Operation::SetSelectedDataSource(successor.clone())),
    }
}
