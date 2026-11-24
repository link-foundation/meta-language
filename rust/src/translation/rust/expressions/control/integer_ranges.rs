//! Actual range tokens into the shared source-form admission.
use super::{Result, RustParser, SPattern, SPatternNode, Span, Token, TokenKind, unsupported};
use crate::translation::rust_integer_range_rules as rules;
impl RustParser {
    pub(super) fn integer_range_pattern(
        &mut self,
        path: &[String],
        first: &Token,
        lower: Option<SPattern>,
    ) -> Result<SPattern> {
        if !self.cursor.is("..") && !self.cursor.is("..=") {
            return Ok(lower.expect("a non-range token follows its actual lower literal"));
        }
        let operation = self.cursor.advance();
        let upper = if self.cursor.is("-") && self.cursor.is_kind_at(TokenKind::Number, 1) {
            let sign = self.cursor.advance();
            let value = self.cursor.advance();
            Some(self.numeric_pattern(&value, &sign, true)?)
        } else if self.cursor.is_kind(TokenKind::Number) {
            let value = self.cursor.advance();
            Some(self.numeric_pattern(&value, &value, false)?)
        } else if self.cursor.is_kind(TokenKind::ByteCharacter) {
            let value = self.cursor.advance();
            Some(SPattern {
                node: SPatternNode::ByteLit {
                    value: value.value.clone(),
                },
                span: Some(value.span()),
            })
        } else {
            match self.primitive_constant_pattern(path)? {
                Some(value) => Some(value),
                None => self.read_source_constant_pattern(path)?,
            }
        };
        let span = Some(Span {
            start: first.start,
            end: upper
                .as_ref()
                .and_then(|value| value.span)
                .map_or(operation.end, |span| span.end),
        });
        let error = rules::read_rust_integer_range_source_error(
            self.ordinary_completion_context(),
            &operation.value,
            lower.is_some(),
            upper.is_some(),
        );
        if !error.is_empty() {
            return Err(unsupported("range pattern", &error, span));
        }
        Ok(SPattern {
            node: SPatternNode::IntegerRange {
                lower: lower.map(Box::new),
                upper: upper.map(Box::new),
                inclusive: operation.value == "..=",
            },
            span,
        })
    }
}
