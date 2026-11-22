use super::*;

pub(super) fn module_facts(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    source: &str,
    language: &str,
    project: &ProgramProjectContext,
    project_requests: &[ProgramFact],
) -> Vec<ProgramFact> {
    let markers = module_markers(language);
    let mut facts = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if !markers.contains(&token.text.as_str()) {
            continue;
        }
        let names = tokens[index + 1..]
            .iter()
            .take_while(|candidate| {
                candidate.text != ";" && !markers.contains(&candidate.text.as_str())
            })
            .filter(|candidate| candidate.kind == TokenKind::Identifier)
            .take(3)
            .map(|candidate| candidate.text.as_str())
            .collect::<Vec<_>>()
            .join(".");
        facts.push(ProgramFact::new(&token.text, names, token.range));
    }
    facts.extend(project.dependencies().iter().map(|dependency| {
        ProgramFact::new(
            "declared-project-dependency",
            dependency,
            ProgramRange::default(),
        )
    }));
    let requests = module_requests(syntax, source, language);
    for request in requests.into_iter().chain(project_requests.iter().cloned()) {
        if project_has_module(project, &request.name, language) {
            facts.push(ProgramFact::new(
                "recognized-toolchain-module",
                &request.name,
                request.range,
            ));
        }
        facts.push(request);
    }
    unique_facts(facts)
}

pub(super) fn type_facts(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    language: &str,
) -> Vec<ProgramFact> {
    let mut facts = syntax
        .iter()
        .filter(|mapping| {
            let term = mapping.term.to_ascii_lowercase();
            term.contains("type") || term.contains("universe")
        })
        .map(|mapping| {
            ProgramFact::new("syntax-type", &mapping.term, mapping.range).with_phase("surface")
        })
        .collect::<Vec<_>>();
    for (index, token) in tokens.iter().enumerate() {
        if token.text == ":"
            && let Some(value) = tokens.get(index + 1)
        {
            facts.push(
                ProgramFact::new("annotation", &value.text, value.range).with_phase("surface"),
            );
        }
        if matches!(token.text.as_str(), "universe" | "Universe" | "Type") {
            facts.push(
                ProgramFact::new(
                    "universe",
                    tokens
                        .get(index + 1)
                        .map_or(&token.text, |value| &value.text),
                    token.range,
                )
                .with_phase("surface"),
            );
        }
    }
    if language == "JavaScript" {
        facts.push(
            ProgramFact::new("dynamic-type", "ECMAScript value", ProgramRange::default())
                .with_phase("runtime"),
        );
    }
    unique_facts(facts)
}

pub(super) fn extension_facts(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    source: &str,
    language: &str,
) -> Vec<ProgramFact> {
    let markers = extension_markers(language);
    let mut facts = tokens
        .iter()
        .filter(|token| markers.contains(&token.text.as_str()))
        .map(|token| ProgramFact::new(&token.text, &token.text, token.range))
        .collect::<Vec<_>>();
    facts.extend(syntax.iter().filter_map(|mapping| {
        let term = mapping.term.to_ascii_lowercase();
        [
            "macro",
            "attribute",
            "decorator",
            "quotation",
            "template",
            "notation",
        ]
        .iter()
        .any(|needle| term.contains(needle))
        .then(|| ProgramFact::new(&mapping.term, &mapping.term, mapping.range))
    }));
    if language == "JavaScript"
        && let Some(start) = source.find("\"use strict\"")
    {
        facts.push(ProgramFact::new(
            "directive",
            "use strict",
            ProgramRange::new(start, start + 12),
        ));
    }
    unique_facts(facts)
}

pub(super) fn proof_facts(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    language: &str,
) -> Vec<ProgramFact> {
    if matches!(language, "JavaScript" | "Rust") {
        return Vec::new();
    }
    let markers = proof_markers(language);
    let facts = tokens
        .iter()
        .filter(|token| markers.contains(&token.text.as_str()))
        .map(|token| ProgramFact::new(&token.text, &token.text, token.range))
        .chain(syntax.iter().filter_map(|mapping| {
            let term = mapping.term.to_ascii_lowercase();
            ["theorem", "proof", "tactic"]
                .iter()
                .any(|needle| term.contains(needle))
                .then(|| ProgramFact::new(&mapping.term, &mapping.term, mapping.range))
        }))
        .collect();
    unique_facts(facts)
}

pub(super) fn diagnostic_facts(network: &LinkNetwork) -> Vec<ProgramDiagnostic> {
    network
        .verify_full_match(None)
        .issues()
        .iter()
        .filter_map(|issue| {
            let link = network.link(issue.link_id())?;
            let span = link.metadata().span();
            Some(ProgramDiagnostic {
                kind: if link.metadata().flags().is_missing() {
                    "missing"
                } else {
                    "parse-error"
                },
                term: link.metadata().term().unwrap_or_default().to_string(),
                range: span.map_or_else(ProgramRange::default, |span| {
                    ProgramRange::new(span.byte_range().start(), span.byte_range().end())
                }),
            })
        })
        .collect()
}

