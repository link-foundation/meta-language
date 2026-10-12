// Literal token readers share the lexer namespace through include!.
// The shared generated decision owns decimal/exponent/suffix boundaries.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn rust_number(source: &Source, index: usize, units: &mut Option<Vec<f64>>) -> Result<Token> {
    let encoded =
        units.get_or_insert_with(|| source.units().iter().map(|unit| f64::from(*unit)).collect());
    let bounds = read_rust_number_bounds(encoded, index as f64);
    let end = bounds[0] as usize;
    let suffix = bounds[1] as usize;
    let floating = bounds[2] > 0.0;
    let raw = source.slice(index, end);
    let digits = source.slice(index, suffix);
    if !floating && digits.ends_with('_') {
        return Err(TranslationError::syntax(
            format!("malformed number {raw}"),
            Some(span(index, end)),
        ));
    }
    let numeric = digits.replace('_', "");
    let value = if floating {
        let number = numeric.parse::<f64>().map_err(|_| {
            TranslationError::syntax(format!("malformed number {raw}"), Some(span(index, end)))
        })?;
        if !number.is_finite() {
            return Err(TranslationError::syntax(
                format!("floating-point literal out of range {raw}"),
                Some(span(index, end)),
            ));
        }
        super::js_number::js_number(number)
    } else {
        numeric
    };
    let mut token = Token::new(TokenKind::Number, value, raw, index, end);
    token.suffix = source.slice(suffix, end);
    token.floating = floating;
    Ok(token)
}

fn javascript_number_regex() -> &'static Regex {
    static NUMBER: OnceLock<Regex> = OnceLock::new();
    NUMBER.get_or_init(|| {
        let run = |digit: &str| format!("{digit}(?:_?{digit})*");
        let decimal = run("[0-9]");
        Regex::new(&format!(
            r"^(?:0[xX]{}|0[oO]{}|0[bB]{}|(?:(?:0|[1-9](?:_?{decimal})?)(?:\.(?:{decimal})?)?|\.{decimal})(?:[eE][+-]?{decimal})?)n?",
            run("[0-9a-fA-F]"),
            run("[0-7]"),
            run("[01]"),
        ))
        .expect("JavaScript numeric literal")
    })
}

/// An ECMAScript numeric literal. A `BigInt` (`5n`, `0x1fn`) keeps its decimal
/// digits as `value` with suffix `n`; a Number keeps `String(Number(raw))`, the
/// value JavaScript prints, with an empty suffix.
fn javascript_number(source: &Source, index: usize) -> Result<Token> {
    let rest = source.slice(index, source.len());
    let found = javascript_number_regex().find(&rest);
    let raw = found.map_or_else(|| source.slice(index, index + 1), |m| m.as_str().to_owned());
    let end = index + raw.len();
    let next = source.char_at(end);
    let malformed = || {
        let stop = if end < source.len() { end + 1 } else { end };
        TranslationError::syntax(
            format!("malformed number {}", source.slice(index, stop)),
            Some(span(index, stop)),
        )
    };
    // `08`, `1_`, `3in` and `1.5n` are SyntaxErrors in JavaScript.
    if found.is_none()
        || next.is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '$')
    {
        return Err(malformed());
    }
    let text = raw.replace('_', "");
    let token = if let Some(digits) = text.strip_suffix('n') {
        let hex = digits.starts_with("0x") || digits.starts_with("0X");
        if !hex && digits.contains(['.', 'e', 'E']) {
            return Err(malformed());
        }
        let value = Decimal::parse(digits).ok_or_else(malformed)?;
        let mut token = Token::new(TokenKind::Number, value.digits, raw, index, end);
        token.suffix = String::from("n");
        token
    } else {
        let value = js_number_literal(&text).ok_or_else(malformed)?;
        Token::new(TokenKind::Number, value, raw, index, end)
    };
    Ok(token)
}

