//! Grammar interchange commands shared by the Rust and JavaScript tools.
//!
//! The commands import, validate, convert, merge, rename, export and round
//! trip grammars. [`run_grammar_command`] is the whole command-line behavior
//! with file reading injected, so the library and the `meta-language` binary
//! report the same standard output, standard error and exit status. It mirrors
//! `js/src/grammar-interchange.js`, and the native grammar listing read by
//! [`parse_native_grammar`] and written by [`render_native_grammar`] mirrors
//! the same file.

use std::collections::{BTreeMap, BTreeSet};

use super::Grammar;
use super::emit::{
    EmitReport, GrammarEmitError, emit_abnf, emit_antlr, emit_bnf, emit_ebnf, emit_gbnf, emit_lark,
    emit_pest, emit_tree_sitter_json,
};
use super::import::{
    GrammarImportError, import_abnf, import_antlr, import_bnf, import_ebnf, import_gbnf,
    import_lark, import_pest, import_tree_sitter_json,
};
use super::merge::{GrammarMergeOptions, GrammarMergeSource, merge_grammars, rename_grammar_rule};
use super::round_trip::{GrammarRoundTrip, GrammarRoundTripError, check_grammar_round_trip};
use super::validate::{Severity, validate};

mod feature_forms;
mod feature_links;
mod json;
mod links;
mod lossless;
mod lowering;
mod native;

pub use feature_links::{render_declaration_links, render_rule_fields};
pub use json::{deserialize_grammar, serialize_grammar};
pub use links::{
    parse_grammar_links, parse_links_expression, percent_decode_links_text,
    percent_encode_links_text, render_grammar_links, render_links_expression, render_rule_link,
};
pub use lossless::{
    GRAMMAR_LOSSLESS_FORMATS, GrammarLayout, GrammarLayoutDefinition, GrammarLayoutImplicit,
    GrammarLosslessError, GrammarSourceDefinition, GrammarSourceSplit, capture_grammar_layout,
    emit_grammar_lossless, import_grammar_lossless, parse_grammar_layout_links,
    render_grammar_layout_links, split_grammar_source,
};
pub use lowering::{
    GRAMMAR_LOWERING_FORMATS, GrammarLowering, GrammarLoweringEncoding, GrammarLoweringError,
    GrammarLoweringFailure, GrammarLoweringFailureKind, GrammarLoweringMetadata,
    GrammarLoweringOptions, GrammarLoweringReport, GrammarLoweringStatus, GrammarLoweringStep,
    MAX_LOWERED_CHARACTERS, check_grammar_lowering, dropped_grammar_features, lower_grammar,
    parse_lowering_metadata, reconstruct_grammar, render_lowering_metadata,
};
pub use native::{
    parse_native_expression, parse_native_grammar, render_native_expression, render_native_feature,
    render_native_grammar,
};

/// Imports a grammar from one notation.
pub type GrammarImporter = fn(&str) -> Result<Grammar, GrammarImportError>;

/// Emits a grammar in one notation, with its fidelity report.
pub type GrammarEmitter = fn(&Grammar) -> Result<(String, EmitReport), GrammarEmitError>;

/// The formats [`grammar_importer`] and the `--from` options read.
pub const GRAMMAR_IMPORT_FORMATS: &[&str] = &[
    "abnf",
    "antlr",
    "bnf",
    "ebnf",
    "gbnf",
    "lark",
    "native",
    "pest",
    "tree-sitter-json",
];

/// The formats [`grammar_emitter`] and the `--to` options write.
pub const GRAMMAR_EXPORT_FORMATS: &[&str] = &[
    "abnf",
    "antlr",
    "bnf",
    "ebnf",
    "gbnf",
    "lark",
    "native",
    "pest",
    "tree-sitter-json",
];

/// The importer of one of [`GRAMMAR_IMPORT_FORMATS`].
#[must_use]
pub fn grammar_importer(format: &str) -> Option<GrammarImporter> {
    let importer: GrammarImporter = match format {
        "abnf" => import_abnf,
        "antlr" => import_antlr,
        "bnf" => import_bnf,
        "ebnf" => import_ebnf,
        "gbnf" => import_gbnf,
        "lark" => import_lark,
        "native" => parse_native_grammar,
        "pest" => import_pest,
        "tree-sitter-json" => import_tree_sitter_json,
        _ => return None,
    };
    Some(importer)
}

