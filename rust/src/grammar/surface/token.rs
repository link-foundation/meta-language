use super::{skeleton_error, GrammarSurfaceError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Delimiter {
    Round,
    Square,
    Brace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum QuoteKind {
    Single,
    Double,
    Backtick,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Token {
    Atom(String),
    Quoted {
        value: String,
        quote: QuoteKind,
    },
    Group {
        delimiter: Delimiter,
        tokens: Vec<Self>,
    },
}

impl Token {
    pub(super) fn atom(&self) -> Option<&str> {
        match self {
            Self::Atom(value) => Some(value),
            Self::Quoted { .. } | Self::Group { .. } => None,
        }
    }

    pub(super) fn is_atom(&self, expected: &str) -> bool {
        self.atom() == Some(expected)
    }
}

pub(super) fn parse_surface_tokens(text: &str) -> Result<Vec<Token>, GrammarSurfaceError> {
    SurfaceTokenizer::new(text).parse_document()
}

struct SurfaceTokenizer<'input> {
    text: &'input str,
    cursor: usize,
}

impl<'input> SurfaceTokenizer<'input> {
    const fn new(text: &'input str) -> Self {
        Self { text, cursor: 0 }
    }

    fn parse_document(mut self) -> Result<Vec<Token>, GrammarSurfaceError> {
        self.parse_tokens_until(None)
    }

    fn parse_tokens_until(
        &mut self,
        closing: Option<char>,
    ) -> Result<Vec<Token>, GrammarSurfaceError> {
        let mut tokens = Vec::new();
        while self.cursor < self.text.len() {
            self.skip_whitespace();
            if self.cursor >= self.text.len() {
                break;
            }

            let character = self.current_char().expect("cursor is inside text");
            if Some(character) == closing {
                self.cursor += character.len_utf8();
                return Ok(tokens);
            }

            let token = match character {
                '(' => self.parse_group(Delimiter::Round, ')')?,
                '[' => self.parse_group(Delimiter::Square, ']')?,
                '{' => self.parse_group(Delimiter::Brace, '}')?,
                ')' | ']' | '}' => {
                    return Err(skeleton_error(format!(
                        "unexpected closing delimiter {character:?}"
                    )));
                }
                '\'' | '"' | '`' => self.parse_quoted(character)?,
                ':' | ',' | '?' | '*' | '+' | '/' | '|' | '&' | '!' | '.' | '^' => {
                    self.cursor += character.len_utf8();
                    Token::Atom(character.to_string())
                }
                _ => self.parse_atom(),
            };
            tokens.push(token);
        }

        if let Some(closing) = closing {
            return Err(skeleton_error(format!(
                "missing closing delimiter {closing:?}"
            )));
        }
        Ok(tokens)
    }

    fn parse_group(
        &mut self,
        delimiter: Delimiter,
        closing: char,
    ) -> Result<Token, GrammarSurfaceError> {
        self.cursor += 1;
        let tokens = self.parse_tokens_until(Some(closing))?;
        Ok(Token::Group { delimiter, tokens })
    }

    fn parse_quoted(&mut self, quote: char) -> Result<Token, GrammarSurfaceError> {
        self.cursor += quote.len_utf8();

        let mut value = String::new();
        while self.cursor < self.text.len() {
            let remaining = &self.text[self.cursor..];
            if remaining.starts_with(quote) {
                let after = &remaining[quote.len_utf8()..];
                if after.starts_with(quote) {
                    value.push(quote);
                    self.cursor += quote.len_utf8() * 2;
                    continue;
                }
                self.cursor += quote.len_utf8();
                return Ok(Token::Quoted {
                    value,
                    quote: quote_kind(quote),
                });
            }

            let character = remaining
                .chars()
                .next()
                .expect("remaining text is non-empty");
            value.push(character);
            self.cursor += character.len_utf8();
        }

        Err(skeleton_error(format!(
            "unterminated quoted string starting with {quote:?}"
        )))
    }

    fn parse_atom(&mut self) -> Token {
        let start = self.cursor;
        while self.cursor < self.text.len() {
            let character = self.current_char().expect("cursor is inside text");
            if character.is_whitespace()
                || matches!(
                    character,
                    '(' | ')'
                        | '['
                        | ']'
                        | '{'
                        | '}'
                        | '\''
                        | '"'
                        | '`'
                        | ':'
                        | ','
                        | '?'
                        | '*'
                        | '+'
                        | '/'
                        | '|'
                        | '&'
                        | '!'
                        | '.'
                        | '^'
                )
            {
                break;
            }
            self.cursor += character.len_utf8();
        }
        Token::Atom(self.text[start..self.cursor].to_string())
    }

    fn skip_whitespace(&mut self) {
        while self.current_char().is_some_and(char::is_whitespace) {
            self.cursor += self
                .current_char()
                .expect("cursor is inside text")
                .len_utf8();
        }
    }

    fn current_char(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }
}

fn quote_kind(quote: char) -> QuoteKind {
    match quote {
        '\'' => QuoteKind::Single,
        '"' => QuoteKind::Double,
        '`' => QuoteKind::Backtick,
        _ => unreachable!("only quote characters reach quote_kind"),
    }
}
