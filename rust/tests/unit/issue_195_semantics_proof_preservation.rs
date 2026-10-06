//! Issue #195 proof preservation: every Lean and Rocq theorem of the pinned
//! translation corpus (`parity/fixtures/four-language-conformance.json`) is
//! carried by the public `translate_program` as an obligation stating the
//! source statement, so every false restatement is rejected by the source
//! kernel and by every target; into Lean and Rocq each theorem and assertion is
//! discharged by the target kernel itself, whose assumption report shows the
//! obligation closed without an axiom of the translation or of another kernel;
//! and into JavaScript and Rust a theorem is a bounded check reported apart
//! from the source-kernel proof, which a restatement true only on the bounded
//! domain passes while every kernel rejects it. Mirrors
//! `js/tests/issue-195-semantics-proof-preservation.test.js`.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;

use meta_language::{
    ProgramTranslation, TranslationObligation, TranslationSupport, translate_program,
};
use regex::Regex;
use serde_json::Value;

use super::issue_195_observations as observations;
use super::issue_195_translation_pairs::{
    LANGUAGES, Run, TemporaryDirectory, corpus, mutate, run, source_text, strings, tool,
    tool_available, toolchain_required,
};

const REQUIREMENT_ID: &str = "I195-SEMANTICS-PROOF-PRESERVATION";
/// The axioms of Lean's own foundations; any other axiom would be an outside authority.
const LEAN_STANDARD_AXIOMS: [&str; 3] = ["propext", "Quot.sound", "Classical.choice"];

fn record(assertion: &str, test_name: &str) {
    let fixture_id = format!(
        "planned:repository-directive:{}",
        REQUIREMENT_ID.to_lowercase()
    );
    observations::record(&observations::Observation {
        requirement_id: REQUIREMENT_ID,
        suffix: "behavior",
        fixture_id: &fixture_id,
        fixture_file: observations::FOUR_LANGUAGE_FIXTURE,
        assertions: &[assertion],
        test_name,
    });
}

/// Whether every toolchain is installed; acceptance runs require all of them.
fn toolchains_present() -> bool {
    if LANGUAGES.iter().all(|language| tool_available(language)) {
        return true;
    }
    assert!(
        !toolchain_required(),
        "{} are required for acceptance",
        LANGUAGES.map(tool).join(", ")
    );
    false
}

fn proof_target(language: &str) -> bool {
    corpus()["targets"][language]["proof"]
        .as_bool()
        .expect("proof flag")
}

fn proof_sources() -> Vec<&'static str> {
    LANGUAGES
        .into_iter()
        .filter(|language| !strings(&corpus()["sources"][language]["theorems"]).is_empty())
        .collect()
}

fn others(language: &str) -> Vec<&'static str> {
    LANGUAGES
        .into_iter()
        .filter(|other| *other != language)
        .collect()
}

fn translate(text: &str, source: &str, target: &str) -> ProgramTranslation {
    let translation = translate_program(text, source, target).expect("public translator");
    assert_eq!(
        translation.contract().support,
        TranslationSupport::SemanticTranslation,
        "{source} -> {target}: {:?}",
        translation.diagnostic()
    );
    translation
}

fn obligations(translation: &ProgramTranslation) -> &[TranslationObligation] {
    &translation
        .semantics()
        .expect("semantic translation")
        .obligations
}

fn theorem_names(translation: &ProgramTranslation) -> Vec<&str> {
    obligations(translation)
        .iter()
        .filter(|obligation| obligation.kind == "theorem")
        .map(|obligation| obligation.source.as_str())
        .collect()
}

/// What a toolchain reports about a checked artifact.
struct Check {
    accepted: bool,
    diagnostics: String,
    theorems: Option<Run>,
}