/// The emitter of one of [`GRAMMAR_EXPORT_FORMATS`].
#[must_use]
pub fn grammar_emitter(format: &str) -> Option<GrammarEmitter> {
    let emitter: GrammarEmitter = match format {
        "abnf" => emit_abnf,
        "antlr" => emit_antlr,
        "bnf" => emit_bnf,
        "ebnf" => emit_ebnf,
        "gbnf" => emit_gbnf,
        "lark" => emit_lark,
        "native" => |grammar| Ok((render_native_grammar(grammar), EmitReport::default())),
        "pest" => emit_pest,
        "tree-sitter-json" => emit_tree_sitter_json,
        _ => return None,
    };
    Some(emitter)
}

/// The help text of `meta-language grammar`, the same in both runtimes.
pub const GRAMMAR_COMMAND_USAGE: &str = "usage: meta-language grammar <command> [options]

commands:
  formats
      list the import and export formats
  import --from FORMAT FILE
      print the grammar as a native listing
  validate --from FORMAT FILE
      report grammar diagnostics; exit 1 when one is an error
  convert --from FORMAT --to FORMAT FILE
      translate a grammar to another format; lossy steps go to standard error
  export --to FORMAT FILE
      export a native listing to another format
  merge --source FORMAT:FILE... [--language NAME] [--require A=B]... [--to FORMAT]
      merge grammars of one language; earlier sources take precedence
  rename --from FORMAT --rule OLD --name NEW [--namespace NS] [--to FORMAT] FILE
      rename a rule and every reference to it
  round-trip --from FORMAT [--accept TEXT]... [--reject TEXT]... FILE
      export and re-import a mutated grammar; exit 1 when it is not preserved
  help
      print this help

exit status: 0 success, 1 a problem was found, 2 a usage or input error
";

/// What one `meta-language grammar` run printed and its exit status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarCommandOutput {
    /// 0 success, 1 a problem was found, 2 a usage or input error.
    pub exit_code: i32,
    /// Standard output.
    pub stdout: String,
    /// Standard error.
    pub stderr: String,
}

/// Reads a file's text for [`run_grammar_command`].
pub type GrammarFileReader<'a> = &'a dyn Fn(&str) -> std::io::Result<String>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Arity {
    One,
    Many,
}

struct CommandSpec {
    options: &'static [(&'static str, Arity)],
    required: &'static [&'static str],
    files: usize,
    run: fn(&mut Context<'_>) -> Result<i32, CommandError>,
}

struct CommandError {
    message: String,
    detail: Option<String>,
}

impl CommandError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            detail: None,
        }
    }

    fn with_detail(message: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            detail: Some(detail.into()),
        }
    }
}

struct Context<'a> {
    options: BTreeMap<&'static str, Vec<String>>,
    files: Vec<String>,
    read_file: GrammarFileReader<'a>,
    stdout: Vec<String>,
    stderr: Vec<String>,
}

impl Context<'_> {
    fn one(&self, option: &str) -> Option<&str> {
        self.options
            .get(option)
            .and_then(|values| values.first())
            .map(String::as_str)
    }

    fn many(&self, option: &str) -> &[String] {
        self.options.get(option).map_or(&[], Vec::as_slice)
    }

    fn required(&self, option: &str) -> &str {
        self.one(option).unwrap_or_default()
    }
}

fn command_spec(name: &str) -> Option<CommandSpec> {
    use Arity::{Many, One};
    Some(match name {
        "formats" => CommandSpec {
            options: &[],
            required: &[],
            files: 0,
            run: formats_command,
        },
        "import" => CommandSpec {
            options: &[("from", One)],
            required: &["from"],
            files: 1,
            run: import_command,
        },
        "validate" => CommandSpec {
            options: &[("from", One)],
            required: &["from"],
            files: 1,
            run: validate_command,
        },
        "convert" => CommandSpec {
            options: &[("from", One), ("to", One)],
            required: &["from", "to"],
            files: 1,
            run: convert_command,
        },
        "export" => CommandSpec {
            options: &[("to", One)],
            required: &["to"],
            files: 1,
            run: export_command,
        },
        "merge" => CommandSpec {
            options: &[
                ("source", Many),
                ("language", One),
                ("require", Many),
                ("to", One),
            ],
            required: &["source"],
            files: 0,
            run: merge_command,
        },
        "rename" => CommandSpec {
            options: &[
                ("from", One),
                ("rule", One),
                ("name", One),
                ("namespace", One),
                ("to", One),
            ],
            required: &["from", "rule", "name"],
            files: 1,
            run: rename_command,
        },
        "round-trip" => CommandSpec {
            options: &[("from", One), ("accept", Many), ("reject", Many)],
            required: &["from"],
            files: 1,
            run: round_trip_command,
        },
        _ => return None,
    })
}

