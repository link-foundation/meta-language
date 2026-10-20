//! Traversal of already checked linked programs for the generated shared policy.
//! No source parsing, saved-source replay or native operation whitelist.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use regex::Regex;
use serde_json::Value;

use crate::shared_numeric_function_rules::{
    accept_shared_boolean_binary, accept_shared_boolean_type_shape, accept_shared_numeric_binary,
    accept_shared_numeric_context, accept_shared_numeric_context_flag,
    accept_shared_numeric_declaration, accept_shared_numeric_entry,
    accept_shared_numeric_expression_shape, accept_shared_numeric_literal,
    accept_shared_numeric_parameter, accept_shared_numeric_source_signature,
    accept_shared_numeric_source_type, accept_shared_numeric_type_shape,
    accept_shared_numeric_unary, infer_shared_boolean_binary_domain,
};
use crate::structured_data_links::{StructuredDataValue, encode_structured_data_links_ordered};
use crate::typed_program_links::{
    decode_typed_program_links, label_of, typed_program_links_schema,
};
use crate::{LinkId, LinkNetwork, LinkType};

pub const SHARED_NUMERIC_FUNCTION_MODEL: &str = "closed binary floating point function";
pub const SHARED_BOOLEAN_FUNCTION_MODEL: &str = "closed boolean function value";

fn fail(message: &str) -> String {
    format!("shared numeric function: {message}")
}
fn string(value: &Value) -> &str {
    value.as_str().unwrap_or("")
}
fn present(value: &Value) -> bool {
    !value.is_null()
}
fn nonempty(value: &Value) -> bool {
    value.as_array().is_some_and(|value| !value.is_empty())
}
fn value_type(value: &Value, boolean_only: bool) -> Result<(), String> {
    if boolean_only
        && !accept_shared_boolean_type_shape(
            string(&value["kind"]),
            value.as_object().is_some_and(|value| value.len() == 1),
        )
    {
        return Err(fail("only Boolean values belong to the callable model"));
    }
    let count = value.as_object().map_or(0, serde_json::Map::len);
    if !accept_shared_numeric_type_shape(string(&value["kind"]), count == 1) {
        return Err(fail("only Boolean and binary64 value types are admitted"));
    }
    Ok(())
}
fn canonical_number(value: f64) -> Result<String, String> {
    let archive = encode_structured_data_links_ordered(&StructuredDataValue::Number(value))?;
    let scalar = archive
        .network
        .link(archive.root)
        .ok_or_else(|| fail("missing canonical number"))?
        .references()[0];
    archive
        .network
        .link(scalar)
        .and_then(|link| link.metadata().definition())
        .map(str::to_owned)
        .ok_or_else(|| fail("missing canonical number"))
}
fn decimal(value: &str) -> bool {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN
        .get_or_init(|| {
            let declarations = crate::structured_data_links::read_structured_data_lino(
                include_str!("data/formal-concept-models.lino"),
            )
            .expect("shared formal primitive declarations");
            Regex::new(
                declarations[SHARED_NUMERIC_FUNCTION_MODEL]["numericLiteralPattern"]
                    .as_str()
                    .expect("shared decimal grammar"),
            )
            .expect("shared decimal grammar")
        })
        .find(value)
        .is_some_and(|matched| matched.as_str() == value)
}

#[derive(Clone)]
struct Relation {
    id: LinkId,
    label: String,
    child: LinkId,
}
type Fields = BTreeMap<LinkId, Vec<Relation>>;
fn relations(network: &LinkNetwork) -> Fields {
    let mut fields: Fields = BTreeMap::new();
    for link in network.links() {
        if link.metadata().link_type() != Some(LinkType::Field) {
            continue;
        }
        let [parent, label, child] = link.references() else {
            continue;
        };
        if let Some(label) = network.link(*label).and_then(|link| link.metadata().term()) {
            fields.entry(*parent).or_default().push(Relation {
                id: link.id(),
                label: label.to_owned(),
                child: *child,
            });
        }
    }
    fields
}
fn relation<'a>(fields: &'a Fields, parent: LinkId, label: &str) -> Option<&'a Relation> {
    fields
        .get(&parent)?
        .iter()
        .find(|field| field.label == label)
}
fn definition(network: &LinkNetwork, id: LinkId) -> Option<&str> {
    network.link(id)?.metadata().definition()
}

