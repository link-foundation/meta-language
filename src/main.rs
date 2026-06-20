use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use clap::{Parser, Subcommand, ValueEnum};

use meta_language::{
    emit_abnf, emit_bnf, emit_ebnf, emit_gbnf, emit_pest, emit_tree_sitter_grammar_js, evaluate,
    grammar_concept_translation_rules, grammar_from_lino, grammar_to_lino, import_abnf,
    import_antlr, import_bnf, import_ebnf, import_gbnf, import_lark, import_pest,
    import_tree_sitter_json, infer_cfg, parse_grammar_surface, translate_grammar_surface,
    write_grammar_surface, Grammar, InferenceOptions, LinkNetwork, MembershipOracle,
    ParseConfiguration, PositiveOnlyOracle, SampleConfig,
};

#[derive(Parser, Debug)]
#[command(
    name = "meta-language",
    about = "Build and verify self-describing links networks"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Print the built-in self-description roots.
    Describe,
    /// Parse text into a lossless token network and verify it is clean.
    Verify {
        /// Language label for the parsed region.
        #[arg(long)]
        language: String,
        /// Source text to parse.
        #[arg(long)]
        text: String,
    },
    /// Infer a grammar from example texts.
    Infer {
        /// One or more files or directories of example texts.
        #[arg(required = true)]
        examples: Vec<PathBuf>,
        /// Output grammar notation.
        #[arg(long, value_enum, default_value = "lino")]
        format: GrammarFormatArg,
        /// Inference strategy / pipeline depth.
        #[arg(long, value_enum, default_value = "auto")]
        strategy: StrategyArg,
        /// Also print precision/recall/F1/runtime to stderr.
        #[arg(long)]
        metrics: bool,
        /// Write the grammar here instead of stdout.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Import a grammar in another notation into the IR / `LiNo`.
    ImportGrammar {
        /// Source notation of the input file.
        #[arg(long, value_enum)]
        format: ImportFormatArg,
        /// Grammar file to import.
        #[arg(required = true)]
        input: PathBuf,
        /// Output notation for the imported grammar.
        #[arg(long, value_enum, default_value = "lino")]
        to: GrammarFormatArg,
        /// Write the grammar here instead of stdout.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Emit an IR/LiNo grammar to another notation.
    EmitGrammar {
        /// Target notation.
        #[arg(long, value_enum)]
        format: EmitFormatArg,
        /// Grammar file (IR/LiNo or native grammar surface) to emit from.
        #[arg(required = true)]
        input: PathBuf,
        /// Write the grammar here instead of stdout.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Translate a grammar's human-facing surface across languages.
    TranslateGrammar {
        /// Grammar file (IR/LiNo or native grammar surface) to translate.
        #[arg(required = true)]
        input: PathBuf,
        /// Source natural language, for example en.
        #[arg(long)]
        from_language: String,
        /// Target natural language, for example ru.
        #[arg(long)]
        to_language: String,
        /// Write the grammar here instead of stdout.
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum GrammarFormatArg {
    /// Canonical links-network `LiNo` encoding.
    Lino,
    /// Native grammar authoring surface.
    MetaLanguage,
    /// Backus-Naur Form.
    Bnf,
    /// Extended Backus-Naur Form.
    Ebnf,
    /// Augmented Backus-Naur Form.
    Abnf,
    /// Parsing Expression Grammar surface.
    Peg,
    /// ANTLR grammar.
    Antlr,
    /// Lark grammar.
    Lark,
    /// GBNF grammar.
    Gbnf,
    /// Tree-sitter grammar.js.
    TreeSitter,
    /// Inferred grammar source tag.
    Inferred,
}

impl GrammarFormatArg {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Lino => "lino",
            Self::MetaLanguage => "meta-language",
            Self::Bnf => "bnf",
            Self::Ebnf => "ebnf",
            Self::Abnf => "abnf",
            Self::Peg => "peg",
            Self::Antlr => "antlr",
            Self::Lark => "lark",
            Self::Gbnf => "gbnf",
            Self::TreeSitter => "tree-sitter",
            Self::Inferred => "inferred",
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ImportFormatArg {
    /// Backus-Naur Form.
    Bnf,
    /// Extended Backus-Naur Form.
    Ebnf,
    /// Augmented Backus-Naur Form.
    Abnf,
    /// Parsing Expression Grammar surface.
    Peg,
    /// ANTLR grammar.
    Antlr,
    /// Lark grammar.
    Lark,
    /// GBNF grammar.
    Gbnf,
    /// Tree-sitter JSON grammar.
    TreeSitter,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum EmitFormatArg {
    /// Backus-Naur Form.
    Bnf,
    /// Extended Backus-Naur Form.
    Ebnf,
    /// Augmented Backus-Naur Form.
    Abnf,
    /// Parsing Expression Grammar surface.
    Peg,
    /// GBNF grammar.
    Gbnf,
    /// Tree-sitter grammar.js.
    TreeSitter,
    /// ANTLR grammar.
    Antlr,
    /// Lark grammar.
    Lark,
}

impl EmitFormatArg {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Bnf => "bnf",
            Self::Ebnf => "ebnf",
            Self::Abnf => "abnf",
            Self::Peg => "peg",
            Self::Gbnf => "gbnf",
            Self::TreeSitter => "tree-sitter",
            Self::Antlr => "antlr",
            Self::Lark => "lark",
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum StrategyArg {
    /// Use the default deterministic structural pipeline.
    Auto,
    /// Enable the incremental inference option.
    Incremental,
}

impl StrategyArg {
    fn options(self) -> InferenceOptions {
        let mut options = InferenceOptions::default();
        if matches!(self, Self::Incremental) {
            options.incremental = true;
        }
        options
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Describe => describe(),
        Command::Verify { language, text } => verify(&language, &text),
        Command::Infer {
            examples,
            format,
            strategy,
            metrics,
            out,
        } => infer(&examples, format, strategy, metrics, out.as_deref()),
        Command::ImportGrammar {
            format,
            input,
            to,
            out,
        } => import_grammar(format, &input, to, out.as_deref()),
        Command::EmitGrammar { format, input, out } => {
            emit_grammar(format, &input, out.as_deref());
        }
        Command::TranslateGrammar {
            input,
            from_language,
            to_language,
            out,
        } => translate_grammar(&input, &from_language, &to_language, out.as_deref()),
    }
}

fn describe() {
    let network = LinkNetwork::self_describing();
    print!("{}", network.self_description_text());
}

fn verify(language: &str, text: &str) {
    let network = LinkNetwork::parse(text, language, ParseConfiguration::default());
    let report = network.verify_full_match(None);

    if report.is_clean() {
        println!("clean");
    } else {
        for issue in report.issues() {
            eprintln!("{}: {:?}", issue.link_id(), issue.kind());
        }
        std::process::exit(1);
    }
}

fn infer(
    paths: &[PathBuf],
    format: GrammarFormatArg,
    strategy: StrategyArg,
    metrics: bool,
    out: Option<&Path>,
) {
    let started_at = Instant::now();
    let examples = read_examples(paths).unwrap_or_else(|error| fail(error));
    if examples.is_empty() {
        fail("no example texts were found");
    }

    let result = infer_cfg(&examples, &PositiveOnlyOracle::new(), strategy.options());
    let rendered = render_grammar(&result.grammar, format).unwrap_or_else(|error| fail(error));
    write_output(&rendered, out).unwrap_or_else(|error| fail(error));

    if metrics {
        print_metrics(
            &result.grammar,
            &examples,
            result.report.rules,
            result.report.bubbles_proposed,
            result.report.merges_accepted,
            result.report.merges_rejected,
            started_at.elapsed(),
        )
        .unwrap_or_else(|error| fail(error));
    }
}

fn import_grammar(format: ImportFormatArg, input: &Path, to: GrammarFormatArg, out: Option<&Path>) {
    let text = read_file(input, "grammar").unwrap_or_else(|error| fail(error));
    let grammar = import_with_format(format, &text).unwrap_or_else(|error| fail(error));
    let rendered = render_grammar(&grammar, to).unwrap_or_else(|error| fail(error));
    write_output(&rendered, out).unwrap_or_else(|error| fail(error));
}

fn emit_grammar(format: EmitFormatArg, input: &Path, out: Option<&Path>) {
    let grammar = read_ir_grammar(input).unwrap_or_else(|error| fail(error));
    let rendered = render_emit_format(&grammar, format).unwrap_or_else(|error| fail(error));
    write_output(&rendered, out).unwrap_or_else(|error| fail(error));
}

fn translate_grammar(input: &Path, from_language: &str, to_language: &str, out: Option<&Path>) {
    if from_language.trim().is_empty() {
        fail("from language must not be empty");
    }
    if to_language.trim().is_empty() {
        fail("to language must not be empty");
    }

    let grammar = read_ir_grammar(input).unwrap_or_else(|error| fail(error));
    let rules = grammar_concept_translation_rules();
    let translated = translate_grammar_surface(&grammar, to_language, &rules)
        .unwrap_or_else(|error| fail(error.to_string()));
    let rendered = grammar_to_lino(&translated);
    write_output(&rendered, out).unwrap_or_else(|error| fail(error));
}

fn read_examples(paths: &[PathBuf]) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    for path in paths {
        collect_example_files(path, &mut files)?;
    }

    files.sort();
    files.dedup();
    files
        .iter()
        .map(|path| read_file(path, "example"))
        .collect()
}

fn collect_example_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;

    if metadata.is_file() {
        files.push(path.to_path_buf());
        return Ok(());
    }

    if metadata.is_dir() {
        let mut entries = fs::read_dir(path)
            .map_err(|error| format!("failed to read directory {}: {error}", path.display()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                format!(
                    "failed to read directory entry in {}: {error}",
                    path.display()
                )
            })?;
        entries.sort_by_key(std::fs::DirEntry::path);
        for entry in entries {
            collect_example_files(&entry.path(), files)?;
        }
        return Ok(());
    }

    Err(format!(
        "example path is not a regular file or directory: {}",
        path.display()
    ))
}

fn import_with_format(format: ImportFormatArg, text: &str) -> Result<Grammar, String> {
    match format {
        ImportFormatArg::Bnf => import_bnf(text),
        ImportFormatArg::Ebnf => import_ebnf(text),
        ImportFormatArg::Abnf => import_abnf(text),
        ImportFormatArg::Peg => import_pest(text),
        ImportFormatArg::Antlr => import_antlr(text),
        ImportFormatArg::Lark => import_lark(text),
        ImportFormatArg::Gbnf => import_gbnf(text),
        ImportFormatArg::TreeSitter => import_tree_sitter_json(text),
    }
    .map_err(|error| error.to_string())
}

fn render_grammar(grammar: &Grammar, format: GrammarFormatArg) -> Result<String, String> {
    match format {
        GrammarFormatArg::Lino => Ok(grammar_to_lino(grammar)),
        GrammarFormatArg::MetaLanguage => Ok(write_grammar_surface(grammar)),
        GrammarFormatArg::Bnf => emit_bnf(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        GrammarFormatArg::Ebnf => emit_ebnf(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        GrammarFormatArg::Abnf => emit_abnf(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        GrammarFormatArg::Peg => emit_pest(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        GrammarFormatArg::Gbnf => emit_gbnf(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        GrammarFormatArg::TreeSitter => {
            emit_tree_sitter_grammar_js(grammar).map_err(|error| error.to_string())
        }
        GrammarFormatArg::Antlr | GrammarFormatArg::Lark | GrammarFormatArg::Inferred => {
            Err(format!("unsupported output format: {}", format.as_str()))
        }
    }
}

fn render_emit_format(grammar: &Grammar, format: EmitFormatArg) -> Result<String, String> {
    match format {
        EmitFormatArg::Bnf => emit_bnf(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        EmitFormatArg::Ebnf => emit_ebnf(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        EmitFormatArg::Abnf => emit_abnf(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        EmitFormatArg::Peg => emit_pest(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        EmitFormatArg::Gbnf => emit_gbnf(grammar)
            .map(|(text, _report)| text)
            .map_err(|error| error.to_string()),
        EmitFormatArg::TreeSitter => {
            emit_tree_sitter_grammar_js(grammar).map_err(|error| error.to_string())
        }
        EmitFormatArg::Antlr | EmitFormatArg::Lark => {
            Err(format!("unsupported output format: {}", format.as_str()))
        }
    }
}

fn read_ir_grammar(input: &Path) -> Result<Grammar, String> {
    let text = read_file(input, "grammar")?;
    grammar_from_lino(&text).or_else(|lino_error| {
        parse_grammar_surface(&text).map_err(|surface_error| {
            format!(
                "failed to parse {} as LiNo ({lino_error}) or grammar surface ({surface_error})",
                input.display()
            )
        })
    })
}

fn print_metrics(
    grammar: &Grammar,
    examples: &[String],
    rules: usize,
    bubbles_proposed: usize,
    merges_accepted: usize,
    merges_rejected: usize,
    runtime: Duration,
) -> Result<(), String> {
    let positives = examples.iter().map(String::as_str).collect::<Vec<_>>();
    let scores = evaluate(
        grammar,
        &PositiveSetOracle::new(examples),
        None,
        &positives,
        &SampleConfig {
            seed: 9,
            count: 64,
            max_depth: 8,
            repeat_cap: 4,
        },
    )
    .map_err(|error| error.to_string())?;

    eprintln!(
        "precision={:.3} recall={:.3} f1={:.3} runtime_ms={} rules={rules} bubbles={bubbles_proposed} merges_accepted={merges_accepted} merges_rejected={merges_rejected}",
        scores.precision,
        scores.recall,
        scores.f1,
        runtime.as_millis()
    );
    Ok(())
}

fn read_file(path: &Path, label: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|error| format!("failed to read {label} {}: {error}", path.display()))
}

fn write_output(text: &str, out: Option<&Path>) -> Result<(), String> {
    if let Some(path) = out {
        fs::write(path, text)
            .map_err(|error| format!("failed to write output {}: {error}", path.display()))?;
    } else {
        print!("{text}");
    }
    Ok(())
}

fn fail(message: impl AsRef<str>) -> ! {
    eprintln!("error: {}", message.as_ref());
    std::process::exit(1);
}

#[derive(Clone, Debug)]
struct PositiveSetOracle {
    examples: BTreeSet<String>,
}

impl PositiveSetOracle {
    fn new(examples: &[String]) -> Self {
        Self {
            examples: examples.iter().cloned().collect(),
        }
    }
}

impl MembershipOracle for PositiveSetOracle {
    fn accepts(&self, text: &str) -> bool {
        self.examples.contains(text)
    }
}
