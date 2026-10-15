//! Complete network-state binary archives under the shared interoperable profile.
//! The generated JavaScript algorithm owns framing, bounds and byte decisions;
//! this adapter supplies native UTF-8 strings and link-network objects.

use std::error::Error;
use std::fmt;
use std::sync::{Arc, OnceLock};

use serde_json::Value;

use crate::link_network::Link;
use crate::link_network_binary_record_rules::transcode_link_network_binary;
use crate::link_network_binary_rules::accept_link_network_binary_integer;
use crate::{
    ByteRange, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, Point, SourceSpan,
    read_structured_data_lino,
};

/// Failure to preserve a network under the selected binary profile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinaryLinksError(String);

impl fmt::Display for BinaryLinksError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "binary links snapshot: {}", self.0)
    }
}
impl Error for BinaryLinksError {}

/// Resource bounds for complete binary network reconstruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BinaryLinksOptions {
    /// Maximum link records and entries in each exact cache section.
    pub record_limit: usize,
    /// Maximum archive bytes and intermediate flat-field entries.
    pub byte_limit: usize,
}

fn schema() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        read_structured_data_lino(include_str!("data/link-network-binary-schema.lino"))
            .expect("the packaged binary network schema is valid")
    })
}

impl Default for BinaryLinksOptions {
    fn default() -> Self {
        Self {
            record_limit: usize::try_from(
                schema()["recordLimit"]
                    .as_u64()
                    .expect("record limit is an integer"),
            )
            .expect("record limit fits usize"),
            byte_limit: usize::try_from(
                schema()["byteLimit"]
                    .as_u64()
                    .expect("byte limit is an integer"),
            )
            .expect("byte limit fits usize"),
        }
    }
}

fn fail(message: impl Into<String>) -> BinaryLinksError {
    BinaryLinksError(message.into())
}

// Only integers in the exact Number domain cross the portable numeric boundary.
#[allow(clippy::cast_precision_loss)]
fn number(value: u64) -> Result<f64, BinaryLinksError> {
    if value > 9_007_199_254_740_991 {
        return Err(fail(
            "unsafe integer is outside the interoperable Number profile",
        ));
    }
    Ok(value as f64)
}

fn length(value: usize) -> Result<f64, BinaryLinksError> {
    number(u64::try_from(value).map_err(|_| fail("length does not fit the interoperable profile"))?)
}

fn append_text(fields: &mut Vec<f64>, value: &str) -> Result<(), BinaryLinksError> {
    fields.push(length(value.len())?);
    fields.extend(value.bytes().map(f64::from));
    Ok(())
}

fn append_optional_text(
    fields: &mut Vec<f64>,
    value: Option<&str>,
) -> Result<(), BinaryLinksError> {
    fields.push(f64::from(u8::from(value.is_some())));
    if let Some(value) = value {
        append_text(fields, value)?;
    }
    Ok(())
}

const fn type_code(value: Option<LinkType>) -> u8 {
    match value {
        None => 0,
        Some(LinkType::Link) => 1,
        Some(LinkType::Reference) => 2,
        Some(LinkType::Relation) => 3,
        Some(LinkType::Language) => 4,
        Some(LinkType::Grammar) => 5,
        Some(LinkType::Type) => 6,
        Some(LinkType::Concept) => 7,
        Some(LinkType::Syntax) => 8,
        Some(LinkType::Field) => 9,
        Some(LinkType::Trivia) => 10,
        Some(LinkType::Token) => 11,
        Some(LinkType::Document) => 12,
        Some(LinkType::Semantic) => 13,
        Some(LinkType::Region) => 14,
        Some(LinkType::Object) => 15,
    }
}

const fn types() -> [Option<LinkType>; 16] {
    [
        None,
        Some(LinkType::Link),
        Some(LinkType::Reference),
        Some(LinkType::Relation),
        Some(LinkType::Language),
        Some(LinkType::Grammar),
        Some(LinkType::Type),
        Some(LinkType::Concept),
        Some(LinkType::Syntax),
        Some(LinkType::Field),
        Some(LinkType::Trivia),
        Some(LinkType::Token),
        Some(LinkType::Document),
        Some(LinkType::Semantic),
        Some(LinkType::Region),
        Some(LinkType::Object),
    ]
}

