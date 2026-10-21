//! Linked operational definitions and closed equivalence certificates.
//! Primitive declarations are shared data. Equality decisions reuse the
//! verifier automatically translated from JavaScript; this adapter preserves
//! network identities, ordered references and labeled relations.

use std::collections::BTreeSet;
use std::sync::OnceLock;

use serde_json::{Value, json};

use crate::link_network::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::shared_numeric_function_model::{
    BOOLEAN_FUNCTION_DECLARATION_VALUE_MODEL, SHARED_BOOLEAN_FUNCTION_MODEL,
    SHARED_NUMERIC_FUNCTION_MODEL, create_shared_numeric_function_meaning,
    read_boolean_function_declaration_value_model, read_shared_boolean_function_model,
    read_shared_numeric_function_model,
};
use crate::translation::boolean_declaration_value_rules::accept_boolean_declaration_value_selector;
use crate::translation::ir::Program;
use crate::typed_program_links::{
    emit_typed_program_links, encode_typed_program_links_with_provenance,
    read_typed_program_binding_identities,
};

mod graph;
mod relation_rules;
use graph::{
    Graph, correspondence, data, exclude_binding_names, fields, graph, snapshot, verify_pairs,
};

const LANGUAGE: &str = "meta-language formal definition";
const SCHEMA: &str = "1";
const ROOT: &str = "formal concept definition";
const CERTIFICATE: &str = "formal equivalence certificate";
const PAIRS: &str = "formal node correspondence list";
const PAIR: &str = "formal node correspondence";

/// A linked definition or certificate with its explicit root identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormalConceptArchive {
    /// Complete graph, including shared identities and cyclic references.
    pub network: LinkNetwork,
    /// Root of the definition or certificate.
    pub root: LinkId,
}

impl FormalConceptArchive {
    /// Restore a serialized archive without interpreting its proof premises.
    ///
    /// # Errors
    /// Refuses malformed links or a missing numeric root.
    pub fn from_json(value: &Value) -> Result<Self, String> {
        let links = value["links"]
            .as_str()
            .ok_or("a formal definition needs a linked archive and a root")?;
        let root = value["root"]
            .as_u64()
            .ok_or("a formal definition needs a linked archive and a root")?;
        Ok(Self {
            network: LinkNetwork::from_lino(links).map_err(|error| error.to_string())?,
            root: LinkId::from_u64(root),
        })
    }

    /// Serialize the full graph and its root, preserving identities.
    #[must_use]
    pub fn to_json(&self) -> Value {
        json!({"links": self.network.to_lino(), "root": self.root.as_u64()})
    }
}

/// Host inputs for the same three supported linked interpretation models.
#[derive(Clone, Copy)]
pub enum FormalConceptDefinitionInput<'a> {
    /// An operational program already checked by the portable core.
    Program(&'a Program),
    /// An existing typed-program graph, rechecked before accepting its basis.
    LinkedProgram {
        network: &'a LinkNetwork,
        root: LinkId,
    },
    /// Exact shared source primitive declaration, or its implicit declaration.
    Declaration(Option<&'a Value>),
}

/// Supported interpretation model and separately inspectable provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormalConceptDefinitionOptions {
    /// Exact key in the shared primitive declarations.
    pub model: String,
    /// Provenance recorded as linked data but excluded from semantic equality.
    pub provenance: Option<Value>,
}

impl Default for FormalConceptDefinitionOptions {
    fn default() -> Self {
        Self {
            model: "checked typed program".to_owned(),
            provenance: None,
        }
    }
}

impl From<&str> for FormalConceptDefinitionOptions {
    fn from(model: &str) -> Self {
        Self {
            model: model.to_owned(),
            provenance: None,
        }
    }
}

fn models() -> &'static Value {
    static MODELS: OnceLock<Value> = OnceLock::new();
    MODELS.get_or_init(|| {
        crate::structured_data_links::read_structured_data_lino(include_str!(
            "data/formal-concept-models.lino"
        ))
        .expect("shared formal primitive declarations are Links Notation")
    })
}

fn metadata(term: &str, definition: Option<&str>) -> LinkMetadata {
    let value = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_language(LANGUAGE)
        .with_named(true)
        .with_term(term);
    if let Some(definition) = definition {
        value.with_definition(definition)
    } else {
        value
    }
}

