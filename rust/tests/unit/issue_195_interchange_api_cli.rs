//! Issue #195 grammar interchange API and command-line tool: every shared
//! command of `parity/fixtures/grammar-importers.json` runs through
//! `run_grammar_command` and through the `meta-language grammar` binary in a
//! directory holding its files, and both must give the recorded exit code,
//! standard output and standard error (the JavaScript tool gave the same when
//! the fixture was written). The successful outputs are then read back
//! independently: listings, conversions, exports, merges and renames are
//! re-imported in their own format and must accept and reject the samples of
//! the shared cases they came from. Mirrors
//! `js/tests/issue-195-interchange-api-cli.test.js`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use meta_language::{
    GRAMMAR_COMMAND_USAGE, GRAMMAR_EXPORT_FORMATS, GRAMMAR_IMPORT_FORMATS, Grammar,
    GrammarCommandOutput, GrammarParser, grammar_importer, parse_native_grammar,
    render_native_grammar, run_grammar_command,
};
use serde_json::Value;

use super::issue_195_observations as observations;

const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-importers.json");
const REQUIREMENT_ID: &str = "I195-INTERCHANGE-API-CLI";

fn record(assertion: &str, test_name: &str) {
    let fixture_id = format!(
        "planned:repository-directive:{}",
        REQUIREMENT_ID.to_lowercase()
    );
    observations::record(&observations::Observation {
        requirement_id: REQUIREMENT_ID,
        suffix: "behavior",
        fixture_id: &fixture_id,
        fixture_file: observations::GRAMMAR_IMPORTER_FIXTURE,
        assertions: &[assertion],
        test_name,
    });
}

fn corpus() -> Value {
    serde_json::from_str(FIXTURE).expect("shared importer fixture is JSON")
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("has {key}"))
}

fn strings(value: &Value, key: &str) -> Vec<String> {
    value[key]
        .as_array()
        .unwrap_or_else(|| panic!("has {key}"))
        .iter()
        .map(|item| item.as_str().expect("string entry").to_owned())
        .collect()
}

fn shared_case<'a>(corpus: &'a Value, id: &str) -> &'a Value {
    corpus["cases"]
        .as_array()
        .expect("cases array")
        .iter()
        .find(|entry| entry["id"] == id)
        .unwrap_or_else(|| panic!("no shared case {id}"))
}

fn command<'a>(corpus: &'a Value, id: &str) -> &'a Value {
    commands(corpus, "")
        .into_iter()
        .find(|entry| entry["id"] == id)
        .unwrap_or_else(|| panic!("no shared command {id}"))
}

fn commands<'a>(corpus: &'a Value, prefix: &str) -> Vec<&'a Value> {
    corpus["commands"]
        .as_array()
        .expect("commands array")
        .iter()
        .filter(|entry| text(entry, "id").starts_with(prefix))
        .collect()
}

/// The shared case the first file of a command comes from.
fn case_of(entry: &Value) -> &str {
    entry["files"]
        .as_object()
        .and_then(|files| files.values().next())
        .and_then(|file| file["case"].as_str())
        .expect("a file from a shared case")
}

fn file_contents(corpus: &Value, entry: &Value) -> BTreeMap<String, String> {
    entry["files"]
        .as_object()
        .expect("files object")
        .iter()
        .map(|(name, value)| {
            let contents = match (value.as_str(), value["case"].as_str()) {
                (Some(literal), _) => literal,
                (None, Some(case)) => text(shared_case(corpus, case), "source"),
                (None, None) => {
                    let index = value["malformed"].as_u64().expect("malformed index");
                    text(
                        &corpus["malformed"][usize::try_from(index).expect("small index")],
                        "source",
                    )
                }
            };
            (name.clone(), contents.to_owned())
        })
        .collect()
}

fn args(entry: &Value) -> Vec<String> {
    strings(entry, "args")
}

fn check_recorded(entry: &Value, actual: &GrammarCommandOutput, runtime: &str) {
    let id = text(entry, "id");
    assert_eq!(
        i64::from(actual.exit_code),
        entry["exitCode"].as_i64().expect("exit code"),
        "{id} ({runtime}) exit code\n{}",
        actual.stderr
    );
    assert_eq!(
        actual.stdout,
        text(entry, "stdout"),
        "{id} ({runtime}) standard output"
    );
    if let Some(head) = entry["stderrHead"].as_str() {
        assert_eq!(
            actual.stderr.split('\n').next(),
            Some(head),
            "{id} ({runtime}) error"
        );
    } else {
        assert_eq!(
            actual.stderr,
            text(entry, "stderr"),
            "{id} ({runtime}) standard error"
        );
    }
}

