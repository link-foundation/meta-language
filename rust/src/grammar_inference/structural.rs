//! Native grammar and archive allocation around the shared structural learner.

use std::collections::{BTreeMap, HashMap};

use super::{index, number};
use crate::grammar::{Grammar, GrammarDeclarations, GrammarExpr, GrammarFormat, GrammarRule};
use crate::grammar_structural_inference_rules::infer_grammar_structural_productions;
use crate::{
    GlobalConceptSource, LinkId, LinkNetwork, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits,
    create_discovered_concept_links, create_meta_language_archive,
    create_source_definition_program_links, decode_structured_data_links,
    encode_structured_data_links, read_meta_language_archive,
};

use serde::{Deserialize, Serialize};

const ALGORITHM: &str = "recursive grammar inference from labelled delimiter productions 1";
const MODEL: &str = "structural-discovery-model";
const GRAMMAR: &str = "structural-discovery-grammar";
const DEFINITION: &str = "structural-discovery-algorithm";
const START: &str = "inferred_structure";

/// Explicit work limits for the delimiter-based structural learner.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StructuralGrammarInferenceLimits {
    /// Maximum number of simultaneously open delimiters.
    pub depth_limit: usize,
    /// Maximum number of distinct observed productions.
    pub production_limit: usize,
    /// Maximum example count and summed token count, each checked separately.
    pub symbol_limit: usize,
    /// Maximum total UTF-8 bytes of the repeated observation tokens.
    pub byte_limit: usize,
}

impl Default for StructuralGrammarInferenceLimits {
    fn default() -> Self {
        Self {
            depth_limit: 32,
            production_limit: 256,
            symbol_limit: 2048,
            byte_limit: 262144,
        }
    }
}

/// Source identity and labelled delimiter inference parameters.
#[derive(Clone, Debug)]
pub struct StructuralGrammarInferenceOptions {
    /// Ordered distinct pairs of observed opening and closing tokens.
    pub delimiters: Vec<[String; 2]>,
    /// Qualified language source identifier.
    pub identifier: String,
    /// Display name of the language.
    pub language: String,
    /// Language source edition.
    pub edition: String,
    /// Bounded work and input profile.
    pub limits: StructuralGrammarInferenceLimits,
}

impl Default for StructuralGrammarInferenceOptions {
    fn default() -> Self {
        Self {
            delimiters: Vec::new(),
            identifier: "inferred-structure".to_owned(),
            language: "Inferred language".to_owned(),
            edition: "structural discovery 1".to_owned(),
            limits: StructuralGrammarInferenceLimits::default(),
        }
    }
}

/// Native recursive grammar and the Links archive of the inference.
#[derive(Clone, Debug)]
pub struct StructuralGrammarInference {
    /// Executable grammar.
    pub grammar: Grammar,
    /// Qualified grammar source for installation and concept discovery.
    pub source: GlobalConceptSource,
    /// Ordered first-seen tokens.
    pub alphabet: Vec<String>,
    /// Ordered rows giving owner, terminals and delimiter-group references.
    pub productions: Vec<Vec<f64>>,
    /// Lossless source, algorithm and concept graph archive.
    pub network: LinkNetwork,
    /// Actual archive entry point.
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
    options: StructuralGrammarInferenceLimits,
    delimiters: Vec<[String; 2]>,
    positives: Vec<Vec<String>>,
    negatives: Vec<Vec<String>>,
    alphabet: Vec<String>,
    productions: Vec<Vec<f64>>,
}

#[derive(Default)]
struct Observations {
    alphabet: Vec<String>,
    positions: HashMap<String, f64>,
    symbols: usize,
    bytes: usize,
}

impl Observations {
    fn encode(
        &mut self,
        examples: &[Vec<String>],
        limits: &StructuralGrammarInferenceLimits,
    ) -> Result<Vec<Vec<f64>>, String> {
        let mut encoded = Vec::with_capacity(examples.len());
        for row in examples {
            if row.len() > limits.symbol_limit - self.symbols {
                return Err(fail("sample symbol limit exceeded"));
            }
            self.symbols += row.len();
            let mut example = Vec::with_capacity(row.len());
            for token in row {
                if !source_text(token) {
                    return Err(fail("tokens must be nonempty Unicode text"));
                }
                self.bytes += token.len();
                if self.bytes > limits.byte_limit {
                    return Err(fail("sample byte limit exceeded"));
                }
                let position = self.positions.entry(token.clone()).or_insert_with(|| {
                    let position = number(self.alphabet.len());
                    self.alphabet.push(token.clone());
                    position
                });
                example.push(*position);
            }
            encoded.push(example);
        }
        Ok(encoded)
    }

