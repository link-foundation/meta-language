//! Host boundary for shared linked TypeScript erasure rules and native CST spans.
use super::typescript_erasure_rules::{
    erase_exported_declaration, erase_optional_parameter_marker, render_erased_type_script_scalar,
    retain_first_erasure_child,
};
use crate::native_grammar_parser::{NativeNode, parse_native};
use crate::translation::Span;
use crate::translation::diagnostics::{TranslationError, unsupported};
use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

/// One original UTF-8 source span erased by the linked TypeScript rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Erasure {
    /// The first original source byte.
    pub start: usize,
    /// The byte after the erased original source region.
    pub end: usize,
    /// The native concrete construct that owns the erased region.
    pub term: String,
}

/// Executable JavaScript and the original TypeScript regions removed from it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErasedTypeScript {
    /// The native JavaScript source with line boundaries preserved.
    pub code: String,
    /// The disjoint erasures in original source byte order.
    pub erasures: Vec<Erasure>,
}

type Rules = (BTreeMap<String, String>, BTreeSet<String>);

fn rules() -> &'static Rules {
    static RULES: OnceLock<Rules> = OnceLock::new();
    RULES.get_or_init(|| {
        let parsed = parse_lino_to_links_with_config(
            include_str!("../data/typescript-erasure.lino"),
            &ParserConfig::without_comments(),
        )
        .expect("shared TypeScript erasure rules are valid Links Notation");
        let mut operations = BTreeMap::new();
        let mut refusals = BTreeSet::new();
        for definition in parsed {
            let LiNo::Link { values, .. } = definition else {
                panic!("an erasure rule must be a link");
            };
            let words: Vec<_> = values
                .into_iter()
                .map(|value| match value {
                    LiNo::Ref(word) => word,
                    LiNo::Link {
                        id: Some(word),
                        values,
                    } if values.is_empty() => word,
                    LiNo::Link { .. } => panic!("an erasure rule field must be a word"),
                })
                .collect();
            match words.as_slice() {
                [kind, term, operation] if kind == "erasure-rule" => {
                    operations.insert(term.clone(), operation.clone());
                }
                [kind, term] if kind == "refusal-rule" => {
                    refusals.insert(term.clone());
                }
                _ => panic!("unknown TypeScript erasure rule"),
            }
        }
        (operations, refusals)
    })
}

fn has_erasure(node: &NativeNode, parent: Option<&NativeNode>) -> bool {
    let (operations, refusals) = rules();
    operations.contains_key(&node.term)
        || refusals.contains(&node.term)
        || erase_optional_parameter_marker(
            &node.term,
            parent.map_or("", |value| value.term.as_str()),
        )
        || (node.term == "type"
            && parent.is_some_and(|parent| {
                matches!(
                    parent.term.as_str(),
                    "import_statement" | "import_specifier"
                )
            }))
        || node
            .children
            .iter()
            .any(|(child, _)| has_erasure(child, Some(node)))
}

pub(super) fn requires_type_script_erasure(source: &str) -> bool {
    has_erasure(&parse_native("native-typescript", source), None)
}

fn refusal(node: &NativeNode, detail: &str) -> TranslationError {
    unsupported(
        &format!("TypeScript {}", node.term),
        detail,
        Some(Span::new(node.start, node.end)),
    )
}

fn visit(
    node: &NativeNode,
    parent: Option<&NativeNode>,
    erasures: &mut Vec<Erasure>,
) -> Result<(), TranslationError> {
    if node.is_error || node.is_missing || node.term == "ERROR" {
        return Err(refusal(node, "the native syntax tree contains an error"));
    }
    let (operations, refusals) = rules();
    if refusals.contains(&node.term) {
        return Err(refusal(
            node,
            "this runtime construct needs explicit translation",
        ));
    }
    let operation = operations.get(&node.term).map(String::as_str);
    if let Some(operation @ ("span" | "declaration")) = operation {
        let owner = if erase_exported_declaration(
            operation,
            parent.is_some_and(|parent| parent.term == "export_statement"),
        ) {
            parent.unwrap()
        } else {
            node
        };
        erasures.push(Erasure {
            start: owner.start,
            end: owner.end,
            term: node.term.clone(),
        });
        return Ok(());
    }
    if let Some(operation @ ("prefix" | "suffix")) = operation {
        let mut named = node
            .children
            .iter()
            .map(|(child, _)| child)
            .filter(|child| child.named && !child.is_extra);
        let retained = if retain_first_erasure_child(operation) {
            named.next()
        } else {
            named.next_back()
        }
        .ok_or_else(|| refusal(node, "the erasure has no retained expression"))?;
        let (start, end) = if retain_first_erasure_child(operation) {
            (retained.end, node.end)
        } else {
            (node.start, retained.start)
        };
        erasures.push(Erasure {
            start,
            end,
            term: node.term.clone(),
        });
        return visit(retained, Some(node), erasures);
    }
    if erase_optional_parameter_marker(&node.term, parent.map_or("", |parent| parent.term.as_str()))
    {
        erasures.push(Erasure {
            start: node.start,
            end: node.end,
            term: parent.unwrap().term.clone(),
        });
    } else if node.term == "import_statement"
        && node.children.iter().any(|(child, _)| child.term == "type")
    {
        erasures.push(Erasure {
            start: node.start,
            end: node.end,
            term: "type_import".to_owned(),
        });
        return Ok(());
    } else if node.term == "import_specifier"
        && node.children.iter().any(|(child, _)| child.term == "type")
    {
        return Err(refusal(
            node,
            "mixed type imports need a linked separator reconstruction rule",
        ));
    }
    for (child, _) in &node.children {
        visit(child, Some(node), erasures)?;
    }
    Ok(())
}

/// Erase compile time TypeScript forms using native concrete syntax spans.
///
/// # Errors
/// Returns a precise obligation for runtime constructs, malformed trees, mixed
/// type imports or erased source that the native JavaScript grammar rejects.
pub fn erase_type_script(source: &str) -> Result<ErasedTypeScript, TranslationError> {
    let root = parse_native("native-typescript", source);
    let mut erasures = Vec::new();
    visit(&root, None, &mut erasures)?;
    erasures.sort_by(|left, right| {
        left.start
            .cmp(&right.start)
            .then_with(|| right.end.cmp(&left.end))
    });
    let mut disjoint: Vec<Erasure> = Vec::new();
    for erasure in erasures {
        if let Some(previous) = disjoint.last()
            && erasure.start < previous.end
        {
            if erasure.end > previous.end {
                return Err(refusal(
                    &root,
                    "TypeScript erasure ranges overlap without containment",
                ));
            }
            continue;
        }
        disjoint.push(erasure);
    }
    let mut code = String::with_capacity(source.len());
    let mut cursor = 0;
    for erasure in &disjoint {
        code.push_str(&source[cursor..erasure.start]);
        for scalar in source[erasure.start..erasure.end].chars() {
            code.push_str(&render_erased_type_script_scalar(
                scalar == '\r',
                scalar == '\n',
            ));
        }
        cursor = erasure.end;
    }
    code.push_str(&source[cursor..]);
    if parse_native("native-javascript", &code).has_error {
        return Err(refusal(&root, "erased source is not valid JavaScript"));
    }
    Ok(ErasedTypeScript {
        code,
        erasures: disjoint,
    })
}
