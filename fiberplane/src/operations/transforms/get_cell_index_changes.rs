use crate::protocols::operations::*;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CellIndexChange<'a> {
    Insertion {
        old_index: u32,
        new_index: u32,
        cell_id: &'a str,
        priority: CellIndexPriority,
    },
    Replacement {
        old_index: u32,
        new_index: u32,
        cell_id: &'a str,
    },
    Removal {
        old_index: u32,
        new_index: u32,
        cell_id: &'a str,
    },
}

/// Returns the adjustments to cell indices necessary for transformation by the
/// given operation.
pub(crate) fn get_cell_index_changes<'a>(
    op: &'a ReplaceCellsOperation,
) -> Vec<CellIndexChange<'a>> {
    let mut old_cells = ConditionalCellIterator::new(&op.old_cells, CellIndexPriority::Normal);
    let mut new_cells = ConditionalCellIterator::new(&op.new_cells, CellIndexPriority::Normal);
    let mut old_referencing_cells =
        ConditionalCellIterator::new(&op.old_referencing_cells, CellIndexPriority::Low);
    let mut new_referencing_cells =
        ConditionalCellIterator::new(&op.new_referencing_cells, CellIndexPriority::Low);

    let mut old_index = 0;
    let mut new_index = 0;
    let mut next_changes =
        || -> Option<(Option<CellIndexChange<'a>>, Option<CellIndexChange<'a>>)> {
            if old_cells.is_drained()
                && new_cells.is_drained()
                && old_referencing_cells.is_drained()
                && new_referencing_cells.is_drained()
            {
                return None;
            }

            let changes = if let Some((old_cell, _)) = old_cells.next(old_index) {
                let changes = if let Some((new_cell, priority)) = new_cells.next(new_index) {
                    let changes = if old_cell.id() == new_cell.id() {
                        (
                            Some(CellIndexChange::Replacement {
                                old_index,
                                new_index,
                                cell_id: old_cell.id(),
                            }),
                            None,
                        )
                    } else {
                        (
                            Some(CellIndexChange::Removal {
                                old_index,
                                new_index,
                                cell_id: old_cell.id(),
                            }),
                            Some(CellIndexChange::Insertion {
                                old_index,
                                new_index,
                                cell_id: new_cell.id(),
                                priority,
                            }),
                        )
                    };

                    new_index += 1;
                    changes
                } else {
                    let change = CellIndexChange::Removal {
                        old_index,
                        new_index,
                        cell_id: old_cell.id(),
                    };
                    (Some(change), None)
                };

                old_index += 1;
                changes
            } else if let Some((old_cell, _)) = old_referencing_cells.next(old_index) {
                let changes =
                    if let Some((new_cell, priority)) = new_referencing_cells.next(new_index) {
                        let changes = if old_cell.id() == new_cell.id() {
                            (
                                Some(CellIndexChange::Replacement {
                                    old_index,
                                    new_index,
                                    cell_id: old_cell.id(),
                                }),
                                None,
                            )
                        } else {
                            (
                                Some(CellIndexChange::Removal {
                                    old_index,
                                    new_index,
                                    cell_id: old_cell.id(),
                                }),
                                Some(CellIndexChange::Insertion {
                                    old_index,
                                    new_index,
                                    cell_id: new_cell.id(),
                                    priority,
                                }),
                            )
                        };

                        new_index += 1;
                        changes
                    } else {
                        let change = CellIndexChange::Removal {
                            old_index,
                            new_index,
                            cell_id: old_cell.id(),
                        };
                        (Some(change), None)
                    };

                old_index += 1;
                changes
            } else if let Some((new_cell, priority)) = new_cells.next(new_index) {
                let change = CellIndexChange::Insertion {
                    old_index,
                    new_index,
                    cell_id: new_cell.id(),
                    priority,
                };

                new_index += 1;
                (Some(change), None)
            } else if let Some((new_cell, priority)) = new_referencing_cells.next(new_index) {
                let change = CellIndexChange::Insertion {
                    old_index,
                    new_index,
                    cell_id: new_cell.id(),
                    priority,
                };

                new_index += 1;
                (Some(change), None)
            } else {
                old_index += 1;
                new_index += 1;
                (None, None)
            };

            Some(changes)
        };

    let mut changes = Vec::new();
    while let Some(next_changes) = next_changes() {
        if let (Some(change), second_change) = next_changes {
            changes.push(change);
            if let Some(change) = second_change {
                changes.push(change);
            }
        }
    }
    changes
}

