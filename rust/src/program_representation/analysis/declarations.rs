//! Token-driven declarations: the binders each language's keywords and
//! delimiters introduce, before syntax-driven binders and extents apply.

use super::{
    declaration_markers, find_token, inside_binder, matching_delimiter, next_identifier, BTreeMap,
    BTreeSet, Declaration, ProgramSourceMapping, SemanticToken, TokenKind, Visibility,
};

pub(super) fn declare_javascript(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    brace_scopes: &BTreeMap<usize, usize>,
    declarations: &mut Vec<Declaration>,
    declared: &mut BTreeSet<usize>,
) {
    for (index, token) in tokens.iter().enumerate() {
        if matches!(token.text.as_str(), "const" | "let" | "var" | "class") {
            let pattern = (token.text != "class")
                .then(|| {
                    syntax.iter().find(|fact| {
                        fact.term == "object_pattern"
                            && tokens
                                .get(index + 1)
                                .is_some_and(|next| fact.range.start == next.range.start)
                    })
                })
                .flatten();
            if let Some(pattern) = pattern {
                for fact in syntax {
                    if fact.range.start < pattern.range.start || fact.range.end > pattern.range.end
                    {
                        continue;
                    }
                    if fact.term == "shorthand_property_identifier_pattern" {
                        if let Some(local) = tokens
                            .iter()
                            .position(|candidate| candidate.range == fact.range)
                        {
                            declare(tokens, local, &token.text, None, declarations, declared);
                        }
                    } else if fact.term == "pair_pattern" {
                        let colon = tokens.iter().position(|candidate| {
                            candidate.text == ":"
                                && candidate.range.start >= fact.range.start
                                && candidate.range.start < fact.range.end
                        });
                        if let Some(local) =
                            colon.and_then(|colon| next_identifier(tokens, colon + 1))
                        {
                            if tokens[local].range.end <= fact.range.end {
                                declare(tokens, local, &token.text, None, declarations, declared);
                            }
                        }
                    }
                }
            } else {
                declare_next(tokens, index + 1, &token.text, None, declarations, declared);
            }
        }
        if token.text == "function" {
            if let Some(name) = next_identifier(tokens, index + 1) {
                declare(tokens, name, "function", None, declarations, declared);
                let open = find_token(tokens, name + 1, "(");
                let close = open.and_then(|open| matching_delimiter(tokens, open, "(", ")"));
                let body = close.and_then(|close| find_token(tokens, close + 1, "{"));
                let scope = body
                    .and_then(|body| brace_scopes.get(&body).copied())
                    .unwrap_or(tokens[name].scope);
                declare_parameters(tokens, open, close, scope, declarations, declared);
            }
        }
        if token.text == "catch" {
            let open = find_token(tokens, index + 1, "(");
            let close = open.and_then(|open| matching_delimiter(tokens, open, "(", ")"));
            let body = close.and_then(|close| find_token(tokens, close + 1, "{"));
            let scope = body
                .and_then(|body| brace_scopes.get(&body).copied())
                .unwrap_or(token.scope);
            declare_parameters(tokens, open, close, scope, declarations, declared);
        }
        if token.text == "import" {
            let end = find_token(tokens, index + 1, "from")
                .or_else(|| find_token(tokens, index + 1, ";"))
                .unwrap_or(tokens.len());
            for cursor in index + 1..end {
                if tokens[cursor].kind == TokenKind::Identifier
                    && (tokens
                        .get(cursor.wrapping_sub(1))
                        .is_some_and(|token| token.text == "as")
                        || !matches!(tokens.get(cursor + 1), Some(token) if token.text == "as"))
                {
                    declare(tokens, cursor, "import", None, declarations, declared);
                }
            }
        }
    }
}

