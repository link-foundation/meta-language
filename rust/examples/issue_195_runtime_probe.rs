use std::fs;
use std::path::PathBuf;

use meta_language::{
    analyze_program, construct_program, decode_program_translation, translate_program, Link,
    LinkNetwork, LinkType, ParseConfiguration, ProgramConstructStatus, ProgramFact,
    ProgramProjectContext, ProgramRange,
};
use serde_json::{json, Value};

fn main() {
    let corpus = corpus();
    let positive = corpus["languages"]
        .as_array()
        .expect("language fixtures")
        .iter()
        .map(|fixture| {
            network_observation(
                fixture["language"]
                    .as_str()
                    .or_else(|| fixture["name"].as_str())
                    .expect("language"),
                fixture["source"].as_str().expect("source"),
            )
        })
        .collect::<Vec<_>>();
    let negative = corpus["negativeCases"]
        .as_array()
        .expect("negative fixtures")
        .iter()
        .map(|fixture| {
            network_observation(
                fixture["language"].as_str().expect("language"),
                fixture["source"].as_str().expect("source"),
            )
        })
        .collect::<Vec<_>>();
    let inventory = inventory()["languages"]
        .as_array()
        .expect("inventory languages")
        .iter()
        .map(|language| {
            network_observation(
                language["name"].as_str().expect("language name"),
                language["source"].as_str().expect("language source"),
            )
        })
        .collect::<Vec<_>>();
    let semantics = corpus["semanticPrograms"]
        .as_array()
        .expect("semantic fixtures")
        .iter()
        .map(program_observation)
        .collect::<Vec<_>>();
    let transforms = corpus["transformationPrograms"]
        .as_array()
        .expect("transformation fixtures")
        .iter()
        .map(transform_observation)
        .collect::<Vec<_>>();
    let translations = translation_observations(&corpus);

    println!(
        "{}",
        serde_json::to_string(&json!({
            "schemaVersion": 1,
            "positive": positive,
            "negative": negative,
            "inventory": inventory,
            "semantics": semantics,
            "transforms": transforms,
            "translations": translations,
        }))
        .expect("probe output serializes")
    );
}

fn corpus() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/four-language-conformance.json");
    serde_json::from_str(&fs::read_to_string(path).expect("shared corpus is readable"))
        .expect("shared corpus is valid JSON")
}

fn inventory() -> Value {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../parity/language-grammar-inventory.json");
    serde_json::from_str(&fs::read_to_string(path).expect("grammar inventory is readable"))
        .expect("grammar inventory is valid JSON")
}

fn network_observation(language: &str, source: &str) -> Value {
    let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    let mut nodes = network
        .links()
        .filter(|link| {
            matches!(
                link.metadata().link_type(),
                Some(LinkType::Syntax | LinkType::Token)
            )
        })
        .map(node_signature)
        .collect::<Vec<_>>();
    nodes.sort();

    let mut edges = network
        .links()
        .filter(|link| {
            matches!(
                link.metadata().link_type(),
                Some(LinkType::Syntax | LinkType::Token)
            )
        })
        .filter_map(|child| {
            canonical_parent(&network, child)
                .map(|parent| format!("{} -> {}", node_signature(parent), node_signature(child)))
        })
        .collect::<Vec<_>>();
    edges.sort();

    let mut fields = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
        .filter_map(|field| {
            let [parent, label, child] = field.references() else {
                return None;
            };
            let parent = network.link(*parent)?;
            let child = network.link(*child)?;
            let label = network.link(*label)?.metadata().term().unwrap_or_default();
            Some(format!(
                "{}: {} -> {}",
                label,
                node_signature(parent),
                node_signature(child)
            ))
        })
        .collect::<Vec<_>>();
    fields.sort();

    // Trivia links attach extra tokens to the Syntax link directly above them;
    // the self-description `trivia` point has no span.
    let mut trivia = network
        .links()
        .filter(|link| {
            link.metadata().link_type() == Some(LinkType::Trivia)
                && link.metadata().span().is_some()
        })
        .map(|link| {
            let references = link
                .references()
                .iter()
                .filter_map(|id| network.link(*id))
                .map(node_signature)
                .collect::<Vec<_>>();
            format!(
                "{}: {}",
                link.metadata().term().unwrap_or_default(),
                references.join(" -> ")
            )
        })
        .collect::<Vec<_>>();
    trivia.sort();

    let mut annotations = network
        .links()
        .filter_map(annotation_signature)
        .collect::<Vec<_>>();
    annotations.sort();

    json!({
        "language": canonical_language(language),
        "source": source,
        "reconstruction": network.reconstruct_text(),
        "clean": network.verify_full_match(None).is_clean(),
        "nodes": nodes,
        "edges": edges,
        "fields": fields,
        "trivia": trivia,
        "annotations": annotations,
    })
}

