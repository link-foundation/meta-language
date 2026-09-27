//! Syntax-driven binding extents for the four-language program analyzer: the
//! binders the token declarers cannot see (Rust patterns, closures, macro
//! metavariables; Lean and Rocq binder groups, `fun` and quantifiers), the
//! source window each declaration is visible in, and the identifiers that
//! interpolated strings expose to name resolution.
//! Mirrors `js/src/program-binding-extents.js`.

use super::declarations::declare;
use super::{
    char_at, identifier_continue, identifier_start, mark, BTreeMap, BTreeSet, Declaration,
    ProgramRange, ProgramSourceMapping, SemanticToken, TokenKind,
};

const RUST_ITEMS: &[&str] = &[
    "struct", "enum", "trait", "type", "const", "static", "mod", "union", "function", "import",
];
const FORMAT_MACROS: &[&str] = &[
    "format",
    "format_args",
    "print",
    "println",
    "eprint",
    "eprintln",
    "panic",
    "unreachable",
    "todo",
    "unimplemented",
];
const OPENERS: &[&str] = &["(", "[", "{", "⦃"];
const CLOSERS: &[&str] = &[")", "]", "}", "⦄"];

/// The source window `[from, until)` a declaration is visible in, the body
/// scope of a Rust module, and whether only `$name` uses may see it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Visibility {
    pub(super) from: usize,
    pub(super) until: usize,
    pub(super) body: Option<usize>,
    pub(super) sigil: bool,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            from: 0,
            until: usize::MAX,
            body: None,
            sigil: false,
        }
    }
}

fn proof_extents(language: &str) -> &'static [&'static str] {
    match language {
        "Lean" => &["fun", "quantifier", "definition", "example", "constructor"],
        "Rocq" => &["lambda_function", "quantifier_term", "sentence"],
        _ => &[],
    }
}

fn proof_binder_nodes(language: &str) -> &'static [&'static str] {
    match language {
        "Lean" => &[
            "explicit_binder",
            "implicit_binder",
            "instance_binder",
            "strict_implicit_binder",
        ],
        "Rocq" => &["binder"],
        _ => &[],
    }
}

pub(super) fn is_rust_item(kind: &str) -> bool {
    RUST_ITEMS.contains(&kind)
}

/// Re-exposes identifiers that name bindings from inside string literals: Lean
/// `s!`/`m!`/`f!` interpolations and inline arguments of Rust format macros.
pub(super) fn unmask_interpolations(
    mask: &mut [bool],
    source: &str,
    language: &str,
    syntax: &[ProgramSourceMapping],
) {
    if language == "Lean" {
        for fact in syntax.iter().filter(|fact| fact.term == "interpolation") {
            if fact.range.end - fact.range.start >= 2 {
                mark(mask, fact.range.start + 1, fact.range.end - 1, true);
            }
        }
        return;
    }
    if language != "Rust" {
        return;
    }
    let literals = syntax
        .iter()
        .filter(|fact| fact.term == "string_literal")
        .map(|fact| (fact.range.start, fact.range))
        .collect::<BTreeMap<_, _>>();
    let bytes = source.as_bytes();
    for fact in syntax.iter().filter(|fact| fact.term == "macro_invocation") {
        let start = fact.range.start;
        let mut offset = start;
        while offset < bytes.len()
            && (bytes[offset].is_ascii_alphanumeric() || bytes[offset] == b'_')
        {
            offset += 1;
        }
        if !FORMAT_MACROS.contains(&&source[start..offset]) {
            continue;
        }
        offset = skip_space(source, offset);
        if bytes.get(offset) != Some(&b'!') {
            continue;
        }
        offset = skip_space(source, offset + 1);
        if bytes.get(offset) != Some(&b'(') {
            continue;
        }
        if let Some(literal) = literals.get(&skip_space(source, offset + 1)) {
            if bytes[literal.start] == b'"' {
                unmask_format_arguments(mask, source, literal.start + 1, literal.end - 1);
            }
        }
    }
}

fn unmask_format_arguments(mask: &mut [bool], source: &str, start: usize, end: usize) {
    let bytes = source.as_bytes();
    let mut offset = start;
    while offset < end {
        if source[offset..].starts_with("{{") || source[offset..].starts_with("}}") {
            offset += 2;
        } else if bytes[offset] == b'{' {
            let mut name_end = offset + 1;
            while name_end < end && bytes[name_end] != b'}' && bytes[name_end] != b':' {
                name_end += 1;
            }
            if is_identifier(&source[offset + 1..name_end], "Rust") {
                mark(mask, offset + 1, name_end, true);
            }
            offset = name_end;
        } else {
            offset += 1;
        }
    }
}

