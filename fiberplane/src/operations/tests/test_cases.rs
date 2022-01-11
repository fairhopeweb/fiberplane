use super::fixtures::TEST_NOTEBOOK;
use crate::{
    operations::{char_count, Notebook, NotebookVisibility},
    protocols::{core::*, operations::*},
};
use once_cell::sync::Lazy;
use time::OffsetDateTime;

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
    create_split_cell_test_cases(&mut test_cases);
    create_update_cell_test_cases(&mut test_cases);
    create_update_notebook_time_range_test_cases(&mut test_cases);

    create_add_data_source_test_cases(&mut test_cases);
    create_update_data_source_test_cases(&mut test_cases);
    create_remove_data_source_test_cases(&mut test_cases);

    test_cases
});

fn create_add_cells_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let new_cell_1 = Cell::Text(TextCell {
        id: "n1".to_owned(),
        content: "New cell 1".to_owned(),
        read_only: None,
    });
    let new_cell_2 = Cell::Text(TextCell {
        id: "n2".to_owned(),
        content: "New cell 2".to_owned(),
        read_only: Some(true),
    });

    // Test appending cells to the back:
    test_cases.push(OperationTestCase {
        operation: Operation::AddCells(AddCellsOperation {
            cells: vec![
                CellWithIndex {
                    cell: new_cell_1.clone(),
                    index: TEST_NOTEBOOK.cells.len() as u32,
                },
                CellWithIndex {
                    cell: new_cell_2.clone(),
                    index: TEST_NOTEBOOK.cells.len() as u32 + 1,
                },
            ],
            referencing_cells: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.push(new_cell_1.clone());
            cells.push(new_cell_2.clone());
        }),
    });

    let new_cell_3 = Cell::Table(TableCell {
        id: "n3".to_owned(),
        source_ids: vec![],
        data: None,
        read_only: None,
    });

    // Test prepending cells to the front:
    test_cases.push(OperationTestCase {
        operation: Operation::AddCells(AddCellsOperation {
            cells: vec![CellWithIndex {
                cell: new_cell_3.clone(),
                index: 0,
            }],
            referencing_cells: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.insert(0, new_cell_3.clone());
        }),
    });

    let new_cell_4 = Cell::Heading(HeadingCell {
        id: "n4".to_owned(),
        heading_type: HeadingType::H3,
        content: "New heading 4".to_owned(),
        read_only: None,
    });

    // Test inserting a cell somewhere in the middle:
    test_cases.push(OperationTestCase {
        operation: Operation::AddCells(AddCellsOperation {
            cells: vec![CellWithIndex {
                cell: new_cell_4.clone(),
                index: 4,
            }],
            referencing_cells: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.insert(4, new_cell_4.clone());
        }),
    });

    let new_cell_5 = Cell::Text(TextCell {
        id: "n5".to_owned(),
        content: "New cell 5".to_owned(),
        read_only: None,
    });

    // Test inserting another at the same position to cover transforms:
    test_cases.push(OperationTestCase {
        operation: Operation::AddCells(AddCellsOperation {
            cells: vec![CellWithIndex {
                cell: new_cell_5.clone(),
                index: 4,
            }],
            referencing_cells: None,
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
        operation: Operation::AddCells(AddCellsOperation {
            cells: vec![CellWithIndex {
                cell: new_cell_6.clone(),
                index: 0,
            }],
            referencing_cells: Some(vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[6]
                    .with_source_ids(vec!["c6".to_owned(), "n6".to_owned()]),
                index: 6,
            }]),
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
        operation: Operation::MergeCells(MergeCellsOperation {
            glue_text: None,
            source_cell: TEST_NOTEBOOK.cells[2].clone(),
            target_cell_id: "c2".to_owned(),
            target_content_length: TEST_NOTEBOOK.cells[1].content().map(char_count).unwrap(),
            referencing_cells: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[1] = cells[1].with_appended_content(cells[2].content().unwrap());
            cells.remove(2);
        }),
    });

    // Make sure the cells of this test case overlap with the ones from the first, to
    // be able to test transforms between them:
    test_cases.push(OperationTestCase {
        operation: Operation::MergeCells(MergeCellsOperation {
            glue_text: None,
            source_cell: TEST_NOTEBOOK.cells[3].clone(),
            target_cell_id: "c3".to_owned(),
            target_content_length: TEST_NOTEBOOK.cells[2].content().map(char_count).unwrap(),
            referencing_cells: Some(vec![TEST_NOTEBOOK.clone_cell_with_index_by_id("c9")]),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_appended_content(cells[3].content().unwrap());
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
        operation: Operation::MergeCells(MergeCellsOperation {
            glue_text: Some("glue".to_owned()),
            source_cell: TEST_NOTEBOOK.cells[3].clone(),
            target_cell_id: "c3".to_owned(),
            target_content_length: TEST_NOTEBOOK.cells[2].content().map(char_count).unwrap(),
            referencing_cells: Some(vec![TEST_NOTEBOOK.clone_cell_with_index_by_id("c9")]),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] =
                cells[2].with_appended_content(&format!("glue{}", cells[3].content().unwrap()));
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
        operation: Operation::MergeCells(MergeCellsOperation {
            glue_text: Some("gluten".to_owned()),
            source_cell: TEST_NOTEBOOK.cells[2].clone(),
            target_cell_id: "c2".to_owned(),
            target_content_length: TEST_NOTEBOOK.cells[1].content().map(char_count).unwrap(),
            referencing_cells: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[1] =
                cells[1].with_appended_content(&format!("gluten{}", cells[2].content().unwrap()));
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
        operation: Operation::RemoveCells(RemoveCellsOperation {
            referencing_cells: Some(vec![TEST_NOTEBOOK.clone_cell_with_index_by_id("c9")]),
            removed_cells: vec![TEST_NOTEBOOK.clone_cell_with_index_by_id("c4")],
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
        operation: Operation::RemoveCells(RemoveCellsOperation {
            removed_cells: vec![
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c4"),
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c5"),
            ],
            referencing_cells: Some(vec![TEST_NOTEBOOK.clone_cell_with_index_by_id("c9")]),
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
        operation: Operation::RemoveCells(RemoveCellsOperation {
            removed_cells: vec![
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c4"),
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c5"),
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c6"),
            ],
            referencing_cells: Some(vec![
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c7"),
                TEST_NOTEBOOK.clone_cell_with_index_by_id("c9"),
            ]),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells.remove(3);
            cells.remove(3);
            cells.remove(3);
            cells.remove(3);
            cells.remove(4);
        }),
    });
}

fn create_replace_text_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c3".to_owned(),
            offset: 5,
            new_text: "replaced".to_owned(),
            old_text: "introductory".to_owned(),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK
            .with_updated_cells(|cells| cells[2] = cells[2].with_content("Some replaced text")),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c3".to_owned(),
            offset: 18,
            new_text: "nonsense".to_owned(),
            old_text: "text".to_owned(),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = cells[2].with_content("Some introductory nonsense")
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c3".to_owned(),
            offset: 17,
            new_text: "_".to_owned(),
            old_text: " ".to_owned(),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK
            .with_updated_cells(|cells| cells[2] = cells[2].with_content("Some introductory_text")),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c3".to_owned(),
            offset: 5,
            new_text: "replacement".to_owned(),
            old_text: "introductory".to_owned(),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK
            .with_updated_cells(|cells| cells[2] = cells[2].with_content("Some replacement text")),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c2".to_owned(),
            offset: 0,
            new_text: "Unl".to_owned(),
            old_text: "L".to_owned(),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK
            .with_updated_cells(|cells| cells[1] = cells[1].with_content("Unlocked subtitle")),
    });

    // Test cases that overlap with split cell cases:
    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c4".to_owned(),
            offset: 4,
            new_text: "".to_owned(),
            old_text: "emstats_".to_owned(),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK
            .with_updated_cells(|cells| cells[3] = cells[3].with_content("go_malloc_bytes")),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c4".to_owned(),
            offset: 18,
            new_text: "count".to_owned(),
            old_text: "bytes".to_owned(),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[3] = cells[3].with_content("go_memstats_alloc_count")
        }),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::ReplaceText(ReplaceTextOperation {
            cell_id: "c4".to_owned(),
            offset: 0,
            new_text: "".to_owned(),
            old_text: "go_".to_owned(),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK
            .with_updated_cells(|cells| cells[3] = cells[3].with_content("memstats_alloc_bytes")),
    });
}

fn create_split_cell_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let split_cell1 = Cell::Prometheus(PrometheusCell {
        id: "s1".to_owned(),
        content: "memstats_alloc_bytes".to_owned(),
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::SplitCell(SplitCellOperation {
            cell_id: "c4".to_owned(),
            removed_text: None,
            split_index: 3,
            new_cell: split_cell1.clone(),
            referencing_cells: None,
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
        operation: Operation::SplitCell(SplitCellOperation {
            cell_id: "c4".to_owned(),
            removed_text: Some("alloc_".to_owned()),
            split_index: 12,
            new_cell: split_cell2.clone(),
            referencing_cells: None,
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
        operation: Operation::SplitCell(SplitCellOperation {
            cell_id: "c4".to_owned(),
            removed_text: None,
            split_index: 18,
            new_cell: split_cell3.clone(),
            referencing_cells: Some(vec![CellWithIndex {
                cell: TEST_NOTEBOOK.cells[8].with_source_ids(vec![
                    "c4".to_owned(),
                    "c6".to_owned(),
                    "s3".to_owned(),
                ]),
                index: 9,
            }]),
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
    let split_cell4 = Cell::ListItem(ListItemCell {
        id: "s4".to_owned(),
        content:
            "Right before our crown jewel: ***a locked, multi-sourced bar graph with a custom time range***!"
                .to_owned(),
        level: None,
        list_type: ListType::Unordered,
        read_only: None,
        start_number: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::SplitCell(SplitCellOperation {
            cell_id: "c8".to_owned(),
            removed_text: Some("\n\n".to_owned()),
            split_index: 63,
            new_cell: split_cell4.clone(),
            referencing_cells: None,
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[7] = cells[7]
                .with_content("No test *notebook* would be complete without some **Markdown**.");
            cells.insert(8, split_cell4);
        }),
    });
}

fn create_update_cell_test_cases(test_cases: &mut Vec<OperationTestCase>) {
    let updated_cell1 = Cell::Text(TextCell {
        id: "c3".to_owned(),
        content: "Some updated text".to_owned(),
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::UpdateCell(UpdateCellOperation {
            old_cell: Box::new(TEST_NOTEBOOK.cells[2].clone()),
            updated_cell: Box::new(updated_cell1.clone()),
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
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::UpdateCell(UpdateCellOperation {
            old_cell: Box::new(TEST_NOTEBOOK.cells[2].clone()),
            updated_cell: Box::new(updated_cell2.clone()),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = updated_cell2.clone();
        }),
    });

    let updated_cell3 = Cell::Heading(HeadingCell {
        id: "c3".to_owned(),
        heading_type: HeadingType::H3,
        content: TEST_NOTEBOOK.cells[2].content().unwrap().to_owned(),
        read_only: None,
    });

    test_cases.push(OperationTestCase {
        operation: Operation::UpdateCell(UpdateCellOperation {
            old_cell: Box::new(TEST_NOTEBOOK.cells[2].clone()),
            updated_cell: Box::new(updated_cell3.clone()),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = updated_cell3.clone();
        }),
    });

    let updated_cell4 = Cell::Text(TextCell {
        id: "c3".to_owned(),
        content: TEST_NOTEBOOK.cells[2].content().unwrap().to_owned(),
        read_only: Some(true),
    });

    test_cases.push(OperationTestCase {
        operation: Operation::UpdateCell(UpdateCellOperation {
            old_cell: Box::new(TEST_NOTEBOOK.cells[2].clone()),
            updated_cell: Box::new(updated_cell4.clone()),
        }),
        expected_apply_operation_result: TEST_NOTEBOOK.with_updated_cells(|cells| {
            cells[2] = updated_cell4.clone();
        }),
    });
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
                id: TEST_NOTEBOOK.id.clone(),
                cells: TEST_NOTEBOOK.cells.clone(),
                read_only: false,
                revision: TEST_NOTEBOOK.revision,
                title: TEST_NOTEBOOK.title.clone(),
                time_range: new_time_range1,
                data_sources: TEST_NOTEBOOK.data_sources.clone(),
                visibility: NotebookVisibility::Private,
                created_at: OffsetDateTime::UNIX_EPOCH,
                updated_at: OffsetDateTime::UNIX_EPOCH,
                created_by: CreatedBy {
                    user_type: UserType::Individual,
                    name: "name".to_string(),
                },
                labels: Vec::new(),
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
                id: TEST_NOTEBOOK.id.clone(),
                cells: TEST_NOTEBOOK.cells.clone(),
                read_only: false,
                revision: TEST_NOTEBOOK.revision,
                title: TEST_NOTEBOOK.title.clone(),
                time_range: new_time_range2,
                data_sources: TEST_NOTEBOOK.data_sources.clone(),
                visibility: NotebookVisibility::Private,
                created_at: OffsetDateTime::UNIX_EPOCH,
                updated_at: OffsetDateTime::UNIX_EPOCH,
                created_by: CreatedBy {
                    user_type: UserType::Individual,
                    name: "name".to_string(),
                },
                labels: Vec::new(),
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
