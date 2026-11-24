use super::cast_syntax_rules::{accept_rust_cast_following_token, read_rust_cast_syntax_message};

impl RustParser {
    /// `e as T` converts between integer types when every value fits.
    pub(super) fn cast(&mut self, path: &[String], options: Options) -> Result<SExpr> {
        let mut value = self.unary(path, options)?;
        while self.cursor.is("as") {
            let token = self.cursor.advance();
            let parenthesized_type = self.cursor.is("(");
            let to = self.parse_type()?;
            if !self.origins.collecting && !self.origins.current_type.wrappers.is_empty() {
                return Err(unsupported(
                    "source cast",
                    "numeric casts require a direct source and target value type",
                    span(&token, self.cursor.peek()),
                ));
            }
            self.require_cast_following_token(parenthesized_type)?;
            let range = joined(value.span, span(&token, self.cursor.peek()), &token);
            value = SExpr::new(
                SNode::Cast {
                    arg: Box::new(value),
                    to,
                    from: None,
                    flavor: Flavor::Exact,
                },
                range,
            );
        }
        Ok(value)
    }

    fn require_cast_following_token(&self, parenthesized_type: bool) -> Result<()> {
        let token = self.cursor.peek();
        if accept_rust_cast_following_token(&token.value, parenthesized_type) {
            Ok(())
        } else {
            Err(TranslationError::syntax(
                read_rust_cast_syntax_message(),
                Some(token.span()),
            ))
        }
    }
}
