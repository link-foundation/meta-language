//! Actual archive/data transport; generated rules own history continuity.
use super::{
    Model, RegularGrammarInference, RegularGrammarInferenceOptions, index, infer_regular_grammar,
    number, read_regular_grammar_inference_links,
};
use crate::grammar_inference_refinement_rules as rules;
use crate::translation::surface_reference_facts::create_surface_reference_facts;
use crate::{
    LinkId, LinkNetwork, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits,
    create_meta_language_archive, create_source_definition_program_links,
    decode_structured_data_links, encode_structured_data_links, read_meta_language_archive,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::sync::Arc;
const MODEL: &str = "regular-discovery-model";
const GRAMMAR: &str = "regular-discovery-grammar";
const ALGORITHM: &str = "regular-discovery-algorithm";
const JOURNAL: &str = "regular-discovery-refinement-journal";
const REFINEMENT: &str = "regular-discovery-refinement-algorithm";
#[cfg(test)]
mod tests {
    use super::rules;
    #[test]
    fn shared_journal_profile_keeps_bounds_versions_and_source_identities() {
        for count in [0.0, 1.0, 8.0] {
            assert!(rules::accept_regular_grammar_refinement_count(count));
        }
        for count in [-1.0, 0.5, 9.0, f64::NAN, f64::INFINITY] {
            assert!(!rules::accept_regular_grammar_refinement_count(count));
        }
        assert!(rules::accept_regular_grammar_refinement_journal(
            1.0,
            "labelled regular grammar refinement 1"
        ));
        assert!(!rules::accept_regular_grammar_refinement_journal(
            2.0,
            "labelled regular grammar refinement 1"
        ));
        assert!(rules::accept_regular_grammar_refinement_identifier(
            "new",
            &["old".to_owned()]
        ));
        assert!(!rules::accept_regular_grammar_refinement_identifier(
            "old",
            &["old".to_owned()]
        ));
    }
}
fn fail(message: impl std::fmt::Display) -> String {
    format!("grammar inference refinement: {message}")
}
fn limits() -> MetaLanguageArchiveLimits {
    MetaLanguageArchiveLimits {
        node_limit: index(rules::read_regular_grammar_refinement_resource_limit(
            "nodes",
        )),
        reference_limit: index(rules::read_regular_grammar_refinement_resource_limit(
            "references",
        )),
        byte_limit: index(rules::read_regular_grammar_refinement_resource_limit(
            "bytes",
        )),
    }
}
fn select<'a>(
    entries: &'a [MetaLanguageArchiveEntry],
    identifier: &str,
    kind: &str,
) -> Result<&'a MetaLanguageArchiveEntry, String> {
    entries
        .iter()
        .find(|entry| entry.identifier == identifier && entry.kind == kind)
        .ok_or_else(|| fail(format!("missing {identifier}")))
}
fn model(entry: &MetaLanguageArchiveEntry) -> Result<Value, String> {
    decode_structured_data_links(&entry.network, entry.root)
}
fn base(
    entries: &[MetaLanguageArchiveEntry],
    model: &MetaLanguageArchiveEntry,
    grammar: &MetaLanguageArchiveEntry,
) -> Result<RegularGrammarInference, String> {
    let mut model = model.clone();
    model.identifier = MODEL.to_owned();
    let mut grammar = grammar.clone();
    grammar.identifier = GRAMMAR.to_owned();
    let archive = create_meta_language_archive(
        &[
            model,
            select(entries, ALGORITHM, "typed program")?.clone(),
            grammar,
        ],
        MetaLanguageArchiveLimits::default(),
    )?;
    read_regular_grammar_inference_links(&archive.network, archive.root)
}
fn journal_of(entries: &[MetaLanguageArchiveEntry]) -> Result<Value, String> {
    let Some(entry) = entries.iter().find(|entry| entry.identifier == JOURNAL) else {
        return Ok(
            json!({"schemaVersion":1,"model":"labelled regular grammar refinement 1","steps":[]}),
        );
    };
    if entry.kind != "grammar inference" {
        return Err(fail("invalid journal entry role"));
    }
    let journal = model(entry)?;
    let Some(fields) = journal.as_object() else {
        return Err(fail("unsupported refinement journal"));
    };
    if !rules::accept_regular_grammar_refinement_journal(
        journal["schemaVersion"].as_f64().unwrap_or(f64::NAN),
        journal["model"].as_str().unwrap_or(""),
    ) || journal["steps"]
        .as_array()
        .is_none_or(|steps| !rules::accept_regular_grammar_refinement_count(number(steps.len())))
    {
        return Err(fail("unsupported refinement journal"));
    }
    if fields.len() != 3
        || !["schemaVersion", "model", "steps"]
            .iter()
            .all(|name| fields.contains_key(*name))
    {
        return Err(fail("unknown journal fields"));
    }
    Ok(journal)
}
struct Checkpoint {
    entries: Vec<MetaLanguageArchiveEntry>,
    journal: Value,
    current: RegularGrammarInference,
    model: Model,
    current_model: MetaLanguageArchiveEntry,
    current_grammar: MetaLanguageArchiveEntry,
}
fn read_checkpoint(network: &LinkNetwork, root: LinkId) -> Result<Checkpoint, String> {
    let entries = read_meta_language_archive(network, root, limits())?.entries;
    let journal = journal_of(&entries)?;
    let stored_definition = entries.iter().find(|entry| entry.identifier == REFINEMENT);
    if entries.iter().any(|entry| entry.identifier == JOURNAL) {
        let bundled = create_source_definition_program_links(
            "meta-language/source-program/grammar-inference-refinement-program",
        )?;
        if stored_definition.is_none_or(|entry| {
            entry.kind != "typed program"
                || entry.root != bundled.root
                || entry.network.to_lino() != bundled.network.to_lino()
        }) {
            return Err(fail(
                "the selected refinement definition differs from its versioned implementation",
            ));
        }
    } else if stored_definition.is_some() {
        return Err(fail("a refinement definition needs its actual journal"));
    }
    let steps = journal["steps"]
        .as_array()
        .ok_or_else(|| fail("unsupported refinement journal"))?;
    let current_model = select(&entries, MODEL, "grammar inference")?.clone();
    let current_grammar = select(&entries, GRAMMAR, "discovered concepts")?.clone();
    let mut models = Vec::new();
    let mut history = Vec::new();
    let mut expected = BTreeSet::new();
    for ordinal in 0..steps.len() {
        let model_name =
            rules::read_regular_grammar_refinement_entry_name(number(ordinal), "model");
        let grammar_name =
            rules::read_regular_grammar_refinement_entry_name(number(ordinal), "grammar");
        let stored_model = select(&entries, &model_name, "grammar inference")?;
        let stored_grammar = select(&entries, &grammar_name, "discovered concepts")?;
        history.push((stored_model.clone(), stored_grammar.clone()));
        models.push(model(stored_model)?);
        expected.insert(model_name);
        expected.insert(grammar_name);
    }
    if entries.iter().any(|entry| {
        entry.identifier.starts_with("regular-discovery-history-")
            && !expected.contains(&entry.identifier)
    }) {
        return Err(fail("unreferenced history entry"));
    }
    models.push(model(&current_model)?);
    let roots = Value::Array(models.iter().chain(steps).cloned().collect());
    let facts = create_surface_reference_facts(&roots);
    let start = index(facts.starts[index(facts.root)]);
    let identities = &facts.targets[start..start + models.len() + steps.len()];
    let decision = rules::validate_regular_grammar_refinement_history(
        &facts.forms,
        &facts.texts,
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        &identities[..models.len()],
        &identities[models.len()..],
    );
    if !matches!(decision, 1.0) {
        return Err(fail(format!(
            "invalid refinement history (reason {decision})"
        )));
    }
    for (stored_model, stored_grammar) in &history {
        base(&entries, stored_model, stored_grammar)?;
    }
    let current = base(&entries, &current_model, &current_grammar)?;
    let model = serde_json::from_value(models.pop().ok_or_else(|| fail("missing current model"))?)
        .map_err(|error| fail(error.to_string()))?;
    Ok(Checkpoint {
        entries,
        journal,
        current,
        model,
        current_model,
        current_grammar,
    })
}
/// Recheck every prior decision/grammar and each exact appended corpus prefix.
/// # Errors
/// Returns altered algorithms/contexts, missing history, discontinuous examples,
/// incompatible settings or exceeded finite graph/learner limits.
pub fn read_refined_regular_grammar_inference_links(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<RegularGrammarInference, String> {
    let checkpoint = read_checkpoint(network, root)?;
    Ok(RegularGrammarInference {
        network: network.clone(),
        root,
        ..checkpoint.current
    })
}
/// Append labelled counterexamples under the same learner and resource settings.
/// Duplicate observations retain their statistical multiplicity. All historical
/// model/grammar groups remain linked; the complete algorithm is shared once.
/// # Errors
/// Returns invalid checkpoints, identities, examples, contradictions or budgets.
pub fn refine_regular_grammar_inference(
    network: &LinkNetwork,
    root: LinkId,
    positives: &[Vec<String>],
    negatives: &[Vec<String>],
    identifier: Option<&str>,
) -> Result<RegularGrammarInference, String> {
    let checkpoint = read_checkpoint(network, root)?;
    let mut steps = checkpoint.journal["steps"]
        .as_array()
        .ok_or_else(|| fail("unsupported refinement journal"))?
        .clone();
    let ordinal = steps.len();
    if !rules::accept_regular_grammar_refinement_count(number(ordinal + 1)) {
        return Err(fail("refinement history limit exceeded"));
    }
    let next_identifier = identifier.map_or_else(
        || {
            rules::read_regular_grammar_refinement_identifier(
                &checkpoint.model.identifier,
                number(ordinal + 1),
            )
        },
        str::to_owned,
    );
    let existing = std::iter::once(checkpoint.model.identifier.clone())
        .chain(
            steps
                .iter()
                .filter_map(|step| step["identifier"].as_str().map(str::to_owned)),
        )
        .collect::<Vec<_>>();
    if !rules::accept_regular_grammar_refinement_identifier(&next_identifier, &existing) {
        return Err(fail(
            "a new source-qualified refinement identity is required",
        ));
    }
    let all_positives = checkpoint
        .model
        .positives
        .iter()
        .chain(positives)
        .cloned()
        .collect::<Vec<_>>();
    let all_negatives = checkpoint
        .model
        .negatives
        .iter()
        .chain(negatives)
        .cloned()
        .collect::<Vec<_>>();
    let next = infer_regular_grammar(
        &all_positives,
        &all_negatives,
        &RegularGrammarInferenceOptions {
            parameters: checkpoint.model.options,
            identifier: next_identifier.clone(),
            language: checkpoint.model.language,
            edition: checkpoint.model.edition,
        },
    )?;
    let mut entries = read_meta_language_archive(
        &next.network,
        next.root,
        MetaLanguageArchiveLimits::default(),
    )?
    .entries;
    steps.push(json!({"identifier":checkpoint.model.identifier,"nextIdentifier":next_identifier,"positives":positives,"negatives":negatives}));
    let data = encode_structured_data_links(
        &json!({"schemaVersion":1,"model":"labelled regular grammar refinement 1","steps":steps}),
    )?;
    let refinement = if let Some(entry) = checkpoint
        .entries
        .iter()
        .find(|entry| entry.identifier == REFINEMENT)
    {
        entry.clone()
    } else {
        let definition = create_source_definition_program_links(
            "meta-language/source-program/grammar-inference-refinement-program",
        )?;
        MetaLanguageArchiveEntry {
            identifier: REFINEMENT.to_owned(),
            kind: "typed program".to_owned(),
            network: Arc::new(definition.network),
            root: definition.root,
        }
    };
    entries.extend(checkpoint.entries.into_iter().filter(|entry| {
        ![MODEL, GRAMMAR, ALGORITHM, JOURNAL, REFINEMENT].contains(&entry.identifier.as_str())
    }));
    let mut old_model = checkpoint.current_model;
    old_model.identifier =
        rules::read_regular_grammar_refinement_entry_name(number(ordinal), "model");
    entries.push(old_model);
    let mut old_grammar = checkpoint.current_grammar;
    old_grammar.identifier =
        rules::read_regular_grammar_refinement_entry_name(number(ordinal), "grammar");
    entries.push(old_grammar);
    entries.push(MetaLanguageArchiveEntry {
        identifier: JOURNAL.to_owned(),
        kind: "grammar inference".to_owned(),
        network: Arc::new(data.network),
        root: data.root,
    });
    entries.push(refinement);
    let archive = create_meta_language_archive(&entries, limits())?;
    Ok(RegularGrammarInference {
        network: archive.network,
        root: archive.root,
        ..next
    })
}
