//! Source token/origin transport; shared generated rules own field/binding plans.
use super::{
    Options, Result, RustParser, SCtor, SData, SExpr, SField, SNode, SPattern, SPatternNode, SRow,
    SourceDeclaration, SourceOrigin, Span, Token, TokenKind, Type, span, type_error, unsupported,
};
use crate::translation::rust_record_rules::{
    accept_rust_record_source_name, plan_rust_record_fields, read_rust_record_binding_names,
    read_rust_record_field_position,
};

mod patterns;
mod updates;
mod variants;

#[derive(Clone)]
pub(super) struct NamedRecord {
    owner: String,
    constructor: String,
    names: Vec<String>,
    syntax: &'static str,
    update: bool,
}

impl RustParser {
    fn record_source_type(&self, ty: &Type, path: &[String]) -> Type {
        match ty {
            Type::Named { path: names, span } => {
                let owner = self.origins.lookup(&self.resolve(names, path), path);
                if let Some(owner) =
                    owner.filter(|owner| matches!(owner.declaration, SourceDeclaration::Data))
                {
                    let mut rooted = vec!["crate".to_owned()];
                    rooted.extend(owner.full_name.split('.').map(str::to_owned));
                    Type::Named {
                        path: rooted,
                        span: *span,
                    }
                } else {
                    ty.clone()
                }
            }
            Type::Array { element, reference } => Type::Array {
                element: Box::new(self.record_source_type(element, path)),
                reference: *reference,
            },
            Type::Tuple { elements } => Type::Tuple {
                elements: elements
                    .iter()
                    .map(|element| self.record_source_type(element, path))
                    .collect(),
            },
            _ => ty.clone(),
        }
    }

    fn record_owner(&self, segments: &[String], path: &[String]) -> Option<NamedRecord> {
        let found = self.origins.lookup(&self.resolve(segments, path), path)?;
        self.named_records.get(&found.full_name).cloned()
    }

    fn record_brace_owner(&self, segments: &[String], path: &[String]) -> Option<NamedRecord> {
        if let Some(record) = self.record_owner(segments, path) {
            return Some(record);
        }
        let found = self.origins.lookup(&self.resolve(segments, path), path)?;
        let SourceDeclaration::Constructor { owner, fields, .. } = found.declaration else {
            return None;
        };
        Some(NamedRecord {
            owner,
            constructor: found.full_name,
            names: (0..fields.len()).map(|index| index.to_string()).collect(),
            syntax: if fields.is_empty() { "unit" } else { "tuple" },
            update: false,
        })
    }

    fn record_binding_names(&self, count: usize) -> Vec<String> {
        let reserved = self
            .reserved_binding_names
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        read_rust_record_binding_names(
            &reserved,
            f64::from(
                u32::try_from(count).expect("a source binding count fits the shared array limit"),
            ),
        )
    }

