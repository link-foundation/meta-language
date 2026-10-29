//! Complete supplied source inventories; products are drafted, never executed.
//! Shared worklist decisions are automatically translated from JavaScript.
use crate::project_translation_rules::{
    accept_project_translation_item, accept_project_translation_source_family,
    compare_project_translation_identifiers, ml_string::Text,
    plan_project_translation_dependency_gaps, read_project_translation_frontier,
};
use crate::{
    SelfTranslationOptions, SelfTranslationSignature, StructuredDataLinks,
    StructuredDataLinksLimits, encode_structured_data_links_with_limits,
    self_translate_with_diagnostics, self_translation_language, self_translation_signatures,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

mod settings;

#[derive(Clone)]
struct Source {
    identifier: String,
    language: &'static str,
    source: String,
    source_sha256: String,
    imports: Vec<(String, String)>,
    options: Value,
    source_issues: Vec<String>,
}
fn fail(message: &str) -> String {
    format!("project translation: {message}")
}
fn digest(source: &str) -> String {
    format!("{:x}", Sha256::digest(source.as_bytes()))
}
fn text(value: &str) -> Text {
    Text::from_text(value)
}
fn limit(options: &Value, key: &str, default: usize) -> Result<usize, String> {
    match options.get(key) {
        None => Ok(default),
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
            .ok_or_else(|| fail("invalid drafting resource limit")),
    }
}
fn sources(inputs: &Value, options: &Value) -> Result<Vec<Source>, String> {
    let inputs = inputs
        .as_array()
        .ok_or_else(|| fail("expected a bounded nonempty complete module inventory"))?;
    if inputs.is_empty() || inputs.len() > limit(options, "moduleLimit", 10000)? {
        return Err(fail(
            "expected a bounded nonempty complete module inventory",
        ));
    }
    let mut seen = BTreeSet::new();
    let mut bytes = 0usize;
    let mut issue_bytes = 0usize;
    let mut records = Vec::new();
    for input in inputs {
        let identifier = input["identifier"]
            .as_str()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| fail("invalid or duplicate source module"))?;
        let source = input["source"]
            .as_str()
            .ok_or_else(|| fail("invalid or duplicate source module"))?;
        if !seen.insert(identifier.to_owned()) {
            return Err(fail("invalid or duplicate source module"));
        }
        let language = input["language"]
            .as_str()
            .and_then(self_translation_language)
            .ok_or_else(|| fail(&format!("unknown source language for {identifier}")))?;
        bytes = bytes
            .checked_add(source.len())
            .ok_or_else(|| fail("source byte resource limit exceeded"))?;
        if bytes > limit(options, "byteLimit", 128 * 1024 * 1024)? {
            return Err(fail("source byte resource limit exceeded"));
        }
        let mut source_issues = Vec::new();
        if let Some(issues) = input.get("sourceIssues").filter(|value| !value.is_null()) {
            let issues = issues
                .as_array()
                .filter(|issues| issues.len() <= 10000)
                .ok_or_else(|| fail("invalid source inventory issues"))?;
            for issue in issues {
                let issue = issue
                    .as_str()
                    .filter(|issue| !issue.is_empty())
                    .ok_or_else(|| fail("invalid source inventory issues"))?;
                issue_bytes = issue_bytes
                    .checked_add(issue.len())
                    .ok_or_else(|| fail("source issue byte resource limit exceeded"))?;
                if issue_bytes > limit(options, "byteLimit", 128 * 1024 * 1024)? {
                    return Err(fail("source issue byte resource limit exceeded"));
                }
                source_issues.push(issue.to_owned());
            }
        }
        let mut imports = Vec::new();
        if let Some(values) = input.get("imports").filter(|value| !value.is_null()) {
            for (specifier, provider) in values
                .as_object()
                .ok_or_else(|| fail("invalid import directory"))?
            {
                let provider = provider
                    .as_str()
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| fail("invalid imported source identity"))?;
                imports.push((specifier.clone(), provider.to_owned()));
            }
        }
        records.push(Source {
            identifier: identifier.to_owned(),
            language,
            source: source.to_owned(),
            source_sha256: digest(source),
            imports,
            source_issues,
            options: input
                .get("options")
                .filter(|value| !value.is_null())
                .cloned()
                .unwrap_or_else(|| json!({})),
        });
    }
    records.sort_by(|first, second| {
        let comparison = compare_project_translation_identifiers(
            &text(&first.identifier),
            &text(&second.identifier),
        );
        if comparison < 0.0 {
            std::cmp::Ordering::Less
        } else if comparison > 0.0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    for record in &records {
        for (_, dependency) in &record.imports {
            if !seen.contains(dependency) {
                return Err(fail(&format!(
                    "missing source {dependency} imported by {}",
                    record.identifier
                )));
            }
        }
    }
    Ok(records)
}
fn diagnostic(message: &str) -> Value {
    json!({"name":"SelfTranslationError","message":message})
}
fn context(
    record: &Source,
    signatures: &BTreeMap<String, Vec<SelfTranslationSignature>>,
    options: &Value,
) -> Result<SelfTranslationOptions, String> {
    let mut result = SelfTranslationOptions::default();
    if let Some(common) = options
        .get("translationOptions")
        .filter(|value| !value.is_null())
    {
        settings::apply(&mut result, common)?;
    }
    settings::apply(&mut result, &record.options)?;
    result.imports.clear();
    if let Some(externals) = options
        .get("externalImports")
        .and_then(|value| value.get(&record.identifier))
    {
        result.imports =
            serde_json::from_value(externals.clone()).map_err(|error| fail(&error.to_string()))?;
    }
    for (specifier, identifier) in &record.imports {
        if result.imports.contains_key(specifier) {
            return Err(fail(
                "an internal source dependency cannot be replaced by an external signature",
            ));
        }
        result
            .imports
            .insert(specifier.clone(), signatures[identifier].clone());
    }
    result.semantic_translation = true;
    Ok(result)
}

/// Draft every supplied source module and retain every unresolved owner.
///
/// Neither source restoration nor a carried envelope is executable translation.
/// Dependencies refine only from actual ordinary compiler publication.
///
/// # Errors
/// Refuses malformed inventories, absent sources and invalid explicit budgets.
pub fn draft_project_translation(inputs: &Value, options: &Value) -> Result<Value, String> {
    let target = options["targetLanguage"]
        .as_str()
        .and_then(self_translation_language)
        .ok_or_else(|| fail("an explicit supported target language is required"))?;
    for name in [
        "moduleLimit",
        "byteLimit",
        "refinementLimit",
        "dependencyWorkLimit",
    ] {
        limit(options, name, 1)?;
    }
    let records = sources(inputs, options)?;
    let identifiers: Vec<_> = records
        .iter()
        .map(|record| text(&record.identifier))
        .collect();
    let mut importers = Vec::new();
    let mut providers = Vec::new();
    for record in &records {
        for (_, provider) in &record.imports {
            importers.push(text(&record.identifier));
            providers.push(text(provider));
        }
    }
    let mut signatures: BTreeMap<String, Vec<SelfTranslationSignature>> = records
        .iter()
        .map(|record| (record.identifier.clone(), Vec::new()))
        .collect();
    let mut pending: BTreeSet<_> = records
        .iter()
        .map(|record| record.identifier.clone())
        .collect();
    let refinement_limit = limit(options, "refinementLimit", records.len() + 1)?;
    let mut passes = Vec::new();
    let mut stable = false;
    for pass in 0..refinement_limit {
        let mut next = signatures.clone();
        let mut refusals = Vec::new();
        for record in &records {
            if !pending.contains(&record.identifier) {
                continue;
            }
            let result = context(record, &signatures, options).and_then(|context| {
                self_translation_signatures(&record.source, record.language, &context)
                    .map_err(|error| error.to_string())
            });
            match result {
                Ok(published) => {
                    next.insert(record.identifier.clone(), published);
                }
                Err(error) => {
                    next.insert(record.identifier.clone(), Vec::new());
                    refusals.push(
                        json!({"identifier":record.identifier,"diagnostic":diagnostic(&error)}),
                    );
                }
            }
        }
        let changed: Vec<_> = records
            .iter()
            .filter(|record| signatures[&record.identifier] != next[&record.identifier])
            .map(|record| record.identifier.clone())
            .collect();
        passes.push(json!({"pass":pass,"changed":changed,"refusals":refusals}));
        signatures = next;
        if changed.is_empty() {
            stable = true;
            break;
        }
        let changed_text: Vec<_> = changed.iter().map(|value| text(value)).collect();
        pending =
            read_project_translation_frontier(&identifiers, &changed_text, &importers, &providers)
                .iter()
                .map(|value| {
                    String::from_utf16(&value.0)
                        .map_err(|_| fail("invalid native worklist identity"))
                })
                .collect::<Result<_, _>>()?;
        if pending.is_empty() {
            stable = true;
            break;
        }
    }
    let mut modules = Vec::new();
    let mut inventory = Vec::new();
    for record in &records {
        let imports: Vec<_> = record
            .imports
            .iter()
            .map(|(specifier, identifier)| json!({"specifier":specifier,"identifier":identifier}))
            .collect();
        inventory.push(json!({"identifier":record.identifier,"language":record.language,"sourceSha256":record.source_sha256,
            "sourceBytes":record.source.len(),"imports":imports,"sourceIssues":record.source_issues}));
        let mut module = json!({"identifier":record.identifier,"sourceLanguage":record.language,"targetLanguage":target,
            "sourceSha256":record.source_sha256,"sourceBytes":record.source.len(),"imports":imports,"sourceIssues":record.source_issues,"signatures":signatures[&record.identifier]});
        let product = context(record, &signatures, options).and_then(|context| {
            self_translate_with_diagnostics(&record.source, record.language, target, &context)
                .map_err(|error| error.to_string())
        });
        match product {
            Ok(report) => {
                let same_family =
                    accept_project_translation_source_family(&text(record.language), &text(target));
                let mut items = Vec::new();
                let mut gaps = record.source_issues.iter().map(|message|json!({"diagnostic":{"name":"SourceInventoryGap","message":message}})).collect::<Vec<_>>();
                for (index, item) in report.translation.items.iter().enumerate() {
                    let mut value = json!({"term":item.term,"start":item.start,"end":item.end,"status":item.status,"reason":item.reason});
                    if let Some(Some(diagnostic)) = report.diagnostics.get(index) {
                        value["diagnostic"] = diagnostic.clone();
                    }
                    if !accept_project_translation_item(&text(item.status), same_family) {
                        gaps.push(value.clone());
                    }
                    items.push(value);
                }
                if !stable {
                    gaps.push(json!({"diagnostic":{"name":"DependencyRefinementGap", "message":"Source signature refinement did not converge within the explicit budget."}}));
                }
                module["emittedCode"] = if gaps.is_empty() {
                    json!(report.translation.code)
                } else {
                    Value::Null
                };
                module["items"] = json!(items);
                module["gaps"] = json!(gaps);
                module["draftCode"] = json!(report.translation.code);
                module["emittedSha256"] = json!(digest(&report.translation.code));
            }
            Err(error) => {
                module["items"] = json!([]);
                let mut gaps=record.source_issues.iter().map(|message|json!({"diagnostic":{"name":"SourceInventoryGap","message":message}})).collect::<Vec<_>>();
                gaps.push(json!({"diagnostic":diagnostic(&error)}));
                module["gaps"] = json!(gaps);
                module["emittedCode"] = Value::Null;
                module["draftCode"] = Value::Null;
                module["emittedSha256"] = Value::Null;
            }
        }
        modules.push(module);
    }
    let positions = records
        .iter()
        .enumerate()
        .map(|(index, record)| (record.identifier.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    let failed = modules
        .iter()
        .enumerate()
        .filter(|(_, module)| !module["gaps"].as_array().unwrap().is_empty())
        .map(|(index, _)| index as f64)
        .collect::<Vec<_>>();
    let mut importer_indices = Vec::new();
    let mut provider_indices = Vec::new();
    for record in &records {
        for (_, provider) in &record.imports {
            importer_indices.push(positions[record.identifier.as_str()] as f64);
            provider_indices.push(positions[provider.as_str()] as f64);
        }
    }
    let witnesses = plan_project_translation_dependency_gaps(
        records.len() as f64,
        &failed,
        &importer_indices,
        &provider_indices,
        limit(options, "dependencyWorkLimit", 1000000)? as f64,
    );
    if witnesses == [-1.0] {
        for module in &mut modules {
            module["gaps"].as_array_mut().unwrap().push(json!({"diagnostic":{"name":"DependencyWorkLimitGap","message":"Complete dependency failure propagation exceeded its explicit work budget."}}));
            module["emittedCode"] = Value::Null;
        }
    } else {
        for witness in witnesses.chunks_exact(3) {
            let index = witness[0] as usize;
            let provider = &records[witness[1] as usize].identifier;
            let root = &records[witness[2] as usize].identifier;
            modules[index]["gaps"].as_array_mut().unwrap().push(json!({"diagnostic":{"name":"DependencySourceGap","message":"A source dependency has an unresolved inventory or translation gap.","provider":provider,"root":root}}));
            modules[index]["emittedCode"] = Value::Null;
        }
    }
    Ok(
        json!({"schemaVersion":1,"interpretation":"independent source semantic translation","targetLanguage":target,
        "inventory":inventory,"dependencyRefinement":{"stable":stable,"limit":refinement_limit,"passes":passes},"modules":modules}),
    )
}

/// Store an actual draft as inspectable Links data, without recording evidence.
///
/// # Errors
/// Returns ordinary structured-data profile and explicit-budget errors.
pub fn project_translation_draft_to_links(
    draft: &Value,
    limits: StructuredDataLinksLimits,
) -> Result<StructuredDataLinks, String> {
    encode_structured_data_links_with_limits(draft, limits)
}