fn transcode(
    fields: &[f64],
    binary: bool,
    options: BinaryLinksOptions,
) -> Result<String, BinaryLinksError> {
    let record_limit = length(options.record_limit)?;
    let byte_limit = length(options.byte_limit)?;
    if !accept_link_network_binary_integer(record_limit)
        || !accept_link_network_binary_integer(byte_limit)
    {
        return Err(fail("invalid resource limits"));
    }
    let result =
        transcode_link_network_binary(fields, binary, record_limit, byte_limit).to_string();
    if result.starts_with('-') {
        let mut pieces = result.split(',');
        let code = pieces.next().unwrap_or("-1").trim_start_matches('-');
        let offset = pieces.next().unwrap_or("0");
        let message = schema()["errors"][code]
            .as_str()
            .unwrap_or("unknown profile failure");
        return Err(fail(format!(
            "{message} at {} {offset}",
            if binary { "byte" } else { "field" }
        )));
    }
    Ok(result)
}

/// Encode every graph link and exact lookup/selection/allocation state.
///
/// # Errors
/// Returns a precise error for unsafe integers, resource bounds or unsupported
/// states in the shared safe-integer and scalar-Unicode profile.
pub fn encode_link_network_binary(
    network: &LinkNetwork,
    options: BinaryLinksOptions,
) -> Result<Vec<u8>, BinaryLinksError> {
    let mut fields = vec![2.0, number(network.next_id)?, length(network.links.len())?];
    if network.links.len() > options.record_limit {
        return Err(fail("record resource limit exceeded"));
    }
    for link in network.links() {
        fields.push(number(link.id().as_u64())?);
        fields.push(length(link.references().len())?);
        for reference in link.references() {
            fields.push(number(reference.as_u64())?);
        }
        let value = link.metadata();
        let flags = value.flags();
        let bits = u8::from(flags.is_error())
            | (u8::from(flags.has_error()) << 1)
            | (u8::from(flags.is_missing()) << 2)
            | (u8::from(flags.is_extra()) << 3);
        fields.extend([
            1.0,
            f64::from(type_code(value.link_type())),
            f64::from(u8::from(value.is_named())),
            f64::from(bits),
            f64::from(u8::from(
                value
                    .term()
                    .is_some_and(|term| network.find_term(term) == Some(link.id())),
            )),
            0.0,
        ]);
        for text in [value.term(), value.definition(), value.language()] {
            append_optional_text(&mut fields, text)?;
        }
        fields.push(f64::from(u8::from(value.span().is_some())));
        if let Some(span) = value.span() {
            for component in [
                span.byte_range().start(),
                span.byte_range().end(),
                span.start_point().row(),
                span.start_point().column(),
                span.end_point().row(),
                span.end_point().column(),
            ] {
                fields.push(length(component)?);
            }
        }
        if fields.len() > options.byte_limit {
            return Err(fail("byte resource limit exceeded"));
        }
    }
    fields.push(length(network.terms.len())?);
    for (term, identity) in &network.terms {
        append_text(&mut fields, term)?;
        fields.push(number(identity.as_u64())?);
    }
    fields.push(length(network.concept_syntax.len())?);
    for ((concept, language), syntax) in &network.concept_syntax {
        if concept.contains('\0') || language.contains('\0') {
            return Err(fail(
                "ambiguous concept-syntax cache key is outside the interoperable profile",
            ));
        }
        for text in [concept.as_ref(), language.as_ref(), syntax.as_ref()] {
            append_text(&mut fields, text)?;
        }
    }
    let encoded = transcode(&fields, false, options)?;
    (0..encoded.len())
        .step_by(2)
        .map(|offset| {
            u8::from_str_radix(&encoded[offset..offset + 2], 16)
                .map_err(|_| fail("generated byte framing is not hexadecimal"))
        })
        .collect()
}

struct Fields {
    values: Vec<u64>,
    cursor: usize,
}
impl Fields {
    fn number(&mut self) -> u64 {
        let value = self.values[self.cursor];
        self.cursor += 1;
        value
    }
    fn size(&mut self) -> Result<usize, BinaryLinksError> {
        usize::try_from(self.number()).map_err(|_| fail("value does not fit usize"))
    }
    fn text(&mut self) -> Result<String, BinaryLinksError> {
        let length = self.size()?;
        let bytes = self.values[self.cursor..self.cursor + length]
            .iter()
            .map(|value| u8::try_from(*value).map_err(|_| fail("invalid string byte")))
            .collect::<Result<Vec<_>, _>>()?;
        self.cursor += length;
        String::from_utf8(bytes).map_err(|_| fail("metadata or cache string is not valid UTF-8"))
    }
    fn optional_text(&mut self) -> Result<Option<String>, BinaryLinksError> {
        if self.number() == 0 {
            Ok(None)
        } else {
            self.text().map(Some)
        }
    }
}

