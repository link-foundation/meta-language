use super::{parse_error, unsupported_error, GrammarImportError};
use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind};

const FORMAT: GrammarFormat = GrammarFormat::Lark;

/// Parses Lark `.lark` grammar text into the grammar IR.
///
/// The importer is a clean-room recursive-descent parser for Lark's structural
/// EBNF surface: rules, terminals, unordered alternation, grouping, optional
/// groups, postfix repetition, counted `~` repetition, string literals, and
/// regex terminals.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the source cannot be parsed as the
/// supported Lark subset or when an explicitly unsupported directive such as
/// `%import` is encountered.
pub fn import_lark(text: &str) -> Result<Grammar, GrammarImportError> {
    Parser::new(Lexer::new(text).tokenize()?).parse_grammar()
}

#[derive(Clone, Debug)]
struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    pending_comments: Vec<String>,
    ignored: Vec<GrammarExpr>,
    first_rule: Option<String>,
}

impl Parser {
    const fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
            pending_comments: Vec::new(),
            ignored: Vec::new(),
            first_rule: None,
        }
    }

    fn parse_grammar(&mut self) -> Result<Grammar, GrammarImportError> {
        let mut grammar = Grammar::new().with_source_format(FORMAT);
        while !self.is_end() {
            self.collect_comments_and_newlines();
            if self.is_end() {
                break;
            }
            if self.try_consume(|kind| matches!(kind, TokenKind::Percent)) {
                self.parse_directive()?;
            } else {
                let rule = self.parse_rule()?;
                if self.first_rule.is_none() {
                    self.first_rule = Some(rule.name.clone());
                }
                grammar.add_rule(rule);
            }
        }

        self.add_ignore_rules(&mut grammar);

        if grammar.rule("start").is_some() {
            grammar.set_start("start");
        } else if let Some(first_rule) = &self.first_rule {
            grammar.set_start(first_rule.clone());
        } else {
            return Err(parse_error(FORMAT, "Lark grammar does not contain rules"));
        }
        Ok(grammar)
    }

    fn parse_rule(&mut self) -> Result<GrammarRule, GrammarImportError> {
        let comments = std::mem::take(&mut self.pending_comments);
        let inline = self.try_consume(|kind| matches!(kind, TokenKind::Question));
        let name = self.expect_ident("rule name")?;
        let mut notes = Vec::new();
        if self.try_consume(|kind| matches!(kind, TokenKind::Dot)) {
            let priority = self.expect_number("rule priority")?;
            notes.push(format!("priority {priority}"));
        }
        self.expect_colon()?;

        let expr = self.parse_choice()?;
        self.consume_rule_end();

        let kind = if inline {
            notes.insert(0, "inline".to_string());
            RuleKind::Silent
        } else if name.chars().next().is_some_and(char::is_uppercase) {
            RuleKind::Token
        } else {
            RuleKind::Normal
        };

        let mut rule = GrammarRule::new(name, expr).with_kind(kind);
        let doc = rule_doc(comments, notes);
        if let Some(doc) = doc {
            rule = rule.with_doc(doc);
        }
        Ok(rule)
    }

    fn parse_directive(&mut self) -> Result<(), GrammarImportError> {
        let directive = self.expect_ident("directive name")?;
        match directive.as_str() {
            "ignore" => {
                let expr = self.parse_choice()?;
                self.ignored.push(expr);
                self.consume_rule_end();
                Ok(())
            }
            "declare" => {
                self.skip_until_newline();
                Ok(())
            }
            "import" => Err(unsupported_error(FORMAT, "%import")),
            directive => Err(unsupported_error(FORMAT, format!("%{directive}"))),
        }
    }

    fn add_ignore_rules(&mut self, grammar: &mut Grammar) {
        for (index, expr) in self.ignored.drain(..).enumerate() {
            let name = if index == 0 {
                "_ignore".to_string()
            } else {
                format!("_ignore_{}", index + 1)
            };
            grammar.add_rule(
                GrammarRule::new(name, expr)
                    .with_kind(RuleKind::Silent)
                    .with_doc("%ignore"),
            );
        }
    }

    fn parse_choice(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        let mut alternatives = Vec::new();
        push_choice_alternative(&mut alternatives, self.parse_sequence()?);
        while self.try_consume_pipe() {
            push_choice_alternative(&mut alternatives, self.parse_sequence()?);
        }
        Ok(finish_choice(alternatives))
    }

    fn parse_sequence(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        let mut items = Vec::new();
        while !self.is_sequence_end() {
            push_sequence_item(&mut items, self.parse_postfix()?);
        }
        Ok(finish_sequence(items))
    }

    fn parse_postfix(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        let mut expr = self.parse_atom()?;
        loop {
            expr = match self.peek_kind() {
                Some(TokenKind::Question) => {
                    self.advance();
                    GrammarExpr::optional(expr)
                }
                Some(TokenKind::Star) => {
                    self.advance();
                    GrammarExpr::zero_or_more(expr)
                }
                Some(TokenKind::Plus) => {
                    self.advance();
                    GrammarExpr::one_or_more(expr)
                }
                Some(TokenKind::Tilde) => {
                    self.advance();
                    self.parse_tilde_repeat(expr)?
                }
                _ => return Ok(expr),
            };
        }
    }

    fn parse_tilde_repeat(&mut self, expr: GrammarExpr) -> Result<GrammarExpr, GrammarImportError> {
        let min = self.expect_number("minimum repeat count")?;
        let max = if self.try_consume(|kind| matches!(kind, TokenKind::Range)) {
            Some(self.expect_number("maximum repeat count")?)
        } else {
            Some(min)
        };
        if let Some(max) = max {
            if min > max {
                return Err(self.error("repeat minimum exceeds maximum"));
            }
        }
        Ok(GrammarExpr::repeat(expr, min, max))
    }

    fn parse_atom(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        let Some(token) = self.peek().cloned() else {
            return Err(self.expected("expression element"));
        };

        match token.kind {
            TokenKind::Ident(name) => {
                self.advance();
                Ok(GrammarExpr::NonTerminal(name))
            }
            TokenKind::String(value) => {
                self.advance();
                Ok(GrammarExpr::Terminal(value))
            }
            TokenKind::Regex(value) => {
                self.advance();
                lower_regex(&value, token.offset)
            }
            TokenKind::Dot => {
                self.advance();
                Ok(GrammarExpr::AnyChar)
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_choice()?;
                self.expect_rparen()?;
                Ok(expr)
            }
            TokenKind::LBracket => {
                self.advance();
                let expr = self.parse_choice()?;
                self.expect_rbracket()?;
                Ok(GrammarExpr::optional(expr))
            }
            _ => Err(self.expected("expression element")),
        }
    }

    fn collect_comments_and_newlines(&mut self) {
        loop {
            match self.peek_kind() {
                Some(TokenKind::Comment(comment)) => {
                    self.pending_comments.push(comment.clone());
                    self.advance();
                }
                Some(TokenKind::Newline) => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn consume_rule_end(&mut self) {
        if matches!(self.peek_kind(), Some(TokenKind::Comment(_))) {
            self.advance();
        }
        if matches!(self.peek_kind(), Some(TokenKind::Newline)) {
            self.advance();
        }
    }

    fn skip_until_newline(&mut self) {
        while !self.is_end() && !matches!(self.peek_kind(), Some(TokenKind::Newline)) {
            self.advance();
        }
        self.consume_rule_end();
    }

    fn expect_ident(&mut self, role: &str) -> Result<String, GrammarImportError> {
        let Some(token) = self.peek().cloned() else {
            return Err(self.expected(role));
        };
        match token.kind {
            TokenKind::Ident(value) => {
                self.advance();
                Ok(value)
            }
            _ => Err(self.expected(role)),
        }
    }

    fn expect_number(&mut self, role: &str) -> Result<usize, GrammarImportError> {
        let Some(token) = self.peek().cloned() else {
            return Err(self.expected(role));
        };
        match token.kind {
            TokenKind::Number(value) => {
                self.advance();
                Ok(value)
            }
            _ => Err(self.expected(role)),
        }
    }

    fn expect_colon(&mut self) -> Result<(), GrammarImportError> {
        if matches!(self.peek_kind(), Some(TokenKind::Colon)) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected("':'"))
        }
    }

    fn expect_rparen(&mut self) -> Result<(), GrammarImportError> {
        if matches!(self.peek_kind(), Some(TokenKind::RParen)) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected("')'"))
        }
    }

    fn expect_rbracket(&mut self) -> Result<(), GrammarImportError> {
        if matches!(self.peek_kind(), Some(TokenKind::RBracket)) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected("']'"))
        }
    }

    fn try_consume(&mut self, predicate: impl FnOnce(&TokenKind) -> bool) -> bool {
        if self.peek_kind().is_some_and(predicate) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn try_consume_pipe(&mut self) -> bool {
        let start = self.cursor;
        while matches!(
            self.peek_kind(),
            Some(TokenKind::Newline | TokenKind::Comment(_))
        ) {
            self.advance();
        }
        if matches!(self.peek_kind(), Some(TokenKind::Pipe)) {
            self.advance();
            true
        } else {
            self.cursor = start;
            false
        }
    }

    fn is_sequence_end(&self) -> bool {
        self.is_end()
            || matches!(
                self.peek_kind(),
                Some(
                    TokenKind::Newline
                        | TokenKind::Comment(_)
                        | TokenKind::Pipe
                        | TokenKind::RParen
                        | TokenKind::RBracket
                )
            )
    }

    fn expected(&self, expected: &str) -> GrammarImportError {
        self.error(format!("expected {expected}"))
    }

    fn error(&self, message: impl Into<String>) -> GrammarImportError {
        let offset = self.peek().map_or(0, |token| token.offset);
        error_at(offset, message)
    }

    fn is_end(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn peek_kind(&self) -> Option<&TokenKind> {
        self.peek().map(|token| &token.kind)
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.cursor];
        self.cursor += 1;
        token
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TokenKind {
    Ident(String),
    String(String),
    Regex(String),
    Number(usize),
    Comment(String),
    Percent,
    Colon,
    Pipe,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Question,
    Star,
    Plus,
    Tilde,
    Range,
    Dot,
    Newline,
}

#[derive(Clone, Debug)]
struct Lexer<'text> {
    text: &'text str,
    cursor: usize,
}

impl<'text> Lexer<'text> {
    const fn new(text: &'text str) -> Self {
        Self { text, cursor: 0 }
    }

    fn tokenize(mut self) -> Result<Vec<Token>, GrammarImportError> {
        let mut tokens = Vec::new();
        while !self.is_end() {
            self.skip_horizontal_whitespace();
            if self.is_end() {
                break;
            }

            let offset = self.cursor;
            let kind = self.next_token_kind()?;
            tokens.push(Token { kind, offset });
        }
        Ok(tokens)
    }

    fn next_token_kind(&mut self) -> Result<TokenKind, GrammarImportError> {
        if self.starts_with("//") {
            return Ok(TokenKind::Comment(self.line_comment()));
        }
        if self.starts_with("..") {
            self.cursor += 2;
            return Ok(TokenKind::Range);
        }

        let Some(character) = self.peek_char() else {
            return Err(error_at(self.cursor, "unexpected end of input"));
        };

        match character {
            '\n' => {
                self.advance_char();
                Ok(TokenKind::Newline)
            }
            '\r' => {
                self.advance_char();
                if self.peek_char() == Some('\n') {
                    self.advance_char();
                }
                Ok(TokenKind::Newline)
            }
            '"' | '\'' => self.string_literal(character).map(TokenKind::String),
            '/' => self.regex_literal().map(TokenKind::Regex),
            '%' => {
                self.advance_char();
                Ok(TokenKind::Percent)
            }
            ':' => {
                self.advance_char();
                Ok(TokenKind::Colon)
            }
            '|' => {
                self.advance_char();
                Ok(TokenKind::Pipe)
            }
            '(' => {
                self.advance_char();
                Ok(TokenKind::LParen)
            }
            ')' => {
                self.advance_char();
                Ok(TokenKind::RParen)
            }
            '[' => {
                self.advance_char();
                Ok(TokenKind::LBracket)
            }
            ']' => {
                self.advance_char();
                Ok(TokenKind::RBracket)
            }
            '?' => {
                self.advance_char();
                Ok(TokenKind::Question)
            }
            '*' => {
                self.advance_char();
                Ok(TokenKind::Star)
            }
            '+' => {
                self.advance_char();
                Ok(TokenKind::Plus)
            }
            '~' => {
                self.advance_char();
                Ok(TokenKind::Tilde)
            }
            '.' => {
                self.advance_char();
                Ok(TokenKind::Dot)
            }
            character if character.is_ascii_digit() => self.number().map(TokenKind::Number),
            character if is_ident_start(character) => Ok(TokenKind::Ident(self.identifier())),
            character => Err(error_at(
                self.cursor,
                format!("unexpected character {character:?}"),
            )),
        }
    }

    fn line_comment(&mut self) -> String {
        let start = self.cursor;
        while let Some(character) = self.peek_char() {
            if character == '\n' || character == '\r' {
                break;
            }
            self.advance_char();
        }
        self.text[start..self.cursor].trim().to_string()
    }

    fn string_literal(&mut self, quote: char) -> Result<String, GrammarImportError> {
        let start = self.cursor;
        self.advance_char();
        let mut value = String::new();
        while let Some(character) = self.advance_char() {
            match character {
                character if character == quote => return Ok(value),
                '\\' => value.push(self.escape_sequence(start)?),
                character => value.push(character),
            }
        }
        Err(error_at(start, "unterminated string literal"))
    }

    fn regex_literal(&mut self) -> Result<String, GrammarImportError> {
        let start = self.cursor;
        self.advance_char();
        let content_start = self.cursor;
        let mut escaped = false;
        while let Some(character) = self.advance_char() {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '/' {
                let content_end = self.cursor - character.len_utf8();
                return Ok(self.text[content_start..content_end].to_string());
            } else if character == '\n' || character == '\r' {
                return Err(error_at(start, "unterminated regex literal"));
            }
        }
        Err(error_at(start, "unterminated regex literal"))
    }

    fn escape_sequence(&mut self, start: usize) -> Result<char, GrammarImportError> {
        let Some(character) = self.advance_char() else {
            return Err(error_at(start, "unterminated escape sequence"));
        };
        match character {
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            'b' => Ok('\u{08}'),
            'f' => Ok('\u{0c}'),
            '\\' | '"' | '\'' | '/' | '[' | ']' | '-' | '^' => Ok(character),
            'x' => self.hex_escape(start, 2),
            'u' => self.hex_escape(start, 4),
            'U' => self.hex_escape(start, 8),
            character => Ok(character),
        }
    }

    fn hex_escape(&mut self, start: usize, digits: usize) -> Result<char, GrammarImportError> {
        let mut value = 0_u32;
        for _ in 0..digits {
            let Some(character) = self.advance_char() else {
                return Err(error_at(start, "unterminated hexadecimal escape"));
            };
            let Some(digit) = character.to_digit(16) else {
                return Err(error_at(
                    self.cursor - character.len_utf8(),
                    "hexadecimal escape requires hexadecimal digits",
                ));
            };
            value = (value << 4) | digit;
        }
        char::from_u32(value).ok_or_else(|| error_at(start, "invalid hexadecimal escape"))
    }

    fn number(&mut self) -> Result<usize, GrammarImportError> {
        let start = self.cursor;
        let mut value = 0_usize;
        while let Some(character) = self.peek_char() {
            let Some(digit) = character.to_digit(10) else {
                break;
            };
            self.advance_char();
            value = value
                .checked_mul(10)
                .and_then(|current| current.checked_add(digit as usize))
                .ok_or_else(|| error_at(start, "number exceeds usize"))?;
        }
        Ok(value)
    }

    fn identifier(&mut self) -> String {
        let start = self.cursor;
        self.advance_char();
        while self.peek_char().is_some_and(is_ident_continue) {
            self.advance_char();
        }
        self.text[start..self.cursor].to_string()
    }

    fn skip_horizontal_whitespace(&mut self) {
        while self.peek_char().is_some_and(|character| {
            character != '\n' && character != '\r' && character.is_whitespace()
        }) {
            self.advance_char();
        }
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.text[self.cursor..].starts_with(prefix)
    }

    const fn is_end(&self) -> bool {
        self.cursor >= self.text.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }

    fn advance_char(&mut self) -> Option<char> {
        let character = self.peek_char()?;
        self.cursor += character.len_utf8();
        Some(character)
    }
}

fn lower_regex(value: &str, offset: usize) -> Result<GrammarExpr, GrammarImportError> {
    trivial_regex_char_class(value).map_or_else(
        || {
            Ok(GrammarExpr::capture(
                "regex",
                GrammarExpr::Terminal(value.to_string()),
            ))
        },
        |content| lower_char_set(content, offset),
    )
}

fn trivial_regex_char_class(value: &str) -> Option<&str> {
    if (!value.starts_with('[') || !value.ends_with(']'))
        || has_unescaped_inner_closing_bracket(value)
    {
        None
    } else {
        Some(&value[1..value.len() - 1])
    }
}

fn has_unescaped_inner_closing_bracket(value: &str) -> bool {
    let mut escaped = false;
    for (index, character) in value.char_indices().skip(1) {
        if index == value.len() - 1 {
            return false;
        }
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == ']' {
            return true;
        }
    }
    false
}

fn lower_char_set(content: &str, offset: usize) -> Result<GrammarExpr, GrammarImportError> {
    let mut scanner = ClassScanner::new(content, offset);
    let negated = scanner.try_consume('^');
    let mut items = Vec::new();
    while !scanner.is_end() {
        let start = scanner.read_char()?;
        if scanner.try_consume_range_separator() {
            let end = scanner.read_char()?;
            if start > end {
                return Err(error_at(offset, "character class range start exceeds end"));
            }
            items.push(CharClassItem::Range(start, end));
        } else {
            items.push(CharClassItem::Char(start));
        }
    }
    if items.is_empty() {
        return Err(error_at(offset, "character class must not be empty"));
    }
    Ok(GrammarExpr::CharClass { negated, items })
}

#[derive(Clone, Debug)]
struct ClassScanner<'text> {
    text: &'text str,
    cursor: usize,
    offset: usize,
}

