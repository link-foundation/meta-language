//! Host representation of the common concept space. Proof decisions use the
//! verifier translated automatically from the JavaScript implementation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};

use crate::global_concept_identity_rules::{
    accept_global_concept_history, select_global_concept_identity,
};
use crate::global_concept_rules::verify_global_concept_correspondence;

use crate::formal_concept_definition::read_formal_concept_definition;
use crate::grammar::interchange::{deserialize_grammar, serialize_grammar};
use crate::grammar::{Grammar, GrammarFormat, GrammarSourceName};
use crate::link_network::LinkNetwork;
use crate::translation::grammar_selection_rules::read_grammar_source_default_matching;

mod links;
mod parser;
pub use links::{global_concept_space_to_links, read_global_concept_space_links};
pub use parser::{GlobalConceptSpaceParser, create_global_concept_space_parser};

/// The closed recursive proof required before sharing a concept identity.
pub const GLOBAL_CONCEPT_PROOF_METHOD: &str = "recursive-structural-bijection";

/// One imported grammar and its qualified origin.
#[derive(Clone, Debug)]
pub struct GlobalConceptSource {
    /// Stable imported source identity.
    pub id: String,
    /// Language of the source.
    pub language: String,
    /// Edition, or empty text when unspecified.
    pub edition: String,
    /// Complete imported grammar.
    pub grammar: Grammar,
}

impl GlobalConceptSource {
    /// Reads an imported source, retaining concept and source-name metadata.
    ///
    /// # Errors
    /// Returns a validation error for invalid source or grammar data.
    pub fn from_json(value: &Value) -> Result<Self, String> {
        require_global_grammar_source_format(value["grammar"].get("sourceFormat"))?;
        let decoded = deserialize_grammar(&value["grammar"].to_string())
            .map_err(|error| error.to_string())?;
        let mut grammar = Grammar::new();
        if let Some(start) = decoded.start() {
            grammar.set_start(start);
        }
        if let Some(format) = decoded.source_format() {
            grammar.set_source_format(format);
        }
        grammar.set_declarations(decoded.declarations().clone());
        grammar.set_kinds(decoded.kinds().to_vec());
        let metadata = array(&value["grammar"]["rules"])?;
        let rules = decoded.rules().to_vec();
        for mut rule in rules {
            let definition = metadata
                .iter()
                .find(|entry| entry["name"] == rule.name)
                .ok_or_else(|| "missing source rule metadata".to_owned())?;
            rule.concept = definition
                .get("concept")
                .and_then(Value::as_str)
                .map(str::to_owned);
            rule.source_names = definition
                .get("sourceNames")
                .map(array)
                .transpose()?
                .unwrap_or_default()
                .iter()
                .map(|alias| {
                    Ok(GrammarSourceName {
                        source: string(&alias["source"])?.to_owned(),
                        name: string(&alias["name"])?.to_owned(),
                    })
                })
                .collect::<Result<_, String>>()?;
            grammar.add_rule(rule);
        }
        Ok(Self {
            id: string(&value["id"])?.to_owned(),
            language: string(&value["language"])?.to_owned(),
            edition: value
                .get("edition")
                .map(string)
                .transpose()?
                .unwrap_or("")
                .to_owned(),
            grammar,
        })
    }
}

/// Grounded meanings, incremental identities and required sharing obligations.
#[derive(Clone, Debug, Default)]
pub struct GlobalConceptOptions {
    /// Canonical semantic records; absent uses the shipped register.
    pub records: Option<Value>,
    /// Explicit supported operational definitions, by origin alias or record id.
    pub formal_definitions: Option<Value>,
    /// Previous concept space whose surviving identities remain stable.
    pub previous: Option<Value>,
    /// Qualified origin pairs that require a proven shared identity.
    pub equivalences: Vec<[String; 2]>,
}

fn string(value: &Value) -> Result<&str, String> {
    value
        .as_str()
        .ok_or_else(|| "expected text in global concept data".to_owned())
}

fn array(value: &Value) -> Result<&[Value], String> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "expected an array in global concept data".to_owned())
}

fn require_global_grammar_source_format(value: Option<&Value>) -> Result<(), String> {
    value
        .filter(|value| !value.is_null())
        .map(|value| {
            let format = value
                .as_str()
                .ok_or("a global grammar source format must be known text")?;
            GrammarFormat::from_tag(format)
                .ok_or_else(|| format!("unknown grammar source format {format}"))
        })
        .transpose()
        .map(|_| ())
}

