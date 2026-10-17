//! Owned source evidence for the standard field-free datatype Clone derivation.
use super::diagnostics::{Result, type_error, unsupported};
use super::ir::Program;
use super::source_type_namespaces::policy;
use super::surface::{SData, SItem, SProgram};
use serde_json::{Value, json};

fn fail() -> super::diagnostics::TranslationError {
    type_error(
        "an owned Rust unit Clone derivation has inconsistent actual datatype or attribute facts",
        None,
    )
}
fn exact(value: &Value, fields: &[&str]) -> bool {
    value.as_object().is_some_and(|record| {
        record.len() == fields.len() && fields.iter().all(|field| record.contains_key(*field))
    })
}
fn policy_fields(name: &str) -> Vec<&str> {
    policy()[name]
        .as_array()
        .unwrap()
        .iter()
        .map(|field| field.as_str().unwrap())
        .collect()
}

/// Validate actual datatype and derive facts before record normalization.
pub fn validate_unit_clone_declaration_record(declaration: &Value) -> Result<()> {
    let proof = &declaration["sourceUnitCloneDerivation"];
    let path = declaration["modulePath"].as_array().ok_or_else(fail)?;
    let constructors = declaration["ctors"].as_array().ok_or_else(fail)?;
    let mut owner = path.clone();
    owner.push(declaration["name"].clone());
    if !exact(proof, &policy_fields("unitCloneFields"))
        || proof["model"] != policy()["unitCloneModel"]
        || proof["language"] != policy()["language"]
        || declaration["k"] != "data"
        || proof["datatypeOwner"] != json!(owner)
        || proof["traitOwner"] != policy()["unitCloneTraitOwner"]
        || proof["constructorNames"]
            != json!(
                constructors
                    .iter()
                    .map(|constructor| &constructor["name"])
                    .collect::<Vec<_>>()
            )
        || constructors.iter().any(|constructor| {
            constructor["fields"]
                .as_array()
                .is_none_or(|fields| !fields.is_empty())
        })
    {
        return Err(fail());
    }
    let attribute = &proof["attribute"];
    let traits = attribute["traits"].as_array().ok_or_else(fail)?;
    if !exact(attribute, &policy_fields("unitCloneAttributeFields"))
        || attribute["name"] != "derive"
        || traits.iter().any(|name| {
            !policy()["unitCloneDeriveArguments"]
                .as_array()
                .unwrap()
                .contains(name)
        })
        || traits
            .iter()
            .enumerate()
            .any(|(index, name)| traits[..index].contains(name))
        || traits.iter().filter(|name| *name == "Clone").count() != 1
    {
        return Err(fail());
    }
    for name in traits {
        if policy()["unitCloneDeriveDependencies"][name.as_str().ok_or_else(fail)?]
            .as_array()
            .is_some_and(|required| required.iter().any(|name| !traits.contains(name)))
        {
            return Err(fail());
        }
    }
    let span = &attribute["span"];
    let start = span["start"].as_u64().ok_or_else(fail)?;
    let end = span["end"].as_u64().ok_or_else(fail)?;
    if !exact(span, &["start", "end"])
        || start > 9_007_199_254_740_991
        || end > 9_007_199_254_740_991
        || end <= start
        || declaration["span"]["start"]
            .as_u64()
            .is_none_or(|start| end > start)
    {
        return Err(fail());
    }
    Ok(())
}

/// Require the closed source context for every present owned derivation.
pub fn validate_unit_clone_records(value: &Value) -> Result<()> {
    let Some(declarations) = value["declarations"].as_array() else {
        return Ok(());
    };
    for declaration in declarations
        .iter()
        .filter(|entry| entry.get("sourceUnitCloneDerivation").is_some())
    {
        if value["sourceLanguage"] != "Rust"
            || value
                .get("imports")
                .is_some_and(|value| value.as_array().is_none_or(|items| !items.is_empty()))
            || value
                .get("externals")
                .is_some_and(|value| value.as_array().is_none_or(|items| !items.is_empty()))
        {
            return Err(fail());
        }
        validate_unit_clone_declaration_record(declaration)?;
    }
    Ok(())
}

/// Validate source-free derivations before general constructor expression checks.
pub fn validate_source_unit_clone_derivations(program: &Program) -> Result<()> {
    validate_unit_clone_records(&serde_json::to_value(program).map_err(|_| fail())?)
}

/// Capture the actual one-attribute, one-Clone unit derivation without source text.
pub fn capture_unit_clone_derivation(
    attributes: &[Value],
    declaration: &SData,
    path: &[String],
) -> Result<Option<Value>> {
    let occurrences = attributes
        .iter()
        .filter_map(|attribute| attribute["traits"].as_array())
        .flat_map(|traits| traits.iter())
        .filter(|name| *name == "Clone")
        .count();
    if occurrences == 0 {
        return Ok(None);
    }
    if attributes.len() != 1 || occurrences != 1 {
        return Err(unsupported(
            "Rust unit Clone derivation",
            "one actual attached derive attribute with one Clone argument is required",
            None,
        ));
    }
    if declaration
        .ctors
        .iter()
        .any(|constructor| !constructor.fields.is_empty())
    {
        return Ok(None);
    }
    let owner = path
        .iter()
        .chain(std::iter::once(&declaration.name))
        .collect::<Vec<_>>();
    let proof = json!({"model":policy()["unitCloneModel"],"language":policy()["language"],
        "datatypeOwner":owner,"traitOwner":policy()["unitCloneTraitOwner"],
        "constructorNames":declaration.ctors.iter().map(|constructor| &constructor.name).collect::<Vec<_>>(),
        "attribute":attributes[0]});
    let mut raw = serde_json::to_value(declaration).map_err(|_| fail())?;
    raw["k"] = json!("data");
    raw["modulePath"] = json!(path);
    raw["sourceUnitCloneDerivation"] = proof.clone();
    validate_unit_clone_declaration_record(&raw)?;
    Ok(Some(proof))
}

/// Inspect the actual surface datatype before constructor or function checking.
pub fn validate_surface_unit_clone_derivations(program: &SProgram) -> Result<()> {
    let mut pending = program
        .items
        .iter()
        .map(|item| (item, Vec::<String>::new()))
        .collect::<Vec<_>>();
    while let Some((item, path)) = pending.pop() {
        match item {
            SItem::Module(module) => {
                let mut inner = path;
                inner.push(module.name.clone());
                pending.extend(module.items.iter().map(|item| (item, inner.clone())));
            }
            SItem::Data(data) if data.source_unit_clone_derivation.is_some() => {
                if program.language.as_str() != "Rust"
                    || !program.imports.is_empty()
                    || !program.externals.is_empty()
                {
                    return Err(fail());
                }
                let mut raw = serde_json::to_value(data).map_err(|_| fail())?;
                raw["k"] = json!("data");
                raw["modulePath"] = json!(path);
                validate_unit_clone_declaration_record(&raw)?;
            }
            _ => {}
        }
    }
    Ok(())
}
