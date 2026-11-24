//! Feed actual callable roots into the existing ordered completion transport.

use super::{
    Environment, Result, RustParser, Slot, Transport, Value, fail, json, name, text, wrap,
};

#[derive(Clone)]
pub(super) struct ParameterState {
    slot: Slot,
    publish: bool,
}

impl RustParser {
    pub(in super::super) fn retain_callable_state_parameter(
        &mut self,
        parameter: String,
        slot: Slot,
        publish: bool,
    ) -> Result<()> {
        let effects =
            self.owned_record_state.effects.as_mut().ok_or_else(|| {
                fail("a callable state parameter needs its actual ordered effects")
            })?;
        if effects.callable_state.contains_key(&parameter) {
            return Err(fail(
                "a callable state parameter needs its own generated name",
            ));
        }
        self.owned_record_state
            .declarations
            .insert(parameter.clone(), slot.clone());
        effects
            .callable_state
            .insert(parameter, ParameterState { slot, publish });
        Ok(())
    }
}

impl Transport<'_> {
    fn callable_state_parameter(&self, name: &str) -> Option<&ParameterState> {
        self.parser
            .owned_record_state
            .effects
            .as_ref()?
            .callable_state
            .get(name)
    }

    pub(super) fn has_callable_state_parameters(&self, node: &Value) -> bool {
        node["params"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|parameter| {
                self.callable_state_parameter(text(parameter, "name"))
                    .is_some()
            })
    }

    pub(super) fn rename_callable_state_parameter(&mut self, previous: &str, fresh: &str) {
        if let Some(state) = self.callable_state_parameter(previous).cloned()
            && let Some(effects) = &mut self.parser.owned_record_state.effects
        {
            self.declarations
                .insert(fresh.to_owned(), state.slot.clone());
            effects.callable_state.insert(fresh.to_owned(), state);
        }
    }

    pub(super) fn lower_callable_state_body(
        &mut self,
        node: &mut Value,
        depth: usize,
    ) -> Result<()> {
        let mut initial = Environment::new();
        let mut published = Vec::new();
        for parameter in node["params"].as_array().into_iter().flatten() {
            let name = text(parameter, "name");
            if let Some(state) = self.callable_state_parameter(name) {
                let slot = &state.slot;
                if initial
                    .insert(slot.lexical.clone(), (slot.clone(), name.to_owned()))
                    .is_some()
                {
                    return Err(fail(
                        "a callable state root must have one actual input parameter",
                    ));
                }
                if state.publish {
                    published.push((slot.lexical.clone(), slot.root.clone()));
                }
            }
        }
        let mut expected = node.get("ret").cloned();
        let packet = !initial.is_empty();
        if packet {
            let fields = expected
                .as_ref()
                .and_then(|ty| ty.get("elements"))
                .and_then(Value::as_array)
                .filter(|fields| fields.len() == 1 + published.len())
                .ok_or_else(|| {
                    fail("a callable completion needs its complete ordered result and state types")
                })?;
            expected = Some(fields[0].clone());
        }
        let result = self.visit(node["body"].take(), initial, depth, expected.as_ref())?;
        let value = if packet {
            let mut items = vec![result.value];
            for (lexical, root) in published {
                let (_, current) = result
                    .environment
                    .get(&lexical)
                    .filter(|(slot, _)| slot.root == root)
                    .ok_or_else(|| {
                        fail("a callable completion lost its actual surviving source root")
                    })?;
                items.push(name(current));
            }
            json!({"k":"tuple","items":items})
        } else {
            result.value
        };
        node["body"] = wrap(result.bindings, value);
        Ok(())
    }
}