impl<'text> ClassScanner<'text> {
    const fn new(text: &'text str, offset: usize) -> Self {
        Self {
            text,
            cursor: 0,
            offset,
        }
    }

    fn read_char(&mut self) -> Result<char, GrammarImportError> {
        let Some(character) = self.advance_char() else {
            return Err(error_at(self.offset, "unexpected end of character class"));
        };
        if character == '\\' {
            self.read_escape()
        } else {
            Ok(character)
        }
    }

    fn read_escape(&mut self) -> Result<char, GrammarImportError> {
        let Some(character) = self.advance_char() else {
            return Err(error_at(self.offset, "unterminated character class escape"));
        };
        match character {
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            'b' => Ok('\u{08}'),
            'f' => Ok('\u{0c}'),
            '\\' | '"' | '\'' | '/' | '[' | ']' | '-' | '^' => Ok(character),
            'x' => self.hex_escape(2),
            'u' => self.hex_escape(4),
            'U' => self.hex_escape(8),
            character => Ok(character),
        }
    }

    fn hex_escape(&mut self, digits: usize) -> Result<char, GrammarImportError> {
        let mut value = 0_u32;
        for _ in 0..digits {
            let Some(character) = self.advance_char() else {
                return Err(error_at(self.offset, "unterminated hexadecimal escape"));
            };
            let Some(digit) = character.to_digit(16) else {
                return Err(error_at(
                    self.offset + self.cursor.saturating_sub(character.len_utf8()),
                    "hexadecimal escape requires hexadecimal digits",
                ));
            };
            value = (value << 4) | digit;
        }
        char::from_u32(value).ok_or_else(|| error_at(self.offset, "invalid hexadecimal escape"))
    }

