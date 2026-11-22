//! Private complete header discovery; every body is parsed by later full passes.
use super::{Result, RustParser, Token, TokenKind};
use crate::translation::rust_declaration_header_rules as rules;
impl RustParser {
    pub(super) fn declaration_body_boundary(&mut self) -> Result<Token> {
        let mut count = 64usize;
        loop {
            let end = self
                .cursor
                .index
                .saturating_add(count)
                .min(self.cursor.tokens.len());
            let tokens = &self.cursor.tokens[self.cursor.index..end];
            let kinds = tokens
                .iter()
                .map(|token| {
                    match token.kind {
                        TokenKind::Punct => "punct",
                        TokenKind::Eof => "eof",
                        _ => "atom",
                    }
                    .to_owned()
                })
                .collect::<Vec<_>>();
            let values = tokens
                .iter()
                .map(|token| token.value.clone())
                .collect::<Vec<_>>();
            let boundary = rules::find_rust_declaration_body_boundary(&kinds, &values);
            if boundary >= 0.0 {
                let boundary = crate::translation::frontend_rules::ml_array::number_index(boundary)
                    .expect("actual complete token boundary");
                let close = tokens[boundary - 1].clone();
                self.cursor.index += boundary;
                return Ok(close);
            }
            if boundary == -1.0 || tokens.len() < count {
                return Err(Self::fail(
                    "expected a complete balanced function body",
                    self.cursor.peek(),
                ));
            }
            count = count
                .checked_mul(2)
                .expect("token window within the actual source allocation");
        }
    }
}

include!("../../../tests/internal/translation/rust/declaration_headers.rs");
