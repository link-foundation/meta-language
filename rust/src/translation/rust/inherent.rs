//! Source owner and token transport into ordinary module/function definitions.
use super::indirection_rules::accept_rust_source_lifetime_reference;
use super::{
    Result, RustParser, SItem, SModule, SParam, SourceDeclaration, SourceOrigin, Type, span,
    unsupported,
};
use crate::translation::rust_inherent_rules::{
    accept_rust_inherent_owner_module, accept_rust_inherent_source_profiles,
    read_rust_inherent_receiver_layer_mode,
};

mod dispatch;
mod paths;

#[derive(Clone)]
pub(super) struct InherentOwner {
    pub(super) path: Vec<String>,
    pub(super) module_path: Vec<String>,
}

#[derive(Clone)]
pub(super) struct InherentMethod {
    pub(super) owner: String,
    pub(super) mode: f64,
    pub(super) receiver: Vec<String>,
}

impl RustParser {
    pub(super) fn split_reference_type_token(&mut self) {
        if !self.cursor.is("&&") {
            return;
        }
        let mut first = self.cursor.peek().clone();
        let mut second = first.clone();
        first.value = "&".to_owned();
        first.raw = "&".to_owned();
        first.end = first.start + 1;
        second.value = "&".to_owned();
        second.raw = "&".to_owned();
        second.start += 1;
        let index = self.cursor.index;
        self.cursor.tokens.splice(index..index + 1, [first, second]);
    }
    pub(super) fn inherent_item(&mut self, path: &[String]) -> Result<SModule> {
        let start = self.cursor.advance();
        let models = [
            &self.origins.array_result_model,
            &self.completion_model,
            &self.source_signature_model,
            &self.scalar_loop_model,
            &self.primitive_owner_model,
            &self.empty_option_source_model,
            &self.type_namespace_model,
            &self.pure_unit_call_model,
            &self.local_value_state_model,
            &self.const_function_model,
            &self.shared_array_handle_source_model,
        ]
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
        if !accept_rust_inherent_source_profiles(&models) {
            return Err(unsupported(
                "Rust inherent profile composition",
                "closed source models need an explicit method composition proof",
                span(&start, self.cursor.peek()),
            ));
        }
        if self.cursor.is("<") {
            return Err(unsupported(
                "generic Rust implementation",
                "type parameters require their complete trait and ownership interpretation",
                span(&start, self.cursor.peek()),
            ));
        }
        let segments = self.path_segments("implementation owner")?;
        if self.cursor.is("<") || self.cursor.is("for") || self.cursor.is("where") {
            return Err(unsupported(
                "generic or trait Rust implementation",
                "traits and predicates need their complete dispatch interpretation",
                span(&start, self.cursor.peek()),
            ));
        }
        let resolved = self.resolve(&segments, path);
        let found = self
            .origins
            .lookup(&resolved, path)
            .filter(|entry| matches!(entry.declaration, SourceDeclaration::Data));
        if found.is_none() && !self.inherent_discovery {
            return Err(unsupported(
                "Rust implementation owner",
                "an inherent implementation needs its actual source datatype",
                span(&start, self.cursor.peek()),
            ));
        }
        let owner = found.map_or_else(
            || {
                if resolved.first().is_some_and(|head| head == "crate") {
                    resolved[1..].to_vec()
                } else {
                    path.iter()
                        .chain(if resolved.first().is_some_and(|head| head == "self") {
                            &resolved[1..]
                        } else {
                            &resolved
                        })
                        .cloned()
                        .collect()
                }
            },
            |entry| {
                entry
                    .full_name
                    .split('.')
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            },
        );
        if !accept_rust_inherent_owner_module(path, &owner) {
            return Err(unsupported(
                "Rust implementation location",
                "cross-module implementation placement requires its explicit namespace mapping",
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect("{", Some("implementation"))?;
        let previous = self.inherent_owner.replace(InherentOwner {
            path: owner.clone(),
            module_path: path.to_vec(),
        });
        let previous_module = self.module_path.clone();
        let result = (|| -> Result<Vec<SItem>> {
            self.inner_attributes()?;
            let mut items = Vec::new();
            while !self.cursor.is("}") {
                self.module_path = path.to_vec();
                self.attributes(None)?;
                self.visibility()?;
                if !self.cursor.is("fn") {
                    return Err(unsupported(
                        "Rust associated item",
                        "associated constants, types, macros and modified functions need their own source interpretation",
                        span(self.cursor.peek(), self.cursor.peek()),
                    ));
                }
                let name = self.cursor.peek_at(1).value.clone();
                self.prepare_record_method_namespace(
                    &owner,
                    &name,
                    span(self.cursor.peek(), self.cursor.peek_at(1)),
                )?;
                let function = self.function(&owner)?;
                let key = owner
                    .iter()
                    .chain(std::iter::once(&function.name))
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(".");
                if !function
                    .params
                    .first()
                    .is_some_and(|parameter| parameter.name == "self")
                {
                    self.inherent_methods.insert(
                        key,
                        InherentMethod {
                            owner: owner.join("."),
                            mode: 0.0,
                            receiver: Vec::new(),
                        },
                    );
                }
                items.push(SItem::Fn(function));
            }
            self.cursor.expect("}", Some("implementation"))?;
            Ok(items)
        })();
        self.inherent_owner = previous;
        self.module_path = previous_module;
        Ok(SModule {
            name: owner.last().cloned().unwrap_or_default(),
            items: result?,
            span: span(&start, self.cursor.peek()),
        })
    }

    pub(super) fn inherent_receiver(
        &mut self,
        name: &str,
        path: &[String],
        index: usize,
    ) -> Result<Option<(SParam, SourceOrigin)>> {
        let Some(owner) = self.inherent_owner.clone() else {
            return Ok(None);
        };
        if !(self.cursor.is("self")
            || self.cursor.is("&")
            || self.cursor.is("mut") && self.cursor.is_at("self", 1))
        {
            return Ok(None);
        }
        let start = self.peek();
        if index != 0 {
            return Err(unsupported(
                "Rust receiver position",
                "a receiver must be the first associated function parameter",
                span(&start, &start),
            ));
        }
        let borrowed = self.cursor.eat("&").is_some();
        let mut lifetime = None;
        if borrowed && self.cursor.eat("'").is_some() {
            let token = self.cursor.identifier(Some("lifetime"))?;
            if !accept_rust_source_lifetime_reference(
                &token.value,
                self.lifetime_bindings.contains(&token.value),
            ) {
                return Err(unsupported(
                    "lifetime parameter",
                    "a reference needs a declared function lifetime, an elided placeholder, or 'static",
                    span(&start, &token),
                ));
            }
            lifetime = Some(token.value);
        }
        if self.cursor.is("mut") {
            return Err(unsupported(
                "Rust mutable or explicit lifetime receiver",
                "receiver mutation and named regions need an explicit ownership interpretation",
                span(&start, self.cursor.peek()),
            ));
        }
        self.cursor.expect("self", Some("method receiver"))?;
        let ty = if self.cursor.eat(":").is_some() {
            if borrowed {
                return Err(unsupported(
                    "Rust receiver syntax",
                    "an explicitly typed receiver uses an unprefixed self binding",
                    span(&start, self.cursor.peek()),
                ));
            }
            let ty = self.parse_type()?;
            let origin = &self.origins.current_type;
            let actual = if let Type::Named { path, .. } = &ty {
                self.origins
                    .lookup(path, &origin.path)
                    .filter(|entry| matches!(entry.declaration, SourceDeclaration::Data))
            } else {
                None
            };
            if !self.inherent_discovery
                && actual.is_none_or(|actual| actual.full_name != owner.path.join("."))
            {
                return Err(unsupported(
                    "Rust explicit receiver owner",
                    "an explicit receiver must resolve to its actual implementing datatype",
                    span(&start, self.cursor.peek()),
                ));
            }
            ty
        } else {
            let mut rooted = vec!["crate".to_owned()];
            rooted.extend(owner.path.clone());
            let ty = self.origins.typed(
                Type::Named {
                    path: rooted,
                    span: None,
                },
                &owner.module_path,
                if borrowed {
                    vec!["reference".to_owned()]
                } else {
                    Vec::new()
                },
            );
            if borrowed {
                self.origins.current_type.source_lifetimes = vec![lifetime];
            }
            ty
        };
        let receiver = self.origins.current_type.wrappers.clone();
        let mode = read_rust_inherent_receiver_layer_mode(&receiver);
        if mode == 0.0 {
            return Err(unsupported(
                "Rust explicit receiver layers",
                "the receiver needs represented owned, immutable reference or standard Box layers",
                span(&start, self.cursor.peek()),
            ));
        }
        let key = path
            .iter()
            .cloned()
            .chain(std::iter::once(name.to_owned()))
            .collect::<Vec<_>>()
            .join(".");
        self.inherent_methods.insert(
            key,
            InherentMethod {
                owner: owner.path.join("."),
                mode,
                receiver,
            },
        );
        Ok(Some((
            SParam {
                name: "self".to_owned(),
                optional: false,
                default_value: None,
                ty: Some(ty),
                span: span(&start, self.cursor.peek()),
                guard: None,
                rocq_type: None,
            },
            self.origins.current_type.clone(),
        )))
    }

    pub(super) fn inherent_self_type(&mut self) -> Option<Type> {
        let owner = self.inherent_owner.as_ref()?;
        if !self.cursor.is("Self") {
            return None;
        }
        self.cursor.advance();
        let mut rooted = vec!["crate".to_owned()];
        rooted.extend(owner.path.clone());
        Some(self.origins.typed(
            Type::Named {
                path: rooted,
                span: None,
            },
            &owner.module_path,
            Vec::new(),
        ))
    }
}
