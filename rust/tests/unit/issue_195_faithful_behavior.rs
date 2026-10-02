//! Issue #195 faithful behavior: every program of the faithful corpus
//! (`faithfulBehavior` in `parity/fixtures/four-language-conformance.json`) is
//! run with its own toolchain and, through the public `translate_program`, in
//! each of the other three of JavaScript, Rust, Lean and Rocq. Every
//! translation prints the lines the source prints and stops with the message
//! the source aborts with, or does not abort where the source does not: Rust's
//! overflow and division panics, `panic!` and `unreachable!`, JavaScript's
//! thrown errors and BigInt division by zero, the output printed inside a
//! function or an assertion before an abort, and Lean's and Rocq's total
//! arithmetic with unbounded numbers. A translation is the target's own
//! program: it carries no source envelope, no assumption and no escape hatch
//! of a proof target. A fault-injected translation that aborts with another
//! message is told apart. Mirrors `js/tests/issue-195-faithful-behavior.test.js`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use meta_language::{
    ProgramTranslation, TranslationSupport, decode_program_translation, translate_program,
};
use regex::Regex;
use serde_json::Value;

use super::issue_195_observations as observations;
use super::issue_195_translation_pairs::{
    LANGUAGES, TemporaryDirectory, corpus, strings, tool, tool_available, toolchain_required,
};

const REQUIREMENT_ID: &str = "I195-SEMANTICS-FAITHFUL-BEHAVIOR";
const BEHAVIORS: [&str; 4] = [
    "errorsPreserved",
    "abortsPreserved",
    "overflowPreserved",
    "effectsPreserved",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn faithful() -> &'static Value {
    static FAITHFUL: OnceLock<Value> = OnceLock::new();
    FAITHFUL.get_or_init(|| {
        let path = root().join(observations::FOUR_LANGUAGE_FIXTURE);
        let shared: Value =
            serde_json::from_str(&fs::read_to_string(path).expect("shared corpus is readable"))
                .expect("shared corpus is valid JSON");
        shared["faithfulBehavior"].clone()
    })
}

fn cases() -> &'static [Value] {
    faithful()["cases"].as_array().expect("faithful cases")
}

fn extension(language: &str) -> &'static str {
    match language {
        "JavaScript" => ".mjs",
        "Rust" => ".rs",
        "Lean" => ".lean",
        _ => ".v",
    }
}

fn source_text(case: &Value) -> String {
    let directory = faithful()["directory"].as_str().expect("faithful directory");
    let file = case["file"].as_str().expect("case file");
    fs::read_to_string(root().join(directory).join(file)).expect("faithful source is readable")
}

/// What a run prints and the message it aborts with (`None` when it ends normally).
#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    lines: Vec<String>,
    abort: Option<String>,
}

/// An observed run, or the diagnostics of a toolchain that rejected or warned about the program.
struct Observed {
    outcome: Result<Outcome, String>,
    failed: bool,
}

fn expected(case: &Value) -> Outcome {
    Outcome {
        lines: strings(&case["lines"])
            .into_iter()
            .map(str::to_owned)
            .collect(),
        abort: case["abort"].as_str().map(str::to_owned),
    }
}

struct Exit {
    ok: bool,
    stdout: String,
    stderr: String,
}

fn exit(command: &mut Command) -> Exit {
    command.output().map_or_else(
        |error| Exit {
            ok: false,
            stdout: String::new(),
            stderr: error.to_string(),
        },
        |output| Exit {
            ok: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        },
    )
}

fn printed(stdout: &str) -> Vec<String> {
    let mut lines = stdout.split('\n').map(str::to_owned).collect::<Vec<_>>();
    lines.pop();
    lines
}

