use super::TransformOperationState;
use crate::protocols::operations::*;

pub(crate) fn transform_add_data_source_operation(
    _: &dyn TransformOperationState,
    successor: &AddDataSourceOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    // Only allow it if the predecessor was not adding a data-source with the
    // same name (updating or removing data-sources with the same name should
    // not be possible).
    match predecessor {
        Operation::AddDataSource(predecessor) => {
            if predecessor.name == successor.name {
                None
            } else {
                Some(Operation::AddDataSource(successor.clone()))
            }
        }
        _ => Some(Operation::AddDataSource(successor.clone())),
    }
}

pub(crate) fn transform_update_data_source_operation(
    _: &dyn TransformOperationState,
    successor: &UpdateDataSourceOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    // Only allow it if the predecessor was not updating or removing the same data-source
    match predecessor {
        Operation::UpdateDataSource(predecessor) => {
            if predecessor.name == successor.name {
                None
            } else {
                Some(Operation::UpdateDataSource(successor.clone()))
            }
        }
        Operation::RemoveDataSource(predecessor) => {
            if predecessor.name == successor.name {
                None
            } else {
                Some(Operation::UpdateDataSource(successor.clone()))
            }
        }
        _ => Some(Operation::UpdateDataSource(successor.clone())),
    }
}

pub(crate) fn transform_remove_data_source_operation(
    _: &dyn TransformOperationState,
    successor: &RemoveDataSourceOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    // Only allow it if the predecessor has not removed the same data-source
    match predecessor {
        Operation::RemoveDataSource(predecessor) => {
            if predecessor.name == successor.name {
                None
            } else {
                Some(Operation::RemoveDataSource(successor.clone()))
            }
        }
        _ => Some(Operation::RemoveDataSource(successor.clone())),
    }
}
