//! Host grammar and Links storage around the automatically translated learner.
use std::collections::BTreeMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::grammar::{
    Grammar, GrammarDeclarations, GrammarExpr, GrammarFormat, GrammarRule, serialize_grammar,
};
use crate::grammar_inference_rules as rules;
use crate::{
    GlobalConceptSource, LinkId, LinkNetwork, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits,
    TypedProgramLinks, create_discovered_concept_links, create_meta_language_archive,
    decode_structured_data_links, decode_typed_program_links, encode_structured_data_links,
    read_discovered_concept_links, read_meta_language_archive,
};

const ALGORITHM: &str = "labelled regular grammar discovery 1";

/// Explicit deterministic learner and resource parameters, shared with JavaScript.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct RegularGrammarInferenceParameters {
    /// `rpni`, `edsm` or the versioned ALERGIA-style strategy.
    pub strategy: String,
    /// Compatibility confidence strictly between zero and one.
    pub alpha: f64,
    /// Maximum prefix tree states, up to 512.
    pub state_limit: usize,
    /// Maximum pairs in one recursive compatibility or fold worklist.
    pub pair_limit: usize,
    /// Maximum attempted state merges.
    pub attempt_limit: usize,
    /// Maximum examples and consumed observation symbols.
    pub symbol_limit: usize,
    /// Maximum repeated observation text bytes.
    pub byte_limit: usize,
}

impl Default for RegularGrammarInferenceParameters {
    fn default() -> Self {
        Self {
            strategy: "rpni".to_owned(),
            alpha: 0.05,
            state_limit: 128,
            pair_limit: 8192,
            attempt_limit: 4096,
            symbol_limit: 2048,
            byte_limit: 4 * 1024 * 1024,
        }
    }
}

/// Source identity retained independently of inferred concept identity.
#[derive(Clone, Debug)]
pub struct RegularGrammarInferenceOptions {
    /// Learner selection and hard resource limits.
    pub parameters: RegularGrammarInferenceParameters,
    /// Qualified source identity.
    pub identifier: String,
    /// Caller-supplied language name.
    pub language: String,
    /// Caller-supplied source edition.
    pub edition: String,
}

impl Default for RegularGrammarInferenceOptions {
    fn default() -> Self {
        Self {
            parameters: RegularGrammarInferenceParameters::default(),
            identifier: "inferred-language".to_owned(),
            language: "Inferred language".to_owned(),
            edition: "regular discovery 1".to_owned(),
        }
    }
}

/// Complete candidate grammar and an archive of its actual declared premises.
#[derive(Clone, Debug)]
pub struct RegularGrammarInference {
    /// Executable right-linear grammar; concrete syntax is the normal parser output.
    pub grammar: Grammar,
    /// Source view that can extend the common dynamic language session.
    pub source: GlobalConceptSource,
    /// Ordered first-observed symbols; symbols have unique text segmentation.
    pub alphabet: Vec<String>,
    /// Numeric native transport of the linked model, including recursive folds.
    pub table: Vec<f64>,
    /// Full model, typed algorithm and recursive concept archive.
    pub network: LinkNetwork,
    /// Actual archive root.
    pub root: LinkId,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Model {
    schema_version: u32,
    algorithm: String,
    identifier: String,
    language: String,
    edition: String,
    options: RegularGrammarInferenceParameters,
    positives: Vec<Vec<String>>,
    negatives: Vec<Vec<String>>,
    alphabet: Vec<String>,
    table: Vec<f64>,
}

fn fail(message: &str) -> String {
    format!("grammar inference: {message}")
}

fn number(value: usize) -> f64 {
    // Every caller has already bounded these resource counts by 16 MiB.
    #[allow(clippy::cast_precision_loss)]
    {
        value as f64
    }
}

fn index(value: f64) -> usize {
    // Only cells accepted by the shared integer/offset shape validator arrive.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    {
        value as usize
    }
}

