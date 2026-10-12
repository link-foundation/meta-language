//! Namespaced, inspectable records for concurrent append-only evidence files.
//!
//! This adapter validates record ownership before removing namespace prefixes
//! and delegates all value semantics to the shared structured-data codec.

use std::collections::HashSet;
use std::fmt::Write as _;

use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};
use serde_json::Value;

use crate::structured_data_link_rules::{
    accept_structured_data_record_field, accept_structured_data_record_shape,
};
use crate::structured_data_links::{
    StructuredDataLinks, StructuredDataLinksLimits, StructuredDataValue,
    encode_structured_data_links, encode_structured_data_links_ordered,
    read_structured_data_lino_ordered_with_limits, read_structured_data_lino_with_limits,
};

fn fail(message: &str) -> String {
    format!("structured data record: {message}")
}

fn accept_record_namespace(namespace: &str) -> bool {
    let bytes = namespace.as_bytes();
    bytes.len() == 36
        && bytes.iter().enumerate().all(|(index, byte)| {
            if matches!(index, 8 | 13 | 18 | 23) {
                *byte == b'-'
            } else {
                byte.is_ascii_digit() || (b'a'..=b'f').contains(byte)
            }
        })
        && (b'1'..=b'8').contains(&bytes[14])
        && matches!(bytes[19], b'8' | b'9' | b'a' | b'b')
}

fn encode_record_frame(graph: &StructuredDataLinks, namespace: &str) -> Result<String, String> {
    if !accept_record_namespace(namespace) {
        return Err(fail("invalid namespace"));
    }
    let mut output = format!("({namespace}/record: {namespace}/{}", graph.root);
    // The existing canonical network serializer supplies all scalar escaping
    // and metadata. Only data identities and references receive a namespace.
    let canonical = graph.network.to_lino();
    for statement in canonical.lines() {
        let (prefix, metadata) = statement
            .split_once(" (meta:")
            .ok_or_else(|| fail("canonical data statement lacks metadata"))?;
        output.push_str(" (");
        for (position, identity) in prefix[1..].split_ascii_whitespace().enumerate() {
            if position > 0 {
                output.push(' ');
            }
            write!(output, "{namespace}/{identity}").expect("writing a string succeeds");
        }
        output.push_str(" (meta:");
        output.push_str(metadata);
    }
    output.push_str(" (meta: (t: semantic) (n: 1) (term: structured%20data%20record) (def: 1) (lang: meta-language%20structured%20data)))\n");
    Ok(output)
}

/// Encode a JSON-compatible observation with a fresh globally unique namespace.
pub fn structured_data_record_to_lino(value: &Value) -> Result<String, String> {
    structured_data_record_to_lino_with_namespace(value, &uuid::Uuid::new_v4().to_string())
}

/// Encode an observation with an explicitly selected record namespace.
pub fn structured_data_record_to_lino_with_namespace(
    value: &Value,
    namespace: &str,
) -> Result<String, String> {
    encode_record_frame(&encode_structured_data_links(value)?, namespace)
}

/// Encode ordered data, retaining exact UTF-16 keys and values.
pub fn structured_data_record_to_lino_ordered(
    value: &StructuredDataValue,
) -> Result<String, String> {
    structured_data_record_to_lino_ordered_with_namespace(value, &uuid::Uuid::new_v4().to_string())
}

/// Encode ordered data with a reproducible explicit namespace.
pub fn structured_data_record_to_lino_ordered_with_namespace(
    value: &StructuredDataValue,
    namespace: &str,
) -> Result<String, String> {
    encode_record_frame(&encode_structured_data_links_ordered(value)?, namespace)
}

fn read_local_identity<'a>(identity: &'a str, namespace: &str) -> Result<&'a str, String> {
    let local = identity
        .strip_prefix(namespace)
        .and_then(|identity| identity.strip_prefix('/'))
        .ok_or_else(|| fail("foreign reference namespace"))?;
    let number = local
        .parse::<u64>()
        .map_err(|_| fail("invalid local identity"))?;
    if number == 0 || number > 9_007_199_254_740_991 || number.to_string() != local {
        return Err(fail("invalid local identity"));
    }
    Ok(local)
}

fn write_record_metadata(node: &LiNo<String>, output: &mut String) -> Result<(), String> {
    match node {
        LiNo::Ref(value) => {
            // Canonical data metadata contains one escape-free token per field.
            if value.is_empty()
                || value.chars().any(|character| {
                    character.is_whitespace() || matches!(character, '(' | ')' | ':' | '"' | '\'')
                })
            {
                return Err(fail("invalid data metadata token"));
            }
            output.push_str(value);
        }
        LiNo::Link {
            id: Some(label),
            values,
        } => {
            if !matches!(label.as_str(), "meta" | "t" | "n" | "term" | "def" | "lang") {
                return Err(fail("invalid data metadata field"));
            }
            write!(output, "({label}:").expect("writing a string succeeds");
            for value in values {
                output.push(' ');
                write_record_metadata(value, output)?;
            }
            output.push(')');
        }
        LiNo::Link { id: None, .. } => {
            return Err(fail("invalid data metadata envelope"));
        }
    }
    Ok(())
}