struct Checked {
    model: String,
    graph: Graph,
}

fn checked(value: &FormalConceptArchive) -> Result<Checked, String> {
    let network = &value.network;
    let wrapper = network
        .link(value.root)
        .ok_or("unsupported or missing formal definition basis")?;
    let meta = wrapper.metadata();
    if meta.language() != Some(LANGUAGE)
        || meta.term() != Some(ROOT)
        || meta.definition() != Some(SCHEMA)
        || wrapper.references().len() != 2
    {
        return Err("unsupported or missing formal definition basis".to_owned());
    }
    let body = wrapper.references()[0];
    let basis = wrapper.references()[1];
    let model_link = network
        .link(basis)
        .ok_or("unsupported or counterfeit formal primitive model")?;
    let model = model_link
        .metadata()
        .definition()
        .ok_or("unsupported or counterfeit formal primitive model")?;
    let declaration = models()
        .get(model)
        .ok_or("unsupported or counterfeit formal primitive model")?;
    if model_link.metadata().language() != Some(LANGUAGE)
        || model_link.metadata().term() != Some("formal primitive model")
        || model_link.references().len() != 1
    {
        return Err("unsupported or counterfeit formal primitive model".to_owned());
    }
    let mut expected = LinkNetwork::new();
    let expected_root = data(&mut expected, declaration)?;
    let expected_graph = graph(&expected, expected_root, &BTreeSet::default())?;
    if correspondence(
        &graph(network, model_link.references()[0], &BTreeSet::default())?,
        &expected_graph,
    )
    .is_none()
    {
        return Err("the formal primitive model differs from its supported declaration".to_owned());
    }
    let mut exclusions = BTreeSet::from([(value.root, "provenance".to_owned())]);
    let mut meaning = None;
    if model == "checked typed program"
        || model == SHARED_NUMERIC_FUNCTION_MODEL
        || model == SHARED_BOOLEAN_FUNCTION_MODEL
        || model == BOOLEAN_FUNCTION_DECLARATION_VALUE_MODEL
    {
        let bindings = read_typed_program_binding_identities(network, body)
            .map_err(|error| error.to_string())?;
        // Supported operations must have both actual target interpretations.
        // The existing adapters diagnose unknown constructs, rather than
        // granting equality from matching but uninterpretable host payloads.
        emit_typed_program_links(network, body, "JavaScript").map_err(|error| error.to_string())?;
        emit_typed_program_links(network, body, "Rust").map_err(|error| error.to_string())?;
        for (parent, relations) in fields(network) {
            for relation in relations {
                if relation.label == "source span"
                    || (parent == body && relation.label == "provenance")
                {
                    exclusions.insert((parent, relation.label.clone()));
                }
            }
        }
        // Alpha equivalence applies only to actual owned binders established
        // by the checked scope walk. Unreachable relation links cannot mark
        // semantic field names or public declarations as lexical bindings.
        exclude_binding_names(network, body, &mut exclusions, &bindings)?;
        if model == SHARED_NUMERIC_FUNCTION_MODEL
            || model == SHARED_BOOLEAN_FUNCTION_MODEL
            || model == BOOLEAN_FUNCTION_DECLARATION_VALUE_MODEL
        {
            let declaration_value = model == BOOLEAN_FUNCTION_DECLARATION_VALUE_MODEL;
            let boolean_value = model == SHARED_BOOLEAN_FUNCTION_MODEL || declaration_value;
            let shared = if declaration_value {
                let entries = fields(network).remove(&value.root).unwrap_or_default();
                let selectors: Vec<_> = entries
                    .iter()
                    .filter(|field| field.label == "selected function declaration")
                    .collect();
                if !accept_boolean_declaration_value_selector(selectors.len() == 1, true) {
                    return Err(
                        "the direct declaration basis requires exactly one owned entry selector"
                            .to_owned(),
                    );
                }
                let shared = read_boolean_function_declaration_value_model(
                    network,
                    body,
                    selectors[0].child,
                )?;
                exclusions.insert((value.root, "selected function declaration".to_owned()));
                exclusions.extend(shared.qualification_fields.iter().cloned());
                shared
            } else if boolean_value {
                read_shared_boolean_function_model(network, body)?
            } else {
                read_shared_numeric_function_model(network, body, false)?
            };
            let mut view = create_shared_numeric_function_meaning(network, &shared)?;
            if boolean_value {
                // The explicit callable observation retains names in the archive.
                for id in &shared.declaration_ids {
                    exclusions.insert((*id, "name".to_owned()));
                    exclusions.insert((*id, "full name".to_owned()));
                }
                let mut references = wrapper.references().to_vec();
                references[0] = shared.entry_target;
                if !view.set_references(value.root, &references) {
                    return Err("missing Boolean callable meaning root".to_owned());
                }
            }
            meaning = Some(view);
            exclusions.extend(shared.signature_fields);
            exclusions.insert((shared.language_parent, shared.language_label));
        }
    } else if correspondence(
        &graph(network, body, &BTreeSet::default())?,
        &expected_graph,
    )
    .is_none()
    {
        return Err(
            "a source interpretation differs from its declared primitive meaning".to_owned(),
        );
    }
    Ok(Checked {
        model: model.to_owned(),
        graph: graph(meaning.as_ref().unwrap_or(network), value.root, &exclusions)?,
    })
}

