//! The grammar and syntax tree levels of [`DecoratorSet`].

use super::feature_runtime::SyntaxTree;
use super::merge::rename_grammar_rule;
use super::{Grammar, GrammarFormat, RuleKind};
use crate::decorators::{
    DecoratorError, DecoratorLevel, DecoratorRecord, DecoratorSet, decorator_record, record_field,
};

/// The grammar the `importer` or `grammar-rule` decorators of `decorators`
/// make of `grammar`.
///
/// Each rule is shown as `{ name, kind, concept }`, with
/// `format` (the grammar's source format) added at the `importer` level.
/// Setting `name` renames the rule and every reference to it, setting `kind`
/// or `concept` changes the rule, and `drop` removes it. The rules are
/// decorated in grammar order; a grammar without decorators of the level is
/// returned unchanged.
///
/// # Errors
///
/// A [`DecoratorError`] for an unknown rule kind or a rename the grammar
/// refuses.
pub fn decorate_grammar(
    grammar: &Grammar,
    decorators: &DecoratorSet,
    level: DecoratorLevel,
) -> Result<Grammar, DecoratorError> {
    if !decorators.has(level) {
        return Ok(grammar.clone());
    }
    let mut renames = Vec::new();
    let mut rules = Vec::new();
    for rule in grammar.rules() {
        let mut record = DecoratorRecord::new();
        record.insert("name".to_owned(), rule.name.clone());
        record.insert("kind".to_owned(), rule.kind.as_str().to_owned());
        record.insert(
            "concept".to_owned(),
            rule.concept.clone().unwrap_or_default(),
        );
        if level == DecoratorLevel::Importer {
            let format = grammar.source_format().map_or("", GrammarFormat::as_str);
            record.insert("format".to_owned(), format.to_owned());
        }
        let Some(decorated) = decorators.decorate(level, &record) else {
            continue;
        };
        let mut rule = rule.clone();
        let kind = record_field(&decorated, "kind");
        rule.kind = RuleKind::from_tag(kind).ok_or_else(|| {
            DecoratorError::new(format!("a decorator set the unknown rule kind {kind:?}"))
        })?;
        let concept = record_field(&decorated, "concept");
        rule.concept = (!concept.is_empty()).then(|| concept.to_owned());
        let name = record_field(&decorated, "name");
        if name != rule.name {
            renames.push((rule.name.clone(), name.to_owned()));
        }
        rules.push(rule);
    }
    let mut result = grammar.clone();
    result.rules = rules;
    for (from, to) in renames {
        result = rename_grammar_rule(&result, &from, &to, None, &[])
            .map_err(|error| DecoratorError::new(error.to_string()))?
            .grammar;
    }
    Ok(result)
}

fn optional(text: &str) -> Option<String> {
    (!text.is_empty()).then(|| text.to_owned())
}

/// The syntax tree the `executor` and `recovery` decorators of `decorators`
/// make of `tree`.
///
/// The `executor` level shows every node and token as
/// `{ type, kind, field, text }` (`text` for tokens): setting `kind` or
/// `field` changes it, and `drop` replaces a node with its children, so the
/// tree still holds every byte; a token cannot be dropped. The `recovery`
/// level shows every ERROR node as `{ type, reason, text }` and every MISSING
/// node as `{ type, kind }`: setting `reason` or `kind` changes it, and
/// `drop` removes a MISSING node, which spans no input; an ERROR node cannot
/// be dropped.
///
/// # Errors
///
/// A [`DecoratorError`] when a decorator drops a token, an ERROR node or the
/// root.
pub fn decorate_syntax_tree(
    tree: &SyntaxTree,
    decorators: &DecoratorSet,
    source: &[u8],
) -> Result<SyntaxTree, DecoratorError> {
    if !decorators.has(DecoratorLevel::Executor) && !decorators.has(DecoratorLevel::Recovery) {
        return Ok(tree.clone());
    }
    let mut roots = visit(tree, decorators, source)?;
    if roots.len() != 1 {
        return Err(DecoratorError::new(
            "a decorator dropped the root of the syntax tree",
        ));
    }
    Ok(roots.remove(0))
}

