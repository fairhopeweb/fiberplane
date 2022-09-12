use super::test_notebook::TEST_NOTEBOOK;
use crate::{
    operations::Notebook,
    protocols::{
        blobs::Blob,
        core::*,
        formatting::{Annotation, AnnotationWithOffset, Formatting},
        operations::*,
    },
    text_util::char_count,
};
use once_cell::sync::Lazy;

pub struct OperationTestCase {
    pub operation: Operation,
    pub expected_apply_operation_result: Notebook,
}

/// Test cases that are used for testing `apply_operation()` and `invert_operation()`
/// (as well as `transform_operation()`, once it's implemented).
pub static TEST_CASES: Lazy<Vec<OperationTestCase>> = Lazy::new(|| {
    let mut test_cases = Vec::new();

    create_add_cells_test_cases(&mut test_cases);
    create_merge_cells_test_cases(&mut test_cases);
    create_move_cells_test_cases(&mut test_cases);
    create_remove_cells_test_cases(&mut test_cases);
    create_replace_text_test_cases(&mut test_cases);
    create_replace_text_field_test_cases(&mut test_cases);
    create_split_cell_test_cases(&mut test_cases);
    create_split_and_merge_cell_test_cases(&mut test_cases);
    create_toggle_formatting_test_cases(&mut test_cases);
    create_update_cell_test_cases(&mut test_cases);
    create_update_notebook_time_range_test_cases(&mut test_cases);

    create_add_data_source_test_cases(&mut test_cases);
    create_update_data_source_test_cases(&mut test_cases);
    create_remove_data_source_test_cases(&mut test_cases);

    create_add_label_test_case(&mut test_cases);
    create_replace_label_test_case(&mut test_cases);
    create_remove_label_test_case(&mut test_cases);

    test_cases
});

fn create_add_cells_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let new_cell_1 = Cell::Text(TextCell {
        id: "n1".to_owned(),
        content: "New cell 1".to_owned(),
        formatting: None,
        read_only: None,
    });
    let new_cell_2 = Cell::Text(TextCell {
        id: "n2".to_owned(),
        content: "New cell 2".to_owned(),
        formatting: Some(Formatting::default()),
        read_only: Some(true),
    });

    // Test appending cells to the back:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![
                CellWithIndex {
                    cell: new_cell_1.clone(),
                    index: TEST_NOTEBOOK.cells.len() as u32,
                },
                CellWithIndex {
                    cell: new_cell_2.clone(),
                    index: TEST_NOTEBOOK.cells.len() as u32 + 1,
                },
            ],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.push(new_cell_1.clone());
            cells.push(new_cell_2.clone());
        }),
    });

    let new_cell_3 = Cell::Table(TableCell {
        id: "n3".to_owned(),
        source_ids: vec![],
        formatting: Some(Formatting::default()),
        title: "New cell 3".to_owned(),
        data: None,
        read_only: None,
    });

    // Test prepending cells to the front:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: new_cell_3.clone(),
                index: 0,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.insert(0, new_cell_3.clone());
        }),
    });

    let new_cell_4 = Cell::Heading(HeadingCell {
        id: "n4".to_owned(),
        heading_type: HeadingType::H3,
        content: "New heading 4".to_owned(),
        formatting: Some(Formatting::default()),
        read_only: None,
    });

    // Test inserting a cell somewhere in the middle:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: new_cell_4.clone(),
                index: 4,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.insert(4, new_cell_4.clone());
        }),
    });

    let new_cell_5 = Cell::Text(TextCell {
        id: "n5".to_owned(),
        content: "New cell 5".to_owned(),
        formatting: Some(Formatting::default()),
        read_only: None,
    });

    // Test inserting another at the same position to cover transforms:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: new_cell_5.clone(),
                index: 4,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.insert(4, new_cell_5.clone());
        }),
    });

    let new_cell_6 = Cell::Prometheus(PrometheusCell {
        id: "n6".to_owned(),
        content: "New cell 6".to_owned(),
        read_only: None,
    });

    // Test inserting a cell and atomatically creating a reference to it:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: new_cell_6.clone(),
                index: 0,
            }],
            new_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[6]
                    .with_source_ids(vec!["c6".to_owned(), "n6".to_owned()]),
                index: 7,
            }],
            old_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[6].clone(),
                index: 6,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.insert(0, new_cell_6);
            if let Cell::Table(c7) = &mut cells[7] {
                c7.source_ids.push("n6".to_owned());
            } else {
                panic!("Cell at index 6 was expected to be a table cell");
            }
        }),
    });
}

