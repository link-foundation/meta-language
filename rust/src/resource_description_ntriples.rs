//! Selected N-Triples syntax using a checked, automatically translated kernel.
//! Native code collects validated scalar results and supplies link identities.

mod provenance;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::sync::OnceLock;

use serde_json::{Value, json};

use crate::resource_description_ntriples_rules::{
    accept_resource_description_n_triples_final_state,
    read_resource_description_n_triples_canonical_escape, read_resource_description_n_triples_role,
    read_resource_description_n_triples_scalar, read_resource_description_n_triples_token,
    read_resource_description_n_triples_transition,
};
use crate::resource_description_rules::{
    accept_resource_description_identifier, accept_resource_description_language,
};
use crate::{
    LinkId, LinkNetwork, ResourceDescriptionLinks, decode_resource_description_graph,
    import_resource_description_graph, read_structured_data_lino,
};

pub use provenance::read_resource_description_ntriples_source;

/// An RDF graph and its independently inspectable syntax provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceDescriptionNTriplesLinks {
    /// Semantic and concrete provenance links.
    pub network: LinkNetwork,
    /// The RDF term graph root.
    pub root: LinkId,
    /// The separate syntax root; conversion never replays it.
    pub syntax_root: LinkId,
}

/// Canonical syntax and an authoritative graph of scoped blank correspondences.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceDescriptionNTriplesOutput {
    /// Syntax reconstructed solely from linked RDF terms.
    pub source: String,
    /// Links from canonical blank labels to actual owned scoped blank terms.
    pub correspondence: ResourceDescriptionLinks,
}

/// A syntax diagnostic, distinguishing unsupported forms from malformed input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceDescriptionNTriplesError {
    /// `malformed`, `unsupported`, or `resourceLimit`.
    pub kind: &'static str,
    /// Exact position in UTF-16 source units.
    pub offset: usize,
    /// The diagnostic without a host-specific stack trace.
    pub message: String,
}

impl std::fmt::Display for ResourceDescriptionNTriplesError {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            output,
            "resource description N-Triples: {} at UTF-16 position {}",
            self.message, self.offset
        )
    }
}
impl std::error::Error for ResourceDescriptionNTriplesError {}