    fn groups(&self, pairs: &[[String; 2]]) -> Result<(Vec<f64>, Vec<f64>), String> {
        let mut sorted = self.alphabet.iter().collect::<Vec<_>>();
        sorted.sort_unstable();
        if sorted
            .windows(2)
            .any(|pair| pair[1].starts_with(pair[0].as_str()))
        {
            return Err(fail(
                "overlapping token prefixes require a separately represented lexer",
            ));
        }
        let mut opening = vec![0.0; self.alphabet.len()];
        let mut closing = vec![0.0; self.alphabet.len()];
        let duplicate = || fail("delimiter pairs need distinct observed tokens");
        for (offset, [first, last]) in pairs.iter().enumerate() {
            let begin = index(*self.positions.get(first).ok_or_else(duplicate)?);
            let end = index(*self.positions.get(last).ok_or_else(duplicate)?);
            if begin == end
                || opening[begin] > 0.0
                || closing[begin] > 0.0
                || opening[end] > 0.0
                || closing[end] > 0.0
            {
                return Err(duplicate());
            }
            opening[begin] = number(offset + 1);
            closing[end] = number(offset + 1);
        }
        Ok((opening, closing))
    }
}

fn fail(message: &str) -> String {
    format!("grammar structural inference: {message}")
}

fn source_text(text: &str) -> bool {
    !text.is_empty() && text.len() <= 4096
}

fn group_name(group: usize) -> String {
    format!("inferred_delimiter_group_{group}")
}

fn sequence(mut items: Vec<GrammarExpr>) -> GrammarExpr {
    match items.len() {
        0 => GrammarExpr::Empty,
        1 => items.remove(0),
        _ => GrammarExpr::Sequence(items),
    }
}

fn grammar_from_productions(
    productions: &[Vec<f64>],
    alphabet: &[String],
    delimiters: &[[String; 2]],
) -> Grammar {
    let mut alternatives = BTreeMap::<usize, Vec<GrammarExpr>>::new();
    for row in productions {
        let group = index(row[0]);
        let items = row[1..]
            .iter()
            .map(|symbol| {
                if *symbol < 0.0 {
                    GrammarExpr::NonTerminal(group_name(index(-*symbol)))
                } else {
                    GrammarExpr::Terminal(alphabet[index(*symbol)].clone())
                }
            })
            .collect();
        alternatives.entry(group).or_default().push(sequence(items));
    }
    let mut grammar = Grammar::new().with_source_format(GrammarFormat::Inferred);
    grammar.set_declarations(GrammarDeclarations {
        matching: Some("generalized".to_owned()),
        ..Default::default()
    });
    for (group, mut entries) in alternatives {
        let interior = if entries.len() == 1 {
            entries.remove(0)
        } else {
            GrammarExpr::Choice {
                ordered: false,
                alternatives: entries,
            }
        };
        if group == 0 {
            grammar.add_rule(GrammarRule::new(START.to_owned(), interior));
        } else {
            let [open, close] = &delimiters[group - 1];
            let expression = GrammarExpr::Sequence(vec![
                GrammarExpr::Terminal(open.clone()),
                interior,
                GrammarExpr::Terminal(close.clone()),
            ]);
            grammar.add_rule(GrammarRule::new(group_name(group), expression));
        }
    }
    grammar.set_start(START.to_owned());
    grammar
}

fn entry(
    identifier: &str,
    kind: &str,
    network: LinkNetwork,
    root: LinkId,
) -> MetaLanguageArchiveEntry {
    MetaLanguageArchiveEntry {
        identifier: identifier.to_owned(),
        kind: kind.to_owned(),
        network: network.into(),
        root,
    }
}