/// Declares binders that only the concrete syntax tree delimits.
pub(super) fn declare_syntax_binders(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    language: &str,
    declarations: &mut Vec<Declaration>,
    declared: &mut BTreeSet<usize>,
) {
    let token_at = token_starts(tokens);
    for fact in syntax {
        let Some(&first) = token_at.get(&fact.range.start) else {
            continue;
        };
        let mut binder = |index: usize, kind: &str| {
            declare(tokens, index, kind, None, declarations, declared);
        };
        let term = fact.term.as_str();
        if language == "JavaScript" {
            if term == "arrow_function" {
                let (open, close) = arrow_parameters(tokens, first);
                for index in open..=close {
                    let previous = index
                        .checked_sub(1)
                        .map(|previous| tokens[previous].text.as_str());
                    if tokens
                        .get(index)
                        .is_some_and(|token| token.kind == TokenKind::Identifier)
                        && (index == open || matches!(previous, Some("(" | "," | "...")))
                    {
                        binder(index, "parameter");
                    }
                }
            }
        } else if language == "Rust" {
            match term {
                "token_binding_pattern" => {
                    if let Some(&name) = token_at.get(&(fact.range.start + 1)) {
                        if tokens[first].text == "$" {
                            binder(name, "metavariable");
                        }
                    }
                }
                "let_declaration" | "let_condition" => {
                    let stops =
                        |token: &SemanticToken| matches!(token.text.as_str(), "=" | ";" | "else");
                    declare_rust_pattern(
                        tokens,
                        first + 1,
                        fact.range.end,
                        "let",
                        &mut binder,
                        stops,
                    );
                }
                "for_expression" => {
                    let stops = |token: &SemanticToken| token.text == "in";
                    declare_rust_pattern(
                        tokens,
                        first + 1,
                        fact.range.end,
                        "for",
                        &mut binder,
                        stops,
                    );
                }
                "closure_parameters" => {
                    let end = fact.range.end - 1;
                    declare_rust_pattern(tokens, first + 1, end, "parameter", &mut binder, |_| {
                        false
                    });
                }
                _ => {}
            }
        } else if proof_binder_nodes(language).contains(&term) {
            for (index, token) in tokens.iter().enumerate().skip(first) {
                if token.range.start >= fact.range.end || token.text == ":" {
                    break;
                }
                if token.kind == TokenKind::Identifier && token.text != "_" {
                    binder(index, "parameter");
                }
            }
        } else if language == "Lean"
            && matches!(term, "fun" | "quantifier")
            && tokens[first].range.end < fact.range.end
        {
            let mut index = first + 1;
            while let Some(token) = tokens
                .get(index)
                .filter(|token| token.range.start < fact.range.end)
            {
                if token.kind == TokenKind::Identifier {
                    if token.text != "_" {
                        binder(index, "binder");
                    }
                    index += 1;
                } else if OPENERS.contains(&token.text.as_str()) {
                    index = closing_index(tokens, index) + 1;
                } else {
                    break;
                }
            }
        }
    }
}

// Pattern bindings: identifiers in binding position up to the pattern end,
// skipping type annotations, constructor paths, and struct field names.
fn declare_rust_pattern(
    tokens: &[SemanticToken],
    first: usize,
    end: usize,
    kind: &str,
    binder: &mut impl FnMut(usize, &str),
    stops: impl Fn(&SemanticToken) -> bool,
) {
    let mut depth = 0_isize;
    let mut annotation = false;
    let text = |index: usize| tokens.get(index).map(|token| token.text.as_str());
    for (index, token) in tokens.iter().enumerate().skip(first) {
        if token.range.start >= end || (depth == 0 && stops(token)) {
            return;
        }
        if OPENERS.contains(&token.text.as_str()) {
            depth += 1;
        } else if CLOSERS.contains(&token.text.as_str()) {
            depth -= 1;
            if depth < 0 {
                return;
            }
        } else if depth == 0 && token.text == ":" {
            annotation = true;
        } else if depth == 0 && token.text == "," {
            annotation = false;
        } else if !annotation
            && token.kind == TokenKind::Identifier
            && token.text != "_"
            && !matches!(text(index + 1), Some("(" | "{" | "::" | "!"))
            && (depth == 0 || text(index + 1) != Some(":"))
            && !matches!(index.checked_sub(1).and_then(text), Some("::" | "."))
        {
            binder(index, kind);
        }
    }
}

