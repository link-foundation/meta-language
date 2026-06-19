use super::{parse_error, GrammarImportError};
use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule};

const FORMAT: GrammarFormat = GrammarFormat::Gbnf;

/// Parses llama.cpp GBNF grammar text into the grammar IR.
///
/// The importer is a clean-room recursive-descent parser for the structural
/// GBNF subset represented by the grammar IR: rules, unordered alternation,
/// grouping, postfix repetition, string literals, and character classes.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the source cannot be parsed as GBNF or
/// when the required `root` rule is missing.
pub fn import_gbnf(text: &str) -> Result<Grammar, GrammarImportError> {
    Parser::new(Lexer::new(text).tokenize()?).parse_grammar()
}

#[derive(Clone, Debug)]
struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    pending_comments: Vec<String>,
}

impl Parser {
    const fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
            pending_comments: Vec::new(),
        }
    }

    fn parse_grammar(&mut self) -> Result<Grammar, GrammarImportError> {
        let mut grammar = Grammar::new().with_source_format(FORMAT);
        while !self.is_end() {
            self.collect_comments_and_newlines();
            if self.is_end() {
                break;
            }
            grammar.add_rule(self.parse_rule()?);
        }

        if grammar.rule("root").is_some() {
            grammar.set_start("root");
            Ok(grammar)
        } else {
            Err(parse_error(
                FORMAT,
                "GBNF grammar does not contain root rule",
            ))
        }
    }

    fn parse_rule(&mut self) -> Result<GrammarRule, GrammarImportError> {
        let comments = std::mem::take(&mut self.pending_comments);
        let name = self.expect_ident("rule name")?;
        self.expect_define()?;
        let expr = self.parse_choice()?;
        self.consume_rule_end();

        let mut rule = GrammarRule::new(name, expr);
        if !comments.is_empty() {
            rule = rule.with_doc(comments.join("\n"));
        }
        Ok(rule)
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
                Some(TokenKind::LBrace) => self.parse_counted_repeat(expr)?,
                _ => return Ok(expr),
            };
        }
    }

    fn parse_counted_repeat(
        &mut self,
        expr: GrammarExpr,
    ) -> Result<GrammarExpr, GrammarImportError> {
        self.expect_lbrace()?;
        let min = self.expect_number("minimum repeat count")?;
        let max = if self.try_consume_comma() {
            if matches!(self.peek_kind(), Some(TokenKind::Number(_))) {
                Some(self.expect_number("maximum repeat count")?)
            } else {
                None
            }
        } else {
            Some(min)
        };
        self.expect_rbrace()?;
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
            TokenKind::CharSet(content) => {
                self.advance();
                lower_char_set(&content, token.offset)
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

    fn expect_define(&mut self) -> Result<(), GrammarImportError> {
        if matches!(self.peek_kind(), Some(TokenKind::Define)) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected("'::='"))
        }
    }

    fn expect_lbrace(&mut self) -> Result<(), GrammarImportError> {
        if matches!(self.peek_kind(), Some(TokenKind::LBrace)) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected("'{'"))
        }
    }

    fn expect_rbrace(&mut self) -> Result<(), GrammarImportError> {
        if matches!(self.peek_kind(), Some(TokenKind::RBrace)) {
            self.advance();
            Ok(())
        } else {
            Err(self.expected("'}'"))
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

    fn try_consume_comma(&mut self) -> bool {
        if matches!(self.peek_kind(), Some(TokenKind::Comma)) {
            self.advance();
            true
        } else {
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
    CharSet(String),
    Number(usize),
    Comment(String),
    Define,
    Pipe,
    LParen,
    RParen,
    Question,
    Star,
    Plus,
    LBrace,
    RBrace,
    Comma,
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
        if self.starts_with("::=") {
            self.cursor += 3;
            return Ok(TokenKind::Define);
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
            '#' => Ok(TokenKind::Comment(self.line_comment())),
            '"' => self.string_literal().map(TokenKind::String),
            '[' => self.char_set().map(TokenKind::CharSet),
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
            '{' => {
                self.advance_char();
                Ok(TokenKind::LBrace)
            }
            '}' => {
                self.advance_char();
                Ok(TokenKind::RBrace)
            }
            ',' => {
                self.advance_char();
                Ok(TokenKind::Comma)
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

    fn string_literal(&mut self) -> Result<String, GrammarImportError> {
        let start = self.cursor;
        self.advance_char();
        let mut value = String::new();
        while let Some(character) = self.advance_char() {
            match character {
                '"' => return Ok(value),
                '\\' => value.push(self.escape_sequence(start)?),
                character => value.push(character),
            }
        }
        Err(error_at(start, "unterminated string literal"))
    }

    fn char_set(&mut self) -> Result<String, GrammarImportError> {
        let start = self.cursor;
        self.advance_char();
        let content_start = self.cursor;
        let mut escaped = false;
        while let Some(character) = self.advance_char() {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == ']' {
                let content_end = self.cursor - character.len_utf8();
                return Ok(self.text[content_start..content_end].to_string());
            }
        }
        Err(error_at(start, "unterminated character class"))
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
            '\\' | '"' | '\'' | '[' | ']' | '-' | '^' => Ok(character),
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
            '\\' | '"' | '\'' | '[' | ']' | '-' | '^' => Ok(character),
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

fn error_at(offset: usize, message: impl Into<String>) -> GrammarImportError {
    parse_error(FORMAT, format!("{} at byte {offset}", message.into()))
}

const fn is_ident_start(character: char) -> bool {
    character == '_' || character.is_ascii_alphabetic()
}

const fn is_ident_continue(character: char) -> bool {
    character == '_' || character == '-' || character.is_ascii_alphanumeric()
}