    fn try_consume(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.advance_char();
            true
        } else {
            false
        }
    }

    fn try_consume_range_separator(&mut self) -> bool {
        if self.peek_char() == Some('-') && self.has_char_after_current() {
            self.advance_char();
            true
        } else {
            false
        }
    }

    fn has_char_after_current(&self) -> bool {
        let mut chars = self.text[self.cursor..].chars();
        chars.next();
        chars.next().is_some()
    }

    const fn is_end(&self) -> bool {
        self.cursor >= self.text.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }

    fn advance_char(&mut self) -> Option<char> {
        let character = self.peek_char()?;
        self.cursor += character.len_utf8();
        Some(character)
    }
}

fn finish_sequence(items: Vec<GrammarExpr>) -> GrammarExpr {
    match items.len() {
        0 => GrammarExpr::Empty,
        1 => items.into_iter().next().expect("one sequence item exists"),
        _ => GrammarExpr::Sequence(items),
    }
}

fn push_sequence_item(items: &mut Vec<GrammarExpr>, item: GrammarExpr) {
    match item {
        GrammarExpr::Empty => {}
        GrammarExpr::Sequence(nested) => {
            for item in nested {
                push_sequence_item(items, item);
            }
        }
        item => items.push(item),
    }
}

fn finish_choice(alternatives: Vec<GrammarExpr>) -> GrammarExpr {
    if alternatives.iter().all(|expr| expr == &GrammarExpr::Empty) {
        return GrammarExpr::Empty;
    }
    match alternatives.len() {
        0 => GrammarExpr::Empty,
        1 => alternatives
            .into_iter()
            .next()
            .expect("one alternative exists"),
        _ => GrammarExpr::Choice {
            ordered: false,
            alternatives,
        },
    }
}

fn push_choice_alternative(alternatives: &mut Vec<GrammarExpr>, alternative: GrammarExpr) {
    match alternative {
        GrammarExpr::Choice {
            ordered: false,
            alternatives: nested,
        } => alternatives.extend(nested),
        alternative => alternatives.push(alternative),
    }
}

fn rule_doc(comments: Vec<String>, notes: Vec<String>) -> Option<String> {
    let mut parts = Vec::new();
    parts.extend(comments.into_iter().filter(|comment| !comment.is_empty()));
    parts.extend(notes);
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("; "))
    }
}

fn error_at(offset: usize, message: impl Into<String>) -> GrammarImportError {
    parse_error(FORMAT, format!("{} at byte {offset}", message.into()))
}

const fn is_ident_start(character: char) -> bool {
    character == '_' || character.is_ascii_alphabetic()
}

const fn is_ident_continue(character: char) -> bool {
    character == '_' || character.is_ascii_alphanumeric()
}