/// Region-scoped language identification and Unicode annotations. Both
/// runtimes share the trigram identifier, so its term is compared exactly; the
/// word segmenter engines differ by runtime, so only their presence is
/// compared and their verdicts show up in the tokens.
fn annotation_signature(link: &Link) -> Option<String> {
    let metadata = link.metadata();
    metadata.span()?;
    let term = metadata.term()?;
    let kind = match metadata.link_type()? {
        LinkType::Language => "language",
        LinkType::Semantic => "semantic",
        _ => return None,
    };
    let term = if term.starts_with("segmentation:") {
        "segmentation"
    } else {
        term
    };
    Some(format!(
        "{kind} {term} @ {}",
        metadata
            .language()
            .map(canonical_language)
            .unwrap_or_default()
    ))
}

fn node_signature(link: &Link) -> String {
    let metadata = link.metadata();
    let span = metadata.span().map(|span| {
        let range = span.byte_range();
        let start = span.start_point();
        let end = span.end_point();
        json!({
            "byteStart": range.start(),
            "byteEnd": range.end(),
            "startRow": start.row(),
            "startColumn": start.column(),
            "endRow": end.row(),
            "endColumn": end.column(),
        })
    });
    let flags = metadata.flags();
    serde_json::to_string(&json!({
        "type": match metadata.link_type() {
            Some(LinkType::Syntax) => "syntax",
            Some(LinkType::Token) => "token",
            _ => "other",
        },
        "term": metadata.term(),
        "language": metadata.language().map(canonical_language),
        "named": metadata.link_type() != Some(LinkType::Token) && metadata.is_named(),
        "span": span,
        "flags": {
            "isError": flags.is_error(),
            "hasError": flags.has_error(),
            "isMissing": flags.is_missing(),
            "isExtra": flags.is_extra(),
        },
    }))
    .expect("node signature serializes")
}

fn canonical_parent<'a>(network: &'a LinkNetwork, child: &Link) -> Option<&'a Link> {
    let parent = child.references().first().and_then(|id| network.link(*id));
    parent.filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
}