fn read_local_frame(
    frame: &LiNo<String>,
    namespaces: &mut HashSet<String>,
    limits: StructuredDataLinksLimits,
) -> Result<String, String> {
    let LiNo::Link {
        id: Some(identity),
        values,
    } = frame
    else {
        return Err(fail("invalid frame identity"));
    };
    let namespace = identity
        .strip_suffix("/record")
        .filter(|namespace| accept_record_namespace(namespace))
        .ok_or_else(|| fail("invalid namespace"))?;
    if !namespaces.insert(namespace.to_owned()) {
        return Err(fail("duplicate namespace"));
    }
    let mut references = Vec::new();
    let mut metadata = Vec::new();
    let mut declarations = Vec::new();
    for value in values {
        match value {
            LiNo::Ref(reference) => references.push(reference),
            LiNo::Link {
                id: Some(label),
                values,
            } if label == "meta" => metadata.push(values),
            LiNo::Link { .. } => declarations.push(value),
        }
    }
    if !accept_structured_data_record_shape(
        &references.len().to_string(),
        &metadata.len().to_string(),
    ) {
        return Err(fail("invalid frame shape"));
    }
    let root = read_local_identity(references[0], namespace)?;
    let mut fields = HashSet::new();
    for field in metadata[0] {
        let LiNo::Link {
            id: Some(label),
            values,
        } = field
        else {
            return Err(fail("invalid frame metadata"));
        };
        if values.len() != 1 || !fields.insert(label.as_str()) {
            return Err(fail("invalid frame metadata"));
        }
        let LiNo::Ref(value) = &values[0] else {
            return Err(fail("invalid frame metadata"));
        };
        if !accept_structured_data_record_field(label, value) {
            return Err(fail("invalid frame metadata"));
        }
    }
    if fields.len() != 5 {
        return Err(fail("incomplete frame metadata"));
    }
    if declarations.len() > limits.node_limit {
        return Err(fail("node resource limit exceeded"));
    }
    let mut identities = HashSet::new();
    let mut declared_root = false;
    let mut output = String::new();
    for declaration in declarations {
        let LiNo::Link {
            id: Some(identity),
            values,
        } = declaration
        else {
            return Err(fail("invalid data declaration"));
        };
        let identity = read_local_identity(identity, namespace)?;
        if !identities.insert(identity) {
            return Err(fail("duplicate local identity"));
        }
        if identity == root {
            declared_root = values.iter().any(|value| {
                let LiNo::Link { id: Some(label), values } = value else {
                    return false;
                };
                label == "meta" && values.iter().any(|field| {
                    matches!(field, LiNo::Link { id: Some(label), values }
                        if label == "term" && values.as_slice() == [LiNo::Ref("structured%20data".to_owned())])
                })
            });
        }
        write!(output, "({identity}:").expect("writing a string succeeds");
        for value in values {
            output.push(' ');
            if let LiNo::Ref(reference) = value {
                output.push_str(read_local_identity(reference, namespace)?);
            } else {
                write_record_metadata(value, &mut output)?;
            }
        }
        output.push_str(")\n");
    }
    // The shared codec independently validates closed references, unique roots,
    // metadata, cycles, structural roles and the decoded expansion budget.
    if !declared_root {
        return Err(fail("frame root is not a declared schema root"));
    }
    Ok(output)
}

fn read_record_frames<T>(
    source: &str,
    limits: StructuredDataLinksLimits,
    decode: impl Fn(&str, StructuredDataLinksLimits) -> Result<T, String>,
) -> Result<Vec<T>, String> {
    if source.len() > limits.byte_limit {
        return Err(fail("byte resource limit exceeded"));
    }
    let frames = parse_lino_to_links_with_config(
        source,
        &ParserConfig::without_comments().with_max_depth(10),
    )
    .map_err(|error| fail(&error.to_string()))?;
    if frames.len() > limits.node_limit {
        return Err(fail("record resource limit exceeded"));
    }
    let mut namespaces = HashSet::new();
    frames
        .iter()
        .map(|frame| decode(&read_local_frame(frame, &mut namespaces, limits)?, limits))
        .collect()
}

/// Read a bounded record document as JSON-compatible values.
pub fn read_structured_data_records(source: &str) -> Result<Vec<Value>, String> {
    read_structured_data_records_with_limits(source, StructuredDataLinksLimits::default())
}

/// Read JSON-compatible records with explicit parsing and expansion limits.
pub fn read_structured_data_records_with_limits(
    source: &str,
    limits: StructuredDataLinksLimits,
) -> Result<Vec<Value>, String> {
    read_record_frames(source, limits, read_structured_data_lino_with_limits)
}

/// Read ordered records, preserving exact UTF-16 strings and entry order.
pub fn read_structured_data_records_ordered(
    source: &str,
) -> Result<Vec<StructuredDataValue>, String> {
    read_structured_data_records_ordered_with_limits(source, StructuredDataLinksLimits::default())
}

/// Read ordered records with explicit parsing and expansion limits.
pub fn read_structured_data_records_ordered_with_limits(
    source: &str,
    limits: StructuredDataLinksLimits,
) -> Result<Vec<StructuredDataValue>, String> {
    read_record_frames(
        source,
        limits,
        read_structured_data_lino_ordered_with_limits,
    )
}
