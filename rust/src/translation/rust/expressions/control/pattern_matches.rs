//! Actual source tokens into the shared macro plan and existing match parser.
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::rust::{Result, RustParser, SExpr, Span, Token, TokenKind, unsupported};
use crate::translation::rust_pattern_match_rules as rules;
use crate::translation::source_diagnostic_attributes::token_kind;
impl RustParser {
    pub(in crate::translation::rust) fn pattern_match(
        &mut self,
        token: &Token,
        path: &[String],
        receiver: SExpr,
        close: &str,
    ) -> Result<SExpr> {
        let owner = self
            .resolve(std::slice::from_ref(&token.value), path)
            .join("::");
        let range = Some(Span::new(token.start, self.cursor.peek().end));
        if !rules::accept_rust_pattern_match_macro_owner(&owner, self.ordinary_completion_context())
        {
            return Err(unsupported(
                "Rust pattern matches!",
                "the actual standard macro needs its ordinary source interpretation",
                range,
            ));
        }
        let kinds = self
            .cursor
            .tokens
            .iter()
            .map(|value| token_kind(value).to_owned())
            .collect::<Vec<_>>();
        let values = self
            .cursor
            .tokens
            .iter()
            .map(|value| value.value.clone())
            .collect::<Vec<_>>();
        let start = serde_json::Number::from(self.cursor.index)
            .as_f64()
            .expect("an actual source token position is finite");
        let plan = rules::plan_rust_pattern_match_rows(&kinds, &values, start, close);
        let Some(end) = plan.first().copied().and_then(number_index) else {
            return Err(unsupported(
                "Rust pattern matches!",
                "a complete delimited source pattern and optional guard are required",
                range,
            ));
        };
        let anchor = self.cursor.tokens[end - 1].clone();
        let mut generated = Vec::new();
        for code in plan.into_iter().skip(1) {
            if code >= 0.0 {
                let index = number_index(code).expect("actual source pattern token position");
                generated.push(self.cursor.tokens[index].clone());
            } else {
                let mut value = anchor.clone();
                value.value = rules::read_rust_pattern_match_token_value(code);
                value.raw = value.value.clone();
                value.kind = match rules::read_rust_pattern_match_token_kind(code).as_str() {
                    "identifier" => TokenKind::Identifier,
                    "punct" => TokenKind::Punct,
                    _ => unreachable!("shared expansion token kind"),
                };
                value.string_units = None;
                value.suffix.clear();
                value.floating = false;
                value.parts.clear();
                generated.push(value);
            }
        }
        let range = Some(Span::new(token.start, anchor.end));
        let mut end_token = anchor;
        end_token.kind = TokenKind::Eof;
        end_token.value.clear();
        end_token.raw.clear();
        end_token.start = end_token.end;
        end_token.string_units = None;
        end_token.suffix.clear();
        end_token.floating = false;
        end_token.parts.clear();
        generated.push(end_token);
        let original = std::mem::replace(&mut self.cursor.tokens, generated);
        self.cursor.index = 0;
        let result = self
            .match_from_scrutinee(path, token, receiver)
            .map(|mut value| {
                let origin = self.origins.origin(&value, path);
                value.span = range;
                self.origins.mark(&value, origin);
                value
            });
        self.cursor.tokens = original;
        self.cursor.index = end;
        result
    }
}