fn create_merge_cells_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[1].with_text(""),
                index: 1,
            }],
            old_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[1].with_text(""),
                    index: 1,
                },
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[2].with_text(""),
                    index: 2,
                },
            ],
            split_offset: Some(TEST_NOTEBOOK.cells[1].content().map(char_count).unwrap()),
            merge_offset: Some(0),
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[1] = cells[1].with_appended_rich_text(
                cells[2].content().unwrap(),
                cells[2].formatting().unwrap(),
            );
            cells.remove(2);
        }),
    });

    // Make sure the cells of this test case overlap with the ones from the first, to
    // be able to test transforms between them:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[2].with_text(""),
                index: 2,
            }],
            old_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[2].with_text(""),
                    index: 2,
                },
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[3].with_text(""),
                    index: 3,
                },
            ],
            new_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].with_source_ids(
                    TEST_NOTEBOOK.cells[8]
                        .source_ids()
                        .iter()
                        .filter(|&&id| id != "c4")
                        .map(|&id| id.to_owned())
                        .collect(),
                ),
                index: 7,
            }],
            old_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].clone(),
                index: 8,
            }],
            split_offset: Some(TEST_NOTEBOOK.cells[2].content().map(char_count).unwrap()),
            merge_offset: Some(0),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_appended_rich_text(
                cells[3].content().unwrap(),
                &cells[3].formatting().cloned().unwrap_or_default(),
            );
            cells.remove(3);

            // Update the referencing cell:
            cells[7] = cells[7].with_source_ids(
                cells[7]
                    .source_ids()
                    .iter()
                    .filter(|&&id| id != "c4")
                    .map(|&id| id.to_owned())
                    .collect(),
            );
        }),
    });

    // Test merging with "glue text", which typically results from inverting a split cell operation:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[2].with_rich_text(
                    "glue",
                    vec![
                        AnnotationWithOffset::new(0, Annotation::StartBold),
                        AnnotationWithOffset::new(4, Annotation::EndBold),
                    ],
                ),
                index: 2,
            }],
            old_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[2].with_text(""),
                    index: 2,
                },
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[3].with_text(""),
                    index: 3,
                },
            ],
            new_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].with_source_ids(
                    TEST_NOTEBOOK.cells[8]
                        .source_ids()
                        .iter()
                        .filter(|&&id| id != "c4")
                        .map(|&id| id.to_owned())
                        .collect(),
                ),
                index: 7,
            }],
            old_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].clone(),
                index: 8,
            }],
            split_offset: Some(TEST_NOTEBOOK.cells[2].content().map(char_count).unwrap()),
            merge_offset: Some(0),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_appended_rich_text(
                &format!("glue{}", cells[3].content().unwrap()),
                &[
                    AnnotationWithOffset::new(0, Annotation::StartBold),
                    AnnotationWithOffset::new(4, Annotation::EndBold),
                ],
            );
            cells.remove(3);

            // Update the referencing cell:
            cells[7] = cells[7].with_source_ids(
                cells[7]
                    .source_ids()
                    .iter()
                    .filter(|&&id| id != "c4")
                    .map(|&id| id.to_owned())
                    .collect(),
            );
        }),
    });

    // Another with glue text, that overlaps with the previous:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[1].with_rich_text(
                    "gluten",
                    vec![
                        AnnotationWithOffset::new(
                            0,
                            Annotation::StartLink {
                                url: "https://en.wikipedia.org/wiki/Gluten".to_owned(),
                            },
                        ),
                        AnnotationWithOffset::new(6, Annotation::EndLink),
                    ],
                ),
                index: 1,
            }],
            old_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[1].with_text(""),
                    index: 1,
                },
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[2].with_text(""),
                    index: 2,
                },
            ],
            split_offset: Some(TEST_NOTEBOOK.cells[1].content().map(char_count).unwrap()),
            merge_offset: Some(0),
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[1] = cells[1].with_appended_rich_text(
                &format!("gluten{}", cells[2].content().unwrap()),
                &[
                    AnnotationWithOffset::new(
                        0,
                        Annotation::StartLink {
                            url: "https://en.wikipedia.org/wiki/Gluten".to_owned(),
                        },
                    ),
                    AnnotationWithOffset::new(6, Annotation::EndLink),
                ],
            );
            cells.remove(2);
        }),
    });
}

