use crate::{
    operations::{
        error::*,
        fixtures::{TEST_CASES, TEST_NOTEBOOK},
        invert_operation,
        transforms::convergence::*,
        transforms::*,
        validate_operation::{get_existing_cell_to_compare, validate_operation},
        ApplyOperationState,
    },
    protocols::{core::*, operations::*, realtime::RejectReason},
    text_util::{char_count, char_slice},
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
        Operation::MoveCells(operation1) => match operation2 {
            Operation::MoveCells(operation2) => moves_converge(operation1, operation2),
            Operation::ReplaceCells(operation2) => {
                move_and_replace_cells_converge(operation1, operation2)
            }
            _ => true,
        },
        Operation::ReplaceCells(operation1) => match operation2 {
            Operation::MoveCells(operation2) => {
                move_and_replace_cells_converge(operation2, operation1)
            }
            Operation::ReplaceCells(operation2) => replace_cells_converge(operation1, operation2),
            Operation::ReplaceText(operation2) => {
                replace_cells_and_replace_text_converge(operation1, operation2)
            }
            _ => true,
        },
        Operation::ReplaceText(operation1) => match operation2 {
            Operation::ReplaceCells(operation2) => {
                replace_cells_and_replace_text_converge(operation2, operation1)
            }
            Operation::ReplaceText(operation2) => replace_texts_converge(operation1, operation2),
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
    assert_eq!(testable_permutations.len(), 988);

    for (i, (operation1, operation2)) in testable_permutations.iter().enumerate() {
        let progress = format!(
            "Successful operations: {}/{} ({} untested)",
            i,
            testable_permutations.len(),
            testable_permutations.len() - i - 1
        );

        let transformed_operation1 = transform_operation(&*TEST_NOTEBOOK, operation1, operation2)?;
        let transformed_operation2 = transform_operation(&*TEST_NOTEBOOK, operation2, operation1)?;

        let notebook_after_operation1 = TEST_NOTEBOOK.apply_operation(operation1)?;
        let notebook_after_operation2 = TEST_NOTEBOOK.apply_operation(operation2)?;

        if let Some(transformed_operation) = transformed_operation1.as_ref() {
            assert_transformed_operation_properties(
                &notebook_after_operation2,
                operation1,
                operation2,
                transformed_operation,
                &progress,
            );
        }

        if let Some(transformed_operation) = transformed_operation2.as_ref() {
            assert_transformed_operation_properties(
                &notebook_after_operation1,
                operation2,
                operation1,
                transformed_operation,
                &progress,
            );
        }

        match (transformed_operation1, transformed_operation2) {
            (Some(transformed_operation1), Some(transformed_operation2)) => {
                // Regardless of order, applying both operations should result in the same notebook:
                assert_eq!(
                    notebook_after_operation1.apply_operation(&transformed_operation2),
                    notebook_after_operation2.apply_operation(&transformed_operation1),
                    "Transformed operations diverged!\n\
                    Operation 1: {:?}\n\
                    Was transformed to: {:?}\n\
                    Operation 2: {:?}\n\
                    Was transformed to: {:?}\n{}",
                    operation1,
                    transformed_operation1,
                    operation2,
                    transformed_operation2,
                    progress
                );
            }
            (Some(transformed_operation1), None) => {
                assert_eq!(
                    notebook_after_operation1,
                    notebook_after_operation2.apply_operation(&transformed_operation1)?,
                    "Transformed operations diverged!\n\
                        Operation 1: {:?}\n\
                        Was transformed to: {:?}\n\
                        Operation 2 (dropped after transform): {:?}\n{}",
                    operation1,
                    transformed_operation1,
                    operation2,
                    progress
                );
            }
            (None, Some(transformed_operation2)) => {
                assert_eq!(
                    notebook_after_operation1.apply_operation(&transformed_operation2)?,
                    notebook_after_operation2,
                    "Transformed operations diverged!\n\
                    Operation 1 (dropped after transform): {:?}\n\
                    Operation 2: {:?}\n\
                    Was transformed to: {:?}\n{}",
                    operation1,
                    operation2,
                    transformed_operation2,
                    progress
                );
            }
            (None, None) => {
                // If both operations tried to do the exact same thing,
                // they'd both make the other obsolete:
                assert_eq!(
                    notebook_after_operation1, notebook_after_operation2,
                    "Transformed operations diverged!\n\
                    Operation 1 (dropped after transform): {:?}\n\
                    Operation 2 (dropped after transform): {:?}\n{}",
                    operation1, operation2, progress
                );
            }
        }
    }

    Ok(())
}

fn assert_transformed_operation_properties(
    notebook: &Notebook,
    original_operation: &Operation,
    predecessor: &Operation,
    transformed_operation: &Operation,
    progress: &str,
) {
    let validation_result = validate_operation(notebook, transformed_operation);
    report_validation_error(
        validation_result,
        notebook,
        original_operation,
        predecessor,
        transformed_operation,
        progress,
    );

    let inverted_operation = invert_operation(transformed_operation);
    let inverted_inverted_operation = invert_operation(&inverted_operation);
    assert_eq!(
        transformed_operation,
        &inverted_inverted_operation,
        "Transformation resulted in non-revertible operation!\n\
        Operation: {:?}\n\
        Was transformed to: {:?}\n\
        After transformation with: {:?}\n\
        Inverted operation: {:?}\n\
        Non-matching double-inverted operation: {:?}\n{}",
        original_operation,
        transformed_operation,
        predecessor,
        inverted_operation,
        inverted_inverted_operation,
        progress
    );
}

fn report_validation_error(
    validation_result: Result<(), RejectReason>,
    notebook: &Notebook,
    original_operation: &Operation,
    predecessor: &Operation,
    transformed_operation: &Operation,
    progress: &str,
) {
    let validation_error = match validation_result {
        Ok(()) => return, // Great!
        Err(reject_reason) => reject_reason,
    };

    if validation_error == RejectReason::InconsistentState {
        match transformed_operation {
            Operation::ReplaceCells(op) => {
                for (i, old_cell) in op.old_cells.iter().enumerate() {
                    if let Some(actual_cell) = notebook.cell_with_index(old_cell.id()) {
                        if let Ok(Some(original_cell)) =
                            get_existing_cell_to_compare(&actual_cell, i, op)
                        {
                            assert_eq!(
                                old_cell,
                                &CellWithIndex {
                                    cell: original_cell,
                                    index: actual_cell.index,
                                },
                                "Transformation resulted in operation with inconsistent state!\n\
                                Operation: {:?}\n\
                                Was transformed to: {:?}\n\
                                After transformation with: {:?}\n{}",
                                original_operation,
                                transformed_operation,
                                predecessor,
                                progress
                            )
                        }
                    }
                }
            }
            Operation::ReplaceText(op) => {
                if let Some(cell) = TransformOperationState::cell(notebook, &op.cell_id) {
                    assert_eq!(
                        cell.text()
                            .map(|text| char_slice(
                                text,
                                op.offset as usize,
                                (op.offset + char_count(&op.old_text)) as usize
                            ))
                            .unwrap_or_default(),
                        &op.old_text,
                        "Transformation resulted in operation with inconsistent state!\n\
                        Operation: {:?}\n\
                        Was transformed to: {:?}\n\
                        After transformation with: {:?}\n{}",
                        original_operation,
                        transformed_operation,
                        predecessor,
                        progress
                    )
                }
            }
            _ => {}
        }
    }

    panic!(
        "Transformation resulted in invalid operation!\n\
            Operation: {:?}\n\
            Was transformed to: {:?}\n\
            After transformation with: {:?}\n\
            Validation error: {:?}\n{}",
        original_operation, transformed_operation, predecessor, validation_error, progress
    );
}
