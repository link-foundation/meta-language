//! Owned closed scalar loop header, state and ordered continuation proof.
//! This isolated draft retains value and type checks in the existing core.
use serde::{Deserialize, Serialize};

mod validation;
pub(crate) use validation::{validate_source_loop, validate_source_loop_at};

/// One actual function parameter participating in the owned loop state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScalarLoopState {
    /// Actual parameter binding identity.
    pub name: String,
    /// Whether the parsed source permits writes to this slot.
    pub mutable: bool,
}

/// One scalar snapshot written to a parameter after every snapshot is read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScalarLoopWrite {
    /// The actual mutable parameter slot.
    pub target: String,
    /// The immutable binding evaluated in the previous state.
    pub binding: String,
}

/// Actual ordered writes and the resulting complete continuation argument list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScalarLoopTransition {
    /// Snapshot evaluation order, independent of parameter position.
    pub writes: Vec<ScalarLoopWrite>,
    /// One actual argument binding for every state slot.
    pub arguments: Vec<String>,
}

/// A selected JavaScript scalar interface position.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScalarLoopInterfaceType {
    /// Number or Boolean value kind checked against the actual declaration.
    pub kind: String,
}

/// Actual inferred Number/Boolean header facts, distinct from Rust owners.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScalarLoopInterface {
    /// The selected JavaScript scalar interpretation.
    pub model: String,
    /// Inferred actual parameter kinds in header order.
    pub parameters: Vec<ScalarLoopInterfaceType>,
    /// Inferred actual result kind.
    pub result: ScalarLoopInterfaceType,
}

/// Exact absolute namespace of the declaration owning a scalar continuation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScalarLoopContinuation {
    /// The shared absolute namespace root; independent of any local value name.
    pub root: String,
    /// Every actual declaration scope segment followed by the original name.
    pub path: Vec<String>,
}

/// An owned proof of one closed scalar loop and its immutable-state lowering.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceLoop {
    /// Explicit versioned source interpretation.
    pub model: String,
    /// Actual parsed source language.
    pub language: String,
    /// Actual original function name, including any checked reserved prefix.
    pub source_name: String,
    /// The explicitly selected iterative tail-continuation interpretation.
    pub lowering: String,
    /// Every actual parameter in its source order.
    pub state: Vec<ScalarLoopState>,
    /// Every actual continuation in expression traversal order.
    pub transitions: Vec<ScalarLoopTransition>,
    /// The JavaScript interface; Rust instead owns a source signature.
    #[serde(rename = "interface", default, skip_serializing_if = "Option::is_none")]
    pub source_interface: Option<ScalarLoopInterface>,
    /// Owned declaration target, present only in the declaration-directed profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation: Option<ScalarLoopContinuation>,
}

pub(crate) fn scalar_loop_policy() -> &'static serde_json::Value {
    static POLICY: std::sync::OnceLock<serde_json::Value> = std::sync::OnceLock::new();
    POLICY.get_or_init(|| {
        crate::read_structured_data_lino(include_str!("../data/rust-scalar-loops.lino"))
            .expect("shared scalar loop policy is structured Links data")
    })
}

pub(crate) fn scalar_loop_snapshot_binding(name: &str) -> bool {
    static PATTERN: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    PATTERN
        .get_or_init(|| {
            regex::Regex::new(
                scalar_loop_policy()["snapshotBindingPattern"]
                    .as_str()
                    .expect("shared snapshot grammar is text"),
            )
            .expect("shared scalar loop snapshot grammar is valid")
        })
        .is_match(name)
}
