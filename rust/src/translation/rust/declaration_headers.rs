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

#[cfg(test)]
mod tests {
    use super::rules;
    #[test]
    fn complete_groups_retain_atomic_tokens_and_refuse_invalid_boundaries() {
        let data = crate::read_structured_data_lino(include_str!(
            "../../data/rust-declaration-header-controls.lino"
        ))
        .expect("authored header controls");
        for row in data["parenthesisCases"]
            .as_array()
            .expect("parenthesis controls")
        {
            let syntax = row["syntax"]
                .as_array()
                .expect("actual syntax roles")
                .iter()
                .map(|value| value.as_str().expect("actual role").to_owned())
                .collect::<Vec<_>>();
            assert_eq!(crate::translation::rust_contextual_closure_rules::find_rust_contextual_closure_parenthesis_count(&syntax), row["depth"].as_f64().expect("independent prefix depth"));
        }
        for row in data["headerCases"]
            .as_array()
            .expect("complete header cases")
        {
            let kinds = row["kinds"]
                .as_array()
                .expect("actual token kinds")
                .iter()
                .map(|value| value.as_str().expect("kind").to_owned())
                .collect::<Vec<_>>();
            let values = row["values"]
                .as_array()
                .expect("actual token values")
                .iter()
                .map(|value| value.as_str().expect("value").to_owned())
                .collect::<Vec<_>>();
            assert_eq!(
                rules::find_rust_declaration_body_boundary(&kinds, &values),
                row["end"].as_f64().expect("independent boundary")
            );
        }
    }
}