fn quoted(text: &str) -> Vec<String> {
    Regex::new(r#""((?:[^"]|"")*)""#)
        .expect("string pattern")
        .captures_iter(text)
        .map(|captures| captures[1].replace("\"\"", "\""))
        .collect()
}

/// The first capture of `pattern` in `text`, or all of `text` when it does not match.
fn message(pattern: &str, text: &str) -> String {
    Regex::new(pattern)
        .expect("abort pattern")
        .captures(text)
        .and_then(|captures| captures.get(1))
        .map_or_else(|| text.to_owned(), |found| found.as_str().to_owned())
}

fn ended(result: &Exit, pattern: &str) -> Observed {
    Observed {
        outcome: Ok(Outcome {
            lines: printed(&result.stdout),
            abort: (!result.ok).then(|| message(pattern, &result.stderr)),
        }),
        failed: !result.ok,
    }
}

/// Runs `code` as a `language` program and observes it: the lines it prints
/// and the message it aborts with.
fn observe(language: &str, code: &str, directory: &Path, name: &str) -> Observed {
    let file = directory.join(format!("{name}{}", extension(language)));
    fs::write(&file, code).expect("program writes");
    let rejected = |diagnostics: String| Observed {
        outcome: Err(diagnostics),
        failed: true,
    };
    match language {
        // An uncaught error ends node with status 1 and its `Name: message` on stderr.
        "JavaScript" => ended(
            &exit(Command::new("node").arg(&file)),
            r"(?m)^\w*Error(?:: (.*))?$",
        ),
        "Rust" => {
            let binary = directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
            let mut rustc = Command::new("rustc");
            rustc.args(["--edition", "2024", "-O"]);
            // The source's arithmetic is checked, as in a debug build; a translation into Rust must abort on its own.
            if name == "source" {
                rustc.args(["-C", "overflow-checks=on"]);
            }
            if cfg!(windows)
                && let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER")
            {
                rustc.arg("-C").arg(format!("linker={linker}"));
            }
            let built = exit(rustc.arg("-o").arg(&binary).arg(&file));
            if !built.ok || built.stderr.contains("warning") {
                return rejected(built.stderr);
            }
            // A panic ends the program with status 101 and its message on the line after `panicked at`.
            ended(&exit(&mut Command::new(&binary)), r"panicked at [^\n]*\n([^\n]*)")
        }
        "Lean" => {
            let result = exit(Command::new("lean").arg("--run").arg(&file));
            let diagnostics = format!("{}{}", result.stdout, result.stderr);
            if Regex::new(r":\d+:\d+: (?:warning|error)")
                .expect("diagnostic pattern")
                .is_match(&diagnostics)
            {
                return rejected(diagnostics);
            }
            // An uncaught `IO.userError` ends the program with status 1 and `uncaught exception: message`.
            ended(&result, r"(?m)^uncaught exception: (.*)$")
        }
        _ => {
            let result = exit(
                Command::new("rocq")
                    .args(["compile", "-q"])
                    .arg(file.file_name().expect("program name"))
                    .current_dir(directory),
            );
            if !result.ok
                || Regex::new(r"(?i)warning|error")
                    .expect("diagnostic pattern")
                    .is_match(&result.stderr)
            {
                return rejected(format!("{}{}", result.stdout, result.stderr));
            }
            // `Eval vm_compute in main` of a Rocq main that can abort ends with the abort, `None` or `Some "message"`.
            let outcome = Regex::new(
                r#",\s*(None|Some\s+"((?:[^"]|"")*)"(?:%string)?)\s*\)\s*:\s*list string \* option string\s*$"#,
            )
            .expect("outcome pattern");
            let outcome = match outcome.captures(&result.stdout) {
                None => Outcome {
                    lines: quoted(&result.stdout),
                    abort: None,
                },
                Some(captures) => Outcome {
                    lines: quoted(&result.stdout[..captures.get(0).expect("match").start()]),
                    abort: captures.get(2).map(|found| found.as_str().replace("\"\"", "\"")),
                },
            };
            Observed {
                outcome: Ok(outcome),
                failed: false,
            }
        }
    }
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

fn translate(text: &str, source: &str, target: &str) -> ProgramTranslation {
    let pair = format!("{source} -> {target}");
    let translation = translate_program(text, source, target).expect("public translator");
    // No pass-through: the target's own program, with nothing assumed and no proof escape hatch.
    assert_eq!(
        translation.contract().support,
        TranslationSupport::SemanticTranslation,
        "{pair}: {:?}",
        translation.diagnostic()
    );
    assert!(translation.diagnostic().is_none(), "{pair}");
    let semantics = translation.semantics().expect("semantic translation");
    assert!(semantics.assumptions.is_empty(), "{pair} assumes nothing");
    assert!(
        !translation
            .code()
            .contains("meta-language:portable-source-envelope"),
        "{pair}"
    );
    assert!(
        decode_program_translation(translation.code(), target).is_err(),
        "{pair}"
    );
    assert!(
        forbidden_markers(target, translation.code()).is_empty(),
        "{pair}"
    );
    translation
}

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

#[test]
fn issue_195_the_faithful_corpus_covers_every_source_language_and_behavior() {
    for language in LANGUAGES {
        for behavior in BEHAVIORS {
            assert!(
                cases().iter().any(|case| case["language"] == language
                    && strings(&case["preserves"]).contains(&behavior)),
                "a {language} program shows {behavior}"
            );
        }
    }
    for case in cases() {
        let file = case["file"].as_str().expect("case file");
        let language = case["language"].as_str().expect("case language");
        assert!(file.ends_with(extension(language)), "{file}");
        assert!(
            strings(&case["preserves"])
                .iter()
                .all(|behavior| BEHAVIORS.contains(behavior)),
            "{file}"
        );
    }
    assert!(cases().iter().filter(|case| !case["abort"].is_null()).count() >= 8);
    assert!(cases().iter().any(|case| !case["abort"].is_null()
        && !strings(&case["lines"]).is_empty()));
}

#[test]
fn issue_195_faithful_behavior_holds_in_all_12_directed_translations() {
    if !toolchains_present() {
        return;
    }
    let mut translated = BTreeSet::new();
    let mut preserved = BEHAVIORS.map(|_| BTreeSet::new());
    for case in cases() {
        let file = case["file"].as_str().expect("case file");
        let source = case["language"].as_str().expect("case language");
        let text = source_text(case);
        let directory = TemporaryDirectory::new(&format!("faithful-{}", file.replace('.', "-")));
        let run = observe(source, &text, &directory.0, "source");
        assert_eq!(
            run.outcome.expect("the source runs"),
            expected(case),
            "{file} runs as recorded"
        );
        for target in LANGUAGES.into_iter().filter(|target| *target != source) {
            let pair = format!("{source} -> {target}");
            let translation = translate(&text, source, target);
            if !case["abort"].is_null() && matches!(target, "Lean" | "Rocq") {
                assert!(
                    translation
                        .semantics()
                        .expect("semantic translation")
                        .encodings
                        .iter()
                        .any(|encoding| encoding.id == "abort-threading"),
                    "{pair} threads the abort"
                );
            }
            let name = format!("to_{}", target.to_lowercase());
            let observed = observe(target, translation.code(), &directory.0, &name);
            match &observed.outcome {
                Ok(outcome) => assert_eq!(
                    outcome,
                    &expected(case),
                    "{file}: {pair} prints and aborts as the source"
                ),
                Err(diagnostics) => panic!("{file}: {pair} is rejected: {diagnostics}"),
            }
            // A program target ends with a failure status where the source aborts.
            if target != "Rocq" {
                assert_eq!(
                    observed.failed,
                    !case["abort"].is_null(),
                    "{file}: {pair} exit status"
                );
            }
            for (index, behavior) in BEHAVIORS.iter().enumerate() {
                if strings(&case["preserves"]).contains(behavior) {
                    preserved[index].insert(pair.clone());
                }
            }
            translated.insert(pair);
        }
    }

    // A translation that aborts with another message is told apart from its source.
    let directory = TemporaryDirectory::new("faithful-mutant");
    for target in LANGUAGES {
        let case = cases()
            .iter()
            .find(|case| case["language"] != target && !case["abort"].is_null())
            .expect("an aborting case from another language");
        let source = case["language"].as_str().expect("case language");
        let abort = case["abort"].as_str().expect("abort message");
        let translation = translate(&source_text(case), source, target);
        assert!(
            translation.code().contains(abort),
            "{target} states the abort message"
        );
        let mutant = translation.code().replace(abort, "a different message");
        let name = format!("mutant_{}", target.to_lowercase());
        let outcome = observe(target, &mutant, &directory.0, &name)
            .outcome
            .unwrap_or_else(|diagnostics| panic!("{target}: {diagnostics}"));
        assert_ne!(
            outcome,
            expected(case),
            "{target} observes the fault-injected abort"
        );
        assert_eq!(outcome.abort.as_deref(), Some("a different message"));
    }

    let pairs = LANGUAGES
        .into_iter()
        .flat_map(|source| {
            LANGUAGES
                .into_iter()
                .filter(move |target| *target != source)
                .map(move |target| format!("{source} -> {target}"))
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(translated, pairs);
    for (index, behavior) in BEHAVIORS.iter().enumerate() {
        assert_eq!(preserved[index], pairs, "{behavior} in every pair");
    }
    let fixture_id = format!(
        "planned:repository-directive:{}",
        REQUIREMENT_ID.to_lowercase()
    );
    observations::record(&observations::Observation {
        requirement_id: REQUIREMENT_ID,
        suffix: "behavior",
        fixture_id: &fixture_id,
        fixture_file: observations::FOUR_LANGUAGE_FIXTURE,
        assertions: &[
            "errorsPreserved",
            "abortsPreserved",
            "overflowPreserved",
            "effectsPreserved",
            "noPassThrough",
        ],
        test_name: "issue_195_faithful_behavior_holds_in_all_12_directed_translations",
    });
}