/// Runs `meta-language grammar <command> ...` over `args`.
///
/// `read_file` returns a file's text. The output, messages and exit status
/// are the same as those of `runGrammarCommand` in the JavaScript package.
#[must_use]
pub fn run_grammar_command(
    args: &[String],
    read_file: GrammarFileReader<'_>,
) -> GrammarCommandOutput {
    let Some((name, rest)) = args.split_first() else {
        return GrammarCommandOutput {
            exit_code: 2,
            stdout: String::new(),
            stderr: GRAMMAR_COMMAND_USAGE.to_owned(),
        };
    };
    if matches!(name.as_str(), "help" | "--help" | "-h") {
        return GrammarCommandOutput {
            exit_code: 0,
            stdout: GRAMMAR_COMMAND_USAGE.to_owned(),
            stderr: String::new(),
        };
    }
    let mut context = Context {
        options: BTreeMap::new(),
        files: Vec::new(),
        read_file,
        stdout: Vec::new(),
        stderr: Vec::new(),
    };
    let result = command_spec(name)
        .ok_or_else(|| {
            CommandError::new(format!(
                "unknown command {name}; run meta-language grammar help"
            ))
        })
        .and_then(|spec| {
            parse_arguments(name, &spec, rest, &mut context)?;
            (spec.run)(&mut context)
        });
    let exit_code = result.unwrap_or_else(|error| {
        context.stderr.push(format!("error: {}", error.message));
        context.stderr.extend(error.detail);
        2
    });
    GrammarCommandOutput {
        exit_code,
        stdout: join_lines(&context.stdout),
        stderr: join_lines(&context.stderr),
    }
}

fn join_lines(lines: &[String]) -> String {
    lines
        .iter()
        .map(|line| {
            if line.ends_with('\n') {
                line.clone()
            } else {
                format!("{line}\n")
            }
        })
        .collect()
}

fn parse_arguments(
    name: &str,
    spec: &CommandSpec,
    rest: &[String],
    context: &mut Context<'_>,
) -> Result<(), CommandError> {
    let mut arguments = rest.iter();
    while let Some(argument) = arguments.next() {
        let Some(option) = argument.strip_prefix("--") else {
            context.files.push(argument.clone());
            continue;
        };
        let Some(&(key, arity)) = spec.options.iter().find(|(key, _)| *key == option) else {
            return Err(CommandError::new(format!(
                "unknown option {argument} for grammar {name}"
            )));
        };
        let Some(value) = arguments.next() else {
            return Err(CommandError::new(format!(
                "option {argument} needs a value"
            )));
        };
        let values = context.options.entry(key).or_default();
        if arity == Arity::One && !values.is_empty() {
            return Err(CommandError::new(format!(
                "option {argument} is given twice"
            )));
        }
        values.push(value.clone());
    }
    for option in spec.required {
        if !context.options.contains_key(option) {
            return Err(CommandError::new(format!(
                "grammar {name} needs --{option}"
            )));
        }
    }
    if context.files.len() > spec.files {
        return Err(CommandError::new(format!(
            "unexpected argument {}",
            context.files[spec.files]
        )));
    }
    if context.files.len() < spec.files {
        return Err(CommandError::new(format!(
            "grammar {name} needs a grammar file"
        )));
    }
    Ok(())
}

fn import_format(format: &str) -> Result<&str, CommandError> {
    if GRAMMAR_IMPORT_FORMATS.contains(&format) {
        Ok(format)
    } else {
        Err(CommandError::new(format!(
            "unknown import format {format} (expected one of: {})",
            GRAMMAR_IMPORT_FORMATS.join(", ")
        )))
    }
}