/// Defines priority for resolving ties in cell indices during transformation.
/// The cell with the highest priority can stay at its given index, while the
/// other cell has to move behind it. If priorities are also tied, the cell with
/// the lexixographically lowest cell ID wins.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum CellIndexPriority {
    /// Used for referencing cells.
    Low,
    /// Used for regular new cells.
    Normal,
    /// Used for new cells that form a range with a regular cell.
    High,
}

impl CellIndexPriority {
    pub(crate) fn successor_should_move(
        predecessor_index: u32,
        predecessor_cell_id: &str,
        predecessor_priority: CellIndexPriority,
        successor_index: u32,
        successor_cell_id: &str,
        successor_priority: CellIndexPriority,
    ) -> bool {
        match predecessor_index.cmp(&successor_index) {
            Ordering::Less => true,
            Ordering::Equal => match predecessor_priority.cmp(&successor_priority) {
                Ordering::Less => false,
                Ordering::Equal => predecessor_cell_id < successor_cell_id,
                Ordering::Greater => true,
            },
            Ordering::Greater => false,
        }
    }
}

/// Iterates over the given range of cells, but only when the index of the cell
/// matches an expected index.
struct ConditionalCellIterator<'a> {
    cells: &'a [CellWithIndex],
    index: usize,
    priority: CellIndexPriority,
}

impl<'a> ConditionalCellIterator<'a> {
    fn is_drained(&self) -> bool {
        self.index == self.cells.len()
    }

    fn new(cells: &'a [CellWithIndex], priority: CellIndexPriority) -> Self {
        Self {
            cells,
            index: 0,
            priority,
        }
    }

    fn next(&mut self, expected_index: u32) -> Option<(&'a CellWithIndex, CellIndexPriority)> {
        if self.is_drained() {
            return None;
        }

        if self.cells[self.index].index == expected_index {
            let next = &self.cells[self.index];
            self.index += 1;
            Some((
                next,
                if self.index > 1 && self.priority == CellIndexPriority::Normal {
                    // Normal priority escalates to make sure ranges are not interrupted
                    CellIndexPriority::High
                } else {
                    self.priority
                },
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        operations::fixtures::TEST_NOTEBOOK,
        protocols::{core::*, formatting::*},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_cell_index_changes() {
        assert_eq!(
            get_cell_index_changes(&ReplaceCellsOperation {
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
            vec![
                CellIndexChange::Replacement {
                    old_index: 2,
                    new_index: 2,
                    cell_id: "c3"
                },
                CellIndexChange::Removal {
                    old_index: 3,
                    new_index: 3,
                    cell_id: "c4"
                }
            ]
        );

        assert_eq!(
            get_cell_index_changes(&ReplaceCellsOperation {
                new_cells: vec![
                    CellWithIndex {
                        cell: TEST_NOTEBOOK.cells[3].with_text(""),
                        index: 3,
                    },
                    CellWithIndex {
                        cell: Cell::Prometheus(PrometheusCell {
                            id: "s3".to_owned(),
                            content: "bytes".to_owned(),
                            read_only: None,
                        }),
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
            vec![
                CellIndexChange::Replacement {
                    old_index: 3,
                    new_index: 3,
                    cell_id: "c4"
                },
                CellIndexChange::Insertion {
                    old_index: 4,
                    new_index: 4,
                    cell_id: "s3",
                    priority: CellIndexPriority::High
                },
                CellIndexChange::Replacement {
                    old_index: 8,
                    new_index: 9,
                    cell_id: "c9"
                },
            ]
        );
    }
}
