use super::{fixtures::TEST_NOTEBOOK, test_cases::TEST_CASES};
use crate::{
    operations::{char_count, error::*, transform_operation::*},
    protocols::operations::*,
};
use pretty_assertions::assert_eq;

/// Returns whether two operations can converge (after transformation). This means they will result
/// in the same notebook regardless of which operation gets applied first.
///
/// Note that operations with causal relationships cannot and don't need to converge, because they
/// will get executed in a fixed order by definition.
///
/// In theory, we prefer all non-causally related operations to converge. But in practice, not all
/// operations can, and we exclude non-converging operations from the test.
fn converge(operation1: &Operation, operation2: &Operation) -> bool {
    match operation1 {
        Operation::AddCells(operation1) => match operation2 {
            Operation::AddCells(operation2) => adds_converge(operation1, operation2),
            Operation::RemoveCells(operation2) => add_and_remove_converge(operation1, operation2),
            _ => true,
        },
        Operation::MergeCells(operation1) => match operation2 {
            Operation::MergeCells(operation2) => merges_converge(operation1, operation2),
            Operation::RemoveCells(operation2) => merge_and_remove_converge(operation1, operation2),
            Operation::UpdateCell(operation2) => merge_and_update_converge(operation1, operation2),
            _ => true,
        },
        Operation::MoveCells(operation1) => match operation2 {
            Operation::MoveCells(operation2) => moves_converge(operation1, operation2),
            _ => true,
        },
        Operation::RemoveCells(operation1) => match operation2 {
            Operation::AddCells(operation2) => add_and_remove_converge(operation2, operation1),
            Operation::MergeCells(operation2) => merge_and_remove_converge(operation2, operation1),
            _ => true,
        },
        Operation::ReplaceText(operation1) => match operation2 {
            Operation::ReplaceText(operation2) => replace_texts_converge(operation1, operation2),
            Operation::SplitCell(operation2) => {
                replace_text_and_split_converge(operation1, operation2)
            }
            Operation::UpdateCell(operation2) => {
                replace_text_and_update_cell_converge(operation1, operation2)
            }
            _ => true,
        },
        Operation::SplitCell(operation1) => match operation2 {
            Operation::ReplaceText(operation2) => {
                replace_text_and_split_converge(operation2, operation1)
            }
            Operation::SplitCell(operation2) => splits_converge(operation1, operation2),
            Operation::UpdateCell(operation2) => &operation1.cell_id != operation2.old_cell.id(),
            _ => true,
        },
        Operation::UpdateCell(operation1) => match operation2 {
            Operation::MergeCells(operation2) => merge_and_update_converge(operation2, operation1),
            Operation::ReplaceText(operation2) => {
                replace_text_and_update_cell_converge(operation2, operation1)
            }
            Operation::SplitCell(operation2) => operation1.old_cell.id() != &operation2.cell_id,
            Operation::UpdateCell(operation2) => {
                operation1.old_cell.id() != operation2.old_cell.id()
            }
            _ => true,
        },
        Operation::UpdateNotebookTimeRange(_) => {
            !matches!(operation2, Operation::UpdateNotebookTimeRange(_))
        }
        Operation::UpdateNotebookTitle(_) => {
            !matches!(operation2, Operation::UpdateNotebookTitle(_))
        }
        Operation::AddDataSource(operation1) => {
            if let Operation::AddDataSource(operation2) = operation2 {
                operation1.name != operation2.name
            } else {
                true
            }
        }
        Operation::UpdateDataSource(operation1) => match operation2 {
            Operation::UpdateDataSource(operation2) => operation1.name != operation2.name,
            Operation::RemoveDataSource(operation2) => operation1.name != operation2.name,
            _ => true,
        },
        Operation::RemoveDataSource(operation1) => match operation2 {
            Operation::UpdateDataSource(operation2) => operation1.name != operation2.name,
            Operation::RemoveDataSource(operation2) => operation1.name != operation2.name,
            _ => true,
        },
        Operation::AddLabel(operation1) => match operation2 {
            // It cannot converge with a AddLabel that has the same key.
            Operation::AddLabel(operation2) => operation1.label.key != operation2.label.key,
            // It cannot converge with a ReplaceLabel that is trying to change its key to the same key.
            Operation::ReplaceLabel(operation2) => operation1.label.key != operation2.new_label.key,
            _ => true,
        },
        Operation::ReplaceLabel(operation1) => match operation2 {
            // It cannot converge with a AddLabel that is trying to add the same key
            Operation::AddLabel(operation2) => operation1.new_label.key != operation2.label.key,
            // It cannot converge with an ReplaceLabel that is replacing the same label, or has the same new key.
            Operation::ReplaceLabel(operation2) => {
                let changed_same_label_key = operation1.old_label.key == operation2.old_label.key;
                let changed_to_same_label_key =
                    operation1.new_label.key == operation2.new_label.key;
                !changed_same_label_key && !changed_to_same_label_key
            }
            // It cannot converge with an RemoveLabel that is trying to remove the same label.
            Operation::RemoveLabel(operation2) => operation1.old_label.key != operation2.label.key,
            _ => true,
        },
        Operation::RemoveLabel(_operation1) => true,
    }
}

// If two operations are trying to add cells with the same IDs, they (currently) cannot converge.
fn adds_converge(add1: &AddCellsOperation, add2: &AddCellsOperation) -> bool {
    let add2_cell_ids: Vec<&String> = add2.cells.iter().map(|c| c.cell.id()).collect();
    !add1
        .cells
        .iter()
        .any(|c| add2_cell_ids.contains(&c.cell.id()))
}

