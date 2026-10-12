//! Inspectable structured values with ordered entries and exact UTF-16 strings.
//! The generated shared rules select node kinds and structural shapes; this
//! adapter supplies native graph identities and JSON interoperability.

use std::collections::{BTreeSet, HashMap, HashSet};

use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};
use serde_json::{Map, Number, Value};

use crate::structured_data_link_rules::{accept_structured_data_shape, read_structured_data_kind};
use crate::{LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType};

const LANGUAGE: &str = "meta-language structured data";

/// Lossless structured data, including ordered keys and unpaired code units.
#[derive(Clone, Debug, PartialEq)]
pub enum StructuredDataValue {
    /// An absent scalar value.
    Null,
    /// A logical value.
    Boolean(bool),
    /// A finite IEEE 754 binary64 value, including negative zero.
    Number(f64),
    /// An exact sequence of UTF-16 code units.
    String(Vec<u16>),
    /// Ordered values.
    Array(Vec<Self>),
    /// Ordered, distinct keys and their values.
    Object(Vec<(Vec<u16>, Self)>),
}

/// A structured value encoded as an explicit rooted Links network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuredDataLinks {
    /// Inspectable containers, keys and scalar values.
    pub network: LinkNetwork,
    /// The schema root.
    pub root: LinkId,
}

/// Bounds on decoding and the size of its resulting value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructuredDataLinksLimits {
    /// Maximum visited occurrences, including repeated references.
    pub node_limit: usize,
    /// Maximum graph traversal depth.
    pub depth_limit: usize,
    /// Maximum serialized input and decoded scalar bytes.
    pub byte_limit: usize,
}

impl Default for StructuredDataLinksLimits {
    fn default() -> Self {
        Self {
            node_limit: 1_000_000,
            depth_limit: 256,
            byte_limit: 128 * 1024 * 1024,
        }
    }
}

fn metadata(term: &str, definition: Option<&str>) -> LinkMetadata {
    let metadata = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_named(true)
        .with_language(LANGUAGE)
        .with_term(term);
    if let Some(text) = definition {
        metadata.with_definition(text)
    } else {
        metadata
    }
}

fn fail(message: &str) -> String {
    format!("structured data: {message}")
}

fn write_number(value: f64) -> Result<String, String> {
    if !value.is_finite() {
        return Err(fail("a number must be finite binary64"));
    }
    if value.to_bits() & 0x7fff_ffff_ffff_ffff == 0 {
        return Ok(if value.is_sign_negative() { "-0" } else { "0" }.to_owned());
    }
    let shortest = value.abs().to_string();
    let (coefficient, exponent) = shortest.split_once(['e', 'E']).map_or(
        Ok((shortest.as_str(), 0_i32)),
        |(coefficient, exponent)| {
            exponent
                .parse::<i32>()
                .map(|exponent| (coefficient, exponent))
                .map_err(|_| fail("a number exponent is invalid"))
        },
    )?;
    let point = coefficient.find('.').unwrap_or(coefficient.len());
    let mut digits: String = coefficient
        .chars()
        .filter(|character| *character != '.')
        .collect();
    let leading = digits
        .chars()
        .take_while(|character| *character == '0')
        .count();
    digits.drain(..leading);
    while digits.ends_with('0') {
        digits.pop();
    }
    let magnitude = i32::try_from(point).map_err(|_| fail("a number is too long"))?
        - i32::try_from(leading).map_err(|_| fail("a number is too long"))?
        + exponent
        - 1;
    let mut output = if (-6..21).contains(&magnitude) {
        let position = magnitude + 1;
        if position <= 0 {
            format!(
                "0.{}{digits}",
                "0".repeat(usize::try_from(-position).unwrap())
            )
        } else {
            let position = usize::try_from(position).unwrap();
            if position >= digits.len() {
                format!("{digits}{}", "0".repeat(position - digits.len()))
            } else {
                format!("{}.{}", &digits[..position], &digits[position..])
            }
        }
    } else {
        let coefficient = if digits.len() == 1 {
            digits
        } else {
            format!("{}.{}", &digits[..1], &digits[1..])
        };
        format!(
            "{coefficient}e{}{magnitude}",
            if magnitude >= 0 { "+" } else { "" }
        )
    };
    if value.is_sign_negative() {
        output.insert(0, '-');
    }
    Ok(output)
}

