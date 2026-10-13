//! Portable type admission and immutable array element checks.

use super::super::array_rules::{accept_rust_source_array_element, accept_rust_source_array_owner};
use super::super::indirection_rules::{
    accept_rust_source_box_owner, accept_rust_source_lifetime_reference,
};
use super::super::product_rules::accept_rust_immutable_product_type;
use super::super::{
    BOOL, Result, RustParser, STRING, SourceOrigin, Token, TokenKind, Type, UNIT,
    UNSUPPORTED_TYPES, rust_fixed_type, span, unsupported,
};

impl RustParser {
    /// Types: machine integers, `bool`, `String`, `&str`, `()`, crate types,
    /// and `&T`/`Box<T>`, which denote the value `T` since every portable value
    /// is immutable.
    pub(in super::super) fn parse_type_value(&mut self) -> Result<Type> {
        let token = self.peek();
        if self.cursor.eat("&").is_some() {
            let source_lifetime = if self.cursor.is("'") {
                self.cursor.advance();
                let lifetime = self.cursor.identifier(Some("lifetime"))?;
                if !accept_rust_source_lifetime_reference(
                    &lifetime.value,
                    self.lifetime_bindings.contains(&lifetime.value),
                ) {
                    return Err(unsupported(
                        "lifetime parameter",
                        "a reference needs a declared function lifetime, an elided placeholder, or 'static",
                        span(&token, &lifetime),
                    ));
                }
                if self.lifetime_bindings.contains(&lifetime.value) && !self.lifetime_header {
                    return Err(unsupported(
                        "local lifetime annotation",
                        "named lifetimes in body annotations require an explicit region proof",
                        span(&token, &lifetime),
                    ));
                }
                Some(lifetime.value)
            } else {
                None
            };
            if self.cursor.is("mut") {
                return Err(unsupported(
                    "mutable reference",
                    "mutation is outside the portable core",
                    span(&token, self.cursor.peek()),
                ));
            }
            if self.cursor.eat("[").is_some() {
                let element_token = self.peek();
                let inner = self.parse_type()?;
                self.array_element(&token, &element_token)?;
                self.cursor.expect("]", Some("immutable slice"))?;
                let result = self.origins.typed(
                    Type::Array {
                        element: Box::new(inner),
                        reference: false,
                    },
                    &self.module_path,
                    vec!["reference".to_owned()],
                );
                self.origins.current_type.array_source_model = Some("immutable slice");
                self.origins.current_type.source_lifetimes = vec![source_lifetime];
                return Ok(result);
            }
            if self.cursor.is("str") {
                self.cursor.advance();
                let result =
                    self.origins
                        .typed(STRING, &self.module_path, vec!["reference".to_owned()]);
                self.origins.current_type.text_source_model = Some("string slice");
                self.origins.current_type.source_lifetimes = vec![source_lifetime];
                return Ok(result);
            }
            let inner = self.parse_type()?;
            let origin = self.origins.current_type.clone();
            let mut wrappers = vec!["reference".to_owned()];
            wrappers.extend(origin.wrappers);
            let result = self.origins.typed(inner, &origin.path, wrappers);
            self.origins.current_type.text_source_model = origin.text_source_model;
            self.origins.current_type.array_source_model = origin.array_source_model;
            self.origins.current_type.source_lifetimes = origin.source_lifetimes;
            self.origins
                .current_type
                .source_lifetimes
                .insert(0, source_lifetime);
            return Ok(result);
        }
        if self.cursor.eat("(").is_some() {
            if self.cursor.eat(")").is_some() {
                return Ok(self.origins.typed(UNIT, &self.module_path, Vec::new()));
            }
            let first = self.parse_type()?;
            let first_origin = self.origins.current_type.clone();
            if self.cursor.eat(",").is_none() {
                self.cursor.expect(")", Some("parenthesised type"))?;
                return Ok(first);
            }
            let mut elements = vec![first];
            let mut fields = vec![first_origin];
            while !self.cursor.is(")") {
                elements.push(self.parse_type()?);
                fields.push(self.origins.current_type.clone());
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect(")", Some("tuple type"))?;
            let children_accepted = fields.iter().all(SourceOrigin::direct_copy_value);
            if !accept_rust_immutable_product_type("tuple", children_accepted, false) {
                return Err(unsupported(
                    "immutable Rust tuple product",
                    "every field requires a direct recursively immutable scalar Copy source value",
                    span(&token, self.cursor.peek()),
                ));
            }
            let ty = self
                .origins
                .typed(Type::Tuple { elements }, &self.module_path, Vec::new());
            self.origins.current_type.copy_evidence = children_accepted.then_some(());
            self.origins.current_type.fields = Some(fields);
            return Ok(ty);
        }
        let segments = self.path_segments("type")?;
        let name = segments.join("::");
        let resolved = self.resolve(&segments, &self.module_path);
        if accept_rust_source_array_owner(&name, false)
            || accept_rust_source_array_owner(&resolved.join("::"), false)
        {
            self.origins.vector_owner(
                &segments,
                &resolved,
                &self.module_path,
                span(&token, self.cursor.peek()),
            )?;
            self.cursor.expect("<", Some("Vec"))?;
            let element_token = self.peek();
            let inner = self.parse_type()?;
            self.array_element(&token, &element_token)?;
            self.cursor.expect(">", Some("Vec"))?;
            return Ok(self.origins.typed(
                Type::Array {
                    element: Box::new(inner),
                    reference: false,
                },
                &self.module_path,
                Vec::new(),
            ));
        }
        if accept_rust_source_box_owner(&name, false)
            || accept_rust_source_box_owner(&resolved.join("::"), false)
        {
            self.origins.box_owner(
                &segments,
                &resolved,
                &self.module_path,
                span(&token, self.cursor.peek()),
            )?;
            self.cursor.expect("<", Some("Box"))?;
            let inner = self.parse_type()?;
            self.cursor.expect(">", Some("Box"))?;
            let origin = self.origins.current_type.clone();
            let mut wrappers = vec!["box".to_owned()];
            wrappers.extend(origin.wrappers);
            let result = self.origins.typed(inner, &origin.path, wrappers);
            self.origins.current_type.text_source_model = origin.text_source_model;
            self.origins.current_type.array_source_model = origin.array_source_model;
            self.origins.current_type.source_lifetimes = origin.source_lifetimes;
            return Ok(result);
        }
        if self.cursor.is("<") {
            return Err(unsupported(
                &format!("type {name}<…>"),
                "generic types are outside the portable core",
                span(&token, self.cursor.peek()),
            ));
        }
        if segments.len() == 1
            && let Some(fixed) = rust_fixed_type(&name)
        {
            return Ok(self.origins.typed(fixed, &self.module_path, Vec::new()));
        }
        if name == "f64" {
            return Ok(self
                .origins
                .typed(super::FLOAT, &self.module_path, Vec::new()));
        }
        if name == "bool" {
            return Ok(self.origins.typed(BOOL, &self.module_path, Vec::new()));
        }
        if name == "String" || resolved.join("::") == "std::string::String" {
            return Ok(self.origins.typed(STRING, &self.module_path, Vec::new()));
        }
        if UNSUPPORTED_TYPES.contains(&name.as_str()) {
            return Err(unsupported(
                &format!("Rust type {name}"),
                "outside the portable core",
                span(&token, &token),
            ));
        }
        let ty = Type::Named {
            path: self.resolve(&segments, &self.module_path),
            span: span(&token, &token),
        };
        Ok(self.origins.typed(ty, &self.module_path, Vec::new()))
    }

    pub(in super::super) fn array_element(
        &self,
        token: &Token,
        element_token: &Token,
    ) -> Result<()> {
        let origin = &self.origins.current_type;
        let shadowed = !self.origins.collecting
            && element_token.kind == TokenKind::Identifier
            && self
                .origins
                .lookup(
                    std::slice::from_ref(&element_token.value),
                    &self.module_path,
                )
                .is_some();
        if shadowed
            || !accept_rust_source_array_element(
                origin.ty.as_ref().map_or("", Type::kind),
                !origin.wrappers.is_empty(),
            )
        {
            return Err(unsupported(
                "Rust array element",
                "only direct immutable scalar and owned string elements are admitted",
                span(token, self.cursor.peek()),
            ));
        }
        Ok(())
    }
}
