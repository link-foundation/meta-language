//! Actual checked value of one canonically positioned record field.
use serde::{Deserialize, Serialize};

use super::Expr;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordEntry {
    pub name: String,
    pub value: Expr,
}
