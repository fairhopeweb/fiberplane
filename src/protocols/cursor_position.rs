use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::usize;

/// The position of the user's cursor, including an optional selection.
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CursorPosition {
    Offset(usize),
    Range { anchor: usize, focus: usize },
}

impl CursorPosition {
    /// Returns the anchor of the selection.
    ///
    /// This is the offset where the selection began, or simply the cursor offset if there is no
    /// active selection.
    pub fn anchor(&self) -> usize {
        match self {
            Self::Offset(anchor) => *anchor,
            Self::Range { anchor, .. } => *anchor,
        }
    }

    /// Returns the focus of the selection.
    ///
    /// This is the offset where the selection ended, which corresponds with the offset the cursor
    /// is currently at.
    pub fn focus(&self) -> usize {
        match self {
            Self::Offset(focus) => *focus,
            Self::Range { focus, .. } => *focus,
        }
    }

    /// Returns the start (leftmost offset, if you will) of the selection.
    ///
    /// If there is no active selection, this corresponds with the current cursor ofset.
    pub fn start(&self) -> usize {
        min(self.anchor(), self.focus())
    }

    /// Returns the end (rightmost offset, if you will) of the selection.
    ///
    /// If there is no active selection, this corresponds with the current cursor ofset.
    pub fn end(&self) -> usize {
        max(self.anchor(), self.focus())
    }
}
