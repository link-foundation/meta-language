//! Retain one actual call and transport its ordered explicit state results.

use super::{
    Environment, Layout, Lowered, OwnedRecordEffects, Result, RustParser, SExpr, Slot,
    SourceOrigin, Span, Transport, Value, decode, encode, fail, json, name, rules, text,
};
use crate::translation::{
    rust::owned_record_policy::state::plan_rust_owned_record_state_packet,
    surface::{SArrayItem, SNode},
};

#[derive(Clone)]
pub(in super::super) struct CallablePlace {
    pub(in super::super) parameter: usize,
    pub(in super::super) slot: Slot,
    pub(in super::super) layouts: Vec<Layout>,
    pub(in super::super) position: SourceOrigin,
}

pub(super) struct CallablePacket {
    parameters: Vec<SourceOrigin>,
    places: Vec<CallablePlace>,
    result: SourceOrigin,
}

impl CallablePacket {
    pub(super) fn slots(&self) -> impl Iterator<Item = &Slot> {
        self.places.iter().map(|place| &place.slot)
    }
}

impl RustParser {
    pub(in super::super) fn retain_callable_packet(
        &mut self,
        call: SExpr,
        parameters: Vec<SourceOrigin>,
        places: Vec<CallablePlace>,
        result: SourceOrigin,
    ) -> Result<SExpr> {
        let SNode::App { args, .. } = &call.node else {
            return Err(fail("a callable state packet needs its actual application"));
        };
        if parameters.len() != args.len()
            || places.is_empty()
            || places.iter().any(|place| place.parameter >= args.len())
            || places
                .windows(2)
                .any(|pair| pair[0].parameter >= pair[1].parameter)
        {
            return Err(fail(
                "a callable packet needs its complete ordered state parameters",
            ));
        }
        let effects = self
            .owned_record_state
            .effects
            .get_or_insert_with(OwnedRecordEffects::default);
        let index = effects.callable_packets.len();
        if index >= rules::read_rust_owned_record_effect_limit("work") as usize {
            return Err(fail(
                "callable state packets exceed their finite source budget",
            ));
        }
        effects.callable_packets.push(CallablePacket {
            parameters,
            places,
            result: result.clone(),
        });
        let span = call.span;
        let node = SExpr::new(
            SNode::Math {
                op: rules::read_rust_owned_record_effect_operation("call"),
                name: index.to_string(),
                args: vec![SArrayItem::new(call, false)],
            },
            span,
        );
        self.origins.mark(&node, result);
        Ok(node)
    }
}

impl Transport<'_> {
    fn callable_packet_position(&self, position: &SourceOrigin) -> Result<Value> {
        let ty = position
            .ty
            .as_ref()
            .ok_or_else(|| fail("a callable packet needs every actual source type"))?;
        encode(&self.parser.record_source_type(ty, &position.path))
    }

    pub(super) fn callable_packet(
        &mut self,
        mut node: Value,
        environment: Environment,
        depth: usize,
    ) -> Result<Lowered> {
        let index = text(&node, "name")
            .parse::<usize>()
            .map_err(|_| fail("invalid private callable packet"))?;
        if index.to_string() != text(&node, "name")
            || !node["args"]
                .as_array()
                .is_some_and(|args| args.len() == 1 && args[0]["spread"] == false)
        {
            return Err(fail(
                "a callable packet needs its actual application operand",
            ));
        }
        let mut call = node["args"][0]["value"].take();
        let packet = self
            .parser
            .owned_record_state
            .effects
            .as_ref()
            .and_then(|effects| effects.callable_packets.get(index))
            .ok_or_else(|| fail("unknown actual callable packet"))?;
        let (parameters, places, output) = (
            packet.parameters.clone(),
            packet.places.clone(),
            packet.result.clone(),
        );
        let span = node
            .get("span")
            .cloned()
            .map(serde_json::from_value::<Span>)
            .transpose()
            .map_err(|_| fail("a callable packet needs its actual source span"))?;
        let args = call["args"]
            .as_array_mut()
            .filter(|args| args.len() == parameters.len())
            .ok_or_else(|| fail("a callable packet needs its complete actual operands"))?;
        let mut lowered = Lowered::plain(Value::Null, environment);
        let mut next_place = places.iter().peekable();
        for (index, (operand, parameter)) in args.iter_mut().zip(&parameters).enumerate() {
            let ty = self.callable_packet_position(parameter)?;
            let value = if let Some(place) = next_place.next_if(|place| place.parameter == index) {
                let current = actual_root(&lowered.environment, &place.slot)?;
                encode(&self.parser.project_record_state(
                    &place.layouts,
                    decode(name(current))?,
                    span,
                ))?
            } else {
                let selected =
                    self.visit(operand.take(), lowered.environment, depth + 1, Some(&ty))?;
                lowered.bindings.extend(selected.bindings);
                lowered.environment = selected.environment;
                selected.value
            };
            if text(&value, "k") == "lambda" {
                *operand = value;
            } else {
                let temporary = self.parser.fresh_product_binding();
                lowered
                    .bindings
                    .push(json!({"k":"let","name":temporary,"type":ty,"value":value}));
                *operand = name(&temporary);
            }
        }
        let mut packet_types = vec![self.callable_packet_position(&output)?];
        for place in &places {
            packet_types.push(self.callable_packet_position(&place.position)?);
        }
        let plan = plan_rust_owned_record_state_packet(places.len() as f64);
        if plan.len() != packet_types.len() {
            return Err(fail(
                "a callable packet exceeds its complete state result bound",
            ));
        }
        let packet_name = self.parser.fresh_product_binding();
        let projection = |index: f64| json!({"k":"tupleIndex","object":name(&packet_name),"index":index as usize});
        lowered.bindings.push(json!({"k":"let","name":packet_name,"type":{"kind":"tuple","elements":packet_types},"value":call}));
        for (place, index) in places.into_iter().zip(plan.iter().skip(1)) {
            let current = actual_root(&lowered.environment, &place.slot)?;
            let value = self.parser.rebuild_record_state(
                &place.layouts,
                0,
                decode(name(current))?,
                decode(projection(*index))?,
                span,
            )?;
            let fresh = self.parser.fresh_product_binding();
            let value = self.state_transition(encode(&value)?, Some(&place.slot), "write")?;
            lowered
                .bindings
                .push(json!({"k":"let","name":fresh,"value":value}));
            lowered
                .environment
                .insert(place.slot.lexical.clone(), (place.slot, fresh));
        }
        lowered.value = projection(plan[0]);
        if let Some(position) = node.get("sourcePosition") {
            lowered.value["sourcePosition"] = position.clone();
        }
        Ok(lowered)
    }
}

fn actual_root<'a>(environment: &'a Environment, slot: &Slot) -> Result<&'a str> {
    environment
        .get(&slot.lexical)
        .filter(|(active, _)| active.root == slot.root)
        .map(|(_, current)| current.as_str())
        .ok_or_else(|| fail("a callable state operand needs its actual live lexical root"))
}
