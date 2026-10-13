//! Opt-in value interpretation for already checked source-free functions.
use crate::formal_concept_definition::{
    FormalConceptArchive, FormalConceptDefinitionInput, FormalConceptDefinitionOptions,
    encode_formal_concept_definition,
};
use crate::formal_function_links::{FormalFunctionSelector, encode_formal_function_definition};
use crate::shared_numeric_function_model::{
    SHARED_NUMERIC_FUNCTION_MODEL, normalize_shared_numeric_function_literals,
    validate_shared_numeric_origin_context,
};
use crate::{LinkId, LinkNetwork};
use serde_json::{Map, Value};

/// Select a closed binary64/Boolean function under the explicit shared model.
/// Source language remains inspectable origin provenance, after model guards.
///
/// # Errors
/// Refuses every operation, type or context without a declared interpretation.
pub fn encode_shared_numeric_function_definition(
    network: &LinkNetwork,
    root: LinkId,
    declaration: FormalFunctionSelector<'_>,
    provenance: Option<&Value>,
) -> Result<FormalConceptArchive, String> {
    validate_shared_numeric_origin_context(network, root)?;
    let selected = encode_formal_function_definition(network, root, declaration, None)?;
    let typed_root = selected
        .network
        .link(selected.root)
        .ok_or("missing selected function definition")?
        .references()[0];
    let model = normalize_shared_numeric_function_literals(&selected.network, typed_root)?;
    let mut origin = Map::new();
    origin.insert(
        "sourceLanguage".to_owned(),
        Value::String(model.source_language),
    );
    if let Some(provenance) = provenance {
        origin.insert("userProvenance".to_owned(), provenance.clone());
    }
    encode_formal_concept_definition(
        FormalConceptDefinitionInput::LinkedProgram {
            network: &selected.network,
            root: typed_root,
        },
        FormalConceptDefinitionOptions {
            model: SHARED_NUMERIC_FUNCTION_MODEL.to_owned(),
            provenance: Some(Value::Object(origin)),
        },
    )
}
