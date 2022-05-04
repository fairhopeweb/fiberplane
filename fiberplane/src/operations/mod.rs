mod apply_operation;
mod changes;
mod error;
mod invert_operation;
mod notebook;
mod relevant_cell_ids_for_operation;
mod transform_operation;
mod validate_operation;

#[cfg(any(feature = "fixtures", test))]
pub mod fixtures;

#[cfg(test)]
mod tests;

pub use apply_operation::{
    apply_operation, replace_formatting, replace_text, ApplyOperationState, CellRefWithIndex,
};
pub use changes::*;
pub use error::*;
pub use invert_operation::invert_operation;
pub use notebook::*;
pub use relevant_cell_ids_for_operation::relevant_cell_ids_for_operation;
pub use transform_operation::{transform_operation, TransformOperationState};
pub use validate_operation::validate_operation;
