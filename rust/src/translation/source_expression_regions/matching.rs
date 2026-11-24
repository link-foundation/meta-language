//! Retain the actual pattern bindings and branch regions of a nominal match.
use std::rc::Rc;

use super::{
    Environment, Position, RegionError, Regions, Result, Value, accept, fail, role, rules,
};
use crate::translation::{
    ir::{Case, Pattern},
    types::Type,
};

impl Regions<'_> {
    pub(super) fn nominal_match(
        &mut self,
        source: Value,
        cases: &[Case],
        env: &Environment,
        bindings: &[String],
        expected: Option<&Position>,
        depth: usize,
    ) -> Result<Value> {
        if cases.is_empty() {
            return Err(fail(
                "a source match requires its actual inhabited case inventory",
            ));
        }
        let owned = matches!(source.role.as_str(), "owned" | "box");
        let count = self.states.len();
        self.branches(count)?;
        let initial_states = self.states.clone();
        let initial_storage = self.storages.clone();
        let mut outcomes = Vec::with_capacity(cases.len());

        for arm in cases {
            self.branches(count)?;
            self.states[..count].clone_from_slice(&initial_states);
            self.storages = initial_storage.clone();
            let mut local = env.clone();
            let start = self.temporaries.len();
            let mut locals = Vec::new();
            let (pattern, fields, bind_count) = match &arm.pattern {
                Pattern::Ctor { data, ctor, binds } => {
                    let declaration = self.data(data).ok_or_else(|| {
                        fail("a source pattern needs its actual nominal constructor identity")
                    })?;
                    if !matches!(source.position.as_deref().map(|p| &p.ty), Some(Type::Data { name }) if name == data)
                    {
                        return Err(fail(
                            "a source pattern needs its actual nominal constructor identity",
                        ));
                    }
                    let index = declaration
                        .ctors
                        .iter()
                        .position(|item| &item.name == ctor)
                        .ok_or_else(|| {
                            fail("a source pattern needs its actual nominal constructor identity")
                        })?;
                    let fields = declaration
                        .source_field_signatures
                        .as_deref()
                        .and_then(|signature| signature.ctors.get(index))
                        .map(|item| item.fields.as_slice())
                        .unwrap_or_default();
                    ("ctor", fields, binds.len())
                }
                Pattern::Bind { .. } => ("bind", &[][..], 0),
                Pattern::Wild => ("wild", &[][..], 0),
                _ => ("unsupported", &[][..], 0),
            };
            let kind = source
                .position
                .as_deref()
                .map(|position| position.ty.kind())
                .unwrap_or("");
            let arity = fields.len() as f64;
            let binding_count = bind_count as f64;
            accept(if owned {
                rules::read_rust_borrowed_source_owned_match_error(
                    self.composed,
                    kind,
                    pattern,
                    arity,
                    binding_count,
                )
            } else {
                rules::read_rust_borrowed_source_match_error(
                    &source.role,
                    kind,
                    pattern,
                    arity,
                    binding_count,
                )
            })?;

            match &arm.pattern {
                Pattern::Bind { name } => {
                    locals.push((name, self.bind(&mut local, name, source.clone())?))
                }
                Pattern::Ctor { binds, .. } => {
                    if owned && source.role == "box" {
                        self.closed_target(source.position.as_deref().ok_or_else(|| {
                            fail("a source Box pattern retains its actual owner")
                        })?)?;
                    }
                    if owned
                        && binds.iter().zip(fields).any(|(name, field)| {
                            name.is_some() && !matches!(role(field).as_str(), "copy" | "reference")
                        })
                    {
                        self.read(source.clone(), "move")?;
                    }
                    for (name, field) in binds.iter().zip(fields) {
                        let Some(name) = name else {
                            continue;
                        };
                        let position = self.position(field)?;
                        let regions = if owned && role(field) == "copy" {
                            Vec::new()
                        } else {
                            source.regions.clone()
                        };
                        let position = if owned {
                            position
                        } else {
                            let mut shared = position.as_ref().clone();
                            shared.wrappers.insert(0, "reference".into());
                            let lifetime = source
                                .position
                                .as_deref()
                                .and_then(|position| position.lifetimes.first())
                                .cloned()
                                .flatten();
                            shared.lifetimes.insert(0, lifetime);
                            Rc::new(shared)
                        };
                        let value = Value::new(Some(position), None, regions, owned, None);
                        locals.push((name, self.bind(&mut local, name, value)?));
                    }
                }
                _ => {}
            }

            let result = match self.expression(&arm.body, &local, bindings, expected, depth + 1) {
                Ok(value) => value,
                Err(RegionError::Diverged) => continue,
                Err(error) => return Err(error),
            };
            let result = self.read(result, "move")?;
            for (name, root) in locals {
                self.expire_binding(root, &local[name]);
            }
            let result = self.finish(result, start)?;
            outcomes.push(self.branch_outcome(result)?);
        }
        self.join_branch_outcomes(count, initial_storage, outcomes)
    }
}
