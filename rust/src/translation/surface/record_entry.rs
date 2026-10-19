//! Ordered ordinary record entries, retaining the original source value.
use serde::{Deserialize, Serialize};

use super::SExpr;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SRecordEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub spread: bool,
    pub value: SExpr,
}