fn read_number(text: &str) -> Result<f64, String> {
    let number = text
        .parse::<f64>()
        .map_err(|_| fail("a number is invalid"))?;
    if write_number(number)? != text {
        return Err(fail("a number is not canonical binary64"));
    }
    Ok(number)
}

struct Encoder {
    network: LinkNetwork,
    limits: StructuredDataLinksLimits,
    bytes: usize,
    scalars: HashMap<(String, Option<String>), LinkId>,
}

impl Encoder {
    fn node(
        &mut self,
        term: &str,
        definition: Option<&str>,
        references: &[LinkId],
        depth: usize,
    ) -> Result<LinkId, String> {
        if self.network.len() >= self.limits.node_limit || depth > self.limits.depth_limit {
            return Err(fail("the value exceeds its structural resource limit"));
        }
        self.bytes = self.bytes.saturating_add(definition.map_or(0, str::len));
        if self.bytes > self.limits.byte_limit {
            return Err(fail("the value exceeds its scalar byte resource limit"));
        }
        let id = self.network.insert_link([], metadata(term, definition));
        self.network.set_references(id, references);
        Ok(id)
    }

    fn scalar(
        &mut self,
        term: &str,
        definition: Option<&str>,
        depth: usize,
    ) -> Result<LinkId, String> {
        if depth > self.limits.depth_limit {
            return Err(fail("the value exceeds its structural resource limit"));
        }
        let key = (term.to_owned(), definition.map(str::to_owned));
        if let Some(id) = self.scalars.get(&key) {
            return Ok(*id);
        }
        let id = self.node(term, definition, &[], depth)?;
        self.scalars.insert(key, id);
        Ok(id)
    }

    fn string(&mut self, text: &[u16], depth: usize) -> Result<LinkId, String> {
        if depth > self.limits.depth_limit {
            return Err(fail("the value exceeds its structural resource limit"));
        }
        if let Ok(scalar) = String::from_utf16(text) {
            self.scalar("string value", Some(&scalar), depth)
        } else {
            let key = (
                "Unicode code unit string".to_owned(),
                Some(format!("{text:?}")),
            );
            if let Some(id) = self.scalars.get(&key) {
                return Ok(*id);
            }
            let units = text
                .iter()
                .map(|unit| self.scalar("Unicode code unit", Some(&unit.to_string()), depth + 1))
                .collect::<Result<Vec<_>, _>>()?;
            let id = self.node("Unicode code unit string", None, &units, depth)?;
            self.scalars.insert(key, id);
            Ok(id)
        }
    }

    fn value(&mut self, value: &StructuredDataValue, depth: usize) -> Result<LinkId, String> {
        if depth > self.limits.depth_limit {
            return Err(fail("the value exceeds its structural resource limit"));
        }
        match value {
            StructuredDataValue::Null => self.scalar("null value", None, depth),
            StructuredDataValue::Boolean(value) => self.scalar(
                "boolean value",
                Some(if *value { "true" } else { "false" }),
                depth,
            ),
            StructuredDataValue::Number(value) => {
                self.scalar("number value", Some(&write_number(*value)?), depth)
            }
            StructuredDataValue::String(text) => self.string(text, depth),
            StructuredDataValue::Array(values) => {
                let id = self.node("array value", None, &[], depth)?;
                let children = values
                    .iter()
                    .map(|value| self.value(value, depth + 1))
                    .collect::<Result<Vec<_>, _>>()?;
                self.network.set_references(id, &children);
                Ok(id)
            }
            StructuredDataValue::Object(entries) => {
                let id = self.node("object value", None, &[], depth)?;
                let mut keys = BTreeSet::new();
                let mut children = Vec::with_capacity(entries.len());
                for (key, value) in entries {
                    if !keys.insert(key) {
                        return Err(fail("an object contains a duplicate key"));
                    }
                    let key = self.string(key, depth + 2)?;
                    let value = self.value(value, depth + 2)?;
                    children.push(self.node("key value pair", None, &[key, value], depth + 1)?);
                }
                self.network.set_references(id, &children);
                Ok(id)
            }
        }
    }
}