fn visit(
    tree: &SyntaxTree,
    decorators: &DecoratorSet,
    source: &[u8],
) -> Result<Vec<SyntaxTree>, DecoratorError> {
    let text_of = |start: usize, end: usize| {
        String::from_utf8_lossy(source.get(start..end).unwrap_or_default()).into_owned()
    };
    let mut copy = tree.clone();
    match &mut copy {
        SyntaxTree::Node {
            kind,
            field,
            children,
            ..
        } => {
            let mut record = DecoratorRecord::new();
            record.insert("type".to_owned(), "node".to_owned());
            record.insert("kind".to_owned(), kind.clone());
            record.insert("field".to_owned(), field.clone().unwrap_or_default());
            let decorated = decorators.decorate(DecoratorLevel::Executor, &record);
            let mut visited = Vec::new();
            for child in children.iter() {
                visited.extend(visit(child, decorators, source)?);
            }
            let Some(decorated) = decorated else {
                return Ok(visited);
            };
            record_field(&decorated, "kind").clone_into(kind);
            *field = optional(record_field(&decorated, "field"));
            *children = visited;
        }
        SyntaxTree::Token {
            kind,
            field,
            start,
            end,
            ..
        } => {
            let mut record = DecoratorRecord::new();
            record.insert("type".to_owned(), "token".to_owned());
            record.insert("kind".to_owned(), kind.clone().unwrap_or_default());
            record.insert("field".to_owned(), field.clone().unwrap_or_default());
            record.insert("text".to_owned(), text_of(*start, *end));
            let Some(decorated) = decorators.decorate(DecoratorLevel::Executor, &record) else {
                return Err(DecoratorError::new(format!(
                    "a decorator dropped the token {}; the executor level keeps every token",
                    kind.as_deref().unwrap_or_default()
                )));
            };
            *kind = optional(record_field(&decorated, "kind"));
            *field = optional(record_field(&decorated, "field"));
        }
        SyntaxTree::Error {
            start, end, reason, ..
        } => {
            let mut record = DecoratorRecord::new();
            record.insert("type".to_owned(), "error".to_owned());
            record.insert("reason".to_owned(), reason.clone().unwrap_or_default());
            record.insert("text".to_owned(), text_of(*start, *end));
            let Some(decorated) = decorators.decorate(DecoratorLevel::Recovery, &record) else {
                return Err(DecoratorError::new(
                    "a decorator dropped an ERROR node; only a MISSING node can be dropped",
                ));
            };
            *reason = optional(record_field(&decorated, "reason"));
        }
        SyntaxTree::Missing { kind, .. } => {
            let mut record = DecoratorRecord::new();
            record.insert("type".to_owned(), "missing".to_owned());
            record.insert("kind".to_owned(), kind.clone().unwrap_or_default());
            let Some(decorated) = decorators.decorate(DecoratorLevel::Recovery, &record) else {
                return Ok(Vec::new());
            };
            *kind = optional(record_field(&decorated, "kind"));
        }
        SyntaxTree::Embed { root, .. } => {
            let mut roots = visit(root, decorators, source)?;
            if roots.len() != 1 {
                return Err(DecoratorError::new(
                    "a decorator dropped the root of an embedded tree",
                ));
            }
            **root = roots.remove(0);
        }
    }
    Ok(vec![copy])
}

/// The emitted grammar `source` after the `emitter` decorators of
/// `decorators`.
///
/// They see each line as `{ format, number, line }` (`number`
/// counts from 1): setting `line` rewrites it and `drop` removes it. Apply it
/// to the source an emitter such as [`super::emit_gbnf`] returns; the emit
/// report is kept as it is.
#[must_use]
pub fn decorate_emitted(format: &str, source: &str, decorators: &DecoratorSet) -> String {
    if !decorators.has(DecoratorLevel::Emitter) {
        return source.to_owned();
    }
    let (body, ending) = source
        .strip_suffix('\n')
        .map_or((source, ""), |body| (body, "\n"));
    let kept: Vec<String> = body
        .split('\n')
        .enumerate()
        .filter_map(|(index, line)| {
            let number = (index + 1).to_string();
            let record = decorator_record([
                ("format", format),
                ("number", number.as_str()),
                ("line", line),
            ]);
            decorators
                .decorate(DecoratorLevel::Emitter, &record)
                .map(|decorated| record_field(&decorated, "line").to_owned())
        })
        .collect();
    if kept.is_empty() {
        String::new()
    } else {
        format!("{}{ending}", kept.join("\n"))
    }
}
