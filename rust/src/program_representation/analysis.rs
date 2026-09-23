use super::{
    BTreeMap, BTreeSet, LinkNetwork, LinkType, ProgramBinding, ProgramFact, ProgramRange,
    ProgramRepresentationError, ProgramScope, ProgramSourceMapping, TokenKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SemanticToken {
    pub(super) kind: TokenKind,
    pub(super) text: String,
    pub(super) range: ProgramRange,
    scope: usize,
}

#[derive(Clone, Debug)]
struct Declaration {
    token: usize,
    kind: String,
    scope: usize,
}

pub(super) fn syntax_facts(network: &LinkNetwork) -> Vec<ProgramSourceMapping> {
    network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
        .filter_map(|link| {
            let span = link.metadata().span()?;
            Some(ProgramSourceMapping {
                link_id: link.id().as_u64(),
                term: link.metadata().term()?.to_string(),
                range: ProgramRange::new(span.byte_range().start(), span.byte_range().end()),
            })
        })
        .collect()
}

pub(super) fn semantic_tokens(
    source: &str,
    language: &str,
    syntax: &[ProgramSourceMapping],
) -> Vec<SemanticToken> {
    let mut mask = code_mask(source, language);
    if language == "JavaScript" {
        for fact in syntax
            .iter()
            .filter(|fact| fact.term.to_ascii_lowercase().contains("regex"))
        {
            mark(&mut mask, fact.range.start, fact.range.end, false);
        }
    }
    let identifiers = syntax
        .iter()
        .filter(|fact| fact.term == "identifier")
        .map(|fact| fact.range)
        .collect::<BTreeSet<_>>();
    let mut result = Vec::new();
    let mut offset = 0;
    while offset < source.len() {
        let character = char_at(source, offset);
        if !mask[offset] || character.is_whitespace() {
            offset += character.len_utf8();
            continue;
        }
        if identifier_start(character, language) {
            let start = offset;
            offset += character.len_utf8();
            while offset < source.len() {
                let next = char_at(source, offset);
                if !mask[offset] || !identifier_continue(next, language) {
                    break;
                }
                offset += next.len_utf8();
            }
            let text = &source[start..offset];
            let range = ProgramRange::new(start, offset);
            let kind = if identifiers.contains(&range) || !is_keyword(language, text) {
                TokenKind::Identifier
            } else {
                TokenKind::Keyword
            };
            result.push(SemanticToken {
                kind,
                text: text.to_string(),
                range,
                scope: 0,
            });
            continue;
        }
        let operator = OPERATORS
            .iter()
            .find(|operator| source[offset..].starts_with(**operator));
        let end = offset + operator.map_or_else(|| character.len_utf8(), |operator| operator.len());
        result.push(SemanticToken {
            kind: TokenKind::Punctuation,
            text: operator.map_or_else(|| character.to_string(), ToString::to_string),
            range: ProgramRange::new(offset, end),
            scope: 0,
        });
        offset = end;
    }
    result
}

pub(super) fn resolve_bindings(
    input: &[SemanticToken],
    source_len: usize,
    language: &str,
) -> (Vec<ProgramScope>, Vec<ProgramBinding>, Vec<ProgramFact>) {
    let mut tokens = input.to_vec();
    let mut scopes = vec![ProgramScope {
        id: "scope:0".to_string(),
        parent: None,
        range: ProgramRange::new(0, source_len),
        depth: 0,
    }];
    let mut stack = vec![0_usize];
    let mut brace_scopes = BTreeMap::new();
    for (index, token) in tokens.iter_mut().enumerate() {
        token.scope = *stack.last().expect("root scope");
        if token.text == "{" {
            let parent = *stack.last().expect("root scope");
            let scope = scopes.len();
            scopes.push(ProgramScope {
                id: format!("scope:{scope}"),
                parent: Some(scopes[parent].id.clone()),
                range: ProgramRange::new(token.range.end, source_len),
                depth: stack.len(),
            });
            brace_scopes.insert(index, scope);
            stack.push(scope);
        } else if token.text == "}" && stack.len() > 1 {
            let closed = stack.pop().expect("non-root scope");
            scopes[closed].range.end = token.range.start;
            token.scope = *stack.last().expect("root scope");
        }
    }
    let mut declarations = Vec::new();
    let mut declared = BTreeSet::new();
    match language {
        "JavaScript" => {
            declare_javascript(&tokens, &brace_scopes, &mut declarations, &mut declared);
        }
        "Rust" => declare_rust(&tokens, &brace_scopes, &mut declarations, &mut declared),
        _ => declare_proof_language(&tokens, language, &mut declarations, &mut declared),
    }
    declarations.sort_by_key(|declaration| tokens[declaration.token].range.start);
    let mut bindings = declarations
        .into_iter()
        .map(|declaration| {
            let token = &tokens[declaration.token];
            (
                declaration.token,
                ProgramBinding {
                    id: format!("{language}:{}", token.range.start),
                    name: token.text.clone(),
                    kind: declaration.kind,
                    scope: scopes[declaration.scope].id.clone(),
                    declaration: token.range,
                    references: Vec::new(),
                },
            )
        })
        .collect::<Vec<_>>();
    let binding_tokens = bindings
        .iter()
        .enumerate()
        .map(|(binding, (token, _))| (*token, binding))
        .collect::<BTreeMap<_, _>>();
    let mut unresolved = Vec::new();
    for (token_index, token) in tokens.iter().enumerate() {
        if token.kind != TokenKind::Identifier || binding_tokens.contains_key(&token_index) {
            continue;
        }
        let mut candidates = bindings
            .iter()
            .enumerate()
            .filter(|(_, (_, binding))| {
                binding.name == token.text
                    && binding.declaration.start <= token.range.start
                    && scope_contains(&scopes, &binding.scope, token.scope)
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        candidates.sort_by_key(|index| {
            let binding = &bindings[*index].1;
            (
                scopes[scope_index(&scopes, &binding.scope)].depth,
                binding.declaration.start,
            )
        });
        if let Some(binding) = candidates.last().copied() {
            bindings[binding].1.references.push(token.range);
        } else {
            unresolved.push(ProgramFact::new("unresolved", &token.text, token.range));
        }
    }
    (
        scopes,
        bindings.into_iter().map(|(_, binding)| binding).collect(),
        unresolved,
    )
}

fn declare_javascript(
    tokens: &[SemanticToken],
    brace_scopes: &BTreeMap<usize, usize>,
    declarations: &mut Vec<Declaration>,
    declared: &mut BTreeSet<usize>,
) {
    for (index, token) in tokens.iter().enumerate() {
        if matches!(token.text.as_str(), "const" | "let" | "var" | "class") {
            declare_next(tokens, index + 1, &token.text, None, declarations, declared);
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

fn declare_rust(
    tokens: &[SemanticToken],
    brace_scopes: &BTreeMap<usize, usize>,
    declarations: &mut Vec<Declaration>,
    declared: &mut BTreeSet<usize>,
) {
    for (index, token) in tokens.iter().enumerate() {
        if token.text == "let" {
            declare_next(tokens, index + 1, "let", None, declarations, declared);
        }
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
        if token.text == "as" {
            declare_next(tokens, index + 1, "import", None, declarations, declared);
        }
    }
}

fn declare_proof_language(
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

fn declare(
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

fn code_mask(source: &str, language: &str) -> Vec<bool> {
    let mut mask = vec![true; source.len()];
    let line_comment = match language {
        "Lean" => Some("--"),
        "Rocq" => None,
        _ => Some("//"),
    };
    let (block_open, block_close) = match language {
        "Lean" => ("/-", "-/"),
        "Rocq" => ("(*", "*)"),
        _ => ("/*", "*/"),
    };
    let mut offset = 0;
    while offset < source.len() {
        if line_comment.is_some_and(|comment| source[offset..].starts_with(comment)) {
            let end = source[offset..]
                .find('\n')
                .map_or(source.len(), |end| offset + end);
            mark(&mut mask, offset, end, false);
            offset = end;
        } else if source[offset..].starts_with(block_open) {
            offset = mask_nested(source, &mut mask, offset, block_open, block_close);
        } else {
            let character = char_at(source, offset);
            if character == '"' || (matches!(language, "JavaScript" | "Rust") && character == '\'')
            {
                offset = mask_quoted(source, &mut mask, offset, character);
            } else if language == "JavaScript" && character == '`' {
                offset = mask_template(source, &mut mask, offset);
            } else {
                offset += character.len_utf8();
            }
        }
    }
    mask
}

fn mask_quoted(source: &str, mask: &mut [bool], start: usize, quote: char) -> usize {
    let mut offset = start;
    mark(mask, offset, offset + quote.len_utf8(), false);
    offset += quote.len_utf8();
    while offset < source.len() {
        let character = char_at(source, offset);
        mark(mask, offset, offset + character.len_utf8(), false);
        offset += character.len_utf8();
        if character == '\\' && offset < source.len() {
            let escaped = char_at(source, offset);
            mark(mask, offset, offset + escaped.len_utf8(), false);
            offset += escaped.len_utf8();
        } else if character == quote {
            break;
        }
    }
    offset
}

fn mask_nested(source: &str, mask: &mut [bool], start: usize, open: &str, close: &str) -> usize {
    let mut offset = start;
    let mut depth = 0_usize;
    while offset < source.len() {
        if source[offset..].starts_with(open) {
            mark(mask, offset, offset + open.len(), false);
            depth += 1;
            offset += open.len();
        } else if source[offset..].starts_with(close) {
            mark(mask, offset, offset + close.len(), false);
            depth = depth.saturating_sub(1);
            offset += close.len();
            if depth == 0 {
                break;
            }
        } else {
            let character = char_at(source, offset);
            mark(mask, offset, offset + character.len_utf8(), false);
            offset += character.len_utf8();
        }
    }
    offset
}

fn mask_template(source: &str, mask: &mut [bool], start: usize) -> usize {
    let mut offset = start;
    mask[offset] = false;
    offset += 1;
    while offset < source.len() {
        if source[offset..].starts_with('\\') {
            mark(mask, offset, (offset + 2).min(source.len()), false);
            offset = (offset + 2).min(source.len());
        } else if source[offset..].starts_with('`') {
            mask[offset] = false;
            offset += 1;
            break;
        } else if source[offset..].starts_with("${") {
            mark(mask, offset, offset + 2, false);
            offset = mask_template_expression(source, mask, offset + 2);
        } else {
            let character = char_at(source, offset);
            mark(mask, offset, offset + character.len_utf8(), false);
            offset += character.len_utf8();
        }
    }
    offset
}

fn mask_template_expression(source: &str, mask: &mut [bool], start: usize) -> usize {
    let mut offset = start;
    let mut depth = 1_usize;
    while offset < source.len() && depth > 0 {
        if source[offset..].starts_with("//") {
            let end = source[offset..]
                .find('\n')
                .map_or(source.len(), |end| offset + end);
            mark(mask, offset, end, false);
            offset = end;
        } else if source[offset..].starts_with("/*") {
            offset = mask_nested(source, mask, offset, "/*", "*/");
        } else {
            let character = char_at(source, offset);
            match character {
                '"' | '\'' => offset = mask_quoted(source, mask, offset, character),
                '`' => offset = mask_template(source, mask, offset),
                '{' => {
                    depth += 1;
                    offset += 1;
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        mask[offset] = false;
                    }
                    offset += 1;
                }
                _ => offset += character.len_utf8(),
            }
        }
    }
    offset
}

pub(super) fn validate_identifier(
    identifier: &str,
    language: &str,
) -> Result<(), ProgramRepresentationError> {
    let mut characters = identifier.chars();
    let valid = characters
        .next()
        .is_some_and(|character| identifier_start(character, language))
        && characters.all(|character| identifier_continue(character, language))
        && !is_keyword(language, identifier);
    if valid {
        Ok(())
    } else {
        Err(ProgramRepresentationError::InvalidIdentifier {
            language: language.to_string(),
            identifier: identifier.to_string(),
        })
    }
}

fn identifier_start(character: char, language: &str) -> bool {
    character == '_' || character.is_alphabetic() || (language == "JavaScript" && character == '$')
}

fn identifier_continue(character: char, language: &str) -> bool {
    identifier_start(character, language)
        || character.is_alphanumeric()
        || character == '\u{200c}'
        || character == '\u{200d}'
        || (language != "Rust" && character == '\'')
}

fn scope_contains(scopes: &[ProgramScope], declaration_scope: &str, reference: usize) -> bool {
    let mut current = Some(reference);
    while let Some(index) = current {
        if scopes[index].id == declaration_scope {
            return true;
        }
        current = scopes[index]
            .parent
            .as_deref()
            .map(|parent| scope_index(scopes, parent));
    }
    false
}

fn scope_index(scopes: &[ProgramScope], id: &str) -> usize {
    scopes
        .iter()
        .position(|scope| scope.id == id)
        .expect("binding scope exists")
}

pub(super) fn scope_by_id<'a>(scopes: &'a [ProgramScope], id: &str) -> &'a ProgramScope {
    &scopes[scope_index(scopes, id)]
}

pub(super) const fn ranges_overlap(left: ProgramRange, right: ProgramRange) -> bool {
    left.start <= right.end && right.start <= left.end
}

fn next_identifier(tokens: &[SemanticToken], start: usize) -> Option<usize> {
    tokens
        .iter()
        .enumerate()
        .skip(start)
        .find_map(|(index, token)| (token.kind == TokenKind::Identifier).then_some(index))
}

fn find_token(tokens: &[SemanticToken], start: usize, text: &str) -> Option<usize> {
    tokens
        .iter()
        .enumerate()
        .skip(start)
        .find_map(|(index, token)| (token.text == text).then_some(index))
}

fn matching_delimiter(
    tokens: &[SemanticToken],
    start: usize,
    open: &str,
    close: &str,
) -> Option<usize> {
    let mut depth = 0_usize;
    for (index, token) in tokens.iter().enumerate().skip(start) {
        if token.text == open {
            depth += 1;
        }
        if token.text == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

fn inside_binder(tokens: &[SemanticToken], index: usize) -> bool {
    let mut depth = 0_usize;
    for token in tokens[..index].iter().rev() {
        if matches!(token.text.as_str(), ")" | "}") {
            depth += 1;
        }
        if matches!(token.text.as_str(), "(" | "{") {
            if depth == 0 {
                return true;
            }
            depth -= 1;
        }
        if depth == 0 && matches!(token.text.as_str(), ";" | ":=" | ".") {
            return false;
        }
    }
    false
}

pub(super) fn unique_facts(facts: Vec<ProgramFact>) -> Vec<ProgramFact> {
    let mut seen = BTreeSet::new();
    facts
        .into_iter()
        .filter(|fact| {
            seen.insert((
                fact.kind.clone(),
                fact.name.clone(),
                fact.range.start,
                fact.range.end,
            ))
        })
        .collect()
}

fn mark(mask: &mut [bool], start: usize, end: usize, value: bool) {
    mask[start..end].fill(value);
}

fn char_at(source: &str, offset: usize) -> char {
    source[offset..].chars().next().expect("character boundary")
}

fn is_keyword(language: &str, value: &str) -> bool {
    keywords(language).contains(&value)
}

fn keywords(language: &str) -> &'static [&'static str] {
    match language {
        "JavaScript" => &[
            "as",
            "async",
            "await",
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "default",
            "delete",
            "do",
            "else",
            "export",
            "extends",
            "finally",
            "for",
            "from",
            "function",
            "if",
            "import",
            "in",
            "instanceof",
            "let",
            "new",
            "of",
            "return",
            "static",
            "super",
            "switch",
            "throw",
            "try",
            "typeof",
            "var",
            "void",
            "while",
            "with",
            "yield",
        ],
        "Rust" => &[
            "as",
            "async",
            "await",
            "break",
            "const",
            "continue",
            "crate",
            "dyn",
            "else",
            "enum",
            "extern",
            "false",
            "fn",
            "for",
            "if",
            "impl",
            "in",
            "let",
            "loop",
            "macro_rules",
            "match",
            "mod",
            "move",
            "mut",
            "pub",
            "ref",
            "return",
            "self",
            "Self",
            "static",
            "struct",
            "super",
            "trait",
            "true",
            "type",
            "union",
            "unsafe",
            "use",
            "where",
            "while",
        ],
        "Lean" => &[
            "axiom",
            "by",
            "class",
            "constant",
            "def",
            "deriving",
            "do",
            "else",
            "end",
            "export",
            "for",
            "from",
            "fun",
            "if",
            "import",
            "in",
            "inductive",
            "instance",
            "let",
            "macro",
            "match",
            "mutual",
            "namespace",
            "notation",
            "open",
            "partial",
            "postfix",
            "prefix",
            "private",
            "protected",
            "section",
            "structure",
            "syntax",
            "theorem",
            "universe",
            "variable",
            "where",
            "with",
        ],
        _ => &[
            "Axiom",
            "Class",
            "CoFixpoint",
            "Definition",
            "End",
            "Fixpoint",
            "From",
            "Import",
            "Inductive",
            "Lemma",
            "Ltac",
            "Module",
            "Notation",
            "Parameter",
            "Proof",
            "Qed",
            "Record",
            "Require",
            "Section",
            "Theorem",
            "Universe",
            "Variable",
            "as",
            "at",
            "end",
            "fix",
            "forall",
            "fun",
            "if",
            "in",
            "let",
            "match",
            "return",
            "then",
            "with",
        ],
    }
}

fn declaration_markers(language: &str) -> &'static [&'static str] {
    if language == "Lean" {
        &[
            "def",
            "theorem",
            "lemma",
            "axiom",
            "constant",
            "inductive",
            "structure",
            "class",
            "namespace",
            "section",
            "variable",
            "macro",
        ]
    } else {
        &[
            "Definition",
            "Theorem",
            "Lemma",
            "Axiom",
            "Parameter",
            "Variable",
            "Inductive",
            "Record",
            "Class",
            "Module",
            "Section",
            "Fixpoint",
            "CoFixpoint",
            "Ltac",
        ]
    }
}

pub(super) fn module_markers(language: &str) -> &'static [&'static str] {
    match language {
        "JavaScript" => &["import", "export", "from"],
        "Rust" => &["use", "mod", "crate"],
        "Lean" => &["import", "namespace", "open", "export"],
        _ => &["From", "Require", "Import", "Module", "Section"],
    }
}

pub(super) fn extension_markers(language: &str) -> &'static [&'static str] {
    match language {
        "JavaScript" => &["String"],
        "Rust" => &["macro_rules", "#"],
        "Lean" => &[
            "macro", "notation", "syntax", "postfix", "prefix", "infix", "infixl", "infixr", "@",
        ],
        _ => &["Notation", "Ltac", "#"],
    }
}

pub(super) fn proof_markers(language: &str) -> &'static [&'static str] {
    if language == "Lean" {
        &["theorem", "lemma", "by", "rfl", "simp", "exact", "apply"]
    } else {
        &[
            "Theorem",
            "Lemma",
            "Proof",
            "Qed",
            "Defined",
            "reflexivity",
            "intros",
            "exact",
            "apply",
        ]
    }
}

pub(super) fn effect_markers(language: &str) -> &'static [&'static str] {
    match language {
        "JavaScript" => &["async", "await", "throw", "yield"],
        "Rust" => &["async", "await", "unsafe", "panic"],
        "Lean" => &["IO", "do", "pure"],
        _ => &["Proof", "Ltac", "Qed"],
    }
}

const OPERATORS: [&str; 22] = [
    "...", "::=", "=>", "->", ":=", "::", "==", "!=", "<=", ">=", "&&", "||", "?.", "??", "**",
    "++", "--", "${", "→", "←", "≤", "≥",
];