fn scratch_directory() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let directory = std::env::temp_dir().join(format!(
        "meta-language-grammar-cli-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&directory).expect("scratch directory");
    directory
}

/// Runs a shared command through the library and the binary, checks both
/// against the record and returns the library result.
fn run(corpus: &Value, entry: &Value) -> GrammarCommandOutput {
    let files = file_contents(corpus, entry);
    let args = args(entry);
    let library = run_grammar_command(&args, &|name| {
        files
            .get(name)
            .cloned()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
    });
    check_recorded(entry, &library, "library");
    let directory = scratch_directory();
    for (name, contents) in &files {
        let path = directory.join(name);
        fs::create_dir_all(path.parent().expect("file parent")).expect("file directory");
        fs::write(path, contents).expect("file written");
    }
    let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .arg("grammar")
        .args(&args)
        .current_dir(&directory)
        .output()
        .expect("binary runs");
    let binary = GrammarCommandOutput {
        exit_code: output.status.code().expect("exit code"),
        stdout: String::from_utf8(output.stdout).expect("UTF-8 output"),
        stderr: String::from_utf8(output.stderr).expect("UTF-8 error"),
    };
    fs::remove_dir_all(&directory).expect("scratch directory removed");
    check_recorded(entry, &binary, "binary");
    library
}

fn option_value<'a>(args: &'a [String], name: &str) -> &'a str {
    let index = args.iter().position(|arg| arg == name).expect("option");
    &args[index + 1]
}

fn import(format: &str, source: &str) -> Grammar {
    grammar_importer(format).expect("known format")(source)
        .unwrap_or_else(|error| panic!("{format} output re-imports: {error}"))
}

fn accepts(grammar: &Grammar, text: &str) -> bool {
    GrammarParser::new(grammar.clone()).accepts(text)
}

/// The grammar accepts and rejects the samples of the shared case `case_id`.
fn assert_samples(corpus: &Value, grammar: &Grammar, case_id: &str, label: &str) {
    let case = shared_case(corpus, case_id);
    for sample in strings(case, "accepts") {
        assert!(accepts(grammar, &sample), "{label} accepts {sample:?}");
    }
    for sample in strings(case, "rejects") {
        assert!(!accepts(grammar, &sample), "{label} rejects {sample:?}");
    }
}

#[test]
fn issue_195_every_shared_command_gives_the_recorded_result_through_the_library_and_the_binary() {
    let corpus = corpus();
    let all = commands(&corpus, "");
    assert_eq!(all.len(), 75);
    let ids = all
        .iter()
        .map(|entry| text(entry, "id"))
        .collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), all.len());
    for entry in &all {
        run(&corpus, entry);
    }
    assert_eq!(
        text(command(&corpus, "help"), "stdout"),
        GRAMMAR_COMMAND_USAGE
    );
    assert_eq!(
        text(command(&corpus, "usage-without-command"), "stderr"),
        GRAMMAR_COMMAND_USAGE
    );
    assert_eq!(
        GRAMMAR_IMPORT_FORMATS,
        [
            "abnf",
            "antlr",
            "bnf",
            "ebnf",
            "gbnf",
            "lark",
            "native",
            "pest",
            "tree-sitter-json"
        ]
    );
    assert_eq!(GRAMMAR_EXPORT_FORMATS, GRAMMAR_IMPORT_FORMATS);
    assert_eq!(
        text(command(&corpus, "formats"), "stdout"),
        format!(
            "import: {}\nexport: {}\n",
            GRAMMAR_IMPORT_FORMATS.join(" "),
            GRAMMAR_EXPORT_FORMATS.join(" ")
        )
    );
    for entry in commands(&corpus, "error:") {
        let id = text(entry, "id");
        assert_eq!(entry["exitCode"], 2, "{id}");
        assert_eq!(text(entry, "stdout"), "", "{id}");
        let error = entry["stderr"]
            .as_str()
            .or_else(|| entry["stderrHead"].as_str())
            .expect("error text");
        assert!(error.starts_with("error: "), "{id}");
    }
}

