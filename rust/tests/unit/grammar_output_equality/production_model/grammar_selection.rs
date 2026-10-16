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
}
