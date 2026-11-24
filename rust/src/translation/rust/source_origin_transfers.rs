impl SourceOrigins {
    pub(super) fn join(&self, origins: &[SourceOrigin]) -> SourceOrigin {
        let returning: Vec<_> = origins.iter().filter(|origin| !origin.diverging).collect();
        let Some(first) = returning.first() else {
            return SourceOrigin {
                diverging: true,
                ..SourceOrigin::value()
            };
        };
        if let Some(error) = returning
            .iter()
            .find_map(|origin| origin.callable_source_owner_error.clone())
        {
            return SourceOrigin {
                callable_source_owner_error: Some(error),
                ..SourceOrigin::uncertain()
            };
        }
        let callable_rows = returning
            .iter()
            .flat_map(|origin| callable_owners::descriptor(origin))
            .collect::<Vec<_>>();
        let callable_error =
            crate::translation::rust_callable_owner_rules::read_rust_callable_owner_join_error(
                &callable_rows,
            );
        if !callable_error.is_empty() {
            return SourceOrigin {
                callable_source_owner_error: Some(callable_error),
                ..SourceOrigin::uncertain()
            };
        }
        let owners = returning
            .iter()
            .map(|origin| origin.pointer_integer_owner())
            .collect::<Vec<_>>();
        if returning
            .iter()
            .any(|origin| origin.integer_source_owner_conflict)
            || !read_rust_pointer_integer_owner_join_error(&owners).is_empty()
        {
            return SourceOrigin::integer_conflict();
        }
        let source_extending_temporary_bindings =
            returning.iter().fold(Vec::new(), |ids, origin| {
                crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(
                    &ids,
                    &origin.source_extending_temporary_bindings,
                )
            });
        let source_value_borrow_bindings = returning.iter().fold(Vec::new(), |all, origin| {
            crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(
                &all,
                &super::copy_closure_lifetimes::value_borrows(origin),
            )
        });
        let source_value_field_borrow_bindings = if returning.iter().all(|origin| {
            origin.source_value_field_borrow_bindings == first.source_value_field_borrow_bindings
        }) {
            first.source_value_field_borrow_bindings.clone()
        } else {
            None
        };
        let source_loan_binding_identities = if first.wrappers.is_empty() {
            Vec::new()
        } else {
            returning.iter().fold(Vec::new(), |loans, origin| {
                crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(
                    &loans,
                    &crate::translation::rust_owned_binding_rules::project_rust_owned_loan_bindings(
                        origin.source_binding_identity.as_deref().unwrap_or(""),
                        &origin.source_loan_binding_identities,
                    ),
                )
            })
        };
        if returning
            .iter()
            .all(|origin| self.same_origin(origin, first))
        {
            return SourceOrigin {
                source_lexical_binding_identity: None,
                source_value_borrow_bindings,
                source_value_field_borrow_bindings,
                integer_source_owner_constraint: Some(select_rust_pointer_integer_result_owner(
                    &owners,
                )),
                array_element_copy_evidence: returning
                    .iter()
                    .all(|origin| origin.array_element_copy_evidence),
                fresh_array_value: if returning
                    .iter()
                    .all(|origin| origin.fresh_array_value.is_some())
                {
                    first.fresh_array_value
                } else {
                    None
                },
                source_extending_temporary_bindings,
                source_loan_binding_identities,
                source_place_loan_binding_identities: Vec::new(),
                source_borrowed_place: returning.iter().any(|origin| origin.source_borrowed_place),
                source_owned_result_proof: returning
                    .iter()
                    .all(|origin| origin.source_owned_result_proof),
                source_standard_sum_pattern_move: returning
                    .iter()
                    .any(|origin| origin.source_standard_sum_pattern_move),
                source_standard_sum_borrowed_field: returning
                    .iter()
                    .any(|origin| origin.source_standard_sum_borrowed_field),
                source_nominal_pattern_move: returning
                    .iter()
                    .any(|origin| origin.source_nominal_pattern_move),
                source_nominal_borrowed_field: returning
                    .iter()
                    .any(|origin| origin.source_nominal_borrowed_field),
                source_binding_identity: if returning
                    .iter()
                    .all(|origin| origin.source_binding_identity == first.source_binding_identity)
                {
                    first.source_binding_identity.clone()
                } else {
                    None
                },
                ..(*first).clone()
            };
        }
        if returning
            .iter()
            .any(|origin| origin.array_source_model.is_some())
        {
            return SourceOrigin {
                array_source_model: Some("unproved array"),
                integer_source_owner_constraint: Some(select_rust_pointer_integer_result_owner(
                    &owners,
                )),
                ..SourceOrigin::uncertain()
            };
        }
        if returning
            .iter()
            .all(|origin| origin.proven && origin.wrappers == first.wrappers)
        {
            let fields = first.fields.as_ref().and_then(|fields| {
                returning
                    .iter()
                    .all(|origin| {
                        origin
                            .fields
                            .as_ref()
                            .is_some_and(|candidate| candidate.len() == fields.len())
                    })
                    .then(|| {
                        (0..fields.len())
                            .map(|index| {
                                let values: Vec<_> = returning
                                    .iter()
                                    .map(|origin| {
                                        origin.fields.as_ref().expect("matched field count")[index]
                                            .clone()
                                    })
                                    .collect();
                                self.join(&values)
                            })
                            .collect()
                    })
            });
            let text_source_model = returning
                .iter()
                .all(|origin| origin.text_source_model == first.text_source_model)
                .then_some(first.text_source_model)
                .flatten();
            let integer_owner = select_rust_pointer_integer_result_owner(&owners);
            return SourceOrigin {
                wrappers: first.wrappers.clone(),
                source_value_borrow_bindings,
                source_value_field_borrow_bindings,
                copy_evidence: returning
                    .iter()
                    .all(|origin| origin.copy_evidence.is_some())
                    .then_some(()),
                fields,
                ty: text_source_model
                    .map(|_| super::STRING)
                    .or_else(|| super::rust_fixed_type(&integer_owner)),
                integer_source_owner_constraint: (!integer_owner.is_empty())
                    .then_some(integer_owner),
                source_extending_temporary_bindings,
                source_loan_binding_identities,
                source_place_loan_binding_identities: Vec::new(),
                text_source_model,
                source_standard_sum_borrowed_field: returning
                    .iter()
                    .any(|origin| origin.source_standard_sum_borrowed_field),
                source_standard_sum_pattern_move: returning
                    .iter()
                    .any(|origin| origin.source_standard_sum_pattern_move),
                source_nominal_pattern_move: returning
                    .iter()
                    .any(|origin| origin.source_nominal_pattern_move),
                source_nominal_borrowed_field: returning
                    .iter()
                    .any(|origin| origin.source_nominal_borrowed_field),
                ..SourceOrigin::value()
            };
        }
        if returning
            .iter()
            .any(|origin| origin.uncertain || !origin.wrappers.is_empty())
        {
            SourceOrigin::uncertain()
        } else {
            SourceOrigin::unknown()
        }
    }

    pub(super) fn checked_join(&self, origins: &[SourceOrigin]) -> Result<SourceOrigin> {
        let returning = origins
            .iter()
            .filter(|origin| !origin.diverging)
            .collect::<Vec<_>>();
        let mut groups = vec![returning];
        while let Some(group) = groups.pop() {
            if let Some(first) = group.first() {
                for origin in &group {
                    let error =
                    crate::translation::rust_copy_closure_rules::read_rust_copy_closure_join_error(
                        first.copy_closure_identity.is_some(),
                        origin.copy_closure_identity.is_some(),
                        first.copy_closure_identity == origin.copy_closure_identity,
                    );
                    if !error.is_empty() {
                        return Err(unsupported("Rust closure source join", &error, None));
                    }
                }
                if let Some(fields) = first.fields.as_ref() {
                    if group.iter().all(|origin| {
                        origin
                            .fields
                            .as_ref()
                            .is_some_and(|next| next.len() == fields.len())
                    }) {
                        for at in 0..fields.len() {
                            groups.push(
                                group
                                    .iter()
                                    .map(|origin| {
                                        &origin.fields.as_ref().expect("actual joined fields")[at]
                                    })
                                    .collect(),
                            );
                        }
                    }
                }
            }
        }
        Ok(self.join(origins))
    }

    pub(super) fn mark(&mut self, node: &SExpr, mut origin: SourceOrigin) {
        if !self.array_result_model.is_empty()
            && let SNode::Array { items, .. } = &node.node
        {
            origin.fresh_array_value = node.span.filter(|_| {
                origin.array_source_model == Some("owned vector")
                    && origin.wrappers.is_empty()
                    && items.iter().all(|item| {
                        !item.spread && self.origin(&item.value, &origin.path).direct_copy_value()
                    })
            });
        }
        if origin.wrappers.is_empty()
            && !matches!(origin.ty, Some(Type::Array { .. }))
            && !self.owned_binding_value(&origin)
        {
            origin.source_place_binding_identity = origin
                .source_place_binding_identity
                .or(origin.source_binding_identity.take());
            origin.source_place_loan_binding_identities =
                crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(
                    &origin.source_place_loan_binding_identities,
                    &origin.source_loan_binding_identities,
                );
            origin.source_loan_binding_identities.clear();
        }
        if let Some(span) = node.span {
            self.expressions.insert(span, origin);
        }
    }

    /// Each constructor's scalar evidence was retained in its live source scope.
    pub(super) fn fresh_array_result(&self, node: &SExpr, path: &[String]) -> bool {
        let children = match &node.node {
            SNode::Let { body, .. } => self.fresh_array_result(body, path),
            SNode::If {
                then, otherwise, ..
            } => self.fresh_array_result(then, path) && self.fresh_array_result(otherwise, path),
            _ => false,
        };
        accept_rust_fresh_array_result_tail(
            node.node.kind(),
            self.origin(node, path).fresh_array_value.is_some(),
            children,
        )
    }

    pub(super) fn mark_owned_string_constructor(&mut self, node: &SExpr, mut origin: SourceOrigin) {
        origin.source_lexical_binding_identity = None;
        origin.source_value_borrow_bindings.clear();
        origin.source_value_field_borrow_bindings = None;
        origin.source_standard_sum_borrowed_field = false;
        origin.source_standard_sum_pattern_move = false;
        origin.source_nominal_borrowed_field = false;
        origin.source_nominal_pattern_move = false;
        origin.source_binding_identity = None;
        origin.source_place_binding_identity = None;
        origin.source_extending_temporary_bindings.clear();
        origin.source_loan_binding_identities.clear();
        origin.source_place_loan_binding_identities.clear();
        origin.source_borrowed_place = false;
        origin.source_owned_result_proof =
            crate::translation::rust_owned_binding_rules::accept_rust_owned_binding_value(
                origin.ty.as_ref().map_or("", Type::kind),
                origin.text_source_model.unwrap_or(""),
                origin.wrappers.len() as f64,
                origin.proven,
            );
        self.mark(node, origin);
    }
    pub(super) fn mark_vector_constructor(
        &mut self,
        node: &SExpr,
        mut origin: SourceOrigin,
    ) -> Result<()> {
        if self.borrowed_vector_source_origin(&origin) {
            return self.capture_borrowed_vector_constructor(node, origin);
        }
        if self.owned_container_source_enabled {
            return self.capture_owned_vector_constructor(node, origin);
        }
        let values = match &node.node {
            SNode::Array { items, .. } => items
                .iter()
                .map(|item| self.origin(&item.value, &origin.path))
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        };
        let element = match origin.ty.as_ref() {
            Some(Type::Array { element, .. }) => {
                let mut value =
                    SourceOrigin::with_type(element.as_ref().clone(), &origin.path, Vec::new());
                self.retain_nominal_copy_origin(&mut value);
                if let Some(owner) = origin
                    .expanded_source_owner
                    .as_ref()
                    .or(origin.source_owner.as_ref())
                    .filter(|owner| {
                        owner.kind == "library"
                            && owner.path.join("::") == "std::vec::Vec"
                            && owner.arguments.len() == 1
                    })
                {
                    value.source_owner = Some(owner.arguments[0].clone());
                }
                Some(value)
            }
            _ => None,
        };
        let invalid_type = element.as_ref().is_some_and(|element| {
            !accept_rust_source_array_element(element.ty.as_ref().map_or("", Type::kind), false)
                && !self.nominal_vector_element(element)
        });
        if invalid_type
            || values.iter().any(|value| {
                (value.ty.is_some() || value.fields.is_some() || !value.wrappers.is_empty())
                    && !accept_rust_source_array_element(
                        value.ty.as_ref().map_or("tuple", Type::kind),
                        !value.wrappers.is_empty(),
                    )
                    && !self.nominal_vector_element(value)
            })
        {
            return Err(unsupported(
                "Rust array element",
                "only direct immutable scalar, owned string or verified nominal Copy vector elements are admitted",
                node.span,
            ));
        }
        origin.source_lexical_binding_identity = None;
        origin.source_value_borrow_bindings.clear();
        origin.source_value_field_borrow_bindings = None;
        origin.source_standard_sum_borrowed_field = false;
        origin.source_standard_sum_pattern_move = false;
        origin.source_nominal_borrowed_field = false;
        origin.source_nominal_pattern_move = false;
        if let Some(span) = node.span {
            self.fresh_vector_constructors.insert(span);
        }
        origin.source_binding_identity = None;
        origin.source_place_binding_identity = None;
        origin.source_loan_binding_identities.clear();
        origin.source_place_loan_binding_identities.clear();
        origin.source_extending_temporary_bindings.clear();
        origin.source_borrowed_place = false;
        self.mark(node, origin);
        Ok(())
    }

    pub(super) fn fresh_vector_argument(
        &self,
        node: &SExpr,
        path: &[String],
        allow_calls: bool,
    ) -> bool {
        let origin = self.origin(node, path);
        if self.borrowed_vector_produced_value(&origin) {
            return true;
        }
        if self.owned_vector_produced_value(&origin) {
            return true;
        }
        if matches!(origin.ty, Some(Type::Array { .. }))
            && origin.source_owned_result_proof
            && self.available_owned_binding(&origin)
        {
            return true;
        }
        if crate::translation::rust_scalar_vector_copy_rules::accept_rust_scalar_vector_copy_occurrence(node.kind(),origin.array_source_model.unwrap_or(""),u32::try_from(origin.wrappers.len()).map_or(f64::INFINITY,f64::from),node.span.is_some_and(|span|self.fresh_vector_constructors.contains(&span))){return true;}
        if allow_calls
            && accept_rust_known_fresh_vector_call(
                node.node.kind(),
                origin.array_source_model.unwrap_or(""),
                serde_json::Number::from(origin.wrappers.len())
                    .as_f64()
                    .expect("actual wrapper count"),
                origin.fresh_array_value.is_some(),
            )
        {
            return true;
        }
        let form = read_rust_fresh_vector_argument_form(
            node.node.kind(),
            origin.diverging,
            origin.array_source_model.unwrap_or(""),
            serde_json::Number::from(origin.wrappers.len())
                .as_f64()
                .expect("actual wrapper count"),
            node.span
                .is_some_and(|span| self.fresh_vector_constructors.contains(&span)),
        );
        match (form.as_str(), &node.node) {
            ("diverging" | "constructor", _) => true,
            ("body", SNode::Let { body, .. } | SNode::Print { body, .. }) => {
                self.fresh_vector_argument(body, path, allow_calls)
            }
            (
                "alternatives",
                SNode::If {
                    then, otherwise, ..
                },
            ) => {
                self.fresh_vector_argument(then, path, allow_calls)
                    && self.fresh_vector_argument(otherwise, path, allow_calls)
            }
            ("rows", SNode::Match { rows, .. }) => {
                !rows.is_empty()
                    && rows
                        .iter()
                        .all(|row| self.fresh_vector_argument(&row.body, path, allow_calls))
            }
            _ => false,
        }
    }

    pub(super) fn certify_array_result(&mut self, name: &str, path: &[String], span: Option<Span>) {
        if self.collecting {
            let full_name = path
                .iter()
                .map(String::as_str)
                .chain([name])
                .collect::<Vec<_>>()
                .join(".");
            if let Some(SourceDeclaration::Function { result, .. }) =
                self.catalog.get_mut(&full_name)
            {
                result.fresh_array_value = span;
            }
        }
    }

    pub(super) fn wrap(&mut self, node: &SExpr, path: &[String], layer: &str) -> Result<()> {
        let mut origin = self.origin(node, path);
        if crate::translation::rust_copy_closure_parameter_rules::test_rust_source_reference_lifetime_layer(layer){if let Some(identity)=origin.source_lexical_binding_identity.as_ref(){origin.source_value_borrow_bindings=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&super::copy_closure_lifetimes::value_borrows(&origin),std::slice::from_ref(identity));}}
        if crate::translation::rust_copy_closure_parameter_rules::test_rust_source_reference_lifetime_layer(layer)&&self.owned_binding_value(&origin){origin=self.capture_owned_borrow(origin);if let Some(id)=origin.source_binding_identity.as_ref(){if !((origin.source_standard_sum_borrowed_field||origin.source_nominal_borrowed_field)&&origin.source_borrowed_place)&&self.source_temporary_bindings.contains(id){origin.source_extending_temporary_bindings=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&origin.source_extending_temporary_bindings,&[id.clone()]);}}}
        if crate::translation::rust_copy_closure_parameter_rules::test_rust_source_reference_lifetime_layer(layer)&&origin.source_place_binding_identity.is_some(){origin.source_binding_identity=origin.source_place_binding_identity.clone();}
        if crate::translation::rust_copy_closure_parameter_rules::test_rust_source_reference_lifetime_layer(layer){origin.source_loan_binding_identities=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&origin.source_loan_binding_identities,&origin.source_place_loan_binding_identities);}
        if crate::translation::rust_copy_closure_parameter_rules::test_rust_source_reference_lifetime_layer(layer){origin.source_value_borrow_bindings=crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&super::copy_closure_lifetimes::value_borrows(&origin),&crate::translation::rust_owned_binding_rules::merge_rust_owned_loan_bindings(&crate::translation::rust_owned_binding_rules::project_rust_owned_loan_bindings(origin.source_binding_identity.as_deref().unwrap_or(""),&origin.source_loan_binding_identities),&crate::translation::rust_owned_binding_rules::project_rust_owned_loan_bindings(origin.source_place_binding_identity.as_deref().unwrap_or(""),&origin.source_place_loan_binding_identities)));}
        origin.wrappers.insert(0, layer.to_owned());
        if crate::translation::rust_copy_closure_parameter_rules::test_rust_source_reference_lifetime_layer(layer){origin.source_lifetimes.insert(0,None);}
        if crate::translation::rust_copy_closure_parameter_rules::test_rust_source_reference_lifetime_layer(layer){origin=self.capture_mutable_record_value_loans(origin,node.span)?;}
        self.mark(node, origin);
        Ok(())
    }

    pub(super) fn dereference(
        &mut self,
        node: &SExpr,
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        if self.finish_owned_box_dereference(node, path, span)? {
            return Ok(());
        }
        let mut origin = self.origin(node, path);
        if !self.collecting && origin.array_source_model.is_some() && origin.wrappers.len() == 1 {
            return Err(unsupported(
                "array dereference",
                "moving an owned vector or an unsized slice needs a separate source ownership proof",
                span,
            ));
        }
        let layer = origin.wrappers.first().map_or("", String::as_str);
        if !self.collecting && (!origin.proven || !accept_rust_immutable_indirection_layer(layer)) {
            return Err(unsupported(
                "dereference",
                "the operand must have a proven immutable reference or standard Box source type",
                span,
            ));
        }
        let lifetime=crate::translation::rust_copy_closure_parameter_rules::test_rust_source_reference_lifetime_layer(layer);
        if !origin.wrappers.is_empty() {
            origin.wrappers.remove(0);
        }
        if lifetime {
            origin.source_borrowed_place = true;
        }
        if lifetime && !origin.source_lifetimes.is_empty() {
            origin.source_lifetimes.remove(0);
        }
        self.validate_copy_closure_value_borrows(&origin, span)?;
        origin = self.finish_standard_sum_field_dereference(origin, span)?;
        if origin.wrappers.is_empty() && origin.direct_copy_value() {
            origin = super::copy_closure_lifetimes::carry_value_borrows(origin.clone(), &origin);
        }
        self.mark(node, origin);
        Ok(())
    }

    pub(super) fn bind(
        &mut self,
        name: &str,
        node: &SExpr,
        path: &[String],
        declared: Option<&SourceOrigin>,
    ) -> Result<()> {
        let actual = self.origin(node, path);
        if !actual.source_box_layers.is_empty() {
            self.reserve_owned_container_copies(&[&actual])?;
        }
        self.pointer_integer_argument(&actual, &SourceOrigin::value(), node.span)?;
        if self.shared_array_handles && !self.collecting && matches!(actual.ty, Some(Type::Array {..}))
            && !crate::translation::rust_shared_array_handle_rules::accept_rust_shared_array_handle_transfer(
                actual.array_source_model.unwrap_or(""), actual.array_source_model.unwrap_or(""),
                f64::from(u32::try_from(actual.wrappers.len()).unwrap_or(u32::MAX)), f64::from(u32::try_from(actual.wrappers.len()).unwrap_or(u32::MAX)),
            ) {
            return Err(unsupported("Rust shared array handle source", "a binding requires an actual owned shared handle, not its vector payload", node.span));
        }
        if !self.collecting && actual.array_source_model.is_some() && actual.uncertain {
            return Err(unsupported(
                "source array join",
                "every array alternative needs the same proven source type and ownership",
                node.span,
            ));
        }
        if !self.collecting
            && actual.array_source_model == Some("owned vector")
            && actual.wrappers.is_empty()
            && !matches!(node.node, SNode::Array { .. })
            && actual.fresh_array_value.is_none()
            && !self.available_owned_binding(&actual)
            && !self.owned_vector_produced_value(&actual)
            && !self.borrowed_vector_produced_value(&actual)
        {
            return Err(unsupported(
                "owned vector binding",
                "moving a local vector needs an explicit consumption proof",
                node.span,
            ));
        }
        if let Some(declared) = declared {
            self.argument(&actual, declared, node.span)?;
        }
        if self.owned_binding_value(&actual) {
            self.consume_owned_binding(&actual, node.span)?;
        }
        let mut binding = declared.cloned().unwrap_or_else(|| actual.clone());
        binding.source_box_layers = actual.source_box_layers.clone();
        if !actual.wrappers.is_empty() {
            binding.source_binding_identity = actual.source_binding_identity.clone();
            binding.source_loan_binding_identities = actual.source_loan_binding_identities.clone();
        }
        binding.source_extending_temporary_bindings.clear();
        binding.source_record_state_collecting_site = None;
        binding.source_place_binding_identity = None;
        binding.source_place_loan_binding_identities.clear();
        binding.fresh_array_value = None;
        binding.fresh_vector_result_contract = false;
        let binding = self.register_owned_binding(binding);
        let binding = super::copy_closure_lifetimes::carry_value_borrows(binding, &actual);
        let binding = self.assign_copy_closure_lexical_binding(binding);
        self.scope.insert(name.to_owned(), binding);
        Ok(())
    }

    pub(super) fn pointer_integer_argument(
        &self,
        actual: &SourceOrigin,
        expected: &SourceOrigin,
        span: Option<Span>,
    ) -> Result<()> {
        if self.collecting || actual.diverging {
            return Ok(());
        }
        if let Some(error) = &actual.callable_source_owner_error {
            return Err(unsupported("Rust callable source owner", error, span));
        }
        let error = if actual.integer_source_owner_conflict {
            read_rust_pointer_integer_owner_error("usize", "u64")
        } else {
            read_rust_pointer_integer_owner_error(
                &actual.pointer_integer_owner(),
                &expected.pointer_integer_owner(),
            )
        };
        if error.is_empty() {
            Ok(())
        } else {
            Err(unsupported("Rust integer source owner", &error, span))
        }
    }
    pub(super) fn pointer_integer_expression(
        &mut self,
        node: &SExpr,
        path: &[String],
    ) -> Result<()> {
        let origin = self.origin(node, path);
        self.pointer_integer_argument(&origin, &SourceOrigin::value(), node.span)?;
        self.mark(node, origin);
        Ok(())
    }
    pub(super) fn argument(
        &self,
        actual: &SourceOrigin,
        expected: &SourceOrigin,
        span: Option<Span>,
    ) -> Result<()> {
        self.argument_with_transfer(actual, expected, span, false)
    }
    pub(super) fn argument_with_transfer(
        &self,
        actual: &SourceOrigin,
        expected: &SourceOrigin,
        span: Option<Span>,
        transferred: bool,
    ) -> Result<()> {
        if self.collecting {
            return Ok(());
        }
        if actual.diverging {
            return Ok(());
        }
        let static_region=crate::translation::rust_borrowed_vector_rules::read_rust_borrowed_vector_static_reference_error(self.borrowed_vector_source_enabled,&expected.wrappers,&expected.source_lifetimes.iter().map(|value|value.clone().unwrap_or_default()).collect::<Vec<_>>());
        if !static_region.is_empty() {
            return Err(unsupported(
                "Rust borrowed vector source",
                &static_region,
                span,
            ));
        }
        self.validate_borrowed_vector_argument(actual, expected, span)?;
        if crate::translation::rust_owned_recursive_datatype_rules::accept_rust_owned_argument_read_check(actual.wrappers.len() as f64,transferred) {
            self.check_owned_origin(actual, span)?;
        }
        self.pointer_integer_argument(actual, expected, span)?;
        let callable_error =
            crate::translation::rust_callable_owner_rules::read_rust_callable_owner_error(
                &callable_owners::descriptor(actual),
                &callable_owners::descriptor(expected),
            );
        if !callable_error.is_empty() {
            return Err(unsupported(
                "Rust callable source owner",
                &callable_error,
                span,
            ));
        }
        let pointer_error = crate::translation::rust_function_pointer_rules::read_rust_function_pointer_coercion_error(
            matches!(expected.ty, Some(Type::Function { .. })) && expected.copy_evidence.is_some(),
            actual.ty.as_ref().map_or("", Type::kind), actual.copy_closure_identity.is_some(),
            actual.copy_closure_capture_count.map_or(-1.0, |count| serde_json::Number::from(count).as_f64().expect("actual capture count")), actual.proven,
        );
        if !pointer_error.is_empty() {
            return Err(unsupported(
                "Rust function pointer coercion",
                pointer_error,
                span,
            ));
        }
        if self.shared_array_handles && (matches!(actual.ty, Some(Type::Array {..})) || matches!(expected.ty, Some(Type::Array {..})))
            && !crate::translation::rust_shared_array_handle_rules::accept_rust_shared_array_handle_transfer(
                actual.array_source_model.unwrap_or(""), expected.array_source_model.unwrap_or(""),
                f64::from(u32::try_from(actual.wrappers.len()).unwrap_or(u32::MAX)), f64::from(u32::try_from(expected.wrappers.len()).unwrap_or(u32::MAX)),
            ) {
            return Err(unsupported("Rust shared array handle source", "call and return ownership must preserve actual shared handles without borrowed payload coercion", span));
        }
        if !actual.proven
            && !actual.uncertain
            && actual.wrappers.is_empty()
            && expected.wrappers.is_empty()
        {
            return Ok(());
        }
        if !actual.proven || !expected.proven {
            return Err(unsupported(
                "source indirection",
                "the source ownership of this value cannot be proved",
                span,
            ));
        }
        let actual_model = actual.array_source_model;
        let expected_model = expected.array_source_model;
        let actual = &actual.wrappers;
        let want = &expected.wrappers;
        let indirected = !actual.is_empty() || !want.is_empty();
        let exact = actual == want;
        let borrowed = actual.first().is_some_and(|layer| layer == "reference")
            && want.first().is_some_and(|layer| layer == "reference");
        if let (Some(actual_model), Some(expected_model)) = (actual_model, expected_model)
            && !accept_rust_source_array_coercion(actual_model, expected_model, borrowed) && !crate::translation::rust_byte_string_expression_rules::accept_rust_static_byte_array_coercion(&ByteStringText::from_text(actual_model),&ByteStringText::from_text(expected_model),borrowed)
        {
            return Err(unsupported(
                "source array coercion",
                "an immutable slice cannot become a reference to an owned vector",
                span,
            ));
        }
        let coerced = actual.len().checked_sub(want.len()).is_some_and(|removed| {
            removed > 0
                && actual
                    .iter()
                    .skip(1)
                    .take(removed)
                    .all(|layer| accept_rust_source_box_coercion_layer(layer, true))
                && actual
                    .iter()
                    .take(1)
                    .chain(actual.iter().skip(removed + 1))
                    .eq(want.iter())
        });
        if !accept_rust_source_indirection_argument((indirected, exact, borrowed, coerced)) {
            return Err(unsupported(
                "source indirection",
                "source ownership requires an explicit dereference or matching immutable reference or Box type",
                span,
            ));
        }
        Ok(())
    }

    pub(super) fn direct(&self, node: &SExpr, path: &[String], span: Option<Span>) -> Result<()> {
        self.argument(&self.origin(node, path), &SourceOrigin::value(), span)
    }

    pub(super) fn conversion(&mut self, node: &SExpr, path: &[String]) -> Result<()> {
        let SNode::ToString { arg } = &node.node else {
            return Ok(());
        };
        let operand = self.origin(arg, path);
        self.check_owned_origin(&operand, node.span)?;
        if operand.ty == Some(super::STRING) {
            self.consume_owned_binding(&operand, node.span)?;
        }
        let form = read_rust_source_expression_origin(node.node.kind(), "from");
        if !self.collecting
            && form == "unboxed-conversion"
            && operand.wrappers.iter().any(|layer| layer == "box")
        {
            self.mark(node, SourceOrigin::uncertain());
            return Ok(());
        }
        if !self.collecting {
            let error=crate::translation::rust_owned_binding_rules::read_rust_owned_string_construction_error(operand.ty.as_ref().map_or("",Type::kind),operand.text_source_model.unwrap_or(""),operand.wrappers.len() as f64,operand.proven,operand.wrappers.iter().all(|layer|layer=="reference"));
            if !error.is_empty() {
                return Err(unsupported("Rust string construction", error, node.span));
            }
        }
        let result = if operand.proven {
            SourceOrigin::text_value()
        } else if operand.uncertain {
            SourceOrigin::uncertain()
        } else {
            SourceOrigin::unknown()
        };
        self.mark_owned_string_constructor(node, result);
        Ok(())
    }

    pub(super) fn call(
        &mut self,
        segments: &[String],
        args: &[SExpr],
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        self.call_with_fresh_vector_arguments(segments, args, path, span, false)
    }

    pub(super) fn call_with_fresh_vector_arguments(
        &mut self,
        segments: &[String],
        args: &[SExpr],
        path: &[String],
        span: Option<Span>,
        ordinary: bool,
    ) -> Result<()> {
        if let Some(span) = span {
            self.fresh_calls.insert(span);
        }
        if self.collecting {
            return Ok(());
        }
        let (expected, result) = match self
            .lookup_value_owner(segments, path)
            .map(|entry| entry.declaration)
        {
            Some(SourceDeclaration::Function { parameters, result }) => (parameters, Some(result)),
            Some(SourceDeclaration::Constructor { fields, .. }) => (fields, None),
            _ => return Ok(()),
        };
        if args.len() != expected.len() {
            return Ok(());
        }
        if let Some(result) = &result {
            self.validate_borrowed_vector_call_regions(args, path, &expected, result, span)?;
        }
        // References can coerce through Box; owned Box cannot become its owned
        // inner value until the source explicitly dereferences it.
        for (argument, expected) in args.iter().zip(&expected) {
            let element = match expected.ty.as_ref() {
                Some(Type::Array { element, .. }) => element.kind(),
                _ => "",
            };
            let mut actual = self.origin(argument, path);
            let transferred = argument
                .span
                .is_some_and(|place| self.owned_argument_transfers.contains(&place));
            if !transferred {
                self.check_owned_origin(&actual, span)?;
            }
            let consumable = transferred
                || self.available_owned_binding(&actual)
                || self.owned_vector_produced_value(&actual)
                || self.borrowed_vector_produced_value(&actual);
            let fresh = expected.array_element_position.is_none()
                && expected.array_source_model == Some("owned vector")
                && expected.wrappers.is_empty()
                && accept_rust_fresh_owned_vector_argument(
                    ordinary,
                    "owned vector",
                    0.0,
                    element,
                    self.fresh_vector_argument(argument, path, true),
                );
            if expected.array_source_model == Some("owned vector")
                && expected.wrappers.is_empty()
                && !fresh
                && !consumable
            {
                return Err(unsupported(
                    "owned vector argument",
                    "passing a vector by value needs an explicit consumption proof",
                    span,
                ));
            }

            if fresh {
                actual.proven = true;
                actual.uncertain = false;
                actual.wrappers.clear();
                actual.array_source_model = Some("owned vector");
            }
            self.argument_with_transfer(&actual, expected, span, transferred)?;
            if !transferred && self.owned_binding_value(expected) {
                self.consume_owned_binding(&actual, span)?;
            }
        }
        Ok(())
    }
}
