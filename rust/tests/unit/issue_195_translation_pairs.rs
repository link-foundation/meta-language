//! Directed-translation evidence for the 12 JavaScript, Rust, Lean and Rocq pairs.
//!
//! Each pair translates the pinned project corpus with the public
//! `translate_program`, reparses the artifact with the target grammar,
//! validates and runs it with the target toolchain, and compares what it
//! prints and proves with the source run by its own toolchain. A
//! fault-injected source (a changed base case) must make the target toolchain
//! reject or observe the translation differently, so the oracle is not the
//! translator's own output. Mirrors `js/tests/issue-195-translation-pairs.test.js`.

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use meta_language::{
    LinkNetwork, LinkType, ParseConfiguration, ProgramTranslation, SemanticTranslation,
    TranslationSupport, decode_program_translation, read_translation_provenance, translate_program,
};
use regex::Regex;
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::issue_195_observations as observations;

const LANGUAGES: [&str; 4] = ["JavaScript", "Rust", "Lean", "Rocq"];
const ASSERTIONS: [&str; 15] = [
    "publicTranslatorUsed",
    "realTargetArtifact",
    "targetParses",
    "nativeTargetValidation",
    "observationContractChecked",
    "semanticPreservationChecked",
    "observationModelRecorded",
    "encodingAndRuntimeRecorded",
    "assumptionsRecorded",
    "formalObligationsDischarged",
    "sourceMappingsPreserved",
    "provenanceRecorded",
    "noSourceRelabelling",
    "noUnsupportedDescriptor",
    "noSilentWeakening",
];

fn tool(language: &str) -> &'static str {
    match language {
        "JavaScript" => "node",
        "Rust" => "rustc",
        "Lean" => "lean",
        _ => "rocq",
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn corpus() -> &'static Value {
    static CORPUS: OnceLock<Value> = OnceLock::new();
    CORPUS.get_or_init(|| {
        let path = root().join(observations::FOUR_LANGUAGE_FIXTURE);
        let shared: Value =
            serde_json::from_str(&fs::read_to_string(path).expect("shared corpus is readable"))
                .expect("shared corpus is valid JSON");
        shared["translationCorpus"].clone()
    })
}

fn corpus_file(name: &str) -> Vec<u8> {
    let directory = corpus()["directory"].as_str().expect("corpus directory");
    fs::read(root().join(directory).join(name)).expect("corpus file is readable")
}

fn expected_bytes() -> &'static [u8] {
    static EXPECTED: OnceLock<Vec<u8>> = OnceLock::new();
    EXPECTED.get_or_init(|| {
        corpus_file(
            corpus()["expectedOutput"]["file"]
                .as_str()
                .expect("expected output"),
        )
    })
}