/// Copy an operational definition with its exact supported shared basis.
///
/// # Errors
/// Refuses unknown models, counterfeit primitive data or unsupported programs.
pub fn encode_formal_concept_definition(
    value: FormalConceptDefinitionInput<'_>,
    options: impl Into<FormalConceptDefinitionOptions>,
) -> Result<FormalConceptArchive, String> {
    let options = options.into();
    let model = options.model.as_str();
    if model == BOOLEAN_FUNCTION_DECLARATION_VALUE_MODEL {
        return Err(
            "the direct declaration basis requires an actual owned entry selector".to_owned(),
        );
    }
    let declaration = models()
        .get(model)
        .ok_or("unsupported formal primitive model")?;
    let (mut network, body) = if model == "checked typed program"
        || model == SHARED_NUMERIC_FUNCTION_MODEL
        || model == SHARED_BOOLEAN_FUNCTION_MODEL
    {
        match value {
            FormalConceptDefinitionInput::Program(program) => {
                let archive = encode_typed_program_links_with_provenance(
                    program,
                    options.provenance.as_ref(),
                )
                .map_err(|error| error.to_string())?;
                (archive.network, archive.root)
            }
            FormalConceptDefinitionInput::LinkedProgram { network, root } => {
                (network.clone(), root)
            }
            FormalConceptDefinitionInput::Declaration(_) => {
                return Err("a checked model needs a typed operational program".to_owned());
            }
        }
    } else {
        let FormalConceptDefinitionInput::Declaration(value) = value else {
            return Err("a source primitive has no declared formal interpretation".to_owned());
        };
        if value.is_some_and(|value| value != declaration) {
            return Err("a source primitive has no declared formal interpretation".to_owned());
        }
        let mut network = LinkNetwork::new();
        let body = data(&mut network, declaration)?;
        (network, body)
    };
    let declaration_root = data(&mut network, declaration)?;
    let basis = network.insert_link(
        [declaration_root],
        metadata("formal primitive model", Some(model)),
    );
    let root = network.insert_link([body, basis], metadata(ROOT, Some(SCHEMA)));
    if let Some(provenance) = options.provenance.as_ref() {
        let provenance = data(&mut network, provenance)?;
        network.insert_field(root, "provenance", provenance);
    }
    let result = FormalConceptArchive { network, root };
    checked(&result)?;
    Ok(result)
}

// Private graph constructor for the additive declaration-value entry API.
// The public options type and original wrapper constructor remain unchanged.
pub(crate) fn encode_formal_function_declaration_context(
    network: &LinkNetwork,
    typed_root: LinkId,
    entry_declaration: LinkId,
    provenance: Option<&Value>,
) -> Result<FormalConceptArchive, String> {
    let mut network = network.clone();
    let model = BOOLEAN_FUNCTION_DECLARATION_VALUE_MODEL;
    let declaration = models()
        .get(model)
        .ok_or("unsupported formal primitive model")?;
    let declaration_root = data(&mut network, declaration)?;
    let basis = network.insert_link(
        [declaration_root],
        metadata("formal primitive model", Some(model)),
    );
    let root = network.insert_link([typed_root, basis], metadata(ROOT, Some(SCHEMA)));
    network.insert_field(root, "selected function declaration", entry_declaration);
    if let Some(provenance) = provenance {
        let provenance = data(&mut network, provenance)?;
        network.insert_field(root, "provenance", provenance);
    }
    let archive = FormalConceptArchive { network, root };
    checked(&archive)?;
    Ok(archive)
}