fn create_move_cells_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    test_cases.push(OperationTestCase {
        operation: Operation::MoveCells(MoveCellsOperation {
            cell_ids: vec!["c2".to_owned()],
            from_index: 1,
            to_index: 2,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            let tmp = cells[1].clone();
            cells[1] = cells[2].clone();
            cells[2] = tmp;
        }),
    });

    // Another move, which overlaps with the previous one, to
    // be able to test transforms between them:
    test_cases.push(OperationTestCase {
        operation: Operation::MoveCells(MoveCellsOperation {
            cell_ids: vec!["c3".to_owned()],
            from_index: 2,
            to_index: 3,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            let tmp = cells[2].clone();
            cells[2] = cells[3].clone();
            cells[3] = tmp;
        }),
    });
}

fn create_remove_cells_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            old_cells: vec![TEST_NOTEBOOK.clone_cell_with_index_by_id("c4")],
            new_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].with_source_ids(vec!["c6".to_owned()]),
                index: 7,
            }],
            old_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].clone(),
                index: 8,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.remove(3);

            if let Cell::Graph(c9) = &mut cells[7] {
                c9.source_ids.remove(0);
                if let Some(data) = &mut c9.data {
                    data.remove("c4");
                } else {
                    panic!("Expected cell to have data");
                }
            } else {
                panic!("Expected cell to be a graph cell");
            }
        }),
    });

    // Make sure the cells of this test case overlap with the ones from the first, to
    // be able to test transforms between them:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            old_cells: vec![
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c4"),
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c5"),
            ],
            new_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].with_source_ids(vec!["c6".to_owned()]),
                index: 6,
            }],
            old_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].clone(),
                index: 8,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.remove(3);
            cells.remove(3);

            if let Cell::Graph(c9) = &mut cells[6] {
                c9.source_ids.remove(0);
                if let Some(data) = &mut c9.data {
                    data.remove("c4");
                } else {
                    panic!("Expected cell to have data");
                }
            } else {
                panic!("Expected cell to be a graph cell");
            }
        }),
    });

    // Test removing all the sources from an output cell:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            old_cells: vec![
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c4"),
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c5"),
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c6"),
            ],
            old_referencing_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[6].clone(),
                    index: 6,
                },
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[8].clone(),
                    index: 8,
                },
            ],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.remove(3);
            cells.remove(3);
            cells.remove(3);
            cells.remove(3);
            cells.remove(4);
        }),
    });

    // Update the output in a provider cell:
    let updated_provider_cell = Cell::Provider(ProviderCell {
        id: "c14".to_owned(),
        formatting: Some(Vec::new()),
        intent: "sentry;my-data-source,x-error-details".to_owned(),
        output: Some(vec![Cell::Text(TextCell {
            id: "c14/output".to_owned(),
            content: "A-OK".to_owned(),
            ..Default::default()
        })]),
        query_data: Some("application/x-www-form-urlencoded,trace_id=123".to_owned()),
        read_only: None,
        response: Some(
            Blob {
                data: "ok".into(),
                mime_type: "text/plain".to_owned(),
            }
            .into(),
        ),
        title: "".to_owned(),
    });
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            old_cells: vec![TEST_NOTEBOOK.clone_cell_with_index_by_id(updated_provider_cell.id())],
            new_cells: vec![CellWithIndex {
                cell: updated_provider_cell.clone(),
                index: 13,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK
            .with_updated_cells(|cells| cells[13] = updated_provider_cell),
    });
}

