//! Prove the source regions of the actual compiler-owned Vec fold packet.
use std::{collections::HashSet, rc::Rc};

use super::{Environment, Expr, Node, Position, RegionError, Regions, Result, Value, fail, rules};
use crate::translation::{
    ir::{Case, LitValue, Pattern},
    source_loop_folds::RustSourceLoopFold,
    types::Type,
};

#[derive(PartialEq, Eq)]
pub(super) struct FoldFrame {
    facet: RustSourceLoopFold,
    packet_type: Type,
}

#[derive(Clone)]
pub(super) struct FoldControl {
    frame: Rc<FoldFrame>,
    variants: Vec<(String, Option<Value>)>,
}

impl Regions<'_> {
    pub(super) fn loop_match(
        &mut self,
        source: Value,
        cases: &[Case],
        env: &Environment,
        bindings: &[String],
        expected: Option<&Position>,
        depth: usize,
    ) -> Result<Value> {
        let mut source = self.read(source, "move")?;
        let control = source
            .control
            .take()
            .ok_or_else(|| fail("a source loop match needs its actual compiler control"))?;
        let Type::Data { name } = &control.frame.facet.control else {
            return Err(fail(
                "a source loop match needs its actual compiler declaration",
            ));
        };
        let count = self.states.len();
        self.branches(count)?;
        let base = self.states.clone();
        let storage_base = self.storages.clone();
        let mut arms = HashSet::new();
        let mut outcomes = Vec::new();
        for arm in cases {
            let Pattern::Ctor { data, ctor, binds } = &arm.pattern else {
                return Err(fail(
                    "source loop control matching retains complete nominal constructor patterns",
                ));
            };
            if data != name || !arms.insert(ctor.as_str()) {
                return Err(fail(
                    "source loop control matching retains its distinct actual variants",
                ));
            }
            let Some((_, payload)) = control.variants.iter().find(|(variant, _)| variant == ctor)
            else {
                continue;
            };
            if binds.len() != usize::from(payload.is_some()) {
                return Err(fail(
                    "a source loop return pattern retains its actual payload binding",
                ));
            }
            self.branches(count)?;
            self.states[..count].clone_from_slice(&base);
            self.storages = storage_base.clone();
            let mut local = env.clone();
            let start = self.temporaries.len();
            let bound = if let (Some(payload), [Some(parameter)]) = (payload, binds.as_slice()) {
                let mut value = payload.clone();
                value.root = None;
                value.producer = true;
                Some((parameter, self.bind(&mut local, parameter, value)?))
            } else {
                None
            };
            let result = match self.expression(&arm.body, &local, bindings, expected, depth + 1) {
                Ok(value) => value,
                Err(RegionError::Diverged) => continue,
                Err(error) => return Err(error),
            };
            let result = self.read(result, "move")?;
            if let Some((parameter, root)) = bound {
                self.expire_binding(root, &local[parameter]);
            }
            let result = self.finish(result, start)?;
            outcomes.push(self.branch_outcome(result)?);
        }
        if !control
            .variants
            .iter()
            .all(|(name, _)| arms.contains(name.as_str()))
        {
            return Err(fail(
                "a source loop control match covers each actual reachable variant",
            ));
        }
        self.join_branch_outcomes(count, storage_base, outcomes)
    }

    pub(super) fn packet_frame(&self, ty: &Type) -> Option<Rc<FoldFrame>> {
        self.folds
            .iter()
            .rfind(|frame| frame.packet_type == *ty)
            .cloned()
    }

    pub(super) fn loop_packet(
        &mut self,
        frame: Rc<FoldFrame>,
        node: &Expr,
        items: &[Expr],
        env: &Environment,
        bindings: &[String],
        depth: usize,
    ) -> Result<Value> {
        if node.source_position.is_some()
            || node.source_state_transition.is_some()
            || items.len() != frame.facet.slots.len() + 2
        {
            return Err(fail(
                "a loop packet retains every actual compiler control and state slot",
            ));
        }
        let Node::Lit {
            value: LitValue::Bool(done),
        } = &items[0].node
        else {
            return Err(fail(
                "a new loop packet retains its actual literal done flag",
            ));
        };
        let mut fields = Vec::with_capacity(items.len());
        for (index, item) in items.iter().enumerate() {
            let position = frame
                .facet
                .slots
                .get(index.wrapping_sub(2))
                .map(|slot| &slot.position);
            let actual = self.expression(item, env, bindings, position, depth + 1)?;
            if index == 1 {
                let control = actual
                    .control
                    .as_ref()
                    .ok_or_else(|| fail("a loop packet needs its actual nominal control value"))?;
                if control.frame != frame
                    || !*done
                        && control
                            .variants
                            .iter()
                            .any(|(name, _)| name != "StatementContinuation")
                {
                    return Err(fail(
                        "every continuing loop packet retains its actual continuation control",
                    ));
                }
            }
            let mut held = self.read(actual, if index >= 2 { "transport" } else { "move" })?;
            held.root = Some(self.root(String::new(), false)?);
            held.producer = true;
            if let Some(storage) = held.storage.as_mut() {
                storage.owns_scope = false;
                storage.transport = true;
            }
            fields.push(held);
        }
        let value = self.create_packet(frame.packet_type.clone(), fields, Some(frame.clone()))?;
        self.verify_loop_slots(&frame, &value)?;
        Ok(value)
    }

    pub(super) fn loop_constructor(
        &mut self,
        node: &Expr,
        name: &str,
        args: &[Expr],
        env: &Environment,
        bindings: &[String],
        depth: usize,
    ) -> Result<Option<Value>> {
        let Some(frame) = self
            .folds
            .iter()
            .rfind(|frame| frame.facet.control == node.ty)
            .cloned()
        else {
            return Ok(None);
        };
        if node.source_position.is_some() || node.source_state_transition.is_some() {
            return Err(fail(
                "a compiler loop control has its own exact nominal position",
            ));
        }
        let payload = match (name, args) {
            ("FunctionResult", [value]) => {
                let actual = self.expression(value, env, bindings, Some(&frame.facet.result), depth + 1)?;
                Some(self.read(actual, "move")?)
            }
            (_, []) if name != "FunctionResult" && super::super::rust_owned_record_loop_rules::read_rust_owned_vector_source_fold_control_variants().iter().any(|variant| variant == name) => None,
            _ => return Err(fail("a loop control retains its actual compiler variant and payload")),
        };
        let regions = payload
            .as_ref()
            .map(|value| value.regions.clone())
            .unwrap_or_default();
        let root = self.root(String::new(), true)?;
        let mut value = Value::new(None, Some(root), regions, true, None);
        value.role = "source loop control".into();
        value.control = Some(FoldControl {
            frame,
            variants: vec![(name.into(), payload)],
        });
        Ok(Some(value))
    }

    pub(super) fn read_loop_control(&mut self, mut value: Value, mode: &str) -> Result<Value> {
        if matches!(mode, "move" | "transport") {
            if value.root.is_none() && !value.producer {
                return Err(fail("a loop control needs its actual available producer"));
            }
            if let Some(root) = value.root.take() {
                self.states[root] = rules::consume_rust_borrowed_source_region(&self.states[root]);
            }
            value.producer = true;
        }
        Ok(value)
    }

    pub(super) fn join_loop_controls(
        &mut self,
        mut left: Value,
        mut right: Value,
    ) -> Result<Value> {
        let (Some(mut left_control), Some(right_control)) =
            (left.control.take(), right.control.take())
        else {
            return Err(fail(
                "source loop control alternatives retain their common compiler declaration",
            ));
        };
        if left_control.frame != right_control.frame {
            return Err(fail(
                "source loop control alternatives retain their common compiler declaration",
            ));
        }
        for (name, payload) in right_control.variants {
            if let Some((_, current)) = left_control
                .variants
                .iter_mut()
                .find(|(variant, _)| variant == &name)
            {
                *current = match (current.take(), payload) {
                    (Some(a), Some(b)) => Some(self.join_values(a, b)?),
                    (None, None) => None,
                    _ => {
                        return Err(fail(
                            "a loop control variant retains its actual payload arity",
                        ));
                    }
                };
            } else {
                left_control.variants.push((name, payload));
            }
        }
        left.regions = self.union([left.regions, right.regions])?;
        left.root = None;
        left.producer = true;
        left.control = Some(left_control);
        self.read(left, "read")
    }

    fn verify_loop_slots(&mut self, frame: &Rc<FoldFrame>, value: &Value) -> Result<()> {
        let packet = value
            .packet
            .as_ref()
            .filter(|packet| packet.loop_frame.as_ref() == Some(frame))
            .ok_or_else(|| fail("a source fold publishes its actual state packet"))?;
        if packet.fields.len() != frame.facet.slots.len() + 2 {
            return Err(fail("a source fold publishes every actual state slot"));
        }
        self.read(value.clone(), "read")?;
        for (slot, field) in frame.facet.slots.iter().zip(&packet.fields[2..]) {
            let storage = self
                .storages
                .get(&slot.binder.identity)
                .ok_or_else(|| fail("a loop slot retains its actual closed storage"))?;
            if storage.binder != slot.binder
                || storage.position.as_ref() != &slot.position
                || !field.storage.as_ref().is_some_and(|held| {
                    held.identity == slot.binder.identity && held.revision == storage.revision
                })
                || !field.regions.is_empty()
            {
                return Err(fail(
                    "every loop slot publishes its actual current closed source revision",
                ));
            }
            self.compatible(field.clone(), Some(&slot.position))?;
        }
        Ok(())
    }

    pub(super) fn loop_expression(
        &mut self,
        node: &Expr,
        facet: &RustSourceLoopFold,
        env: &Environment,
        bindings: &[String],
        expected: Option<&Position>,
        depth: usize,
    ) -> Result<Value> {
        if !self.composed
            || expected.is_some()
            || node.source_position.is_some()
            || node.source_state_transition.is_some()
            || self.folds.len() >= 128
        {
            return Err(fail(
                "an owned source fold needs its actual composed packet position",
            ));
        }
        let facet = crate::translation::source_loop_folds::check_loop_fold(
            facet,
            node,
            "Rust",
            |ty| Ok(ty.clone()),
            |ty| match ty {
                Type::Data { name } => self.data(name),
                _ => None,
            },
        )?;
        let Node::Method { object, args, .. } = &node.node else {
            return Err(fail("a source fold needs its actual checked reduce"));
        };
        let [Some(callback), Some(initial)] = args.as_slice() else {
            return Err(fail(
                "a source fold needs its actual checked callback and initial packet",
            ));
        };
        let Node::Lambda { params, body } = &callback.node else {
            return Err(fail("a source fold needs its actual checked callback"));
        };
        let [accumulator, parameter] = params.as_slice() else {
            return Err(fail(
                "a source fold needs its actual checked callback formals",
            ));
        };
        let Node::If {
            otherwise: active, ..
        } = &body.node
        else {
            return Err(fail(
                "a source fold needs its actual checked completed guard",
            ));
        };
        for position in [&facet.input, &facet.item]
            .into_iter()
            .chain(facet.slots.iter().map(|slot| &slot.position))
        {
            self.scope(position, bindings)?;
            self.closed_target(position)?;
        }
        self.scope(&facet.result, bindings)?;
        let input = self.expression(object, env, bindings, Some(&facet.input), depth + 1)?;
        self.read(input, "move")?;
        let frame = Rc::new(FoldFrame {
            facet: facet.clone(),
            packet_type: node.ty.clone(),
        });
        self.folds.push(frame.clone());
        let result = (|| {
            let zero = self.expression(initial, env, bindings, None, depth + 1)?;
            self.verify_loop_slots(&frame, &zero)?;
            let count = self.states.len();
            self.branches(count)?;
            let base = self.states.clone();
            let storage_base = self.storages.clone();
            let mut local = env.clone();
            let mut held = zero.clone();
            held.root = None;
            held.producer = true;
            let accumulator_root = self.bind(&mut local, &accumulator.name, held)?;
            let item = self.position(&facet.item)?;
            let item = Value::new(Some(item), None, Vec::new(), true, None);
            let item_root = self.bind(&mut local, &parameter.name, item)?;
            let one = match self.expression(active, &local, bindings, None, depth + 1) {
                Ok(value) => value,
                Err(RegionError::Diverged) => {
                    self.states[..count].clone_from_slice(&base);
                    self.storages = storage_base;
                    self.expire_binding(accumulator_root, &local[&accumulator.name]);
                    self.expire_binding(item_root, &local[&parameter.name]);
                    return self.read(zero, "read");
                }
                Err(error) => return Err(error),
            };
            let one = self.read(one, "move")?;
            self.verify_loop_slots(&frame, &one)?;
            self.expire_binding(accumulator_root, &local[&accumulator.name]);
            self.expire_binding(item_root, &local[&parameter.name]);
            self.read(one.clone(), "read")?;

            let carried: HashSet<_> = facet
                .slots
                .iter()
                .map(|slot| slot.binder.identity.as_str())
                .collect();
            if storage_base.len() != self.storages.len()
                || storage_base.iter().any(|(identity, storage)| {
                    !carried.contains(identity.as_str())
                        && self.storages.get(identity).map(|after| after.revision)
                            != Some(storage.revision)
                })
            {
                return Err(fail(
                    "iteration-local and noncarried storage cannot change across loop iterations",
                ));
            }
            let carried_roots: HashSet<_> = storage_base
                .iter()
                .filter(|(identity, _)| carried.contains(identity.as_str()))
                .map(|(_, storage)| storage.revision)
                .collect();
            if (0..count)
                .any(|root| !carried_roots.contains(&root) && base[root] != self.states[root])
            {
                return Err(fail(
                    "every repeated loop capture retains its actual ownership and loan epochs",
                ));
            }

            self.branches(self.states.len())?;
            let step = self.states.clone();
            let storage_step = self.storages.clone();
            self.merge_state_prefix(count, &base, &step);
            self.join_storages(
                storage_base.clone(),
                (&storage_base, &base),
                (&storage_step, &step),
            )?;
            let joined = self.join_values(zero, one)?;
            self.verify_loop_slots(&frame, &joined)?;
            Ok(joined)
        })();
        self.folds.pop();
        result
    }
}
