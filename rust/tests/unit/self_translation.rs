//! Self-translation between JavaScript, TypeScript and Rust over the cases the
//! JavaScript runtime checks too (`parity/self-translation`): each case's
//! expected output and items, the way back to its source, and the Rust side
//! of its calls, compiled from the hand-written or translated Rust.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{External, ExternalParam};
use meta_language::translation::types::Type;
use meta_language::{
    DecoratorSet, SELF_TRANSLATION_LANGUAGES, SelfTranslation, SelfTranslationItem,
    SelfTranslationOptions, self_translate, self_translate_decorated, self_translate_with,
    self_translation_language, self_translation_signatures,
};

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/self-translation/cases.lino";
const HEADER: &str = "// meta-language:self-translation:v1 ";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn read(name: &str) -> String {
    let path = root().join("parity/self-translation").join(name);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn observe(requirement_id: &str, assertions: &[&str], test_name: &str) {
    let fixture_id = format!(
        "planned:repository-directive:{}",
        requirement_id.to_lowercase()
    );
    record(&Observation {
        requirement_id,
        suffix: "behavior",
        fixture_id: &fixture_id,
        fixture_file: FIXTURE,
        assertions,
        test_name,
    });
}

/// A link read as the words and links inside it.
enum Tree {
    Word(String),
    Link(Vec<Self>),
}

impl Tree {
    fn word(&self) -> &str {
        match self {
            Self::Word(word) => word,
            Self::Link(_) => panic!("a word, not a link"),
        }
    }

    fn values(&self) -> &[Self] {
        match self {
            Self::Link(values) => values,
            Self::Word(_) => panic!("a link, not a word"),
        }
    }

    /// The values after the head of the link `name` inside this one.
    fn field(&self, name: &str) -> &[Self] {
        self.optional_field(name)
            .unwrap_or_else(|| panic!("the link has a {name} field"))
    }

    /// [`Self::field`], or `None` where this link has no `name` field.
    fn optional_field(&self, name: &str) -> Option<&[Self]> {
        self.values().iter().find_map(|value| match value {
            Self::Link(values) if values.first().map(Self::word) == Some(name) => {
                Some(&values[1..])
            }
            _ => None,
        })
    }
}

fn tree(node: &LiNo<String>) -> Tree {
    match node {
        LiNo::Ref(word) => Tree::Word(word.clone()),
        LiNo::Link {
            id: Some(word),
            values,
        } if values.is_empty() => Tree::Word(word.clone()),
        LiNo::Link { values, .. } => Tree::Link(values.iter().map(tree).collect()),
    }
}

fn links(text: &str) -> Vec<Tree> {
    parse_lino_to_links_with_config(text, &ParserConfig::without_comments())
        .expect("the corpus reads as Links Notation")
        .iter()
        .map(tree)
        .collect()
}

struct Case {
    id: String,
    source: String,
    from: String,
    to: String,
    expected: String,
    decorators: Option<String>,
}

impl Case {
    /// The emitter decorators the case is translated with, or none.
    fn decorators(&self) -> DecoratorSet {
        self.decorators
            .as_ref()
            .map_or_else(DecoratorSet::default, |file| {
                DecoratorSet::from_lino(&read(file))
                    .unwrap_or_else(|error| panic!("{}: {error}", self.id))
            })
    }

    fn translate(&self) -> meta_language::SelfTranslation {
        self_translate_decorated(
            &read(&self.source),
            &self.from,
            &self.to,
            &self.decorators(),
        )
        .unwrap_or_else(|error| panic!("{}: {error}", self.id))
    }
}

/// A case whose decorated translation matches hand-written Rust.
struct HandWritten {
    case: String,
    rust: String,
    functions: Vec<String>,
}

struct Call {
    case: String,
    rust: String,
    arguments: Vec<(String, String)>,
    result: String,
}

fn corpus() -> (Vec<Case>, Vec<Call>) {
    let (cases, calls, _) = full_corpus();
    (cases, calls)
}

fn full_corpus() -> (Vec<Case>, Vec<Call>, Vec<HandWritten>) {
    let statements = links(&read("cases.lino"));
    let head = |statement: &Tree| statement.values()[0].word().to_owned();
    let first = |statement: &Tree, name: &str| statement.field(name)[0].word().to_owned();
    let cases = statements
        .iter()
        .filter(|statement| head(statement) == "case")
        .map(|statement| Case {
            id: statement.values()[1].word().to_owned(),
            source: first(statement, "source"),
            from: first(statement, "from"),
            to: first(statement, "to"),
            expected: first(statement, "expected"),
            decorators: statement
                .optional_field("decorators")
                .and_then(<[Tree]>::first)
                .map(|file| file.word().to_owned()),
        })
        .collect();
    let calls = statements
        .iter()
        .filter(|statement| head(statement) == "call")
        .map(|statement| Call {
            case: statement.values()[1].word().to_owned(),
            rust: first(statement, "rust"),
            arguments: statement
                .field("arguments")
                .iter()
                .map(|argument| {
                    let values = argument.values();
                    (values[0].word().to_owned(), values[1].word().to_owned())
                })
                .collect(),
            result: first(statement, "result"),
        })
        .collect();
    let hand_written = statements
        .iter()
        .filter(|statement| head(statement) == "hand-written")
        .map(|statement| HandWritten {
            case: statement.values()[1].word().to_owned(),
            rust: first(statement, "rust"),
            functions: statement
                .field("functions")
                .iter()
                .map(|name| name.word().to_owned())
                .collect(),
        })
        .collect();
    (cases, calls, hand_written)
}

/// The items of an expected `.items.lino` file.
fn expected_items(file: &str) -> Vec<SelfTranslationItem> {
    links(&read(file))
        .iter()
        .map(|item| {
            let words: Vec<&str> = item.values().iter().map(Tree::word).collect();
            let status = SELF_TRANSLATION_STATUSES
                .iter()
                .find(|status| **status == words[4])
                .expect("a known status");
            SelfTranslationItem {
                term: words[3].to_owned(),
                start: words[1].parse().expect("a start offset"),
                end: words[2].parse().expect("an end offset"),
                status,
                reason: words.get(5).map(|reason| (*reason).to_owned()),
            }
        })
        .collect()
}

const SELF_TRANSLATION_STATUSES: [&str; 6] = [
    "kept",
    "translated",
    "restored",
    "comment",
    "provenance",
    "carried",
];

#[test]
fn every_shared_case_translates_to_its_expected_output_and_items() {
    let (cases, _) = corpus();
    for case in &cases {
        let translation = case.translate();
        assert_eq!(translation.source_language, case.from);
        assert_eq!(translation.target_language, case.to);
        assert_eq!(translation.code, read(&case.expected), "{}", case.id);
        assert_eq!(
            translation.items,
            expected_items(&format!("{}.items.lino", case.expected)),
            "{}",
            case.id
        );
    }
    let name = "every_shared_case_translates_to_its_expected_output_and_items";
    observe(
        "I195-SELF-TRANSLATION-SHARED-CORPUS",
        &["expectedOutputsInLinksNotation", "checkedInBothRuntimes"],
        name,
    );
    assert!(
        cases
            .iter()
            .any(|case| case.from == "Rust" && case.to != "Rust")
    );
    assert!(
        cases
            .iter()
            .any(|case| case.from != "Rust" && case.to == "Rust")
    );
    observe(
        "I195-SELF-TRANSLATION-TOOL",
        &[
            "libraryApiInBothPackages",
            "javascriptToRust",
            "rustToJavaScript",
        ],
        name,
    );
}

#[test]
fn an_unedited_translation_translates_back_to_its_source_byte_for_byte() {
    let (cases, _) = corpus();
    for case in &cases {
        let source = read(&case.source);
        // A source that is itself an edited translation translates its edits again.
        if source.starts_with(HEADER) {
            continue;
        }
        let back = self_translate(&read(&case.expected), &case.to, &case.from)
            .unwrap_or_else(|error| panic!("{}: {error}", case.id));
        assert_eq!(back.code, source, "{}", case.id);
    }
    let edited = cases
        .iter()
        .find(|case| case.id == "arithmetic-edited-to-javascript")
        .expect("the edited case");
    let translation =
        self_translate(&read(&edited.source), &edited.from, &edited.to).expect("translates");
    let statuses: BTreeMap<&str, &str> = translation
        .items
        .iter()
        .map(|item| (item.status, item.term.as_str()))
        .collect();
    assert_eq!(statuses.get("translated"), Some(&"function_item"));
    assert_eq!(statuses.get("restored"), Some(&"line_comment"));
    observe(
        "I195-SELF-TRANSLATION-ROUND-TRIP",
        &["provenanceKeepsCommentsAndNames"],
        "an_unedited_translation_translates_back_to_its_source_byte_for_byte",
    );
}

/// The top-level functions of Rust `text` by name, whitespace normalized.
fn rust_functions(text: &str) -> BTreeMap<String, String> {
    let function = regex::Regex::new(r"(?m)^(?:pub )?fn ([a-z_0-9]+)[^\n]*\{\n(?:[^\n]*\n)*?\}$")
        .expect("the function pattern compiles");
    let space = regex::Regex::new(r"\s+").expect("the space pattern compiles");
    function
        .captures_iter(text)
        .map(|found| {
            (
                found[1].to_owned(),
                space.replace_all(&found[0], " ").into_owned(),
            )
        })
        .collect()
}

#[test]
fn decorators_bring_a_translation_to_the_hand_written_rust_and_still_restore_its_source() {
    let (cases, _, hand_written) = full_corpus();
    assert!(!hand_written.is_empty());
    for HandWritten {
        case: id,
        rust,
        functions,
    } in &hand_written
    {
        let case = cases
            .iter()
            .find(|candidate| &candidate.id == id)
            .expect("the hand-written case");
        let decorators = case.decorators();
        assert!(!decorators.decorators().is_empty(), "{id}");
        let expected = rust_functions(&read(rust));
        let decorated = rust_functions(&case.translate().code);
        let generic_code = self_translate(&read(&case.source), &case.from, &case.to)
            .expect("translates")
            .code;
        let generic = rust_functions(&generic_code);
        for name in functions {
            assert_eq!(decorated.get(name), expected.get(name), "{id} {name}");
            assert_ne!(generic.get(name), expected.get(name), "{id} {name}");
        }
        // Removing every decorator gives exactly the generic translation.
        let removed = decorators
            .ids()
            .iter()
            .fold(decorators.clone(), |set, decorator| {
                set.remove(decorator).expect("the decorator is removed")
            });
        assert_eq!(
            self_translate_decorated(&read(&case.source), &case.from, &case.to, &removed)
                .expect("translates")
                .code,
            generic_code
        );
        // The decorated code is the provenance, so the translation back restores the source.
        assert_eq!(
            self_translate(&read(&case.expected), &case.to, &case.from)
                .expect("translates back")
                .code,
            read(&case.source),
            "{id}"
        );
    }
    observe(
        "I195-SELF-TRANSLATION-SHARED-CORPUS",
        &["decoratorsMatchHandWritten"],
        "decorators_bring_a_translation_to_the_hand_written_rust_and_still_restore_its_source",
    );
}

#[test]
fn meta_languages_own_modules_round_trip_byte_for_byte() {
    let mut translated = 0;
    for (file, language, other) in [
        ("rust/src/binary_format.rs", "Rust", "TypeScript"),
        ("js/src/primitives.js", "JavaScript", "Rust"),
    ] {
        let source = fs::read_to_string(root().join(file)).expect("the module reads");
        let same = self_translate(&source, language, language).expect("translates");
        assert_eq!(same.code, source, "{file}");
        let there = self_translate(&source, language, other).expect("translates");
        translated += there
            .items
            .iter()
            .filter(|item| item.status == "translated")
            .count();
        let back = self_translate(&there.code, other, language).expect("translates back");
        assert_eq!(back.code, source, "{file}");
    }
    assert!(translated > 0, "some item of the modules is translated");
    observe(
        "I195-SELF-TRANSLATION-ROUND-TRIP",
        &["sameLanguageByteIdentical"],
        "meta_languages_own_modules_round_trip_byte_for_byte",
    );
}

/// The Rust expression a corpus argument names.
fn literal((kind, text): &(String, String)) -> String {
    match kind.as_str() {
        "str" => format!("String::from({text:?})"),
        "bool" => text.clone(),
        _ => format!("{text}{kind}"),
    }
}

/// Compiles `code` with a `main` that prints each of `lines` and runs it;
/// clippy denies every warning in translated Rust.
fn run(code: &str, lines: &[String], name: &str, translated: bool) -> Vec<String> {
    let directory = std::env::temp_dir().join(format!(
        "meta-language-self-translation-{}-{name}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).expect("scratch directory");
    let file = directory.join("main.rs");
    let binary = directory.join(format!("main{}", std::env::consts::EXE_SUFFIX));
    let prints = lines.iter().fold(String::new(), |mut prints, line| {
        let _ = writeln!(prints, "    println!(\"{{}}\", {line});");
        prints
    });
    fs::write(&file, format!("{code}\nfn main() {{\n{prints}}}\n")).expect("program writes");
    let mut compiler = Command::new(if translated { "clippy-driver" } else { "rustc" });
    compiler.args(["--edition", "2024"]);
    if translated {
        compiler.args(["-D", "warnings"]);
    } else {
        compiler.args(["-A", "warnings"]);
    }
    if cfg!(windows)
        && let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER")
    {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let built = compiler
        .arg("-o")
        .arg(&binary)
        .arg(&file)
        .output()
        .expect("the compiler runs");
    assert!(
        built.status.success(),
        "{name} compiles cleanly:\n{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let ran = Command::new(&binary).output().expect("the program runs");
    assert!(ran.status.success(), "{name} runs");
    fs::remove_dir_all(&directory).ok();
    String::from_utf8(ran.stdout)
        .expect("utf-8 output")
        .lines()
        .map(str::to_owned)
        .collect()
}

#[test]
fn the_rust_side_of_every_case_computes_the_shared_results() {
    let (cases, calls) = corpus();
    for case in &cases {
        let mine: Vec<&Call> = calls.iter().filter(|call| call.case == case.id).collect();
        if mine.is_empty() {
            continue;
        }
        let translated = case.from != "Rust";
        let code = read(if translated {
            &case.expected
        } else {
            &case.source
        });
        let lines: Vec<String> = mine
            .iter()
            .map(|call| {
                let arguments: Vec<String> = call.arguments.iter().map(literal).collect();
                format!("{}({})", call.rust, arguments.join(", "))
            })
            .collect();
        let results: Vec<String> = mine.iter().map(|call| call.result.clone()).collect();
        assert_eq!(
            run(&code, &lines, &case.id, translated),
            results,
            "{}",
            case.id
        );
    }
    let name = "the_rust_side_of_every_case_computes_the_shared_results";
    observe(
        "I195-SELF-TRANSLATION-ROUND-TRIP",
        &["crossLanguageBehaviorPreserved"],
        name,
    );
    observe(
        "I195-SELF-TRANSLATION-TOOL",
        &["generatedRustCompiles"],
        name,
    );
}

#[test]
fn the_translate_command_prints_the_translation_and_its_items() {
    let (cases, _) = corpus();
    let case = cases
        .iter()
        .find(|case| case.id == "geometry-to-javascript")
        .expect("the case");
    let source: &Path = &root().join("parity/self-translation").join(&case.source);
    let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .args(["translate", "--to", "javascript"])
        .arg(source)
        .output()
        .expect("the command runs");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        read(&case.expected)
    );
    let listed = Command::new(env!("CARGO_BIN_EXE_meta-language"))
        .args(["translate", "--to", "js", "--from", "rs", "--items"])
        .arg(source)
        .output()
        .expect("the command runs");
    let expected = expected_items(&format!("{}.items.lino", case.expected))
        .iter()
        .fold(String::new(), |mut expected, item| {
            let reason = item
                .reason
                .as_ref()
                .map(|reason| format!(" ({reason})"))
                .unwrap_or_default();
            let _ = writeln!(
                expected,
                "{}..{} {} {}{reason}",
                item.start, item.end, item.term, item.status
            );
            expected
        });
    assert_eq!(String::from_utf8_lossy(&listed.stdout), expected);
    observe(
        "I195-SELF-TRANSLATION-TOOL",
        &["cliInBothPackages"],
        "the_translate_command_prints_the_translation_and_its_items",
    );
}

#[test]
fn languages_are_named_by_name_or_extension_and_others_are_refused() {
    assert_eq!(
        SELF_TRANSLATION_LANGUAGES,
        ["JavaScript", "TypeScript", "Rust"]
    );
    assert_eq!(self_translation_language("rs"), Some("Rust"));
    assert_eq!(self_translation_language("TS"), Some("TypeScript"));
    assert_eq!(self_translation_language("mjs"), Some("JavaScript"));
    assert_eq!(self_translation_language("python"), None);
    assert!(self_translate("x\n", "python", "rust").is_err());
}

/// The Rust a translation emits, without its provenance lines.
fn emitted_rust(translation: &SelfTranslation) -> String {
    translation
        .code
        .split('\n')
        .filter(|line| !line.starts_with("// "))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The status of the item of `source` whose text contains `needle`.
fn status_of<'a>(translation: &'a SelfTranslation, source: &str, needle: &str) -> &'a str {
    translation
        .items
        .iter()
        .find(|item| source[item.start..item.end].contains(needle))
        .map_or("", |item| item.status)
}

#[test]
fn an_item_calls_the_other_top_level_items_of_its_module() {
    let source = "/** @param {number} x @returns {number} */\nfunction square(x) {\n  return x * x;\n}\n\n/** @param {number} x @returns {number} */\nexport function cappedSquare(x) {\n  return square(x) + 1;\n}\n";
    let translation = self_translate(source, "JavaScript", "Rust").expect("translates");
    assert!(
        translation
            .items
            .iter()
            .all(|item| item.status == "translated")
    );
    let rust = emitted_rust(&translation);
    assert!(
        rust.contains("pub fn capped_square(x: f64) -> f64 {\n    (square(x) + 1f64)\n}"),
        "{rust}"
    );
    // The translation back restores the source.
    let back = self_translate(&translation.code, "Rust", "JavaScript").expect("translates back");
    assert_eq!(back.code, source);
}

#[test]
fn a_sibling_that_does_not_translate_leaves_its_callers_carried() {
    let source = "/** @param {bigint} n @returns {boolean} */\nfunction isEven(n) {\n  return n === 0n ? true : isOdd(n - 1n);\n}\n\n/** @param {bigint} n @returns {boolean} */\nfunction isOdd(n) {\n  return n === 0n ? false : isEven(n - 1n);\n}\n\n/** @param {number} x @returns {number} */\nfunction opaque(x) {\n  return [x].map((y) => y)[0];\n}\n\n/** @param {number} x @returns {number} */\nfunction caller(x) {\n  return opaque(x) + 1;\n}\n";
    let translation = self_translate(source, "JavaScript", "Rust").expect("translates");
    // Mutually recursive siblings are a cycle, which stays unbound.
    for needle in [
        "function isEven",
        "function isOdd",
        "function opaque",
        "function caller",
    ] {
        assert_eq!(
            status_of(&translation, source, needle),
            "carried",
            "{needle}"
        );
    }
    let caller = translation
        .items
        .iter()
        .find(|item| source[item.start..].starts_with("function caller"))
        .expect("the caller is an item");
    assert_eq!(caller.reason.as_deref(), Some("type"));
}

#[test]
fn a_relative_import_of_items_of_the_crate_translates_as_a_use_declaration() {
    let math = "/** @param {number} x @returns {number} */\nexport function double(x) {\n  return x * 2;\n}\n\nfunction hidden(x) {\n  return x;\n}\n";
    let defaults = SelfTranslationOptions::default();
    let signatures = self_translation_signatures(math, "JavaScript", &defaults).expect("reads");
    assert_eq!(
        signatures,
        [External::Function {
            name: "double".to_owned(),
            params: vec![ExternalParam {
                name: "x".to_owned(),
                ty: Type::Float,
            }],
            ret: Type::Float,
        }]
    );
    let quad = "import { double } from './math.mjs';\n\n/** @param {number} x @returns {number} */\nexport function quadruple(x) {\n  return double(double(x));\n}\n";
    let options = SelfTranslationOptions {
        imports: [("./math.mjs".to_owned(), signatures)].into(),
        ..SelfTranslationOptions::default()
    };
    let bound = self_translate_with(quad, "JavaScript", "Rust", &options).expect("translates");
    assert!(bound.items.iter().all(|item| item.status == "translated"));
    let rust = emitted_rust(&bound);
    assert!(
        rust.contains("use crate::math::double;\n\npub fn quadruple(x: f64) -> f64 {\n    double(double(x))\n}"),
        "{rust}"
    );
    // Without the signatures the import still translates; its users are carried.
    let unbound = self_translate(quad, "JavaScript", "Rust").expect("translates");
    assert_eq!(status_of(&unbound, quad, "import {"), "translated");
    assert_eq!(status_of(&unbound, quad, "function quadruple"), "carried");
    // A renamed function is named in snake case on both sides of `as`.
    let renamed = SelfTranslationOptions {
        imports: [(
            "./math.mjs".to_owned(),
            vec![External::Function {
                name: "cappedSquare".to_owned(),
                params: Vec::new(),
                ret: Type::Float,
            }],
        )]
        .into(),
        ..SelfTranslationOptions::default()
    };
    let translation = self_translate_with(
        "import { cappedSquare as capped } from './math.mjs';\n",
        "JavaScript",
        "Rust",
        &renamed,
    )
    .expect("translates");
    assert!(emitted_rust(&translation).contains("use crate::math::capped_square as capped;\n"));
    // The module directory places the module inside the crate.
    let nested = SelfTranslationOptions {
        module_directory: vec!["agentic".to_owned(), "crate".to_owned()],
        ..SelfTranslationOptions::default()
    };
    let translation = self_translate_with(
        "import { realm } from '../host.mjs';\nimport { a, b } from './sub/part-two.js';\n",
        "JavaScript",
        "Rust",
        &nested,
    )
    .expect("translates");
    assert!(emitted_rust(&translation).contains(
        "use crate::agentic::host::realm;\nuse crate::agentic::crate_::sub::part_two::{a, b};"
    ));
}

#[test]
fn imports_outside_the_crate_are_refused_with_their_own_diagnostics() {
    for (source, message) in [
        (
            "import fs from 'node:fs';",
            "import from 'node:fs': Node.js built-in modules are outside the portable core at 0..15",
        ),
        (
            "import { readFileSync } from 'node:fs';",
            "import from 'node:fs': Node.js built-in modules are outside the portable core at 0..29",
        ),
        (
            "import { parse } from 'links-notation';",
            "import from 'links-notation': packages are outside the portable core; a module imports the items of the other modules of its crate by relative path, as in import { f } from './m.mjs' at 0..22",
        ),
        (
            "import * as m from './m.mjs';",
            "namespace import: import the items of a module by name, as in import { f } from './m.mjs', or the assertion module as a whole, as in import assert from 'node:assert/strict' at 0..7",
        ),
        (
            "import m from './m.mjs';",
            "default import from './m.mjs': a translated module has no default export; import its items by name, as in import { f } from './m.mjs' at 0..14",
        ),
        (
            "import {} from './m.mjs';",
            "import {} from './m.mjs': an import names the items it imports at 0..15",
        ),
    ] {
        let error = parse_javascript(source).expect_err("the import is refused");
        assert_eq!(error.message(), message);
        let translation =
            self_translate(&format!("{source}\n"), "JavaScript", "Rust").expect("translates");
        assert_eq!(translation.items[0].status, "carried", "{source}");
    }
    // A whole program is one module, so a relative import is refused there.
    let error = parse_javascript("import { f } from './m.mjs';").expect_err("refused");
    assert_eq!(
        error.message(),
        "import from './m.mjs': a relative import names another module of a crate, and self-translation translates a crate module by module at 0..18"
    );
    // The crate root has no parent directory.
    let climbing = self_translate("import { f } from '../m.mjs';\n", "JavaScript", "Rust")
        .expect("translates");
    assert_eq!(climbing.items[0].status, "carried");
    assert!(
        self_translation_signatures("fn main() {}\n", "Rust", &SelfTranslationOptions::default())
            .is_err()
    );
}