fn create_replace_text_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c3".to_owned(),
            field: None,
            offset: 5,
            new_text: "replaced".to_owned(),
            new_formatting: Some(vec![
                AnnotationWithOffset::new(0, Annotation::StartItalics),
                AnnotationWithOffset::new(8, Annotation::EndItalics),
            ]),
            old_text: "introductory".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_rich_text(
                "Some replaced text",
                vec![
                    AnnotationWithOffset::new(5, Annotation::StartItalics),
                    AnnotationWithOffset::new(13, Annotation::EndItalics),
                ],
            )
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c3".to_owned(),
            field: None,
            offset: 18,
            new_text: "nonsense".to_owned(),
            new_formatting: Some(vec![
                AnnotationWithOffset::new(0, Annotation::StartStrikethrough),
                AnnotationWithOffset::new(8, Annotation::EndStrikethrough),
            ]),
            old_text: "text".to_owned(),
            old_formatting: Some(Formatting::default()),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_rich_text(
                "Some introductory nonsense",
                vec![
                    AnnotationWithOffset::new(18, Annotation::StartStrikethrough),
                    AnnotationWithOffset::new(26, Annotation::EndStrikethrough),
                ],
            )
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c3".to_owned(),
            field: None,
            offset: 17,
            new_text: "_".to_owned(),
            new_formatting: None,
            old_text: " ".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_rich_text("Some introductory_text", Formatting::default())
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c3".to_owned(),
            field: None,
            offset: 5,
            new_text: "replacement".to_owned(),
            new_formatting: Some(vec![
                AnnotationWithOffset::new(0, Annotation::StartBold),
                AnnotationWithOffset::new(11, Annotation::EndBold),
            ]),
            old_text: "introductory".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_rich_text(
                "Some replacement text",
                vec![
                    AnnotationWithOffset::new(5, Annotation::StartBold),
                    AnnotationWithOffset::new(16, Annotation::EndBold),
                ],
            )
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c2".to_owned(),
            field: None,
            offset: 0,
            new_text: "Unl".to_owned(),
            new_formatting: None,
            old_text: "L".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[1] = cells[1].with_rich_text("Unlocked subtitle", Formatting::default())
        }),
    });

    // Test cases that overlap with split cell cases:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c4".to_owned(),
            field: None,
            offset: 4,
            new_text: "".to_owned(),
            new_formatting: None,
            old_text: "emstats_".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK
            .with_updated_cells(|cells| cells[3] = cells[3].with_content("go_malloc_bytes")),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c4".to_owned(),
            field: None,
            offset: 18,
            new_text: "count".to_owned(),
            new_formatting: None,
            old_text: "bytes".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[3] = cells[3].with_content("go_memstats_alloc_count")
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c4".to_owned(),
            field: None,
            offset: 0,
            new_text: "".to_owned(),
            new_formatting: None,
            old_text: "go_".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK
            .with_updated_cells(|cells| cells[3] = cells[3].with_content("memstats_alloc_bytes")),
    });

    // Test cases that apply to a cell with existing formatting:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c8".to_owned(),
            field: None,
            offset: 17,
            new_text: "s".to_owned(),
            new_formatting: None,
            old_text: "".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[7] = cells[7].with_rich_text(
                "No test *notebooks* would be complete without some **Markdown**.\n\
            \n\
            Right before our crown jewel: ***a locked, multi-sourced bar graph with a custom \
            time range***!",
                vec![
                    AnnotationWithOffset::new(8, Annotation::StartItalics),
                    AnnotationWithOffset::new(19, Annotation::EndItalics),
                    AnnotationWithOffset::new(51, Annotation::StartBold),
                    AnnotationWithOffset::new(63, Annotation::EndBold),
                    AnnotationWithOffset::new(96, Annotation::StartBold),
                    AnnotationWithOffset::new(96, Annotation::StartItalics),
                    AnnotationWithOffset::new(160, Annotation::EndBold),
                    AnnotationWithOffset::new(160, Annotation::EndItalics),
                ],
            )
        }),
    });

    // Another insertion at the exact same offset as the previous one:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c8".to_owned(),
            field: None,
            offset: 17,
            new_text: "ing".to_owned(),
            new_formatting: None,
            old_text: "".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[7] = cells[7].with_rich_text(
                "No test *notebooking* would be complete without some **Markdown**.\n\
            \n\
            Right before our crown jewel: ***a locked, multi-sourced bar graph with a custom \
            time range***!",
                vec![
                    AnnotationWithOffset::new(8, Annotation::StartItalics),
                    AnnotationWithOffset::new(21, Annotation::EndItalics),
                    AnnotationWithOffset::new(53, Annotation::StartBold),
                    AnnotationWithOffset::new(65, Annotation::EndBold),
                    AnnotationWithOffset::new(98, Annotation::StartBold),
                    AnnotationWithOffset::new(98, Annotation::StartItalics),
                    AnnotationWithOffset::new(162, Annotation::EndBold),
                    AnnotationWithOffset::new(162, Annotation::EndItalics),
                ],
            )
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c8".to_owned(),
            field: None,
            offset: 7,
            new_text: "".to_owned(),
            new_formatting: None,
            old_text: " *notebook*".to_owned(),
            old_formatting: Some(vec![
                AnnotationWithOffset::new(1, Annotation::StartItalics),
                AnnotationWithOffset::new(11, Annotation::EndItalics),
            ]),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[7] = cells[7].with_rich_text(
                "No test would be complete without some **Markdown**.\n\
            \n\
            Right before our crown jewel: ***a locked, multi-sourced bar graph with a custom \
            time range***!",
                vec![
                    AnnotationWithOffset::new(39, Annotation::StartBold),
                    AnnotationWithOffset::new(51, Annotation::EndBold),
                    AnnotationWithOffset::new(84, Annotation::StartBold),
                    AnnotationWithOffset::new(84, Annotation::StartItalics),
                    AnnotationWithOffset::new(148, Annotation::EndBold),
                    AnnotationWithOffset::new(148, Annotation::EndItalics),
                ],
            )
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            old_cells: vec![TEST_NOTEBOOK.clone_cell_with_index_by_id("c4")],
            new_cells: vec![
                CellWithIndex {
                    cell: Cell::Text(TextCell {
                        id: "n1".to_owned(),
                        content: "hello".to_owned(),
                        ..Default::default()
                    }),
                    index: 3,
                },
                CellWithIndex {
                    cell: Cell::Text(TextCell {
                        id: "n2".to_owned(),
                        content: "hello".to_owned(),
                        ..Default::default()
                    }),
                    index: 4,
                },
            ],
            old_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].clone(),
                index: 8,
            }],
            new_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].with_source_ids(vec!["c6".to_owned()]),
                index: 9,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[3] = Cell::Text(TextCell {
                id: "n1".to_owned(),
                content: "hello".to_owned(),
                ..Default::default()
            });
            cells.insert(
                4,
                Cell::Text(TextCell {
                    id: "n2".to_owned(),
                    content: "hello".to_owned(),
                    ..Default::default()
                }),
            );
            cells[9] = cells[9].with_source_ids(vec!["c6".to_owned()]);
        }),
    })
}