// A remove operation cannot converge with an add cells operation if it tries to remove the cells
// added by that operation (which should be acceptable, because they would be considered to have a
// causal relationship).
fn add_and_remove_converge(add: &AddCellsOperation, remove: &RemoveCellsOperation) -> bool {
    let add_cell_ids: Vec<&String> = add.cells.iter().map(|c| c.cell.id()).collect();
    !remove
        .removed_cells
        .iter()
        .any(|c| add_cell_ids.contains(&c.cell.id()))
}

// If the source xor target of a merge is removed (not both!), and there are
// other cells to remove as well, we cannot converge because the transformation
// would require multiple operations to represent the result.
fn merge_and_remove_converge(merge: &MergeCellsOperation, remove: &RemoveCellsOperation) -> bool {
    if remove.removed_cells.len() < 2 {
        return true;
    }

    let source_removed = remove
        .removed_cells
        .iter()
        .any(|c| c.cell.id() == merge.source_cell.id());
    let target_removed = remove
        .removed_cells
        .iter()
        .any(|c| c.cell.id() == &merge.target_cell_id);
    !(source_removed ^ target_removed)
}

fn merges_converge(merge1: &MergeCellsOperation, merge2: &MergeCellsOperation) -> bool {
    // Two identical merges make each other obsolete, which converges by definition:
    if merge1 == merge2 {
        return true;
    }

    // Merges (currently) only converge if they don't involve the same cells:
    merge1.target_cell_id != merge2.target_cell_id
        && merge1.source_cell.id() != merge2.source_cell.id()
}

fn replace_text_and_split_converge(
    replace: &ReplaceTextOperation,
    split: &SplitCellOperation,
) -> bool {
    if replace.cell_id != split.cell_id {
        return true;
    }

    // Converge works as long as the split doesn't overlap with the replaced region:
    replace.offset + char_count(&replace.old_text) <= split.split_index
        || split.split_index
            + split
                .removed_text
                .as_ref()
                .map(char_count)
                .unwrap_or_default()
            >= replace.offset
}

fn replace_texts_converge(
    replace1: &ReplaceTextOperation,
    replace2: &ReplaceTextOperation,
) -> bool {
    // Two identical replacements make each other obsolete, which converges by definition:
    if replace1 == replace2 {
        return true;
    }

    // Replacements in different cells always converge:
    if replace1.cell_id != replace2.cell_id {
        return true;
    }

    // Convergence works as long as there's no overlap in the regions being replaced:
    replace1.offset + char_count(&replace1.old_text) < replace2.offset
        || replace1.offset > replace2.offset + char_count(&replace2.old_text)
}

#[test]
pub fn test_transform_operation() -> Result<(), Error> {
    let testable_operations: Vec<&Operation> = TEST_CASES
        .iter()
        .map(|test_case| &test_case.operation)
        .collect();

    let testable_permutations: Vec<(&Operation, &Operation)> = testable_operations
        .iter()
        .enumerate()
        .flat_map(|(i, &operation)| {
            testable_operations
                .iter()
                .skip(i) // Otherwise all permutations would be tested twice.
                .filter(move |&other| converge(operation, other))
                .map(move |other| (operation, *other))
        })
        .collect();

    // Verify the amount of permutations, to make sure we don't accidentally skip any:
    assert_eq!(testable_permutations.len(), 716);

    for (operation1, operation2) in testable_permutations.iter() {
        match (
            transform_operation(&*TEST_NOTEBOOK, operation1, operation2)?,
            transform_operation(&*TEST_NOTEBOOK, operation2, operation1)?,
        ) {
            (Some(transformed_operation1), Some(transformed_operation2)) => {
                // Regardless of order, applying both operations should result in the same notebook:
                assert_eq!(
                    TEST_NOTEBOOK
                        .apply_operation(operation1)?
                        .apply_operation(&transformed_operation2),
                    TEST_NOTEBOOK
                        .apply_operation(operation2)?
                        .apply_operation(&transformed_operation1),
                    "Transformed operations diverged!\n\
                    Operation 1: {:?}\n\
                    Was transformed to: {:?}\n\
                    Operation 2: {:?}\n\
                    Was transformed to: {:?}",
                    operation1,
                    transformed_operation1,
                    operation2,
                    transformed_operation2,
                );
            }
            (Some(transformed_operation1), None) => {
                assert_eq!(
                    TEST_NOTEBOOK.apply_operation(operation1)?,
                    TEST_NOTEBOOK
                        .apply_operation(operation2)?
                        .apply_operation(&transformed_operation1)?,
                    "Transformed operations diverged!\n\
                        Operation 1: {:?}\n\
                        Was transformed to: {:?}\n\
                        Operation 2 (dropped after transform): {:?}",
                    operation1,
                    transformed_operation1,
                    operation2,
                );
            }
            (None, Some(transformed_operation2)) => {
                assert_eq!(
                    TEST_NOTEBOOK
                        .apply_operation(operation1)?
                        .apply_operation(&transformed_operation2)?,
                    TEST_NOTEBOOK.apply_operation(operation2)?,
                    "Transformed operations diverged!\n\
                    Operation 1 (dropped after transform): {:?}\n\
                    Operation 2: {:?}\n\
                    Was transformed to: {:?}",
                    operation1,
                    operation2,
                    transformed_operation2,
                );
            }
            (None, None) => {
                // If both operations tried to do the exact same thing,
                // they'd both make the other obsolete:
                assert_eq!(
                    TEST_NOTEBOOK.apply_operation(operation1)?,
                    TEST_NOTEBOOK.apply_operation(operation2)?,
                    "Transformed operations diverged!\n\
                    Operation 1 (dropped after transform): {:?}\n\
                    Operation 2 (dropped after transform): {:?}",
                    operation1,
                    operation2,
                );
            }
        }
    }

    Ok(())
}
