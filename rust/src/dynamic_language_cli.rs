//! Command transport over the existing dynamic grammar/definition algorithms.
//! The shared source-generated vocabulary owns argument admission.
use crate::dynamic_language_command_rules as rules;
use crate::grammar_inference::{
    STRUCTURAL_GRAMMAR_INFERENCE_MODEL_IDENTIFIER, regular_grammar_refinement_archive_limits,
};
use crate::{
    DynamicMetaLanguage, DynamicMetaLanguageOptions, DynamicParseOptions, GlobalConceptSource,
    LoadedMetaLanguageArchive, MetaLanguageArchive, MetaLanguageArchiveLimits,
    RegularGrammarInferenceOptions, StructuralGrammarInferenceOptions,
    create_dynamic_meta_language, infer_regular_grammar, infer_structural_grammar,
    meta_language_archive_to_binary, meta_language_archive_to_link_cli_store,
    meta_language_archive_to_text, parse_grammar_links, read_meta_language_archive_binary,
    read_meta_language_archive_link_cli_store, read_meta_language_archive_text,
    read_refined_regular_grammar_inference_links, read_structural_grammar_inference_links,
    read_structured_data_lino, refine_regular_grammar_inference,
    refine_structural_grammar_inference, render_grammar_links, structured_data_to_lino,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;

mod arguments;
mod configuration;
use arguments::Arguments;

/// Binary-safe command output; a rejected parse can still retain its CST archive.
#[derive(Clone, Debug)]
pub struct DynamicLanguageCommandOutput {
    /// Zero for success, one for an actual parse rejection, two for invalid input.
    pub exit_code: i32,
    /// Complete Links text, graph bytes, or a Links source inventory.
    pub stdout: Vec<u8>,
    /// Links rejection diagnostics, or an argument/library error and usage.
    pub stderr: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LabelledExamples {
    positives: Vec<Vec<String>>,
    negatives: Vec<Vec<String>>,
    delimiters: Option<Vec<[String; 2]>>,
    scanner_observations: Option<Vec<crate::GrammarScannerObservation>>,
    #[serde(default)]
    scanner_character_classes: bool,
    scanner_mode_stack_observations: Option<Vec<crate::GrammarScannerModeStackObservation>>,
}

fn labelled_examples(
    text: &str,
    allow_discovery_options: bool,
) -> Result<LabelledExamples, String> {
    let value = read_structured_data_lino(text)?;
    let message =
        || "labelled examples must contain positive and negative symbol sequences".to_owned();
    let scanner_fields = [
        "scannerObservations",
        "scannerModeStackObservations",
        "scannerCharacterClasses",
    ];
    if [
        "delimiters",
        "scannerObservations",
        "scannerModeStackObservations",
    ]
    .into_iter()
    .any(|field| {
        value
            .get(field)
            .is_some_and(|observations| !allow_discovery_options || observations.is_null())
    }) || (!allow_discovery_options && value.get("scannerCharacterClasses").is_some())
    {
        return Err(message());
    }
    if value.get("delimiters").is_some()
        && scanner_fields
            .iter()
            .any(|field| value.get(*field).is_some())
    {
        return Err(
            "structural delimiters and scanner observations require separate discovery inputs"
                .to_owned(),
        );
    }
    serde_json::from_value(value).map_err(|_| message())
}

fn archive_input<T, B>(
    args: &Arguments,
    read_text: &mut T,
    read_bytes: &mut B,
    limits: MetaLanguageArchiveLimits,
) -> Result<LoadedMetaLanguageArchive, String>
where
    T: FnMut(&str) -> Result<String, String>,
    B: FnMut(&str) -> Result<Vec<u8>, String>,
{
    let file = args
        .file
        .as_deref()
        .ok_or("a language archive input is required")?;
    if args.options.contains_key("--input-link-cli-store") {
        read_meta_language_archive_link_cli_store(&read_bytes(file)?, limits)
    } else if args.options.contains_key("--input-binary") {
        read_meta_language_archive_binary(&read_bytes(file)?, limits)
    } else {
        read_meta_language_archive_text(&read_text(file)?, limits)
    }
}

fn archive_output(
    archive: &MetaLanguageArchive,
    args: &Arguments,
    limits: MetaLanguageArchiveLimits,
) -> Result<Vec<u8>, String> {
    if args.options.contains_key("--link-cli-store") {
        meta_language_archive_to_link_cli_store(archive, limits)
    } else if args.options.contains_key("--binary") {
        meta_language_archive_to_binary(archive, limits)
    } else {
        Ok(meta_language_archive_to_text(archive, limits)?.into_bytes())
    }
}

fn discovery_archive<T, B>(
    args: &Arguments,
    read_text: &mut T,
    read_bytes: &mut B,
) -> Result<MetaLanguageArchive, String>
where
    T: FnMut(&str) -> Result<String, String>,
    B: FnMut(&str) -> Result<Vec<u8>, String>,
{
    let identifier = args.options.get("--identifier");
    let result = if args.command == "discover" {
        let file = args
            .file
            .as_deref()
            .ok_or("a labelled examples input is required")?;
        let examples = labelled_examples(&read_text(file)?, true)?;
        if let Some(delimiters) = examples.delimiters {
            let defaults = StructuralGrammarInferenceOptions::default();
            let options = StructuralGrammarInferenceOptions {
                delimiters,
                identifier: identifier.cloned().unwrap_or(defaults.identifier),
                language: args
                    .options
                    .get("--language")
                    .cloned()
                    .unwrap_or(defaults.language),
                edition: args
                    .options
                    .get("--edition")
                    .cloned()
                    .unwrap_or(defaults.edition),
                ..defaults
            };
            let result =
                infer_structural_grammar(&examples.positives, &examples.negatives, &options)?;
            return Ok(MetaLanguageArchive {
                network: result.network,
                root: result.root,
            });
        }
        let mut options = RegularGrammarInferenceOptions::default();
        options.identifier = identifier.cloned().unwrap_or(options.identifier);
        options.language = args
            .options
            .get("--language")
            .cloned()
            .unwrap_or(options.language);
        options.edition = args
            .options
            .get("--edition")
            .cloned()
            .unwrap_or(options.edition);
        options.scanner_observations = examples.scanner_observations;
        options.scanner_character_classes = examples.scanner_character_classes;
        options.scanner_mode_stack_observations = examples.scanner_mode_stack_observations;
        infer_regular_grammar(&examples.positives, &examples.negatives, &options)?
    } else {
        let archive = archive_input(
            args,
            read_text,
            read_bytes,
            regular_grammar_refinement_archive_limits(),
        )?;
        let structural = archive
            .entries
            .iter()
            .any(|entry| entry.identifier == STRUCTURAL_GRAMMAR_INFERENCE_MODEL_IDENTIFIER);
        if args.command == "install" {
            let source = if structural {
                read_structural_grammar_inference_links(&archive.network, archive.root)?.source
            } else {
                read_refined_regular_grammar_inference_links(&archive.network, archive.root)?.source
            };
            return Ok(create_dynamic_meta_language(
                &[source],
                &DynamicMetaLanguageOptions::default(),
            )?
            .archive()
            .clone());
        }
        let file = args
            .options
            .get("--examples")
            .ok_or("refine needs a labelled examples input")?;
        let examples = labelled_examples(&read_text(file)?, false)?;
        if structural {
            let result = refine_structural_grammar_inference(
                &archive.network,
                archive.root,
                &examples.positives,
                &examples.negatives,
                identifier.map(String::as_str),
            )?;
            return Ok(MetaLanguageArchive {
                network: result.network,
                root: result.root,
            });
        }
        refine_regular_grammar_inference(
            &archive.network,
            archive.root,
            &examples.positives,
            &examples.negatives,
            identifier.map(String::as_str),
        )?
    };
    Ok(MetaLanguageArchive {
        network: result.network,
        root: result.root,
    })
}

fn execute<T, B>(
    args: &Arguments,
    read_text: &mut T,
    read_bytes: &mut B,
) -> Result<DynamicLanguageCommandOutput, String>
where
    T: FnMut(&str) -> Result<String, String>,
    B: FnMut(&str) -> Result<Vec<u8>, String>,
{
    let limits = MetaLanguageArchiveLimits::default();
    if matches!(args.command.as_str(), "discover" | "refine" | "install") {
        let archive = discovery_archive(args, read_text, read_bytes)?;
        let output_limits = if args.command == "install" {
            limits
        } else {
            regular_grammar_refinement_archive_limits()
        };
        return Ok(success(archive_output(&archive, args, output_limits)?));
    }
    let options = configuration::read(args.options.get("--configuration"), read_text)?;
    let additions = args
        .sources
        .iter()
        .map(|source| {
            Ok(GlobalConceptSource {
                id: source[0].clone(),
                language: source[1].clone(),
                edition: source[2].clone(),
                grammar: parse_grammar_links(&read_text(&source[3])?)
                    .map_err(|error| error.to_string())?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let session = if args.command == "prepare" {
        if additions.is_empty() {
            return Err("prepare needs complete source grammars".to_owned());
        }
        create_dynamic_meta_language(&additions, &options)?
    } else {
        let archive = archive_input(args, read_text, read_bytes, limits)?;
        let session = DynamicMetaLanguage::new(&archive, &DynamicMetaLanguageOptions::default())?;
        if args.command == "extend" {
            if additions.is_empty() {
                return Err("extend needs complete source grammars".to_owned());
            }
            let mut options = options;
            options.replace = args.options.contains_key("--replace");
            session.extend(&additions, &options)?
        } else {
            session
        }
    };
    if args.command == "list" {
        let selected = args.options.get("--source");
        let rows = session.sources().iter().filter(|source| selected.is_none_or(|id| id == &source.id))
            .map(|source| json!({"identifier":source.id,"language":source.language,"edition":source.edition,"rules":source.grammar.rules().len()})).collect::<Vec<_>>();
        if selected.is_some() && rows.is_empty() {
            return Err("unknown source-qualified grammar identifier".to_owned());
        }
        return Ok(success(structured_data_to_lino(&json!(rows))?.into_bytes()));
    }
    if args.command == "grammar" {
        let source = session
            .sources()
            .iter()
            .find(|source| args.options.get("--source") == Some(&source.id))
            .ok_or("grammar needs a known source-qualified identifier")?;
        return Ok(success(render_grammar_links(&source.grammar).into_bytes()));
    }
    let mut archive = session.archive().clone();
    let mut exit_code = 0;
    let mut stderr = String::new();
    if args.command == "parse" {
        let source = args
            .options
            .get("--source")
            .ok_or("parse needs a source identity and an input file")?;
        let input = args
            .options
            .get("--input")
            .ok_or("parse needs a source identity and an input file")?;
        let parsed = session.parse(
            source,
            &read_bytes(input)?,
            &configuration::parse_options(&args.options)?,
        )?;
        if !parsed.outcome.ok {
            exit_code = 1;
            let data = parsed.outcome_data(256)?;
            stderr = structured_data_to_lino(&json!({"rejection":data["rejection"]}))?;
        }
        let Some(saved) = parsed.archive else {
            return Ok(DynamicLanguageCommandOutput {
                exit_code: 1,
                stdout: Vec::new(),
                stderr,
            });
        };
        archive = saved;
    }
    let stdout = archive_output(&archive, args, limits)?;
    Ok(DynamicLanguageCommandOutput {
        exit_code,
        stdout,
        stderr,
    })
}
fn success(stdout: Vec<u8>) -> DynamicLanguageCommandOutput {
    DynamicLanguageCommandOutput {
        exit_code: 0,
        stdout,
        stderr: String::new(),
    }
}

/// Run source-free dynamic preparation, extension, parsing and grammar export.
/// File callbacks only supply bytes; no process, parser oracle or source fetch runs.
pub fn run_dynamic_language_command<T, B>(
    args: &[String],
    mut read_text: T,
    mut read_bytes: B,
) -> DynamicLanguageCommandOutput
where
    T: FnMut(&str) -> Result<String, String>,
    B: FnMut(&str) -> Result<Vec<u8>, String>,
{
    if args
        .first()
        .is_some_and(|value| ["help", "--help", "-h"].contains(&value.as_str()))
    {
        return success(rules::read_dynamic_language_command_usage().into_bytes());
    }
    match arguments::parse(args).and_then(|args| execute(&args, &mut read_text, &mut read_bytes)) {
        Ok(value) => value,
        Err(error) => DynamicLanguageCommandOutput {
            exit_code: 2,
            stdout: Vec::new(),
            stderr: format!(
                "error: {error}\n{}",
                rules::read_dynamic_language_command_usage()
            ),
        },
    }
}