#[test]
fn issue_195_grammar_import_lists_every_shared_case_as_a_native_grammar() {
    let corpus = corpus();
    let imports = commands(&corpus, "import:");
    assert_eq!(
        imports.len(),
        corpus["cases"].as_array().expect("cases").len()
    );
    for entry in imports {
        let id = text(entry, "id");
        let case = shared_case(&corpus, case_of(entry));
        let file = entry["files"]
            .as_object()
            .and_then(|files| files.keys().next())
            .expect("one file");
        assert_eq!(
            args(entry),
            ["import", "--from", text(case, "format"), file]
        );
        let listing = parse_native_grammar(&run(&corpus, entry).stdout)
            .unwrap_or_else(|error| panic!("{id} listing parses: {error}"));
        assert_eq!(
            listing.start_rule().map(|rule| rule.name.as_str()),
            Some(text(case, "start")),
            "{id}"
        );
        for rule in strings(case, "rules") {
            assert!(listing.rule(&rule).is_some(), "{id} lists {rule}");
        }
        assert_eq!(
            render_native_grammar(&listing),
            text(entry, "stdout"),
            "{id} listing is a fixed point"
        );
        assert_samples(&corpus, &listing, text(case, "id"), id);
    }
    let mut rejected = commands(&corpus, "error:malformed:");
    rejected.extend(commands(&corpus, "error:native:"));
    assert_eq!(
        rejected.len(),
        corpus["malformed"].as_array().expect("malformed").len() + 11
    );
    for entry in rejected {
        let args = args(entry);
        let error = entry["stderr"]
            .as_str()
            .or_else(|| entry["stderrHead"].as_str())
            .expect("error text");
        assert!(
            error.starts_with(&format!(
                "error: cannot import {} as {} (",
                args[3], args[2]
            )),
            "{}",
            text(entry, "id")
        );
    }
    record(
        "importCommand",
        "issue_195_grammar_import_lists_every_shared_case_as_a_native_grammar",
    );
}

#[test]
fn issue_195_grammar_validate_reports_every_diagnostic_of_a_broken_grammar() {
    let corpus = corpus();
    assert_eq!(
        run(&corpus, command(&corpus, "validate:abnf:message")).stdout,
        "0 error(s), 0 warning(s)\n"
    );
    let problems = command(&corpus, "validate:problems");
    let output = run(&corpus, problems);
    assert_eq!(output.exit_code, 1);
    let lines = output.stdout.trim_end().split('\n').collect::<Vec<_>>();
    let (summary, diagnostics) = lines.split_last().expect("summary line");
    let kinds = [
        "duplicate-rule",
        "undefined-non-terminal",
        "left-recursion",
        "unreachable-rule",
        "nullable-repetition",
        "unused-capture",
    ];
    let mut found = diagnostics
        .iter()
        .map(|line| {
            let (head, _) = line.split_once(": ").expect("diagnostic line");
            let parts = head.split(' ').collect::<Vec<_>>();
            assert_eq!(parts.len(), 3, "{line}");
            assert!(["error", "warning"].contains(&parts[0]), "{line}");
            assert!(kinds.contains(&parts[1]), "{line}");
            head.to_owned()
        })
        .collect::<Vec<_>>();
    found.sort();
    // Read off problems.grammar by hand: `expr` and `a -> b -> a` recurse on
    // the left, `missing` is undefined, `repeat0(optional(...))` and `b` can
    // match nothing, `lbl` is never used, and `orphan`, `a` and `b` are not
    // reachable from `expr`.
    assert_eq!(
        found,
        [
            "error left-recursion a",
            "error left-recursion expr",
            "error undefined-non-terminal term",
            "warning nullable-repetition b",
            "warning nullable-repetition term",
            "warning unreachable-rule a",
            "warning unreachable-rule b",
            "warning unreachable-rule orphan",
            "warning unused-capture term",
        ]
    );
    let errors = found
        .iter()
        .filter(|line| line.starts_with("error "))
        .count();
    assert_eq!(
        *summary,
        format!("{errors} error(s), {} warning(s)", found.len() - errors)
    );
    record(
        "validateCommand",
        "issue_195_grammar_validate_reports_every_diagnostic_of_a_broken_grammar",
    );
}

#[test]
fn issue_195_grammar_convert_writes_every_target_format_so_that_it_re_imports() {
    let corpus = corpus();
    let converts = commands(&corpus, "convert:");
    let mut targets = BTreeSet::new();
    for entry in converts {
        let id = text(entry, "id");
        let output = run(&corpus, entry);
        let args = args(entry);
        let to = option_value(&args, "--to");
        if output.exit_code != 0 {
            assert_eq!(output.exit_code, 2, "{id}");
            assert_eq!(output.stdout, "", "{id}");
            assert!(
                output.stderr.starts_with(&format!(
                    "error: cannot export as {to}\n{to} emit unsupported construct: "
                )),
                "{id}"
            );
            continue;
        }
        targets.insert(to.to_owned());
        for note in output.stderr.lines() {
            assert!(note.starts_with("lossy: "), "{id}: {note}");
        }
        assert_samples(&corpus, &import(to, &output.stdout), case_of(entry), id);
    }
    assert_eq!(
        targets,
        ["antlr", "ebnf", "gbnf", "lark", "pest", "tree-sitter-json"]
            .map(str::to_owned)
            .into()
    );
    record(
        "convertCommand",
        "issue_195_grammar_convert_writes_every_target_format_so_that_it_re_imports",
    );
}

