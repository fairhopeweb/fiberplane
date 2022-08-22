use super::*;
use crate::protocols::{core::Cell, formatting::Formatting};

/// Represents the changes we've seen to a single cell.
enum CellChangeState {
    None,
    Inserted {
        cell: Cell,
        index: u32,
    },
    Updated {
        cell: Cell,
    },
    Moved {
        cell_id: String,
        index: u32,
    },
    TextUpdated {
        cell_id: String,
        field: Option<String>,
        text: String,
        formatting: Option<Formatting>,
    },
}

impl CellChangeState {
    fn cell_id(&self) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Inserted { cell, .. } => Some(cell.id()),
            Self::Updated { cell } => Some(cell.id()),
            Self::Moved { cell_id, .. } => Some(cell_id),
            Self::TextUpdated { cell_id, .. } => Some(cell_id),
        }
    }

    fn into_change(self) -> Option<Change> {
        match self {
            Self::None => None,
            Self::Inserted { cell, index } => {
                Some(Change::InsertCell(InsertCellChange { cell, index }))
            }
            Self::Updated { cell } => Some(Change::UpdateCell(UpdateCellChange { cell })),
            Self::Moved { cell_id, index } => Some(Change::MoveCells(MoveCellsChange {
                cell_ids: vec![cell_id],
                index,
            })),
            Self::TextUpdated {
                cell_id,
                field,
                text,
                formatting,
            } => Some(Change::UpdateCellText(UpdateCellTextChange {
                cell_id,
                field,
                text,
                formatting,
            })),
        }
    }
}