fn strategy(options: &RegularGrammarInferenceParameters) -> Result<f64, String> {
    let selected = match options.strategy.as_str() {
        "rpni" => 0.0,
        "edsm" => 1.0,
        "alergia" => 2.0,
        _ => return Err(fail("invalid strategy or confidence")),
    };
    if !options.alpha.is_finite() || options.alpha <= 0.0 || options.alpha >= 1.0 {
        return Err(fail("invalid strategy or confidence"));
    }
    for (name, value, maximum) in [
        ("stateLimit", options.state_limit, 512),
        ("pairLimit", options.pair_limit, 32768),
        ("attemptLimit", options.attempt_limit, 32768),
        ("symbolLimit", options.symbol_limit, 16384),
        ("byteLimit", options.byte_limit, 16 * 1024 * 1024),
    ] {
        if value == 0 || value > maximum {
            return Err(fail(&format!("invalid {name}")));
        }
    }
    Ok(selected)
}

fn samples(
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    options: &RegularGrammarInferenceParameters,
) -> Result<(Vec<String>, Vec<f64>), String> {
    if positives.len().saturating_add(negatives.len()) > options.symbol_limit {
        return Err(fail("expected bounded labelled example lists"));
    }
    let mut alphabet: Vec<String> = Vec::new();
    let mut indexes = BTreeMap::new();
    let mut encoded = Vec::new();
    let (mut count, mut bytes) = (0usize, 0usize);
    for (list, label) in [(positives, 1.0), (negatives, 0.0)] {
        for example in list {
            if example.len() > options.symbol_limit - count {
                return Err(fail("sample symbol limit exceeded"));
            }
            count += example.len();
            encoded.extend([label, number(example.len())]);
            for symbol in example {
                if symbol.is_empty() {
                    return Err(fail("symbols must be nonempty Unicode text"));
                }
                bytes = bytes.saturating_add(symbol.len());
                if bytes > options.byte_limit {
                    return Err(fail("sample byte limit exceeded"));
                }
                let next = alphabet.len();
                let at = *indexes.entry(symbol.clone()).or_insert_with(|| {
                    alphabet.push(symbol.clone());
                    next
                });
                encoded.push(number(at));
            }
        }
    }
    for (left, first) in alphabet.iter().enumerate() {
        for (right, second) in alphabet.iter().enumerate() {
            if left != right && first.starts_with(second) {
                return Err(fail(
                    "overlapping symbol prefixes require a separate lexer definition",
                ));
            }
        }
    }
    Ok((alphabet, encoded))
}

fn table(
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    options: &RegularGrammarInferenceParameters,
) -> Result<(Vec<String>, Vec<f64>), String> {
    let selected = strategy(options)?;
    let (alphabet, input) = samples(positives, negatives, options)?;
    let table = rules::infer_regular_grammar_table(
        &input,
        number(alphabet.len()),
        selected,
        options.alpha,
        number(options.state_limit),
        number(options.pair_limit),
        number(options.attempt_limit),
    );
    if table.first().is_some_and(|code| *code < 0.0) {
        return Err(fail(&rules::read_inference_failure_reason(table[0])));
    }
    if !rules::accept_inference_table_shape(&table, number(alphabet.len())) {
        return Err(fail("invalid model table"));
    }
    Ok((alphabet, table))
}

