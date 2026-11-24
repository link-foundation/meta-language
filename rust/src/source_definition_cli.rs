//! Source catalog command transport. Shared definitions own argument admission.
use crate::source_definition_catalog_rules::{self as rules, ml_string::Text};
use crate::{
    DynamicLanguageCommandOutput, FormalFunctionSelector, MetaLanguageArchiveEntry,
    MetaLanguageArchiveLimits, create_meta_language_archive,
    create_source_definition_formal_function_links, create_source_definition_program_links,
    meta_language_archive_to_binary, meta_language_archive_to_text, source_definition_catalog,
    structured_data_to_lino,
};
use std::sync::Arc;

fn execute(args: &[String]) -> Result<Vec<u8>, String> {
    let command = args.first().map_or("", String::as_str);
    let mut positionals = Vec::new();
    let mut binary = false;
    for value in args.iter().skip(1) {
        if value == "--binary" && !binary {
            binary = true;
        } else if value.starts_with("--") {
            return Err(format!("unexpected definition argument {value}"));
        } else {
            positionals.push(value.as_str());
        }
    }
    if !rules::accept_source_definition_command_arguments(
        &Text::from_text(command),
        serde_json::Number::from(positionals.len())
            .as_f64()
            .ok_or("unrepresentable definition argument count")?,
        binary,
    ) {
        return Err("unknown definition operation or argument count".to_owned());
    }
    if command == "list" {
        return structured_data_to_lino(
            &serde_json::to_value(source_definition_catalog()?)
                .map_err(|error| error.to_string())?,
        )
        .map(String::into_bytes);
    }
    let (network, root) = if command == "function" {
        let definition = create_source_definition_formal_function_links(
            positionals[0],
            FormalFunctionSelector::Name(positionals[1]),
        )?;
        (definition.network, definition.root)
    } else {
        let program = create_source_definition_program_links(positionals[0])?;
        (program.network, program.root)
    };
    let identifier = if command == "function" {
        format!("{}#{}", positionals[0], positionals[1])
    } else {
        positionals[0].to_owned()
    };
    let limits = MetaLanguageArchiveLimits::default();
    let archive = create_meta_language_archive(
        &[MetaLanguageArchiveEntry {
            identifier,
            kind: if command == "function" {
                "formal concept definition"
            } else {
                "typed program"
            }
            .to_owned(),
            network: Arc::new(network),
            root,
        }],
        limits,
    )?;
    if binary {
        meta_language_archive_to_binary(&archive, limits)
    } else {
        meta_language_archive_to_text(&archive, limits).map(String::into_bytes)
    }
}

/// List source identities or export one complete program/formal archive.
/// Unsupported formal models remain explicit diagnostics, never inferred proofs.
#[must_use]
pub fn run_source_definition_command(args: &[String]) -> DynamicLanguageCommandOutput {
    let usage = rules::read_source_definition_command_usage().to_string();
    if args
        .first()
        .is_some_and(|value| matches!(value.as_str(), "help" | "--help" | "-h"))
    {
        return DynamicLanguageCommandOutput {
            exit_code: 0,
            stdout: usage.into_bytes(),
            stderr: String::new(),
        };
    }
    match execute(args) {
        Ok(stdout) => DynamicLanguageCommandOutput {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        },
        Err(error) => DynamicLanguageCommandOutput {
            exit_code: 2,
            stdout: Vec::new(),
            stderr: format!("error: {error}\n{usage}"),
        },
    }
}