fn block_comment_end(source: &Source, index: usize, comments: &CommentSyntax) -> Result<usize> {
    let open = comments.open.encode_utf16().count();
    let close = comments.close.encode_utf16().count();
    let mut depth = 0usize;
    let mut cursor = index;
    while cursor < source.len() {
        if source.starts_with(comments.open, cursor) {
            depth += 1;
            cursor += open;
            if !comments.nested && depth > 1 {
                depth = 1;
            }
            continue;
        }
        if source.starts_with(comments.close, cursor) {
            // A close before any open (impossible from `tokenize`) would go negative in JavaScript.
            depth = depth.saturating_sub(1);
            cursor += close;
            if depth == 0 {
                return Ok(cursor);
            }
            continue;
        }
        cursor += 1;
    }
    Err(TranslationError::syntax(
        "unterminated block comment",
        Some(span(index, source.len())),
    ))
}

fn read_unicode_escape(
    source: &Source,
    cursor: usize,
    language: Language,
) -> Result<(char, usize)> {
    let units = source.units()[cursor..source.len().min(cursor + 12)]
        .iter()
        .map(|unit| f64::from(*unit))
        .collect();
    match decode_unicode_escape(units, language == Language::JavaScript) {
        UnicodeEscape::Scalar(code, end) => {
            // The generated decoder returns integral scalar values and
            // offsets inside the twelve-code-unit input window.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let character = char::from_u32(code as u32).expect("decoded Unicode scalar");
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            Ok((character, end as usize))
        }
        UnicodeEscape::Malformed => Err(TranslationError::syntax(
            "malformed unicode escape",
            Some(span(cursor, cursor + 2)),
        )),
        UnicodeEscape::Unsupported(end, escape_length) => {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let (end, escape_length) = (end as usize, escape_length as usize);
            Err(TranslationError::new(
                ErrorKind::Unsupported,
                format!(
                    "unicode escape {} is not a scalar value; Rust strings hold scalar values only",
                    source.slice(cursor, cursor + escape_length)
                ),
                Some(span(cursor, cursor + end)),
            ))
        }
    }
}

fn string_token(source: &Source, index: usize, language: Language) -> Result<Token> {
    let quote = source.units[index];
    let mut cursor = index + 1;
    let mut value: Vec<u16> = Vec::new();
    while cursor < source.len() {
        let unit = source.units[cursor];
        if unit == quote {
            if language == Language::Rocq && source.is_char(cursor + 1, '"') {
                value.push(u16::from(b'"'));
                cursor += 2;
                continue;
            }
            let mut token = Token::new(
                TokenKind::String,
                String::from_utf16_lossy(&value),
                source.slice(index, cursor + 1),
                index,
                cursor + 1,
            );
            token.raw = source.slice(index, cursor + 1);
            return Ok(token);
        }
        if unit == u16::from(b'\\') && language != Language::Rocq {
            let escaped = source.char_at(cursor + 1);
            if escaped == Some('u') || (language == Language::JavaScript && escaped == Some('x')) {
                let (character, end) = read_unicode_escape(source, cursor, language)?;
                let mut encoded = [0; 2];
                value.extend_from_slice(character.encode_utf16(&mut encoded));
                cursor += end;
                continue;
            }
            if language == Language::JavaScript {
                if escaped.is_some_and(|character| matches!(character, '1'..='9'))
                    || (escaped == Some('0')
                        && source
                            .char_at(cursor + 2)
                            .is_some_and(|character| character.is_ascii_digit()))
                {
                    return Err(TranslationError::syntax(
                        "legacy octal string escape",
                        Some(span(cursor, cursor + 2)),
                    ));
                }
                if matches!(escaped, Some('\n' | '\r' | '\u{2028}' | '\u{2029}')) {
                    cursor += if escaped == Some('\r') && source.is_char(cursor + 2, '\n') {
                        3
                    } else {
                        2
                    };
                    continue;
                }
            }
            let replacement = match escaped {
                Some('n') => "\n",
                Some('t') => "\t",
                Some('r') => "\r",
                Some('\\') => "\\",
                Some('"') => "\"",
                Some('\'') => "'",
                Some('0') => "\0",
                Some('{') if language == Language::JavaScript => "{",
                Some('{') => "\\{",
                Some('b') if language == Language::JavaScript => "\u{0008}",
                Some('f') if language == Language::JavaScript => "\u{000c}",
                Some('v') if language == Language::JavaScript => "\u{000b}",
                Some(_) if language == Language::JavaScript => {
                    value.push(source.units[cursor + 1]);
                    cursor += 2;
                    continue;
                }
                _ => {
                    return Err(TranslationError::syntax(
                        format!(
                            "unsupported string escape \\{}",
                            escaped_text(source, cursor + 1)
                        ),
                        Some(span(cursor, cursor + 2)),
                    ));
                }
            };
            value.extend(replacement.encode_utf16());
            cursor += 2;
            continue;
        }
        if (unit == u16::from(b'\n') || unit == u16::from(b'\r'))
            && language == Language::JavaScript
        {
            break;
        }
        value.push(unit);
        cursor += 1;
    }
    Err(TranslationError::syntax(
        "unterminated string literal",
        Some(span(index, source.len())),
    ))
}