fn grammar_from_table(table: &[f64], alphabet: &[String]) -> Grammar {
    let states = (0..table.len())
        .step_by(8)
        .filter(|at| {
            table[*at] == 0.0
                && index(rules::find_inference_representative(table, number(*at))) == *at
        })
        .collect::<Vec<_>>();
    let names = states
        .iter()
        .enumerate()
        .map(|(ordinal, state)| (*state, format!("inferred_state_{ordinal}")))
        .collect::<BTreeMap<_, _>>();
    let mut grammar = Grammar::new().with_source_format(GrammarFormat::Inferred);
    grammar.set_declarations(GrammarDeclarations {
        matching: Some("generalized".to_owned()),
        ..Default::default()
    });
    for state in states {
        let mut alternatives = Vec::new();
        if table[state + 2] == 1.0 && table[state + 3] == 0.0 {
            alternatives.push(GrammarExpr::Empty);
        }
        for edge in (0..table.len()).step_by(8) {
            if table[edge] == 1.0 && index(table[edge + 1]) == state {
                let target = index(rules::find_inference_representative(table, table[edge + 3]));
                alternatives.push(GrammarExpr::Sequence(vec![
                    GrammarExpr::Terminal(alphabet[index(table[edge + 2])].clone()),
                    GrammarExpr::NonTerminal(names[&target].clone()),
                ]));
            }
        }
        let expression = if alternatives.len() == 1 {
            alternatives.remove(0)
        } else {
            GrammarExpr::Choice {
                ordered: false,
                alternatives,
            }
        };
        grammar.add_rule(GrammarRule::new(names[&state].clone(), expression));
    }
    grammar.set_start(names[&index(rules::find_inference_representative(table, 0.0))].clone());
    grammar
}

/// Load the complete typed algorithm definition, without asserting execution.
///
/// # Errors
/// Refuses malformed packaged source graphs or failed static typing.
pub fn create_regular_grammar_inference_program_links() -> Result<TypedProgramLinks, String> {
    let network = LinkNetwork::from_lino(include_str!("data/grammar-inference-program.lino"))
        .map_err(|error| error.to_string())?;
    let roots = network
        .links()
        .filter(|link| {
            link.metadata().language() == Some("meta-language typed program")
                && link.metadata().term() == Some("typed program")
                && link.metadata().definition() == Some("1")
        })
        .map(crate::Link::id)
        .collect::<Vec<_>>();
    if roots.len() != 1 {
        return Err(fail("the algorithm needs one actual typed program root"));
    }
    let root = roots[0];
    decode_typed_program_links(&network, root).map_err(|error| error.to_string())?;
    Ok(TypedProgramLinks { network, root })
}

/// Discover and archive a complete candidate from declared symbol observations.
///
/// # Errors
/// Refuses malformed samples, contradictory labels, resource exhaustion and
/// symbol vocabularies requiring a separately represented lexer.
pub fn infer_regular_grammar(
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    options: &RegularGrammarInferenceOptions,
) -> Result<RegularGrammarInference, String> {
    for value in [&options.identifier, &options.language, &options.edition] {
        if value.is_empty() || value.len() > 4096 {
            return Err(fail("invalid source identity"));
        }
    }
    let (alphabet, table) = table(positives, negatives, &options.parameters)?;
    let grammar = grammar_from_table(&table, &alphabet);
    let source = GlobalConceptSource {
        id: options.identifier.clone(),
        language: options.language.clone(),
        edition: options.edition.clone(),
        grammar: grammar.clone(),
    };
    let model = Model {
        schema_version: 1,
        algorithm: ALGORITHM.to_owned(),
        identifier: source.id.clone(),
        language: source.language.clone(),
        edition: source.edition.clone(),
        options: options.parameters.clone(),
        positives: positives.to_vec(),
        negatives: negatives.to_vec(),
        alphabet: alphabet.clone(),
        table: table.clone(),
    };
    let data = encode_structured_data_links(
        &serde_json::to_value(model).map_err(|error| error.to_string())?,
    )?;
    let definitions = create_regular_grammar_inference_program_links()?;
    let discovered =
        create_discovered_concept_links(std::slice::from_ref(&source), &Default::default())?;
    let archive = create_meta_language_archive(
        &[
            MetaLanguageArchiveEntry {
                identifier: "regular-discovery-model".to_owned(),
                kind: "grammar inference".to_owned(),
                network: Arc::new(data.network),
                root: data.root,
            },
            MetaLanguageArchiveEntry {
                identifier: "regular-discovery-algorithm".to_owned(),
                kind: "typed program".to_owned(),
                network: Arc::new(definitions.network),
                root: definitions.root,
            },
            MetaLanguageArchiveEntry {
                identifier: "regular-discovery-grammar".to_owned(),
                kind: "discovered concepts".to_owned(),
                network: Arc::new(discovered.network),
                root: discovered.root,
            },
        ],
        MetaLanguageArchiveLimits::default(),
    )?;
    Ok(RegularGrammarInference {
        grammar,
        source,
        alphabet,
        table,
        network: archive.network,
        root: archive.root,
    })
}

