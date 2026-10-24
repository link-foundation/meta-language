//! Transport checked operands into complete ordinary source-defined admission.
//! Admission runs before formal effect analysis; no hidden target error path.
use super::diagnostics::{Result, type_error, unsupported};
use super::ir::{Decl, Program};
use super::surface::{SItem, SModule, SProgram};
use super::{
    Language,
    types::{Type, array},
};
use serde_json::{Value, json};
use std::borrow::Cow;

fn excluded(key: &str) -> bool {
    matches!(key, "type" | "from" | "to" | "domain" | "span") || key.starts_with("source")
}
fn selected(value: &Value) -> bool {
    if value["k"] == "record" && value.get("order").is_some_and(|order| !order.is_null()) {
        return true;
    }
    match value {
        Value::Array(items) => items.iter().any(selected),
        Value::Object(fields) => fields
            .iter()
            .any(|(key, value)| !excluded(key) && selected(value)),
        _ => false,
    }
}
fn transport(error: impl std::fmt::Display) -> super::diagnostics::TranslationError {
    type_error(error.to_string(), None)
}

struct Lowerer {
    prefix: String,
    namespace: String,
    ordinal: usize,
}
impl Lowerer {
    fn lower(&mut self, value: &Value) -> Result<Value> {
        let mut result = match value {
            Value::Array(items) => Value::Array(
                items
                    .iter()
                    .map(|item| self.lower(item))
                    .collect::<Result<_>>()?,
            ),
            Value::Object(fields) => Value::Object(
                fields
                    .iter()
                    .map(|(key, value)| {
                        Ok((
                            key.clone(),
                            if excluded(key) {
                                value.clone()
                            } else {
                                self.lower(value)?
                            },
                        ))
                    })
                    .collect::<Result<_>>()?,
            ),
            _ => value.clone(),
        };
        if value["k"] != "record" || value.get("order").is_none_or(Value::is_null) {
            return Ok(result);
        }
        if value["type"]["kind"] != "record" || value["type"]["ordered"] != true {
            return Err(type_error(
                "an ordered record needs matching payload and order types",
                None,
            ));
        }
        let entries = result["entries"]
            .as_array()
            .ok_or_else(|| type_error("an ordered record needs actual fields", None))?;
        let actual_entries: Vec<super::ir::RecordEntry> =
            serde_json::from_value(Value::Array(entries.clone())).map_err(transport)?;
        let names = serde_json::to_value(super::ordered_record_values::property_name_rows(
            &actual_entries,
        ))
        .map_err(transport)?;
        let mut bindings = Vec::new();
        let mut fields = Vec::new();
        for entry in entries {
            self.ordinal += 1;
            let name = format!("{}recordField{}", self.prefix, self.ordinal);
            bindings.push((name.clone(), entry["value"].clone()));
            let mut current = entry.clone();
            current["value"] = json!({"k":"var","name":name,"type":entry["value"]["type"]});
            fields.push(current);
        }
        result["entries"] = Value::Array(fields);
        result["order"] = json!({"k":"call","fn":format!("{}.createJavaScriptRecordPropertyOrder",self.namespace),"args":[names,result["order"]],"type":{"kind":"array","element":{"kind":"float"}}});
        for (name, binding) in bindings.into_iter().rev() {
            result =
                json!({"k":"let","name":name,"value":binding,"body":result,"type":value["type"]});
            if let Some(span) = value.get("span").filter(|span| !span.is_null()) {
                result["span"] = span.clone();
            }
        }
        Ok(result)
    }
}

pub(super) fn prepare(program: &Program) -> Result<Cow<'_, Program>> {
    let value = serde_json::to_value(program).map_err(transport)?;
    let roots = value["declarations"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|entry| [&entry["body"], &entry["prop"]])
        .chain(std::iter::once(&value["main"]["effects"]));
    if !roots.into_iter().any(selected) {
        return Ok(Cow::Borrowed(program));
    }
    if program.source_language != Language::JavaScript {
        return Err(unsupported(
            "ordered record source library",
            "this source profile requires JavaScript Number and exception semantics",
            None,
        ));
    }
    if program.output_threaded || program.aborts_threaded {
        return Err(type_error(
            "record constructor admission must precede effect threading",
            None,
        ));
    }
    let source =
        crate::read_structured_data_lino(include_str!("../data/record-order-surface-library.lino"))
            .map_err(transport)?;
    if source["version"] != 1
        || source["model"] != "complete ordinary record order source library 1"
    {
        return Err(type_error(
            "an ordered record needs its complete closed source library",
            None,
        ));
    }
    let mut surface: SProgram =
        serde_json::from_value(source["surface"].clone()).map_err(transport)?;
    if surface.language != Language::JavaScript
        || !surface.externals.is_empty()
        || surface
            .main
            .as_ref()
            .is_some_and(|main| !main.effects.is_empty())
    {
        return Err(type_error(
            "an ordered record needs its complete closed source library",
            None,
        ));
    }
    let prefix = super::source_identifiers::read_source_identifier_supply(
        super::source_identifiers::collect_source_identifier_texts(&value),
    );
    let namespace = format!("{prefix}recordOrder");
    surface.items = vec![SItem::Module(SModule {
        name: namespace.clone(),
        items: surface.items,
        span: None,
    })];
    surface.main = None;
    let library = super::check::check_program(&surface)?;
    for entry in &library.declarations {
        if program.declaration(entry.full_name()).is_some()
            || program
                .externals
                .iter()
                .any(|external| external.name() == entry.full_name())
        {
            return Err(type_error(
                "an ordered record library namespace collides with an actual declaration",
                None,
            ));
        }
    }
    let constructor =
        library.declaration(&format!("{namespace}.createJavaScriptRecordPropertyOrder"));
    if !matches!(constructor,Some(Decl::Fn(entry))if entry.params.len()==2&&entry.params[0].ty.same(&array(array(Type::Float)))&&entry.params[1].ty.same(&array(Type::Float))&&entry.ret.same(&array(Type::Float)))
    {
        return Err(type_error(
            "an ordered record constructor needs its complete owned signature",
            None,
        ));
    }
    let mut lowerer = Lowerer {
        prefix,
        namespace,
        ordinal: 0,
    };
    let mut result = program.clone();
    for entry in &mut result.declarations {
        match entry {
            Decl::Fn(entry) => {
                entry.body = serde_json::from_value(
                    lowerer.lower(&serde_json::to_value(&entry.body).map_err(transport)?)?,
                )
                .map_err(transport)?
            }
            Decl::Theorem(entry) => {
                entry.prop = serde_json::from_value(
                    lowerer.lower(&serde_json::to_value(&entry.prop).map_err(transport)?)?,
                )
                .map_err(transport)?
            }
            _ => {}
        }
    }
    if let Some(main) = &mut result.main {
        main.effects = serde_json::from_value(
            lowerer.lower(&serde_json::to_value(&main.effects).map_err(transport)?)?,
        )
        .map_err(transport)?;
    }
    result.items.extend(library.items);
    result.declarations.extend(library.declarations);
    Ok(Cow::Owned(result))
}
