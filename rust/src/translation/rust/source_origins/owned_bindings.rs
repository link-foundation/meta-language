//! Identity allocation and snapshots; state decisions are generated shared rules.
use super::{Result, SExpr, SNode, SourceOrigin, SourceOrigins, Span, Type, unsupported};
use crate::translation::rust_owned_binding_rules as rules;
use std::collections::{HashMap, HashSet};
impl SourceOrigins {
    pub(in super::super) fn track_owned_bindings(&self) -> bool {
        rules::accept_rust_owned_binding_tracking(
            self.collecting,
            self.owned_binding_excluded
                || self.owned_local_values
                || self.shared_array_handles
                || !self.array_result_model.is_empty(),
        )
    }
    pub(in super::super) fn owned_binding_value(&self, origin: &SourceOrigin) -> bool {
        self.track_owned_bindings()
            && rules::accept_rust_owned_binding_value(
                origin.ty.as_ref().map_or("", Type::kind),
                origin
                    .array_source_model
                    .or(origin.text_source_model)
                    .unwrap_or(""),
                origin.wrappers.len() as f64,
                origin.proven,
            )
    }
    pub(in super::super) fn check_owned_origin(
        &self,
        origin: &SourceOrigin,
        span: Option<Span>,
    ) -> Result<()> {
        if !self.track_owned_bindings() {
            return Ok(());
        }
        let identities = rules::project_rust_owned_loan_bindings(
            origin.source_binding_identity.as_deref().unwrap_or(""),
            &origin.source_loan_binding_identities,
        );
        let states = identities
            .iter()
            .map(|id| {
                self.source_binding_states
                    .get(id)
                    .cloned()
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>();
        let error = rules::read_rust_owned_loan_read_error(&states);
        if !error.is_empty() {
            return Err(unsupported("Rust owned binding", &error, span));
        }
        Ok(())
    }
    pub(in super::super) fn register_owned_binding(
        &mut self,
        mut origin: SourceOrigin,
    ) -> SourceOrigin {
        if self.owned_binding_value(&origin) {
            let id = self.next_source_binding_identity.to_string();
            self.next_source_binding_identity += 1;
            origin.source_binding_identity = Some(id.clone());
            origin.source_extending_temporary_bindings.clear();
            origin.source_place_binding_identity = None;
            origin.source_loan_binding_identities.clear();
            origin.source_place_loan_binding_identities.clear();
            self.source_binding_states
                .insert(id, "available".to_owned());
        }
        origin
    }
    pub(in super::super) fn available_owned_binding(&self, origin: &SourceOrigin) -> bool {
        self.owned_binding_value(origin)
            && origin.source_binding_identity.as_ref().is_some_and(|id| {
                rules::read_rust_owned_binding_read_error(
                    self.source_binding_states
                        .get(id)
                        .map_or("", String::as_str),
                )
                .is_empty()
            })
    }
    pub(in super::super) fn consume_owned_binding(
        &mut self,
        origin: &SourceOrigin,
        span: Option<Span>,
    ) -> Result<bool> {
        if !self.owned_binding_value(origin) || origin.source_binding_identity.is_none() {
            return Ok(false);
        }
        self.check_owned_origin(origin, span)?;
        let id = origin.source_binding_identity.as_ref().unwrap();
        let state =
            rules::consume_rust_owned_binding_state(self.source_binding_states.get(id).unwrap());
        self.source_binding_states.insert(id.clone(), state);
        Ok(true)
    }
    pub(in super::super) fn check_owned_name(&self, node: &SExpr, path: &[String]) -> Result<()> {
        if let SNode::Name { path: names } = &node.node {
            if names.len() == 1 {
                if let Some(origin) = self.scope.get(&names[0]) {
                    self.check_owned_origin(origin, node.span)?;
                }
            }
        }
        let _ = path;
        Ok(())
    }
    pub(in super::super) fn transfer_owned_result(
        &mut self,
        node: &SExpr,
        path: &[String],
    ) -> Result<SourceOrigin> {
        let origin = self.origin(node, path);
        if !self.owned_binding_value(&origin)
            || !rules::accept_rust_owned_result_transfer(
                origin.source_binding_identity.is_some(),
                self.available_owned_binding(&origin),
                origin.source_owned_result_proof,
                origin.fresh_array_value.is_some()
                    || node
                        .span
                        .is_some_and(|place| self.fresh_vector_constructors.contains(&place)),
            )
        {
            return Ok(origin);
        }
        self.consume_owned_binding(&origin, node.span)?;
        let mut result = self.register_owned_binding(origin);
        result.source_owned_result_proof = true;
        if let Some(id) = result.source_binding_identity.as_ref() {
            self.source_temporary_bindings.insert(id.clone());
        }
        self.mark(node, result.clone());
        Ok(result)
    }
    pub(in super::super) fn owned_temporary_frame(&self) -> HashSet<String> {
        self.source_temporary_bindings.clone()
    }
    pub(in super::super) fn capture_owned_borrow(&mut self, origin: SourceOrigin) -> SourceOrigin {
        if !self.owned_binding_value(&origin) || origin.source_binding_identity.is_some() {
            return origin;
        }
        let result = self.register_owned_binding(origin);
        if let Some(id) = result.source_binding_identity.as_ref() {
            self.source_temporary_bindings.insert(id.clone());
        }
        result
    }
    pub(in super::super) fn finish_owned_expression(
        &mut self,
        node: &SExpr,
        path: &[String],
        frame: &HashSet<String>,
        extending: bool,
    ) {
        let origin = self.origin(node, path);
        for id in self.source_temporary_bindings.clone() {
            let Some(state) = self.source_binding_states.get(&id).cloned() else {
                continue;
            };
            let disposition = rules::read_rust_owned_temporary_disposition(
                frame.contains(&id),
                extending && origin.source_extending_temporary_bindings.contains(&id),
                self.owned_binding_value(&origin)
                    && origin.source_binding_identity.as_ref() == Some(&id),
            );
            if disposition == "extend" {
                self.source_temporary_bindings.remove(&id);
            } else if disposition == "expire" {
                self.source_binding_states
                    .insert(id.clone(), rules::expire_rust_owned_binding_state(&state));
                self.source_temporary_bindings.remove(&id);
            }
        }
    }
    pub(in super::super) fn prepare_owned_argument(
        &mut self,
        node: &SExpr,
        parameter: Option<&SourceOrigin>,
        path: &[String],
    ) -> Result<()> {
        if !parameter.is_some_and(|value| self.owned_binding_value(value)) {
            return Ok(());
        }
        let origin = self.origin(node, path);
        if self.consume_owned_binding(&origin, node.span)? {
            if let Some(place) = node.span {
                self.owned_argument_transfers.insert(place);
            }
        }
        Ok(())
    }
    pub(in super::super) fn expire_owned_bindings(
        &mut self,
        before: &HashMap<String, String>,
        result: &SourceOrigin,
    ) {
        let escaping = if self.owned_binding_value(result) {
            result.source_binding_identity.clone()
        } else {
            None
        };
        for (id, state) in &mut self.source_binding_states {
            if !before.contains_key(id)
                && escaping.as_ref() != Some(id)
                && !result.source_extending_temporary_bindings.contains(id)
            {
                *state = rules::expire_rust_owned_binding_state(state);
            }
        }
    }
    pub(in super::super) fn snapshot_owned_bindings(&self) -> HashMap<String, String> {
        self.source_binding_states.clone()
    }
    pub(in super::super) fn restore_owned_bindings(&mut self, snapshot: HashMap<String, String>) {
        self.source_binding_states = snapshot;
    }
    pub(in super::super) fn join_owned_bindings(
        &mut self,
        left: &HashMap<String, String>,
        right: &HashMap<String, String>,
        left_returns: bool,
        right_returns: bool,
    ) {
        let ids = left
            .keys()
            .chain(right.keys())
            .cloned()
            .collect::<HashSet<_>>();
        self.source_binding_states = ids
            .into_iter()
            .map(|id| {
                let state = rules::merge_rust_owned_binding_states(
                    left.get(&id).map_or("absent", String::as_str),
                    right.get(&id).map_or("absent", String::as_str),
                    left_returns,
                    right_returns,
                );
                (id, state)
            })
            .collect();
    }
    pub(in super::super) fn check_owned_guard(
        &self,
        before: &HashMap<String, String>,
        span: Option<Span>,
    ) -> Result<()> {
        let ids = before.keys().collect::<Vec<_>>();
        let left = ids
            .iter()
            .map(|id| before.get(*id).unwrap().clone())
            .collect::<Vec<_>>();
        let right = ids
            .iter()
            .map(|id| {
                self.source_binding_states
                    .get(*id)
                    .cloned()
                    .unwrap_or_else(|| "absent".to_owned())
            })
            .collect::<Vec<_>>();
        if !rules::accept_rust_owned_binding_guard(&left, &right) {
            return Err(unsupported(
                "Rust owned binding",
                "owned consumption in guards needs its complete fallthrough path proof",
                span,
            ));
        }
        Ok(())
    }
}
