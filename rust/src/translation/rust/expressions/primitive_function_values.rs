//! Source token/type/origin transport into existing shared operation bodies.
use super::primitive_try_from::origin;
use super::{
    BOOL, FLOAT, Result, RustParser, SExpr, SNode, Span, Type, rust_fixed_type, type_error,
};
use crate::translation::{
    rust_primitive_associated_call_rules as associated, rust_primitive_default_rules as defaults,
    rust_primitive_from_rules as from_rules, rust_primitive_function_value_rules as rules,
};
fn value_type(owner: &str, span: Option<Span>) -> Result<Type> {
    match owner {
        "bool" => Ok(BOOL),
        "f64" => Ok(FLOAT),
        _ => rust_fixed_type(owner).ok_or_else(|| {
            type_error(
                "a primitive function value lost its complete source type",
                span,
            )
        }),
    }
}
impl RustParser {
    pub(in super::super) fn read_primitive_function_value(
        &mut self,
        path: &[String],
        expected: Option<&super::SourceOrigin>,
    ) -> Result<Option<SExpr>> {
        let Some(head) = self.primitive_try_head(path) else {
            return Ok(None);
        };
        if self.cursor.is_at("(", head.offset) {
            return Ok(None);
        }
        let method = head.resolved.last().cloned().unwrap_or_default();
        let trait_alias = self
            .aliases
            .get(&path.join("."))
            .and_then(|aliases| aliases.get("Default"));
        let trait_conflict = method == "default"
            && (self.origins.lookup(&["Default".to_owned()], path).is_some()
                || self.origins.scope.contains_key("Default")
                || trait_alias.is_some_and(|segments| {
                    !defaults::accept_rust_primitive_default_import(
                        &segments.join("::"),
                        true,
                        false,
                    )
                }));
        let owner = if method == "default" {
            defaults::read_rust_primitive_default_owner(
                &head.resolved,
                self.ordinary_completion_context(),
                head.shadowed || trait_conflict,
                head.absolute,
            )
        } else if method == "from" {
            from_rules::read_rust_primitive_from_owner(
                &head.resolved,
                self.ordinary_completion_context(),
                head.shadowed,
                head.absolute,
            )
        } else {
            associated::read_rust_primitive_associated_owner(
                &head.resolved,
                self.ordinary_completion_context(),
                head.shadowed,
                head.absolute,
            )
        };
        let parameter = expected
            .filter(|origin| matches!(origin.ty, Some(Type::Function { .. })))
            .and_then(|origin| origin.callable_parameters.as_ref())
            .and_then(|parameters| parameters.first());
        let parameter_owner = parameter.and_then(|origin| {
            origin
                .expanded_source_owner
                .as_ref()
                .or(origin.source_owner.as_ref())
        });
        let (bits, signed) = match parameter.and_then(|origin| origin.ty.as_ref()) {
            Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
            _ => (String::new(), false),
        };
        let source = if method == "from" {
            rules::read_rust_primitive_from_function_source(
                &owner,
                expected
                    .and_then(|origin| match &origin.ty {
                        Some(Type::Function { params, .. }) => {
                            Some(u32::try_from(params.len()).map_or(f64::INFINITY, f64::from))
                        }
                        _ => None,
                    })
                    .unwrap_or(-1.0),
                parameter
                    .and_then(|origin| origin.ty.as_ref())
                    .map_or("", Type::kind),
                &bits,
                signed,
                parameter_owner.map_or("", |owner| owner.kind.as_str()),
                &parameter_owner.map_or_else(String::new, |owner| owner.path.join("::")),
                parameter_owner.map_or(-1.0, |owner| {
                    u32::try_from(owner.arguments.len()).map_or(f64::INFINITY, f64::from)
                }),
                parameter.is_some_and(|origin| origin.proven),
                parameter.map_or(-1.0, |origin| {
                    u32::try_from(origin.wrappers.len()).map_or(f64::INFINITY, f64::from)
                }),
            )
        } else {
            String::new()
        };
        let conversion = method == "from" && !source.is_empty();
        let arity = if conversion {
            1.0
        } else {
            rules::read_rust_primitive_function_value_arity(&owner, &method)
        };
        if arity < 0.0 {
            return Ok(None);
        }
        let span = Some(Span::new(
            head.start.start,
            self.cursor.peek_at(head.offset - 1).end,
        ));
        let mut params = Vec::new();
        let mut parameter_origins = Vec::new();
        let mut args = Vec::new();
        // The shared signature domain has at most two parameters. Bounds are
        // compared as metadata without casting any represented source operand.
        for index in 0..2u32 {
            if f64::from(index) >= arity {
                break;
            }
            let name = self.fresh_product_binding();
            let parameter_owner = if conversion {
                source.clone()
            } else {
                rules::read_rust_primitive_function_value_parameter_owner(
                    &owner,
                    &method,
                    f64::from(index),
                )
            };
            let ty = value_type(&parameter_owner, span)?;
            let actual = origin(ty.clone(), &parameter_owner, path);
            let token = if index == 0 {
                head.start.clone()
            } else {
                self.cursor.peek_at(head.offset - 1).clone()
            };
            let argument = SExpr::new(
                SNode::Name {
                    path: vec![name.clone()],
                },
                Some(Span::new(token.start, token.end)),
            );
            self.origins.mark(&argument, actual.clone());
            args.push(argument);
            params.push(super::super::SParam {
                name,
                optional: false,
                default_value: None,
                ty: Some(ty),
                span: None,
                guard: None,
                rocq_type: None,
            });
            parameter_origins.push(actual);
        }
        let previous = self.origins.enter(None);
        for (parameter, actual) in params.iter().zip(&parameter_origins) {
            self.origins
                .scope
                .insert(parameter.name.clone(), actual.clone());
        }
        let mut token = head.start.clone();
        token.value = method.clone();
        let body = if conversion {
            self.project_primitive_from(args[0].clone(), &owner, span, path)
        } else if method == "default" {
            self.project_primitive_default(&owner, path, span)
        } else {
            self.project_primitive_associated_body(&args[0], &token, &args[1..], span, path)
        };
        self.origins.scope = previous;
        let body = body?;
        for _ in 0..head.offset {
            self.cursor.advance();
        }
        let result_owner = if conversion {
            owner.clone()
        } else {
            rules::read_rust_primitive_function_value_result_owner(&owner, &method)
        };
        let result = value_type(&result_owner, span)?;
        let ty = Type::Function {
            params: params
                .iter()
                .map(|parameter| {
                    parameter
                        .ty
                        .clone()
                        .expect("complete original primitive parameter")
                })
                .collect(),
            ret: Box::new(result.clone()),
        };
        let node = SExpr::new(
            SNode::Lambda {
                params,
                ret: Some(result.clone()),
                captures: Vec::new(),
                assigned_captures: None,
                body: Box::new(body),
            },
            span,
        );
        let mut actual = super::SourceOrigin::with_type(ty, path, Vec::new());
        actual.copy_evidence = Some(());
        actual.callable_parameters = Some(parameter_origins);
        let mut result_origin = origin(result, &result_owner, path);
        if result_owner == "f64" {
            result_origin.float_source_model = Some("binary64 scalar");
        }
        actual.callable_result = Some(Box::new(result_origin));
        self.origins.mark(&node, actual);
        Ok(Some(node))
    }
}
