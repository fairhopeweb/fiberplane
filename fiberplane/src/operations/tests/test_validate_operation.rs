use crate::{
    operations::{
        fixtures::{TEST_CASES, TEST_NOTEBOOK},
        validate_operation,
    },
    protocols::{core::*, operations::*, realtime::RejectReason},
    text_util::char_count,
};
use pretty_assertions::assert_eq;

#[test]
pub fn test_validate_test_cases() {
    // All operations from the test cases should be able to be validated without errors.
    for test_case in TEST_CASES.iter() {
        assert_eq!(
            validate_operation(&*TEST_NOTEBOOK, &test_case.operation),
            Ok(()),
            "Operation from test cases failed to validate: {:?}",
            test_case.operation
        );
    }
}

#[test]
pub fn test_invalid_replace_cells_operations() {
    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::ReplaceCells(ReplaceCellsOperation {
                new_cells: vec![CellWithIndex {
                    cell: Cell::Text(TextCell {
                        id: "out_of_range".to_owned(),
                        content: "Out of range".to_owned(),
                        ..Default::default()
                    }),
                    index: TEST_NOTEBOOK.cells.len() as u32 + 1
                }],
                ..Default::default()
            })
        ),
        Err(RejectReason::CellIndexOutOfBounds)
    );

    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::ReplaceCells(ReplaceCellsOperation {
                new_cells: vec![CellWithIndex {
                    cell: Cell::Text(TextCell {
                        id: "c1".to_owned(),
                        content: "Duplicate".to_owned(),
                        ..Default::default()
                    }),
                    index: 0
                }],
                ..Default::default()
            })
        ),
        Err(RejectReason::DuplicateCellId {
            cell_id: "c1".to_owned()
        })
    );

    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::ReplaceCells(ReplaceCellsOperation {
                new_cells: vec![
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
                ..Default::default()
            })
        ),
        Err(RejectReason::DuplicateCellId {
            cell_id: "duplicate".to_owned()
        })
    );

    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::ReplaceCells(ReplaceCellsOperation {
                new_cells: vec![
                    CellWithIndex {
                        cell: TEST_NOTEBOOK.cells[3].with_text(""),
                        index: 3,
                    },
                    CellWithIndex {
                        cell: Cell::Loki(LokiCell {
                            id: "s1".to_owned(),
                            content: "memstats_alloc_bytes".to_owned(),
                            read_only: None,
                        }),
                        index: 4,
                    },
                ],
                old_cells: vec![CellWithIndex {
                    // The cell content doesn't align with the offset:
                    cell: TEST_NOTEBOOK.cells[3].with_text("memstats_alloc_bytes"),
                    index: 3,
                }],
                split_offset: Some(4),
                merge_offset: None,
                ..Default::default()
            })
        ),
        Err(RejectReason::InconsistentState)
    );

    assert_eq!(
        validate_operation(
            &*TEST_NOTEBOOK,
            &Operation::ReplaceCells(ReplaceCellsOperation {
                new_cells: vec![
                    CellWithIndex {
                        cell: TEST_NOTEBOOK.cells[3].with_text(""),
                        index: 3,
                    },
                    CellWithIndex {
                        cell: Cell::Loki(LokiCell {
                            id: "s1".to_owned(),
                            content: "memstats_alloc_bytes".to_owned(),
                            read_only: None,
                        }),
                        index: 4,
                    },
                ],
                old_cells: vec![CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[3].with_text(""),
                    index: 3,
                }],
                // Invalid split offset:
                split_offset: TEST_NOTEBOOK.cells[3]
                    .text()
                    .map(char_count)
                    .map(|text_len| text_len + 1),
                merge_offset: None,
                ..Default::default()
            })
        ),
        Err(RejectReason::FailedPrecondition {
            message: "`split_offset` is outside of target cell's text length".to_owned(),
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
                field: None,
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
                field: None,
            })
        ),
        Err(RejectReason::InconsistentState)
    );
}