type Result<T> = std::result::Result<T, ResourceDescriptionNTriplesError>;
fn error(
    kind: &'static str,
    offset: usize,
    message: impl Into<String>,
) -> ResourceDescriptionNTriplesError {
    ResourceDescriptionNTriplesError {
        kind,
        offset,
        message: message.into(),
    }
}
fn graph_error(message: String) -> ResourceDescriptionNTriplesError {
    error(
        if message.contains("limit") {
            "resourceLimit"
        } else {
            "malformed"
        },
        0,
        message,
    )
}
fn profile() -> &'static Value {
    static PROFILE: OnceLock<Value> = OnceLock::new();
    PROFILE.get_or_init(|| {
        read_structured_data_lino(include_str!(
            "data/resource-description-ntriples-profile.lino"
        ))
        .expect("the packaged N-Triples profile is valid")
    })
}
fn graph_schema() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        read_structured_data_lino(include_str!("data/resource-description-schema.lino"))
            .expect("the packaged RDF graph schema is valid")
    })
}
fn units(source: &str) -> Vec<f64> {
    source.encode_utf16().map(f64::from).collect()
}
// The generated kernel emits bounded nonnegative integral scalar fields.
// This single checked boundary supplies native integer indexing/encoding.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn natural(value: f64) -> usize {
    assert!(
        value.is_finite() && value.fract() == 0.0 && (0.0..=f64::from(u32::MAX)).contains(&value)
    );
    usize::try_from(value as u32).expect("a bounded kernel field fits native indexing")
}
fn number(value: usize) -> f64 {
    f64::from(
        u32::try_from(value).expect("a bounded syntax position fits the portable scalar profile"),
    )
}
fn identity(term: &Value) -> String {
    let parts = match term["kind"].as_str().unwrap() {
        "blank" => json!(["blank", term["sourceIdentifier"], term["value"]]),
        "literal" => json!(["literal", term["value"], term["datatype"], term["language"]]),
        _ => json!(["identifier", term["value"]]),
    };
    serde_json::to_string(&parts).expect("resource term strings serialize")
}
fn identifier(value: &str, offset: usize) -> Result<Value> {
    if value.contains(['[', ']']) {
        return Err(error(
            "unsupported",
            offset,
            "bracketed identifiers require an address grammar outside this profile",
        ));
    }
    if !accept_resource_description_identifier(&units(value)) {
        return Err(error(
            "malformed",
            offset,
            "an absolute identifier is required",
        ));
    }
    Ok(json!({"kind":"identifier","value":value}))
}
fn lexical_error(diagnostic: f64) -> &'static str {
    match natural(diagnostic) {
        1 => "invalid Unicode scalar",
        2 => "empty language tag",
        3 => "invalid blank node label",
        4 => "invalid Unicode scalar escape",
        5 => "unknown character escape",
        6 => "illegal unescaped character",
        7 => "unterminated term",
        _ => "unexpected syntax character",
    }
}
fn slice(input: &[f64], start: usize, end: usize) -> Result<String> {
    String::from_utf16(
        &input[start..end]
            .iter()
            .map(|value| u16::try_from(natural(*value)).unwrap())
            .collect::<Vec<_>>(),
    )
    .map_err(|_| error("malformed", start, "invalid Unicode scalar"))
}
fn token_value(input: &[f64], start: usize, end: usize, kind: usize) -> Result<String> {
    if kind == 2 || kind == 4 {
        return slice(input, start + if kind == 2 { 2 } else { 1 }, end);
    }
    if kind != 1 && kind != 3 {
        return Ok(String::new());
    }
    let mut result = String::new();
    let mut index = start + 1;
    while index < end - 1 {
        let scalar = read_resource_description_n_triples_scalar(input, number(index), number(kind));
        if scalar[2] != 0.0 || scalar[0] < 1.0 {
            return Err(error("malformed", index, lexical_error(scalar[2])));
        }
        result.push(
            char::from_u32(u32::try_from(natural(scalar[1])).unwrap())
                .ok_or_else(|| error("malformed", index, "invalid Unicode scalar"))?,
        );
        index += natural(scalar[0]);
    }
    Ok(result)
}
struct Token {
    kind: usize,
    start: usize,
    end: usize,
    raw: String,
}
struct Statement {
    tokens: Vec<usize>,
    qualifiers: Vec<usize>,
    period: usize,
}
struct Parsed {
    triples: Vec<Value>,
    tokens: Vec<Token>,
    statements: Vec<Statement>,
}
fn parse(source: &str, scope: &str, mapping: &BTreeMap<String, Value>) -> Result<Parsed> {
    let input = units(source);
    if input.len() > 262_144 {
        return Err(error(
            "resourceLimit",
            0,
            "a source of at most 262144 UTF-16 units is required",
        ));
    }
    let mut parsed = Parsed {
        triples: Vec::new(),
        tokens: Vec::new(),
        statements: Vec::new(),
    };
    let mut state = 0;
    let mut start = 0;
    let mut triple = Vec::new();
    let mut token_ids = Vec::new();
    let mut qualifiers = Vec::new();
    while start < input.len() {
        if parsed.tokens.len() >= 10_000 {
            return Err(error("resourceLimit", start, "the token limit is exceeded"));
        }
        let token = read_resource_description_n_triples_token(&input, number(start));
        let kind = natural(token[0]);
        let end = natural(token[1]);
        if kind == 0 {
            return Err(error("malformed", end, lexical_error(token[2])));
        }
        let next = read_resource_description_n_triples_transition(number(state), number(kind));
        if next < 0.0 {
            return Err(error(
                "malformed",
                start,
                "a token cannot occupy this statement position",
            ));
        }
        let value = token_value(&input, start, end, kind)?;
        if value.encode_utf16().count() > 65_536 {
            return Err(error(
                "resourceLimit",
                start,
                "a term exceeds 65536 UTF-16 units",
            ));
        }
        let token_index = parsed.tokens.len();
        parsed.tokens.push(Token {
            kind,
            start,
            end,
            raw: slice(&input, start, end)?,
        });
        let role = natural(read_resource_description_n_triples_role(
            number(state),
            number(kind),
        ));
        match role {
            1..=3 => {
                if role == 1 {
                    triple.clear();
                    token_ids.clear();
                    qualifiers.clear();
                }
                let term = if kind == 1 {
                    identifier(&value, start)?
                } else if kind == 2 {
                    mapping.get(&value).cloned().unwrap_or_else(
                        || json!({"kind":"blank","value":value,"sourceIdentifier":scope}),
                    )
                } else {
                    json!({"kind":"literal","value":value})
                };
                triple.push(term);
                token_ids.push(token_index);
            }
            4 => {
                let lower = value.to_lowercase();
                let grandfathered = graph_schema()["grandfatheredLanguages"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|tag| tag == &lower);
                if !accept_resource_description_language(&units(&value), grandfathered) {
                    return Err(error("malformed", start, "invalid language tag"));
                }
                triple[2]["language"] = Value::String(value);
                qualifiers.push(token_index);
            }
            5 => {
                triple[2]["datatype"] = identifier(&value, start)?["value"].clone();
                qualifiers.push(token_index);
            }
            6 => {
                if triple[2]["datatype"] == graph_schema()["languageDatatype"]
                    && triple[2].get("language").is_none()
                {
                    return Err(error(
                        "malformed",
                        start,
                        "a language string requires a language tag",
                    ));
                }
                parsed.triples.push(Value::Array(triple.clone()));
                parsed.statements.push(Statement {
                    tokens: token_ids.clone(),
                    qualifiers: qualifiers.clone(),
                    period: token_index,
                });
                triple.clear();
            }
            _ => {
                if kind == 6 {
                    qualifiers.push(token_index);
                }
            }
        }
        state = natural(next);
        start = end;
    }
    if !accept_resource_description_n_triples_final_state(number(state)) {
        return Err(error(
            "malformed",
            input.len(),
            "an incomplete statement remains at the end of input",
        ));
    }
    Ok(parsed)
}

