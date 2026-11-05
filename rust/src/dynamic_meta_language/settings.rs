//! Native views of explicit records and decorators; no source decisions here.
use super::fail;
use crate::grammar::feature_runtime::FeatureParseOptions;
use crate::{DecoratorAction, GlobalConceptOptions};
use serde_json::{Value, json};

pub(super) fn configuration(options: &GlobalConceptOptions) -> Result<Value, String> {
    let mut records = options.records.clone().unwrap_or_else(|| json!([]));
    for record in records
        .as_array_mut()
        .ok_or_else(|| fail("records must be an array"))?
    {
        record
            .as_object_mut()
            .ok_or_else(|| fail("a record must be an object"))?
            .shift_remove("formalDefinition");
    }
    Ok(json!({"schemaVersion":1,"records":records,"equivalences":options.equivalences}))
}

pub(super) fn parser(options: &FeatureParseOptions) -> Value {
    let mut result = json!({});
    if let Some(rule) = &options.start_rule {
        result["startRule"] = json!(rule);
    }
    if let Some(value) = options.max_depth {
        result["maxDepth"] = json!(value);
    }
    if let Some(value) = options.step_limit {
        result["stepLimit"] = json!(value);
    }
    if let Some(value) = options.memo_limit {
        result["memoLimit"] = json!(value);
    }
    if let Some(value) = options.memory_limit {
        result["memoryLimit"] = json!(value);
    }
    if let Some(value) = options.reject_ambiguity {
        result["ambiguity"] = json!(if value { "reject" } else { "report" });
    }
    if let Some(value) = options.accept_recovery {
        result["recovery"] = json!(if value { "accept" } else { "reject" });
    }
    if let Some(value) = options.error_recovery {
        result["errorRecovery"] = json!(value);
    }
    if let Some(value) = options.max_repairs {
        result["maxRepairs"] = json!(value);
    }
    result["decorators"] = Value::Array(options.decorators.decorators().iter().map(|value| {
        let actions: Vec<_> = value.actions().iter().map(|action| match action {
            DecoratorAction::Set { field, value } => json!({"op":"set","field":field,"value":value}),
            DecoratorAction::Replace { field, from, to } => json!({"op":"replace","field":field,"from":from,"to":to}),
            DecoratorAction::Drop => json!({"op":"drop"}),
        }).collect();
        json!({"id":value.id(),"level":value.level().as_str(),"order":value.order(),"when":value.when(),"actions":actions})
    }).collect());
    result
}