#[test]
fn issue_195_grammar_export_writes_a_native_listing_in_another_format() {
    let corpus = corpus();
    let to_abnf = command(&corpus, "export:native-to-abnf");
    let abnf = import("abnf", &run(&corpus, to_abnf).stdout);
    for sample in ["1", "1+2", "9+0+5"] {
        assert!(accepts(&abnf, sample), "{sample}");
    }
    for sample in ["", "+", "1+", "1+x", "12"] {
        assert!(!accepts(&abnf, sample), "{sample}");
    }
    let to_native = command(&corpus, "export:native-to-native");
    let native = run(&corpus, to_native).stdout;
    let listing = text(&to_native["files"], "sum.grammar");
    let (comment, rest) = listing.split_once('\n').expect("listing lines");
    assert!(comment.starts_with("# "));
    assert_eq!(native, rest);
    assert_eq!(
        render_native_grammar(&parse_native_grammar(&native).expect("listing parses")),
        native
    );
    record(
        "exportCommand",
        "issue_195_grammar_export_writes_a_native_listing_in_another_format",
    );
}

#[test]
fn issue_195_grammar_merge_writes_one_grammar_and_fails_on_an_unproven_equivalence() {
    let corpus = corpus();
    let merged = command(&corpus, "merge:bnf-and-ebnf");
    let output = run(&corpus, merged);
    let grammar = import("ebnf", &output.stdout);
    assert_samples(&corpus, &grammar, "bnf:message", "merge");
    assert_samples(&corpus, &grammar, "ebnf:message", "merge");
    // The equal `letter` and `digit` rules of both sources become one rule each.
    assert_eq!(
        output
            .stdout
            .lines()
            .filter(|line| line.starts_with("letter = "))
            .count(),
        1
    );
    for rule in ["letter", "digit"] {
        assert!(
            output.stderr.lines().any(|line| line.starts_with(&format!(
                "merged {rule}: message.bnf:{rule} message.ebnf:{rule} ("
            ))),
            "{rule}"
        );
    }
    let unresolved = command(&corpus, "merge:required-equivalence-unresolved");
    let output = run(&corpus, unresolved);
    assert_eq!(output.exit_code, 1);
    assert_eq!(
        output.stderr.trim_end().lines().last(),
        Some("unresolved message.abnf:message = message.pest:word: not-proven")
    );
    let listing = parse_native_grammar(&output.stdout).expect("listing parses");
    assert_samples(&corpus, &listing, "abnf:message", "unresolved merge");
    record(
        "mergeCommand",
        "issue_195_grammar_merge_writes_one_grammar_and_fails_on_an_unproven_equivalence",
    );
}

#[test]
fn issue_195_grammar_rename_renames_a_rule_with_its_references() {
    let corpus = corpus();
    let output = run(&corpus, command(&corpus, "rename:referenced-rule"));
    let grammar = import("ebnf", &output.stdout);
    assert!(grammar.rule("name").is_some());
    assert!(grammar.rule("word").is_none());
    assert!(!output.stdout.contains("word"));
    assert_eq!(output.stderr, "alias name = word\n");
    assert_samples(&corpus, &grammar, "ebnf:message", "rename");
    assert_eq!(
        run(&corpus, command(&corpus, "rename:unknown-rule")).stderr,
        "error: grammar has no rule nothing\n"
    );
    record(
        "renameCommand",
        "issue_195_grammar_rename_renames_a_rule_with_its_references",
    );
}

#[test]
fn issue_195_grammar_round_trip_preserves_every_shared_case() {
    let corpus = corpus();
    let trips = commands(&corpus, "round-trip:")
        .into_iter()
        .filter(|entry| entry["id"] != "round-trip:native-wrong-samples")
        .collect::<Vec<_>>();
    assert_eq!(
        trips.len(),
        corpus["cases"].as_array().expect("cases").len()
    );
    for entry in trips {
        let id = text(entry, "id");
        let case = shared_case(&corpus, case_of(entry));
        let args = args(entry);
        let samples = |option: &str| {
            args.windows(2)
                .filter(|pair| pair[0] == option)
                .map(|pair| pair[1].clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(samples("--accept"), strings(case, "accepts"), "{id}");
        assert_eq!(samples("--reject"), strings(case, "rejects"), "{id}");
        assert_eq!(run(&corpus, entry).stdout, "preserved\n", "{id}");
    }
    let output = run(&corpus, command(&corpus, "round-trip:native-wrong-samples"));
    assert_eq!(output.exit_code, 1);
    assert_eq!(
        output.stdout.lines().collect::<Vec<_>>(),
        [
            "broken",
            "sample-rejected imported: 1+x",
            "sample-accepted imported: 1+2",
            "sample-rejected reimported: 1+x",
            "sample-accepted reimported: 1+2",
        ]
    );
    record(
        "roundTripCommand",
        "issue_195_grammar_round_trip_preserves_every_shared_case",
    );
}