/// Encode ordered data without replacing any scalar or key.
pub fn encode_structured_data_links_ordered(
    value: &StructuredDataValue,
) -> Result<StructuredDataLinks, String> {
    let mut encoder = Encoder {
        network: LinkNetwork::new(),
        limits: StructuredDataLinksLimits::default(),
        bytes: 0,
        scalars: HashMap::new(),
    };
    let body = encoder.value(value, 1)?;
    let root = encoder.node("structured data", Some("1"), &[body], 0)?;
    Ok(StructuredDataLinks {
        network: encoder.network,
        root,
    })
}

struct Decoder<'a> {
    network: &'a LinkNetwork,
    limits: StructuredDataLinksLimits,
    visits: usize,
    bytes: usize,
    seen: HashSet<LinkId>,
    active: HashSet<LinkId>,
}

impl Decoder<'_> {
    fn enter(
        &mut self,
        id: LinkId,
        depth: usize,
    ) -> Result<(String, Option<String>, Vec<LinkId>), String> {
        self.visits = self.visits.saturating_add(1);
        if self.visits > self.limits.node_limit || depth > self.limits.depth_limit {
            return Err(fail("the value exceeds its structural resource limit"));
        }
        if !self.active.insert(id) {
            return Err(fail("the value contains a structural cycle"));
        }
        if id.as_u64() == 0 || id.as_u64() > 9_007_199_254_740_991 {
            return Err(fail("a link identity is not canonical"));
        }
        let link = self
            .network
            .link(id)
            .ok_or_else(|| fail("a reference is dangling"))?;
        let metadata = link.metadata();
        if metadata.link_type() != Some(LinkType::Semantic)
            || !metadata.is_named()
            || metadata.language() != Some(LANGUAGE)
            || metadata.span().is_some()
            || metadata.flags() != LinkFlags::clean()
        {
            return Err(fail("a node has foreign or malformed metadata"));
        }
        let kind = read_structured_data_kind(metadata.term().unwrap_or_default());
        let definition = metadata.definition().map(str::to_owned);
        let references = link.references().to_vec();
        self.seen.insert(id);
        self.bytes = self
            .bytes
            .saturating_add(definition.as_ref().map_or(0, String::len));
        if self.bytes > self.limits.byte_limit {
            return Err(fail("the value exceeds its scalar byte resource limit"));
        }
        if !accept_structured_data_shape(&kind, &references.len().to_string(), definition.is_some())
        {
            return Err(fail("a node has an unknown kind or invalid shape"));
        }
        Ok((kind, definition, references))
    }

    fn string(&mut self, id: LinkId, depth: usize) -> Result<Vec<u16>, String> {
        let value = self.value(id, depth)?;
        if let StructuredDataValue::String(text) = value {
            Ok(text)
        } else {
            Err(fail("an object key must be a string"))
        }
    }

    fn pair(
        &mut self,
        id: LinkId,
        depth: usize,
    ) -> Result<(Vec<u16>, StructuredDataValue), String> {
        let (kind, _, references) = self.enter(id, depth)?;
        if kind != "pair" {
            return Err(fail("an object child must be a key value pair"));
        }
        let key = self.string(references[0], depth + 1)?;
        let value = self.value(references[1], depth + 1)?;
        self.active.remove(&id);
        Ok((key, value))
    }

    fn value(&mut self, id: LinkId, depth: usize) -> Result<StructuredDataValue, String> {
        let (kind, definition, references) = self.enter(id, depth)?;
        let value = match kind.as_str() {
            "null" => StructuredDataValue::Null,
            "boolean" => StructuredDataValue::Boolean(match definition.as_deref() {
                Some("true") => true,
                Some("false") => false,
                _ => return Err(fail("a boolean definition is invalid")),
            }),
            "number" => {
                StructuredDataValue::Number(read_number(definition.as_deref().unwrap_or_default())?)
            }
            "string" => {
                StructuredDataValue::String(definition.unwrap_or_default().encode_utf16().collect())
            }
            "units" => {
                let mut units = Vec::with_capacity(references.len());
                for unit in references {
                    let (kind, definition, _) = self.enter(unit, depth + 1)?;
                    if kind != "unit" {
                        return Err(fail("a code unit string contains a foreign node"));
                    }
                    let text = definition.ok_or_else(|| fail("a code unit lacks a definition"))?;
                    let value = text
                        .parse::<u16>()
                        .map_err(|_| fail("a code unit is invalid"))?;
                    if value.to_string() != text {
                        return Err(fail("a code unit is not canonical"));
                    }
                    units.push(value);
                    self.active.remove(&unit);
                }
                StructuredDataValue::String(units)
            }
            "array" => StructuredDataValue::Array(
                references
                    .iter()
                    .map(|child| self.value(*child, depth + 1))
                    .collect::<Result<_, _>>()?,
            ),
            "object" => {
                let mut keys = BTreeSet::new();
                let mut entries = Vec::with_capacity(references.len());
                for child in references {
                    let (key, value) = self.pair(child, depth + 1)?;
                    if !keys.insert(key.clone()) {
                        return Err(fail("an object contains a duplicate key"));
                    }
                    entries.push((key, value));
                }
                StructuredDataValue::Object(entries)
            }
            _ => {
                return Err(fail(
                    "a schema root, key pair or code unit occurs as a value",
                ));
            }
        };
        self.active.remove(&id);
        Ok(value)
    }
}

