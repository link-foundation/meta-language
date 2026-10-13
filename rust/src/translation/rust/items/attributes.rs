//! Attribute syntax, retaining the existing admitted derive and lint behavior.
use serde_json::{Value, json};

use super::super::{
    ACCEPTED_DERIVES, ACCEPTED_LINTS, Result, RustParser, Token, span, unsupported,
};

impl RustParser {
    pub(in super::super) fn inner_attributes(&mut self) -> Result<()> {
        self.empty_option_attribute_guard()?;
        while self.cursor.is("#") && self.cursor.is_at("!", 1) {
            let start = self.cursor.advance();
            self.cursor.advance();
            let _ = self.attribute_body(&start)?;
        }
        Ok(())
    }

    pub(in super::super) fn attributes(&mut self) -> Result<Vec<Value>> {
        let mut derivations = Vec::new();
        self.empty_option_attribute_guard()?;
        while self.cursor.is("#") && self.cursor.is_at("[", 1) {
            let start = self.cursor.advance();
            if let Some(derivation) = self.attribute_body(&start)? {
                derivations.push(derivation);
            }
        }
        Ok(derivations)
    }

    /// `#[derive(Clone, …)]` and lint attributes do not change behaviour.
    pub(in super::super) fn attribute_body(&mut self, start: &Token) -> Result<Option<Value>> {
        let c = &mut self.cursor;
        c.expect("[", Some("attribute"))?;
        let name = c.identifier(Some("attribute"))?.value;
        let mut traits = Vec::new();
        if name == "derive" {
            c.expect("(", Some("derive"))?;
            while !c.is(")") {
                let derived = c.identifier(Some("derive"))?;
                if !ACCEPTED_DERIVES.contains(&derived.value.as_str()) {
                    return Err(unsupported(
                        &format!("derive({})", derived.value),
                        "only derives that add no behaviour are portable",
                        span(&derived, &derived),
                    ));
                }
                traits.push(derived.value);
                if c.eat(",").is_none() {
                    break;
                }
            }
            c.expect(")", Some("derive"))?;
        } else if ACCEPTED_LINTS.contains(&name.as_str()) {
            let mut depth = 0_i64;
            while !c.at_end() && (depth != 0 || !c.is("]")) {
                if c.is("(") {
                    depth += 1;
                }
                if c.is(")") {
                    depth -= 1;
                }
                c.advance();
            }
        } else {
            return Err(unsupported(
                &format!("#[{name}]"),
                "attributes that change compilation are outside the portable core",
                span(start, c.peek()),
            ));
        }
        let end = c.expect("]", Some("attribute"))?;
        Ok((name == "derive")
            .then(|| json!({"name":"derive", "traits": traits, "span": span(start, &end)})))
    }
}