fn create_replace_text_field_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c14".to_owned(),
            field: Some("trace_id".to_owned()),
            offset: 0,
            new_text: "456".to_owned(),
            new_formatting: None,
            old_text: "123".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[13] = Cell::Provider(ProviderCell {
                id: "c14".to_owned(),
                formatting: Some(Vec::new()),
                intent: "sentry;my-data-source,x-error-details".to_owned(),
                output: None,
                query_data: Some("application/x-www-form-urlencoded,trace_id=456".to_owned()),
                read_only: None,
                response: None,
                title: "".to_owned(),
            })
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c14".to_owned(),
            field: Some("other_field".to_owned()),
            offset: 0,
            new_text: "test".to_owned(),
            new_formatting: None,
            old_text: "".to_owned(),
            old_formatting: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[13] = Cell::Provider(ProviderCell {
                id: "c14".to_owned(),
                formatting: Some(Vec::new()),
                intent: "sentry;my-data-source,x-error-details".to_owned(),
                output: None,
                query_data: Some(
                    "application/x-www-form-urlencoded,other_field=test&trace_id=123".to_owned(),
                ),
                read_only: None,
                response: None,
                title: "".to_owned(),
            })
        }),
    });
}

fn create_split_cell_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let split_cell1 = Cell::Prometheus(PrometheusCell {
        id: "s1".to_owned(),
        content: "memstats_alloc_bytes".to_owned(),
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[3].with_text(""),
                    index: 3,
                },
                CellWithIndex {
                    cell: split_cell1.clone(),
                    index: 4,
                },
            ],
            old_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[3].with_text("memstats_alloc_bytes"),
                index: 3,
            }],
            split_offset: Some(3),
            merge_offset: None,
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[3] = cells[3].with_content("go_");
            cells.insert(4, split_cell1);
        }),
    });

    let split_cell2 = Cell::Prometheus(PrometheusCell {
        id: "s2".to_owned(),
        content: "bytes".to_owned(),
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[3].with_text(""),
                    index: 3,
                },
                CellWithIndex {
                    cell: split_cell2.clone(),
                    index: 4,
                },
            ],
            old_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[3].with_text("alloc_bytes"),
                index: 3,
            }],
            split_offset: Some(12),
            merge_offset: None,
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[3] = cells[3].with_content("go_memstats_");
            cells.insert(4, split_cell2);
        }),
    });

    // Test adding a reference to the newly split-off cell:
    let split_cell3 = Cell::Prometheus(PrometheusCell {
        id: "s3".to_owned(),
        content: "bytes".to_owned(),
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[3].with_text(""),
                    index: 3,
                },
                CellWithIndex {
                    cell: split_cell3.clone(),
                    index: 4,
                },
            ],
            old_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[3].with_text("bytes"),
                index: 3,
            }],
            split_offset: Some(18),
            merge_offset: None,
            new_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].with_source_ids(vec![
                    "c4".to_owned(),
                    "c6".to_owned(),
                    "s3".to_owned(),
                ]),
                index: 9,
            }],
            old_referencing_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].clone(),
                index: 8,
            }],
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[3] = cells[3].with_content("go_memstats_alloc_");
            cells.insert(4, split_cell3);

            // Update the referencing cell:
            let mut c9_source_ids: Vec<String> = cells[9]
                .source_ids()
                .iter()
                .map(|&id| id.to_owned())
                .collect();
            assert_eq!(c9_source_ids.len(), 2);
            c9_source_ids.push("s3".to_owned());
            cells[9] = cells[9].with_source_ids(c9_source_ids);
        }),
    });

    // Splitting a list item results in two list items:
    let content_before_split = "No test *notebook* would be complete without some **Markdown**.";
    let removed_content = "\n\n";
    let content_after_split = "Right before our crown jewel: ***a locked, multi-sourced bar graph with a custom time range***!";
    let split_cell4 = Cell::ListItem(ListItemCell {
        id: "s4".to_owned(),
        content: content_after_split.to_owned(),
        formatting: Some(vec![
            AnnotationWithOffset::new(30, Annotation::StartBold),
            AnnotationWithOffset::new(30, Annotation::StartItalics),
            AnnotationWithOffset::new(94, Annotation::EndBold),
            AnnotationWithOffset::new(94, Annotation::EndItalics),
        ]),
        level: None,
        list_type: ListType::Unordered,
        read_only: None,
        start_number: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[7].with_text(""),
                    index: 7,
                },
                CellWithIndex {
                    cell: split_cell4.clone(),
                    index: 8,
                },
            ],
            old_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[7].with_rich_text(
                    &format!("{}{}", removed_content, content_after_split),
                    split_cell4
                        .formatting()
                        .map(|formatting| {
                            formatting
                                .iter()
                                .map(|annotation| {
                                    annotation.translate(removed_content.len() as i64)
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                ),
                index: 7,
            }],
            split_offset: Some(content_before_split.len() as u32),
            merge_offset: None,
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[7] = cells[7].with_rich_text(
                content_before_split,
                vec![
                    AnnotationWithOffset::new(8, Annotation::StartItalics),
                    AnnotationWithOffset::new(18, Annotation::EndItalics),
                    AnnotationWithOffset::new(50, Annotation::StartBold),
                    AnnotationWithOffset::new(62, Annotation::EndBold),
                ],
            );
            cells.insert(8, split_cell4);
        }),
    });
}

