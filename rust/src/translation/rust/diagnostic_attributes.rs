//! Actual token and source ownership for the generated diagnostic attribute plan.

use super::{Result, RustParser, SFn, Token, type_error};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::source_diagnostic_attributes::{create_facet, token_kind};
use serde_json::Value;

pub(super) struct DiagnosticAttributeColumns {
    kinds: Vec<String>,
    values: Vec<String>,
}

impl DiagnosticAttributeColumns {
    fn new(tokens: &[Token]) -> Self {
        Self {
            kinds: tokens
                .iter()
                .map(|token| token_kind(token).to_owned())
                .collect(),
            values: tokens.iter().map(|token| token.value.clone()).collect(),
        }
    }
}

impl RustParser {
    pub(super) fn read_diagnostic_attribute(&mut self) -> bool {
        let Some(start) = self.cursor.index.checked_sub(1) else {
            return false;
        };
        let tokens = &self.cursor.tokens;
        let columns = self
            .diagnostic_attribute_columns
            .get_or_insert_with(|| DiagnosticAttributeColumns::new(tokens));
        let position = serde_json::Number::from(start)
            .as_f64()
            .expect("a native token position has a finite binary64 representation");
        let plan =
            crate::translation::rust_diagnostic_attribute_rules::plan_rust_diagnostic_attribute(
                &columns.kinds,
                &columns.values,
                position,
            );
        let Some(end) = plan.first().copied().and_then(number_index) else {
            return false;
        };
        self.cursor.index = end;
        true
    }

    pub(super) fn create_diagnostic_attribute_facet(
        &self,
        first_attribute: usize,
        const_keyword: usize,
        function: &SFn,
        module_path: &[String],
    ) -> Result<Value> {
        let source = self.original_source.as_deref().ok_or_else(|| type_error(
            "Rust diagnostic source attributes: actual complete source and token positions are required",
            None,
        ))?;
        create_facet(
            source,
            &self.cursor.tokens,
            first_attribute,
            const_keyword,
            function,
            module_path,
        )
    }
}