fn export_format(format: &str) -> Result<&str, CommandError> {
    if GRAMMAR_EXPORT_FORMATS.contains(&format) {
        Ok(format)
    } else {
        Err(CommandError::new(format!(
            "unknown export format {format} (expected one of: {})",
            GRAMMAR_EXPORT_FORMATS.join(", ")
        )))
    }
}

fn read(context: &Context<'_>, file: &str) -> Result<String, CommandError> {
    (context.read_file)(file).map_err(|_| CommandError::new(format!("cannot read {file}")))
}

fn import_failure(file: &str, format: &str, error: &GrammarImportError) -> CommandError {
    let category = match error {
        GrammarImportError::Unsupported { .. } => "unsupported construct",
        GrammarImportError::Parse { .. } => "parse error",
    };
    CommandError::with_detail(
        format!("cannot import {file} as {format} ({category})"),
        error.to_string(),
    )
}

fn emit_failure(format: &str, error: &GrammarEmitError) -> CommandError {
    CommandError::with_detail(format!("cannot export as {format}"), error.to_string())
}

fn load(context: &Context<'_>, file: &str, format: &str) -> Result<Grammar, CommandError> {
    let source = read(context, file)?;
    let import = grammar_importer(format).expect("checked import format");
    import(&source).map_err(|error| import_failure(file, format, &error))
}

fn emit(context: &mut Context<'_>, format: &str, grammar: &Grammar) -> Result<(), CommandError> {
    let emit = grammar_emitter(format).expect("checked export format");
    let (source, report) = emit(grammar).map_err(|error| emit_failure(format, &error))?;
    context.stdout.push(source);
    context
        .stderr
        .extend(report.lossy.iter().map(|note| format!("lossy: {note}")));
    Ok(())
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "every command shares the runner signature"
)]
fn formats_command(context: &mut Context<'_>) -> Result<i32, CommandError> {
    context
        .stdout
        .push(format!("import: {}", GRAMMAR_IMPORT_FORMATS.join(" ")));
    context
        .stdout
        .push(format!("export: {}", GRAMMAR_EXPORT_FORMATS.join(" ")));
    Ok(0)
}

fn import_command(context: &mut Context<'_>) -> Result<i32, CommandError> {
    let format = import_format(context.required("from"))?;
    let grammar = load(context, &context.files[0], format)?;
    context.stdout.push(render_native_grammar(&grammar));
    Ok(0)
}

fn validate_command(context: &mut Context<'_>) -> Result<i32, CommandError> {
    let format = import_format(context.required("from"))?;
    let diagnostics = validate(&load(context, &context.files[0], format)?);
    for diagnostic in &diagnostics {
        context.stdout.push(format!(
            "{} {} {}: {}",
            diagnostic.severity.as_str(),
            diagnostic.kind.as_str(),
            diagnostic.location.rule,
            diagnostic.message
        ));
    }
    let errors = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == Severity::Error)
        .count();
    context.stdout.push(format!(
        "{errors} error(s), {} warning(s)",
        diagnostics.len() - errors
    ));
    Ok(i32::from(errors > 0))
}

fn convert_command(context: &mut Context<'_>) -> Result<i32, CommandError> {
    let to = export_format(context.required("to"))?.to_owned();
    let format = import_format(context.required("from"))?;
    let grammar = load(context, &context.files[0], format)?;
    emit(context, &to, &grammar)?;
    Ok(0)
}

fn export_command(context: &mut Context<'_>) -> Result<i32, CommandError> {
    let to = export_format(context.required("to"))?.to_owned();
    let grammar = load(context, &context.files[0], "native")?;
    emit(context, &to, &grammar)?;
    Ok(0)
}