fn template_token(source: &Source, index: usize, units: &mut Option<Vec<f64>>) -> Result<Token> {
    let mut parts = Vec::new();
    let mut cursor = index + 1;
    let mut text: Vec<u16> = Vec::new();
    while cursor < source.len() {
        let unit = source.units[cursor];
        if unit == u16::from(b'`') {
            parts.push(TemplatePart {
                text: String::from_utf16_lossy(&text),
                expression: None,
            });
            let raw = source.slice(index, cursor + 1);
            let mut token = Token::new(TokenKind::Template, String::new(), raw, index, cursor + 1);
            token.parts = parts;
            return Ok(token);
        }
        if unit == u16::from(b'\\') {
            if matches!(source.char_at(cursor + 1), Some('u' | 'x')) {
                let (character, end) = read_unicode_escape(source, cursor, Language::JavaScript)?;
                let mut encoded = [0; 2];
                text.extend_from_slice(character.encode_utf16(&mut encoded));
                cursor += end;
                continue;
            }
            if source
                .char_at(cursor + 1)
                .is_some_and(|character| matches!(character, '1'..='9'))
                || (source.char_at(cursor + 1) == Some('0')
                    && source
                        .char_at(cursor + 2)
                        .is_some_and(|character| character.is_ascii_digit()))
            {
                return Err(TranslationError::syntax(
                    "legacy octal template escape",
                    Some(span(cursor, cursor + 3)),
                ));
            }
            if matches!(
                source.char_at(cursor + 1),
                Some('\n' | '\r' | '\u{2028}' | '\u{2029}')
            ) {
                cursor += if source.is_char(cursor + 1, '\r') && source.is_char(cursor + 2, '\n') {
                    3
                } else {
                    2
                };
                continue;
            }
            let replacement = match source.char_at(cursor + 1) {
                Some('n') => '\n',
                Some('t') => '\t',
                Some('r') => '\r',
                Some('b') => '\u{0008}',
                Some('f') => '\u{000c}',
                Some('v') => '\u{000b}',
                Some('0') => '\0',
                Some('\\') => '\\',
                Some('`') => '`',
                Some('$') => '$',
                Some(character) => character,
                None => {
                    return Err(TranslationError::syntax(
                        format!(
                            "unsupported template escape \\{}",
                            escaped_text(source, cursor + 1)
                        ),
                        Some(span(cursor, cursor + 2)),
                    ));
                }
            };
            text.push(replacement as u16);
            cursor += 2;
            continue;
        }
        if unit == u16::from(b'\r') {
            text.push(u16::from(b'\n'));
            cursor += if source.is_char(cursor + 1, '\n') {
                2
            } else {
                1
            };
            continue;
        }
        if source.starts_with("${", cursor) {
            let close = interpolation_boundary(source, cursor + 1, units)?;
            parts.push(TemplatePart {
                text: String::from_utf16_lossy(&text),
                expression: Some(TemplateExpression {
                    source: source.slice(cursor + 2, close),
                    offset: cursor + 2,
                }),
            });
            text.clear();
            cursor = close + 1;
            continue;
        }
        text.push(unit);
        cursor += 1;
    }
    Err(TranslationError::syntax(
        "unterminated template literal",
        Some(span(index, source.len())),
    ))
}

