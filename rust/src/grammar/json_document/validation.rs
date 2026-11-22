use std::collections::HashSet;

use serde_json::{Value, json};

use super::document::{Document, Node};
use super::schema::{Analysis, formats, keywords, reference, types};
use super::{JsonDocumentIssue, JsonSchemaIssue, pointer_child};
use crate::{LinkId, LinkNetwork};

/// Validation result with source-linked diagnostics, including on incomplete input.
pub struct JsonValidation {
    /// True when the document parsed and every supplied constraint could be verified.
    pub valid: bool,
    /// The complete JSON value, if it could be decoded.
    pub value: Option<Value>,
    /// Syntax and instance validation errors.
    pub errors: Vec<JsonDocumentIssue>,
    /// Unsupported or malformed schema constraints.
    pub schema_issues: Vec<JsonSchemaIssue>,
    /// The concrete syntax links for the original text.
    pub network: LinkNetwork,
}

/// Validates JSON and locates diagnostics on concrete syntax links.
#[must_use]
pub fn validate_json(source: &str, schema: &Value) -> JsonValidation {
    let document = Document::read(source);
    let (value, errors, schema_issues) = validate(&document, schema);
    JsonValidation {
        valid: errors.is_empty() && schema_issues.is_empty(),
        value,
        errors,
        schema_issues,
        network: document.network,
    }
}

pub(super) fn validate(
    document: &Document<'_>,
    schema: &Value,
) -> (Option<Value>, Vec<JsonDocumentIssue>, Vec<JsonSchemaIssue>) {
    let mut validator = Validator {
        analysis: Analysis::new(schema),
        active: HashSet::new(),
        errors: document.syntax_issue().into_iter().collect(),
    };
    let value = if validator.errors.is_empty() {
        document.value().and_then(Node::read)
    } else {
        None
    };
    if let Some(node) = document.value() {
        if value.is_none() && validator.errors.is_empty() {
            validator
                .errors
                .push(node.issue("value", vec![json!("a representable JSON value")]));
        }
        validator.visit(schema, node);
    }
    (value, validator.errors, validator.analysis.issues)
}

fn same_value(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.as_f64() == right.as_f64(),
        (Value::Array(left), Value::Array(right)) => {
            left.len() == right.len() && left.iter().zip(right).all(|(a, b)| same_value(a, b))
        }
        (Value::Object(left), Value::Object(right)) => {
            left.len() == right.len()
                && left.iter().all(|(key, value)| {
                    right.get(key).is_some_and(|other| same_value(value, other))
                })
        }
        _ => left == right,
    }
}

struct Validator<'schema> {
    analysis: Analysis<'schema>,
    active: HashSet<(*const Value, LinkId)>,
    errors: Vec<JsonDocumentIssue>,
}

impl<'schema> Validator<'schema> {
    fn report(&mut self, node: &Node<'_>, keyword: &str, expected: Vec<Value>) {
        self.errors.push(node.issue(keyword, expected));
    }