fn context_flag_supported(
    network: &LinkNetwork,
    fields: &Fields,
    body: LinkId,
    name: &str,
) -> bool {
    let flag = relation(fields, body, &label_of(name));
    let scalar = flag.and_then(|field| network.link(field.child));
    accept_shared_numeric_context_flag(
        flag.is_some(),
        scalar.is_some_and(|link| {
            link.metadata().term() == typed_program_links_schema()["scalars"]["boolean"].as_str()
                && link.references().is_empty()
        }),
        scalar.is_some_and(|link| link.metadata().definition() == Some("false")),
    )
}

// The native checked Program has no fields for these two JS interpretation
// flags. Preserve their declared premise before projecting a selected closure.
pub fn validate_shared_numeric_origin_context(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<(), String> {
    let body = *network
        .link(root)
        .and_then(|link| link.references().first())
        .ok_or_else(|| fail("missing checked program"))?;
    let fields = relations(network);
    let language = relation(&fields, body, &label_of("sourceLanguage"))
        .and_then(|field| definition(network, field.child))
        .unwrap_or("");
    let unsupported = ["referenceSemantics", "utf16Strings"]
        .iter()
        .any(|name| !context_flag_supported(network, &fields, body, name));
    if !accept_shared_numeric_context(language, unsupported) {
        return Err(fail(
            "reference and string interpretations are outside the model",
        ));
    }
    Ok(())
}

pub struct SharedNumericFunctionModel {
    pub source_language: String,
    pub language_parent: LinkId,
    pub language_label: String,
    literals: Vec<Relation>,
    pub signature_fields: Vec<(LinkId, String)>,
    pub entry_target: LinkId,
    pub declaration_ids: BTreeSet<LinkId>,
}

pub fn read_shared_numeric_function_model(
    network: &LinkNetwork,
    root: LinkId,
    canonical: bool,
) -> Result<SharedNumericFunctionModel, String> {
    read_shared_function_value_model(network, root, canonical, false)
}

pub fn read_shared_boolean_function_model(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<SharedNumericFunctionModel, String> {
    read_shared_function_value_model(network, root, false, true)
}

fn read_shared_function_value_model(
    network: &LinkNetwork,
    root: LinkId,
    canonical: bool,
    boolean_only: bool,
) -> Result<SharedNumericFunctionModel, String> {
    let checked = decode_typed_program_links(network, root).map_err(|error| error.to_string())?;
    // Serialization exposes the same checked field names that JS traverses.
    // It is an in-memory host boundary, never an opaque stored JSON premise.
    let program = serde_json::to_value(&checked).map_err(|error| error.to_string())?;
    let fields = relations(network);
    let body = *network
        .link(root)
        .and_then(|link| link.references().first())
        .ok_or_else(|| fail("missing checked program"))?;
    let schema = typed_program_links_schema();
    let label = label_of("sourceLanguage");
    let source =
        relation(&fields, body, &label).ok_or_else(|| fail("missing source-language premise"))?;
    let unsupported = nonempty(&program["externals"])
        || nonempty(&program["imports"])
        || nonempty(&program["main"]["effects"])
        || [
            "outputThreaded",
            "abortsThreaded",
            "referenceSemantics",
            "utf16Strings",
        ]
        .iter()
        .any(|name| !context_flag_supported(network, &fields, body, name));
    let language = string(&program["sourceLanguage"]);
    if !accept_shared_numeric_context(language, unsupported) {
        return Err(fail(
            "effects, imports and reference interpretations are outside the model",
        ));
    }
    let declarations = program["declarations"]
        .as_array()
        .ok_or_else(|| fail("missing declarations"))?;
    let owned: BTreeSet<&str> = declarations
        .iter()
        .map(|value| string(&value["fullName"]))
        .collect();
    let entry = declarations
        .iter()
        .find(|value| value["fullName"] == "selected_function_definition")
        .ok_or_else(|| fail("a checked selected-function wrapper is required"))?;
    if !accept_shared_numeric_entry(
        entry["generated"] == true,
        string(&entry["body"]["k"]),
        owned.contains(string(&entry["body"]["fn"])),
    ) {
        return Err(fail("a checked selected-function wrapper is required"));
    }
    let mut expressions = Vec::new();
    for declaration in declarations {
        if !accept_shared_numeric_declaration(
            string(&declaration["k"]),
            nonempty(&declaration["modulePath"]),
        ) {
            return Err(fail("only flat closed function declarations are admitted"));
        }
        let signature = &declaration["sourceSignature"];
        if language == "Rust" {
            if signature.is_null() && !std::ptr::eq(declaration, entry) {
                return Err(fail(
                    "a Rust function needs its owned source signature facet",
                ));
            }
            if !signature.is_null() {
                let descriptors = signature["parameters"]
                    .as_array()
                    .ok_or_else(|| fail("missing source signature parameters"))?;
                let parameters = declaration["params"]
                    .as_array()
                    .ok_or_else(|| fail("missing parameters"))?;
                if !accept_shared_numeric_source_signature(
                    string(&signature["model"]),
                    string(&signature["language"]),
                    descriptors.len() == parameters.len(),
                    nonempty(&signature["lifetimeBindings"]),
                ) {
                    return Err(fail(
                        "a Rust source signature has an unsupported model or layout",
                    ));
                }
                for (descriptor, expected) in descriptors
                    .iter()
                    .zip(parameters.iter().map(|parameter| &parameter["type"]))
                    .chain(std::iter::once((&signature["result"], &declaration["ret"])))
                {
                    value_type(&descriptor["type"], boolean_only)?;
                    let path = descriptor["owner"]["path"]
                        .as_array()
                        .ok_or_else(|| fail("missing source owner path"))?;
                    let owner_path = if path.len() == 1 {
                        string(&path[0])
                    } else {
                        ""
                    };
                    if descriptor["type"]["kind"] != expected["kind"]
                        || !accept_shared_numeric_source_type(
                            string(&descriptor["type"]["kind"]),
                            string(&descriptor["owner"]["kind"]),
                            owner_path,
                            nonempty(&descriptor["wrappers"])
                                || nonempty(&descriptor["lifetimes"])
                                || nonempty(&descriptor["owner"]["arguments"]),
                        )
                    {
                        return Err(fail(
                            "a Rust source signature requires direct primitive f64 or Boolean values",
                        ));
                    }
                }
            }
        } else if !signature.is_null() {
            return Err(fail(
                "a foreign source signature is outside the JavaScript value premise",
            ));
        }
        value_type(&declaration["ret"], boolean_only)?;
        for parameter in declaration["params"]
            .as_array()
            .ok_or_else(|| fail("missing parameters"))?
        {
            value_type(&parameter["type"], boolean_only)?;
            if !accept_shared_numeric_parameter(
                present(&parameter["guard"]),
                present(&parameter["default"]),
            ) {
                return Err(fail(
                    "guarded and default parameters need a separate interpretation",
                ));
            }
        }
        expressions.push(&declaration["body"]);
    }
    let mut at = 0;
    while at < expressions.len() {
        if at > 100_000 {
            return Err(fail("expression resource limit exceeded"));
        }
        let node = expressions[at];
        at += 1;
        value_type(&node["type"], boolean_only)?;
        let kind = string(&node["k"]);
        if !accept_shared_numeric_expression_shape(
            kind,
            present(&node["rounding"]) || present(&node["byZero"]),
        ) {
            return Err(fail("an operation has no shared numeric interpretation"));
        }
        match kind {
            "lit" => {
                let text = string(&node["value"]);
                let number = text.parse::<f64>().unwrap_or(f64::NAN);
                if !accept_shared_numeric_literal(
                    string(&node["type"]["kind"]),
                    (
                        node["value"].is_boolean(),
                        node["value"].is_string() && decimal(text),
                        number.is_finite(),
                        !canonical || canonical_number(number).is_ok_and(|value| value == text),
                        canonical,
                    ),
                ) {
                    return Err(fail(
                        "a literal is not a Boolean or canonical finite binary64",
                    ));
                }
            }
            "unary" => {
                if !accept_shared_numeric_unary(
                    string(&node["op"]),
                    string(&node["type"]["kind"]),
                    string(&node["semantics"]),
                ) {
                    return Err(fail("unsupported unary primitive"));
                }
                expressions.push(&node["arg"]);
            }
            "binary" => {
                let domain = if node["domain"].is_null()
                    && boolean_only
                    && infer_shared_boolean_binary_domain(string(&node["op"]))
                {
                    &node["type"]
                } else {
                    &node["domain"]
                };
                value_type(domain, boolean_only)?;
                let accept = if boolean_only {
                    accept_shared_boolean_binary
                } else {
                    accept_shared_numeric_binary
                };
                if !accept(
                    string(&node["op"]),
                    string(&domain["kind"]),
                    string(&node["semantics"]),
                ) {
                    return Err(fail("unsupported binary primitive"));
                }
                expressions.extend([&node["left"], &node["right"]]);
            }
            "let" => expressions.extend([&node["value"], &node["body"]]),
            "if" => expressions.extend([&node["cond"], &node["then"], &node["else"]]),
            "call" => {
                if !owned.contains(string(&node["fn"])) {
                    return Err(fail("a call captures an external interpretation"));
                }
                expressions.extend(
                    node["args"]
                        .as_array()
                        .ok_or_else(|| fail("missing call arguments"))?,
                );
            }
            _ => {}
        }
    }
    let directory = relation(&fields, body, &label_of("declarations"))
        .ok_or_else(|| fail("missing declaration mapping"))?
        .child;
    let declaration_ids: BTreeSet<_> = network
        .link(directory)
        .ok_or_else(|| fail("missing declaration mapping"))?
        .references()
        .iter()
        .map(|id| {
            network
                .link(*id)
                .ok_or_else(|| fail("missing declaration pair"))
                .and_then(|link| {
                    link.references()
                        .get(1)
                        .copied()
                        .ok_or_else(|| fail("missing declaration identity"))
                })
        })
        .collect::<Result<_, _>>()?;
    let entry_id = network
        .link(directory)
        .ok_or_else(|| fail("missing declaration mapping"))?
        .references()
        .iter()
        .find_map(|id| {
            let pair = network.link(*id)?.references();
            if definition(network, *pair.first()?) == Some("selected_function_definition") {
                pair.get(1).copied()
            } else {
                None
            }
        })
        .ok_or_else(|| fail("missing selected wrapper identity"))?;
    let child = |id, name: &str| {
        relation(&fields, id, &label_of(name))
            .map(|field| field.child)
            .ok_or_else(|| fail("missing selected wrapper field"))
    };
    let entry_body = child(entry_id, "body")?;
    let params_id = child(entry_id, "params")?;
    let params = network
        .link(params_id)
        .ok_or_else(|| fail("missing selected wrapper parameters"))?
        .references();
    let args_id = child(entry_body, "args")?;
    let args = network
        .link(args_id)
        .ok_or_else(|| fail("missing selected wrapper arguments"))?
        .references();
    let target = child(entry_body, string(&schema["references"]["declaration"]))?;
    if target == entry_id
        || args.len() != params.len()
        || args.iter().zip(params).any(|(argument, parameter)| {
            child(*argument, string(&schema["references"]["binding"])) != Ok(*parameter)
        })
    {
        return Err(fail(
            "the selected wrapper must pass each owned parameter to a distinct declared function",
        ));
    }
    let mut seen = BTreeSet::new();
    let mut pending = vec![root];
    let mut literals = Vec::new();
    let mut signature_fields = Vec::new();
    let mut at = 0;
    while at < pending.len() {
        if pending.len() > 100_000 {
            return Err(fail("reference resource limit exceeded"));
        }
        let id = pending[at];
        at += 1;
        if !seen.insert(id) {
            continue;
        }
        pending.extend_from_slice(
            network
                .link(id)
                .ok_or_else(|| fail("dangling numeric reference"))?
                .references(),
        );
        for field in fields.get(&id).into_iter().flatten() {
            if field.label != "source span" && field.label != "provenance" {
                pending.push(field.child);
            }
            if matches!(field.label.as_str(), "type" | "domain" | "return type") {
                let type_fields = fields
                    .get(&field.child)
                    .ok_or_else(|| fail("missing type fields"))?;
                let type_kind = relation(&fields, field.child, &label_of("kind"))
                    .and_then(|field| definition(network, field.child))
                    .unwrap_or("");
                let accept = if boolean_only {
                    accept_shared_boolean_type_shape
                } else {
                    accept_shared_numeric_type_shape
                };
                if !accept(type_kind, type_fields.len() == 1) {
                    return Err(fail("only Boolean and binary64 value types are admitted"));
                }
            }
        }
        if let Some(signature) = relation(&fields, id, &label_of("sourceSignature")) {
            if !declaration_ids.contains(&id) {
                return Err(fail(
                    "a source signature belongs only to its owned declaration",
                ));
            }
            if network
                .link(signature.child)
                .and_then(|link| link.metadata().term())
                != schema["containers"]["record"].as_str()
            {
                return Err(fail(
                    "a present source signature must be an owned structured facet",
                ));
            }
            signature_fields.push((id, signature.label.clone()));
        }
        let kind = relation(&fields, id, &label_of("k"));
        let value = relation(&fields, id, &label_of("value"));
        if kind.is_some_and(|field| definition(network, field.child) == Some("lit"))
            && value.is_some_and(|field| {
                network.link(field.child).is_some_and(|link| {
                    link.metadata().term() == schema["scalars"]["string"].as_str()
                })
            })
        {
            literals.push(value.unwrap().clone());
        }
    }
    Ok(SharedNumericFunctionModel {
        source_language: language.to_owned(),
        language_parent: body,
        language_label: source.label.clone(),
        literals,
        signature_fields,
        entry_target: target,
        declaration_ids,
    })
}

pub fn normalize_shared_numeric_function_literals(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<SharedNumericFunctionModel, String> {
    read_shared_numeric_function_model(network, root, false)
}

/// The original typed literal strings are never replaced in their archive.
pub fn create_shared_numeric_function_meaning(
    network: &LinkNetwork,
    model: &SharedNumericFunctionModel,
) -> Result<LinkNetwork, String> {
    let mut meaning = network.clone();
    let mut replacements = BTreeMap::new();
    for relation in &model.literals {
        let original = network
            .link(relation.child)
            .ok_or_else(|| fail("missing numeric literal"))?;
        let value = original
            .metadata()
            .definition()
            .ok_or_else(|| fail("missing numeric literal"))?
            .parse::<f64>()
            .map_err(|error| error.to_string())?;
        let metadata = original
            .metadata()
            .clone()
            .with_language("meta-language formal definition")
            .with_term("binary floating point bit pattern")
            .with_definition(format!("{:016x}", value.to_bits()));
        let replacement = *replacements
            .entry(relation.child)
            .or_insert_with(|| meaning.insert_link([], metadata));
        let mut references = meaning
            .link(relation.id)
            .ok_or_else(|| fail("missing numeric literal relation"))?
            .references()
            .to_vec();
        references[2] = replacement;
        meaning.set_references(relation.id, &references);
    }
    Ok(meaning)
}