/// The escaped unit as JavaScript interpolates it (`undefined` past the end).
fn escaped_text(source: &Source, index: usize) -> String {
    if index < source.len() {
        source.slice(index, index + 1)
    } else {
        "undefined".to_owned()
    }
}

/// The index of the `}` closing the `{` at `open`.
///
/// # Errors
///
/// Returns a syntax error when the braces are unbalanced.
pub fn matching_brace(source: &Source, open: usize) -> Result<usize> {
    interpolation_boundary(source, open, &mut None)
}

fn interpolation_boundary(
    source: &Source,
    open: usize,
    units: &mut Option<Vec<f64>>,
) -> Result<usize> {
    let tokens = tokenize_region(source, Language::JavaScript, open + 1, true, units)?;
    let close = tokens.tokens.last().expect("end token").end;
    if close < source.len() {
        return Ok(close);
    }
    Err(TranslationError::syntax(
        "unbalanced braces",
        Some(span(open, source.len())),
    ))
}

/// Cursor over a token list with the helpers every frontend needs.
#[derive(Clone, Debug)]
pub struct TokenCursor {
    pub tokens: Vec<Token>,
    pub language: Language,
    pub index: usize,
}

impl TokenCursor {
    #[must_use]
    pub const fn new(tokens: Vec<Token>, language: Language) -> Self {
        Self {
            tokens,
            language,
            index: 0,
        }
    }

    #[must_use]
    pub fn peek(&self) -> &Token {
        self.peek_at(0)
    }

    #[must_use]
    pub fn peek_at(&self, offset: usize) -> &Token {
        &self.tokens[(self.index + offset).min(self.tokens.len() - 1)]
    }

    pub fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if token.kind != TokenKind::Eof {
            self.index += 1;
        }
        token
    }

    #[must_use]
    pub fn is(&self, value: &str) -> bool {
        self.is_at(value, 0)
    }

    #[must_use]
    pub fn is_at(&self, value: &str, offset: usize) -> bool {
        let token = self.peek_at(offset);
        matches!(token.kind, TokenKind::Punct | TokenKind::Identifier) && token.value == value
    }

    #[must_use]
    pub fn is_kind(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    #[must_use]
    pub fn is_kind_at(&self, kind: TokenKind, offset: usize) -> bool {
        self.peek_at(offset).kind == kind
    }

    pub fn eat(&mut self, value: &str) -> Option<Token> {
        if self.is(value) {
            Some(self.advance())
        } else {
            None
        }
    }

    /// Consumes `value`.
    ///
    /// # Errors
    ///
    /// Returns a syntax error naming the token found instead.
    pub fn expect(&mut self, value: &str, context: Option<&str>) -> Result<Token> {
        if self.is(value) {
            return Ok(self.advance());
        }
        let token = self.peek();
        Err(TranslationError::syntax(
            format!(
                "expected {value}{} but found {}",
                context.map_or_else(String::new, |context| format!(" in {context}")),
                describe(token)
            ),
            Some(token.span()),
        ))
    }

    /// Consumes an identifier.
    ///
    /// # Errors
    ///
    /// Returns a syntax error naming the token found instead.
    pub fn identifier(&mut self, context: Option<&str>) -> Result<Token> {
        let token = self.peek();
        if token.kind != TokenKind::Identifier {
            return Err(TranslationError::syntax(
                format!(
                    "expected identifier{} but found {}",
                    context.map_or_else(String::new, |context| format!(" in {context}")),
                    describe(token)
                ),
                Some(token.span()),
            ));
        }
        Ok(self.advance())
    }

    #[must_use]
    pub fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }
}