pub(super) fn project_diagnostics(
    modules: &[ProgramFact],
    resolved: &[(String, usize)],
) -> Vec<ProgramDiagnostic> {
    let recognized = modules
        .iter()
        .filter(|fact| fact.kind == "recognized-toolchain-module")
        .map(|fact| fact.name.as_str())
        .collect::<std::collections::HashSet<_>>();
    modules
        .iter()
        .filter(|fact| {
            fact.kind == "module-import"
                && !recognized.contains(fact.name.as_str())
                && !resolved
                    .iter()
                    .any(|(name, start)| *name == fact.name && *start == fact.range.start)
        })
        .map(|fact| ProgramDiagnostic {
            kind: "missing-project-context",
            term: fact.name.clone(),
            range: fact.range,
        })
        .collect()
}

pub(super) fn construct_facts(
    program: &ProgramRepresentation,
    context: &project::ProjectResult,
) -> Vec<ProgramConstruct> {
    SEMANTIC_CONSTRUCTS
        .iter()
        .map(|kind| {
            let mut evidence = construct_evidence(program, kind);
            evidence.extend(project::project_evidence(&program.project, context, kind));
            if evidence.is_empty() && *kind == "surface-expansion-elaboration-traces" {
                return ProgramConstruct {
                    kind,
                    status: ProgramConstructStatus::Unavailable,
                    evidence,
                    rationale: Some(
                        "no macro expansion or elaboration trace has been produced".to_string(),
                    ),
                };
            }
            if evidence.is_empty()
                && *kind == "proof-terms-and-tactics"
                && matches!(program.language, "JavaScript" | "Rust")
            {
                return ProgramConstruct {
                    kind,
                    status: ProgramConstructStatus::NotApplicable,
                    evidence,
                    rationale: Some(format!(
                        "{} defines no proof/tactic sublanguage",
                        program.language
                    )),
                };
            }
            let represented = !evidence.is_empty();
            ProgramConstruct {
                kind,
                status: if represented {
                    ProgramConstructStatus::Represented
                } else {
                    ProgramConstructStatus::NotPresent
                },
                evidence,
                rationale: (!represented).then(|| {
                    "the analyzed source contains no instance of this construct".to_string()
                }),
            }
        })
        .collect()
}

fn construct_evidence(program: &ProgramRepresentation, kind: &str) -> Vec<ProgramFact> {
    match kind {
        "modules-and-imports" => program.modules.clone(),
        "scopes-and-bindings" => program
            .bindings
            .iter()
            .map(|binding| ProgramFact::new(&binding.kind, &binding.name, binding.declaration))
            .collect(),
        "recursive-definitions" => program
            .bindings
            .iter()
            .filter(|binding| {
                matches!(
                    binding.kind.as_str(),
                    "function" | "Fixpoint" | "CoFixpoint" | "def"
                ) && !binding.references.is_empty()
            })
            .map(|binding| ProgramFact::new("recursive", &binding.name, binding.declaration))
            .collect(),
        "types-and-universes" => program.types.clone(),
        "effects" => effect_facts(program),
        "attributes" => program
            .extensions
            .iter()
            .filter(|fact| {
                let kind = fact.kind.to_ascii_lowercase();
                ["attribute", "directive", "allow", "local", "simp", "#"]
                    .iter()
                    .any(|needle| kind.contains(needle))
            })
            .cloned()
            .collect(),
        "macros-and-notation" => program
            .extensions
            .iter()
            .filter(|fact| {
                let kind = fact.kind.to_ascii_lowercase();
                [
                    "macro", "notation", "template", "tagged", "prefix", "postfix", "infix",
                    "syntax",
                ]
                .iter()
                .any(|needle| kind.contains(needle))
            })
            .cloned()
            .collect(),
        "proof-terms-and-tactics" => program.proofs.clone(),
        "project-context-and-dependencies" => program
            .project
            .files()
            .iter()
            .chain(program.project.dependencies())
            .map(|name| ProgramFact::new("project", name, ProgramRange::default()))
            .collect(),
        _ => Vec::new(),
    }
}

fn effect_facts(program: &ProgramRepresentation) -> Vec<ProgramFact> {
    let mut facts = Vec::new();
    for marker in effect_markers(program.language) {
        facts.extend(program.source.match_indices(marker).map(|(start, value)| {
            ProgramFact::new(
                "effect",
                value,
                ProgramRange::new(start, start + value.len()),
            )
        }));
    }
    unique_facts(facts)
}