/// Reload exact network state without source parsing or token reconstruction.
/// Legacy version 1 restores its graph and recorded term registrations only.
///
/// # Errors
/// Returns a precise error for malformed framing, unsupported profile values,
/// invalid UTF-8, duplicate state keys or exceeded resource limits.
pub fn decode_link_network_binary(
    bytes: &[u8],
    options: BinaryLinksOptions,
) -> Result<LinkNetwork, BinaryLinksError> {
    if bytes.len() > options.byte_limit {
        return Err(fail("byte resource limit exceeded"));
    }
    let input = bytes
        .iter()
        .map(|byte| f64::from(*byte))
        .collect::<Vec<_>>();
    let decoded = transcode(&input, true, options)?;
    let values = decoded
        .trim_end_matches(',')
        .split(',')
        .map(|value| {
            value
                .parse::<u64>()
                .map_err(|_| fail("generated numeric framing is invalid"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut fields = Fields { values, cursor: 0 };
    let version = fields.number();
    let next_identity = if version == 2 { fields.number() } else { 1 };
    let count = fields.size()?;
    let mut network = LinkNetwork::new();
    for _ in 0..count {
        let identity = LinkId::from_u64(fields.number());
        let arity = fields.size()?;
        let references = (0..arity)
            .map(|_| LinkId::from_u64(fields.number()))
            .collect::<Vec<_>>();
        fields.number(); // Shared scanner checked the metadata version.
        let link_type = types()[fields.size()?];
        let named = fields.number() == 1;
        let bits = fields.number();
        let registered = fields.number() == 1;
        fields.number(); // Shared scanner rejected tombstones.
        let term = fields.optional_text()?;
        let definition = fields.optional_text()?;
        let language = fields.optional_text()?;
        let span = if fields.number() == 1 {
            Some(SourceSpan::new(
                ByteRange::new(fields.size()?, fields.size()?),
                Point::new(fields.size()?, fields.size()?),
                Point::new(fields.size()?, fields.size()?),
            ))
        } else {
            None
        };
        let mut metadata = LinkMetadata::new().with_named(named);
        if let Some(value) = link_type {
            metadata = metadata.with_link_type(value);
        }
        if let Some(value) = term {
            metadata = metadata.with_term(value);
        }
        if let Some(value) = definition {
            metadata = metadata.with_definition(value);
        }
        if let Some(value) = language {
            metadata = metadata.with_language(value);
        }
        if let Some(value) = span {
            metadata = metadata.with_span(value);
        }
        let mut flags = LinkFlags::clean();
        if bits & 1 != 0 {
            flags = flags.with_error();
        }
        if bits & 2 != 0 {
            flags = flags.with_containing_error();
        }
        if bits & 4 != 0 {
            flags = flags.with_missing();
        }
        if bits & 8 != 0 {
            flags = flags.with_extra();
        }
        metadata = metadata.with_flags(flags);
        if version == 1
            && registered
            && let Some(term) = metadata.term()
        {
            network.terms.insert(Arc::from(term), identity);
        }
        network.next_id = network.next_id.max(identity.as_u64() + 1);
        network.links.insert(
            identity,
            Arc::new(Link {
                id: identity,
                references: Arc::from(references),
                metadata,
            }),
        );
    }
    if version == 2 {
        let terms = fields.size()?;
        for _ in 0..terms {
            let key: Arc<str> = Arc::from(fields.text()?);
            let identity = LinkId::from_u64(fields.number());
            if network.terms.insert(key, identity).is_some() {
                return Err(fail("duplicate term-cache key"));
            }
        }
        let syntax = fields.size()?;
        for _ in 0..syntax {
            let concept: Arc<str> = Arc::from(fields.text()?);
            let language: Arc<str> = Arc::from(fields.text()?);
            let value: Arc<str> = Arc::from(fields.text()?);
            if concept.contains('\0') || language.contains('\0') {
                return Err(fail(
                    "ambiguous concept-syntax cache key is outside the interoperable profile",
                ));
            }
            if network
                .concept_syntax
                .insert((concept, language), value)
                .is_some()
            {
                return Err(fail("duplicate selected-syntax key"));
            }
        }
        network.next_id = next_identity;
    }
    Ok(network)
}
