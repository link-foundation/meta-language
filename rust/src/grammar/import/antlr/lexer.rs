use super::error_at;
use crate::grammar::import::GrammarImportError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Token {
    pub(super) kind: TokenKind,
    pub(super) offset: usize,
}

impl Token {
    pub(super) fn ident(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Ident(value) => Some(value),
            _ => None,
        }
    }

    pub(super) fn is_keyword(&self, keyword: &str) -> bool {
        self.ident() == Some(keyword)
    }

    pub(super) fn text(&self) -> String {
        match &self.kind {
            TokenKind::Ident(value) | TokenKind::Comment(value) => value.clone(),
            TokenKind::String(value) => format!("'{}'", escape_literal(value)),
            TokenKind::CharSet(value) => format!("[{value}]"),
            TokenKind::Action(value) => format!("{{{value}}}"),
            TokenKind::Colon => ":".to_string(),
            TokenKind::Semicolon => ";".to_string(),
            TokenKind::Pipe => "|".to_string(),
            TokenKind::LParen => "(".to_string(),
            TokenKind::RParen => ")".to_string(),
            TokenKind::Question => "?".to_string(),
            TokenKind::Star => "*".to_string(),
            TokenKind::Plus => "+".to_string(),
            TokenKind::Tilde => "~".to_string(),
            TokenKind::Dot => ".".to_string(),
            TokenKind::Equal => "=".to_string(),
            TokenKind::PlusEqual => "+=".to_string(),
            TokenKind::Arrow => "->".to_string(),
            TokenKind::Range => "..".to_string(),
            TokenKind::Comma => ",".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum TokenKind {
    Ident(String),
    String(String),
    CharSet(String),
    Action(String),
    Comment(String),
    Colon,
    Semicolon,
    Pipe,
    LParen,
    RParen,
    Question,
    Star,
    Plus,
    Tilde,
    Dot,
    Equal,
    PlusEqual,
    Arrow,
    Range,
    Comma,
}

#[derive(Clone, Debug)]
pub(super) struct Lexer<'text> {
    text: &'text str,
    cursor: usize,
}

impl<'text> Lexer<'text> {
    pub(super) const fn new(text: &'text str) -> Self {
        Self { text, cursor: 0 }
    }

    pub(super) fn tokenize(mut self) -> Result<Vec<Token>, GrammarImportError> {
        let mut tokens = Vec::new();
        while !self.is_end() {
            self.skip_whitespace();
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
        if self.starts_with("/*") {
            return self.block_comment().map(TokenKind::Comment);
        }

        let Some(character) = self.peek_char() else {
            return Err(error_at(self.cursor, "unexpected end of input"));
        };

        match character {
            '\'' => self.string_literal().map(TokenKind::String),
            '[' => self.char_set().map(TokenKind::CharSet),
            '{' => self.action_block().map(TokenKind::Action),
            ':' => {
                self.advance_char();
                Ok(TokenKind::Colon)
            }
            ';' => {
                self.advance_char();
                Ok(TokenKind::Semicolon)
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
            '?' => {
                self.advance_char();
                Ok(TokenKind::Question)
            }
            '*' => {
                self.advance_char();
                Ok(TokenKind::Star)
            }
            '+' if self.starts_with("+=") => {
                self.cursor += 2;
                Ok(TokenKind::PlusEqual)
            }
            '+' => {
                self.advance_char();
                Ok(TokenKind::Plus)
            }
            '~' => {
                self.advance_char();
                Ok(TokenKind::Tilde)
            }
            '.' if self.starts_with("..") => {
                self.cursor += 2;
                Ok(TokenKind::Range)
            }
            '.' => {
                self.advance_char();
                Ok(TokenKind::Dot)
            }
            '=' => {
                self.advance_char();
                Ok(TokenKind::Equal)
            }
            '-' if self.starts_with("->") => {
                self.cursor += 2;
                Ok(TokenKind::Arrow)
            }
            ',' => {
                self.advance_char();
                Ok(TokenKind::Comma)
            }
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

    fn block_comment(&mut self) -> Result<String, GrammarImportError> {
        let start = self.cursor;
        self.cursor += 2;
        while !self.is_end() {
            if self.starts_with("*/") {
                self.cursor += 2;
                return Ok(self.text[start..self.cursor].trim().to_string());
            }
            self.advance_char();
        }
        Err(error_at(start, "unterminated block comment"))
    }

    fn string_literal(&mut self) -> Result<String, GrammarImportError> {
        let start = self.cursor;
        self.advance_char();
        let mut value = String::new();
        while let Some(character) = self.advance_char() {
            match character {
                '\'' => return Ok(value),
                '\\' => value.push(self.escape_sequence(start)?),
                character => value.push(character),
            }
        }
        Err(error_at(start, "unterminated string literal"))
    }

    fn char_set(&mut self) -> Result<String, GrammarImportError> {
        let start = self.cursor;
        self.advance_char();
        let mut escaped = false;
        let content_start = self.cursor;
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
        Err(error_at(start, "unterminated character set"))
    }

    fn action_block(&mut self) -> Result<String, GrammarImportError> {
        let start = self.cursor;
        self.advance_char();
        let content_start = self.cursor;
        let mut depth = 1_usize;
        while let Some(character) = self.advance_char() {
            match character {
                '\'' | '"' => self.skip_quoted_in_action(character, start)?,
                '/' if self.starts_with("/") => self.skip_line_comment_in_action(),
                '/' if self.starts_with("*") => self.skip_block_comment_in_action(start)?,
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        let content_end = self.cursor - character.len_utf8();
                        return Ok(self.text[content_start..content_end].to_string());
                    }
                }
                _ => {}
            }
        }
        Err(error_at(start, "unterminated action block"))
    }

    fn skip_quoted_in_action(
        &mut self,
        quote: char,
        action_start: usize,
    ) -> Result<(), GrammarImportError> {
        let mut escaped = false;
        while let Some(character) = self.advance_char() {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == quote {
                return Ok(());
            }
        }
        Err(error_at(action_start, "unterminated quoted action text"))
    }

    fn skip_line_comment_in_action(&mut self) {
        self.advance_char();
        while let Some(character) = self.peek_char() {
            if character == '\n' || character == '\r' {
                break;
            }
            self.advance_char();
        }
    }

    fn skip_block_comment_in_action(
        &mut self,
        action_start: usize,
    ) -> Result<(), GrammarImportError> {
        self.advance_char();
        while !self.is_end() {
            if self.starts_with("*/") {
                self.cursor += 2;
                return Ok(());
            }
            self.advance_char();
        }
        Err(error_at(
            action_start,
            "unterminated block comment in action",
        ))
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
            '\\' | '\'' | '"' | '[' | ']' | '-' => Ok(character),
            'u' => self.unicode_escape(start),
            character => Ok(character),
        }
    }

    fn unicode_escape(&mut self, start: usize) -> Result<char, GrammarImportError> {
        let mut value = 0_u32;
        for _ in 0..4 {
            let Some(character) = self.advance_char() else {
                return Err(error_at(start, "unterminated unicode escape"));
            };
            let Some(digit) = character.to_digit(16) else {
                return Err(error_at(
                    self.cursor - character.len_utf8(),
                    "unicode escape requires hexadecimal digits",
                ));
            };
            value = (value << 4) | digit;
        }
        char::from_u32(value).ok_or_else(|| error_at(start, "invalid unicode escape"))
    }

    fn identifier(&mut self) -> String {
        let start = self.cursor;
        self.advance_char();
        while self.peek_char().is_some_and(is_ident_continue) {
            self.advance_char();
        }
        self.text[start..self.cursor].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.peek_char().is_some_and(char::is_whitespace) {
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

fn escape_literal(value: &str) -> String {
    value
        .chars()
        .flat_map(char::escape_default)
        .collect::<String>()
        .replace('\'', "\\'")
}

const fn is_ident_start(character: char) -> bool {
    character == '_' || character.is_ascii_alphabetic()
}

const fn is_ident_continue(character: char) -> bool {
    character == '_' || character.is_ascii_alphanumeric()
}
