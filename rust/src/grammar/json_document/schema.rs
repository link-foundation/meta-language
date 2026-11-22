use std::collections::{HashMap, HashSet};

use regex::Regex;
use serde_json::Value;

use super::patterns::validation_pattern;
use super::{JsonSchemaIssue, pointer_child};

const POLICY: &str = include_str!("../../data/json-document/schema.lino");

pub(super) fn keywords(category: &str) -> impl Iterator<Item = &'static str> {
    POLICY
        .lines()
        .find_map(|row| {
            let mut terms = row.trim_matches(['(', ')']).split_ascii_whitespace();
            (terms.next() == Some(category)).then_some(terms)
        })
        .into_iter()
        .flatten()
}

pub(super) fn formats(name: &str) -> Option<fn(&str) -> bool> {
    match name {
        "date" => Some(date),
        "date-time" => Some(date_time),
        "uuid" => Some(uuid),
        "email" => Some(email),
        _ => None,
    }
}

fn date(value: &str) -> bool {
    static SHAPE: std::sync::LazyLock<Regex> =
        std::sync::LazyLock::new(|| Regex::new(r"\A([0-9]{4})-([0-9]{2})-([0-9]{2})\z").unwrap());
    let Some(fields) = SHAPE.captures(value) else {
        return false;
    };
    let year: u16 = fields[1].parse().unwrap();
    let month: usize = fields[2].parse().unwrap();
    let day: u8 = fields[3].parse().unwrap();
    if !(1..=12).contains(&month) || day == 0 {
        return false;
    }
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    day <= [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ][month - 1]
}

fn date_time(value: &str) -> bool {
    static SHAPE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
        Regex::new(r"\A([0-9]{4}-[0-9]{2}-[0-9]{2})[Tt]([0-9]{2}):([0-9]{2}):([0-9]{2})(?:\.[0-9]+)?(?:[Zz]|[+-]([0-9]{2}):([0-9]{2}))\z").unwrap()
    });
    SHAPE.captures(value).is_some_and(|fields| {
        let number = |index| {
            fields
                .get(index)
                .map_or(0, |part| part.as_str().parse::<u8>().unwrap())
        };
        date(&fields[1])
            && number(2) <= 23
            && number(3) <= 59
            && number(4) <= 60
            && number(5) <= 23
            && number(6) <= 59
    })
}

fn uuid(value: &str) -> bool {
    static SHAPE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
        Regex::new(r"\A[0-9A-Fa-f]{8}-(?:[0-9A-Fa-f]{4}-){3}[0-9A-Fa-f]{12}\z").unwrap()
    });
    SHAPE.is_match(value)
}

fn email(value: &str) -> bool {
    static SHAPE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
        Regex::new(r#"(?-u:\A(?:[A-Za-z0-9!#$%&'*+\-/=?^_`{|}~]+(?:\.[A-Za-z0-9!#$%&'*+\-/=?^_`{|}~]+)*|"(?:[\x20-\x21\x23-\x5b\x5d-\x7e]|\\[\x20-\x7e])*")@(?:[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?)(?:\.[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?)*\z)"#).unwrap()
    });
    SHAPE.is_match(value)
}

pub(super) fn reference<'schema>(root: &'schema Value, reference: &str) -> Option<&'schema Value> {
    let fragment = reference.strip_prefix('#')?;
    let mut bytes = fragment.bytes();
    let mut decoded = Vec::with_capacity(fragment.len());
    while let Some(byte) = bytes.next() {
        decoded.push(if byte == b'%' {
            (char::from(bytes.next()?).to_digit(16)? * 16
                + char::from(bytes.next()?).to_digit(16)?)
            .try_into()
            .ok()?
        } else {
            byte
        });
    }
    let pointer = std::str::from_utf8(&decoded).ok()?;
    for escaped in pointer.split('~').skip(1) {
        if !escaped.starts_with(['0', '1']) {
            return None;
        }
    }
    root.pointer(pointer)
        .filter(|value| value.is_boolean() || value.is_object())
}

pub(super) fn types(value: &Value) -> &[Value] {
    value
        .as_array()
        .map_or_else(|| std::slice::from_ref(value), Vec::as_slice)
}

pub(super) struct Analysis<'schema> {
    pub root: &'schema Value,
    pub issues: Vec<JsonSchemaIssue>,
    pub patterns: HashMap<*const Value, Regex>,
}