/// Sets the source window each declaration is visible in, the body scope of
/// Rust modules, and the scope of proof-language binder groups.
pub(super) fn apply_binding_extents(
    declarations: &mut [Declaration],
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    language: &str,
    brace_scopes: &BTreeMap<usize, usize>,
) {
    let token_at = token_starts(tokens);
    for declaration in declarations.iter_mut() {
        let token = &tokens[declaration.token];
        declaration.visibility = Visibility {
            from: token.range.start,
            ..Visibility::default()
        };
        if language == "JavaScript" {
            declaration.visibility.from = 0;
            if declaration.kind == "parameter" {
                let arrow = smallest(syntax, &["arrow_function"], token.range, None, |fact| {
                    token_at.get(&fact.range.start).is_some_and(|&first| {
                        declaration.token <= arrow_parameters(tokens, first).1
                    })
                });
                if let Some(arrow) = arrow {
                    declaration.visibility.until = arrow.range.end;
                }
            }
        } else if language == "Rust" {
            rust_extent(declaration, tokens, syntax, brace_scopes);
        } else {
            proof_extent(declaration, tokens, syntax, language, &token_at);
        }
    }
}

fn rust_extent(
    declaration: &mut Declaration,
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    brace_scopes: &BTreeMap<usize, usize>,
) {
    let range = tokens[declaration.token].range;
    let visibility = &mut declaration.visibility;
    match declaration.kind.as_str() {
        kind if is_rust_item(kind) => {
            visibility.from = 0;
            if kind == "mod"
                && tokens
                    .get(declaration.token + 1)
                    .is_some_and(|next| next.text == "{")
            {
                visibility.body = brace_scopes.get(&(declaration.token + 1)).copied();
            }
        }
        "let" => {
            let terms = &["let_declaration", "let_condition"];
            if let Some(statement) = smallest(syntax, terms, range, None, |_| true) {
                visibility.from = statement.range.end;
                if statement.term == "let_condition" {
                    let owners = &["if_expression", "while_expression"];
                    let owner = smallest(
                        syntax,
                        owners,
                        statement.range,
                        Some(&statement.term),
                        |_| true,
                    );
                    if let Some(owner) = owner {
                        visibility.until = owner.range.end;
                    }
                }
            }
        }
        "for" => {
            if let Some(looped) = smallest(syntax, &["for_expression"], range, None, |_| true) {
                let body = syntax
                    .iter()
                    .find(|fact| fact.term == "block" && fact.range.end == looped.range.end);
                visibility.from = body.map_or(looped.range.end, |body| body.range.start);
                visibility.until = looped.range.end;
            }
        }
        "parameter" => {
            let parameters = smallest(syntax, &["closure_parameters"], range, None, |_| true);
            let closure = parameters.and_then(|parameters| {
                smallest(
                    syntax,
                    &["closure_expression"],
                    parameters.range,
                    Some(&parameters.term),
                    |_| true,
                )
            });
            if let (Some(parameters), Some(closure)) = (parameters, closure) {
                visibility.from = parameters.range.end;
                visibility.until = closure.range.end;
            }
        }
        "metavariable" => {
            if let Some(rule) = smallest(syntax, &["macro_rule"], range, None, |_| true) {
                visibility.until = rule.range.end;
            }
            visibility.sigil = true;
        }
        _ => {}
    }
}

fn proof_extent(
    declaration: &mut Declaration,
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    language: &str,
    token_at: &BTreeMap<usize, usize>,
) {
    let range = tokens[declaration.token].range;
    if declaration.kind == "let" {
        let terms: &[&str] = if language == "Lean" {
            &["let"]
        } else {
            &["let_expression"]
        };
        let expression = smallest(syntax, terms, range, None, |fact| {
            fact.range.start < range.start
        });
        if let Some(expression) = expression {
            declaration.visibility.from = if language == "Lean" {
                lean_let_value_end(tokens, syntax, expression.range)
            } else {
                rocq_let_body_start(tokens, expression.range, token_at)
            };
            declaration.visibility.until = expression.range.end;
        }
        return;
    }
    if declaration.kind != "parameter" && declaration.kind != "binder" {
        return;
    }
    let group = smallest(syntax, proof_binder_nodes(language), range, None, |_| true);
    if group.is_none() && declaration.kind != "binder" {
        return;
    }
    let extent = smallest(syntax, proof_extents(language), range, None, |_| true);
    if let Some(extent) = extent {
        declaration.visibility.until = extent.range.end;
    }
    if let Some(&anchor) = extent
        .or(group)
        .and_then(|fact| token_at.get(&fact.range.start))
    {
        declaration.scope = tokens[anchor].scope;
    }
}