fn create_split_and_merge_cell_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            old_cells: vec![
                CellWithIndex {
                    cell: Cell::Text(TextCell {
                        id: "c3".to_owned(),
                        content: "introductory text".to_owned(),
                        formatting: Some(Formatting::default()),
                        read_only: None,
                    }),
                    index: 2,
                },
                CellWithIndex {
                    cell: Cell::Prometheus(PrometheusCell {
                        id: "c4".to_owned(),
                        content: "go_memstats".to_owned(),
                        read_only: None,
                    }),
                    index: 3,
                },
            ],
            new_cells: vec![CellWithIndex {
                cell: Cell::Text(TextCell {
                    id: "c3".to_owned(),
                    content: "".to_owned(),
                    formatting: Some(Formatting::default()),
                    read_only: None,
                }),
                index: 2,
            }],
            split_offset: Some(5),
            merge_offset: Some(11),
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_text("Some _alloc_bytes");
            cells.remove(3);
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            old_cells: vec![
                CellWithIndex {
                    cell: Cell::Heading(HeadingCell {
                        id: "c2".to_owned(),
                        heading_type: HeadingType::H2,
                        content: "subtitle".to_owned(),
                        formatting: Some(Formatting::default()),
                        read_only: Some(true),
                    }),
                    index: 1,
                },
                CellWithIndex {
                    cell: Cell::Text(TextCell {
                        id: "c3".to_owned(),
                        content: "Some ".to_owned(),
                        formatting: Some(Formatting::default()),
                        read_only: None,
                    }),
                    index: 2,
                },
            ],
            new_cells: vec![CellWithIndex {
                cell: Cell::Heading(HeadingCell {
                    id: "c2".to_owned(),
                    heading_type: HeadingType::H2,
                    content: "heading".to_owned(),
                    formatting: Some(Formatting::default()),
                    read_only: Some(true),
                }),
                index: 1,
            }],
            split_offset: Some(7),
            merge_offset: Some(5),
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[1] = cells[1].with_text("Locked headingintroductory text");
            cells.remove(2);
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            old_cells: vec![
                CellWithIndex {
                    cell: Cell::Text(TextCell {
                        id: "c3".to_owned(),
                        content: "text".to_owned(),
                        formatting: Some(Formatting::default()),
                        read_only: None,
                    }),
                    index: 2,
                },
                CellWithIndex {
                    cell: Cell::Prometheus(PrometheusCell {
                        id: "c4".to_owned(),
                        content: "go_memstats".to_owned(),
                        read_only: None,
                    }),
                    index: 3,
                },
            ],
            new_cells: vec![CellWithIndex {
                cell: Cell::Text(TextCell {
                    id: "c3".to_owned(),
                    content: "".to_owned(),
                    formatting: Some(Formatting::default()),
                    read_only: None,
                }),
                index: 2,
            }],
            split_offset: Some(18),
            merge_offset: Some(11),
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_text("Some introductory _alloc_bytes");
            cells.remove(3);
        }),
    });
}

