use crate::{
    operations::{
        fixtures::{TEST_CASES, TEST_NOTEBOOK},
        validate_operation,
    },
    protocols::{core::*, operations::*, realtime::RejectReason},
};
use pretty_assertions::assert_eq;

#[test]
pub fn test_validate_test_cases() -> Result<(), RejectReason> {
    // All operations from the test cases should be able to be validated without errors.
    for test_case in TEST_CASES.iter() {
        validate_operation(&*TEST_NOTEBOOK, &test_case.operation)?;
    }

    Ok(())
}

#[test]
pub fn test_invalid_add_cells_operations() {
    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::AddCells(AddCellsOperation {
                cells: vec![CellWithIndex {
                    cell: Cell::Text(TextCell {
                        id: "out_of_range".to_owned(),
                        content: "Out of range".to_owned(),
                        ..Default::default()
                    }),
                    index: TEST_NOTEBOOK.cells.len() as u32 + 1
                }],
                referencing_cells: None,
            })
        ),
        Err(RejectReason::CellIndexOutOfBounds)
    );

    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::AddCells(AddCellsOperation {
                cells: vec![CellWithIndex {
                    cell: Cell::Text(TextCell {
                        id: "c1".to_owned(),
                        content: "Duplicate".to_owned(),
                        ..Default::default()
                    }),
                    index: 0
                }],
                referencing_cells: None,
            })
        ),
        Err(RejectReason::DuplicateCellId {
            cell_id: "c1".to_owned()
        })
    );

    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::AddCells(AddCellsOperation {
                cells: vec![
                    CellWithIndex {
                        cell: Cell::Text(TextCell {
                            id: "duplicate".to_owned(),
                            content: "Duplicate 1".to_owned(),
                            ..Default::default()
                        }),
                        index: 1
                    },
                    CellWithIndex {
                        cell: Cell::Text(TextCell {
                            id: "duplicate".to_owned(),
                            content: "Duplicate 2".to_owned(),
                            ..Default::default()
                        }),
                        index: 2
                    }
                ],
                referencing_cells: None,
            })
        ),
        Err(RejectReason::DuplicateCellId {
            cell_id: "duplicate".to_owned()
        })
    );
}

#[test]
pub fn test_invalid_replace_text_operations() {
    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::ReplaceText(ReplaceTextOperation {
                cell_id: "not_existing".to_owned(),
                offset: 5,
                new_text: "replaced".to_owned(),
                new_formatting: Some(Vec::new()),
                old_text: "introductory".to_owned(),
                old_formatting: None,
            })
        ),
        Err(RejectReason::CellNotFound {
            cell_id: "not_existing".to_owned()
        })
    );

    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::ReplaceText(ReplaceTextOperation {
                cell_id: "c3".to_owned(),
                offset: 5,
                new_text: "replaced".to_owned(),
                new_formatting: Some(Vec::new()),
                old_text: "INTRODUCTORY".to_owned(),
                old_formatting: None,
            })
        ),
        Err(RejectReason::InconsistentState)
    );
}
