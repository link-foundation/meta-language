use super::GraphQlAdapterError;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Document {
    pub operation: super::GraphQlOperationType,
    pub root_fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Field {
    pub alias: Option<String>,
    pub name: String,
    pub arguments: Vec<Argument>,
    pub selection: Vec<Self>,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Argument {
    pub name: String,
    pub value: ValueNode,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ValueNode {
    pub value: ValueKind,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum ValueKind {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Enum(String),
    List(Vec<ValueNode>),
    Object(Vec<(String, ValueNode)>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ByteSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum TokenKind {
    Name(String),
    String(String),
    Integer(i64),
    Float(f64),
    Punctuation(char),
    End,
}

#[derive(Clone, Debug, PartialEq)]
struct Token {
    kind: TokenKind,
    span: ByteSpan,
}

pub(super) fn parse(source: &str) -> Result<Document, GraphQlAdapterError> {
    Parser::new(lex(source)?).parse_document()
}

fn lex(source: &str) -> Result<Vec<Token>, GraphQlAdapterError> {
    let mut tokens = Vec::new();
    let mut cursor = 0;
    while cursor < source.len() {
        let character = source[cursor..]
            .chars()
            .next()
            .expect("cursor is within source");
        if character.is_whitespace() || character == ',' {
            cursor += character.len_utf8();
            continue;
        }
        if character == '#' {
            cursor += character.len_utf8();
            while cursor < source.len() {
                let comment_character = source[cursor..]
                    .chars()
                    .next()
                    .expect("cursor is within source");
                cursor += comment_character.len_utf8();
                if comment_character == '\n' {
                    break;
                }
            }
            continue;
        }

        let start = cursor;
        if is_name_start(character) {
            cursor += character.len_utf8();
            while cursor < source.len() {
                let next = source[cursor..]
                    .chars()
                    .next()
                    .expect("cursor is within source");
                if !is_name_continue(next) {
                    break;
                }
                cursor += next.len_utf8();
            }
            tokens.push(Token {
                kind: TokenKind::Name(source[start..cursor].to_string()),
                span: ByteSpan { start, end: cursor },
            });
            continue;
        }
        if character == '"' {
            cursor = string_end(source, cursor)?;
            let raw = &source[start..cursor];
            let decoded: String = serde_json::from_str(raw).map_err(|error| {
                GraphQlAdapterError::new(format!("invalid GraphQL string at byte {start}: {error}"))
            })?;
            tokens.push(Token {
                kind: TokenKind::String(decoded),
                span: ByteSpan { start, end: cursor },
            });
            continue;
        }
        if character == '-' || character.is_ascii_digit() {
            cursor = number_end(source, cursor);
            let raw = &source[start..cursor];
            let kind = if raw.contains(['.', 'e', 'E']) {
                let value = raw.parse::<f64>().map_err(|error| {
                    GraphQlAdapterError::new(format!(
                        "invalid GraphQL number at byte {start}: {error}"
                    ))
                })?;
                if !value.is_finite() {
                    return Err(GraphQlAdapterError::new(format!(
                        "non-finite GraphQL number at byte {start}"
                    )));
                }
                TokenKind::Float(value)
            } else {
                TokenKind::Integer(raw.parse::<i64>().map_err(|error| {
                    GraphQlAdapterError::new(format!(
                        "invalid GraphQL integer at byte {start}: {error}"
                    ))
                })?)
            };
            tokens.push(Token {
                kind,
                span: ByteSpan { start, end: cursor },
            });
            continue;
        }
        if "!$():=@[]{|}".contains(character) {
            cursor += character.len_utf8();
            tokens.push(Token {
                kind: TokenKind::Punctuation(character),
                span: ByteSpan { start, end: cursor },
            });
            continue;
        }
        return Err(GraphQlAdapterError::new(format!(
            "unsupported GraphQL token {character:?} at byte {start}"
        )));
    }
    tokens.push(Token {
        kind: TokenKind::End,
        span: ByteSpan {
            start: source.len(),
            end: source.len(),
        },
    });
    Ok(tokens)
}

fn string_end(source: &str, start: usize) -> Result<usize, GraphQlAdapterError> {
    if source[start..].starts_with("\"\"\"") {
        return Err(GraphQlAdapterError::new(format!(
            "GraphQL block strings are not supported at byte {start}"
        )));
    }
    let mut escaped = false;
    let mut cursor = start + 1;
    while cursor < source.len() {
        let character = source[cursor..]
            .chars()
            .next()
            .expect("cursor is within source");
        cursor += character.len_utf8();
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == '"' {
            return Ok(cursor);
        } else if character == '\n' || character == '\r' {
            break;
        }
    }
    Err(GraphQlAdapterError::new(format!(
        "unterminated GraphQL string at byte {start}"
    )))
}

fn number_end(source: &str, start: usize) -> usize {
    let mut cursor = start;
    while cursor < source.len() {
        let character = source[cursor..]
            .chars()
            .next()
            .expect("cursor is within source");
        if !matches!(character, '-' | '+' | '.' | 'e' | 'E') && !character.is_ascii_digit() {
            break;
        }
        cursor += character.len_utf8();
    }
    cursor
}

const fn is_name_start(character: char) -> bool {
    character == '_' || character.is_ascii_alphabetic()
}

const fn is_name_continue(character: char) -> bool {
    is_name_start(character) || character.is_ascii_digit()
}

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    const fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn parse_document(mut self) -> Result<Document, GraphQlAdapterError> {
        let operation = if self.take_name("query") {
            super::GraphQlOperationType::Query
        } else if self.take_name("mutation") {
            super::GraphQlOperationType::Mutation
        } else if self.at_punctuation('{') {
            super::GraphQlOperationType::Query
        } else {
            return self.error("expected query, mutation, or shorthand selection");
        };

        if matches!(self.current().kind, TokenKind::Name(_)) {
            self.cursor += 1;
        }
        if self.at_punctuation('(') {
            return self.error("GraphQL variables are not supported; bind literal values first");
        }
        let root_fields = self.parse_selection_set()?;
        if !matches!(self.current().kind, TokenKind::End) {
            return self
                .error("multiple operations, fragments, and trailing tokens are unsupported");
        }
        Ok(Document {
            operation,
            root_fields,
        })
    }

    fn parse_selection_set(&mut self) -> Result<Vec<Field>, GraphQlAdapterError> {
        self.expect_punctuation('{')?;
        let mut fields = Vec::new();
        while !self.at_punctuation('}') {
            if matches!(self.current().kind, TokenKind::End) {
                return self.error("unterminated selection set");
            }
            fields.push(self.parse_field()?);
        }
        self.expect_punctuation('}')?;
        if fields.is_empty() {
            return self.error("empty selection sets are unsupported");
        }
        Ok(fields)
    }

    fn parse_field(&mut self) -> Result<Field, GraphQlAdapterError> {
        let first = self.expect_name()?;
        let start = first.1.start;
        let (alias, name) = if self.take_punctuation(':') {
            (Some(first.0), self.expect_name()?.0)
        } else {
            (None, first.0)
        };
        let arguments = if self.take_punctuation('(') {
            let mut arguments = Vec::new();
            while !self.at_punctuation(')') {
                if matches!(self.current().kind, TokenKind::End) {
                    return self.error("unterminated field arguments");
                }
                let (argument_name, _) = self.expect_name()?;
                self.expect_punctuation(':')?;
                let value = self.parse_value()?;
                if arguments
                    .iter()
                    .any(|argument: &Argument| argument.name == argument_name)
                {
                    return self.error("duplicate field argument");
                }
                arguments.push(Argument {
                    name: argument_name,
                    value,
                });
            }
            self.expect_punctuation(')')?;
            arguments
        } else {
            Vec::new()
        };
        let selection = if self.at_punctuation('{') {
            self.parse_selection_set()?
        } else {
            Vec::new()
        };
        let end = self.tokens[self.cursor.saturating_sub(1)].span.end;
        Ok(Field {
            alias,
            name,
            arguments,
            selection,
            span: ByteSpan { start, end },
        })
    }

    fn parse_value(&mut self) -> Result<ValueNode, GraphQlAdapterError> {
        let token = self.current().clone();
        let (value, span) = match token.kind {
            TokenKind::Name(name) => {
                self.cursor += 1;
                let value = match name.as_str() {
                    "null" => ValueKind::Null,
                    "true" => ValueKind::Boolean(true),
                    "false" => ValueKind::Boolean(false),
                    _ => ValueKind::Enum(name),
                };
                (value, token.span)
            }
            TokenKind::String(value) => {
                self.cursor += 1;
                (ValueKind::String(value), token.span)
            }
            TokenKind::Integer(value) => {
                self.cursor += 1;
                (ValueKind::Integer(value), token.span)
            }
            TokenKind::Float(value) => {
                self.cursor += 1;
                (ValueKind::Float(value), token.span)
            }
            TokenKind::Punctuation('[') => {
                self.cursor += 1;
                let mut values = Vec::new();
                while !self.at_punctuation(']') {
                    if matches!(self.current().kind, TokenKind::End) {
                        return self.error("unterminated list value");
                    }
                    values.push(self.parse_value()?);
                }
                let end = self.expect_punctuation(']')?.end;
                (
                    ValueKind::List(values),
                    ByteSpan {
                        start: token.span.start,
                        end,
                    },
                )
            }
            TokenKind::Punctuation('{') => {
                self.cursor += 1;
                let mut fields = Vec::new();
                while !self.at_punctuation('}') {
                    if matches!(self.current().kind, TokenKind::End) {
                        return self.error("unterminated object value");
                    }
                    let (name, _) = self.expect_name()?;
                    self.expect_punctuation(':')?;
                    let value = self.parse_value()?;
                    if fields.iter().any(|(field, _)| field == &name) {
                        return self.error("duplicate object field");
                    }
                    fields.push((name, value));
                }
                let end = self.expect_punctuation('}')?.end;
                (
                    ValueKind::Object(fields),
                    ByteSpan {
                        start: token.span.start,
                        end,
                    },
                )
            }
            _ => return self.error("expected a literal GraphQL value"),
        };
        Ok(ValueNode { value, span })
    }

    fn current(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    fn at_punctuation(&self, expected: char) -> bool {
        self.current().kind == TokenKind::Punctuation(expected)
    }

    fn take_punctuation(&mut self, expected: char) -> bool {
        if self.at_punctuation(expected) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn expect_punctuation(&mut self, expected: char) -> Result<ByteSpan, GraphQlAdapterError> {
        if !self.at_punctuation(expected) {
            return self.error(&format!("expected {expected:?}"));
        }
        let span = self.current().span;
        self.cursor += 1;
        Ok(span)
    }

    fn take_name(&mut self, expected: &str) -> bool {
        if matches!(&self.current().kind, TokenKind::Name(name) if name == expected) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn expect_name(&mut self) -> Result<(String, ByteSpan), GraphQlAdapterError> {
        let token = self.current().clone();
        let TokenKind::Name(name) = token.kind else {
            return self.error("expected a GraphQL name");
        };
        self.cursor += 1;
        Ok((name, token.span))
    }

    fn error<T>(&self, message: &str) -> Result<T, GraphQlAdapterError> {
        Err(GraphQlAdapterError::new(format!(
            "{message} at byte {}",
            self.current().span.start
        )))
    }
}