/// Checks `code` with the `language` toolchain. A kernel (Lean, Rocq) checks
/// every proof and reports the assumptions each of `names` rests on; a program
/// (JavaScript, Rust) runs its `--ml-check-theorems` bounded checks.
fn check(language: &str, code: &str, directory: &Path, name: &str, names: &[&str]) -> Check {
    match language {
        "Lean" | "Rocq" => {
            let lean = language == "Lean";
            let file = directory.join(format!("{name}{}", if lean { ".lean" } else { ".v" }));
            let report = names
                .iter()
                .map(|theorem| {
                    if lean {
                        format!("#print axioms {theorem}")
                    } else {
                        format!("Print Assumptions {theorem}.")
                    }
                })
                .collect::<Vec<_>>();
            fs::write(&file, format!("{code}\n{}\n", report.join("\n"))).expect("artifact writes");
            let result = if lean {
                run(Command::new("lean").arg(&file))
            } else {
                run(Command::new("rocq")
                    .args(["compile", "-q"])
                    .arg(file.file_name().expect("artifact name"))
                    .current_dir(directory))
            };
            let diagnostics = format!("{}\n{}", result.stdout, result.stderr);
            let lowered = diagnostics.to_lowercase();
            Check {
                accepted: result.ok
                    && !["warning", "error", "sorry"]
                        .iter()
                        .any(|word| lowered.contains(word)),
                diagnostics,
                theorems: None,
            }
        }
        "JavaScript" => {
            let program = directory.join(format!("{name}.mjs"));
            fs::write(&program, code).expect("artifact writes");
            let theorems = run(Command::new("node")
                .arg(&program)
                .arg("--ml-check-theorems"));
            Check {
                accepted: true,
                diagnostics: theorems.stderr.clone(),
                theorems: Some(theorems),
            }
        }
        _ => {
            let file = directory.join(format!("{name}.rs"));
            let program = directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
            fs::write(&file, code).expect("artifact writes");
            let mut rustc = Command::new("rustc");
            rustc.args(["--edition", "2024", "-O"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                rustc.arg("-C").arg(format!("linker={linker}"));
            }
            let compiled = run(rustc.arg("-o").arg(&program).arg(&file));
            if !compiled.ok {
                return Check {
                    accepted: false,
                    diagnostics: compiled.stderr,
                    theorems: None,
                };
            }
            let theorems = run(Command::new(&program).arg("--ml-check-theorems"));
            Check {
                accepted: true,
                diagnostics: theorems.stderr.clone(),
                theorems: Some(theorems),
            }
        }
    }
}

/// The axioms Lean reports for `name`, or `None` when it reports nothing.
fn lean_axioms(diagnostics: &str, name: &str) -> Option<Vec<String>> {
    let line = diagnostics
        .lines()
        .find(|text| text.starts_with(&format!("'{name}' ")))?;
    if line == format!("'{name}' does not depend on any axioms") {
        return Some(Vec::new());
    }
    let pattern = Regex::new(r"^'[^']+' depends on axioms: \[([^\]]*)\]$").expect("axioms pattern");
    pattern.captures(line).map(|captures| {
        captures[1]
            .split(',')
            .map(|axiom| axiom.trim().to_owned())
            .collect()
    })
}

/// Whether the kernel run reports every one of `names` closed by its own foundations.
fn closed_by_kernel(language: &str, diagnostics: &str, names: &[&str]) -> bool {
    if language == "Lean" {
        return names.iter().all(|name| {
            lean_axioms(diagnostics, name).is_some_and(|axioms| {
                axioms
                    .iter()
                    .all(|axiom| LEAN_STANDARD_AXIOMS.contains(&axiom.as_str()))
            })
        });
    }
    !diagnostics.lines().any(|line| line.starts_with("Axioms:"))
        && diagnostics
            .matches("Closed under the global context")
            .count()
            == names.len()
}

fn forbidden_markers(target: &str, code: &str) -> Vec<String> {
    let body = code.split('\n').skip(1).collect::<Vec<_>>().join("\n");
    strings(&corpus()["targets"][target]["forbidden"])
        .into_iter()
        .filter(|marker| {
            Regex::new(&format!(r"\b{}\b", regex::escape(marker)))
                .expect("marker pattern")
                .is_match(&body)
        })
        .map(str::to_owned)
        .collect()
}

fn declares(code: &str, pattern: &str) -> bool {
    Regex::new(pattern)
        .expect("declaration pattern")
        .is_match(code)
}

#[test]
fn issue_195_theorems_are_carried_as_obligations_stating_the_source_theorem() {
    if !toolchains_present() {
        return;
    }
    assert_eq!(proof_sources(), ["Lean", "Rocq"]);
    let mut rejected = 0;
    for source in proof_sources() {
        let from = &corpus()["sources"][source];
        let (_, text) = source_text(source);
        let mutations = from["theoremMutations"]
            .as_array()
            .expect("theorem mutations");
        let theorems = strings(&from["theorems"]);
        assert_eq!(
            mutations
                .iter()
                .map(|mutation| mutation["theorem"].as_str().expect("theorem"))
                .collect::<Vec<_>>(),
            theorems
        );
        for target in others(source) {
            let translation = translate(&text, source, target);
            assert_eq!(
                theorem_names(&translation),
                theorems,
                "{source} -> {target}"
            );
            for obligation in obligations(&translation)
                .iter()
                .filter(|obligation| obligation.kind == "theorem")
            {
                let closed = declares(
                    &text,
                    &format!(
                        r"(?m)^(?:theorem|Theorem) {} :",
                        regex::escape(&obligation.source)
                    ),
                );
                assert_eq!(
                    obligation.closed_goal, closed,
                    "{source} -> {target} {}",
                    obligation.source
                );
                let pattern = if proof_target(target) {
                    format!(
                        r"(?m)^(?:theorem|Theorem) {}\b",
                        regex::escape(&obligation.target)
                    )
                } else {
                    format!(
                        r"theorem {}: holds on the bounded domain",
                        regex::escape(&obligation.source)
                    )
                };
                assert!(declares(translation.code(), &pattern), "{pattern}");
            }
        }
        // Every false restatement of a theorem is rejected by the source kernel
        // and by every target, so the carried obligation is the source statement.
        let directory = TemporaryDirectory::new(&format!("proof-{source}"));
        for mutation in mutations {
            let mutated = mutate(&text, mutation);
            let theorem = mutation["theorem"].as_str().expect("theorem");
            let own = check(source, &mutated, &directory.0, "source", &[]);
            assert!(!own.accepted, "{source} kernel rejects the false {theorem}");
            thread::scope(|scope| {
                for target in others(source) {
                    let (mutated, directory, theorems) = (&mutated, &directory.0, &theorems);
                    scope.spawn(move || {
                        let translation = translate(mutated, source, target);
                        assert_eq!(&theorem_names(&translation), theorems);
                        let name = format!("{target}_{theorem}").to_lowercase();
                        let result = check(target, translation.code(), directory, &name, &[]);
                        if proof_target(target) {
                            assert!(
                                !result.accepted,
                                "{target} kernel rejects the false {source} {theorem}"
                            );
                        } else {
                            assert!(result.accepted, "{}", result.diagnostics);
                            let report = result.theorems.expect("theorem report");
                            assert!(!report.ok, "{target} bounded check fails on {theorem}");
                            assert!(
                                report.stderr.contains(&format!(
                                    "theorem {theorem} fails on a bounded input"
                                )),
                                "{}",
                                report.stderr
                            );
                        }
                    });
                }
            });
            rejected += others(source).len();
        }
    }
    assert_eq!(rejected, 24);
    record(
        "theoremsCarriedAsObligations",
        "issue_195_theorems_are_carried_as_obligations_stating_the_source_theorem",
    );
}

/// Checks the translation of `source` into the kernel `target`: every
/// obligation is discharged by the target kernel, which reports it closed by
/// its own foundations. Returns the number of obligations.
fn discharge_by_kernel(source: &str, target: &str, directory: &Path) -> usize {
    let from = &corpus()["sources"][source];
    let theorems = strings(&from["theorems"]).len();
    let assertions =
        usize::try_from(from["assertions"].as_u64().expect("assertions")).expect("assertion count");
    let translation = translate(&source_text(source).1, source, target);
    let obligations = obligations(&translation);
    assert_eq!(
        obligations.len(),
        theorems + assertions,
        "{source} -> {target}"
    );
    for obligation in obligations {
        // The record names the obligation and its discharger;
        // it carries no verdict, which only the kernel run gives.
        let TranslationObligation {
            source: name,
            target: _,
            kind,
            closed_goal: _,
            discharge,
            check,
        } = obligation;
        assert!(kind == "theorem" || kind == "assertion", "{name}");
        assert_eq!(discharge, "target-kernel", "{source} -> {target} {name}");
        assert_eq!(*check, None, "{source} -> {target} {name}");
    }
    assert_eq!(
        obligations
            .iter()
            .filter(|obligation| obligation.kind == "assertion")
            .count(),
        assertions
    );
    assert!(
        forbidden_markers(target, translation.code()).is_empty(),
        "{source} -> {target} admits nothing"
    );
    let names = obligations
        .iter()
        .map(|obligation| obligation.target.as_str())
        .collect::<Vec<_>>();
    let name = format!("from_{source}").to_lowercase();
    let result = check(target, translation.code(), directory, &name, &names);
    assert!(result.accepted, "{}", result.diagnostics);
    assert!(
        closed_by_kernel(target, &result.diagnostics, &names),
        "{}",
        result.diagnostics
    );
    names.len()
}

#[test]
fn issue_195_proof_obligations_into_lean_and_rocq_are_discharged_by_the_target_kernel() {
    if !toolchains_present() {
        return;
    }
    let mut discharged = 0;
    for target in LANGUAGES
        .into_iter()
        .filter(|language| proof_target(language))
    {
        let directory = TemporaryDirectory::new(&format!("proof-{target}"));
        discharged += thread::scope(|scope| {
            let mut handles = Vec::new();
            for source in others(target) {
                let directory = &directory.0;
                handles.push(scope.spawn(move || discharge_by_kernel(source, target, directory)));
            }
            handles
                .into_iter()
                .map(|handle| handle.join().expect("kernel check"))
                .sum::<usize>()
        });
    }
    // 4 theorems from each kernel source and 1 assertion from each program
    // source, into both kernels.
    assert_eq!(discharged, 2 * (4 + 1 + 1));
    record(
        "obligationsDischargedByTarget",
        "issue_195_proof_obligations_into_lean_and_rocq_are_discharged_by_the_target_kernel",
    );
}

/// Checks the translation of `source` into the program target `target`:
/// theorems are bounded checks whose proof stays with the source kernel.
fn check_bounded_target(source: &str, target: &str, text: &str, directory: &Path) {
    let theorems = strings(&corpus()["sources"][source]["theorems"]);
    let translation = translate(text, source, target);
    for obligation in obligations(&translation) {
        assert_ne!(
            obligation.discharge, "target-kernel",
            "{source} -> {target}"
        );
        if obligation.kind == "theorem" {
            assert_eq!(obligation.discharge, "source-kernel");
            assert_eq!(obligation.check.as_deref(), Some("bounded"));
        } else {
            assert_eq!(obligation.discharge, "runtime-assertion");
            assert_eq!(obligation.check, None);
        }
    }
    let encoding = translation
        .semantics()
        .expect("semantic translation")
        .encodings
        .iter()
        .find(|encoding| encoding.id == "theorem-properties");
    assert_eq!(
        encoding.is_some(),
        !theorems.is_empty(),
        "{source} -> {target}"
    );
    if let Some(encoding) = encoding {
        assert!(
            encoding
                .statement
                .ends_with("bounded domain, and its proof remains checked by the source kernel"),
            "{}",
            encoding.statement
        );
    }
    let name = format!("{target}_from_{source}").to_lowercase();
    let result = check(target, translation.code(), directory, &name, &[]);
    assert!(result.accepted, "{}", result.diagnostics);
    let report = result.theorems.expect("theorem report");
    assert!(report.ok, "{}", report.stderr);
    let reported = report
        .stdout
        .lines()
        .filter(|line| line.starts_with("theorem "))
        .collect::<Vec<_>>();
    let expected = theorems
        .iter()
        .map(|name| format!("theorem {name}: holds on the bounded domain"))
        .collect::<Vec<_>>();
    assert_eq!(reported, expected, "{source} -> {target}");
}

/// A restatement true on the bounded domain but false in general passes the
/// bounded checks, while the source kernel and both target kernels reject it.
fn check_bounded_only(source: &str, text: &str, mutation: &Value, directory: &Path) {
    let mutated = mutate(text, mutation);
    let theorem = mutation["theorem"].as_str().expect("theorem");
    let own = check(source, &mutated, directory, "bounded_only", &[]);
    assert!(
        !own.accepted,
        "{source} kernel rejects the bounded-only {theorem}"
    );
    thread::scope(|scope| {
        for target in others(source) {
            let mutated = &mutated;
            scope.spawn(move || {
                let translation = translate(mutated, source, target);
                let obligation = obligations(&translation)
                    .iter()
                    .find(|obligation| obligation.source == theorem)
                    .expect("bounded-only obligation");
                let name = format!("bounded_only_{target}").to_lowercase();
                let result = check(target, translation.code(), directory, &name, &[]);
                if proof_target(target) {
                    assert_eq!(obligation.discharge, "target-kernel");
                    assert!(
                        !result.accepted,
                        "{target} kernel rejects the bounded-only {source} {theorem}"
                    );
                } else {
                    assert_eq!(
                        (obligation.discharge.as_str(), obligation.check.as_deref()),
                        ("source-kernel", Some("bounded"))
                    );
                    let report = result.theorems.expect("theorem report");
                    assert!(report.ok, "{}", report.stderr);
                    let holds = format!("theorem {theorem}: holds on the bounded domain");
                    assert!(report.stdout.lines().any(|line| line == holds), "{holds}");
                }
            });
        }
    });
}

#[test]
fn issue_195_bounded_theorem_checks_are_reported_separately_from_proofs() {
    if !toolchains_present() {
        return;
    }
    let programs = LANGUAGES
        .into_iter()
        .filter(|language| !proof_target(language))
        .collect::<Vec<_>>();
    assert_eq!(programs, ["JavaScript", "Rust"]);
    for source in LANGUAGES {
        let from = &corpus()["sources"][source];
        let theorems = strings(&from["theorems"]);
        let (_, text) = source_text(source);
        let directory = TemporaryDirectory::new(&format!("proof-bounded-{source}"));
        // The proof of every theorem stays with the source kernel, which closes it.
        if !theorems.is_empty() {
            let own = check(source, &text, &directory.0, "source", &theorems);
            assert!(own.accepted, "{}", own.diagnostics);
            assert!(
                closed_by_kernel(source, &own.diagnostics, &theorems),
                "{}",
                own.diagnostics
            );
        }
        thread::scope(|scope| {
            for target in programs.iter().filter(|target| **target != source) {
                let (text, directory) = (&text, &directory.0);
                scope.spawn(move || check_bounded_target(source, target, text, directory));
            }
        });
        if !from["boundedOnlyMutation"].is_null() {
            check_bounded_only(source, &text, &from["boundedOnlyMutation"], &directory.0);
        }
    }
    assert_eq!(
        proof_sources()
            .into_iter()
            .map(|source| corpus()["sources"][source]["boundedOnlyMutation"]["theorem"].as_str())
            .collect::<Vec<_>>(),
        [Some("sumTo_formula"), Some("sumTo_formula")]
    );
    record(
        "boundedChecksReportedSeparately",
        "issue_195_bounded_theorem_checks_are_reported_separately_from_proofs",
    );
}

#[test]
fn issue_195_rocq_proof_steps_rewrite_with_each_rule_a_bounded_number_of_times() {
    // An unbounded `rewrite <- ?ih` with `ih : Tree.mirror l = l` rewrites `l`
    // into `Tree.mirror l` forever, so a false restatement must meet a bound to
    // be rejected in seconds rather than after minutes of search.
    let step = Regex::new(r"\((rewrite(?: <-)?) ([^;]*); ml_close\)").expect("rewrite step");
    let bounded = Regex::new(r"^\d+\?\S+$").expect("bounded rule");
    let mut steps = 0;
    for source in proof_sources()
        .into_iter()
        .filter(|language| *language != "Rocq")
    {
        let (_, text) = source_text(source);
        let mutations = corpus()["sources"][source]["theoremMutations"]
            .as_array()
            .expect("theorem mutations");
        let variants = std::iter::once(text.clone())
            .chain(mutations.iter().map(|mutation| mutate(&text, mutation)));
        for variant in variants {
            let translation = translate(&variant, source, "Rocq");
            for captures in step.captures_iter(translation.code()) {
                for rule in captures[2].split(", ") {
                    assert!(
                        bounded.is_match(rule),
                        "{source} -> Rocq rewrites with {rule} unboundedly"
                    );
                }
                steps += 1;
            }
        }
    }
    assert!(
        steps > 0,
        "the corpus proofs rewrite with hypotheses or lemmas"
    );
}
