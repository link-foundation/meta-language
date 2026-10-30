//! Qualified name, constructor, primitive and conversion expressions.
use super::array_rules::accept_rust_source_array_owner;
use super::indirection_rules::accept_rust_source_box_owner;
use super::{
    FLOAT, Flavor, Options, Result, RustParser, SExpr, SNode, SourceOrigin, Span, Token, Type,
    rust_fixed_type, span, type_error, unsupported,
};

impl RustParser {
    /// A name, a call of a function or constructor, or a conversion (`u64::from(x)`).
    pub(super) fn path_expression(
        &mut self,
        token: &Token,
        path: &[String],
        options: Options,
    ) -> Result<SExpr> {
        let segments = self.path_segments("expression")?;
        let name = segments.join("::");
        let resolved = self.resolve(&segments, path);
        if !self.primitive_owner_model.is_empty()
            && segments.len() > 1
            && super::float_predicate_rules::accept_rust_float_constant_name(
                segments.last().unwrap(),
            )
            && let Some(owner) =
                self.read_standard_primitive_position(&segments[..segments.len() - 1], path)?
        {
            let range = Some(Span::new(
                token.start,
                self.cursor.tokens[self.cursor.index - 1].end,
            ));
            if self.cursor.is("(") || self.origins.scope.contains_key(&segments[0]) {
                return Err(unsupported(
                    "Rust primitive constant owner",
                    "an associated constant needs its actual unshadowed type path and value syntax",
                    range,
                ));
            }
            let name = segments.last().unwrap();
            let result = super::float_predicates::constant_expression(name, range)?;
            self.primitive_constant_sites
                .push(serde_json::json!({"owner":owner,"name":name,"span":range}));
            let mut origin = SourceOrigin::with_type(FLOAT, path, Vec::new());
            origin.source_owner = Some(crate::translation::source_signatures::SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["f64".to_owned()],
                arguments: Vec::new(),
            });
            origin.primitive_owner_position = Some(owner);
            self.origins.mark(&result, origin);
            return Ok(result);
        }
        if segments.len() == 2
            && segments[0] == "f64"
            && super::float_predicate_rules::accept_rust_float_constant_name(&segments[1])
        {
            let range = Some(Span::new(
                token.start,
                self.cursor.tokens[self.cursor.index - 1].end,
            ));
            let source_owner = vec!["f64".to_owned()];
            let owner = self.resolve(&source_owner, path);
            let shadowed = self.origins.lookup(&source_owner, path).is_some()
                || self.origins.lookup(&owner, path).is_some();
            if !super::float_predicate_rules::accept_rust_float_constant_owner(
                &owner.join("::"),
                shadowed,
            ) || self.cursor.is("(")
            {
                return Err(unsupported(
                    "Rust binary64 constant",
                    "associated constants require the exact unshadowed primitive f64 owner and value syntax",
                    range,
                ));
            }
            let result = super::float_predicates::constant_expression(&segments[1], range)?;
            let mut origin = SourceOrigin::with_type(FLOAT, path, Vec::new());
            origin.source_owner = Some(crate::translation::source_signatures::SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["f64".to_owned()],
                arguments: Vec::new(),
            });
            self.origins.mark(&result, origin);
            return Ok(result);
        }
        if self.cursor.is("::")
            && self.cursor.is_at("<", 1)
            && (accept_rust_source_array_owner(&name, false)
                || accept_rust_source_array_owner(&resolved.join("::"), false))
        {
            self.refuse_shared_bare_vector(span(token, self.cursor.peek()))?;
            self.origins.vector_owner(
                &segments,
                &resolved,
                path,
                span(token, self.cursor.peek()),
            )?;
            self.cursor.advance();
            self.cursor.advance();
            let element_token = self.peek();
            let element = self.parse_type()?;
            let element_owner = self.origins.current_type.source_owner.clone();
            self.array_element(token, &element_token)?;
            self.cursor.expect(">", Some("Vec constructor"))?;
            self.cursor.expect("::", Some("Vec constructor"))?;
            self.cursor.expect("new", Some("Vec constructor"))?;
            self.cursor.expect("(", Some("Vec constructor"))?;
            self.cursor.expect(")", Some("Vec constructor"))?;
            let result = SExpr::new(
                SNode::Array {
                    items: Vec::new(),
                    element: Some(element.clone()),
                },
                span(token, self.cursor.peek()),
            );
            let mut origin = SourceOrigin::with_type(
                Type::Array {
                    element: Box::new(element),
                    reference: false,
                },
                path,
                Vec::new(),
            );
            origin.source_owner =
                element_owner.map(|owner| crate::translation::source_signatures::SourceOwner {
                    kind: "library".to_owned(),
                    path: vec!["std".to_owned(), "vec".to_owned(), "Vec".to_owned()],
                    arguments: vec![owner],
                });
            self.origins.mark(&result, origin);
            return Ok(result);
        }
        if let Some(shared) = self.shared_array_constructor(&segments, token, path)? {
            return Ok(shared);
        }
        let c = &self.cursor;
        if c.is("::") && c.is_at("<", 1) {
            return Err(unsupported(
                "turbofish",
                "generic functions are outside the portable core",
                span(token, c.peek()),
            ));
        }
        let last_is_upper = segments
            .last()
            .and_then(|segment| segment.chars().next())
            .is_some_and(|first| first.is_ascii_uppercase());
        if c.is("{") && (last_is_upper || self.has_record_constructions) && !options.no_struct {
            return self.record_construction(&segments, path, token);
        }
        let record_path = self.record_value_path(&segments, path, c.is("("));
        self.require_type_alias_value_constructor(
            &segments,
            path,
            Some(Span {
                start: token.start,
                end: self.cursor.tokens[self.cursor.index - 1].end,
            }),
        )?;
        if !c.is("(") {
            if let Some(reference) =
                self.read_function_reference(&segments, path, span(token, token))?
            {
                return Ok(reference);
            }
            let node = SExpr::new(
                SNode::Name {
                    path: record_path.unwrap_or_else(|| self.source_value_path(&segments, path)),
                },
                span(token, token),
            );
            self.shared_array_name(&node, path)?;
            self.origins.check_owned_name(&node, path)?;
            return Ok(node);
        }
        self.cursor.advance();
        let callee = SExpr::new(
            SNode::Name {
                path: segments.clone(),
            },
            span(token, token),
        );
        let mut args = self.contextual_call_arguments(path, &callee)?;
        self.cursor.expect(")", Some("call"))?;
        let range = span(token, self.cursor.peek());
        if segments.len() == 1 && self.origins.scope.contains_key(&segments[0]) {
            let function = SExpr::new(
                SNode::Name {
                    path: segments.clone(),
                },
                span(token, token),
            );
            if let Some(call) = self.read_copy_closure_call(&function, &args, path, range)? {
                return Ok(call);
            }
            return Err(type_error(crate::translation::rust_source_constant_rules::read_rust_source_callable_value_error(self.origins.scope[&segments[0]].ty.as_ref().map_or("",Type::kind),"local value"),range));
        }
        if let Some(call) = self.source_constant_call(&segments, &args, path, range)? {
            return Ok(call);
        }
        if let Some(record_path) = record_path {
            self.origins.call(&record_path, &args, path, range)?;
            let function = SExpr::new(SNode::Name { path: record_path }, span(token, token));
            return Ok(if args.is_empty() {
                SExpr::new(function.node, range)
            } else {
                SExpr::new(
                    SNode::App {
                        func: Box::new(function),
                        args,
                    },
                    range,
                )
            });
        }
        if let Some(shared) = self.shared_array_call(&segments, &args, path, range)? {
            return Ok(shared);
        }
        if let Some(empty) =
            self.read_empty_string_constructor(&segments, args.len(), path, range)?
        {
            return Ok(empty);
        }
        let owner = &segments[..segments.len() - 1];
        let resolved_owner = if owner.is_empty() {
            Vec::new()
        } else {
            self.resolve(owner, path)
        };
        if segments.last().is_some_and(|segment| segment == "new")
            && (accept_rust_source_box_owner(&owner.join("::"), false)
                || accept_rust_source_box_owner(&resolved_owner.join("::"), false))
        {
            if !self.local_value_state_model.is_empty()
                && !super::owned_local_state_rules::accept_rust_owned_local_source_operation(
                    "box construction",
                )
            {
                return Err(unsupported(
                    "Rust owned local state",
                    "Box allocation requires a separate source ownership interpretation",
                    range,
                ));
            }
            if args.len() != 1 {
                return Err(type_error("Box::new expects 1 argument", range));
            }
            self.origins
                .box_owner(owner, &resolved_owner, path, range)?;
            let result = args.remove(0);
            self.origins.wrap(&result, path, "box")?;
            return Ok(result);
        }
        if name == "String::from" {
            self.require_type_namespace_body_constructor(&name, range)?;
            if args.len() != 1 {
                return Err(type_error("String::from expects 1 argument", range));
            }
            let result = SExpr::new(
                SNode::ToString {
                    arg: Box::new(args.remove(0)),
                },
                range,
            );
            self.origins.conversion(&result, path);
            if !self.local_value_state_model.is_empty() {
                self.local_value_constructors.insert(
                    range.expect("a parsed constructor has a range"),
                    Span::new(token.start, self.cursor.tokens[self.cursor.index - 1].end),
                );
            }
            return Ok(result);
        }
        let fixed = rust_fixed_type(&segments[0]);
        if !self.ordinary_completion_context()
            && segments.len() == 2
            && segments[1] == "from"
            && let Some(to) = fixed.clone()
        {
            self.require_type_namespace_body_constructor(&name, range)?;
            if args.len() != 1 {
                return Err(type_error(format!("{name} expects 1 argument"), range));
            }
            return Ok(SExpr::new(
                SNode::Cast {
                    arg: Box::new(args.remove(0)),
                    to,
                    from: None,
                    flavor: Flavor::Exact,
                },
                range,
            ));
        }
        if segments.len() == 2 && fixed.is_some() {
            return Err(unsupported(&name, "outside the portable core", range));
        }
        if ["std", "core", "alloc"].contains(&segments[0].as_str()) {
            return Err(unsupported(
                &name,
                "the standard library is outside the portable core",
                range,
            ));
        }
        let callee = self.source_value_path(&segments, path);
        let ordinary = self.ordinary_completion_context();
        self.origins
            .call_with_fresh_vector_arguments(&callee, &args, path, range, ordinary)?;
        if crate::translation::rust_source_constant_rules::read_rust_source_call_form(
            self.ordinary_completion_context(),
            serde_json::Number::from(args.len())
                .as_f64()
                .expect("actual argument count"),
        ) == "name"
        {
            return Ok(SExpr::new(SNode::Name { path: callee }, range));
        }
        Ok(SExpr::new(
            SNode::App {
                func: Box::new(SExpr::new(SNode::Name { path: callee }, span(token, token))),
                args,
            },
            range,
        ))
    }
}
