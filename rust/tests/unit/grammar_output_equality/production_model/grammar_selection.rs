// Generated from complete owned Rust CST declarations. Do not edit by hand.
use super::program::Name;
use super::results::Tree;
use serde_json::Value;
use std::rc::Rc;
#[derive(Clone, Debug)]
pub(super) struct AcceptedSelection {
    pub(super) language: Name,
    pub(super) root: Rc<Tree>,
    pub(super) program: usize,
    pub(super) provenance: Value,
}

#[derive(Clone, Debug)]
pub(super) struct SelectionMetadata {
    pub(super) mode: Name,
    pub(super) mapping: Name,
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) candidates: Vec<Value>,
    pub(super) provenance: Value,
    pub(super) alternatives: Vec<AcceptedSelection>,
    pub(super) ambiguous: bool,
    pub(super) source_mapping: Option<Value>,
    pub(super) source_bytes: Option<Rc<[u8]>>,
}
// Bind every copied declaration without relaxing native lints.
const _: fn() = || {
    let _ = |value: &self::AcceptedSelection| {
        let _ = (
            &value.language,
            &value.root,
            &value.program,
            &value.provenance,
        );
    };
    let _ = |value: &self::SelectionMetadata| {
        let _ = (
            &value.mode,
            &value.mapping,
            &value.start,
            &value.end,
            &value.candidates,
            &value.provenance,
            &value.alternatives,
            &value.ambiguous,
            &value.source_mapping,
            &value.source_bytes,
        );
    };
};
