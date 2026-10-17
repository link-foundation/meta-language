//! Existing constant-item adapter, separated to bound the item module.
use super::super::SFn;
use super::{Options, Result, RustParser, SourceDeclaration, Type, span, unsupported};

impl RustParser {
    /// `const N: T = e;` is a function without parameters.
    pub(in super::super) fn const_item(&mut self, path: &[String]) -> Result<SFn> {
        self.require_type_namespace_without_constant()?;
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("const"))?.value;
        self.cursor.expect(":", Some("const"))?;
        let ret = self.parse_type()?;
        let source_result = self.origins.current_type.clone();
        if matches!(ret, Type::Array { .. }) {
            return Err(unsupported(
                "Rust array constant",
                "vector allocation and slice constants require a separate constant evaluation model",
                span(&start, self.cursor.peek()),
            ));
        }
        self.origins.declaration(
            &name,
            path,
            SourceDeclaration::Function {
                parameters: Vec::new(),
                result: source_result.clone(),
            },
        )?;
        self.cursor.expect("=", Some("const"))?;
        let body = self.expr(path, Options::default())?;
        self.origins
            .argument(&self.origins.origin(&body, path), &source_result, body.span)?;
        self.cursor.expect(";", Some("const"))?;
        Ok(SFn {
            source_diagnostic_attributes: None,
            semantic_ret: None,
            name,
            params: Vec::new(),
            ret: Some(ret),
            body,
            span: span(&start, self.cursor.peek()),
            generated: false,
            source_signature: None,
            source_primitive_owners: None,
            source_empty_option: None,
            source_type_namespace: None,
            source_local_state: None,
            source_const_function: None,
            source_pure_unit: None,
            source_loop: None,
        })
    }
}
