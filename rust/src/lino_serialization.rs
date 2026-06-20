//! Lossless serialization of a [`LinkNetwork`] to and from links-notation text.
//!
//! [`LinkNetwork::to_lino`] projects every link in the network onto a single
//! canonical links-notation statement, keyed by the link's numeric identifier
//! (a doublets-style id discipline shared by text and future binary storage).
//! [`LinkNetwork::from_lino`] reconstructs the exact same network from that
//! text. The pair forms a round-trip: `from_lino(to_lino(n))` is isomorphic to
//! `n` for any network, covering references, names, types, terms, definitions,
//! languages, source spans, parse flags, and term registration.
//!
//! The emitted dialect is plain links-notation accepted by the
//! [`links_notation`] 0.13 crate, so other ecosystem parsers can consume the
//! output. Each statement has the shape:
//!
//! ```text
//! (<id>: <ref> ... (meta: (t: <type>) (n: <0|1>) (term: <pct>) ...))
//! ```
//!
//! where references are decimal link ids and the trailing `meta` sublink
//! carries metadata. String payloads (`term`, `def`, `lang`) are
//! percent-encoded so they always form a single escape-free reference token,
//! sidestepping the crate's quote-escaping edge cases. The `meta` keys are
//! non-numeric, so they never collide with numeric reference ids, and the
//! references (`Ref` nodes) are structurally distinct from the `meta` sublink
//! (a `Link` node) in the parsed AST.
//!
//! This is distinct from [`LinkNetwork::parse`] with the `"LiNo"` language,
//! which interprets human-authored links-notation into a fresh semantic
//! network. `to_lino`/`from_lino` are an exact serialization pair for an
//! already-built network.

use std::error::Error;
use std::fmt;
use std::fmt::Write as _;
use std::sync::Arc;

use links_notation::{parse_lino_to_links, LiNo};

use crate::link_flags::LinkFlags;
use crate::link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::source::{ByteRange, Point, SourceSpan};

/// Error returned when [`LinkNetwork::from_lino`] cannot reconstruct a network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinoSerializationError {
    /// The text could not be parsed as links-notation.
    Parse(String),
    /// The text parsed but did not match the serialization schema.
    Structure(String),
}

impl fmt::Display for LinoSerializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(message) => write!(formatter, "links-notation parse error: {message}"),
            Self::Structure(message) => {
                write!(formatter, "serialization structure error: {message}")
            }
        }
    }
}

impl Error for LinoSerializationError {}

impl LinkNetwork {
    /// Serializes the entire network to canonical links-notation text.
    ///
    /// Every link becomes one statement keyed by its numeric id. The output is
    /// accepted by the [`links_notation`] crate parser and round-trips back
    /// through [`LinkNetwork::from_lino`].
    #[must_use]
    pub fn to_lino(&self) -> String {
        let registered: std::collections::BTreeSet<u64> =
            self.terms.values().map(|id| id.0).collect();
        let mut output = String::new();
        for link in self.links.values() {
            encode_link(link, registered.contains(&link.id.0), &mut output);
            output.push('\n');
        }
        output
    }

    /// Reconstructs a network from text produced by [`LinkNetwork::to_lino`].
    ///
    /// # Errors
    ///
    /// Returns [`LinoSerializationError`] when the text is not valid
    /// links-notation or does not match the serialization schema.
    pub fn from_lino(text: &str) -> Result<Self, LinoSerializationError> {
        let statements = parse_lino_to_links(text)
            .map_err(|error| LinoSerializationError::Parse(error.to_string()))?;
        let mut network = Self::new();
        for statement in &statements {
            let LiNo::Link {
                id: Some(id),
                values,
            } = statement
            else {
                return Err(LinoSerializationError::Structure(
                    "top-level statement must be an identified link".to_string(),
                ));
            };
            let link_id = LinkId(parse_u64(id)?);
            let mut references = Vec::new();
            let mut meta_values: Option<&Vec<LiNo<String>>> = None;
            for value in values {
                match value {
                    LiNo::Ref(reference) => references.push(LinkId(parse_u64(reference)?)),
                    LiNo::Link {
                        id: Some(key),
                        values: fields,
                    } if key == "meta" => meta_values = Some(fields),
                    LiNo::Link { .. } => {
                        return Err(LinoSerializationError::Structure(
                            "statement values must be references or a meta sublink".to_string(),
                        ))
                    }
                }
            }
            let meta_values = meta_values.ok_or_else(|| {
                LinoSerializationError::Structure(
                    "statement is missing its meta sublink".to_string(),
                )
            })?;
            let (metadata, registered) = decode_meta(meta_values)?;
            if registered {
                if let Some(term) = metadata.term() {
                    network.terms.insert(Arc::from(term), link_id);
                }
            }
            network.next_id = network.next_id.max(link_id.0 + 1);
            network.links.insert(
                link_id,
                Arc::new(Link {
                    id: link_id,
                    references: Arc::from(references),
                    metadata,
                }),
            );
        }
        Ok(network)
    }
}

