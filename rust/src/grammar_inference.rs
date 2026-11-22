//! Host grammar and Links storage around the automatically translated learner.
use std::collections::BTreeMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::grammar::{
    Grammar, GrammarDeclarations, GrammarExpr, GrammarFormat, GrammarRule, serialize_grammar,
};
use crate::grammar_discovery_parser_oracle::{
    GRAMMAR_DISCOVERY_PARSER_ORACLE, create_oracle_entry, read_oracle_entry,
};
use crate::grammar_inference_rules as rules;
use crate::grammar_scanner_inference::{
    GrammarScannerModeStackObservation, GrammarScannerObservation, InferredGrammarScanner,
    SCANNER_ALGORITHM, SCANNER_CLASS_ALGORITHM, SCANNER_MODE_STACK_ALGORITHM,
    create_grammar_scanner_character_class_program_links,
    create_grammar_scanner_inference_program_links,
    create_grammar_scanner_mode_stack_program_links, infer_grammar_scanner,
    infer_grammar_scanner_classes, infer_grammar_scanner_mode_stack,
};
use crate::{
    GlobalConceptSource, GrammarDiscoveryOracleObservation, LinkId, LinkNetwork,
    MetaLanguageArchive, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits, TypedProgramLinks,
    create_discovered_concept_links, create_meta_language_archive, decode_structured_data_links,
    decode_typed_program_links, encode_structured_data_links, extend_meta_language_archive,
    observe_grammar_discovery_parser_oracle, read_discovered_concept_links,
    read_meta_language_archive,
};

const ALGORITHM: &str = "labelled regular grammar discovery 1";
mod refinement;
mod structural;
pub(crate) use refinement::limits as regular_grammar_refinement_archive_limits;
pub use refinement::{
    read_refined_regular_grammar_inference_links, refine_regular_grammar_inference,
};
pub(crate) use structural::STRUCTURAL_GRAMMAR_INFERENCE_MODEL_IDENTIFIER;
pub use structural::{
    StructuralGrammarInference, StructuralGrammarInferenceLimits,
    StructuralGrammarInferenceOptions, infer_structural_grammar,
    infer_structural_grammar_with_parser_oracle, read_structural_grammar_inference_links,
    refine_structural_grammar_inference,
};

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
    /// Optional finite scanner trace. Sample symbols then refer to its external tokens.
    pub scanner_observations: Option<Vec<GrammarScannerObservation>>,
    /// Coalesce observed scalars into finite character classes.
    pub scanner_character_classes: bool,
    /// Optional finite observations of complete mode stacks before and after a token.
    pub scanner_mode_stack_observations: Option<Vec<GrammarScannerModeStackObservation>>,
}

