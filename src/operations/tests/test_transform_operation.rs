use super::fixtures::TEST_NOTEBOOK;
use super::test_cases::TEST_CASES;
use crate::operations::{error::*, transform_operation::*};
use crate::protocols::operations::*;
use pretty_assertions::assert_eq;

/// Returns whether two operations should converge to the same notebook (after transformation).
///
/// In theory, we prefer any two operations to converge. In practice, not all operations currently
/// can, and we exclude non-converging operations from the test.
fn converge(operation1: &Operation, operation2: &Operation) -> bool {
    match operation1 {
        Operation::AddCells(operation1) => {
            if let Operation::AddCells(operation2) = operation2 {
                // If two operations are trying to add cells with the same IDs,
                // they (currently) cannot converge:
                let operation2_cell_ids: Vec<&String> =
                    operation2.cells.iter().map(|c| c.cell.id()).collect();
                !operation1
                    .cells
                    .iter()
                    .any(|c| operation2_cell_ids.contains(&c.cell.id()))
            } else {
                true
            }
        }
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
            Operation::MergeCells(operation2) => merge_and_remove_converge(operation2, operation1),
            _ => true,
        },
        Operation::SplitCell(operation1) => match operation2 {
            Operation::SplitCell(operation2) => splits_converge(operation1, operation2),
            Operation::UpdateCell(operation2) => &operation1.cell_id != operation2.old_cell.id(),
            _ => true,
        },
        Operation::UpdateCell(operation1) => match operation2 {
            Operation::MergeCells(operation2) => merge_and_update_converge(operation2, operation1),
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
    }
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
        && &merge1.target_cell_id != merge2.source_cell.id()
        && merge1.source_cell.id() != &merge2.target_cell_id
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
    assert_eq!(testable_permutations.len(), 288);

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
                        .apply_operation(&transformed_operation1)
                );
            }
            (Some(transformed_operation1), None) => {
                assert_eq!(
                    TEST_NOTEBOOK.apply_operation(operation1)?,
                    TEST_NOTEBOOK
                        .apply_operation(operation2)?
                        .apply_operation(&transformed_operation1)?
                );
            }
            (None, Some(transformed_operation2)) => {
                assert_eq!(
                    TEST_NOTEBOOK
                        .apply_operation(operation1)?
                        .apply_operation(&transformed_operation2)?,
                    TEST_NOTEBOOK.apply_operation(operation2)?
                );
            }
            (None, None) => {
                // If both operations tried to do the exact same thing,
                // they'd both make the other obsolete:
                assert_eq!(
                    TEST_NOTEBOOK.apply_operation(operation1)?,
                    TEST_NOTEBOOK.apply_operation(operation2)?
                );
            }
        }
    }

    Ok(())
}