/// Infer executable recursive productions and archive the inference premises.
///
/// # Errors
/// Refuses malformed or over-budget observations and groups that accept a labelled negative.
pub fn infer_structural_grammar(
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    options: &StructuralGrammarInferenceOptions,
) -> Result<StructuralGrammarInference, String> {
    if [&options.identifier, &options.language, &options.edition]
        .iter()
        .any(|text| !source_text(text))
    {
        return Err(fail("invalid source identity"));
    }
    let limits = &options.limits;
    if !(1..=64).contains(&limits.depth_limit)
        || !(1..=512).contains(&limits.production_limit)
        || !(1..=4096).contains(&limits.symbol_limit)
        || !(1..=1048576).contains(&limits.byte_limit)
    {
        return Err(fail("invalid resource limit"));
    }
    if positives.is_empty() || positives.len().saturating_add(negatives.len()) > limits.symbol_limit
    {
        return Err(fail(
            "expected bounded labelled examples with at least one positive",
        ));
    }
    if !(1..=32).contains(&options.delimiters.len()) {
        return Err(fail("expected bounded delimiter pairs"));
    }
    let mut observations = Observations::default();
    let accepted = observations.encode(positives, limits)?;
    let rejected = observations.encode(negatives, limits)?;
    let (opening, closing) = observations.groups(&options.delimiters)?;
    let productions = infer_grammar_structural_productions(
        &accepted,
        &rejected,
        &opening,
        &closing,
        number(limits.depth_limit),
        number(limits.production_limit),
    );
    match productions[0][0] {
        -1.0 => return Err(fail("unbalanced delimiters in a positive example")),
        -2.0 => return Err(fail("delimiter nesting limit exceeded")),
        -3.0 => return Err(fail("production limit exceeded")),
        -4.0 => {
            return Err(fail(
                "recursive production merge accepts a negative example",
            ));
        }
        _ => {}
    }
    let grammar =
        grammar_from_productions(&productions, &observations.alphabet, &options.delimiters);
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
        options: limits.clone(),
        delimiters: options.delimiters.clone(),
        positives: positives.to_vec(),
        negatives: negatives.to_vec(),
        alphabet: observations.alphabet.clone(),
        productions: productions.clone(),
    };
    let data = encode_structured_data_links(
        &serde_json::to_value(model).map_err(|error| error.to_string())?,
    )?;
    let bundled = create_source_definition_program_links(
        "meta-language/source-program/grammar-structural-inference-program",
    )?;
    let discovered =
        create_discovered_concept_links(std::slice::from_ref(&source), &Default::default())?;
    let archive = create_meta_language_archive(
        &[
            entry(MODEL, "grammar inference", data.network, data.root),
            entry(DEFINITION, "typed program", bundled.network, bundled.root),
            entry(
                GRAMMAR,
                "discovered concepts",
                discovered.network,
                discovered.root,
            ),
        ],
        MetaLanguageArchiveLimits::default(),
    )?;
    Ok(StructuralGrammarInference {
        grammar,
        source,
        alphabet: observations.alphabet,
        productions,
        network: archive.network,
        root: archive.root,
    })
}

/// Reload and recheck the model, typed algorithm and executable grammar.
///
/// # Errors
/// Refuses malformed checkpoints and any change to an archived inference entry.
pub fn read_structural_grammar_inference_links(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<StructuralGrammarInference, String> {
    let entries =
        read_meta_language_archive(network, root, MetaLanguageArchiveLimits::default())?.entries;
    let Some(premise) = entries
        .iter()
        .find(|entry| entry.identifier == MODEL && entry.kind == "grammar inference")
    else {
        return Err(fail("invalid structural discovery checkpoint"));
    };
    if entries.len() != 3 {
        return Err(fail("invalid structural discovery checkpoint"));
    }
    let model: Model = serde_json::from_value(decode_structured_data_links(
        &premise.network,
        premise.root,
    )?)
    .map_err(|error| error.to_string())?;
    if model.schema_version != 1 || model.algorithm != ALGORITHM {
        return Err(fail("unsupported structural discovery model"));
    }
    let options = StructuralGrammarInferenceOptions {
        delimiters: model.delimiters,
        identifier: model.identifier,
        language: model.language,
        edition: model.edition,
        limits: model.options,
    };
    let mut result = infer_structural_grammar(&model.positives, &model.negatives, &options)?;
    let expected = read_meta_language_archive(
        &result.network,
        result.root,
        MetaLanguageArchiveLimits::default(),
    )?
    .entries;
    for entry in expected {
        let actual = entries
            .iter()
            .find(|candidate| candidate.identifier == entry.identifier);
        if !actual.is_some_and(|actual| {
            actual.kind == entry.kind
                && actual.root == entry.root
                && actual.network.to_lino() == entry.network.to_lino()
        }) {
            return Err(fail(
                "checkpoint differs from its actual structural discovery model",
            ));
        }
    }
    result.network = network.clone();
    result.root = root;
    Ok(result)
}
