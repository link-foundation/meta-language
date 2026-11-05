//! Command transport over the existing dynamic grammar/definition algorithms.
//! The shared source-generated vocabulary owns argument admission.
use crate::dynamic_language_command_rules as rules;
use crate::{
    DynamicMetaLanguage, DynamicMetaLanguageOptions, DynamicParseOptions, GlobalConceptSource,
    MetaLanguageArchiveLimits, create_dynamic_meta_language, meta_language_archive_to_binary,
    meta_language_archive_to_text, parse_grammar_links, read_meta_language_archive_binary,
    read_meta_language_archive_text, read_structured_data_lino, render_grammar_links,
    structured_data_to_lino,
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

fn execute<T, B>(
    args: &Arguments,
    read_text: &mut T,
    read_bytes: &mut B,
) -> Result<DynamicLanguageCommandOutput, String>
where
    T: FnMut(&str) -> Result<String, String>,
    B: FnMut(&str) -> Result<Vec<u8>, String>,
{
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
    let limits = MetaLanguageArchiveLimits::default();
    let session = if args.command == "prepare" {
        if additions.is_empty() {
            return Err("prepare needs complete source grammars".to_owned());
        }
        create_dynamic_meta_language(&additions, &options)?
    } else {
        let file = args
            .file
            .as_deref()
            .ok_or("a language archive input is required")?;
        let archive = if args.options.contains_key("--input-binary") {
            read_meta_language_archive_binary(&read_bytes(file)?, limits)?.archive
        } else {
            read_meta_language_archive_text(&read_text(file)?, limits)?.archive
        };
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
    let stdout = if args.options.contains_key("--binary") {
        meta_language_archive_to_binary(&archive, limits)?
    } else {
        meta_language_archive_to_text(&archive, limits)?.into_bytes()
    };
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
