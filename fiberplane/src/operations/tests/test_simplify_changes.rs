use crate::{
    operations::*,
    protocols::{core::*, formatting::Formatting},
};
use pretty_assertions::assert_eq;

#[test]
pub fn test_simplify_delete_changes() {
    let changes = vec![
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "1".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
        }),
        Change::DeleteCell(DeleteCellChange {
            cell_id: "test_cell".to_owned(),
        }),
    ];

    // The update is no longer relevant:
    assert_eq!(
        simplify_changes(changes),
        vec![Change::DeleteCell(DeleteCellChange {
            cell_id: "test_cell".to_owned(),
        })]
    );

    let changes = vec![
        Change::InsertCell(InsertCellChange {
            cell: Cell::Text(TextCell {
                content: "1".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
            index: 3,
        }),
        Change::DeleteCell(DeleteCellChange {
            cell_id: "test_cell".to_owned(),
        }),
    ];

    // The insert and delete cancel each other out:
    assert_eq!(simplify_changes(changes), vec![]);
}

#[test]
pub fn test_simplify_move_changes() {
    let changes = vec![
        Change::MoveCells(MoveCellsChange {
            cell_ids: vec!["test_cell".to_owned()],
            index: 3,
        }),
        Change::MoveCells(MoveCellsChange {
            cell_ids: vec!["test_cell".to_owned()],
            index: 2,
        }),
    ];

    // Only the last move is still relevant:
    assert_eq!(
        simplify_changes(changes),
        vec![Change::MoveCells(MoveCellsChange {
            cell_ids: vec!["test_cell".to_owned()],
            index: 2,
        })]
    );

    let changes = vec![
        Change::InsertCell(InsertCellChange {
            cell: Cell::Text(TextCell {
                content: "1".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
            index: 3,
        }),
        Change::MoveCells(MoveCellsChange {
            cell_ids: vec!["test_cell".to_owned()],
            index: 2,
        }),
    ];

    // We immediately insert at the new index:
    assert_eq!(
        simplify_changes(changes),
        vec![Change::InsertCell(InsertCellChange {
            cell: Cell::Text(TextCell {
                content: "1".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
            index: 2,
        })]
    );
}

#[test]
pub fn test_simplify_insert_changes() {
    let changes = vec![
        Change::InsertCell(InsertCellChange {
            cell: Cell::Text(TextCell {
                content: "1".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
            index: 3,
        }),
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "2".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
        }),
    ];

    // We immediately insert with the final content:
    assert_eq!(
        simplify_changes(changes),
        vec![Change::InsertCell(InsertCellChange {
            cell: Cell::Text(TextCell {
                content: "2".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
            index: 3
        })]
    );
}

#[test]
pub fn test_simplify_update_changes() {
    let changes = vec![
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "1".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
        }),
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "2".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
        }),
    ];

    // We only keep the last update:
    assert_eq!(
        simplify_changes(changes),
        vec![Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "2".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            })
        })]
    );
}

#[test]
pub fn test_simplify_update_and_delete_changes() {
    let changes = vec![
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "1".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
        }),
        Change::DeleteCell(DeleteCellChange {
            cell_id: "other_cell".to_owned(),
        }),
    ];

    // Deletion shouldn't affect update to other cell:
    assert_eq!(simplify_changes(changes.clone()), changes);
}

#[test]
pub fn test_simplify_update_text_changes() {
    let changes = vec![
        Change::UpdateCellText(UpdateCellTextChange {
            cell_id: "test_cell".to_owned(),
            text: "1".to_owned(),
            formatting: None,
        }),
        Change::UpdateCellText(UpdateCellTextChange {
            cell_id: "test_cell".to_owned(),
            text: "2".to_owned(),
            formatting: None,
        }),
    ];

    // We only keep the last update:
    assert_eq!(
        simplify_changes(changes),
        vec![Change::UpdateCellText(UpdateCellTextChange {
            cell_id: "test_cell".to_owned(),
            text: "2".to_owned(),
            formatting: None,
        })]
    );
}

#[test]
pub fn test_simplify_update_and_update_text_changes() {
    let changes = vec![
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "1".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
        }),
        Change::UpdateCellText(UpdateCellTextChange {
            cell_id: "test_cell".to_owned(),
            text: "2".to_owned(),
            formatting: None,
        }),
    ];

    // We merge the updated text into the update:
    assert_eq!(
        simplify_changes(changes),
        vec![Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "2".to_owned(),
                id: "test_cell".to_owned(),
                formatting: Some(Formatting::default()),
                ..Default::default()
            }),
        })]
    );

    let changes = vec![
        Change::UpdateCellText(UpdateCellTextChange {
            cell_id: "test_cell".to_owned(),
            text: "1".to_owned(),
            formatting: None,
        }),
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "2".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
        }),
    ];

    // Updates simply supersede a text update:
    assert_eq!(
        simplify_changes(changes),
        vec![Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "2".to_owned(),
                id: "test_cell".to_owned(),
                ..Default::default()
            }),
        })]
    );
}

#[test]
pub fn test_simplify_complex_changes() {
    let changes = vec![
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "2_1".to_owned(),
                id: "test_cell2".to_owned(),
                ..Default::default()
            }),
        }),
        Change::InsertCell(InsertCellChange {
            cell: Cell::Text(TextCell {
                content: "1_1".to_owned(),
                id: "test_cell1".to_owned(),
                ..Default::default()
            }),
            index: 3,
        }),
        Change::MoveCells(MoveCellsChange {
            cell_ids: vec!["test_cell1".to_owned()],
            index: 2,
        }),
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "1_2".to_owned(),
                id: "test_cell1".to_owned(),
                ..Default::default()
            }),
        }),
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "3_1".to_owned(),
                id: "test_cell3".to_owned(),
                ..Default::default()
            }),
        }),
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "2_2".to_owned(),
                id: "test_cell2".to_owned(),
                ..Default::default()
            }),
        }),
        Change::DeleteCell(DeleteCellChange {
            cell_id: "test_cell2".to_owned(),
        }),
        Change::UpdateCell(UpdateCellChange {
            cell: Cell::Text(TextCell {
                content: "3_2".to_owned(),
                id: "test_cell3".to_owned(),
                ..Default::default()
            }),
        }),
        Change::AddDataSource(AddDataSourceChange {
            data_source: Box::new(NotebookDataSource::Inline(InlineDataSource {
                data_source: DataSource::Prometheus(PrometheusDataSource {
                    url: "http://localhost:9090".to_owned(),
                }),
            })),
            name: "test_data_source".to_owned(),
        }),
    ];

    assert_eq!(
        simplify_changes(changes),
        vec![
            Change::InsertCell(InsertCellChange {
                cell: Cell::Text(TextCell {
                    content: "1_2".to_owned(),
                    id: "test_cell1".to_owned(),
                    ..Default::default()
                }),
                index: 2,
            }),
            Change::DeleteCell(DeleteCellChange {
                cell_id: "test_cell2".to_owned(),
            }),
            Change::AddDataSource(AddDataSourceChange {
                data_source: Box::new(NotebookDataSource::Inline(InlineDataSource {
                    data_source: DataSource::Prometheus(PrometheusDataSource {
                        url: "http://localhost:9090".to_owned(),
                    }),
                })),
                name: "test_data_source".to_owned(),
            }),
            Change::UpdateCell(UpdateCellChange {
                cell: Cell::Text(TextCell {
                    content: "3_2".to_owned(),
                    id: "test_cell3".to_owned(),
                    ..Default::default()
                }),
            }),
        ]
    );
}
