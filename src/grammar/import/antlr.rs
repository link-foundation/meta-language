mod lexer;

use lexer::{Lexer, Token, TokenKind};

use super::{parse_error, unsupported_error, GrammarImportError};
use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind};

const FORMAT: GrammarFormat = GrammarFormat::Antlr;

/// Parses ANTLR v4 `.g4` grammar text into the grammar IR.
///
/// The importer is a clean-room parser for the structural ANTLR grammar subset
/// needed by the grammar IR. ANTLR alternatives are lowered as unordered CFG
/// choices, while lexer commands and dropped target-language actions are
/// preserved in rule documentation.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the source cannot be parsed as the
/// supported ANTLR subset or when a parsed prelude construct is recognized but
/// not yet representable.
pub fn import_antlr(text: &str) -> Result<Grammar, GrammarImportError> {
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
            self.collect_comments();
            if self.is_end() {
                break;
            }
            if self.parse_header()? || self.parse_skipped_directive()? {
                continue;
            }
            grammar.add_rule(self.parse_rule()?);
        }

        let Some(start) = grammar
            .rules()
            .iter()
            .find(|rule| rule.kind() == RuleKind::Normal)
            .or_else(|| grammar.rules().first())
            .map(|rule| rule.name.clone())
        else {
            return Err(parse_error(FORMAT, "ANTLR grammar does not contain rules"));
        };
        grammar.set_start(start);
        Ok(grammar)
    }

    fn parse_header(&mut self) -> Result<bool, GrammarImportError> {
        let matched = if self.check_keyword("grammar") {
            self.advance();
            true
        } else if (self.check_keyword("lexer") || self.check_keyword("parser"))
            && self.check_next_keyword("grammar")
        {
            self.advance();
            self.advance();
            true
        } else {
            false
        };

        if !matched {
            return Ok(false);
        }
        self.expect_ident("grammar name")?;
        self.expect_semicolon()?;
        self.pending_comments.clear();
        Ok(true)
    }

    fn parse_skipped_directive(&mut self) -> Result<bool, GrammarImportError> {
        if self.check_any_keyword(&["options", "tokens", "channels"]) {
            self.advance();
            if matches!(self.peek_kind(), Some(TokenKind::Action(_))) {
                self.advance();
                self.try_consume_semicolon();
                self.pending_comments.clear();
                return Ok(true);
            }
            self.skip_until_semicolon()?;
            self.pending_comments.clear();
            return Ok(true);
        }

        if self.check_any_keyword(&["import", "mode"]) {
            self.advance();
            self.skip_until_semicolon()?;
            self.pending_comments.clear();
            return Ok(true);
        }

        Ok(false)
    }

    fn parse_rule(&mut self) -> Result<GrammarRule, GrammarImportError> {
        let comments = std::mem::take(&mut self.pending_comments);
        let fragment = self.try_consume_keyword("fragment");
        let name = self.expect_ident("rule name")?;
        self.reject_rule_prelude()?;
        self.expect_colon()?;

        let mut notes = Vec::new();
        let expr = self.parse_choice(&mut notes)?;
        let command = if self.try_consume_arrow() {
            Some(self.parse_lexer_command()?)
        } else {
            None
        };
        self.expect_semicolon()?;

        let kind = if fragment {
            RuleKind::Silent
        } else if name.chars().next().is_some_and(char::is_uppercase) {
            RuleKind::Token
        } else {
            RuleKind::Normal
        };

        let mut rule = GrammarRule::new(name, expr).with_kind(kind);
        if let Some(doc) = rule_doc(comments, notes, command) {
            rule = rule.with_doc(doc);
        }
        Ok(rule)
    }

    fn reject_rule_prelude(&self) -> Result<(), GrammarImportError> {
        if self.check_colon() {
            return Ok(());
        }
        if let Some(keyword) = self.peek().and_then(Token::ident) {
            if matches!(keyword, "locals" | "returns" | "throws" | "options") {
                return Err(unsupported_error(FORMAT, format!("rule prelude {keyword}")));
            }
        }
        if matches!(self.peek_kind(), Some(TokenKind::CharSet(_))) {
            return Err(unsupported_error(FORMAT, "rule arguments"));
        }
        Err(self.expected("':' before rule body"))
    }

    fn parse_choice(&mut self, notes: &mut Vec<String>) -> Result<GrammarExpr, GrammarImportError> {
        let mut alternatives = Vec::new();
        push_choice_alternative(&mut alternatives, self.parse_sequence(notes)?);
        while self.try_consume_pipe() {
            push_choice_alternative(&mut alternatives, self.parse_sequence(notes)?);
        }
        Ok(finish_choice(alternatives))
    }

    fn parse_sequence(
        &mut self,
        notes: &mut Vec<String>,
    ) -> Result<GrammarExpr, GrammarImportError> {
        let mut items = Vec::new();
        loop {
            self.skip_inline_comments();
            if self.is_sequence_end() {
                break;
            }
            push_sequence_item(&mut items, self.parse_element(notes)?);
        }
        Ok(finish_sequence(items))
    }

    fn parse_element(
        &mut self,
        notes: &mut Vec<String>,
    ) -> Result<GrammarExpr, GrammarImportError> {
        self.skip_inline_comments();
        if matches!(self.peek_kind(), Some(TokenKind::Action(_))) {
            return Ok(self.parse_action(notes));
        }

        if let Some(label) = self.label_ahead() {
            self.advance();
            self.advance();
            let expr = self.parse_prefixed(notes)?;
            return Ok(GrammarExpr::capture(label, expr));
        }

        self.parse_prefixed(notes)
    }

    fn parse_prefixed(
        &mut self,
        notes: &mut Vec<String>,
    ) -> Result<GrammarExpr, GrammarImportError> {
        self.skip_inline_comments();
        let expr = if self.try_consume_tilde() {
            negate_expr(self.parse_atom(notes)?)
        } else {
            self.parse_atom(notes)?
        };
        self.parse_suffixes(expr)
    }

    fn parse_atom(&mut self, notes: &mut Vec<String>) -> Result<GrammarExpr, GrammarImportError> {
        self.skip_inline_comments();
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
                if self.try_consume_range() {
                    let end = self.expect_string("range end")?;
                    let start = single_char(&value, "range start", token.offset)?;
                    let end = single_char(&end, "range end", token.offset)?;
                    if start > end {
                        return Err(error_at(token.offset, "literal range start exceeds end"));
                    }
                    Ok(GrammarExpr::CharRange(start, end))
                } else {
                    Ok(GrammarExpr::Terminal(value))
                }
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
                let expr = self.parse_choice(notes)?;
                self.expect_rparen()?;
                Ok(expr)
            }
            TokenKind::Action(_) => Ok(self.parse_action(notes)),
            _ => Err(self.expected("expression element")),
        }
    }

    fn parse_action(&mut self, notes: &mut Vec<String>) -> GrammarExpr {
        self.advance();
        if self.try_consume_question() {
            notes.push("dropped predicate".to_string());
        } else {
            notes.push("dropped action".to_string());
        }
        GrammarExpr::Empty
    }

    fn parse_suffixes(&mut self, mut expr: GrammarExpr) -> Result<GrammarExpr, GrammarImportError> {
        loop {
            let Some(suffix) = self.consume_suffix() else {
                return Ok(expr);
            };
            expr = match suffix {
                Suffix::Optional => GrammarExpr::optional(expr),
                Suffix::ZeroOrMore => GrammarExpr::zero_or_more(expr),
                Suffix::OneOrMore => GrammarExpr::one_or_more(expr),
            };
            if self.try_consume_question() {
                expr = GrammarExpr::capture("non_greedy", expr);
            }
        }
    }

    fn parse_lexer_command(&mut self) -> Result<String, GrammarImportError> {
        let mut tokens = Vec::new();
        while !self.is_end() && !self.check_semicolon() {
            if matches!(self.peek_kind(), Some(TokenKind::Comment(_))) {
                self.advance();
            } else {
                tokens.push(self.advance().clone());
            }
        }
        if tokens.is_empty() {
            return Err(self.expected("lexer command"));
        }
        Ok(format_command(&tokens))
    }

    fn skip_until_semicolon(&mut self) -> Result<(), GrammarImportError> {
        while !self.is_end() {
            if self.try_consume_semicolon() {
                return Ok(());
            }
            self.advance();
        }
        Err(self.expected("';' after directive"))
    }

    fn collect_comments(&mut self) {
        while let Some(TokenKind::Comment(comment)) = self.peek_kind() {
            self.pending_comments.push(comment.clone());
            self.advance();
        }
    }

    fn skip_inline_comments(&mut self) {
        while matches!(self.peek_kind(), Some(TokenKind::Comment(_))) {
            self.advance();
        }
    }

    fn label_ahead(&self) -> Option<String> {
        let label = self.peek()?.ident()?;
        if matches!(
            self.tokens.get(self.cursor + 1).map(|token| &token.kind),
            Some(TokenKind::Equal | TokenKind::PlusEqual)
        ) {
            Some(label.to_string())
        } else {
            None
        }
    }

    fn consume_suffix(&mut self) -> Option<Suffix> {
        match self.peek_kind() {
            Some(TokenKind::Question) => {
                self.advance();
                Some(Suffix::Optional)
            }
            Some(TokenKind::Star) => {
                self.advance();
                Some(Suffix::ZeroOrMore)
            }
            Some(TokenKind::Plus) => {
                self.advance();
                Some(Suffix::OneOrMore)
            }
            _ => None,
        }
    }

    fn is_sequence_end(&self) -> bool {
        self.is_end()
            || matches!(
                self.peek_kind(),
                Some(TokenKind::Pipe | TokenKind::Semicolon | TokenKind::Arrow | TokenKind::RParen)
            )
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

    fn expect_string(&mut self, role: &str) -> Result<String, GrammarImportError> {
        let Some(token) = self.peek().cloned() else {
            return Err(self.expected(role));
        };
        match token.kind {
            TokenKind::String(value) => {
                self.advance();
                Ok(value)
            }
            _ => Err(self.expected(role)),
        }
    }

    fn expect_colon(&mut self) -> Result<(), GrammarImportError> {
        if self.check_colon() {
            self.advance();
            Ok(())
        } else {
            Err(self.expected("':'"))
        }
    }

    fn expect_semicolon(&mut self) -> Result<(), GrammarImportError> {
        if self.check_semicolon() {
            self.advance();
            Ok(())
        } else {
            Err(self.expected("';'"))
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

    fn expected(&self, expected: &str) -> GrammarImportError {
        let offset = self.peek().map_or(0, |token| token.offset);
        error_at(offset, format!("expected {expected}"))
    }

    fn try_consume_keyword(&mut self, keyword: &str) -> bool {
        if self.check_keyword(keyword) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn try_consume_arrow(&mut self) -> bool {
        if matches!(self.peek_kind(), Some(TokenKind::Arrow)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn try_consume_pipe(&mut self) -> bool {
        if matches!(self.peek_kind(), Some(TokenKind::Pipe)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn try_consume_question(&mut self) -> bool {
        if matches!(self.peek_kind(), Some(TokenKind::Question)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn try_consume_tilde(&mut self) -> bool {
        if matches!(self.peek_kind(), Some(TokenKind::Tilde)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn try_consume_range(&mut self) -> bool {
        if matches!(self.peek_kind(), Some(TokenKind::Range)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn try_consume_semicolon(&mut self) -> bool {
        if self.check_semicolon() {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check_keyword(&self, keyword: &str) -> bool {
        self.peek().is_some_and(|token| token.is_keyword(keyword))
    }

    fn check_next_keyword(&self, keyword: &str) -> bool {
        self.tokens
            .get(self.cursor + 1)
            .is_some_and(|token| token.is_keyword(keyword))
    }

    fn check_any_keyword(&self, keywords: &[&str]) -> bool {
        self.peek()
            .and_then(Token::ident)
            .is_some_and(|value| keywords.contains(&value))
    }

    fn check_colon(&self) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::Colon))
    }

    fn check_semicolon(&self) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::Semicolon))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Suffix {
    Optional,
    ZeroOrMore,
    OneOrMore,
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
            '\\' | '\'' | '"' | '[' | ']' | '-' | '^' => Ok(character),
            character => Ok(character),
        }
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

fn negate_expr(expr: GrammarExpr) -> GrammarExpr {
    match expr {
        GrammarExpr::CharClass {
            negated: false,
            items,
        } => GrammarExpr::CharClass {
            negated: true,
            items,
        },
        expr => GrammarExpr::not(expr),
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

fn single_char(value: &str, role: &str, offset: usize) -> Result<char, GrammarImportError> {
    let mut chars = value.chars();
    let Some(character) = chars.next() else {
        return Err(error_at(
            offset,
            format!("{role} must contain one character"),
        ));
    };
    if chars.next().is_some() {
        return Err(error_at(
            offset,
            format!("{role} {value:?} must contain one character"),
        ));
    }
    Ok(character)
}

fn rule_doc(comments: Vec<String>, notes: Vec<String>, command: Option<String>) -> Option<String> {
    let mut parts = Vec::new();
    parts.extend(comments.into_iter().filter(|comment| !comment.is_empty()));
    parts.extend(notes);
    parts.extend(command);
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("; "))
    }
}

fn format_command(tokens: &[Token]) -> String {
    let mut output = "->".to_string();
    for token in tokens {
        match token.kind {
            TokenKind::LParen => output.push('('),
            TokenKind::RParen => output.push(')'),
            TokenKind::Comma => output.push_str(", "),
            _ => {
                if output == "->" || (!output.ends_with('(') && !output.ends_with(", ")) {
                    output.push(' ');
                }
                output.push_str(&token.text());
            }
        }
    }
    output
}

fn error_at(offset: usize, message: impl Into<String>) -> GrammarImportError {
    parse_error(FORMAT, format!("{} at byte {offset}", message.into()))
}