fn program_observation(fixture: &Value) -> Value {
    let source = fixture["source"].as_str().expect("source");
    let language = fixture["language"].as_str().expect("language");
    let project = ProgramProjectContext::new(
        fixture["project"]["root"].as_str().expect("project root"),
        strings(&fixture["project"]["files"]),
        strings(&fixture["project"]["dependencies"]),
    );
    let program = analyze_program(source, language, project).expect("semantic analysis");
    json!({
        "language": language,
        "source": source,
        "scopes": program.scopes().iter().map(|scope| json!({
            "parent": scope.parent().is_some(),
            "range": range(scope.range()),
            "depth": scope.depth(),
        })).collect::<Vec<_>>(),
        "bindings": program.bindings().iter().map(|binding| json!({
            "name": binding.name(),
            "kind": binding.kind(),
            "declaration": range(binding.declaration()),
            "references": binding.references().iter().copied().map(range).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "unresolvedReferences": sorted(facts(program.unresolved_references())),
        "modules": sorted(facts(program.modules())),
        "types": sorted(facts(&program.types().iter()
            .filter(|fact| fact.kind() != "syntax-type").cloned().collect::<Vec<_>>())),
        "extensions": sorted(facts(&program.extensions().iter()
            .filter(|fact| semantic_extension(language, source, fact)).cloned().collect::<Vec<_>>())),
        "proofs": sorted(facts(&program.proofs().iter()
            .filter(|fact| semantic_proof(language, source, fact)).cloned().collect::<Vec<_>>())),
        "sourceMappingCoverage": source_mapping_coverage(
            source.len(),
            program
                .source_mappings()
                .iter()
                .map(meta_language::ProgramSourceMapping::range),
        ),
        "diagnostics": sorted(program.diagnostics().iter().map(|diagnostic| json!({
            "kind": diagnostic.kind(),
            "term": diagnostic.term(),
            "range": range(diagnostic.range()),
        })).collect()),
        "constructs": sorted(program.constructs().iter().map(|construct| json!({
            "kind": construct.kind(),
            "status": match construct.status() {
                ProgramConstructStatus::Represented => "represented",
                ProgramConstructStatus::NotPresent => "not-present",
                ProgramConstructStatus::NotApplicable => "not-applicable",
                ProgramConstructStatus::Unavailable => "unavailable",
            },
            "evidencePresent": !construct.evidence().is_empty(),
            "rationale": construct.rationale(),
        })).collect()),
    })
}

fn source_mapping_coverage(source_len: usize, ranges: impl Iterator<Item = ProgramRange>) -> Value {
    let ranges = ranges.collect::<Vec<_>>();
    json!({
        "nonEmpty": !ranges.is_empty(),
        "fullSpan": ranges.iter().any(|range| range.start() == 0 && range.end() == source_len),
        "rangesValid": ranges.iter().all(|range| range.start() <= range.end() && range.end() <= source_len),
    })
}

fn semantic_extension(language: &str, source: &str, fact: &ProgramFact) -> bool {
    let markers: &[&str] = match language {
        "JavaScript" => &["String", "directive"],
        "Rust" => &["macro_rules", "#"],
        "Lean" => &[
            "macro", "notation", "syntax", "postfix", "prefix", "infix", "infixl", "infixr", "@",
        ],
        _ => &["Notation", "Ltac", "#"],
    };
    markers.contains(&fact.kind())
        && (fact.kind() == "directive"
            || source.get(fact.range().start()..fact.range().end()) == Some(fact.kind()))
}

fn semantic_proof(language: &str, source: &str, fact: &ProgramFact) -> bool {
    let markers: &[&str] = match language {
        "Lean" => &["theorem", "lemma", "by", "rfl", "simp", "exact", "apply"],
        "Rocq" => &[
            "Theorem",
            "Lemma",
            "Proof",
            "Qed",
            "Defined",
            "reflexivity",
            "intros",
            "exact",
            "apply",
        ],
        _ => &[],
    };
    markers.contains(&fact.kind())
        && source.get(fact.range().start()..fact.range().end()) == Some(fact.kind())
}

fn sorted(mut values: Vec<Value>) -> Vec<Value> {
    values.sort_by_key(ToString::to_string);
    values
}

fn facts(values: &[ProgramFact]) -> Vec<Value> {
    values
        .iter()
        .map(|fact| {
            json!({
                "kind": fact.kind(),
                "name": fact.name(),
                "range": range(fact.range()),
                "phase": fact.phase(),
            })
        })
        .collect()
}

fn range(value: ProgramRange) -> Value {
    json!({ "start": value.start(), "end": value.end() })
}

fn transform_observation(fixture: &Value) -> Value {
    let language = fixture["language"].as_str().expect("language");
    let source = fixture["source"].as_str().expect("source");
    let first = fixture["first"].as_str().expect("first");
    let inserted = fixture["inserted"].as_str().expect("inserted");
    let program = construct_program(source, language, ProgramProjectContext::default())
        .expect("structured construction");
    let first_range = ProgramRange::new(0, first.len());
    let second_range = ProgramRange::new(first.len(), source.len());
    let replacement = first
        .replace("first", "primary")
        .replace("FIRST", "primary");
    json!({
        "language": language,
        "queryCount": program.query_syntax("identifier").len(),
        "emit": program.emit(),
        "replace": program.replace(first_range, &replacement).unwrap().emit(),
        "insert": program.insert(second_range.end(), inserted).unwrap().emit(),
        "delete": program.delete(second_range).unwrap().emit(),
        "clone": program.clone_range(first_range, second_range.end()).unwrap().emit(),
        "move": program.move_range(second_range, 0).unwrap().emit(),
    })
}

fn translation_observations(corpus: &Value) -> Vec<Value> {
    let languages = ["JavaScript", "Rust", "Lean", "Rocq"];
    let fixtures = corpus["semanticPrograms"]
        .as_array()
        .expect("semantic fixtures");
    let mut observations = Vec::new();
    for source_language in languages {
        let source = fixtures
            .iter()
            .find(|fixture| fixture["language"] == source_language)
            .and_then(|fixture| fixture["source"].as_str())
            .expect("translation source");
        for target_language in languages {
            if source_language == target_language {
                continue;
            }
            let translated =
                translate_program(source, source_language, target_language).expect("translation");
            let decoded = decode_program_translation(translated.code(), target_language)
                .expect("translation decode");
            observations.push(json!({
                "sourceLanguage": source_language,
                "targetLanguage": target_language,
                "code": translated.code(),
                "decodedSource": decoded.source(),
                "contract": {
                    "schemaVersion": translated.contract().schema_version,
                    "source": translated.contract().source,
                    "target": translated.contract().target,
                    "support": match translated.contract().support {
                        meta_language::TranslationSupport::PortableEncoding => "portable-encoding",
                        meta_language::TranslationSupport::SemanticSubset => "semantic-subset",
                    },
                    "observation": translated.contract().observation,
                    "requiredRuntime": translated.contract().required_runtime,
                    "encoding": translated.contract().encoding,
                    "assumptions": translated.contract().assumptions,
                    "obligation": translated.contract().obligation,
                },
            }));
        }
    }
    observations
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("string array")
        .iter()
        .map(|item| item.as_str().expect("string").to_string())
        .collect()
}

fn canonical_language(language: &str) -> &str {
    if language.eq_ignore_ascii_case("coq") {
        "Rocq"
    } else {
        language
    }
}
