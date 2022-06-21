use super::TransformOperationState;
use crate::protocols::operations::*;

pub(crate) fn transform_add_label_operation(
    _: &dyn TransformOperationState,
    successor: &AddLabelOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    // Only allow it:
    // - if the predecessor was not adding a label with the same key.
    // - if the predecessor was not updating another label to the same key.
    match predecessor {
        Operation::AddLabel(predecessor) if predecessor.label.key == successor.label.key => None,
        Operation::ReplaceLabel(predecessor)
            if predecessor.new_label.key == successor.label.key =>
        {
            None
        }
        _ => Some(Operation::AddLabel(successor.clone())),
    }
}

pub(crate) fn transform_replace_label_operation(
    _: &dyn TransformOperationState,
    successor: &ReplaceLabelOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    match predecessor {
        // Only allow it:
        // - if the predecessor was not adding an label with the same key.
        // - if the predecessor was not updating the same label.
        // - if the predecessor was not updating an existing label to the same key.
        // - if the predecessor was not removing an label with the same key.
        Operation::AddLabel(predecessor) if predecessor.label.key == successor.new_label.key => {
            None
        }
        Operation::ReplaceLabel(predecessor) => {
            let changed_same_label_key = predecessor.old_label.key == successor.old_label.key;
            let changed_to_same_label_key = predecessor.new_label.key == successor.new_label.key;
            if changed_same_label_key || changed_to_same_label_key {
                None
            } else {
                Some(Operation::ReplaceLabel(successor.clone()))
            }
        }
        Operation::RemoveLabel(predecessor) if predecessor.label.key == successor.old_label.key => {
            None
        }
        _ => Some(Operation::ReplaceLabel(successor.clone())),
    }
}

pub(crate) fn transform_remove_label_operation(
    _: &dyn TransformOperationState,
    successor: &RemoveLabelOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    // Only allow it:
    // - if the predecessor has not updated the same label.
    // - if the predecessor has not removed the same label.
    match predecessor {
        Operation::ReplaceLabel(predecessor)
            if predecessor.old_label.key == successor.label.key =>
        {
            None
        }
        Operation::RemoveLabel(predecessor) if predecessor.label.key == successor.label.key => None,
        _ => Some(Operation::RemoveLabel(successor.clone())),
    }
}