/// Writes one `(<id>: <refs> (meta: ...))` statement into `output`.
fn encode_link(link: &Link, registered: bool, output: &mut String) {
    write!(output, "({}:", link.id.0).expect("writing to a String never fails");
    for reference in link.references.iter() {
        write!(output, " {}", reference.0).expect("writing to a String never fails");
    }
    output.push_str(" (meta:");
    let metadata = &link.metadata;
    if let Some(link_type) = metadata.link_type() {
        write!(output, " (t: {link_type})").expect("writing to a String never fails");
    }
    write!(output, " (n: {})", u8::from(metadata.is_named()))
        .expect("writing to a String never fails");
    if let Some(term) = metadata.term() {
        write!(output, " (term: {})", percent_encode(term))
            .expect("writing to a String never fails");
    }
    if let Some(definition) = metadata.definition() {
        write!(output, " (def: {})", percent_encode(definition))
            .expect("writing to a String never fails");
    }
    if let Some(language) = metadata.language() {
        write!(output, " (lang: {})", percent_encode(language))
            .expect("writing to a String never fails");
    }
    if let Some(span) = metadata.span() {
        let byte_range = span.byte_range();
        let start = span.start_point();
        let end = span.end_point();
        write!(
            output,
            " (span: {} {} {} {} {} {})",
            byte_range.start(),
            byte_range.end(),
            start.row(),
            start.column(),
            end.row(),
            end.column(),
        )
        .expect("writing to a String never fails");
    }
    let bits = flag_bits(metadata.flags());
    if bits != 0 {
        write!(output, " (flags: {bits})").expect("writing to a String never fails");
    }
    if registered {
        output.push_str(" (reg: 1)");
    }
    output.push_str("))");
}

/// Decodes a `meta` sublink's fields into metadata and a registration flag.
fn decode_meta(fields: &[LiNo<String>]) -> Result<(LinkMetadata, bool), LinoSerializationError> {
    let mut metadata = LinkMetadata::new();
    let mut registered = false;
    let mut flag_bits = 0u8;
    for field in fields {
        let LiNo::Link {
            id: Some(key),
            values,
        } = field
        else {
            return Err(LinoSerializationError::Structure(
                "meta field must be an identified link".to_string(),
            ));
        };
        match key.as_str() {
            "t" => metadata = metadata.with_link_type(parse_link_type(single_ref(values)?)?),
            "n" => metadata = metadata.with_named(single_ref(values)? == "1"),
            "term" => metadata = metadata.with_term(percent_decode(single_ref(values)?)?),
            "def" => metadata = metadata.with_definition(percent_decode(single_ref(values)?)?),
            "lang" => metadata = metadata.with_language(percent_decode(single_ref(values)?)?),
            "span" => metadata = metadata.with_span(parse_span(values)?),
            "flags" => flag_bits = parse_u8(single_ref(values)?)?,
            "reg" => registered = true,
            other => {
                return Err(LinoSerializationError::Structure(format!(
                    "unknown meta field `{other}`"
                )))
            }
        }
    }
    if flag_bits != 0 {
        let mut flags = LinkFlags::clean();
        if flag_bits & 0b0001 != 0 {
            flags = flags.with_error();
        }
        if flag_bits & 0b0010 != 0 {
            flags = flags.with_containing_error();
        }
        if flag_bits & 0b0100 != 0 {
            flags = flags.with_missing();
        }
        if flag_bits & 0b1000 != 0 {
            flags = flags.with_extra();
        }
        metadata = metadata.with_flags(flags);
    }
    Ok((metadata, registered))
}

