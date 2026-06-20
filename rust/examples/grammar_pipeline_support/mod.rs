#![allow(dead_code)]

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use meta_language::{
    infer_cfg, validate, Grammar, GrammarFormat, GrammarParser, InferenceOptions,
    PositiveOnlyOracle, RustParserArtifacts,
};

#[allow(dead_code)]
#[derive(pest_derive::Parser)]
#[grammar_inline = "probe = { \"\" }"]
struct PestDependencyProbe;

pub const ARITH_EXAMPLES: &[&str] = &["1+2", "3*4", "1+2*3", "(1+2)*3", "7"];
pub const CSV_ROW_EXAMPLES: &[&str] = &["a,b,c", "x", "1,2", ",,"];
pub const JSONISH_EXAMPLES: &[&str] = &["{}", "{\"a\":1}", "[1,2]", "true"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExternalRun {
    Ran,
    Skipped(String),
}

pub fn infer_valid_grammar(examples: &[&str]) -> Grammar {
    let examples = examples
        .iter()
        .map(|example| (*example).to_string())
        .collect::<Vec<_>>();
    infer_valid_grammar_from_strings(&examples)
}

pub fn infer_valid_grammar_from_strings(examples: &[String]) -> Grammar {
    let result = infer_cfg(
        examples,
        &PositiveOnlyOracle::new(),
        InferenceOptions::default(),
    );
    assert_eq!(
        result.grammar.source_format(),
        Some(GrammarFormat::Inferred)
    );
    assert!(
        !result.grammar.rules().is_empty(),
        "inference emitted no rules"
    );
    assert_no_error_diagnostics(&result.grammar);
    assert_runtime_accepts_all_strings(&result.grammar, examples);
    result.grammar
}

pub fn assert_no_error_diagnostics(grammar: &Grammar) {
    let diagnostics = validate(grammar);
    let errors = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.is_error())
        .collect::<Vec<_>>();
    assert!(errors.is_empty(), "validation errors: {errors:#?}");
}

pub fn assert_runtime_accepts_all(grammar: &Grammar, examples: &[&str]) {
    let examples = examples
        .iter()
        .map(|example| (*example).to_string())
        .collect::<Vec<_>>();
    assert_runtime_accepts_all_strings(grammar, &examples);
}

pub fn assert_runtime_accepts_all_strings(grammar: &Grammar, examples: &[String]) {
    let parser = GrammarParser::new(grammar.clone());
    assert!(
        parser.diagnostics().is_empty(),
        "runtime parser diagnostics: {:#?}",
        parser.diagnostics()
    );
    for example in examples {
        assert!(parser.accepts(example), "grammar rejected {example:?}");
    }
}

pub fn assert_runtime_rejects(grammar: &Grammar, negative: &str) {
    let parser = GrammarParser::new(grammar.clone());
    assert!(
        !parser.accepts(negative),
        "grammar unexpectedly accepted {negative:?}"
    );
}

pub fn compile_and_run_rust_parser(
    artifacts: &RustParserArtifacts,
    start_rule: &str,
    examples: &[String],
    negative: Option<&str>,
) -> Result<(), String> {
    let parser_name = parser_struct_name(&artifacts.parser_struct)?;
    let temp_dir = unique_temp_path("generated-rust-parser");
    fs::create_dir_all(&temp_dir)
        .map_err(|error| format!("failed to create {}: {error}", temp_dir.display()))?;
    let source_path = temp_dir.join("main.rs");
    let binary_path = temp_dir.join(format!("generated-parser{}", std::env::consts::EXE_SUFFIX));
    fs::write(
        &source_path,
        rust_driver_source(artifacts, &parser_name, start_rule, examples, negative),
    )
    .map_err(|error| format!("failed to write {}: {error}", source_path.display()))?;

    let deps_dir = target_deps_dir()?;
    let pest = find_dependency_artifact(&deps_dir, &["libpest-", "pest-"], &["rlib"])?;
    let pest_derive = find_dependency_artifact(
        &deps_dir,
        &["libpest_derive-", "pest_derive-"],
        &[std::env::consts::DLL_EXTENSION],
    )?;
    let mut rustc = Command::new("rustc");
    rustc
        .arg("--edition=2021")
        .arg(&source_path)
        .arg("-L")
        .arg(format!("dependency={}", deps_dir.display()))
        .arg("--extern")
        .arg(format!("pest={}", pest.display()))
        .arg("--extern")
        .arg(format!("pest_derive={}", pest_derive.display()));
    if let Some(linker) = rustc_linker_override() {
        rustc.arg("-C").arg(format!("linker={linker}"));
    }
    let compile = rustc
        .arg("-o")
        .arg(&binary_path)
        .output()
        .map_err(|error| format!("failed to run rustc: {error}"))?;
    ensure_success("rustc generated parser", &compile)?;

    let run = Command::new(&binary_path)
        .output()
        .map_err(|error| format!("failed to run {}: {error}", binary_path.display()))?;
    ensure_success("generated parser", &run)
}

