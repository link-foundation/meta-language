//! Actual source pointer signatures and free declaration references.
use super::{
    Result, RustParser, SExpr, SNode, SourceDeclaration, SourceOrigin, Span, TokenKind, Type, UNIT,
    unsupported,
};
use crate::translation::rust_function_pointer_rules as rules;
pub(super) fn clone_callable_type_origin(origin:&SourceOrigin)->SourceOrigin{super::source_type_origins::clone_source_type_origin(origin)}
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
            parameter_origins.push(clone_callable_type_origin(&self.origins.current_type));
            copies.push(self.origins.current_type.direct_copy_value());
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some("function pointer"))?;
        let ret = if self.cursor.eat("->").is_some() {
            self.parse_type()?
        } else {
            let result=self.origins.typed(UNIT, &self.module_path.clone(), Vec::new());
            self.origins.current_type.source_owner=Some(crate::translation::source_signatures::SourceOwner {kind:"unit".into(),path:Vec::new(),arguments:Vec::new()});
            result
        };
        let safe=rules::accept_rust_function_pointer_safe_source(&start.value,&start.raw);
        let roles=if safe{parameter_origins.iter().map(|origin|self.origins.standard_sum_ownership_facts(origin).profile).collect::<Vec<_>>()}else{Vec::new()};
        let result_role=if safe{self.origins.standard_sum_ownership_facts(&self.origins.current_type).profile}else{String::new()};
        let actual=rules::read_rust_function_pointer_domain_error(self.ordinary_completion_context(),&copies,self.origins.current_type.direct_copy_value(),&roles,&result_role,self.source_version_minor);
        let reason=rules::read_rust_function_pointer_header_domain_error(self.ordinary_completion_context(),&copies,self.origins.current_type.direct_copy_value(),&roles,&result_role,self.source_version_minor,self.header_discovery);
        if !reason.is_empty() {
            return Err(unsupported(
                "Rust function pointer",
                &reason,
                Some(start.span()),
            ));
        }
        let result_origin = clone_callable_type_origin(&self.origins.current_type);
        let ty = Type::Function {
            params,
            ret: Box::new(ret),
        };
        self.origins
            .typed(ty.clone(), &self.module_path.clone(), Vec::new());
        self.origins.current_type.copy_evidence = actual.is_empty().then_some(());
        self.origins.current_type.proven=actual.is_empty();
        self.origins.current_type.source_function_pointer_header=!actual.is_empty();
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
            .lookup_value_owner(&self.source_value_path(segments, path), path)
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
        let parameters=parameters.iter().map(clone_callable_type_origin).collect::<Vec<_>>();let result=clone_callable_type_origin(&result);
        let copies = parameters
            .iter()
            .map(SourceOrigin::direct_copy_value)
            .collect::<Vec<_>>();
        let roles=parameters.iter().map(|origin|self.origins.standard_sum_ownership_facts(origin).profile).collect::<Vec<_>>();let result_role=self.origins.standard_sum_ownership_facts(&result).profile;
        let actual=rules::read_rust_function_pointer_domain_error(self.ordinary_completion_context(),&copies,result.direct_copy_value(),&roles,&result_role,self.source_version_minor);
        let reason=rules::read_rust_function_pointer_header_domain_error(self.ordinary_completion_context(),&copies,result.direct_copy_value(),&roles,&result_role,self.source_version_minor,self.header_discovery);
        if !reason.is_empty() {
            return Err(unsupported("Rust function reference", &reason, span));
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
        origin.proven=actual.is_empty();origin.copy_evidence=actual.is_empty().then_some(());origin.source_function_pointer_header=!actual.is_empty();
        let owners=parameters.iter().chain(std::iter::once(&result)).map(|origin|origin.expanded_source_owner.clone().or_else(||origin.source_owner.clone())).collect::<Option<Vec<_>>>().ok_or_else(||unsupported("Rust function reference","the complete source function needs every actual parameter and result owner",span))?;
        origin.source_owner=Some(crate::translation::source_signatures::SourceOwner {kind:"function".into(),path:Vec::new(),arguments:owners});
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
            Err(unsupported("Rust callable observation", &reason, span))
        }
    }
}