pub(super) fn declare_rust(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    brace_scopes: &BTreeMap<usize, usize>,
    declarations: &mut Vec<Declaration>,
    declared: &mut BTreeSet<usize>,
) {
    let uses = syntax
        .iter()
        .filter(|fact| fact.term == "use_declaration")
        .map(|fact| fact.range)
        .collect::<Vec<_>>();
    for (index, token) in tokens.iter().enumerate() {
        if matches!(
            token.text.as_str(),
            "struct" | "enum" | "trait" | "type" | "const" | "static" | "mod" | "union"
        ) {
            declare_next(tokens, index + 1, &token.text, None, declarations, declared);
        }
        if token.text == "fn" {
            if let Some(name) = next_identifier(tokens, index + 1) {
                declare(tokens, name, "function", None, declarations, declared);
                let open = find_token(tokens, name + 1, "(");
                let close = open.and_then(|open| matching_delimiter(tokens, open, "(", ")"));
                let body = close.and_then(|close| find_token(tokens, close + 1, "{"));
                let scope = body
                    .and_then(|body| brace_scopes.get(&body).copied())
                    .unwrap_or(tokens[name].scope);
                if let (Some(open), Some(close)) = (open, close) {
                    for cursor in open + 1..close {
                        if tokens[cursor].kind == TokenKind::Identifier
                            && tokens
                                .get(cursor + 1)
                                .is_some_and(|token| token.text == ":")
                        {
                            declare(
                                tokens,
                                cursor,
                                "parameter",
                                Some(scope),
                                declarations,
                                declared,
                            );
                        }
                    }
                }
            }
        }
        if token.text == "macro_rules"
            && tokens.get(index + 1).is_some_and(|token| token.text == "!")
        {
            declare_next(tokens, index + 2, "macro", None, declarations, declared);
        }
        if token.text == "as"
            && uses
                .iter()
                .any(|range| token.range.start >= range.start && token.range.end <= range.end)
        {
            declare_next(tokens, index + 1, "import", None, declarations, declared);
        }
    }
}

pub(super) fn declare_proof_language(
    tokens: &[SemanticToken],
    language: &str,
    declarations: &mut Vec<Declaration>,
    declared: &mut BTreeSet<usize>,
) {
    for (index, token) in tokens.iter().enumerate() {
        if declaration_markers(language).contains(&token.text.as_str()) {
            declare_next(tokens, index + 1, &token.text, None, declarations, declared);
        }
        if token.text == "let" {
            declare_next(tokens, index + 1, "let", None, declarations, declared);
        }
        if token.kind == TokenKind::Identifier
            && tokens.get(index + 1).is_some_and(|token| token.text == ":")
            && inside_binder(tokens, index)
        {
            declare(tokens, index, "parameter", None, declarations, declared);
        }
    }
}

fn declare_next(
    tokens: &[SemanticToken],
    start: usize,
    kind: &str,
    scope: Option<usize>,
    declarations: &mut Vec<Declaration>,
    declared: &mut BTreeSet<usize>,
) {
    if let Some(index) = next_identifier(tokens, start) {
        declare(tokens, index, kind, scope, declarations, declared);
    }
}

pub(super) fn declare(
    tokens: &[SemanticToken],
    token: usize,
    kind: &str,
    scope: Option<usize>,
    declarations: &mut Vec<Declaration>,
    declared: &mut BTreeSet<usize>,
) {
    if !matches!(tokens.get(token), Some(token) if token.kind == TokenKind::Identifier)
        || !declared.insert(token)
    {
        return;
    }
    declarations.push(Declaration {
        token,
        kind: kind.to_string(),
        scope: scope.unwrap_or(tokens[token].scope),
        visibility: Visibility::default(),
    });
}

fn declare_parameters(
    tokens: &[SemanticToken],
    open: Option<usize>,
    close: Option<usize>,
    scope: usize,
    declarations: &mut Vec<Declaration>,
    declared: &mut BTreeSet<usize>,
) {
    let (Some(open), Some(close)) = (open, close) else {
        return;
    };
    let mut segment = open + 1;
    for index in open + 1..=close {
        if index == close || tokens[index].text == "," {
            if let Some(candidate) = next_identifier(tokens, segment).filter(|value| *value < index)
            {
                declare(
                    tokens,
                    candidate,
                    "parameter",
                    Some(scope),
                    declarations,
                    declared,
                );
            }
            segment = index + 1;
        }
    }
}
