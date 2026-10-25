//! Actual constant declaration owners into the shared checked value construction.
use super::{
    Result, RustParser, SExpr, SNode, SourceDeclaration, SourceOrigin, Span, Type, type_error,
    unsupported,
};
use crate::translation::rust_source_constant_rules as rules;
impl RustParser {
    pub(super) fn source_value_path(&self, names: &[String], path: &[String]) -> Vec<String> {
        if names.len() == 1 && self.origins.scope.contains_key(&names[0]) {
            return names.to_vec();
        }
        let resolved = self.resolve(names, path);
        if self.ordinary_completion_context() {
            rules::read_rust_source_value_path(&self.module_path, &resolved)
        } else {
            resolved
        }
    }

    pub(super) fn require_nonconstant_binding(
        &self,
        name: &str,
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        if self
            .origins
            .lookup(
                &rules::read_rust_source_value_path(
                    &self.module_path,
                    &self.resolve(&[name.to_owned()], path),
                ),
                path,
            )
            .is_some_and(|entry| matches!(entry.declaration, SourceDeclaration::Constant { .. }))
        {
            return Err(unsupported(
                "Rust constant pattern",
                rules::read_rust_source_constant_pattern_error("const"),
                span,
            ));
        }
        Ok(())
    }

    pub(super) fn source_constant_value(
        &mut self,
        owner: &str,
        result: &SourceOrigin,
        path: &[String],
        span: Option<Span>,
    ) -> SExpr {
        let temporary = self.fresh_product_binding();
        let plan = rules::plan_rust_source_constant_value("const", owner, &temporary);
        let mut name = vec!["crate".to_owned()];
        name.extend(plan[3].split('.').map(str::to_owned));
        let value = SExpr::new(SNode::Name { path: name }, span);
        let body = SExpr::new(
            SNode::Name {
                path: vec![plan[5].clone()],
            },
            span,
        );
        let node = SExpr::new(
            SNode::Let {
                name: plan[1].clone(),
                ty: None,
                value: Box::new(value),
                body: Box::new(body),
            },
            span,
        );
        self.origins.mark(&node, result.clone());
        let _ = path; // The owner is already the complete source-qualified path.
        node
    }
    pub(super) fn source_constant_call(
        &mut self,
        names: &[String],
        args: &[SExpr],
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if names.len() == 1 && self.origins.scope.contains_key(&names[0]) {
            return Ok(None);
        }
        let Some(entry) = self
            .origins
            .lookup(&self.source_value_path(names, path), path)
        else {
            return Ok(None);
        };
        let SourceDeclaration::Constant { result } = &entry.declaration else {
            return Ok(None);
        };
        let error = rules::read_rust_source_callable_value_error(
            result.ty.as_ref().map_or("", Type::kind),
            "constant",
        );
        if !error.is_empty() {
            return Err(type_error(error, span));
        }
        let value = self.source_constant_value(&entry.full_name, result, path, span);
        self.read_copy_closure_call(&value, args, path, span)
    }
}