impl<'schema> Analysis<'schema> {
    pub fn new(root: &'schema Value) -> Self {
        let mut analysis = Self {
            root,
            issues: Vec::new(),
            patterns: HashMap::new(),
        };
        analysis.visit(root, "");
        analysis
    }

    fn issue(&mut self, pointer: &str, keyword: &str, message: String) {
        self.issues.push(JsonSchemaIssue {
            keyword: keyword.to_owned(),
            pointer: pointer_child(pointer, keyword),
            message,
        });
    }

    fn visit(&mut self, schema: &'schema Value, pointer: &str) {
        let Some(rules) = schema.as_object() else {
            if !schema.is_boolean() {
                self.issues.push(JsonSchemaIssue {
                    keyword: "schema".to_owned(),
                    pointer: pointer.to_owned(),
                    message: "Expected a JSON Schema object or boolean".to_owned(),
                });
            }
            return;
        };
        for (keyword, value) in rules {
            let keyword = keyword.as_str();
            let location = pointer_child(pointer, keyword);
            let malformed = if keywords("annotation").any(|name| name == keyword) {
                None
            } else if keywords("map").any(|name| name == keyword) {
                if let Some(children) = value.as_object() {
                    for (name, child) in children {
                        self.visit(child, &pointer_child(&location, name));
                    }
                    None
                } else {
                    Some("an object of schemas")
                }
            } else if keywords("list").any(|name| name == keyword) {
                if let Some(children) = value.as_array().filter(|children| !children.is_empty()) {
                    for (index, child) in children.iter().enumerate() {
                        self.visit(child, &pointer_child(&location, &index.to_string()));
                    }
                    None
                } else {
                    Some("a nonempty array of schemas")
                }
            } else if keywords("schema").any(|name| name == keyword) {
                self.visit(value, &location);
                None
            } else {
                self.keyword(schema, pointer, keyword, value)
            };
            if let Some(expected) = malformed {
                self.issue(pointer, keyword, format!("Expected {expected}"));
            }
        }
    }

    fn keyword(
        &mut self,
        schema: &'schema Value,
        pointer: &str,
        keyword: &str,
        value: &'schema Value,
    ) -> Option<&'static str> {
        match keyword {
            "const" => None,
            "type" => {
                let types = types(value);
                let mut seen = HashSet::new();
                (types.is_empty()
                    || !types.iter().all(|value| {
                        value.as_str().is_some_and(|name| {
                            keywords("type").any(|valid| valid == name) && seen.insert(name)
                        })
                    }))
                .then_some("a JSON type or a nonempty list of unique JSON types")
            }
            "enum" => (!value.as_array().is_some_and(|values| !values.is_empty()))
                .then_some("a nonempty array of permitted values"),
            "required" => {
                let mut seen = HashSet::new();
                (!value.as_array().is_some_and(|keys| {
                    keys.iter()
                        .all(|key| key.as_str().is_some_and(|name| seen.insert(name)))
                }))
                .then_some("an array of unique property names")
            }
            "minItems" | "maxItems" => (!value
                .as_u64()
                .is_some_and(|value| value <= 9_007_199_254_740_991))
            .then_some("a nonnegative safe integer"),
            "pattern" => {
                let Some(pattern) = value.as_str() else {
                    return Some("an ECMA-262 regular expression");
                };
                match validation_pattern(pattern) {
                    Ok(pattern) => {
                        self.patterns.insert(std::ptr::from_ref(schema), pattern);
                    }
                    Err(error) => self.issue(
                        pointer,
                        keyword,
                        format!("Unsupported regular expression {pattern}: {error}"),
                    ),
                }
                None
            }
            "format" => {
                let Some(format) = value.as_str() else {
                    return Some("a format name");
                };
                if formats(format).is_none() {
                    self.issue(
                        pointer,
                        keyword,
                        format!("Unsupported string format {format}"),
                    );
                }
                None
            }
            "$ref" => {
                if value
                    .as_str()
                    .and_then(|name| reference(self.root, name))
                    .is_none()
                {
                    self.issue(
                        pointer,
                        keyword,
                        format!("Unresolved local schema reference {value}"),
                    );
                }
                None
            }
            keyword if keywords("limit").any(|name| name == keyword) => {
                (!value.is_number()).then_some("a finite number")
            }
            _ => {
                self.issue(
                    pointer,
                    keyword,
                    format!("Unsupported schema keyword {keyword}"),
                );
                None
            }
        }
    }
}
