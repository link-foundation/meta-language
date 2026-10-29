//! Full concrete source storage and draft transport through existing archives.
//! Shared generated definitions decide source membership and product admission.
use crate::project_translation_archive_rules as rules;
use crate::{
    LinkId, LinkNetwork, MetaLanguageArchive, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits,
    SourceProjectArchiveView, StructuredDataLinksLimits, create_meta_language_archive,
    create_source_project_archive, decode_structured_data_links_with_limits,
    draft_project_translation, project_translation_draft_to_links, read_meta_language_archive,
    read_source_project_archive,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::sync::Arc;
fn fail(message: &str) -> String {
    format!("project translation archive: {message}")
}
fn vocabulary(role: &str) -> String {
    rules::read_project_translation_archive_vocabulary(role)
}
fn bound(options: &Value, key: &str, fallback: usize) -> Result<usize, String> {
    match options.get(key) {
        None | Some(Value::Null) => Ok(fallback),
        Some(value) => value
            .as_f64()
            .filter(|value| {
                value.is_finite()
                    && value.fract() == 0.0
                    && *value >= 1.0
                    && *value <= 9_007_199_254_740_991.0
                    && *value <= usize::MAX as f64
            })
            .map(|value| value as usize)
            .ok_or_else(|| fail("invalid archive resource limit")),
    }
}
fn limits(options: &Value) -> Result<MetaLanguageArchiveLimits, String> {
    if !options.is_object() {
        return Err(fail("expected a resource options record"));
    }
    Ok(MetaLanguageArchiveLimits {
        node_limit: bound(options, "nodeLimit", 1000000)?,
        reference_limit: bound(options, "referenceLimit", 4000000)?,
        byte_limit: bound(options, "byteLimit", 128 * 1024 * 1024)?,
    })
}
fn data_limits(options: &Value) -> Result<StructuredDataLinksLimits, String> {
    Ok(StructuredDataLinksLimits {
        node_limit: bound(options, "nodeLimit", 1000000)?,
        byte_limit: bound(options, "byteLimit", 128 * 1024 * 1024)?,
        depth_limit: bound(options, "depthLimit", 256)?,
    })
}
fn string<'a>(row: &'a Value, key: &str) -> Result<&'a str, String> {
    row[key]
        .as_str()
        .ok_or_else(|| fail("invalid project source directory"))
}
fn list<'a>(row: &'a Value, key: &str) -> Result<&'a Vec<Value>, String> {
    row[key]
        .as_array()
        .ok_or_else(|| fail("invalid project source directory"))
}
fn text_values(values: &[Value]) -> Result<Vec<String>, String> {
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .ok_or_else(|| fail("invalid project source directory"))
        })
        .collect()
}
fn import_columns(values: &[Value]) -> Result<(Vec<String>, Vec<String>), String> {
    let mut specifiers = Vec::new();
    let mut providers = Vec::new();
    for row in values {
        if !row.as_object().is_some_and(|row| row.len() == 2) {
            return Err(fail("invalid project source directory"));
        }
        specifiers.push(string(row, "specifier")?.to_owned());
        providers.push(string(row, "identifier")?.to_owned());
    }
    Ok((specifiers, providers))
}
/// Draft products and retain every original complete CST, including failures.
/// This runs no generated product and supplies no behavioral evidence.
pub fn create_project_translation_archive(
    inputs: &Value,
    options: &Value,
) -> Result<MetaLanguageArchive, String> {
    let selected = limits(options)?;
    let draft = draft_project_translation(inputs, options)?;
    let packet = project_translation_draft_to_links(&draft, data_limits(options)?)?;
    let inventory = draft["inventory"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| {
            (
                file["identifier"].as_str().unwrap(),
                file["language"].as_str().unwrap(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let files=inputs.as_array().unwrap().iter().map(|row|{let path=row["identifier"].as_str().unwrap();json!({"path":path,"source":row["source"],"language":rules::read_project_translation_archive_source_language(inventory[path],path)})}).collect::<Vec<_>>();
    let originals = inputs
        .as_array()
        .unwrap()
        .iter()
        .map(|row| (row["identifier"].as_str().unwrap(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let configuration = crate::encode_structured_data_links_with_limits(
        &json!({"schemaVersion":1,"options":options,"modules":draft["inventory"].as_array().unwrap().iter().map(|file|json!({"identifier":file["identifier"],"language":file["language"],"imports":file["imports"],"sourceIssues":file["sourceIssues"],"options":originals[file["identifier"].as_str().unwrap()].get("options").filter(|value|!value.is_null()).cloned().unwrap_or_else(||json!({}))})).collect::<Vec<_>>()}),
        data_limits(options)?,
    )?;
    let sources = create_source_project_archive(&json!(files), options)?;
    create_meta_language_archive(
        &[
            MetaLanguageArchiveEntry {
                identifier: vocabulary("configuration"),
                kind: "language configuration".to_owned(),
                network: Arc::new(configuration.network),
                root: configuration.root,
            },
            MetaLanguageArchiveEntry {
                identifier: vocabulary("draft"),
                kind: "language configuration".to_owned(),
                network: Arc::new(packet.network),
                root: packet.root,
            },
            MetaLanguageArchiveEntry {
                identifier: vocabulary("sources"),
                kind: "concrete syntax".to_owned(),
                network: Arc::new(sources.network),
                root: sources.root,
            },
        ],
        selected,
    )
}
/// Complete parser-free source reconstruction plus the unchanged draft.
pub struct ProjectTranslationArchiveView {
    pub draft: Value,
    pub configuration: Value,
    pub concrete: SourceProjectArchiveView,
}
/// Reload without original files, a parser, a compiler or a current catalog.
/// Source and product hashes validate preservation, not semantic correctness.
pub fn read_project_translation_archive(
    network: &LinkNetwork,
    root: LinkId,
    options: &Value,
) -> Result<ProjectTranslationArchiveView, String> {
    let archive = read_meta_language_archive(network, root, limits(options)?)?;
    if archive.entries.len() != 3 {
        return Err(fail("missing or foreign project translation entry"));
    }
    let configuration_entry = archive
        .entries
        .iter()
        .find(|entry| entry.identifier == vocabulary("configuration"))
        .filter(|entry| entry.kind == "language configuration")
        .ok_or_else(|| fail("missing or foreign project translation entry"))?;
    let configuration = decode_structured_data_links_with_limits(
        &configuration_entry.network,
        configuration_entry.root,
        data_limits(options)?,
    )?;
    let draft_entry = archive
        .entries
        .iter()
        .find(|entry| entry.identifier == vocabulary("draft"))
        .filter(|entry| entry.kind == "language configuration")
        .ok_or_else(|| fail("missing or foreign project translation entry"))?;
    let source_entry = archive
        .entries
        .iter()
        .find(|entry| entry.identifier == vocabulary("sources"))
        .filter(|entry| entry.kind == "concrete syntax")
        .ok_or_else(|| fail("missing or foreign project translation entry"))?;
    let draft = decode_structured_data_links_with_limits(
        &draft_entry.network,
        draft_entry.root,
        data_limits(options)?,
    )?;
    let concrete = read_source_project_archive(&source_entry.network, source_entry.root, options)?;
    let inventory = list(&draft, "inventory")?;
    let modules = list(&draft, "modules")?;
    let error = rules::read_project_translation_archive_header_error(
        draft["schemaVersion"]
            .as_f64()
            .ok_or_else(|| fail("invalid project translation draft"))?,
        string(&draft, "interpretation")?,
        inventory.len() as f64,
        modules.len() as f64,
        concrete.sources.len() as f64,
    );
    if !error.is_empty() {
        return Err(fail(&error));
    }
    let settings = configuration["modules"]
        .as_array()
        .ok_or_else(|| fail("invalid project translation configuration"))?;
    if !configuration["options"].is_object() {
        return Err(fail("invalid project translation configuration"));
    }
    let selected = configuration["options"]["targetLanguage"]
        .as_str()
        .and_then(crate::self_translation_language)
        .unwrap_or("");
    let error = rules::read_project_translation_archive_configuration_error(
        configuration["schemaVersion"]
            .as_f64()
            .ok_or_else(|| fail("invalid project translation configuration"))?,
        settings.len() as f64,
        inventory.len() as f64,
        string(&draft, "targetLanguage")?,
        selected,
    );
    if !error.is_empty() {
        return Err(fail(&error));
    }
    for (index, file) in inventory.iter().enumerate() {
        let module = &modules[index];
        let source = &concrete.sources[index];
        let settings = &settings[index];
        if settings["identifier"] != file["identifier"]
            || settings["language"] != file["language"]
            || !settings["options"].is_object()
            || module["targetLanguage"] != draft["targetLanguage"]
        {
            return Err(fail("project translation configuration changed"));
        }

        for (row, language) in [(file, "language"), (module, "sourceLanguage")] {
            let error = rules::read_project_translation_archive_source_error(
                string(row, "identifier")?,
                string(row, language)?,
                row["sourceBytes"]
                    .as_f64()
                    .ok_or_else(|| fail("invalid project source directory"))?,
                string(row, "sourceSha256")?,
                &source.path,
                &source.language,
                source.bytes as f64,
                &source.sha256,
            );
            if !error.is_empty() {
                return Err(fail(&error));
            }
        }
        let gaps = list(module, "gaps")?;
        let issues = text_values(list(file, "sourceIssues")?)?;
        let actual_issues = text_values(list(module, "sourceIssues")?)?;
        let gap_values = gaps
            .iter()
            .filter(|gap| gap["diagnostic"]["name"] == "SourceInventoryGap")
            .map(|gap| gap["diagnostic"]["message"].clone())
            .collect::<Vec<_>>();
        let gap_issues = text_values(&gap_values)?;
        let (specifiers, providers) = import_columns(list(file, "imports")?)?;
        let (actual_specifiers, actual_providers) = import_columns(list(module, "imports")?)?;
        let error = rules::read_project_translation_archive_binding_error(
            &issues,
            &actual_issues,
            &gap_issues,
            &specifiers,
            &providers,
            &actual_specifiers,
            &actual_providers,
        );
        if !error.is_empty() {
            return Err(fail(&error));
        }
        let settings_issues = text_values(list(settings, "sourceIssues")?)?;
        let (settings_specifiers, settings_providers) = import_columns(list(settings, "imports")?)?;
        let error = rules::read_project_translation_archive_binding_error(
            &issues,
            &settings_issues,
            &gap_issues,
            &specifiers,
            &providers,
            &settings_specifiers,
            &settings_providers,
        );
        if !error.is_empty() {
            return Err(fail(&error));
        }
        let code = module
            .get("draftCode")
            .ok_or_else(|| fail("invalid project product"))?;
        let emitted = module
            .get("emittedCode")
            .ok_or_else(|| fail("invalid project product"))?;
        if !(code.is_null() || code.is_string()) || !(emitted.is_null() || emitted.is_string()) {
            return Err(fail("invalid project product"));
        }
        let digest = if let Some(code) = code.as_str() {
            module["emittedSha256"]
                .as_str()
                .is_some_and(|value| value == format!("{:x}", Sha256::digest(code.as_bytes())))
        } else {
            module.get("emittedSha256") == Some(&Value::Null)
        };
        let error = rules::read_project_translation_archive_product_error(
            !code.is_null(),
            !emitted.is_null(),
            gaps.len() as f64,
            emitted == code,
            digest,
        );
        if !error.is_empty() {
            return Err(fail(&error));
        }
    }
    Ok(ProjectTranslationArchiveView {
        draft,
        configuration,
        concrete,
    })
}
/// Exact process output, including the ordinary complete-network binary form.
pub struct ProjectTranslationArchiveCommandOutput {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: String,
}
/// Transport a Links source inventory into its complete unexecuted draft archive.
pub fn run_project_translation_archive_command<F>(
    args: &[String],
    read_file: F,
) -> ProjectTranslationArchiveCommandOutput
where
    F: Fn(&str) -> Result<String, String>,
{
    const USAGE: &str = "usage: meta-language project-translation <inventory.lino> --to <language> [--abstract] [--binary]\nDrafts every source and preserves complete original CSTs, products and diagnostics as Links. AST is an optional view. No generated program is executed.\n";
    let plan = rules::plan_project_translation_archive_command(args);
    if plan == [-1.0] {
        return ProjectTranslationArchiveCommandOutput {
            exit_code: 0,
            stdout: USAGE.as_bytes().to_vec(),
            stderr: String::new(),
        };
    }
    if plan.len() != 4 {
        return ProjectTranslationArchiveCommandOutput {
            exit_code: 2,
            stdout: Vec::new(),
            stderr: format!("error: invalid project translation arguments\n{USAGE}"),
        };
    }
    let result = (|| -> Result<Vec<u8>, String> {
        let argument = |value: f64| -> Result<&str, String> {
            let index = value
                .to_string()
                .parse::<usize>()
                .map_err(|_| fail("invalid shared command plan"))?;
            args.get(index)
                .map(String::as_str)
                .ok_or_else(|| fail("invalid shared command plan"))
        };
        let input = read_file(argument(plan[0])?)?;
        let value = crate::read_structured_data_lino(&input)?;
        if !value.is_object()
            || !value["inputs"].is_array()
            || value
                .get("options")
                .is_some_and(|options| !options.is_null() && !options.is_object())
        {
            return Err(fail("expected a source inventory record"));
        }
        let mut options = value
            .get("options")
            .filter(|value| !value.is_null())
            .cloned()
            .unwrap_or_else(|| json!({}));
        options["targetLanguage"] = json!(argument(plan[1])?);
        options["projection"] = json!(if plan[2] == 1.0 {
            "AbstractSyntax"
        } else {
            "ConcreteSyntax"
        });
        let packet = create_project_translation_archive(&value["inputs"], &options)?;
        if plan[3] == 1.0 {
            crate::meta_language_archive_to_binary(&packet, limits(&options)?)
        } else {
            crate::meta_language_archive_to_text(&packet, limits(&options)?).map(String::into_bytes)
        }
    })();
    match result {
        Ok(stdout) => ProjectTranslationArchiveCommandOutput {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        },
        Err(error) => ProjectTranslationArchiveCommandOutput {
            exit_code: 1,
            stdout: Vec::new(),
            stderr: format!("error: {error}\n"),
        },
    }
}