/// Portable preparation for the generated closed-bijection verifier.
///
/// # Errors
/// Refuses every unsupported or invalid basis before returning graph data.
pub fn read_formal_concept_definition(value: &Value) -> Result<Value, String> {
    let archive = FormalConceptArchive::from_json(value)?;
    let result = checked(&archive)?;
    Ok(
        json!({"schemaVersion": 1, "model": result.model, "shapes": result.graph.shapes, "offsets": result.graph.offsets, "edges": result.graph.edges}),
    )
}

/// Prove equality under one supported model using a closed correspondence.
#[must_use]
pub fn prove_formal_concept_equivalence(
    first: &FormalConceptArchive,
    second: &FormalConceptArchive,
) -> Option<FormalConceptArchive> {
    let one = checked(first).ok()?;
    let other = checked(second).ok()?;
    if one.model != other.model {
        return None;
    }
    let pairs = correspondence(&one.graph, &other.graph)?;
    let mut network = LinkNetwork::new();
    let left = snapshot(&mut network, &one.graph)?;
    let right = snapshot(&mut network, &other.graph)?;
    let entries = pairs
        .into_iter()
        .map(|(a, b)| network.insert_link([left[a], right[b]], metadata(PAIR, None)))
        .collect::<Vec<_>>();
    let list = network.insert_dynamic_link(&entries, metadata(PAIRS, None));
    let root = network.insert_link(
        [left[0], right[0], list],
        metadata(CERTIFICATE, Some(SCHEMA)),
    );
    Some(FormalConceptArchive { network, root })
}

fn verify(
    first: &FormalConceptArchive,
    second: &FormalConceptArchive,
    certificate: &FormalConceptArchive,
) -> Result<bool, String> {
    let one = checked(first)?;
    let other = checked(second)?;
    if one.model != other.model {
        return Ok(false);
    }
    let network = &certificate.network;
    let link = network
        .link(certificate.root)
        .ok_or("missing formal equivalence certificate")?;
    if link.metadata().language() != Some(LANGUAGE)
        || link.metadata().term() != Some(CERTIFICATE)
        || link.metadata().definition() != Some(SCHEMA)
        || link.references().len() != 3
    {
        return Ok(false);
    }
    let left = graph(network, link.references()[0], &BTreeSet::default())?;
    let right = graph(network, link.references()[1], &BTreeSet::default())?;
    if !left.same_shape(&one.graph) || !right.same_shape(&other.graph) {
        return Ok(false);
    }
    let list = network
        .link(link.references()[2])
        .ok_or("missing formal node correspondence list")?;
    if list.metadata().language() != Some(LANGUAGE)
        || list.metadata().term() != Some(PAIRS)
        || list.references().len() != left.shapes.len()
        || left.shapes.len() != right.shapes.len()
    {
        return Ok(false);
    }
    let mut pairs = Vec::new();
    for id in list.references() {
        let pair = network
            .link(*id)
            .ok_or("missing formal node correspondence")?;
        if pair.metadata().language() != Some(LANGUAGE)
            || pair.metadata().term() != Some(PAIR)
            || pair.references().len() != 2
        {
            return Ok(false);
        }
        let Some(a) = left.indices.get(&pair.references()[0]) else {
            return Ok(false);
        };
        let Some(b) = right.indices.get(&pair.references()[1]) else {
            return Ok(false);
        };
        pairs.push((*a, *b));
    }
    Ok(pairs.first() == Some(&(0, 0)) && verify_pairs(&left, &right, &pairs))
}

/// Recheck the certificate's copied premises and every correspondence pair.
#[must_use]
pub fn verify_formal_concept_equivalence(
    first: &FormalConceptArchive,
    second: &FormalConceptArchive,
    certificate: &FormalConceptArchive,
) -> bool {
    verify(first, second, certificate).unwrap_or(false)
}