impl Default for RegularGrammarInferenceOptions {
    fn default() -> Self {
        Self {
            parameters: RegularGrammarInferenceParameters::default(),
            identifier: "inferred-language".to_owned(),
            language: "Inferred language".to_owned(),
            edition: "regular discovery 1".to_owned(),
            scanner_observations: None,
            scanner_character_classes: false,
            scanner_mode_stack_observations: None,
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
    /// Ordered first-observed symbols, as literals or declared scanner token names.
    pub alphabet: Vec<String>,
    /// Numeric native transport of the linked model, including recursive folds.
    pub table: Vec<f64>,
    /// Independent native parser observations, when the discovery used an oracle.
    pub oracle_observations: Option<Vec<GrammarDiscoveryOracleObservation>>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scanner_observations: Option<Vec<GrammarScannerObservation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scanner_character_classes: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scanner_mode_stack_observations: Option<Vec<GrammarScannerModeStackObservation>>,
}

fn infer_scanner(
    observations: &[GrammarScannerObservation],
    classes: bool,
) -> Result<InferredGrammarScanner, String> {
    if classes {
        infer_grammar_scanner_classes(observations, None)
    } else {
        infer_grammar_scanner(observations, None)
    }
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
    scanner: Option<&InferredGrammarScanner>,
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
    if let Some(scanner) = scanner {
        if scanner
            .scanner
            .tokens
            .iter()
            .any(|token| token.starts_with("inferred_state_"))
        {
            return Err(fail(
                "scanner token name collides with the inferred rule namespace",
            ));
        }
        if alphabet
            .iter()
            .any(|symbol| !scanner.scanner.tokens.contains(symbol))
        {
            return Err(fail(
                "sample contains a token missing from the scanner observations",
            ));
        }
    } else {
        for (left, first) in alphabet.iter().enumerate() {
            for (right, second) in alphabet.iter().enumerate() {
                if left != right && first.starts_with(second) {
                    return Err(fail(
                        "overlapping symbol prefixes require a separate lexer definition",
                    ));
                }
            }
        }
    }
    Ok((alphabet, encoded))
}

fn table(
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    options: &RegularGrammarInferenceParameters,
    scanner: Option<&InferredGrammarScanner>,
) -> Result<(Vec<String>, Vec<f64>), String> {
    let selected = strategy(options)?;
    let (alphabet, input) = samples(positives, negatives, options, scanner)?;
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

fn grammar_from_table(
    table: &[f64],
    alphabet: &[String],
    scanner: Option<&InferredGrammarScanner>,
) -> Grammar {
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
        modes: scanner.map_or_else(Vec::new, |definition| definition.modes.clone()),
        scanners: scanner.map_or_else(Vec::new, |definition| vec![definition.scanner.clone()]),
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
                let symbol = alphabet[index(table[edge + 2])].clone();
                alternatives.push(GrammarExpr::Sequence(vec![
                    if scanner.is_some() {
                        GrammarExpr::NonTerminal(symbol)
                    } else {
                        GrammarExpr::Terminal(symbol)
                    },
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
    let classes = options.scanner_character_classes;
    let mode_stacks = options.scanner_mode_stack_observations.is_some();
    if mode_stacks && options.scanner_observations.is_some() {
        return Err(fail("select one scanner observation format"));
    }
    if classes && !mode_stacks && options.scanner_observations.is_none() {
        return Err(fail(
            "scanner character classes require scanner observations",
        ));
    }
    let scanner = if let Some(observations) = options.scanner_mode_stack_observations.as_deref() {
        Some(infer_grammar_scanner_mode_stack(
            observations,
            None,
            classes,
        )?)
    } else {
        options
            .scanner_observations
            .as_deref()
            .map(|observations| infer_scanner(observations, classes))
            .transpose()?
    };
    let (alphabet, table) = table(positives, negatives, &options.parameters, scanner.as_ref())?;
    let grammar = grammar_from_table(&table, &alphabet, scanner.as_ref());
    let source = GlobalConceptSource {
        id: options.identifier.clone(),
        language: options.language.clone(),
        edition: options.edition.clone(),
        grammar: grammar.clone(),
    };
    let model = Model {
        schema_version: if mode_stacks {
            4
        } else if classes {
            3
        } else if scanner.is_some() {
            2
        } else {
            1
        },
        algorithm: ALGORITHM.to_owned(),
        identifier: source.id.clone(),
        language: source.language.clone(),
        edition: source.edition.clone(),
        options: options.parameters.clone(),
        positives: positives.to_vec(),
        negatives: negatives.to_vec(),
        alphabet: alphabet.clone(),
        table: table.clone(),
        scanner_observations: options.scanner_observations.clone(),
        scanner_character_classes: classes.then_some(true),
        scanner_mode_stack_observations: options.scanner_mode_stack_observations.clone(),
    };
    let data = encode_structured_data_links(
        &serde_json::to_value(model).map_err(|error| error.to_string())?,
    )?;
    let definitions = create_regular_grammar_inference_program_links()?;
    let discovered =
        create_discovered_concept_links(std::slice::from_ref(&source), &Default::default())?;
    let mut entries = vec![
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
    ];
    if scanner.is_some() {
        let definition = create_grammar_scanner_inference_program_links()?;
        entries.push(MetaLanguageArchiveEntry {
            identifier: SCANNER_ALGORITHM.to_owned(),
            kind: "typed program".to_owned(),
            network: Arc::new(definition.network),
            root: definition.root,
        });
    }
    if classes {
        let definition = create_grammar_scanner_character_class_program_links()?;
        entries.push(MetaLanguageArchiveEntry {
            identifier: SCANNER_CLASS_ALGORITHM.to_owned(),
            kind: "typed program".to_owned(),
            network: Arc::new(definition.network),
            root: definition.root,
        });
    }
    if mode_stacks {
        let definition = create_grammar_scanner_mode_stack_program_links()?;
        entries.push(MetaLanguageArchiveEntry {
            identifier: SCANNER_MODE_STACK_ALGORITHM.to_owned(),
            kind: "typed program".to_owned(),
            network: Arc::new(definition.network),
            root: definition.root,
        });
    }
    let archive = create_meta_language_archive(&entries, MetaLanguageArchiveLimits::default())?;
    Ok(RegularGrammarInference {
        grammar,
        source,
        alphabet,
        table,
        oracle_observations: None,
        network: archive.network,
        root: archive.root,
    })
}

/// Discover a scalar regular grammar using a bounded native parser oracle.
///
/// # Errors
/// Refuses scanner profiles, invalid examples or grammar, contradictions and
/// resource limits encountered by the oracle or the inference algorithm.
pub fn infer_regular_grammar_with_parser_oracle(
    grammar: &Grammar,
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    candidates: &[Vec<String>],
    options: &RegularGrammarInferenceOptions,
) -> Result<RegularGrammarInference, String> {
    if options.scanner_observations.is_some()
        || options.scanner_character_classes
        || options.scanner_mode_stack_observations.is_some()
    {
        return Err(fail(
            "native parser oracle discovery requires a scalar regular language input",
        ));
    }
    let observed =
        observe_grammar_discovery_parser_oracle(grammar, positives, negatives, candidates)?;
    let result = infer_regular_grammar(&observed.positives, &observed.negatives, options)?;
    let entry = create_oracle_entry(
        grammar,
        positives,
        negatives,
        candidates,
        &observed.observations,
    )?;
    let archive = extend_meta_language_archive(
        &MetaLanguageArchive {
            network: result.network,
            root: result.root,
        },
        &[entry],
        false,
        &[],
        MetaLanguageArchiveLimits::default(),
    )?;
    Ok(RegularGrammarInference {
        network: archive.network,
        root: archive.root,
        oracle_observations: Some(observed.observations),
        ..result
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
    let model_data = decode_structured_data_links(&data.network, data.root)?;
    let has_scanner_field = model_data.get("scannerObservations").is_some();
    let has_class_field = model_data.get("scannerCharacterClasses").is_some();
    let has_stack_field = model_data.get("scannerModeStackObservations").is_some();
    let model: Model =
        serde_json::from_value(model_data).map_err(|error| fail(&error.to_string()))?;
    let classes = model.scanner_character_classes == Some(true);
    let mode_stacks = model.schema_version == 4;
    if model.algorithm != ALGORITHM
        || !matches!(model.schema_version, 1..=4)
        || has_scanner_field != matches!(model.schema_version, 2 | 3)
        || has_scanner_field != model.scanner_observations.is_some()
        || has_stack_field != mode_stacks
        || has_stack_field != model.scanner_mode_stack_observations.is_some()
        || has_class_field != classes
        || (model.schema_version == 3 && !classes)
        || (model.schema_version < 3 && classes)
    {
        return Err(fail("unsupported model version"));
    }
    if classes {
        let definition = select(SCANNER_CLASS_ALGORITHM, "typed program")?;
        let bundled = create_grammar_scanner_character_class_program_links()?;
        if definition.root != bundled.root
            || definition.network.to_lino() != bundled.network.to_lino()
        {
            return Err(fail(
                "the selected scanner character class definition differs from its versioned implementation",
            ));
        }
    } else if decoded
        .entries
        .iter()
        .any(|entry| entry.identifier == SCANNER_CLASS_ALGORITHM)
    {
        return Err(fail(
            "the scanner character class definition needs its versioned model",
        ));
    }
    if mode_stacks {
        let definition = select(SCANNER_MODE_STACK_ALGORITHM, "typed program")?;
        let bundled = create_grammar_scanner_mode_stack_program_links()?;
        if definition.root != bundled.root
            || definition.network.to_lino() != bundled.network.to_lino()
        {
            return Err(fail(
                "the selected scanner mode stack definition differs from its versioned implementation",
            ));
        }
    } else if decoded
        .entries
        .iter()
        .any(|entry| entry.identifier == SCANNER_MODE_STACK_ALGORITHM)
    {
        return Err(fail(
            "the scanner mode stack definition needs its versioned model",
        ));
    }
    let scanner = if model.schema_version >= 2 {
        let definition = select(SCANNER_ALGORITHM, "typed program")?;
        let bundled = create_grammar_scanner_inference_program_links()?;
        if definition.root != bundled.root
            || definition.network.to_lino() != bundled.network.to_lino()
        {
            return Err(fail(
                "the selected scanner definition differs from its versioned implementation",
            ));
        }
        if let Some(observations) = model.scanner_mode_stack_observations.as_deref() {
            Some(infer_grammar_scanner_mode_stack(
                observations,
                None,
                classes,
            )?)
        } else {
            Some(infer_scanner(
                model
                    .scanner_observations
                    .as_deref()
                    .ok_or_else(|| fail("unsupported model version"))?,
                classes,
            )?)
        }
    } else {
        if decoded
            .entries
            .iter()
            .any(|entry| entry.identifier == SCANNER_ALGORITHM)
        {
            return Err(fail(
                "the selected scanner definition needs its actual observations",
            ));
        }
        None
    };
    let definitions = select("regular-discovery-algorithm", "typed program")?;
    let bundled = create_regular_grammar_inference_program_links()?;
    if definitions.root != bundled.root
        || definitions.network.to_lino() != bundled.network.to_lino()
    {
        return Err(fail(
            "the selected algorithm definition differs from its versioned implementation",
        ));
    }
    let (alphabet, table) = table(
        &model.positives,
        &model.negatives,
        &model.options,
        scanner.as_ref(),
    )?;
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
    let grammar = grammar_from_table(&table, &alphabet, scanner.as_ref());
    if discovered.sources.len() != 1
        || source.id != model.identifier
        || source.language != model.language
        || source.edition != model.edition
        || serialize_grammar(&source.grammar) != serialize_grammar(&grammar)
    {
        return Err(fail("candidate grammar differs from its actual model"));
    }
    let oracle_observations = decoded
        .entries
        .iter()
        .find(|entry| entry.identifier == GRAMMAR_DISCOVERY_PARSER_ORACLE)
        .map(|entry| read_oracle_entry(entry, &model.positives, &model.negatives, false))
        .transpose()?;
    Ok(RegularGrammarInference {
        grammar: source.grammar.clone(),
        source: source.clone(),
        alphabet,
        table,
        oracle_observations,
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