// Token range of an arrow function's parameter list: a bare identifier, or
// the tokens inside its parentheses, after an optional `async`.
fn arrow_parameters(tokens: &[SemanticToken], first: usize) -> (usize, usize) {
    let text = |index: usize| tokens.get(index).map(|token| token.text.as_str());
    let mut open = first;
    if text(open) == Some("async") && text(open + 1) != Some("=>") {
        open += 1;
    }
    if text(open) != Some("(") {
        return (open, open);
    }
    let close = closing_index(tokens, open);
    (open + 1, close - 1)
}

fn lean_let_value_end(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    expression: ProgramRange,
) -> usize {
    let value = tokens
        .iter()
        .position(|token| token.range.start > expression.start && token.text == ":=")
        .and_then(|assign| tokens.get(assign + 1))
        .filter(|value| value.range.start < expression.end);
    let Some(value) = value else {
        return expression.end;
    };
    syntax
        .iter()
        .filter(|fact| fact.range.start == value.range.start && fact.range.end < expression.end)
        .map(|fact| fact.range.end)
        .fold(value.range.end, usize::max)
}

fn rocq_let_body_start(
    tokens: &[SemanticToken],
    expression: ProgramRange,
    token_at: &BTreeMap<usize, usize>,
) -> usize {
    let Some(&first) = token_at.get(&expression.start) else {
        return expression.end;
    };
    let mut depth = 0_usize;
    for token in tokens
        .iter()
        .skip(first + 1)
        .take_while(|token| token.range.start < expression.end)
    {
        if token.text == "let" {
            depth += 1;
        }
        if token.text == "in" {
            if depth == 0 {
                return token.range.end;
            }
            depth -= 1;
        }
    }
    expression.end
}

/// Rust `use a::b::name;` declarations that bring one item into scope.
pub(super) fn rust_use_aliases(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
) -> Vec<usize> {
    let token_at = token_starts(tokens);
    let text = |index: usize| tokens.get(index).map(|token| token.text.as_str());
    syntax
        .iter()
        .filter(|fact| fact.term == "use_declaration")
        .filter_map(|fact| {
            let last = tokens.iter().rposition(|token| {
                token.range.start >= fact.range.start
                    && token.range.end < fact.range.end
                    && token.kind == TokenKind::Identifier
            })?;
            let first = *token_at.get(&fact.range.start)?;
            let plain = last > 0
                && text(last - 1) == Some("::")
                && text(last + 1) == Some(";")
                && !tokens[first.min(last)..last]
                    .iter()
                    .any(|token| matches!(token.text.as_str(), "{" | "*" | "as"));
            plain.then_some(last)
        })
        .collect()
}

fn is_identifier(value: &str, language: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|character| identifier_start(character, language))
        && characters.all(|character| identifier_continue(character, language))
}

// The smallest fact with an accepted term that contains `range`, other than
// the fact `range` itself.
fn smallest<'a>(
    syntax: &'a [ProgramSourceMapping],
    terms: &[&str],
    range: ProgramRange,
    term: Option<&str>,
    extra: impl Fn(&ProgramSourceMapping) -> bool,
) -> Option<&'a ProgramSourceMapping> {
    let mut best: Option<&ProgramSourceMapping> = None;
    for fact in syntax {
        if !terms.contains(&fact.term.as_str())
            || fact.range.start > range.start
            || fact.range.end < range.end
            || (fact.range == range && term == Some(fact.term.as_str()))
            || !extra(fact)
        {
            continue;
        }
        if best.map_or(true, |best| {
            fact.range.end - fact.range.start < best.range.end - best.range.start
        }) {
            best = Some(fact);
        }
    }
    best
}

fn closing_index(tokens: &[SemanticToken], open: usize) -> usize {
    let mut depth = 0_isize;
    for (index, token) in tokens.iter().enumerate().skip(open) {
        if OPENERS.contains(&token.text.as_str()) {
            depth += 1;
        }
        if CLOSERS.contains(&token.text.as_str()) {
            depth -= 1;
            if depth == 0 {
                return index;
            }
        }
    }
    tokens.len()
}

fn skip_space(source: &str, offset: usize) -> usize {
    let mut cursor = offset;
    while cursor < source.len() && char_at(source, cursor).is_whitespace() {
        cursor += char_at(source, cursor).len_utf8();
    }
    cursor
}

fn token_starts(tokens: &[SemanticToken]) -> BTreeMap<usize, usize> {
    tokens
        .iter()
        .enumerate()
        .map(|(index, token)| (token.range.start, index))
        .collect()
}
