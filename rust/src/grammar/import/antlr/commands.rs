//! ANTLR lexer commands: the channel `-> skip` and `-> channel(NAME)` put a
//! rule on, and the retyping of `-> type(NAME)`.

use super::{finish_choice, is_channel_number, is_identifier, push_choice_alternative};
use crate::grammar::{GrammarExpr, GrammarRule, RuleKind};

/// The channel a lexer command puts its token on: `skip` and `channel(NAME)`
/// make the token trivia, which the runtime skips between tokens.
pub(super) fn command_channel(command: &str) -> Option<String> {
    let parts: Vec<&str> = command
        .strip_prefix("->")
        .unwrap_or(command)
        .split(',')
        .map(str::trim)
        .collect();
    for part in &parts {
        let name = part
            .strip_prefix("channel(")
            .and_then(|rest| rest.strip_suffix(')'));
        if let Some(name) = name
            && (is_identifier(name) || is_channel_number(name))
        {
            return Some(name.to_owned());
        }
    }
    parts.contains(&"skip").then(|| "skip".to_owned())
}

/// The token type `-> type(NAME)` gives a rule's tokens.
pub(super) fn command_type(command: &str) -> Option<String> {
    command
        .strip_prefix("->")
        .unwrap_or(command)
        .split(',')
        .filter_map(|part| part.trim().strip_prefix("type(")?.strip_suffix(')'))
        .find(|name| is_identifier(name))
        .map(str::to_owned)
}

/// A rule whose `-> type(X)` command retypes its tokens matches where X does:
/// X gains a reference to it. An X that only `tokens {...}` declares becomes
/// a token rule of the rules that retype to it. An X that already has that
/// alternative, as an exported grammar re-imports, keeps it once.
pub(super) fn apply_retypes(parsed: Vec<(GrammarRule, Option<String>)>) -> Vec<GrammarRule> {
    let mut retypes = Vec::new();
    let mut rules = Vec::with_capacity(parsed.len());
    for (rule, retype) in parsed {
        if let Some(target) = retype.filter(|target| *target != rule.name) {
            retypes.push((rule.name.clone(), target));
        }
        rules.push(rule);
    }
    for (source, target) in retypes {
        let reference = GrammarExpr::NonTerminal(source.clone());
        let note = format!("also {source}, which -> type({target}) retypes");
        let Some(rule) = rules.iter_mut().find(|rule| rule.name == target) else {
            rules.push(
                GrammarRule::new(target, reference)
                    .with_kind(RuleKind::Token)
                    .with_doc(note),
            );
            continue;
        };
        let has_alternative = match &rule.expr {
            GrammarExpr::Choice {
                ordered: false,
                alternatives,
            } => alternatives.contains(&reference),
            expr => *expr == reference,
        };
        if !has_alternative {
            let mut alternatives = Vec::new();
            push_choice_alternative(
                &mut alternatives,
                std::mem::replace(&mut rule.expr, GrammarExpr::Empty),
            );
            push_choice_alternative(&mut alternatives, reference);
            rule.expr = finish_choice(alternatives);
        }
        rule.doc = Some(match rule.doc.take() {
            Some(doc) if doc.split("; ").any(|part| part == note) => doc,
            Some(doc) => format!("{doc}; {note}"),
            None => note,
        });
    }
    rules
}
