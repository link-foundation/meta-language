//! Checked tuple transport and branch joins for actual mutable source binders.
use super::{
    Environment, Expr, Position, RegionError, Regions, Result, Storage, Value, fail, rules,
    same_owner,
};
use crate::translation::types::Type;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub(super) struct StatePacket {
    pub(super) ty: Type,
    pub(super) fields: Vec<Value>,
    pub(super) loop_frame: Option<std::rc::Rc<super::folds::FoldFrame>>,
}

pub(super) struct BranchOutcome {
    pub(super) states: Vec<String>,
    pub(super) storages: HashMap<String, Storage>,
    pub(super) value: Value,
}

impl Regions<'_> {
    pub(super) fn read_state_packet(&mut self, mut value: Value, mode: &str) -> Result<Value> {
        let packet = value.packet.as_ref().expect("selected state packet");
        if matches!(mode, "move" | "transport") || packet.loop_frame.is_some() {
            for field in &packet.fields {
                self.read(field.clone(), "read")?;
            }
        }
        if !matches!(mode, "move" | "transport") {
            return Ok(value);
        }
        if value.root.is_none() && !value.producer {
            return Err(fail("a whole packet needs its actual available producer"));
        }
        if let Some(id) = value.root.take() {
            self.states[id] = rules::consume_rust_borrowed_source_region(&self.states[id]);
        }
        value.producer = true;
        Ok(value)
    }

    pub(super) fn state_packet(
        &mut self,
        node: &Expr,
        items: &[Expr],
        environment: &Environment,
        bindings: &[String],
        depth: usize,
    ) -> Result<Value> {
        if let Some(frame) = self.packet_frame(&node.ty) {
            return self.loop_packet(frame, node, items, environment, bindings, depth);
        }
        if !self.composed
            || items.is_empty()
            || items.len() > 513
            || node.source_position.is_some()
            || node.source_state_transition.is_some()
        {
            return Err(fail(
                "a state packet needs its actual complete direct result/state tuple type",
            ));
        }
        let Type::Tuple { elements } = &node.ty else {
            return Err(fail("a state packet needs its actual complete tuple type"));
        };
        if elements.len() != items.len() {
            return Err(fail(
                "a state packet retains every actual result and state slot",
            ));
        }
        let mut fields = Vec::with_capacity(items.len());
        let mut identities = HashSet::new();
        for (index, item) in items.iter().enumerate() {
            let actual = self.expression(item, environment, bindings, None, depth + 1)?;
            if actual.packet.is_some() || item.ty != elements[index] {
                return Err(fail(
                    "a direct state packet retains only its actual scalar or nominal slot types",
                ));
            }
            let transported = index > 0
                || actual
                    .storage
                    .as_ref()
                    .is_some_and(|storage| storage.transport);
            if index > 0
                && !actual
                    .storage
                    .as_ref()
                    .is_some_and(|storage| identities.insert(storage.identity.clone()))
            {
                return Err(fail(
                    "each published state slot retains its distinct initialized source binder and current revision",
                ));
            }
            let mut held = self.read(actual, if transported { "transport" } else { "move" })?;
            held.root = Some(self.root(String::new(), false)?);
            held.producer = true;
            if let Some(storage) = held.storage.as_mut() {
                storage.owns_scope &= index == 0;
                storage.transport = transported;
            }
            fields.push(held);
        }
        self.create_packet(node.ty.clone(), fields, None)
    }

    pub(super) fn create_packet(
        &mut self,
        ty: Type,
        fields: Vec<Value>,
        loop_frame: Option<std::rc::Rc<super::folds::FoldFrame>>,
    ) -> Result<Value> {
        let regions = self.union(fields.iter().map(|field| field.regions.clone()))?;
        let mut value = Value::new(
            None,
            Some(self.root(String::new(), true)?),
            regions,
            true,
            None,
        );
        value.role = "source state packet".into();
        value.packet = Some(StatePacket {
            ty,
            fields,
            loop_frame,
        });
        Ok(value)
    }

    pub(super) fn read_packet_field(
        &mut self,
        value: Value,
        index: usize,
        expected: &Type,
    ) -> Result<Value> {
        let packet = value
            .packet
            .ok_or_else(|| fail("a state projection needs its actual packet"))?;
        let Type::Tuple { elements } = &packet.ty else {
            return Err(fail("a state projection needs its actual tuple type"));
        };
        if elements.get(index) != Some(expected) {
            return Err(fail(
                "a state projection retains its actual complete packet and slot type",
            ));
        }
        let field = packet
            .fields
            .get(index)
            .cloned()
            .ok_or_else(|| fail("a state projection retains its actual slot"))?;
        let transported = index >= if packet.loop_frame.is_some() { 2 } else { 1 }
            || field
                .storage
                .as_ref()
                .is_some_and(|storage| storage.transport);
        let mut held = self.read(field, if transported { "transport" } else { "move" })?;
        held.root = None;
        held.producer = true;
        Ok(held)
    }

    pub(super) fn conditional(
        &mut self,
        condition: &Expr,
        alternatives: [&Expr; 2],
        environment: &Environment,
        bindings: &[String],
        expected: Option<&Position>,
        depth: usize,
    ) -> Result<Value> {
        let condition_value = self.expression(condition, environment, bindings, None, depth + 1)?;
        let condition_value = self.read(condition_value, "read")?;
        if condition.ty != Type::Bool
            || !matches!(condition_value.role.as_str(), "copy" | "inferred numeric")
        {
            return Err(fail(
                "a state conditional uses its actual scalar Boolean condition",
            ));
        }

        let count = self.states.len();
        self.branches(count)?;
        let previous = self.states.clone();
        let storage_base = self.storages.clone();
        let mut alternatives_checked = Vec::with_capacity(alternatives.len());
        for branch in alternatives {
            self.branches(count)?;
            self.states[..count].clone_from_slice(&previous);
            self.storages = storage_base.clone();
            let start = self.temporaries.len();
            let result = match self.expression(branch, environment, bindings, expected, depth + 1) {
                Ok(value) => value,
                Err(RegionError::Diverged) => continue,
                Err(error) => return Err(error),
            };
            let result = self.read(result, "move")?;
            let result = self.finish(result, start)?;
            alternatives_checked.push(self.branch_outcome(result)?);
        }
        self.join_branch_outcomes(count, storage_base, alternatives_checked)
    }

    pub(super) fn branch_outcome(&mut self, value: Value) -> Result<BranchOutcome> {
        self.branches(self.states.len())?;
        Ok(BranchOutcome {
            states: self.states.clone(),
            storages: self.storages.clone(),
            value,
        })
    }

    pub(super) fn join_branch_outcomes(
        &mut self,
        count: usize,
        base: HashMap<String, Storage>,
        outcomes: Vec<BranchOutcome>,
    ) -> Result<Value> {
        let mut outcomes = outcomes.into_iter().peekable();
        let Some(mut joined) = outcomes.next() else {
            return Err(RegionError::Diverged);
        };
        if outcomes.peek().is_none() {
            if !base
                .keys()
                .all(|identity| joined.storages.contains_key(identity))
                || base.len() != joined.storages.len()
            {
                return Err(fail(
                    "state branches retain exactly the actual enclosing storage roots",
                ));
            }
            self.states[..joined.states.len()].clone_from_slice(&joined.states);
            self.storages = joined.storages;
            return self.read(joined.value, "read");
        }
        while let Some(right) = outcomes.next() {
            self.merge_state_prefix(count, &joined.states, &right.states);
            self.join_storages(
                base.clone(),
                (&joined.storages, &joined.states),
                (&right.storages, &right.states),
            )?;
            let value = self.join_values(joined.value, right.value)?;
            if outcomes.peek().is_none() {
                return Ok(value);
            }
            joined = self.branch_outcome(value)?;
        }
        unreachable!("at least the first two source outcomes were present")
    }

    pub(super) fn merge_state_prefix(&mut self, count: usize, left: &[String], right: &[String]) {
        for index in 0..count {
            self.states[index] =
                rules::merge_rust_borrowed_source_region(&left[index], &right[index]);
        }
    }

    pub(super) fn join_storages(
        &mut self,
        base: HashMap<String, Storage>,
        (left, left_states): (&HashMap<String, Storage>, &[String]),
        (right, right_states): (&HashMap<String, Storage>, &[String]),
    ) -> Result<()> {
        if base.len() != left.len() || base.len() != right.len() {
            return Err(fail(
                "state branches retain exactly the actual enclosing storage roots",
            ));
        }
        let mut joined = base;
        for (identity, original) in &mut joined {
            let same_storage = |storage: &&Storage| {
                storage.binder == original.binder && storage.position == original.position
            };
            let a = left
                .get(identity)
                .filter(same_storage)
                .ok_or_else(|| fail("a source branch changes its actual enclosing binder"))?;
            let b = right
                .get(identity)
                .filter(same_storage)
                .ok_or_else(|| fail("a source branch changes its actual enclosing binder"))?;
            if a.revision == b.revision {
                original.revision = a.revision;
            } else {
                let state = rules::merge_rust_borrowed_source_region(
                    &left_states[a.revision],
                    &right_states[b.revision],
                );
                let joined_revision = self.root(String::new(), false)?;
                self.states[joined_revision] = state;
                original.revision = joined_revision;
                for old in [a.revision, b.revision] {
                    self.states[old] =
                        rules::consume_rust_borrowed_source_region(&self.states[old]);
                }
            }
        }
        self.storages = joined;
        Ok(())
    }

    pub(super) fn join_values(&mut self, left: Value, right: Value) -> Result<Value> {
        let mut left = self.resolved(left);
        let mut right = self.resolved(right);
        if left.role != right.role {
            return Err(fail(
                "source branch results need their complete common position",
            ));
        }
        if left.control.is_some() || right.control.is_some() {
            return self.join_loop_controls(left, right);
        }
        match (left.packet.take(), right.packet.take()) {
            (Some(left), Some(right))
                if left.ty == right.ty
                    && left.fields.len() == right.fields.len()
                    && left.loop_frame == right.loop_frame =>
            {
                let fields = left
                    .fields
                    .into_iter()
                    .zip(right.fields)
                    .map(|(a, b)| {
                        let mut field = self.join_values(a, b)?;
                        field.root = Some(self.root(String::new(), false)?);
                        Ok(field)
                    })
                    .collect::<Result<Vec<_>>>()?;
                self.create_packet(left.ty, fields, left.loop_frame)
            }
            (None, None) => {
                let right_position = right.position.as_deref();
                if !(left.position.is_none() && right_position.is_none()
                    || same_owner(left.position.as_deref(), right_position))
                {
                    return Err(fail(
                        "source branch results need their complete common position",
                    ));
                }
                let mut result = left;
                result.root = None;
                result.producer = true;
                result.regions = self.union([result.regions, right.regions])?;
                match (&mut result.storage, right.storage) {
                    (Some(a), Some(b))
                        if a.identity == b.identity
                            && a.owns_scope == b.owns_scope
                            && a.transport == b.transport =>
                    {
                        a.revision = self
                            .storages
                            .get(&a.identity)
                            .ok_or_else(|| {
                                fail(
                                    "a joined state carrier retains its surviving original storage",
                                )
                            })?
                            .revision;
                    }
                    (None, None) => {}
                    _ => {
                        return Err(fail(
                            "state branches retain their actual common source binder",
                        ));
                    }
                }
                self.read(result, "read")
            }
            _ => Err(fail("state branch joins retain their exact packet slots")),
        }
    }
}