fn create_toggle_formatting_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    // Strip boldness from two cells, only one of which has a bold section:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[11].with_rich_text(
                        "both",
                        vec![AnnotationWithOffset {
                            annotation: Annotation::EndItalics,
                            offset: 4,
                        }],
                    ),
                    index: 11,
                },
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[12].with_rich_text("🇳🇱 and", Formatting::default()),
                    index: 12,
                },
            ],
            old_cells: vec![
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[11].with_rich_text(
                        "both",
                        vec![
                            AnnotationWithOffset {
                                annotation: Annotation::StartBold,
                                offset: 0,
                            },
                            AnnotationWithOffset {
                                annotation: Annotation::EndItalics,
                                offset: 4,
                            },
                            AnnotationWithOffset {
                                annotation: Annotation::EndBold,
                                offset: 4,
                            },
                        ],
                    ),
                    index: 11,
                },
                CellWithIndex {
                    cell: TEST_NOTEBOOK.cells[12].with_rich_text("🇳🇱 and", Formatting::default()),
                    index: 12,
                },
            ],
            split_offset: Some(12),
            merge_offset: Some(6),
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[11] = TEST_NOTEBOOK.cells[11].with_rich_text(
                "italic bold both",
                vec![
                    AnnotationWithOffset {
                        annotation: Annotation::StartItalics,
                        offset: 0,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndItalics,
                        offset: 6,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::StartBold,
                        offset: 7,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndBold,
                        offset: 11,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::StartItalics,
                        offset: 12,
                    },
                    AnnotationWithOffset {
                        annotation: Annotation::EndItalics,
                        offset: 16,
                    },
                ],
            );
        }),
    });
}

fn create_update_cell_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let updated_cell1 = Cell::Text(TextCell {
        id: "c3".to_owned(),
        content: "Some updated text".to_owned(),
        formatting: Some(vec![
            AnnotationWithOffset::new(5, Annotation::StartItalics),
            AnnotationWithOffset::new(12, Annotation::EndItalics),
        ]),
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: updated_cell1.clone(),
                index: 2,
            }],
            old_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[2].clone(),
                index: 2,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = updated_cell1.clone();
        }),
    });

    // Make sure the cells of this test case overlap with the ones from the first, to
    // be able to test transforms between them:
    let updated_cell2 = Cell::Heading(HeadingCell {
        id: "c3".to_owned(),
        heading_type: HeadingType::H2,
        content: TEST_NOTEBOOK.cells[2].content().unwrap().to_owned(),
        formatting: Some(Formatting::default()),
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: updated_cell2.clone(),
                index: 2,
            }],
            old_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[2].clone(),
                index: 2,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = updated_cell2.clone();
        }),
    });

    let updated_cell3 = Cell::Heading(HeadingCell {
        id: "c3".to_owned(),
        heading_type: HeadingType::H3,
        content: TEST_NOTEBOOK.cells[2].content().unwrap().to_owned(),
        formatting: Some(Formatting::default()),
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: updated_cell3.clone(),
                index: 2,
            }],
            old_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[2].clone(),
                index: 2,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = updated_cell3.clone();
        }),
    });

    let updated_cell4 = Cell::Text(TextCell {
        id: "c3".to_owned(),
        content: TEST_NOTEBOOK.cells[2].content().unwrap().to_owned(),
        formatting: Some(Formatting::default()),
        read_only: Some(true),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: updated_cell4.clone(),
                index: 2,
            }],
            old_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[2].clone(),
                index: 2,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = updated_cell4.clone();
        }),
    });

    let updated_cell5: Cell = Cell::Log(LogCell {
        id: "c11".to_owned(),
        title: "Logs".to_owned(),
        source_ids: vec!["c10".to_owned()],
        read_only: Some(true),
        time_range: Some(TimeRange {
            from: 50.0,
            to: 150.0,
        }),
        ..Default::default()
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: vec![CellWithIndex {
                cell: updated_cell5.clone(),
                index: 10,
            }],
            old_cells: vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[10].clone(),
                index: 10,
            }],
            ..Default::default()
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[10] = updated_cell5.clone();
        }),
    })
}