/// Reload and recompute this version's decisions from the actual stored model.
///
/// # Errors
/// Refuses altered algorithm definitions, model decisions or candidate grammars.
pub fn read_regular_grammar_inference_links(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<RegularGrammarInference, String> {
    let decoded = read_meta_language_archive(network, root, MetaLanguageArchiveLimits::default())?;
    let select = |identifier: &str, kind: &str| {
        decoded
            .entries
            .iter()
            .find(|entry| entry.identifier == identifier && entry.kind == kind)
            .ok_or_else(|| fail(&format!("missing {identifier}")))
    };
    let data = select("regular-discovery-model", "grammar inference")?;
    let model: Model =
        serde_json::from_value(decode_structured_data_links(&data.network, data.root)?)
            .map_err(|error| fail(&error.to_string()))?;
    if model.schema_version != 1 || model.algorithm != ALGORITHM {
        return Err(fail("unsupported model version"));
    }
    let definitions = select("regular-discovery-algorithm", "typed program")?;
    let bundled = create_regular_grammar_inference_program_links()?;
    if definitions.root != bundled.root
        || definitions.network.to_lino() != bundled.network.to_lino()
    {
        return Err(fail(
            "the selected algorithm definition differs from its versioned implementation",
        ));
    }
    let (alphabet, table) = table(&model.positives, &model.negatives, &model.options)?;
    if alphabet != model.alphabet || table != model.table {
        return Err(fail(
            "model decisions do not follow the stored observations and algorithm",
        ));
    }
    let candidate = select("regular-discovery-grammar", "discovered concepts")?;
    let discovered = read_discovered_concept_links(&candidate.network, candidate.root)?;
    let source = discovered
        .sources
        .first()
        .ok_or_else(|| fail("candidate grammar differs from its actual model"))?;
    let grammar = grammar_from_table(&table, &alphabet);
    if discovered.sources.len() != 1
        || source.id != model.identifier
        || source.language != model.language
        || source.edition != model.edition
        || serialize_grammar(&source.grammar) != serialize_grammar(&grammar)
    {
        return Err(fail("candidate grammar differs from its actual model"));
    }
    Ok(RegularGrammarInference {
        grammar: source.grammar.clone(),
        source: source.clone(),
        alphabet,
        table,
        network: network.clone(),
        root,
    })
}

/// Run the actual inferred automaton over a new symbol sequence.
///
/// # Errors
/// Refuses malformed models and oversized input. Unknown symbols reject.
pub fn accept_inferred_grammar_symbols(
    model: &RegularGrammarInference,
    symbols: &[String],
) -> Result<bool, String> {
    if symbols.len() > 16384 {
        return Err(fail("input symbol limit exceeded"));
    }
    if model.alphabet.len() > 16384
        || model.alphabet.iter().any(String::is_empty)
        || model
            .alphabet
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != model.alphabet.len()
        || !rules::accept_inference_table_shape(&model.table, number(model.alphabet.len()))
    {
        return Err(fail("invalid model table"));
    }
    let mut input = Vec::new();
    for symbol in symbols {
        let Some(index) = model.alphabet.iter().position(|value| value == symbol) else {
            return Ok(false);
        };
        input.push(number(index));
    }
    Ok(rules::accept_inferred_symbol_sequence(&model.table, &input))
}
