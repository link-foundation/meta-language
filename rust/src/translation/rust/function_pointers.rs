//! Actual source pointer signatures and free declaration references.
use super::{
    Result, RustParser, SExpr, SNode, SourceDeclaration, SourceOrigin, Span, TokenKind, Type, UNIT,
    unsupported,
};
use crate::translation::rust_function_pointer_rules as rules;
impl RustParser {
    pub(super) fn read_function_pointer_type(&mut self) -> Result<Type> {
        let start = self.cursor.advance();
        self.cursor.expect("(", Some("function pointer"))?;
        let mut params = Vec::new();
        let mut copies = Vec::new();
        let mut parameter_origins = Vec::new();
        while !self.cursor.is(")") {
            if self.cursor.is_kind(TokenKind::Identifier) && self.cursor.is_at(":", 1) {
                self.cursor.advance();
                self.cursor.advance();
            }
            params.push(self.parse_type()?);
            parameter_origins.push(self.origins.current_type.clone());
            copies.push(self.origins.current_type.direct_copy_value());
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some("function pointer"))?;
        let ret = if self.cursor.eat("->").is_some() {
            self.parse_type()?
        } else {
            self.origins
                .typed(UNIT, &self.module_path.clone(), Vec::new())
        };
        let reason = rules::read_rust_function_pointer_type_error(
            self.ordinary_completion_context(),
            &copies,
            self.origins.current_type.direct_copy_value(),
        );
        if !reason.is_empty() {
            return Err(unsupported(
                "Rust function pointer",
                reason,
                Some(start.span()),
            ));
        }
        let result_origin = self.origins.current_type.clone();
        let ty = Type::Function {
            params,
            ret: Box::new(ret),
        };
        self.origins
            .typed(ty.clone(), &self.module_path.clone(), Vec::new());
        self.origins.current_type.copy_evidence = Some(());
        self.origins.current_type.callable_parameters = Some(parameter_origins);
        self.origins.current_type.callable_result = Some(Box::new(result_origin));
        Ok(ty)
    }

    pub(super) fn read_function_reference(
        &mut self,
        segments: &[String],
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if segments.len() == 1
            && let Some(origin) = self.origins.scope.get(&segments[0]).cloned()
        {
            if !matches!(origin.ty, Some(Type::Function { .. })) {
                return Ok(None);
            }
            let node = SExpr::new(
                SNode::Name {
                    path: segments.to_vec(),
                },
                span,
            );
            self.origins.mark(&node, origin);
            return Ok(Some(node));
        }
        let Some(entry) = self
            .origins
            .lookup(&self.source_value_path(segments, path), path)
        else {
            return Ok(None);
        };
        if let SourceDeclaration::Constant { result } = &entry.declaration {
            return Ok(Some(self.source_constant_value(
                &entry.full_name,
                result,
                path,
                span,
            )));
        }
        let SourceDeclaration::Function { parameters, result } = entry.declaration else {
            return Ok(None);
        };
        let copies = parameters
            .iter()
            .map(SourceOrigin::direct_copy_value)
            .collect::<Vec<_>>();
        let reason = rules::read_rust_function_pointer_type_error(
            self.ordinary_completion_context(),
            &copies,
            result.direct_copy_value(),
        );
        if !reason.is_empty() {
            return Err(unsupported("Rust function reference", reason, span));
        }
        let ty = Type::Function {
            params: parameters
                .iter()
                .map(|origin| origin.ty.clone().expect("actual pointer parameter type"))
                .collect(),
            ret: Box::new(result.ty.clone().expect("actual pointer result type")),
        };
        let mut full = vec!["crate".to_owned()];
        full.extend(entry.full_name.split('.').map(str::to_owned));
        let node = SExpr::new(SNode::FunctionValue { path: full }, span);
        let mut origin = SourceOrigin::with_type(ty, path, Vec::new());
        origin.copy_evidence = Some(());
        origin.callable_parameters = Some(parameters);
        origin.callable_result = Some(Box::new(result));
        self.origins.mark(&node, origin);
        Ok(Some(node))
    }

    pub(super) fn refuse_callable_observation(
        &self,
        value: &SExpr,
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        let origin = self.origins.origin(value, path);
        let reason = rules::read_rust_function_pointer_observation_error(
            origin.ty.as_ref().map_or("", Type::kind),
        );
        if reason.is_empty() {
            Ok(())
        } else {
            Err(unsupported("Rust callable observation", reason, span))
        }
    }
}
