//! Attribute syntax, retaining the existing admitted derive and lint behavior.
use serde_json::{Value, json};

use super::super::{
    ACCEPTED_DERIVES, ACCEPTED_LINTS, Result, RustParser, Token, span, unsupported,
};

impl RustParser {
    pub(in super::super) fn inner_attributes(&mut self) -> Result<()> {
        if !self.const_function_model.is_empty() && self.cursor.is("#") && self.cursor.is_at("!", 1)
        {
            return Err(unsupported(
                "Rust const function context",
                "crate attributes require an independent compilation interpretation",
                None,
            ));
        }
        self.pure_unit_attribute_guard()?;
        self.empty_option_attribute_guard()?;
        while self.cursor.is("#") && self.cursor.is_at("!", 1) {
            let start = self.cursor.advance();
            self.cursor.advance();
            let _ = self.attribute_body(&start, None)?;
        }
        Ok(())
    }

    pub(in super::super) fn attributes(
        &mut self,
        mut diagnostic_attributes: Option<&mut Vec<Token>>,
    ) -> Result<Vec<Value>> {
        self.pure_unit_attribute_guard()?;
        let mut derivations = Vec::new();
        self.empty_option_attribute_guard()?;
        while self.cursor.is("#") && self.cursor.is_at("[", 1) {
            let start = self.cursor.advance();
            if let Some(derivation) =
                self.attribute_body(&start, diagnostic_attributes.as_deref_mut())?
            {
                derivations.push(derivation);
            }
        }
        Ok(derivations)
    }

    /// `#[derive(Clone, …)]` and lint attributes do not change behaviour.
    pub(in super::super) fn attribute_body(
        &mut self,
        start: &Token,
        diagnostic_attributes: Option<&mut Vec<Token>>,
    ) -> Result<Option<Value>> {
        if !self.const_function_model.is_empty()
            && let Some(attributes) = diagnostic_attributes
            && self.read_diagnostic_attribute()
        {
            attributes.push(start.clone());
            return Ok(None);
        }
        let c = &mut self.cursor;
        c.expect("[", Some("attribute"))?;
        let name = c.identifier(Some("attribute"))?.value;
        if !self.const_function_model.is_empty() && (name != "must_use" || !c.is("]")) {
            return Err(unsupported(
                "Rust const function attribute",
                "only the actual zero-argument must_use function attribute is admitted",
                None,
            ));
        }
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
        Ok((name == "derive").then(|| {
            json!({"name":"derive", "traits": traits, "span":{"start":start.start,"end":end.end}})
        }))
    }
}