/// Simplifies a list of changes.
///
/// Whenever multiple operations have been processed in a batch, chances are the resulting changes
/// have an overlap in affected cells. In order to avoid overhead in serialization and persistence,
/// we attempt to simplify those changes, so that multiple changes to the same cell get reduced to
/// a single change.
///
/// This is implemented using a two-pass algorithm:
///
/// - The first pass works by keeping the state for the current cell open for modification by
///   follow-up changes. Whenever we encounter a change for a different cell ID, we "commit" the
///   changes for the cell that is currently open. This way, we can easily maintain the order of
///   changes and avoid interleaving changes to different cells. This is important because
///   insert/move/delete changes affect the indices of all following cells, and reordering them
///   could have unintended side-effects.
/// - The second pass specifically looks for update changes that are obsoleted by later changes,
///   regardless of changes to other cells in between. This step is still valuable because changes
///   to other cells might get interwoven during rebase operations.
pub fn simplify_changes(changes: Vec<Change>) -> Vec<Change> {
    let mut simplified_changes: Vec<Change> = vec![];

    // PASS 1:
    let mut current_cell_state = CellChangeState::None;
    for change in changes.into_iter() {
        use CellChangeState::*;
        match change {
            Change::DeleteCell(delete_change) => {
                if current_cell_state.cell_id() == Some(&delete_change.cell_id) {
                    match current_cell_state {
                        Inserted { .. } => { /* The delete simply cancels out the insert. */ }
                        Moved { cell_id, index } => {
                            simplified_changes.push(Change::MoveCells(MoveCellsChange {
                                cell_ids: vec![cell_id],
                                index,
                            }));
                            simplified_changes.push(Change::DeleteCell(delete_change));
                        }
                        _ => {
                            simplified_changes.push(Change::DeleteCell(delete_change));
                        }
                    }
                } else {
                    if let Some(change) = current_cell_state.into_change() {
                        simplified_changes.push(change);
                    }
                    simplified_changes.push(Change::DeleteCell(delete_change));
                }

                current_cell_state = None;
            }
            Change::InsertCell(insert_change) => {
                if let Some(change) = current_cell_state.into_change() {
                    simplified_changes.push(change);
                }

                current_cell_state = Inserted {
                    cell: insert_change.cell,
                    index: insert_change.index,
                };
            }
            Change::MoveCells(move_change) => {
                if move_change.cell_ids.len() == 1 {
                    let cell_id = &move_change.cell_ids[0];
                    let index = move_change.index;
                    if current_cell_state.cell_id() == Some(cell_id) {
                        current_cell_state = match current_cell_state {
                            None => Moved {
                                cell_id: cell_id.clone(),
                                index,
                            },
                            Inserted { cell, index: _ } => Inserted { cell, index },
                            Updated { cell } => {
                                simplified_changes
                                    .push(Change::UpdateCell(UpdateCellChange { cell }));
                                Moved {
                                    cell_id: cell_id.clone(),
                                    index,
                                }
                            }
                            Moved { cell_id, index: _ } => Moved { cell_id, index },
                            TextUpdated {
                                cell_id,
                                field,
                                text,
                                formatting,
                            } => {
                                simplified_changes.push(Change::UpdateCellText(
                                    UpdateCellTextChange {
                                        cell_id: cell_id.clone(),
                                        field,
                                        text,
                                        formatting,
                                    },
                                ));
                                Moved { cell_id, index }
                            }
                        };
                    } else {
                        if let Some(change) = current_cell_state.into_change() {
                            simplified_changes.push(change);
                        }

                        current_cell_state = Moved {
                            cell_id: cell_id.clone(),
                            index,
                        };
                    }
                } else {
                    if let Some(change) = current_cell_state.into_change() {
                        simplified_changes.push(change);
                    }

                    simplified_changes.push(Change::MoveCells(move_change));

                    current_cell_state = None;
                }
            }
            Change::UpdateCell(update_change) => {
                let cell = update_change.cell;
                if current_cell_state.cell_id() == Some(cell.id()) {
                    current_cell_state = match current_cell_state {
                        None => Updated { cell },
                        Inserted { cell: _, index } => Inserted { cell, index },
                        Updated { .. } | TextUpdated { .. } => Updated { cell },
                        Moved { cell_id, index } => {
                            simplified_changes.push(Change::MoveCells(MoveCellsChange {
                                cell_ids: vec![cell_id],
                                index,
                            }));
                            Updated { cell }
                        }
                    };
                } else {
                    if let Some(change) = current_cell_state.into_change() {
                        simplified_changes.push(change);
                    }

                    current_cell_state = Updated { cell };
                }
            }
            Change::UpdateCellText(UpdateCellTextChange {
                cell_id,
                field,
                formatting,
                text,
            }) => {
                if current_cell_state.cell_id() == Some(&cell_id) {
                    current_cell_state = match current_cell_state {
                        None => TextUpdated {
                            cell_id,
                            field,
                            text,
                            formatting,
                        },
                        Inserted { cell, index } => Inserted {
                            cell: cell.with_text_for_field(&text, formatting, field.as_deref()),
                            index,
                        },
                        Updated { cell } => Updated {
                            cell: cell.with_text_for_field(&text, formatting, field.as_deref()),
                        },
                        Moved { cell_id, index } => {
                            simplified_changes.push(Change::MoveCells(MoveCellsChange {
                                cell_ids: vec![cell_id.clone()],
                                index,
                            }));
                            TextUpdated {
                                cell_id,
                                field,
                                text,
                                formatting,
                            }
                        }
                        TextUpdated {
                            cell_id: current_cell_id,
                            field: current_field,
                            text: current_text,
                            formatting: current_formatting,
                        } => {
                            if field != current_field {
                                simplified_changes.push(Change::UpdateCellText(
                                    UpdateCellTextChange {
                                        cell_id: current_cell_id,
                                        field: current_field,
                                        text: current_text,
                                        formatting: current_formatting,
                                    },
                                ));
                            }
                            TextUpdated {
                                cell_id,
                                field,
                                text,
                                formatting,
                            }
                        }
                    };
                } else {
                    if let Some(change) = current_cell_state.into_change() {
                        simplified_changes.push(change);
                    }

                    current_cell_state = TextUpdated {
                        cell_id,
                        field,
                        text,
                        formatting,
                    };
                }
            }
            other => simplified_changes.push(other),
        }
    }

    if let Some(change) = current_cell_state.into_change() {
        simplified_changes.push(change);
    }

    // PASS 2:
    let mut i = 0;
    while i < simplified_changes.len() {
        // We skip the change if it's an update that is followed by another that will supersede it:
        let skip_change = match &simplified_changes[i] {
            Change::UpdateCell(update_change) => simplified_changes.iter().skip(i + 1).any(|change| {
                matches!(change, Change::UpdateCell(other) if other.cell.id() == update_change.cell.id()) ||
                matches!(change, Change::DeleteCell(other) if other.cell_id == update_change.cell.id())
            }),
            Change::UpdateCellText(update_change) => simplified_changes.iter().skip(i + 1).any(|change| {
                matches!(change, Change::UpdateCell(other) if other.cell.id() == update_change.cell_id) ||
                matches!(change, Change::UpdateCellText(other) if other.cell_id == update_change.cell_id && other.field == update_change.field) ||
                matches!(change, Change::DeleteCell(other) if other.cell_id == update_change.cell_id)
            }),
            _ => false
        };

        if skip_change {
            simplified_changes.remove(i);
        } else {
            i += 1;
        }
    }

    simplified_changes
}