/// Decode ordered values with default resource bounds.
pub fn decode_structured_data_links_ordered(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<StructuredDataValue, String> {
    decode_structured_data_links_ordered_with_limits(
        network,
        root,
        StructuredDataLinksLimits::default(),
    )
}

/// Decode ordered values, rejecting cycles, foreign nodes and excessive output.
pub fn decode_structured_data_links_ordered_with_limits(
    network: &LinkNetwork,
    root: LinkId,
    limits: StructuredDataLinksLimits,
) -> Result<StructuredDataValue, String> {
    if limits.node_limit == 0 || limits.depth_limit == 0 || limits.byte_limit == 0 {
        return Err(fail("resource limits must be positive"));
    }
    if network.len() > limits.node_limit {
        return Err(fail("the network exceeds its resource limit"));
    }
    let mut decoder = Decoder {
        network,
        limits,
        visits: 0,
        bytes: 0,
        active: HashSet::new(),
        seen: HashSet::new(),
    };
    let (kind, definition, references) = decoder.enter(root, 0)?;
    if kind != "root" || definition.as_deref() != Some("1") {
        return Err(fail("the root has an unsupported schema"));
    }
    let value = decoder.value(references[0], 1)?;
    if decoder.seen.len() != network.len() {
        return Err(fail("the network contains unreachable nodes"));
    }
    Ok(value)
}

fn from_json(value: &Value, depth: usize) -> Result<StructuredDataValue, String> {
    if depth > StructuredDataLinksLimits::default().depth_limit {
        return Err(fail("the value exceeds its structural resource limit"));
    }
    Ok(match value {
        Value::Null => StructuredDataValue::Null,
        Value::Bool(value) => StructuredDataValue::Boolean(*value),
        Value::Number(number) => {
            let value = number
                .as_f64()
                .ok_or_else(|| fail("a JSON number is not binary64"))?;
            if let Some(integer) = number.as_i64() {
                // i128 has sufficient range for all JSON integer inputs, so
                // this comparison detects binary64 rounding without saturation.
                #[allow(clippy::cast_possible_truncation)]
                let represented = value as i128;
                if represented != i128::from(integer) {
                    return Err(fail(
                        "a JSON integer is not exactly representable as binary64",
                    ));
                }
            } else if let Some(integer) = number.as_u64() {
                #[allow(clippy::cast_possible_truncation)]
                let represented = value as i128;
                if represented != i128::from(integer) {
                    return Err(fail(
                        "a JSON integer is not exactly representable as binary64",
                    ));
                }
            }
            StructuredDataValue::Number(value)
        }
        Value::String(text) => StructuredDataValue::String(text.encode_utf16().collect()),
        Value::Array(values) => StructuredDataValue::Array(
            values
                .iter()
                .map(|value| from_json(value, depth + 1))
                .collect::<Result<_, _>>()?,
        ),
        Value::Object(entries) => StructuredDataValue::Object(
            entries
                .iter()
                .map(|(key, value)| {
                    Ok((key.encode_utf16().collect(), from_json(value, depth + 2)?))
                })
                .collect::<Result<_, String>>()?,
        ),
    })
}

fn into_json(value: StructuredDataValue) -> Result<Value, String> {
    Ok(match value {
        StructuredDataValue::Null => Value::Null,
        StructuredDataValue::Boolean(value) => Value::Bool(value),
        StructuredDataValue::Number(value) => Value::Number(into_json_number(value)?),
        StructuredDataValue::String(units) => Value::String(String::from_utf16(&units).map_err(|_| fail("a JSON string cannot preserve an unpaired UTF-16 code unit; use the ordered value API"))?),
        StructuredDataValue::Array(values) => Value::Array(values.into_iter().map(into_json).collect::<Result<_, _>>()?),
        StructuredDataValue::Object(entries) => {
            let mut result = Map::new();
            for (key, value) in entries {
                let key = String::from_utf16(&key).map_err(|_| fail("a JSON key cannot preserve an unpaired UTF-16 code unit; use the ordered value API"))?;
                result.insert(key, into_json(value)?);
            }
            Value::Object(result)
        }
    })
}

fn into_json_number(value: f64) -> Result<Number, String> {
    let text = write_number(value)?;
    if text != "-0" {
        if let Ok(integer) = text.parse::<i64>() {
            return Ok(Number::from(integer));
        }
        if let Ok(integer) = text.parse::<u64>() {
            return Ok(Number::from(integer));
        }
    }
    Number::from_f64(value).ok_or_else(|| fail("a number is not finite"))
}

/// Encode a JSON-compatible value; ordered encoding retains insertion order.
pub fn encode_structured_data_links(value: &Value) -> Result<StructuredDataLinks, String> {
    encode_structured_data_links_ordered(&from_json(value, 1)?)
}

/// Decode a JSON-compatible value; use the ordered API to retain key order.
pub fn decode_structured_data_links(network: &LinkNetwork, root: LinkId) -> Result<Value, String> {
    into_json(decode_structured_data_links_ordered(network, root)?)
}

/// Decode JSON-compatible data with explicit structural limits.
pub fn decode_structured_data_links_with_limits(
    network: &LinkNetwork,
    root: LinkId,
    limits: StructuredDataLinksLimits,
) -> Result<Value, String> {
    into_json(decode_structured_data_links_ordered_with_limits(
        network, root, limits,
    )?)
}

/// Serialize a JSON-compatible value as inspectable, rooted Links Notation.
pub fn structured_data_to_lino(value: &Value) -> Result<String, String> {
    structured_data_to_lino_ordered(&from_json(value, 1)?)
}

/// Serialize an ordered value, including exact UTF-16 code unit strings.
pub fn structured_data_to_lino_ordered(value: &StructuredDataValue) -> Result<String, String> {
    let graph = encode_structured_data_links_ordered(value)?;
    Ok(graph.network.to_lino())
}

fn word(node: &LiNo<String>) -> Result<&str, String> {
    if let LiNo::Ref(word) = node {
        Ok(word)
    } else {
        Err(fail("an expected word is a nested link"))
    }
}

fn link_id(node: &LiNo<String>) -> Result<LinkId, String> {
    let word = word(node)?;
    let value = word
        .parse::<u64>()
        .map_err(|_| fail("a link identity is invalid"))?;
    if value == 0 || value > 9_007_199_254_740_991 || value.to_string() != word {
        return Err(fail("a link identity is not canonical"));
    }
    Ok(LinkId::from_u64(value))
}

fn preflight_statement(node: &LiNo<String>) -> Result<LinkId, String> {
    let LiNo::Link {
        id: Some(identity),
        values,
    } = node
    else {
        return Err(fail("a network statement lacks a numeric identity"));
    };
    let id = link_id(&LiNo::Ref(identity.clone()))?;
    let mut metadata = None;
    for child in values {
        match child {
            LiNo::Ref(_) => {
                link_id(child)?;
            }
            LiNo::Link {
                id: Some(label),
                values,
            } if label == "meta" && metadata.is_none() => {
                metadata = Some(values);
            }
            _ => {
                return Err(fail(
                    "a network statement has malformed metadata or references",
                ));
            }
        }
    }
    let mut fields = BTreeSet::new();
    for field in metadata.ok_or_else(|| fail("a network statement lacks metadata"))? {
        let LiNo::Link {
            id: Some(label),
            values,
        } = field
        else {
            return Err(fail("a metadata field has an invalid shape"));
        };
        if !matches!(label.as_str(), "t" | "n" | "term" | "def" | "lang") || !fields.insert(label) {
            return Err(fail("a metadata field is unknown or duplicated"));
        }
        if values.len() != 1 {
            return Err(fail("a metadata field must contain exactly one word"));
        }
        if label == "n" && word(&values[0])? != "1" {
            return Err(fail("the named flag is invalid"));
        }
        word(&values[0])?;
    }
    if !["t", "n", "term", "lang"]
        .iter()
        .all(|required| fields.iter().any(|field| field.as_str() == *required))
    {
        return Err(fail("a network statement lacks required metadata"));
    }
    Ok(id)
}

/// Read ordered structured data from self-describing Links Notation.
pub fn read_structured_data_lino_ordered(text: &str) -> Result<StructuredDataValue, String> {
    read_structured_data_lino_ordered_with_limits(text, StructuredDataLinksLimits::default())
}

/// Read ordered data with bounded parsing and expansion.
pub fn read_structured_data_lino_ordered_with_limits(
    text: &str,
    limits: StructuredDataLinksLimits,
) -> Result<StructuredDataValue, String> {
    if text.len() > limits.byte_limit {
        return Err(fail("the text exceeds its resource limit"));
    }
    let nodes =
        parse_lino_to_links_with_config(text, &ParserConfig::without_comments().with_max_depth(8))
            .map_err(|error| fail(&error.to_string()))?;
    if nodes.len() > limits.node_limit {
        return Err(fail("the network exceeds its resource limit"));
    }
    let mut identities = BTreeSet::new();
    for node in &nodes {
        if !identities.insert(preflight_statement(node)?) {
            return Err(fail("a network contains a duplicate identity"));
        }
    }
    drop(nodes);
    let network = LinkNetwork::from_lino(text).map_err(|error| fail(&error.to_string()))?;
    let roots: Vec<_> = network
        .links()
        .filter(|link| link.metadata().term() == Some("structured data"))
        .map(crate::link_network::Link::id)
        .collect();
    if roots.len() != 1 {
        return Err(fail(
            "a serialized value must contain exactly one schema root",
        ));
    }
    decode_structured_data_links_ordered_with_limits(&network, roots[0], limits)
}

/// Read JSON-compatible data from self-describing Links Notation.
pub fn read_structured_data_lino(text: &str) -> Result<Value, String> {
    into_json(read_structured_data_lino_ordered(text)?)
}

/// Read JSON-compatible data with explicit parsing and expansion limits.
pub fn read_structured_data_lino_with_limits(
    text: &str,
    limits: StructuredDataLinksLimits,
) -> Result<Value, String> {
    into_json(read_structured_data_lino_ordered_with_limits(text, limits)?)
}