fn canonical(value: &Value) -> String {
    match value {
        Value::Array(values) => format!(
            "[{}]",
            values.iter().map(canonical).collect::<Vec<_>>().join(",")
        ),
        Value::Object(values) => {
            let ordered: BTreeMap<_, _> = values.iter().collect();
            format!(
                "{{{}}}",
                ordered
                    .iter()
                    .map(|(key, value)| format!("{}:{}", json!(key), canonical(value)))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
        _ => value.to_string(),
    }
}

fn declarations(value: Option<&Value>) -> Value {
    let mut context = json!({"matching":null,"settling":null,"imports":[],"modes":[],"extras":[],"conflicts":[],"precedences":[],"macros":[],"scanners":[]});
    if let Some(Value::Object(entries)) = value {
        for (key, value) in entries {
            let lexical_context =
                key == "lexers" && value.as_array().is_some_and(|entries| !entries.is_empty());
            if (context.get(key).is_some() || lexical_context) && !value.is_null() {
                context[key] = value.clone();
            }
        }
    }
    context
}

fn production(rule: &Value) -> Value {
    let mut result = json!({"kind":rule["kind"],"expression":rule["expression"]});
    for key in ["parameters", "channel", "modes", "action"] {
        if let Some(value) = rule.get(key) {
            result[key] = value.clone();
        }
    }
    result
}

struct Node {
    source: usize,
    origin: Value,
    definition: Value,
    meaning: String,
}

struct Graph {
    shapes: Vec<String>,
    meanings: Vec<String>,
    offsets: Vec<usize>,
    edges: Vec<usize>,
}

fn shape(
    value: &Value,
    source: &str,
    aliases: &BTreeMap<String, usize>,
    edges: &mut Vec<usize>,
) -> Value {
    match value {
        Value::Array(values) => Value::Array(
            values
                .iter()
                .map(|value| shape(value, source, aliases, edges))
                .collect(),
        ),
        Value::Object(values) => {
            let mut shaped = Map::new();
            let ordered: BTreeMap<_, _> = values.iter().collect();
            for (key, entry) in ordered {
                let value = if value["kind"] == "ref" && key == "name" && entry.is_string() {
                    let target =
                        aliases.get(&format!("{source}:{}", entry.as_str().unwrap_or_default()));
                    target.map_or_else(
                        || json!({"unresolved":source,"name":entry}),
                        |target| {
                            let index = edges.len();
                            edges.push(*target);
                            json!({"reference":index})
                        },
                    )
                } else {
                    shape(entry, source, aliases, edges)
                };
                shaped.insert(key.clone(), value);
            }
            Value::Object(shaped)
        }
        _ => value.clone(),
    }
}

fn correspondence(graph: &Graph, first: usize, second: usize) -> Option<Vec<(usize, usize)>> {
    let mut pending = vec![(first, second)];
    let mut witness = Vec::new();
    let mut left_map = BTreeMap::new();
    let mut right_map = BTreeMap::new();
    let mut at = 0;
    while at < pending.len() {
        let (left, right) = pending[at];
        at += 1;
        if left_map.contains_key(&left) || right_map.contains_key(&right) {
            if left_map.get(&left) != Some(&right) || right_map.get(&right) != Some(&left) {
                return None;
            }
            continue;
        }
        if graph.shapes[left] != graph.shapes[right]
            || graph.meanings[left] != graph.meanings[right]
        {
            return None;
        }
        left_map.insert(left, right);
        right_map.insert(right, left);
        witness.push((left, right));
        let count = graph.offsets[left + 1] - graph.offsets[left];
        if count != graph.offsets[right + 1] - graph.offsets[right] {
            return None;
        }
        for edge in 0..count {
            pending.push((
                graph.edges[graph.offsets[left] + edge],
                graph.edges[graph.offsets[right] + edge],
            ));
        }
    }
    let number = |index: usize| serde_json::Number::from(index).as_f64().unwrap();
    let to_numbers = |values: &[usize]| values.iter().map(|index| number(*index)).collect();
    let verified = verify_global_concept_correspondence(
        graph.shapes.clone(),
        graph.meanings.clone(),
        to_numbers(&graph.offsets),
        to_numbers(&graph.edges),
        witness.iter().map(|(left, _)| number(*left)).collect(),
        witness.iter().map(|(_, right)| number(*right)).collect(),
    );
    verified.then_some(witness)
}

const fn representative(parents: &[usize], mut index: usize) -> usize {
    while parents[index] != index {
        index = parents[index];
    }
    index
}

/// Copies every source rule, then shares identities only after a closed proof.
///
/// # Errors
/// Returns a validation error for malformed or colliding source identities.
pub fn create_global_concept_space(
    sources: &[GlobalConceptSource],
    options: &GlobalConceptOptions,
) -> Result<Value, String> {
    let mut source_values = Vec::new();
    for source in sources {
        let mut grammar: Value = serde_json::from_str(&serialize_grammar(&source.grammar))
            .map_err(|error| error.to_string())?;
        for (definition, rule) in grammar["rules"]
            .as_array_mut()
            .ok_or("missing grammar rules")?
            .iter_mut()
            .zip(source.grammar.rules())
        {
            if let Some(concept) = &rule.concept {
                definition["concept"] = json!(concept);
            }
            definition["sourceNames"] = json!(
                rule.source_names
                    .iter()
                    .map(|name| json!({"source":name.source,"name":name.name}))
                    .collect::<Vec<_>>()
            );
        }
        source_values.push(json!({"id":source.id,"language":source.language,"edition":source.edition,"grammar":grammar}));
    }
    create_global_concept_space_in(&source_values, options).map(|(space, _, _)| space)
}

/// Resolves a construct through its proven identity and preserves alternatives.
/// Language aliases use the same catalog as ordinary public language parsing.
#[must_use]
pub fn translate_global_construct(
    space: &Value,
    alias: &str,
    language: &str,
    edition: Option<&str>,
) -> Value {
    let empty = |relation| json!({"relation":relation,"concept":null,"origins":[],"proofs":[]});
    if space["ambiguities"]
        .as_array()
        .is_some_and(|entries| entries.iter().any(|entry| entry["alias"] == alias))
    {
        return empty("ambiguous");
    }
    let id = &space["mappings"][alias];
    let Some(concept) = space["concepts"]
        .as_array()
        .and_then(|concepts| concepts.iter().find(|concept| concept["id"] == *id))
    else {
        return empty("unknown");
    };
    let canonical_language = |language: &str| {
        crate::language_catalog::language_entry(language)
            .map_or_else(|| language.to_owned(), |entry| entry.name.clone())
    };
    let target = canonical_language(language);
    let origins: Vec<_> = concept["origins"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|origin| {
            origin["language"]
                .as_str()
                .is_some_and(|language| canonical_language(language) == target)
                && edition.is_none_or(|edition| origin["edition"] == edition)
        })
        .cloned()
        .collect();
    let relation = match origins.len() {
        0 => "untranslatable",
        1 => "translated",
        _ => "ambiguous",
    };
    json!({"relation":relation,"concept":id,"origins":origins,"proofs":concept["proofs"]})
}

// JSON/grammar adaptation is host glue. The generated verifier is the sole
// authority that accepts a correspondence and permits shared identities.
fn create_global_concept_space_in(
    sources: &[Value],
    options: &GlobalConceptOptions,
) -> Result<(Value, Graph, BTreeMap<String, usize>), String> {
    let default_records: Value = crate::structured_data_links::read_structured_data_lino(
        include_str!("data/concept-records.lino"),
    )?;
    let records = array(
        options
            .records
            .as_ref()
            .unwrap_or_else(|| &default_records["concepts"]),
    )?;
    let catalog: Value = crate::structured_data_links::read_structured_data_lino(include_str!(
        "data/language-catalog.lino"
    ))?;
    let mut records_by_alias: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    let mut records_by_id = BTreeMap::new();
    for (index, record) in records.iter().enumerate() {
        records_by_id
            .entry(string(&record["id"])?.to_owned())
            .or_insert(index);
        for alias in array(&record["sourceAliases"])? {
            records_by_alias
                .entry((
                    string(&alias["source"])?.to_owned(),
                    string(&alias["name"])?.to_owned(),
                ))
                .or_default()
                .push(index);
        }
    }
    let mut ordered = sources.to_vec();
    ordered.sort_by(|first, second| {
        first["id"]
            .as_str()
            .unwrap_or_default()
            .cmp(second["id"].as_str().unwrap_or_default())
    });
    let mut source_ids = BTreeSet::new();
    let mut aliases = BTreeMap::new();
    let mut nodes = Vec::new();
    let mut ambiguities = Vec::new();
    let mut formal = Map::new();
    let mut contexts = Vec::new();
    for (source_index, source) in ordered.iter().enumerate() {
        let id = string(&source["id"])?;
        let language = string(&source["language"])?;
        let edition = string(&source["edition"])?;
        if id.is_empty() || language.is_empty() {
            return Err(
                "a global concept source needs an identity, language, edition and imported grammar"
                    .to_owned(),
            );
        }
        require_global_grammar_source_format(source["grammar"].get("sourceFormat"))?;
        if !source_ids.insert(id.to_owned()) {
            return Err(format!("duplicate global concept source {id}"));
        }
        let mut rules = array(&source["grammar"]["rules"])?.to_vec();
        rules.sort_by(|first, second| {
            first["name"]
                .as_str()
                .unwrap_or_default()
                .cmp(second["name"].as_str().unwrap_or_default())
        });
        contexts.push(declarations(source["grammar"].get("declarations")));
        let mut native_sources = BTreeSet::from([format!("native:{language}")]);
        if let Some(entry) = crate::language_catalog::language_entry(language) {
            for grammar in &entry.grammars {
                if let Some(file) = catalog["nativeGrammars"][&grammar.id]["file"].as_str() {
                    let basename = file.rsplit('/').next().unwrap_or(file);
                    native_sources.insert(format!(
                        "native:{}",
                        basename.strip_suffix(".lino").unwrap_or(basename)
                    ));
                }
            }
        }
        for rule in rules {
            let name = string(&rule["name"])?;
            let alias = format!("{id}:{name}");
            if aliases.contains_key(&alias) {
                return Err(format!("colliding global concept origin {alias}"));
            }
            let mut source_names = rule
                .get("sourceNames")
                .map(array)
                .transpose()?
                .unwrap_or_default()
                .to_vec();
            source_names.sort_by(|first, second| {
                (
                    first["source"].as_str().unwrap_or_default(),
                    first["name"].as_str().unwrap_or_default(),
                )
                    .cmp(&(
                        second["source"].as_str().unwrap_or_default(),
                        second["name"].as_str().unwrap_or_default(),
                    ))
            });
            let origin = json!({"alias":alias,"source":id,"language":language,"edition":edition,"rule":name,"concept":rule.get("concept").unwrap_or(&Value::Null),"sourceNames":source_names});
            let mut spellings = vec![json!({"source":id,"name":name})];
            spellings.extend(
                native_sources
                    .iter()
                    .map(|source| json!({"source":source,"name":name})),
            );
            spellings.extend(source_names);
            let mut matching_records = BTreeSet::new();
            for spelling in spellings {
                let key = (
                    string(&spelling["source"])?.to_owned(),
                    string(&spelling["name"])?.to_owned(),
                );
                if let Some(indices) = records_by_alias.get(&key) {
                    matching_records.extend(indices.iter().copied());
                }
            }
            let mut meanings: Vec<_> = matching_records
                .iter()
                .map(|index| records[*index]["id"].clone())
                .collect();
            if let Some(concept) = rule.get("concept")
                && !meanings.contains(concept)
            {
                meanings.push(concept.clone());
            }
            meanings.sort_by(|first, second| {
                first
                    .as_str()
                    .unwrap_or_default()
                    .cmp(second.as_str().unwrap_or_default())
            });
            if meanings.len() > 1 {
                ambiguities.push(json!({"alias":alias,"meanings":meanings}));
            }
            let record = if meanings.len() == 1 {
                records_by_id
                    .get(string(&meanings[0])?)
                    .map(|index| &records[*index])
            } else {
                None
            };
            // Catalog identifiers and prose nominate candidates, never proofs.
            // Only a supported, checked operational graph grounds a meaning.
            let declared = options
                .formal_definitions
                .as_ref()
                .and_then(|definitions| definitions.get(&alias))
                .filter(|definition| !definition.is_null())
                .or_else(|| {
                    record.and_then(|record| {
                        options
                            .formal_definitions
                            .as_ref()
                            .and_then(|definitions| {
                                record["id"].as_str().and_then(|id| definitions.get(id))
                            })
                            .filter(|definition| !definition.is_null())
                            .or_else(|| record.get("formalDefinition"))
                    })
                });
            let mut meaning = format!("unproved:{}", canonical(&origin));
            if meanings.len() <= 1
                && let Some(declared) = declared
                && let Ok(prepared) = read_formal_concept_definition(declared)
                && let Some(links) = declared["links"].as_str()
                && let Ok(network) = LinkNetwork::from_lino(links)
                && let Some(root) = declared["root"].as_u64()
            {
                meaning = canonical(&prepared);
                formal.insert(
                    alias.clone(),
                    json!({"links":network.to_lino(),"root":root}),
                );
            }
            aliases.insert(alias, nodes.len());
            nodes.push(Node {
                source: source_index,
                origin,
                definition: production(&rule),
                meaning,
            });
        }
    }
    let mut graph = Graph {
        shapes: Vec::new(),
        meanings: Vec::new(),
        offsets: vec![0],
        edges: Vec::new(),
    };
    for node in &nodes {
        let source = string(&ordered[node.source]["id"])?;
        let mut context = contexts[node.source].clone();
        if context["matching"].is_null() {
            context["matching"] = json!(read_grammar_source_default_matching(
                ordered[node.source]["grammar"]["sourceFormat"]
                    .as_str()
                    .unwrap_or_default()
            ));
        }
        if context["settling"].is_null() {
            context["settling"] = match context["matching"].as_str() {
                Some("peg") => json!(["first"]),
                Some("longest") => json!(["tokens", "precedence", "dynamic", "ambiguity"]),
                _ => json!(["dynamic", "ambiguity"]),
            };
        }
        let mut targets = Vec::new();
        let definition = shape(&node.definition, source, &aliases, &mut targets);
        let context = shape(&context, source, &aliases, &mut targets);
        graph.shapes.push(canonical(
            &json!({"definition":definition,"context":context}),
        ));
        graph.meanings.push(node.meaning.clone());
        graph.edges.extend(targets);
        graph.offsets.push(graph.edges.len());
    }
    let mut parents: Vec<_> = (0..nodes.len()).collect();
    let mut buckets: Vec<Vec<usize>> = Vec::new();
    let mut bucket_keys = BTreeMap::new();
    for index in 0..nodes.len() {
        let key = canonical(&json!([graph.shapes[index], graph.meanings[index]]));
        let next = buckets.len();
        let bucket = *bucket_keys.entry(key).or_insert(next);
        if bucket == buckets.len() {
            buckets.push(Vec::new());
        }
        buckets[bucket].push(index);
    }
    let mut proofs = Vec::new();
    for bucket in buckets {
        for (at, first) in bucket.iter().enumerate() {
            for second in bucket.iter().skip(at + 1) {
                if representative(&parents, *first) == representative(&parents, *second) {
                    continue;
                }
                if let Some(witness) = correspondence(&graph, *first, *second) {
                    proofs.push(json!({"method":GLOBAL_CONCEPT_PROOF_METHOD,"first":nodes[*first].origin["alias"],"second":nodes[*second].origin["alias"],"correspondence":witness.iter().map(|(left,right)| json!([nodes[*left].origin["alias"],nodes[*right].origin["alias"]])).collect::<Vec<_>>()}));
                    for (left, right) in witness {
                        let one = representative(&parents, left);
                        let other = representative(&parents, right);
                        parents[one.max(other)] = one.min(other);
                    }
                }
            }
        }
    }
    let mut classes: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for index in 0..nodes.len() {
        classes
            .entry(representative(&parents, index))
            .or_default()
            .push(index);
    }
    let previous = options
        .previous
        .as_ref()
        .and_then(|previous| previous.get("concepts"))
        .filter(|concepts| !concepts.is_null())
        .map(array)
        .transpose()
        .map_err(|_| "invalid global concept history".to_owned())?
        .unwrap_or_default();
    let reserved = previous
        .iter()
        .map(|concept| string(&concept["id"]).map(str::to_owned))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "invalid global concept history".to_owned())?;
    let mut previous_aliases = Vec::new();
    for concept in previous {
        for origin in
            array(&concept["origins"]).map_err(|_| "invalid global concept history".to_owned())?
        {
            previous_aliases.push(
                string(&origin["alias"])
                    .map_err(|_| "invalid global concept history".to_owned())?
                    .to_owned(),
            );
        }
    }
    let unique_identities = reserved.iter().collect::<BTreeSet<_>>().len();
    let unique_aliases = previous_aliases.iter().collect::<BTreeSet<_>>().len();
    if !accept_global_concept_history(
        &reserved,
        &previous_aliases,
        serde_json::Number::from(unique_identities)
            .as_f64()
            .unwrap(),
        serde_json::Number::from(unique_aliases).as_f64().unwrap(),
    ) {
        return Err("invalid global concept history".to_owned());
    }
    let mut assigned = Vec::new();
    let mut concepts = Vec::new();
    let mut mappings = Map::new();
    let mut redirects = Map::new();
    for indices in classes.values() {
        let origins: Vec<_> = indices
            .iter()
            .map(|index| nodes[*index].origin.clone())
            .collect();
        let present: BTreeSet<_> = origins
            .iter()
            .map(|origin| string(&origin["alias"]))
            .collect::<Result<_, _>>()?;
        let mut established = Vec::new();
        for concept in previous {
            let mut surviving: Vec<_> = array(&concept["origins"])?
                .iter()
                .map(|origin| string(&origin["alias"]))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .filter(|alias| aliases.contains_key(*alias))
                .collect();
            surviving.sort_unstable();
            if surviving
                .first()
                .is_some_and(|alias| present.contains(alias))
            {
                established.push(string(&concept["id"])?.to_owned());
            }
        }
        established.sort_unstable();
        let fallback = {
            let digest = Sha256::digest(canonical(&origins[0]).as_bytes())
                .iter()
                .fold(String::new(), |mut text, byte| {
                    let _ = write!(text, "{byte:02x}");
                    text
                });
            format!("global-concept.{digest}")
        };
        let id = select_global_concept_identity(
            established.first().cloned().unwrap_or_default(),
            fallback,
            &reserved,
            &assigned,
        );
        if id.is_empty() {
            return Err("duplicate global concept identity".to_owned());
        }
        assigned.push(id.clone());
        for old in established.iter().skip(1) {
            redirects.insert(old.clone(), json!(id));
        }
        let proof_indices: Vec<_> = proofs
            .iter()
            .enumerate()
            .filter_map(|(index, proof)| {
                proof["correspondence"]
                    .as_array()
                    .is_some_and(|pairs| {
                        pairs.iter().any(|pair| {
                            present.contains(pair[0].as_str().unwrap_or_default())
                                && present.contains(pair[1].as_str().unwrap_or_default())
                        })
                    })
                    .then_some(index)
            })
            .collect();
        let definitions: Vec<_> = indices.iter().map(|index| json!({"origin":nodes[*index].origin["alias"],"definition":nodes[*index].definition})).collect();
        concepts.push(json!({"id":id,"origins":origins,"definition":definitions[0]["definition"],"definitions":definitions,"proofs":proof_indices}));
        for origin in &origins {
            mappings.insert(string(&origin["alias"])?.to_owned(), json!(id));
        }
    }
    let mut failures = Vec::new();
    for members in &options.equivalences {
        let reason = if members.iter().any(|alias| !aliases.contains_key(alias)) {
            Some("unknown-origin")
        } else if mappings.get(&members[0]) != mappings.get(&members[1]) {
            Some("no-one-to-one-proof")
        } else {
            None
        };
        if let Some(reason) = reason {
            failures.push(json!({"members":members,"reason":reason}));
        }
    }
    let sources: Vec<_> = ordered.iter().enumerate().map(|(index, source)| {
        let mut origin = json!({"id":source["id"],"language":source["language"],"edition":source["edition"],"start":source["grammar"].get("start").unwrap_or(&Value::Null),"ruleOrder":source["grammar"]["rules"].as_array().map(|rules| rules.iter().map(|rule| rule["name"].clone()).collect::<Vec<_>>()).unwrap_or_default(),"context":contexts[index]});
        if let Some(format) = source["grammar"].get("sourceFormat").filter(|format| !format.is_null()) {
            origin["sourceFormat"] = format.clone();
        }
        origin
    }).collect();
    Ok((
        json!({"schemaVersion":1,"sources":sources,"concepts":concepts,"mappings":mappings,"proofs":proofs,"failures":failures,"ambiguities":ambiguities,"redirects":redirects,"formalDefinitions":formal}),
        graph,
        aliases,
    ))
}
