//! Native TypeScript annotation spans projected into the common typed frontend.
use super::typescript_erasure::{ErasedTypeScript, erase_type_script};
use super::typescript_erasure_rules::render_type_script_documentation_pattern;
use crate::native_grammar_parser::{NativeNode, parse_native};
use crate::translation::Span;
use crate::translation::diagnostics::{ErrorKind, TranslationError};
use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};
use std::collections::BTreeMap;
use std::sync::OnceLock;

fn rules() -> &'static BTreeMap<String, String> {
    static RULES: OnceLock<BTreeMap<String, String>> = OnceLock::new();
    RULES.get_or_init(|| {
        parse_lino_to_links_with_config(
            include_str!("../data/typescript-portable-typing.lino"),
            &ParserConfig::without_comments(),
        )
        .expect("shared TypeScript typing rules are valid Links Notation")
        .into_iter()
        .map(|definition| {
            let LiNo::Link { values, .. } = definition else {
                panic!("a typing rule must be a link");
            };
            let words: Vec<_> = values
                .into_iter()
                .map(|value| match value {
                    LiNo::Ref(word) => word,
                    LiNo::Link {
                        id: Some(word),
                        values,
                    } if values.is_empty() => word,
                    LiNo::Link { .. } => panic!("a typing rule field must be a word"),
                })
                .collect();
            let [kind, term, operation] = words.as_slice() else {
                panic!("a typing rule has three fields");
            };
            assert_eq!(kind, "typing-rule");
            (term.clone(), operation.clone())
        })
        .collect()
    })
}

fn field<'a>(node: &'a NativeNode, names: &[&str]) -> Option<&'a NativeNode> {
    node.children.iter().find_map(|(child, field)| {
        field
            .as_deref()
            .filter(|field| names.contains(field))
            .map(|_| child)
    })
}

fn annotation_type(source: &str, node: &NativeNode) -> Result<String, TranslationError> {
    let retained = if node.term == "type_annotation" {
        node.children
            .iter()
            .map(|(child, _)| child)
            .find(|child| child.named && !child.is_extra)
    } else {
        Some(node)
    }
    .ok_or_else(|| {
        TranslationError::new(
            ErrorKind::Unsupported,
            "TypeScript annotation has no type expression",
            Some(Span::new(node.start, node.end)),
        )
    })?;
    let mut value = String::new();
    let mut line_ending = false;
    for scalar in source[retained.start..retained.end].chars() {
        if matches!(scalar, '\r' | '\n') {
            if !line_ending {
                value.push(' ');
            }
            line_ending = true;
        } else {
            value.push(scalar);
            line_ending = false;
        }
    }
    if value.contains("*/") {
        return Err(TranslationError::new(
            ErrorKind::Unsupported,
            "TypeScript type cannot be represented as a portable documentation annotation",
            Some(Span::new(retained.start, retained.end)),
        ));
    }
    Ok(value)
}

fn visit(
    source: &str,
    node: &NativeNode,
    parent: Option<&NativeNode>,
    projections: &mut Vec<(usize, String)>,
) -> Result<(), TranslationError> {
    let owner = parent
        .filter(|parent| parent.term == "export_statement")
        .unwrap_or(node);
    let mut tags = Vec::new();
    match rules().get(&node.term).map(String::as_str) {
        Some("function") => {
            if let Some(parameters) = field(node, &["parameters"]) {
                for (parameter, _) in &parameters.children {
                    if let (Some(annotation), Some(name)) = (
                        field(parameter, &["type"]),
                        field(parameter, &["pattern", "name"]),
                    ) {
                        let name =
                            render_type_script_documentation_pattern(&source[name.start..name.end]);
                        let name = if parameter.term == "optional_parameter" {
                            format!("[{name}]")
                        } else {
                            name
                        };
                        tags.push(format!(
                            "@param {{{}}} {name}",
                            annotation_type(source, annotation)?
                        ));
                    }
                }
            }
            if let Some(returns) = field(node, &["return_type"]) {
                tags.push(format!(
                    "@returns {{{}}}",
                    annotation_type(source, returns)?
                ));
            }
        }
        Some("alias") => {
            if let (Some(value), Some(name)) = (field(node, &["value"]), field(node, &["name"])) {
                tags.push(format!(
                    "@typedef {{{}}} {}",
                    annotation_type(source, value)?,
                    &source[name.start..name.end]
                ));
            }
        }
        _ => {}
    }
    if !tags.is_empty() {
        projections.push((
            owner.start,
            format!(
                "/**\n{}\n */\n",
                tags.iter()
                    .map(|tag| format!(" * {tag}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        ));
    }
    for (child, _) in &node.children {
        visit(source, child, Some(node), projections)?;
    }
    Ok(())
}

/// Project supported TypeScript annotations into the shared frontend's `JSDoc` form.
///
/// # Errors
/// Returns the existing annotation or syntax obligation when the source cannot
/// be projected by the shared typing rules.
pub fn project_type_script_typing(source: &str) -> Result<String, TranslationError> {
    let root = parse_native("native-typescript", source);
    let ErasedTypeScript { code, erasures } = erase_type_script(source)?;
    let mut projections = Vec::new();
    visit(source, &root, None, &mut projections)?;
    projections.sort_by_key(|(offset, _)| *offset);
    let mut output = String::new();
    let mut cursor = 0;
    for (offset, annotation) in projections {
        let removed: usize = erasures
            .iter()
            .filter(|erasure| erasure.end <= offset)
            .map(|erasure| {
                erasure.end - erasure.start - source[erasure.start..erasure.end].chars().count()
            })
            .sum();
        let offset = offset - removed;
        output.push_str(&code[cursor..offset]);
        output.push_str(&annotation);
        cursor = offset;
    }
    output.push_str(&code[cursor..]);
    Ok(output)
}