fn merge_command(context: &mut Context<'_>) -> Result<i32, CommandError> {
    let to = export_format(context.one("to").unwrap_or("native"))?.to_owned();
    let language = context.one("language").unwrap_or("grammar").to_owned();
    let mut ids = BTreeSet::new();
    let mut sources = Vec::new();
    for (precedence, argument) in context.many("source").iter().enumerate() {
        let Some((format, file)) = argument.split_once(':') else {
            return Err(CommandError::new(format!(
                "a merge source is FORMAT:FILE, not {argument}"
            )));
        };
        let format = import_format(format)?;
        let id = base_name(file);
        if !ids.insert(id.to_owned()) {
            return Err(CommandError::new(format!(
                "merge sources need distinct file names, {id} is given twice"
            )));
        }
        let precedence = u32::try_from(precedence).unwrap_or(u32::MAX);
        sources.push(
            GrammarMergeSource::new(id, language.as_str(), load(context, file, format)?)
                .with_precedence(precedence),
        );
    }
    let mut required_equivalences = Vec::new();
    for pair in context.many("require") {
        let Some((first, second)) = pair.split_once('=') else {
            return Err(CommandError::new(format!(
                "a required equivalence is SOURCE:RULE=SOURCE:RULE, not {pair}"
            )));
        };
        required_equivalences.push((first.to_owned(), second.to_owned()));
    }
    let options = GrammarMergeOptions {
        required_equivalences,
        ..GrammarMergeOptions::default()
    };
    let result =
        merge_grammars(&sources, &options).map_err(|error| CommandError::new(error.to_string()))?;
    let group = &result.groups[0];
    emit(context, &to, &group.grammar)?;
    for decision in &group.decisions {
        context.stderr.push(format!(
            "{} {}: {} ({})",
            decision.kind.as_str(),
            decision.name,
            decision.members.join(" "),
            decision.basis
        ));
    }
    for alternative in group.alternatives.iter().chain(&result.alternatives) {
        context.stderr.push(format!(
            "alternative {} {}: {}",
            alternative.reason.as_str(),
            alternative.name,
            alternative.options.join(" ")
        ));
    }
    for failure in &result.failures {
        context.stderr.push(format!(
            "unresolved {}: {}",
            failure.members.join(" = "),
            failure.reason.as_str()
        ));
    }
    Ok(i32::from(result.status() != "complete"))
}

fn base_name(file: &str) -> &str {
    file.rsplit(['/', '\\']).next().unwrap_or(file)
}

fn rename_command(context: &mut Context<'_>) -> Result<i32, CommandError> {
    let to = export_format(context.one("to").unwrap_or("native"))?.to_owned();
    let format = import_format(context.required("from"))?;
    let grammar = load(context, &context.files[0], format)?;
    let renamed = rename_grammar_rule(
        &grammar,
        context.required("rule"),
        context.required("name"),
        context.one("namespace"),
        &[],
    )
    .map_err(|error| CommandError::new(error.to_string()))?;
    emit(context, &to, &renamed.grammar)?;
    for alias in &renamed.aliases {
        context
            .stderr
            .push(format!("alias {} = {}", alias.canonical, alias.original));
    }
    Ok(0)
}

fn round_trip_command(context: &mut Context<'_>) -> Result<i32, CommandError> {
    let format = import_format(context.required("from"))?.to_owned();
    if !GRAMMAR_EXPORT_FORMATS.contains(&format.as_str()) {
        return Err(CommandError::new(format!(
            "round-trip needs a format that is both imported and exported (one of: {})",
            GRAMMAR_EXPORT_FORMATS.join(", ")
        )));
    }
    let file = context.files[0].clone();
    let source = read(context, &file)?;
    let import = grammar_importer(&format).expect("checked import format");
    let emit = grammar_emitter(&format).expect("checked export format");
    let (accepts, rejects) = (context.many("accept"), context.many("reject"));
    let report = check_grammar_round_trip(
        &source,
        &GrammarRoundTrip::new(&import, &emit).with_samples(accepts, rejects),
    )
    .map_err(|error| match error {
        GrammarRoundTripError::Import(error) => import_failure(&file, &format, &error),
        GrammarRoundTripError::Emit(error) => emit_failure(&format, &error),
        error @ (GrammarRoundTripError::Normalize(_) | GrammarRoundTripError::MissingStartRule) => {
            CommandError::with_detail(format!("cannot round-trip {file}"), error.to_string())
        }
    })?;
    context.stdout.push(report.status.as_str().to_owned());
    for failure in &report.failures {
        context.stdout.push(format!(
            "{} {}: {}",
            failure.kind.as_str(),
            failure.stage.as_str(),
            failure.detail
        ));
    }
    Ok(i32::from(report.status.as_str() != "preserved"))
}