    fn visit(&mut self, schema: &'schema Value, node: &Node<'_>) {
        let complete = !node.flags.has_error();
        let kind = match node.kind.as_str() {
            "true" | "false" => "boolean",
            kind => kind,
        };
        if !complete && !matches!(kind, "object" | "array") {
            return;
        }
        if schema == &Value::Bool(false) {
            self.report(node, "false", Vec::new());
            return;
        }
        let Some(rules) = schema.as_object() else {
            return;
        };
        let identity = (std::ptr::from_ref(schema), node.link_id);
        if !self.active.insert(identity) {
            self.report(
                node,
                "$ref",
                vec![json!("a reference that advances into the instance")],
            );
            return;
        }
        if let Some(target) = rules
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|reference_name| reference(self.analysis.root, reference_name))
        {
            self.visit(target, node);
        }
        let number = (kind == "number")
            .then(|| node.text.parse::<f64>().ok())
            .flatten();
        if let Some(declared) = rules.get("type")
            && !types(declared).iter().any(|expected| {
                expected.as_str() == Some(kind)
                    || (expected.as_str() == Some("integer")
                        && number.is_some_and(|value| value.fract() == 0.0))
            })
        {
            self.report(node, "type", types(declared).to_vec());
        }
        if complete
            && (rules.contains_key("enum") || rules.contains_key("const"))
            && let Some(value) = node.read()
        {
            if let Some(alternatives) = rules.get("enum").and_then(Value::as_array)
                && !alternatives
                    .iter()
                    .any(|expected| same_value(expected, &value))
            {
                self.report(node, "enum", alternatives.clone());
            }
            if let Some(expected) = rules.get("const")
                && !same_value(expected, &value)
            {
                self.report(node, "const", vec![expected.clone()]);
            }
        }
        match kind {
            "object" => self.object(schema, node, complete),
            "array" => self.array(schema, node, complete),
            "string" => self.string(schema, node),
            "number" => {
                if let Some(number) = number {
                    for keyword in keywords("limit") {
                        if let Some(limit) = schema[keyword].as_f64() {
                            let invalid = match keyword {
                                "minimum" => number < limit,
                                "maximum" => number > limit,
                                "exclusiveMinimum" => number <= limit,
                                "exclusiveMaximum" => number >= limit,
                                _ => unreachable!("registered numeric bound"),
                            };
                            if invalid {
                                self.report(node, keyword, vec![schema[keyword].clone()]);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        if complete {
            for keyword in ["anyOf", "oneOf"] {
                if let Some(choices) = schema[keyword].as_array() {
                    self.combination(keyword, choices, node);
                }
            }
        }
        self.active.remove(&identity);
    }

    fn object(&mut self, schema: &'schema Value, node: &Node<'_>, complete: bool) {
        let pairs: indexmap::IndexMap<_, _> = node
            .children
            .iter()
            .filter(|pair| pair.kind == "pair")
            .filter_map(|pair| Some((pair.key()?, pair)))
            .collect();
        if complete && let Some(required) = schema["required"].as_array() {
            for key in required.iter().filter_map(Value::as_str) {
                if !pairs.contains_key(key) {
                    let mut issue = node.issue("required", vec![json!(key)]);
                    issue.pointer = pointer_child(&node.pointer, key);
                    self.errors.push(issue);
                }
            }
        }
        let properties = schema["properties"].as_object();
        for (key, pair) in pairs {
            if let Some(rules) = properties.and_then(|properties| properties.get(&key)) {
                if let Some(value) = pair.field("value") {
                    self.visit(rules, value);
                }
            } else if schema["additionalProperties"] == Value::Bool(false) {
                let expected = properties
                    .into_iter()
                    .flat_map(serde_json::Map::keys)
                    .cloned()
                    .map(Value::String)
                    .collect();
                self.report(
                    pair.field("key").unwrap_or(pair),
                    "additionalProperties",
                    expected,
                );
            } else if schema["additionalProperties"].is_object()
                && let Some(value) = pair.field("value")
            {
                self.visit(&schema["additionalProperties"], value);
            }
        }
    }

    fn array(&mut self, schema: &'schema Value, node: &Node<'_>, complete: bool) {
        let items: Vec<_> = node
            .children
            .iter()
            .filter(|child| child.is_value())
            .collect();
        let length = u64::try_from(items.len()).expect("document length");
        if complete {
            if schema["minItems"].as_u64().is_some_and(|min| length < min) {
                self.report(node, "minItems", vec![schema["minItems"].clone()]);
            }
            if schema["maxItems"].as_u64().is_some_and(|max| length > max) {
                self.report(node, "maxItems", vec![schema["maxItems"].clone()]);
            }
        }
        let prefix = schema["prefixItems"]
            .as_array()
            .map_or(&[][..], Vec::as_slice);
        for (index, item) in items.into_iter().enumerate() {
            if let Some(child) = prefix.get(index) {
                self.visit(child, item);
            } else if schema["items"] == Value::Bool(false) && !item.flags.has_error() {
                self.report(item, "items", Vec::new());
            } else if let Some(child) = schema.get("items") {
                self.visit(child, item);
            }
        }
    }

    fn string(&mut self, schema: &'schema Value, node: &Node<'_>) {
        let Ok(value) = serde_json::from_str::<String>(node.text) else {
            return;
        };
        if self
            .analysis
            .patterns
            .get(&std::ptr::from_ref(schema))
            .is_some_and(|pattern| !pattern.is_match(&value))
        {
            self.report(node, "pattern", vec![schema["pattern"].clone()]);
        }
        if schema["format"]
            .as_str()
            .and_then(formats)
            .is_some_and(|matches| !matches(&value))
        {
            self.report(node, "format", vec![schema["format"].clone()]);
        }
    }

    fn combination(&mut self, keyword: &str, choices: &'schema [Value], node: &Node<'_>) {
        let mut alternatives = Vec::with_capacity(choices.len());
        for choice in choices {
            let before = self.errors.len();
            self.visit(choice, node);
            alternatives.push(self.errors.split_off(before));
        }
        let accepted = alternatives
            .iter()
            .filter(|branch| branch.is_empty())
            .count();
        if accepted == 0 || (keyword == "oneOf" && accepted != 1) {
            let expected = if keyword == "oneOf" {
                "one schema"
            } else {
                "at least one schema"
            };
            let mut issue = node.issue(keyword, vec![json!(expected)]);
            issue.alternatives = alternatives;
            issue.accepted = Some(accepted);
            self.errors.push(issue);
        }
    }
}

pub(super) fn accepts_constant(root: &Value, schema: &Value, instance: &Value) -> bool {
    let source = instance.to_string();
    let document = Document::read(&source);
    let Some(node) = document.value() else {
        return false;
    };
    let mut validator = Validator {
        analysis: Analysis::new(root),
        active: HashSet::new(),
        errors: document.syntax_issue().into_iter().collect(),
    };
    validator.visit(schema, node);
    validator.errors.is_empty()
}
