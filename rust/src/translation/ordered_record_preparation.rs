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
use std::collections::HashSet;

fn excluded(key: &str) -> bool {
    matches!(key, "type" | "from" | "to" | "domain" | "span") || key.starts_with("source")
}
fn selected(value: &Value) -> bool {
    if value["k"] == "math"
        && super::javascript_enumeration_rules::read_java_script_record_order_kernel_arity(
            value["op"].as_str().unwrap_or_default(),
        ) > 0.0
    {
        return true;
    }
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
    specializations: Vec<(
        String,
        super::record_value_order_specialization::Specialization,
    )>,
    operations: HashSet<String>,
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
        if value["k"] == "math" && (value["op"] == "reorderJavaScriptRecordPropertyValues"||value["op"]==super::javascript_enumeration_rules::read_java_script_object_entry_pairing_operation()) {
            let pairing=value["op"]==super::javascript_enumeration_rules::read_java_script_object_entry_pairing_operation();
            let element: Type =
                serde_json::from_value(result["args"][usize::from(pairing)]["type"]["element"].clone())
                    .map_err(transport)?;
            let key = element.key();
            let index = if let Some(index) = self
                .specializations
                .iter()
                .position(|(identity, _)| identity == &key)
            {
                index
            } else {
                let namespace = format!(
                    "{}recordValues{}",
                    self.prefix,
                    self.specializations.len() + 1
                );
                let specialization =
                    super::record_value_order_specialization::specialize(&element, &namespace)?;
                self.specializations.push((key, specialization));
                self.specializations.len() - 1
            };
            let lower=if pairing{super::record_value_order_specialization::lower_entries}else{super::record_value_order_specialization::lower};
            return lower(
                value,
                result["args"].as_array().ok_or_else(|| {
                    type_error(
                        "record value ordering needs every actual owned operand",
                        None,
                    )
                })?,
                &self.specializations[index].1,
                &self.prefix,
                &mut self.ordinal,
            );
        }
        if value["k"] == "math"
            && super::javascript_enumeration_rules::read_java_script_record_order_kernel_arity(
                value["op"].as_str().unwrap_or_default(),
            ) > 0.0
        {
            if !self
                .operations
                .contains(value["op"].as_str().unwrap_or_default())
            {
                return Err(type_error(
                    "an ordinary record order operation has no complete source definition",
                    None,
                ));
            }
            let mut call = json!({"k":"call","fn":format!("{}.{}",self.namespace,value["op"].as_str().unwrap()),"args":result["args"],"type":result["type"]});
            if let Some(span) = value.get("span").filter(|span| !span.is_null()) {
                call["span"] = span.clone();
            }
            return Ok(call);
        }
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
    for declaration in &library.declarations {
        let operation = declaration.name();
        let arity = super::javascript_enumeration_rules::read_java_script_record_order_kernel_arity(
            operation,
        );
        if arity == 0.0 {
            continue;
        }
        let Decl::Fn(entry) = declaration else {
            return Err(type_error(
                "an ordinary record order library operation needs its complete owned signature",
                None,
            ));
        };
        if serde_json::Number::from(entry.params.len()).as_f64() != Some(arity)
            || !entry
                .ret
                .same(&super::check::record_order_operations::role_type(
                &super::javascript_enumeration_rules::read_java_script_record_order_kernel_result(
                    operation,
                ),
            )?)
        {
            return Err(type_error(
                "an ordinary record order library operation needs its complete owned signature",
                None,
            ));
        }
        for (index, parameter) in entry.params.iter().enumerate() {
            let index = serde_json::Number::from(index)
                .as_f64()
                .expect("a bounded library arity is a Number");
            if !parameter.ty.same(&super::check::record_order_operations::role_type(&super::javascript_enumeration_rules::read_java_script_record_order_kernel_parameter(operation,index))?){return Err(type_error("an ordinary record order library operation needs its complete owned signature",None));}
        }
    }
    let mut lowerer = Lowerer {
        specializations: Vec::new(),
        operations: library
            .declarations
            .iter()
            .filter_map(|declaration| {
                matches!(declaration, Decl::Fn(_)).then(|| declaration.name().to_owned())
            })
            .collect(),
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
    for (_, specialization) in lowerer.specializations {
        for declaration in &specialization.library.declarations {
            if result.declaration(declaration.full_name()).is_some()
                || program
                    .externals
                    .iter()
                    .any(|external| external.name() == declaration.full_name())
            {
                return Err(type_error(
                    "a record value specialization collides with an actual declaration",
                    None,
                ));
            }
        }
        result.items.extend(specialization.library.items);
        result
            .declarations
            .extend(specialization.library.declarations);
    }
    Ok(Cow::Owned(result))
}
