//! Namespaced, inspectable records for concurrent append-only evidence files.
//!
//! This adapter validates record ownership before removing namespace prefixes
//! and delegates all value semantics to the shared structured-data codec.

use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::hash::{BuildHasher, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::structured_data_link_rules::{
    accept_structured_data_record_field, accept_structured_data_record_shape,
};
use crate::structured_data_links::{
    StructuredDataLinks, StructuredDataLinksLimits, StructuredDataValue,
    encode_structured_data_links, encode_structured_data_links_ordered,
    read_structured_data_lino_ordered_with_limits, read_structured_data_lino_with_limits,
};
use crate::{LinkId, LinkNetwork};

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

fn create_record_namespace() -> String {
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut seed = RandomState::new().build_hasher();
    seed.write_u64(sequence);
    let mut digest = Sha256::new();
    digest.update(seed.finish().to_le_bytes());
    digest.update(std::process::id().to_le_bytes());
    digest.update(sequence.to_le_bytes());
    digest.update(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_le_bytes(),
    );
    let digest = digest.finalize();
    let mut bytes = [0_u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    // Version 8 declares this application-selected namespace construction;
    // the variant retains the RFC UUID layout. Readers reject collisions.
    bytes[6] = (bytes[6] & 0x0f) | 0x80;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let mut namespace = String::with_capacity(36);
    for (index, byte) in bytes.iter().enumerate() {
        if matches!(index, 4 | 6 | 8 | 10) {
            namespace.push('-');
        }
        write!(namespace, "{byte:02x}").expect("writing a string succeeds");
    }
    namespace
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
    if output.len() > StructuredDataLinksLimits::default().byte_limit {
        return Err(fail("serialized frame byte resource limit exceeded"));
    }
    Ok(output)
}

/// Encode a JSON-compatible observation with a fresh globally unique namespace.
pub fn structured_data_record_to_lino(value: &Value) -> Result<String, String> {
    structured_data_record_to_lino_with_namespace(value, &create_record_namespace())
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
    structured_data_record_to_lino_ordered_with_namespace(value, &create_record_namespace())
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
) -> Result<(String, LinkId), String> {
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
                        if label == "term" && matches!(values.as_slice(), [LiNo::Ref(term)] if term == "structured%20data"))
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
    Ok((
        output,
        LinkId::from_u64(root.parse().map_err(|_| fail("invalid root identity"))?),
    ))
}

fn read_record_graph_cost(
    network: &LinkNetwork,
    root: LinkId,
    limits: StructuredDataLinksLimits,
) -> Result<(usize, usize), String> {
    fn visit(
        network: &LinkNetwork,
        id: LinkId,
        depth: usize,
        limits: StructuredDataLinksLimits,
        costs: &mut HashMap<LinkId, (usize, usize)>,
    ) -> Result<(usize, usize), String> {
        if depth > limits.depth_limit {
            return Err(fail("depth resource limit exceeded"));
        }
        if let Some(cost) = costs.get(&id) {
            return Ok(*cost);
        }
        let link = network.link(id).ok_or_else(|| fail("dangling reference"))?;
        let mut cost = (1_usize, link.metadata().definition().map_or(0, str::len));
        for reference in link.references() {
            let child = visit(network, *reference, depth + 1, limits, costs)?;
            cost.0 = cost.0.saturating_add(child.0);
            cost.1 = cost.1.saturating_add(child.1);
            if cost.0 > limits.node_limit || cost.1 > limits.byte_limit {
                return Err(fail("expanded data resource limit exceeded"));
            }
        }
        costs.insert(id, cost);
        Ok(cost)
    }
    // The shared decoder has already rejected cycles and malformed graphs.
    visit(network, root, 0, limits, &mut HashMap::new())
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
    let mut records = Vec::new();
    let mut used = (0_usize, 0_usize);
    for frame in &frames {
        let (local, root) = read_local_frame(frame, &mut namespaces, limits)?;
        let value = decode(&local, limits)?;
        let network = LinkNetwork::from_lino(&local).map_err(|error| fail(&error.to_string()))?;
        let cost = read_record_graph_cost(&network, root, limits)?;
        used.0 = used.0.saturating_add(cost.0);
        used.1 = used.1.saturating_add(cost.1);
        if used.0 > limits.node_limit || used.1 > limits.byte_limit {
            return Err(fail("aggregate record resource limit exceeded"));
        }
        records.push(value);
    }
    Ok(records)
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