fn expected_lines() -> Vec<String> {
    lines_of(&String::from_utf8_lossy(expected_bytes()))
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut hex, byte| {
            use std::fmt::Write as _;
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .map(|items| items.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

/// Acceptance runs must execute every pair; other runs skip pairs whose toolchain is absent.
fn toolchain_required() -> bool {
    std::env::var_os("ISSUE_195_OBSERVATION_FILE").is_some()
}

fn tool_available(language: &str) -> bool {
    static AVAILABLE: OnceLock<Vec<bool>> = OnceLock::new();
    let available = AVAILABLE.get_or_init(|| {
        LANGUAGES
            .iter()
            .map(|language| {
                Command::new(tool(language))
                    .arg("--version")
                    .output()
                    .is_ok_and(|output| output.status.success())
            })
            .collect()
    });
    available[LANGUAGES
        .iter()
        .position(|name| *name == language)
        .expect("corpus language")]
}

struct Run {
    ok: bool,
    stdout: String,
    stderr: String,
}

fn run(command: &mut Command) -> Run {
    command.output().map_or_else(
        |error| Run {
            ok: false,
            stdout: String::new(),
            stderr: error.to_string(),
        },
        |output| Run {
            ok: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        },
    )
}

fn lines_of(text: &str) -> Vec<String> {
    let mut lines = text.split('\n').map(str::to_owned).collect::<Vec<_>>();
    lines.pop();
    lines
}

/// Rocq prints `Eval vm_compute in main` as a list of string literals, as
/// `"line"%string :: ... :: nil` or, with the string scope open, `["line"; ...]`.
fn rocq_lines(stdout: &str) -> Vec<String> {
    let literal = Regex::new(r#""((?:[^"]|"")*)""#).expect("string literal pattern");
    literal
        .captures_iter(stdout)
        .map(|captures| captures[1].replace("\"\"", "\""))
        .collect()
}

struct Execution {
    accepted: bool,
    diagnostics: String,
    ran: bool,
    lines: Vec<String>,
    theorems: Option<Run>,
}

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "issue-195-{}-{}-{}",
            name.to_ascii_lowercase(),
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        fs::create_dir_all(&path).expect("temporary directory");
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Validates `code` as a `language` program with its toolchain and observes it:
/// the printed lines, and for program targets the `--ml-check-theorems` report.
fn execute(language: &str, code: &str, directory: &Path, name: &str) -> Execution {
    let extension = corpus()["sources"][language]["extension"]
        .as_str()
        .expect("extension");
    let file = directory.join(format!("{name}{extension}"));
    fs::write(&file, code).expect("artifact writes");
    let binary = directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let observe = |arguments: &[&str]| match language {
        "JavaScript" => run(Command::new("node").arg(&file).args(arguments)),
        _ => run(Command::new(&binary).args(arguments)),
    };
    let validation = match language {
        "JavaScript" => run(Command::new("node").arg("--check").arg(&file)),
        "Rust" => {
            let mut rustc = Command::new("rustc");
            rustc.args(["--edition", "2024", "-O"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                rustc.arg("-C").arg(format!("linker={linker}"));
            }
            run(rustc.arg("-o").arg(&binary).arg(&file))
        }
        "Lean" => run(Command::new("lean").arg("--run").arg(&file)),
        _ => run(Command::new("rocq")
            .args(["compile", "-q"])
            .arg(file.file_name().expect("artifact name"))
            .current_dir(directory)),
    };
    let diagnostics = format!("{}\n{}", validation.stdout, validation.stderr);
    let lowered = diagnostics.to_lowercase();
    let proof = matches!(language, "Lean" | "Rocq");
    let clean = validation.ok
        && !(proof
            && ["warning", "error", "sorry"]
                .iter()
                .any(|word| lowered.contains(word)));
    if !clean {
        return Execution {
            accepted: false,
            diagnostics,
            ran: false,
            lines: Vec::new(),
            theorems: None,
        };
    }
    let (main, theorems) = match language {
        "Lean" => (validation, None),
        "Rocq" => {
            let lines = rocq_lines(&validation.stdout);
            let stdout = lines
                .iter()
                .flat_map(|line| [line.as_str(), "\n"])
                .collect();
            (
                Run {
                    stdout,
                    ..validation
                },
                None,
            )
        }
        _ => (observe(&[]), Some(observe(&["--ml-check-theorems"]))),
    };
    Execution {
        accepted: true,
        diagnostics,
        ran: main.ok,
        lines: lines_of(&main.stdout),
        theorems,
    }
}

fn source_text(language: &str) -> (Vec<u8>, String) {
    let bytes = corpus_file(
        corpus()["sources"][language]["file"]
            .as_str()
            .expect("source file"),
    );
    let text = String::from_utf8(bytes.clone()).expect("UTF-8 source");
    (bytes, text)
}

fn source_run(language: &str) -> &'static Execution {
    static RUNS: [OnceLock<Execution>; 4] = [
        OnceLock::new(),
        OnceLock::new(),
        OnceLock::new(),
        OnceLock::new(),
    ];
    let index = LANGUAGES
        .iter()
        .position(|name| *name == language)
        .expect("corpus language");
    RUNS[index].get_or_init(|| {
        let directory = TemporaryDirectory::new(&format!("source-{language}"));
        execute(language, &source_text(language).1, &directory.0, "project")
    })
}

fn mutate(text: &str, mutation: &Value) -> String {
    let find = mutation["find"].as_str().expect("mutation site");
    let replace = mutation["replace"].as_str().expect("mutation replacement");
    assert_eq!(
        text.matches(find).count(),
        1,
        "the mutation site {find:?} is unique"
    );
    text.replacen(find, replace, 1)
}

fn last_segment(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

fn identifier_terms(network: &LinkNetwork) -> HashSet<String> {
    network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Token))
        .filter_map(|link| link.metadata().term().map(str::to_owned))
        .collect()
}

/// A source span in UTF-16 code units, as the translator records it.
fn utf16_slice(text: &str, start: usize, end: usize) -> String {
    let units = text.encode_utf16().collect::<Vec<_>>();
    String::from_utf16_lossy(&units[start..end])
}

fn forbidden_markers(target: &str, code: &str) -> Vec<String> {
    let body = code.split_once('\n').map_or("", |(_, rest)| rest);
    strings(&corpus()["targets"][target]["forbidden"])
        .into_iter()
        .filter(|marker| {
            Regex::new(&format!(r"\b{marker}\b"))
                .expect("marker pattern")
                .is_match(body)
        })
        .map(str::to_owned)
        .collect()
}

struct Pair {
    source: &'static str,
    target: &'static str,
    source_bytes: Vec<u8>,
    text: String,
    translation: ProgramTranslation,
    passed: BTreeSet<&'static str>,
}

impl Pair {
    fn from(&self) -> &'static Value {
        &corpus()["sources"][self.source]
    }

    fn to(&self) -> &'static Value {
        &corpus()["targets"][self.target]
    }

    fn proof(&self) -> bool {
        self.to()["proof"].as_bool().expect("proof flag")
    }

    fn code(&self) -> &str {
        self.translation.code()
    }

    const fn semantics(&self) -> &SemanticTranslation {
        self.translation.semantics().expect("semantic translation")
    }

    fn translate(source: &'static str, target: &'static str) -> Self {
        let (source_bytes, text) = source_text(source);
        let from = &corpus()["sources"][source];
        assert_eq!(
            sha256(&source_bytes),
            from["sha256"],
            "{} is pinned",
            from["file"]
        );
        assert_eq!(
            sha256(expected_bytes()),
            corpus()["expectedOutput"]["sha256"],
            "the pinned expected output"
        );
        let translation = translate_program(&text, source, target).expect("public translator");
        assert_eq!(translation.source_language(), source);
        assert_eq!(translation.target_language(), target);
        let mut passed = BTreeSet::from(["publicTranslatorUsed"]);

        let contract = translation.contract();
        assert_eq!(
            contract.support,
            TranslationSupport::SemanticTranslation,
            "{:?}",
            translation.diagnostic()
        );
        assert!(translation.diagnostic().is_none());
        assert!(contract.obligation.is_none());
        assert!(
            !translation
                .code()
                .contains("meta-language:portable-source-envelope")
        );
        assert!(decode_program_translation(translation.code(), target).is_err());
        passed.insert("noUnsupportedDescriptor");
        Self {
            source,
            target,
            source_bytes,
            text,
            translation,
            passed,
        }
    }

    fn reparse(&mut self) -> LinkNetwork {
        let grammar = corpus()["sources"][self.target]["grammar"]
            .as_str()
            .expect("grammar");
        let network = LinkNetwork::parse(self.code(), grammar, ParseConfiguration::default());
        assert_eq!(
            network.reconstruct_text(),
            self.code(),
            "the target CST reconstructs the artifact byte for byte"
        );
        assert!(
            network.verify_full_match(None).is_clean(),
            "{} artifact parses without error nodes",
            self.target
        );
        self.passed.insert("targetParses");
        network
    }

    fn check_artifact(&mut self, network: &LinkNetwork) {
        let terms = identifier_terms(network);
        let semantics = self.semantics();
        let javascript = self.target == "JavaScript";
        for mapping in &semantics.mappings {
            let name = last_segment(&mapping.target);
            if javascript && mapping.kind == "data" {
                // Untyped JavaScript has no type declaration; the data encoding stands for it.
                assert!(
                    semantics
                        .encodings
                        .iter()
                        .any(|encoding| encoding.id == "data"),
                    "{}",
                    mapping.target
                );
            } else if javascript && mapping.kind == "constructor" {
                assert!(self.code().contains(&format!("$: '{name}'")), "tags {name}");
            } else {
                assert!(
                    terms.contains(name),
                    "{} declares {}",
                    self.target,
                    mapping.target
                );
            }
        }
        self.passed.insert("realTargetArtifact");
    }

    fn check_behaviour(&mut self, target_run: &Execution, original_run: &Execution) {
        let expected = expected_lines();
        assert_eq!(self.semantics().entry.as_deref(), Some("main"));
        assert_eq!(
            target_run.lines, expected,
            "{} prints the observed lines",
            self.target
        );
        self.passed.insert("observationContractChecked");

        assert!(original_run.accepted, "{}", original_run.diagnostics);
        assert_eq!(
            original_run.lines, expected,
            "{} source prints the observed lines",
            self.source
        );
        assert_eq!(target_run.lines, original_run.lines);

        let mutated = mutate(&self.text, &self.from()["mutation"]);
        let mutant = translate_program(&mutated, self.source, self.target).expect("mutant");
        assert_eq!(
            mutant.contract().support,
            TranslationSupport::SemanticTranslation
        );
        let directory = TemporaryDirectory::new(&format!("mutant-{}-{}", self.source, self.target));
        let name = format!("mutant_{}_to_{}", self.source, self.target).to_ascii_lowercase();
        let mutant_run = execute(self.target, mutant.code(), &directory.0, &name);
        // `fact 5 = 120` is a theorem or assertion of every corpus program, so a proof
        // target's kernel rejects the mutant; a program target aborts on the translated
        // assertion or reports the translated theorem as failing.
        if self.proof() {
            assert!(
                !mutant_run.accepted,
                "{} kernel rejects the mutant",
                self.target
            );
        } else {
            assert!(mutant_run.accepted, "{}", mutant_run.diagnostics);
            assert_ne!(
                mutant_run.lines, expected,
                "{} observes the mutant",
                self.target
            );
            let theorems_fail = mutant_run.theorems.as_ref().is_some_and(|run| !run.ok);
            assert!(
                !mutant_run.ran || theorems_fail,
                "{} reports fact_five",
                self.target
            );
        }
        self.passed.insert("semanticPreservationChecked");
    }

    fn check_records(&mut self) {
        let semantics = self.semantics();
        let contract = self.translation.contract();
        assert_ne!(contract.observation, "");
        assert!(semantics.observation_procedure.contains(tool(self.target)));
        let mut passed = vec!["observationModelRecorded"];

        assert_ne!(contract.encoding, "");
        assert!(
            semantics
                .encodings
                .iter()
                .any(|encoding| encoding.id == "program-output")
        );
        for encoding in &semantics.encodings {
            assert!(
                !encoding.id.is_empty() && !encoding.statement.is_empty(),
                "{}",
                encoding.id
            );
        }
        assert_eq!(
            semantics.runtime_dependencies.first().map(String::as_str),
            self.to()["runtime"].as_str()
        );
        let imports = Regex::new(r"(?m)^From ([A-Za-z]+) Require Import ([^.\n]+)\.$")
            .expect("import pattern");
        for captures in imports.captures_iter(self.code()) {
            for module in captures[2].split_whitespace() {
                let dependency = format!("{}.{module}", &captures[1]);
                assert!(
                    semantics.runtime_dependencies.contains(&dependency),
                    "{dependency}"
                );
            }
        }
        passed.push("encodingAndRuntimeRecorded");

        let pair = format!("{} -> {}", self.source, self.target);
        let ids = semantics
            .assumptions
            .iter()
            .map(|a| a.id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            strings(&corpus()["assumptions"][&pair]),
            "{pair} assumptions"
        );
        let statements = semantics.assumptions.iter().map(|a| a.statement.as_str());
        assert!(contract.assumptions.iter().copied().eq(statements));
        assert!(semantics.assumptions.iter().all(|a| !a.details.is_empty()));
        passed.push("assumptionsRecorded");
        self.passed.extend(passed);
    }

    fn check_obligations(&mut self, target_run: &Execution, original_run: &Execution) {
        let semantics = self.semantics();
        let from = self.from();
        let theorems = semantics
            .obligations
            .iter()
            .filter(|obligation| obligation.kind == "theorem")
            .map(|obligation| obligation.source.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            theorems,
            strings(&from["theorems"]),
            "every source theorem is an obligation"
        );
        let assertions = semantics
            .obligations
            .iter()
            .filter(|o| o.kind == "assertion")
            .count();
        assert_eq!(Some(assertions as u64), from["assertions"].as_u64());
        for obligation in &semantics.obligations {
            match obligation.discharge.as_str() {
                "target-kernel" => {
                    assert!(self.proof());
                    let declared = Regex::new(&format!(
                        r"(?m)^(?:theorem|Theorem) {}\b",
                        regex::escape(&obligation.target)
                    ))
                    .expect("theorem pattern");
                    assert!(declared.is_match(self.code()), "{}", obligation.target);
                }
                "source-kernel" => {
                    assert!(!self.proof());
                    assert!(
                        original_run.accepted,
                        "{} checks {}",
                        self.source, obligation.source
                    );
                    let report = target_run.theorems.as_ref().expect("theorem report");
                    assert!(report.ok, "{}", report.stderr);
                    let prefix = format!("theorem {}: holds", obligation.source);
                    assert!(
                        report.stdout.lines().any(|line| line.starts_with(&prefix)),
                        "{prefix}"
                    );
                }
                discharge => {
                    assert_eq!(discharge, "runtime-assertion");
                    assert!(!self.proof());
                    assert!(target_run.ran);
                }
            }
        }
        self.passed.insert("formalObligationsDischarged");
    }

    fn check_mappings_and_provenance(&mut self, target_run: &Execution) {
        let semantics = self.semantics();
        let sources = semantics
            .mappings
            .iter()
            .map(|m| m.source.as_str())
            .collect::<Vec<_>>();
        for name in strings(&self.from()["mappings"]) {
            assert!(sources.contains(&name), "{name} is mapped");
        }
        for mapping in semantics
            .mappings
            .iter()
            .filter(|m| m.kind != "constructor")
        {
            let span = mapping.source_span.as_ref().expect("source span");
            let covered = utf16_slice(&self.text, span.start, span.end);
            assert!(
                covered.contains(last_segment(&mapping.source)),
                "{} span",
                mapping.source
            );
        }
        let mut passed = vec!["sourceMappingsPreserved"];

        let from_sha = self.from()["sha256"].as_str().expect("pinned sha");
        let provenance = read_translation_provenance(self.code(), self.target).expect("provenance");
        assert_eq!(provenance.source_language, self.source);
        assert_eq!(provenance.source_sha256, from_sha);
        assert_eq!(provenance.source_bytes, self.source_bytes.len());
        assert_eq!(
            semantics.provenance.header,
            self.code().split('\n').next().unwrap_or("")
        );
        assert_eq!(semantics.provenance.source_sha256, from_sha);
        passed.push("provenanceRecorded");

        assert!(
            !self.code().contains(&self.text),
            "the artifact does not carry the source"
        );
        assert_eq!(semantics.provenance.source_language, self.source);
        assert_ne!(sha256(self.code().as_bytes()), from_sha);
        passed.push("noSourceRelabelling");

        let markers = forbidden_markers(self.target, self.code());
        assert!(
            markers.is_empty(),
            "no admitted or axiomatised obligations: {markers:?}"
        );
        assert_eq!(
            target_run.lines.len(),
            expected_lines().len(),
            "no printed effect is erased"
        );
        passed.push("noSilentWeakening");
        self.passed.extend(passed);
    }
}

fn check_pair(source: &'static str, target: &'static str) {
    if !(tool_available(source) && tool_available(target)) {
        assert!(
            !toolchain_required(),
            "{} and {} are required for acceptance",
            tool(source),
            tool(target)
        );
        eprintln!(
            "skipped {source} -> {target}: {} or {} is not installed",
            tool(source),
            tool(target)
        );
        return;
    }
    let mut pair = Pair::translate(source, target);
    let network = pair.reparse();
    let directory = TemporaryDirectory::new(&format!("{source}-{target}"));
    let name = format!("{source}_to_{target}").to_ascii_lowercase();
    let target_run = execute(target, pair.code(), &directory.0, &name);
    let original_run = source_run(source);
    assert!(target_run.accepted, "{}", target_run.diagnostics);
    assert!(target_run.ran, "{}", target_run.diagnostics);
    pair.passed.insert("nativeTargetValidation");

    pair.check_artifact(&network);
    pair.check_behaviour(&target_run, original_run);
    pair.check_records();
    pair.check_obligations(&target_run, original_run);
    pair.check_mappings_and_provenance(&target_run);

    assert_eq!(pair.passed, BTreeSet::from(ASSERTIONS));
    let requirement_id = format!(
        "I195-TRANSLATE-{}-to-{}",
        source.to_ascii_lowercase(),
        target.to_ascii_lowercase()
    );
    let test_name = format!(
        "{source} -> {target} translation of the project corpus is validated, observed and fault-injected natively"
    );
    observations::record(&observations::Observation {
        requirement_id: &requirement_id,
        suffix: "positive",
        fixture_id: &format!("planned:translation:{source}:{target}"),
        fixture_file: observations::FOUR_LANGUAGE_FIXTURE,
        assertions: &ASSERTIONS,
        test_name: &test_name,
    });
}

macro_rules! pair_tests {
    ($($name:ident: $source:literal => $target:literal,)*) => {
        $(
            #[test]
            fn $name() {
                check_pair($source, $target);
            }
        )*
    };
}

pair_tests! {
    javascript_to_rust_translation_is_validated_observed_and_fault_injected: "JavaScript" => "Rust",
    javascript_to_lean_translation_is_validated_observed_and_fault_injected: "JavaScript" => "Lean",
    javascript_to_rocq_translation_is_validated_observed_and_fault_injected: "JavaScript" => "Rocq",
    rust_to_javascript_translation_is_validated_observed_and_fault_injected: "Rust" => "JavaScript",
    rust_to_lean_translation_is_validated_observed_and_fault_injected: "Rust" => "Lean",
    rust_to_rocq_translation_is_validated_observed_and_fault_injected: "Rust" => "Rocq",
    lean_to_javascript_translation_is_validated_observed_and_fault_injected: "Lean" => "JavaScript",
    lean_to_rust_translation_is_validated_observed_and_fault_injected: "Lean" => "Rust",
    lean_to_rocq_translation_is_validated_observed_and_fault_injected: "Lean" => "Rocq",
    rocq_to_javascript_translation_is_validated_observed_and_fault_injected: "Rocq" => "JavaScript",
    rocq_to_rust_translation_is_validated_observed_and_fault_injected: "Rocq" => "Rust",
    rocq_to_lean_translation_is_validated_observed_and_fault_injected: "Rocq" => "Lean",
}

#[test]
fn fault_injected_corpus_is_rejected_by_the_source_toolchain_itself() {
    let expected = expected_lines();
    for language in LANGUAGES {
        if !tool_available(language) {
            assert!(
                !toolchain_required(),
                "{} is required for acceptance",
                tool(language)
            );
            continue;
        }
        let mutated = mutate(
            &source_text(language).1,
            &corpus()["sources"][language]["mutation"],
        );
        let directory = TemporaryDirectory::new(&format!("mutant-{language}"));
        let mutant_run = execute(language, &mutated, &directory.0, "project");
        assert!(
            !mutant_run.accepted || !mutant_run.ran || mutant_run.lines != expected,
            "{language} observes its own fault-injected source"
        );
    }
}
