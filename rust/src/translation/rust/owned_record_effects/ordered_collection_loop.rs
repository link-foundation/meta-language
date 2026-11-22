//! State and completion transport for a single evaluation of an ordered head.
use crate::translation::source_signatures::RustSourceExpressionPosition;

use super::{
    Environment, Lowered, Result, Transport, Value, encode, fail, json, loop_control, loops, name,
    text, wrap,
};

fn project(binding: &str, index: usize) -> Value {
    json!({"k":"tupleIndex","object":name(binding),"index":index})
}

impl Transport<'_> {
    pub(super) fn ordered_collection_loop(
        &mut self,
        mut node: Value,
        environment: Environment,
        depth: usize,
    ) -> Result<Lowered> {
        let ordinal = text(&node, "name")
            .parse::<usize>()
            .ok()
            .filter(|ordinal| ordinal.to_string() == text(&node, "name"))
            .ok_or_else(|| fail("ordered iteration needs its actual source ordinal"))?;
        if !node["args"]
            .as_array()
            .is_some_and(|args| args.len() == 2 && args.iter().all(|arg| arg["spread"] == false))
        {
            return Err(fail("ordered iteration needs its actual input and body"));
        }
        let entry = self
            .parser
            .owned_record_state
            .effects
            .as_ref()
            .and_then(|effects| effects.loops.entries.get(ordinal))
            .ok_or_else(|| fail("ordered iteration needs its actual source inventory"))?;
        let source = entry
            .ordered
            .clone()
            .ok_or_else(|| fail("ordered iteration needs its actual admitted item and exits"))?;
        let forwarded = entry.destination.is_some();
        if source.returns != 0 && !forwarded {
            return Err(fail(
                "an ordered loop return needs its enclosing completion consumer",
            ));
        }

        let input_type = encode(&source.input_type)?;
        let item_type = encode(&source.item_type)?;
        let mut input = self.visit(
            node["args"][0]["value"].take(),
            environment,
            depth + 1,
            Some(&input_type),
        )?;
        let collection = self.parser.fresh_product_binding();
        let accumulator = self.parser.fresh_product_binding();
        let result = self.parser.fresh_product_binding();
        let control = json!({"kind":"named","path":source.control_owner});
        let mut elements = vec![json!({"kind":"bool"}), control.clone()];
        let mut state = Environment::new();
        let mut item_bindings = Vec::new();
        let mut keys = Vec::new();
        for (index, slot) in source.slots.iter().enumerate() {
            if !input.environment.contains_key(&slot.lexical) {
                return Err(fail(
                    "ordered iteration lost an initialized outer state root",
                ));
            }
            let ty = slot
                .origin
                .ty
                .as_ref()
                .ok_or_else(|| fail("ordered state needs its actual complete root type"))?;
            let ty = encode(&self.parser.record_source_type(ty, &slot.origin.path))?;
            let binding = self.parser.fresh_product_binding();
            state.insert(slot.lexical.clone(), (slot.clone(), binding.clone()));
            keys.push(slot.lexical.clone());
            item_bindings.push(
                json!({"k":"let","name":binding,"type":ty,"value":project(&accumulator,index+2)}),
            );
            elements.push(ty);
        }
        let ret = json!({"kind":"tuple","elements":elements});
        let owner = &source.control_owner;
        let continuation = loop_control(owner, "StatementContinuation", Vec::new());
        let initial =
            self.ordered_loop_packet(&input.environment, &keys, false, continuation.clone())?;
        let lowered = self.visit(
            node["args"][1]["value"].take(),
            state,
            depth + 1,
            Some(&control),
        )?;
        let returned = self.parser.fresh_product_binding();
        let mut cases = Vec::new();
        for variant in [
            "FunctionResult",
            "StatementContinuation",
            "LoopBreak",
            "LoopContinue",
        ] {
            let disposition = loops::read_rust_owned_record_loop_exit_disposition(variant);
            let mut path = owner.clone();
            path.push(variant.to_owned());
            let (binds, completion) = if disposition == "return" {
                (
                    vec![returned.clone()],
                    loop_control(owner, "FunctionResult", vec![name(&returned)]),
                )
            } else {
                (Vec::new(), continuation.clone())
            };
            let packet = self.ordered_loop_packet(
                &lowered.environment,
                &keys,
                disposition != "continue",
                completion,
            )?;
            cases.push(json!({"pattern":{"k":"ctor","path":path,"binds":binds},"body":packet}));
        }
        item_bindings.extend(lowered.bindings);
        let step = wrap(
            item_bindings,
            json!({"k":"match1","scrutinee":lowered.value,"cases":cases}),
        );
        let mut fold = json!({
            "k":"method", "object":name(&collection), "op":"reduce",
            "args":[{"k":"lambda","params":[{"name":accumulator,"type":ret},{"name":source.element,"type":item_type}],
                "ret":ret,"body":{"k":"if","cond":project(&accumulator,0),"then":name(&accumulator),"else":step}},
                initial],
        });
        if let Some(facet) = &source.source_fold {
            fold["span"] = encode(&facet.span)?;
            fold["sourceLoopFold"] = encode(facet)?;
            input.value["sourcePosition"] = encode(&RustSourceExpressionPosition {
                model: "Rust borrowed expression source position 1".into(),
                position: facet.input.clone(),
            })?;
        }
        input
            .bindings
            .push(json!({"k":"let","name":collection,"type":input_type,"value":input.value}));
        input
            .bindings
            .push(json!({"k":"let","name":result,"type":ret,"value":fold}));
        for (index, key) in keys.iter().enumerate() {
            let binding = self.parser.fresh_product_binding();
            input.bindings.push(json!({"k":"let","name":binding,"type":elements[index+2],"value":project(&result,index+2)}));
            input
                .environment
                .get_mut(key)
                .expect("actual initialized root")
                .1 = binding;
        }
        input.value = if forwarded {
            project(&result, 1)
        } else {
            json!({"k":"unit"})
        };
        Ok(input)
    }

    fn ordered_loop_packet(
        &self,
        environment: &Environment,
        keys: &[String],
        done: bool,
        value: Value,
    ) -> Result<Value> {
        let mut packet = self.loop_packet(environment, keys, None, Some(value))?;
        packet["items"]
            .as_array_mut()
            .expect("actual state packet")
            .insert(0, json!({"k":"bool","value":done}));
        Ok(packet)
    }
}