    pub(super) fn record_item(&mut self, path: &[String]) -> Result<SData> {
        let start = self.cursor.advance();
        let token = self.cursor.identifier(Some("struct"))?;
        let name = token.value.clone();
        if !accept_rust_record_source_name(&name) {
            return Err(unsupported(
                "Rust record type owner",
                "a builtin-shadowing record needs its complete namespace interpretation",
                span(&token, &token),
            ));
        }
        if self.cursor.is("<") || self.cursor.is("where") {
            return Err(unsupported(
                "generic Rust record",
                "record parameters and predicates need their complete ownership interpretation",
                span(&start, self.cursor.peek()),
            ));
        }
        let mut fields = Vec::new();
        let mut origins = Vec::new();
        let mut names = Vec::new();
        let syntax = if self.cursor.eat("{").is_some() {
            while !self.cursor.is("}") {
                self.attributes(None)?;
                self.visibility()?;
                let field = self.cursor.identifier(Some("record field"))?;
                self.cursor.expect(":", Some("record field"))?;
                let ty = self.parse_type()?;
                names.push(field.value.clone());
                fields.push(SField {
                    name: Some(field.value),
                    ty,
                    rocq_type: None,
                    span: None,
                });
                origins.push(self.origins.current_type.clone());
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect("}", Some("struct"))?;
            "named"
        } else if self.cursor.eat("(").is_some() {
            while !self.cursor.is(")") {
                self.attributes(None)?;
                self.visibility()?;
                names.push(fields.len().to_string());
                let ty = self.parse_type()?;
                fields.push(SField {
                    name: None,
                    ty,
                    rocq_type: None,
                    span: None,
                });
                origins.push(self.origins.current_type.clone());
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect(")", Some("struct"))?;
            self.cursor.expect(";", Some("struct"))?;
            "tuple"
        } else {
            self.cursor.expect(";", Some("struct"))?;
            "unit"
        };
        if plan_rust_record_fields(&names, &names)
            .first()
            .is_some_and(|code| *code < 0.0)
        {
            return Err(type_error(
                "a Rust record declares a duplicate field",
                span(&start, self.cursor.peek()),
            ));
        }
        let mut owner_path = path.to_vec();
        owner_path.push(name.clone());
        let full_name = owner_path.join(".");
        let constructor = format!("{full_name}.{name}");
        self.named_records.insert(
            full_name.clone(),
            NamedRecord {
                owner: full_name.clone(),
                constructor,
                names,
                syntax,
                update: true,
            },
        );
        self.origins
            .declaration(&name, path, SourceDeclaration::Data)?;
        let mut result_path = vec!["crate".to_owned()];
        result_path.extend(owner_path.clone());
        self.origins.declaration(
            &name,
            &owner_path,
            SourceDeclaration::Constructor {
                owner: full_name,
                fields: origins,
                result: SourceOrigin::with_type(
                    Type::Named {
                        path: result_path,
                        span: None,
                    },
                    &[],
                    Vec::new(),
                ),
            },
        )?;
        Ok(SData {
            name: name.clone(),
            ctors: vec![SCtor { name, fields }],
            span: span(&start, self.cursor.peek()),
            generated: false,
            source_unit_clone_derivation: None,
        })
    }

    pub(super) fn record_construction(
        &mut self,
        segments: &[String],
        path: &[String],
        start: &Token,
    ) -> Result<SExpr> {
        let record = self.record_brace_owner(segments, path);
        self.cursor.expect("{", Some("record construction"))?;
        let mut names = Vec::new();
        let mut values = Vec::new();
        let mut base = None;
        while !self.cursor.is("}") {
            if self.cursor.eat("..").is_some() {
                base = Some(self.expr(path, Options::default())?);
                break;
            }
            self.attributes(None)?;
            let next = self.cursor.peek();
            let numeric = next.kind == TokenKind::Number
                && (next.raw == "0"
                    || !next.raw.starts_with('0')
                        && next.raw.bytes().all(|byte| byte.is_ascii_digit()));
            let field = if numeric {
                self.cursor.advance()
            } else {
                self.cursor.identifier(Some("record field"))?
            };
            let name = if numeric {
                field.raw.clone()
            } else {
                field.value.clone()
            };
            let colon = self.cursor.eat(":").is_some();
            if numeric && !colon {
                self.cursor.expect(":", Some("tuple record field"))?;
            }
            let value = if colon {
                self.expr(path, Options::default())?
            } else {
                SExpr::new(
                    SNode::Name {
                        path: vec![field.value.clone()],
                    },
                    span(&field, &field),
                )
            };
            names.push(name);
            values.push(value);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("record construction"))?;
        let range = span(start, self.cursor.peek());
        let Some(record) = record else {
            if !self.origins.collecting {
                return Err(unsupported(
                    "Rust record constructor",
                    "the complete source must own its nominal record declaration",
                    range,
                ));
            }
            let mut guessed = self.resolve(segments, path);
            guessed.push(segments.last().cloned().unwrap_or_default());
            return Ok(SExpr::new(
                SNode::App {
                    func: Box::new(SExpr::new(SNode::Name { path: guessed }, None)),
                    args: values,
                },
                range,
            ));
        };
        if let Some(base) = base {
            return self.record_update(&record, &names, values, base, path, range);
        }
        let plan = plan_rust_record_fields(&record.names, &names);
        if plan.first().is_some_and(|code| *code < 0.0) {
            return Err(type_error(
                "a Rust record construction has duplicate, unknown or missing fields",
                range,
            ));
        }
        let positions = plan
            .iter()
            .map(|value| {
                value
                    .to_string()
                    .parse::<usize>()
                    .map_err(|_| type_error("invalid Rust record source position", range))
            })
            .collect::<Result<Vec<_>>>()?;
        let bindings = self.record_binding_names(values.len());
        let constructor_path = record
            .constructor
            .split('.')
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let ordered = positions
            .iter()
            .map(|index| values[*index].clone())
            .collect::<Vec<_>>();
        self.origins.call(&constructor_path, &ordered, &[], range)?;
        let mut rooted = vec!["crate".to_owned()];
        rooted.extend(constructor_path.clone());
        let args = positions
            .iter()
            .map(|index| {
                SExpr::new(
                    SNode::Name {
                        path: vec![bindings[*index].clone()],
                    },
                    range,
                )
            })
            .collect::<Vec<_>>();
        let constructor = SExpr::new(SNode::Name { path: rooted }, range);
        let mut body = if args.is_empty() {
            constructor
        } else {
            SExpr::new(
                SNode::App {
                    func: Box::new(constructor),
                    args,
                },
                range,
            )
        };
        let field_origins = self
            .origins
            .lookup(&constructor_path, &[])
            .and_then(|entry| {
                if let SourceDeclaration::Constructor { fields, .. } = entry.declaration {
                    Some(fields)
                } else {
                    None
                }
            });
        let binding_types = (0..values.len())
            .map(|index| {
                let position = positions.iter().position(|position| *position == index)?;
                let origin = field_origins.as_ref()?.get(position)?;
                origin
                    .ty
                    .as_ref()
                    .map(|ty| self.record_source_type(ty, &origin.path).into())
            })
            .collect::<Vec<_>>();
        for ((binding, value), ty) in bindings.into_iter().zip(values).zip(binding_types).rev() {
            body = SExpr::new(
                SNode::Let {
                    name: binding,
                    ty,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                range,
            );
        }
        if let Some(found) = self.origins.lookup(&constructor_path, &[]) {
            if let SourceDeclaration::Constructor { result, .. } = found.declaration {
                self.origins.mark(&body, result);
            }
        }
        Ok(body)
    }

    pub(super) fn record_projection(
        &mut self,
        object: &SExpr,
        field: &str,
        path: &[String],
        range: Option<Span>,
    ) -> Result<Option<SExpr>> {
        let origin = self.origins.origin(object, path);
        let record = if let Some(Type::Named { path: owner, .. }) = &origin.ty {
            self.record_owner(owner, &origin.path)
        } else {
            None
        };
        let Some(record) = record else {
            return Ok(
                (self.origins.collecting && self.has_record_declarations).then(|| {
                    SExpr::new(
                        SNode::Field {
                            object: Box::new(object.clone()),
                            field: field.to_owned(),
                        },
                        range,
                    )
                }),
            );
        };
        let position = read_rust_record_field_position(&record.names, field);
        if position < 0.0 {
            return Err(type_error(
                &format!("a Rust record has no field {field}"),
                range,
            ));
        }
        let index = position
            .to_string()
            .parse::<usize>()
            .map_err(|_| type_error("invalid Rust record source position", range))?;
        let bindings = self.record_binding_names(record.names.len());
        let constructor_path = record
            .constructor
            .split('.')
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let mut rooted = vec!["crate".to_owned()];
        rooted.extend(constructor_path.clone());
        let pattern = SPattern {
            node: SPatternNode::Ctor {
                path: rooted,
                args: bindings
                    .iter()
                    .map(|name| SPattern {
                        node: SPatternNode::Bind { name: name.clone() },
                        span: None,
                    })
                    .collect(),
            },
            span: None,
        };
        let node = SExpr::new(
            SNode::Match {
                scrutinees: vec![object.clone()],
                rows: vec![SRow {
                    patterns: vec![pattern],
                    body: SExpr::new(
                        SNode::Name {
                            path: vec![bindings[index].clone()],
                        },
                        range,
                    ),
                    span: None,
                }],
            },
            range,
        );
        if let Some(found) = self.origins.lookup(&constructor_path, &[]) {
            if let SourceDeclaration::Constructor { fields, .. } = found.declaration {
                if let Some(field) = fields.get(index) {
                    self.origins.mark(&node, field.clone());
                }
            }
        }
        Ok(Some(node))
    }

    pub(super) fn record_value_path(
        &self,
        segments: &[String],
        path: &[String],
        called: bool,
    ) -> Option<Vec<String>> {
        if segments.len() == 1 && self.origins.scope.contains_key(&segments[0]) {
            return None;
        }
        let record = self.record_owner(segments, path)?;
        if if called {
            record.syntax != "tuple"
        } else {
            record.syntax != "unit"
        } {
            return None;
        }
        let mut rooted = vec!["crate".to_owned()];
        rooted.extend(record.constructor.split('.').map(str::to_owned));
        Some(rooted)
    }
}