/// Packs the four parse-status bits into a single byte.
fn flag_bits(flags: LinkFlags) -> u8 {
    u8::from(flags.is_error())
        | (u8::from(flags.has_error()) << 1)
        | (u8::from(flags.is_missing()) << 2)
        | (u8::from(flags.is_extra()) << 3)
}

/// Builds a [`SourceSpan`] from the six decimal values of a `span` field.
fn parse_span(values: &[LiNo<String>]) -> Result<SourceSpan, LinoSerializationError> {
    if values.len() != 6 {
        return Err(LinoSerializationError::Structure(
            "span field requires six numbers".to_string(),
        ));
    }
    let mut numbers = [0usize; 6];
    for (slot, value) in numbers.iter_mut().zip(values) {
        let LiNo::Ref(reference) = value else {
            return Err(LinoSerializationError::Structure(
                "span field values must be numbers".to_string(),
            ));
        };
        *slot = reference.parse().map_err(|_| {
            LinoSerializationError::Structure(format!("invalid span number `{reference}`"))
        })?;
    }
    Ok(SourceSpan::new(
        ByteRange::new(numbers[0], numbers[1]),
        Point::new(numbers[2], numbers[3]),
        Point::new(numbers[4], numbers[5]),
    ))
}

/// Returns the single reference held by a one-value meta field.
fn single_ref(values: &[LiNo<String>]) -> Result<&str, LinoSerializationError> {
    match values {
        [LiNo::Ref(reference)] => Ok(reference),
        _ => Err(LinoSerializationError::Structure(
            "meta field must hold exactly one reference".to_string(),
        )),
    }
}

/// Maps a link-type token back to its [`LinkType`] variant.
fn parse_link_type(token: &str) -> Result<LinkType, LinoSerializationError> {
    Ok(match token {
        "link" => LinkType::Link,
        "reference" => LinkType::Reference,
        "relation" => LinkType::Relation,
        "language" => LinkType::Language,
        "grammar" => LinkType::Grammar,
        "type" => LinkType::Type,
        "concept" => LinkType::Concept,
        "syntax" => LinkType::Syntax,
        "field" => LinkType::Field,
        "trivia" => LinkType::Trivia,
        "token" => LinkType::Token,
        "document" => LinkType::Document,
        "semantic" => LinkType::Semantic,
        "region" => LinkType::Region,
        "object" => LinkType::Object,
        other => {
            return Err(LinoSerializationError::Structure(format!(
                "unknown link type `{other}`"
            )))
        }
    })
}

fn parse_u64(value: &str) -> Result<u64, LinoSerializationError> {
    value
        .parse()
        .map_err(|_| LinoSerializationError::Structure(format!("invalid link id `{value}`")))
}

fn parse_u8(value: &str) -> Result<u8, LinoSerializationError> {
    value
        .parse()
        .map_err(|_| LinoSerializationError::Structure(format!("invalid flags value `{value}`")))
}

/// Percent-encodes a string into a single escape-free links-notation token.
///
/// The empty string maps to the sentinel `%`, which never results from
/// encoding a non-empty string (a literal `%` byte becomes `%25`).
fn percent_encode(value: &str) -> String {
    if value.is_empty() {
        return "%".to_string();
    }
    let mut encoded = String::with_capacity(value.len());
    for &byte in value.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.') {
            encoded.push(byte as char);
        } else {
            write!(encoded, "%{byte:02X}").expect("writing to a String never fails");
        }
    }
    encoded
}

/// Reverses [`percent_encode`].
fn percent_decode(value: &str) -> Result<String, LinoSerializationError> {
    if value == "%" {
        return Ok(String::new());
    }
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return Err(LinoSerializationError::Structure(
                    "truncated percent escape".to_string(),
                ));
            }
            let high = hex_value(bytes[index + 1])?;
            let low = hex_value(bytes[index + 2])?;
            decoded.push((high << 4) | low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).map_err(|_| {
        LinoSerializationError::Structure("percent escape is not valid UTF-8".to_string())
    })
}

fn hex_value(byte: u8) -> Result<u8, LinoSerializationError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(LinoSerializationError::Structure(
            "invalid percent escape digit".to_string(),
        )),
    }
}