fn create_update_notebook_time_range_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let new_time_range1 = TimeRange {
        from: 100.0,
        to: 200.0,
    };

    test_cases.push(OperationTestCase {
        operation: Operation::UpdateNotebookTimeRange(UpdateNotebookTimeRangeOperation {
            old_time_range: TEST_NOTEBOOK.time_range.clone(),
            time_range: new_time_range1.clone(),
        }),
        expected_apply_operation_result: {
            Notebook {
                time_range: new_time_range1,
                ..TEST_NOTEBOOK.clone()
            }
        },
    });

    // Another one to see if they can converge:
    let new_time_range2 = TimeRange {
        from: 150.0,
        to: 250.0,
    };

    test_cases.push(OperationTestCase {
        operation: Operation::UpdateNotebookTimeRange(UpdateNotebookTimeRangeOperation {
            old_time_range: TEST_NOTEBOOK.time_range.clone(),
            time_range: new_time_range2.clone(),
        }),
        expected_apply_operation_result: {
            Notebook {
                time_range: new_time_range2,
                ..TEST_NOTEBOOK.clone()
            }
        },
    });
}

fn create_add_data_source_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let data_source = NotebookDataSource::Organization(OrganizationDataSource {
        id: String::from(""),
        name: String::from("org_data_source_a"),
        default_data_source: true,
        data_source: DataSource::Prometheus(PrometheusDataSource {
            url: String::from("https://localhost:9000"),
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::AddDataSource(AddDataSourceOperation {
            name: String::from("org_data_source_a"),
            data_source: Box::new(data_source.clone()),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_data_sources(|data_sources| {
            data_sources.insert(String::from("org_data_source_a"), data_source.clone());
        }),
    });
}

fn create_update_data_source_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let data_source_name = String::from("inline_data_source_a");
    let data_source = NotebookDataSource::Organization(OrganizationDataSource {
        id: String::from(""),
        name: String::from("org_data_source_a"),
        default_data_source: true,
        data_source: DataSource::Prometheus(PrometheusDataSource {
            url: String::from("https://localhost:9000"),
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::UpdateDataSource(UpdateDataSourceOperation {
            name: data_source_name.clone(),
            data_source: Box::new(data_source.clone()),
            old_data_source: Box::new(TEST_NOTEBOOK.data_sources[&data_source_name].clone()),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_data_sources(|data_sources| {
            data_sources.insert(data_source_name, data_source.clone());
        }),
    });
}

fn create_remove_data_source_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let data_source_name = String::from("inline_data_source_a");

    test_cases.push(OperationTestCase {
        operation: Operation::RemoveDataSource(RemoveDataSourceOperation {
            name: data_source_name.clone(),
            data_source: Box::new(TEST_NOTEBOOK.data_sources[&data_source_name].clone()),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_data_sources(|data_sources| {
            data_sources.remove(&data_source_name);
        }),
    });
}

fn create_add_label_test_case(test_cases: &mut Vec<OperationTestCase>) {
    test_cases.push(OperationTestCase {
        operation: Operation::AddLabel(AddLabelOperation {
            label: Label::new("label-a", "label-a-value"),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_labels(|labels| {
            let label = Label::new("label-a", "label-a-value");
            labels.push(label);
        }),
    });
}

fn create_replace_label_test_case(test_cases: &mut Vec<OperationTestCase>) {
    // Test updating the value
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceLabel(ReplaceLabelOperation {
            old_label: Label::new("existing-key", "existing-value"),
            new_label: Label::new("existing-key", "new-value"),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_labels(|labels| {
            if let Some(label) = labels.iter_mut().find(|label| label.key == "existing-key") {
                *label = Label::new("existing-key", "new-value")
            } else {
                panic!("label not found");
            };
        }),
    });

    // Test updating the key
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceLabel(ReplaceLabelOperation {
            old_label: Label::new("existing-key", "existing-value"),
            new_label: Label::new("new-key", "existing-value"),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_labels(|labels| {
            if let Some(label) = labels.iter_mut().find(|label| label.key == "existing-key") {
                *label = Label::new("new-key", "existing-value")
            } else {
                panic!("label not found");
            };
        }),
    });
}

fn create_remove_label_test_case(test_cases: &mut Vec<OperationTestCase>) {
    test_cases.push(OperationTestCase {
        operation: Operation::RemoveLabel(RemoveLabelOperation {
            label: Label::new("existing-key", "existing-value"),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_labels(|labels| {
            let target_key = String::from("existing-key");
            labels.retain(|label| label.key != target_key);
        }),
    });
}