/// Import selected N-Triples syntax and retain its separate lossless provenance.
///
/// # Errors
/// Rejects malformed syntax, unsupported identifier forms and bounded resource excess.
pub fn import_resource_description_ntriples(
    source: &str,
    source_identifier: &str,
    correspondence: Option<&ResourceDescriptionLinks>,
) -> Result<ResourceDescriptionNTriplesLinks> {
    let mapping = provenance::correspondence(correspondence)?;
    let parsed = parse(source, source_identifier, &mapping)?;
    let graph =
        import_resource_description_graph(&Value::Array(parsed.triples.clone()), source_identifier)
            .map_err(graph_error)?;
    provenance::attach(graph, source_identifier, &parsed)
}
fn literal(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        let escaped =
            read_resource_description_n_triples_canonical_escape(f64::from(u32::from(character)));
        if escaped < 0.0 {
            output.push(character);
        } else {
            output.push('\\');
            output.push(char::from_u32(u32::try_from(natural(escaped)).unwrap()).unwrap());
        }
    }
    output.push('"');
    output
}
fn canonical_term(term: &Value, labels: &BTreeMap<String, String>) -> String {
    match term["kind"].as_str().unwrap() {
        "identifier" => format!("<{}>", term["value"].as_str().unwrap()),
        "blank" => format!("_:{}", labels[&identity(term)]),
        _ => {
            let mut result = literal(term["value"].as_str().unwrap());
            if !term["language"].as_str().unwrap().is_empty() {
                result.push('@');
                result.push_str(term["language"].as_str().unwrap());
            } else if term["datatype"] != graph_schema()["stringDatatype"] {
                write!(result, "^^<{}>", term["datatype"].as_str().unwrap())
                    .expect("formatting a String succeeds");
            }
            result
        }
    }
}

/// Reconstruct canonical syntax from RDF terms and link its blank identities.
///
/// # Errors
/// Rejects invalid graph semantics or excessive correspondence resources.
pub fn emit_resource_description_ntriples(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<ResourceDescriptionNTriplesOutput> {
    let graph = decode_resource_description_graph(network, root).map_err(graph_error)?;
    let mut blank_keys = graph["triples"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|triple| triple.as_array().unwrap())
        .filter(|term| term["kind"] == "blank")
        .map(identity)
        .collect::<Vec<_>>();
    blank_keys.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
    blank_keys.dedup();
    let labels = blank_keys
        .into_iter()
        .enumerate()
        .map(|(index, key)| (key, format!("b{index}")))
        .collect::<BTreeMap<_, _>>();
    let mut lines = graph["triples"]
        .as_array()
        .unwrap()
        .iter()
        .map(|triple| {
            format!(
                "{} .\n",
                triple
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| canonical_term(value, &labels))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        })
        .collect::<Vec<_>>();
    lines.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
    Ok(ResourceDescriptionNTriplesOutput {
        source: lines.concat(),
        correspondence: provenance::create_correspondence(network, root, &labels)?,
    })
}
