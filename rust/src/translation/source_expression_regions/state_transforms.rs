//! Prove finite compiler helpers against the actual closed storage they carry.
use std::rc::Rc;

use super::{
    Environment, Expr, FnDecl, FoldFrame, Node, Position, Regions, Result, Storage, StorageValue,
    Value, fail, role, rules,
};
use crate::translation::{
    source_loop_folds::RustSourceFoldSlot, source_state_transforms::RustFiniteLoopStateTransform,
};

impl Regions<'_> {
    fn transform_result(&mut self, position: &Position) -> Result<Value> {
        let regions = match role(position).as_str() {
            "reference" => {
                self.closed_target(position)?;
                vec![0]
            }
            "borrowed vector" => {
                self.closed_target(position.array_element_position.as_deref().ok_or_else(
                    || fail("a borrowed compiler result needs its actual element position"),
                )?)?;
                vec![0]
            }
            _ => {
                self.closed_output(position)?;
                Vec::new()
            }
        };
        Ok(Value::new(
            Some(self.position(position)?),
            None,
            regions,
            true,
            None,
        ))
    }

    fn current_transform_slot(&self, slot: &RustSourceFoldSlot, value: &Value) -> Result<usize> {
        let identity = &slot.binder.identity;
        let storage = self
            .storages
            .get(identity)
            .ok_or_else(|| fail("compiler state storage retains its actual initialized binder"))?;
        if storage.binder != slot.binder
            || storage.position.as_ref() != &slot.position
            || !value.regions.is_empty()
            || !value.storage.as_ref().is_some_and(|actual| {
                &actual.identity == identity && actual.revision == storage.revision
            })
        {
            return Err(fail(
                "a compiler state slot retains its actual current closed revision",
            ));
        }
        Ok(storage.revision)
    }

    fn new_transform_slot(&mut self, slot: &RustSourceFoldSlot) -> Result<Value> {
        let position = self.position(&slot.position)?;
        let revision = self.root(String::new(), false)?;
        self.prepare_record_image(revision, &position)?;
        let identity = slot.binder.identity.clone();
        self.storages.insert(
            identity.clone(),
            Storage {
                binder: slot.binder.clone(),
                position: position.clone(),
                revision,
            },
        );
        let mut value = Value::new(Some(position), None, Vec::new(), true, None);
        value.storage = Some(StorageValue {
            identity,
            revision,
            owns_scope: false,
            transport: true,
        });
        Ok(value)
    }

    pub(super) fn state_transform_call(
        &mut self,
        node: &Expr,
        function: &FnDecl,
        facet: &RustFiniteLoopStateTransform,
        args: &[Expr],
        env: &Environment,
        bindings: &[String],
        depth: usize,
    ) -> Result<Value> {
        if !self.composed
            || args.len() != facet.slots.len()
            || node.ty != function.ret
            || node.source_position.is_some()
            || node.source_state_transition.is_some()
        {
            return Err(fail(
                "a compiler state call retains its actual complete result and arguments",
            ));
        }
        for (argument, slot) in args.iter().zip(&facet.slots) {
            if !matches!(&argument.node, Node::Var { .. }) {
                return Err(fail(
                    "compiler state operands retain their actual selected lexical bindings",
                ));
            }
            let value =
                self.expression(argument, env, bindings, Some(&slot.position), depth + 1)?;
            let revision = self.current_transform_slot(slot, &value)?;
            self.read(value, "transport")?;
            self.retire_field_lifetime(revision, "move");
            self.states[revision] =
                rules::consume_rust_borrowed_source_region(&self.states[revision]);
        }

        let result = self.transform_result(&facet.result)?;
        let mut output = if let Some(control) = &facet.control {
            self.make_loop_control(
                Rc::new(FoldFrame::finite(control, &facet.result)),
                vec![
                    ("StatementContinuation".into(), None),
                    ("FunctionResult".into(), Some(result)),
                ],
            )?
        } else {
            result
        };
        // The returned tuple owns its output until the enclosing source binding
        // projects it, independently of the lifetime of this call's temporaries.
        output.root = None;
        let mut fields = vec![output];
        for slot in &facet.slots {
            fields.push(self.new_transform_slot(slot)?);
        }
        self.create_packet(node.ty.clone(), fields, None)
    }

    pub(super) fn state_transform_body(
        &mut self,
        function: &FnDecl,
        facet: &RustFiniteLoopStateTransform,
    ) -> Result<()> {
        let mut env = Environment::new();
        for (parameter, slot) in function.params.iter().zip(&facet.slots) {
            self.closed_output(&slot.position)?;
            let value = self.new_transform_slot(slot)?;
            self.bind(&mut env, &parameter.name, value)?;
        }

        self.transform_result(&facet.result)?;
        let mut bindings = Vec::new();
        let mut position = Some(&facet.result);
        while let Some(current) = position {
            bindings.extend(
                current
                    .lifetimes
                    .iter()
                    .flatten()
                    .filter(|name| !matches!(name.as_str(), "static" | "_"))
                    .cloned(),
            );
            position = current.array_element_position.as_deref();
        }
        let frame = facet
            .control
            .as_ref()
            .map(|control| Rc::new(FoldFrame::finite(control, &facet.result)));
        self.folds.extend(frame.iter().cloned());
        let checked = self.expression(&function.body, &env, &bindings, None, 0);
        if frame.is_some() {
            self.folds.pop();
        }
        let result = self.read(checked?, "move")?;
        let packet = result
            .packet
            .as_ref()
            .filter(|packet| {
                packet.ty == function.ret && packet.fields.len() == facet.slots.len() + 1
            })
            .ok_or_else(|| {
                fail("a compiler state body returns its exact actual result and state tuple")
            })?;
        for (slot, value) in facet.slots.iter().zip(&packet.fields[1..]) {
            self.current_transform_slot(slot, value)?;
            let value = self.read(value.clone(), "read")?;
            self.compatible(value, Some(&slot.position))?;
        }
        if self.storages.len() != facet.slots.len() {
            return Err(fail(
                "compiler state local storage cannot escape its lexical body",
            ));
        }

        let output = &packet.fields[0];
        if let Some(frame) = frame {
            let control = output
                .control
                .as_ref()
                .filter(|value| value.frame == frame)
                .ok_or_else(|| {
                    fail("a compiler state body retains its actual enclosing completion type")
                })?;
            for (variant, payload) in &control.variants {
                match (variant.as_str(), payload) {
                    ("StatementContinuation", None) => {}
                    ("FunctionResult", Some(value)) => {
                        self.compatible(value.clone(), Some(&facet.result))?;
                        if value
                            .regions
                            .iter()
                            .any(|root| self.lifetimes[*root] != "static")
                        {
                            return Err(fail(
                                "closed compiler inputs cannot supply an escaping nonstatic result loan",
                            ));
                        }
                    }
                    _ => {
                        return Err(fail(
                            "a finite compiler state body carries only continuation or function result",
                        ));
                    }
                }
            }
        } else {
            self.compatible(output.clone(), Some(&facet.result))?;
        }
        Ok(())
    }
}