pub fn run_javascript_parser(
    module: &str,
    examples: &[String],
    negative: Option<&str>,
) -> Result<ExternalRun, String> {
    let node = Command::new("node").arg("--version").output();
    if node.is_err() {
        return Ok(ExternalRun::Skipped(
            "`node` was not found on PATH".to_string(),
        ));
    }

    let peggy = Command::new("node")
        .arg("-e")
        .arg("import('peggy').then(() => {}).catch(() => process.exit(42));")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .map_err(|error| format!("failed to check peggy availability: {error}"))?;
    if !peggy.status.success() {
        return Ok(ExternalRun::Skipped(
            "Node package `peggy` is not available".to_string(),
        ));
    }

    let temp_dir = unique_temp_path("generated-js-parser");
    fs::create_dir_all(&temp_dir)
        .map_err(|error| format!("failed to create {}: {error}", temp_dir.display()))?;
    let script_path = temp_dir.join("parser-check.mjs");
    fs::write(
        &script_path,
        javascript_driver_source(module, examples, negative),
    )
    .map_err(|error| format!("failed to write {}: {error}", script_path.display()))?;

    let output = Command::new("node")
        .arg(&script_path)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .map_err(|error| format!("failed to run node parser check: {error}"))?;
    ensure_success("generated JavaScript parser", &output)?;
    Ok(ExternalRun::Ran)
}

pub fn load_fixture_corpus(name: &str) -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/grammar/corpora")
        .join(format!("{name}.txt"));
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    text.lines().map(str::to_string).collect()
}

pub fn write_example_directory(stem: &str, examples: &[String]) -> PathBuf {
    let dir = unique_temp_path(stem);
    fs::create_dir_all(&dir).unwrap_or_else(|error| {
        panic!("failed to create {}: {error}", dir.display());
    });
    for (index, example) in examples.iter().enumerate() {
        let path = dir.join(format!("example-{index}.txt"));
        fs::write(&path, example).unwrap_or_else(|error| {
            panic!("failed to write {}: {error}", path.display());
        });
    }
    dir
}

pub fn unique_temp_path(stem: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "meta-language-{stem}-{}-{nanos}",
        std::process::id()
    ))
}

fn parser_struct_name(parser_struct: &str) -> Result<String, String> {
    parser_struct
        .lines()
        .find_map(|line| {
            line.strip_prefix("pub struct ")
                .and_then(|name| name.strip_suffix(';'))
                .map(str::to_string)
        })
        .ok_or_else(|| "generated Rust parser struct was not found".to_string())
}

fn rust_driver_source(
    artifacts: &RustParserArtifacts,
    parser_name: &str,
    start_rule: &str,
    examples: &[String],
    negative: Option<&str>,
) -> String {
    let samples = examples
        .iter()
        .map(|example| format!("{example:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let negative_check = negative.map_or_else(String::new, |negative| {
        format!(
            "    if {parser_name}::parse(Rule::{start_rule}, {negative:?}).is_ok() {{\n        panic!(\"unexpectedly parsed negative sample\");\n    }}\n"
        )
    });

    format!(
        "use pest::Parser as _;\n\n{}\n{}\nfn main() {{\n    let samples: &[&str] = &[{samples}];\n    for sample in samples {{\n        {parser_name}::parse(Rule::{start_rule}, sample).unwrap_or_else(|error| {{\n            panic!(\"failed to parse {{sample:?}}: {{error}}\");\n        }});\n    }}\n{negative_check}}}\n",
        artifacts.parser_struct, artifacts.ast_types
    )
}

fn javascript_driver_source(module: &str, examples: &[String], negative: Option<&str>) -> String {
    let samples = examples
        .iter()
        .map(|example| format!("{example:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let negative_check = negative.map_or_else(String::new, |negative| {
        format!(
            "\ntry {{\n  parser.parse({negative:?});\n  console.error('unexpectedly parsed negative sample');\n  process.exit(2);\n}} catch (_error) {{}}\n"
        )
    });
    format!(
        "{module}\nconst samples = [{samples}];\nfor (const sample of samples) {{\n  parser.parse(sample);\n}}\n{negative_check}"
    )
}

fn target_deps_dir() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|error| format!("current_exe failed: {error}"))?;
    let parent = exe
        .parent()
        .ok_or_else(|| format!("{} has no parent directory", exe.display()))?;

    if parent.file_name() == Some(OsStr::new("deps")) {
        return Ok(parent.to_path_buf());
    }
    if parent.file_name() == Some(OsStr::new("examples")) {
        let debug_dir = parent
            .parent()
            .ok_or_else(|| format!("{} has no debug parent", parent.display()))?;
        return Ok(debug_dir.join("deps"));
    }

    Ok(Path::new(env!("CARGO_MANIFEST_DIR")).join("target/debug/deps"))
}

#[allow(clippy::missing_const_for_fn)]
fn rustc_linker_override() -> Option<String> {
    #[cfg(all(windows, target_env = "msvc"))]
    {
        std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER")
            .ok()
            .filter(|linker| !linker.trim().is_empty())
    }

    #[cfg(not(all(windows, target_env = "msvc")))]
    {
        None
    }
}

fn find_dependency_artifact(
    deps_dir: &Path,
    prefixes: &[&str],
    extensions: &[&str],
) -> Result<PathBuf, String> {
    let mut candidates = fs::read_dir(deps_dir)
        .map_err(|error| format!("failed to read {}: {error}", deps_dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            let Some(name) = path.file_name().and_then(OsStr::to_str) else {
                return false;
            };
            let Some(extension) = path.extension().and_then(OsStr::to_str) else {
                return false;
            };
            prefixes.iter().any(|prefix| name.starts_with(prefix))
                && extensions.contains(&extension)
        })
        .collect::<Vec<_>>();
    candidates.sort();
    candidates.into_iter().next().ok_or_else(|| {
        format!(
            "no dependency artifact with prefixes {prefixes:?} and extensions {extensions:?} in {}",
            deps_dir.display()
        )
    })
}

fn ensure_success(label: &str, output: &std::process::Output) -> Result<(), String> {
    if output.status.success() {
        return Ok(());
    }
    Err(format!(
        "{label} failed with status {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ))
}
