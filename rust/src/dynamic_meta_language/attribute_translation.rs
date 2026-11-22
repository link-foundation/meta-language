//! Translate concrete outputs of an installed attributed grammar through the
//! ordinary checked program and target emitters.

use super::{DynamicMetaLanguage, DynamicParseOptions, DynamicParseResult, fail};
use crate::grammar::feature_runtime::{FeatureParseOptions, OperationValue, SyntaxTree};
use crate::grammar_attribute_inference_rules::{
    accept_grammar_attribute_selection_names, accept_grammar_attribute_translation_target,
};
use crate::translation::Language;
use crate::translation::check::check_program;
use crate::translation::surface::{SEffect, SExpr, SMain, SNode, SProgram, ShowStyle};
use crate::translation::types::FLOAT;
use crate::typed_program_links::{
    TypedProgramLinks, TypedProgramLinksEmission, emit_typed_program_links,
    encode_typed_program_links,
};

/// Select an attribute on a concrete child occurrence, in output order.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DynamicAttributeSelection {
    /// Child indices in the concrete tree, including trivia.
    pub path: Vec<usize>,
    /// Required concrete node or token kind.
    pub kind: String,
    /// Attribute set by the installed grammar.
    pub attribute: String,
}

/// Emitted executable output and its exact checked and concrete inputs.
#[derive(Clone, Debug)]
pub struct DynamicAttributeTranslation {
    /// The parse retained under the same validated definition snapshot.
    pub parsed: DynamicParseResult,
    /// The checked Links representation of the ordered attribute outputs.
    pub links: TypedProgramLinks,
    /// The selected target and its contract.
    pub output: TypedProgramLinksEmission,
}

fn select_attribute<'a>(
    mut node: &'a SyntaxTree,
    selection: &DynamicAttributeSelection,
) -> Option<&'a OperationValue> {
    for &index in &selection.path {
        let SyntaxTree::Node { children, .. } = node else {
            return None;
        };
        node = children.get(index)?;
    }
    match node {
        SyntaxTree::Node {
            kind,
            trivia: false,
            attributes: Some(attributes),
            ..
        }
        | SyntaxTree::Token {
            kind: Some(kind),
            trivia: false,
            attributes: Some(attributes),
            ..
        } if *kind == selection.kind => attributes.get(&selection.attribute),
        _ => None,
    }
}

fn attribute_output(value: &OperationValue) -> Result<SEffect, String> {
    let node = match value {
        OperationValue::Text(value) => SNode::Str {
            value: value.clone(),
        },
        OperationValue::Integer(value) if value.unsigned_abs() <= (1u64 << 53) - 1 => SNode::Num {
            value: value.to_string(),
            ty: Some(FLOAT),
            negative: false,
            unit: false,
        },
        OperationValue::Integer(_) => {
            return Err(fail(
                "attribute translation needs a text or safe integer result",
            ));
        }
    };
    Ok(SEffect::Print {
        expr: SExpr::new(node, None),
        style: ShowStyle::JsConsole,
        span: None,
    })
}

impl DynamicMetaLanguage {
    /// Parse using the installed grammar and emit its selected attributed outputs.
    ///
    /// # Errors
    /// Rejects unsupported targets, ambiguous or recovered parses, invalid
    /// selections, and checker or target emission errors.
    pub fn translate_attributes(
        &self,
        source_identifier: &str,
        input: &[u8],
        target: Language,
        selections: &[DynamicAttributeSelection],
        options: FeatureParseOptions,
    ) -> Result<DynamicAttributeTranslation, String> {
        if !accept_grammar_attribute_translation_target(target.as_str()) {
            return Err(fail(
                "attribute translation needs a JavaScript or Rust target",
            ));
        }
        if selections.is_empty() || selections.len() > self.limits.node_limit {
            return Err(fail(
                "attribute translation needs a bounded nonempty list of selections",
            ));
        }
        if selections.iter().any(|selection| {
            selection.path.len() > self.depth_limit
                || !accept_grammar_attribute_selection_names(&selection.kind, &selection.attribute)
        }) {
            return Err(fail("invalid attribute translation selection"));
        }
        let parsed = self.parse(
            source_identifier,
            input,
            &DynamicParseOptions {
                parser: FeatureParseOptions {
                    reject_ambiguity: Some(true),
                    accept_recovery: Some(false),
                    ..options
                },
                ..DynamicParseOptions::default()
            },
        )?;
        let tree = parsed
            .outcome
            .tree
            .as_ref()
            .filter(|_| parsed.outcome.ok)
            .ok_or_else(|| {
                fail("attribute translation needs an accepted unambiguous concrete tree")
            })?;
        let effects = selections.iter().map(|selection| {
            let value = select_attribute(tree, selection).ok_or_else(|| {
                fail("attribute translation selection does not identify a concrete attributed occurrence")
            })?;
            attribute_output(value)
        }).collect::<Result<Vec<_>, _>>()?;
        let program = check_program(&SProgram {
            source_class_owners: None,
            source_class_methods: Vec::new(),
            reference_semantics: None,
            source_runtime_shell: None,
            source_pure_unit_calls: None,
            language: Language::JavaScript,
            items: Vec::new(),
            main: Some(SMain {
                source_runtime_worker: None,
                effects,
                span: None,
                sequential_async: false,
            }),
            imports: Vec::new(),
            externals: Vec::new(),
        })
        .map_err(|error| error.to_string())?;
        let links = encode_typed_program_links(&program).map_err(|error| error.to_string())?;
        let output = emit_typed_program_links(&links.network, links.root, target.as_str())
            .map_err(|error| error.to_string())?;
        Ok(DynamicAttributeTranslation {
            parsed,
            links,
            output,
        })
    }
}
