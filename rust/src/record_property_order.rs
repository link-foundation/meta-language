//! Fallible transport into the complete automatically translated order rules.
use crate::translation::javascript_record_order_rules as rules;
use std::{error::Error, fmt};

/// The same TypeError message as the JavaScript own-field order operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordPropertyOrderError {
    message: String,
}
impl RecordPropertyOrderError {
    /// Original source error class.
    #[must_use]
    pub const fn class_name(&self) -> &'static str {
        "TypeError"
    }
    /// Original source message, without a host prefix.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}
impl fmt::Display for RecordPropertyOrderError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str(&self.message)
    }
}
impl Error for RecordPropertyOrderError {}
fn admit(message: String) -> Result<(), RecordPropertyOrderError> {
    if message.is_empty() {
        Ok(())
    } else {
        Err(RecordPropertyOrderError { message })
    }
}

pub use rules::{
    accept_java_script_record_order_names, accept_java_script_record_order_permutation,
    accept_java_script_record_property_order,
};

/// Create numeric-first own-field order from actual first insertion positions.
///
/// # Errors
/// Returns the source TypeError for noncanonical names or a nonpermutation.
pub fn create_java_script_record_property_order(
    names: &[Vec<f64>],
    insertion: &[f64],
) -> Result<Vec<f64>, RecordPropertyOrderError> {
    admit(rules::read_java_script_record_order_creation_error(
        names, insertion,
    ))?;
    Ok(rules::create_java_script_record_property_order(
        names, insertion,
    ))
}
/// Merge actual definition/spread chunks without moving overwritten fields.
///
/// # Errors
/// Returns the source TypeError for missing, invalid or out-of-range fields.
pub fn merge_java_script_record_property_order(
    names: &[Vec<f64>],
    chunks: &[Vec<f64>],
) -> Result<Vec<f64>, RecordPropertyOrderError> {
    admit(rules::read_java_script_record_order_merge_error(
        names, chunks,
    ))?;
    Ok(rules::merge_java_script_record_property_order(
        names, chunks,
    ))
}
/// Rebase a complete source order into a new actual canonical field layout.
///
/// # Errors
/// Returns the source TypeError for an invalid order or a missing target field.
pub fn rebase_java_script_record_property_order(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    order: &[f64],
) -> Result<Vec<f64>, RecordPropertyOrderError> {
    admit(rules::read_java_script_record_order_rebase_error(
        target, source, order,
    ))?;
    Ok(rules::rebase_java_script_record_property_order(
        target, source, order,
    ))
}
/// Read every exact UTF16 field name through its independently stored order.
///
/// # Errors
/// Returns the source TypeError for malformed or noncanonical order data.
pub fn read_java_script_record_property_keys(
    names: &[Vec<f64>],
    order: &[f64],
) -> Result<Vec<Vec<f64>>, RecordPropertyOrderError> {
    admit(rules::read_java_script_record_order_key_error(names, order))?;
    Ok(rules::read_java_script_record_property_keys(names, order))
}
