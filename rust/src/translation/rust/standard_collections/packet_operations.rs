impl RustParser {
    fn decode_collection_scalar(
        &mut self,
        library: &dyn CollectionSourceDeclarations,
        value: SExpr,
        ty: &Type,
        domain: &str,
        result: &Type,
        location: Option<Span>,
    ) -> Result<SExpr> {
        self.match_collection_packet(library.program(), ty, value, location, |_, tag, fields| {
            Ok(match (domain, tag) {
                ("String", "rustCollectionString") => {
                    let decoder = callable(
                        library.program(),
                        "decodeRustStandardCollectionText",
                        1,
                        location,
                    )?;
                    apply(
                        library_path(&decoder.full_name),
                        vec![required_field(&fields, "units", location)?.0.clone()],
                        location,
                    )
                }
                ("bool", "rustCollectionBoolean") => {
                    required_field(&fields, "value", location)?.0.clone()
                }
                (_, "rustCollectionInteger") if matches!(result, Type::Fixed { .. }) => cast(
                    required_field(&fields, "value", location)?.0.clone(),
                    result.clone(),
                    location,
                ),
                _ => invalid(
                    library,
                    "collection result lost its actual scalar variant",
                    location,
                ),
            })
        })
    }

    pub(in super::super) fn standard_collection_clone_call(
        &mut self,
        segments: &[String],
        arguments: &[SExpr],
        module: &[String],
        location: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if self.standard_collections.is_none() || segments.last().is_none_or(|name| name != "clone")
        {
            return Ok(None);
        }
        let source_owner = &segments[..segments.len() - 1];
        if !self.standard_collection_trait(
            source_owner,
            module,
            ["std::clone::Clone", "core::clone::Clone"],
        ) {
            return Ok(None);
        }
        let Some(receiver) = arguments.first() else {
            return Ok(None);
        };
        let actual = self.origins.origin(receiver, module);
        if self.standard_collection_origin(&actual).is_none() {
            return Ok(None);
        }
        if arguments.len() != 1 || actual.wrappers.as_slice() != ["reference"] {
            return Err(fail(
                "standard collection Clone requires its actual single shared receiver",
                location,
            ));
        }
        self.lower_standard_collection_method(receiver, "clone", module, location, false, None)
    }

    pub(in super::super) fn standard_collection_method(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        module: &[String],
    ) -> Result<Option<SExpr>> {
        let actual = self.origins.origin(receiver, module);
        if self.standard_collection_origin(&actual).is_none() {
            return Ok(None);
        }
        if matches!(
            method.value.as_str(),
            "iter" | "keys" | "values" | "into_iter"
        ) && self.ordered_iteration_head.is_some()
        {
            self.cursor
                .expect("(", Some("ordered collection iterator"))?;
            let end = self
                .cursor
                .expect(")", Some("ordered collection iterator"))?;
            let (mut node, item) = self
                .standard_collection_iteration(receiver, &method.value, module)?
                .ok_or_else(|| {
                    fail(
                        "iteration requires its actual standard BTree owner",
                        Some(method.span()),
                    )
                })?;
            let origin = self.origins.origin(&node, module);
            node.span = receiver.span.map(|start| Span::new(start.start, end.end));
            self.origins.mark(&node, origin);
            self.ordered_iteration_head
                .as_mut()
                .expect("actual ordered iteration head reader")
                .push(super::OrderedIterationHead {
                    expression: node.clone(),
                    item,
                    owned_vector: None,
                    end_token: self.cursor.index,
                });
            return Ok(Some(node));
        }
        if matches!(method.value.as_str(), "eq" | "ne") {
            self.cursor
                .expect("(", Some("standard collection PartialEq method"))?;
            let argument = self.expr(module, Options::default())?;
            self.cursor.eat(",");
            self.cursor
                .expect(")", Some("standard collection PartialEq method"))?;
            let operation = if method.value == "eq" {
                BinaryOp::Eq
            } else {
                BinaryOp::Ne
            };
            return self.standard_collection_comparison(
                receiver,
                &argument,
                operation,
                true,
                module,
                Some(method.span()),
            );
        }
        self.lower_standard_collection_method(
            receiver,
            &method.value,
            module,
            Some(method.span()),
            true,
            None,
        )
    }

    pub(in super::super) fn standard_collection_iteration(
        &mut self,
        receiver: &SExpr,
        method: &str,
        module: &[String],
    ) -> Result<Option<(SExpr, SourceOrigin)>> {
        let actual = self.origins.origin(receiver, module);
        let Some(carrier) = self.standard_collection_origin(&actual) else {
            return Ok(None);
        };
        let location = receiver.span;
        if !test_rust_standard_collection_ordered(&Text::from_text(&carrier.owner.path.join("::")))
        {
            return Err(fail(
                "ordered iteration requires its actual standard BTree owner",
                location,
            ));
        }
        let carrier = carrier.clone();
        let owned = method == "into_iter";
        let loan = if owned {
            None
        } else {
            if !(actual.wrappers.is_empty() || actual.wrappers.as_slice() == ["reference"])
                || !matches!(
                    (carrier.arguments.len(), method),
                    (1, "iter") | (2, "iter" | "keys" | "values")
                )
            {
                return Err(fail(
                    "a shared iterator needs its actual standard BTree method and receiver",
                    location,
                ));
            }
            Some(self.borrow_ordered_collection_receiver(receiver, actual.clone(), location)?)
        };
        let regions = loan.as_ref().map_or_else(Vec::new, |origin| {
            origin.source_value_borrow_bindings.clone()
        });
        let item_origin = |origin: &SourceOrigin| {
            let mut selected = origin.clone();
            if !owned {
                selected.wrappers = vec!["reference".to_owned()];
                selected.source_lifetimes =
                    vec![actual.source_lifetimes.first().cloned().flatten()];
                selected.source_borrowed_place = true;
                selected.copy_evidence = Some(());
                selected.source_owned_result_proof = false;
                selected.source_value_borrow_bindings = regions.clone();
            }
            selected
        };
        let (collection_module, item) = match (carrier.arguments.as_slice(), method) {
            ([key], _) => ("btree_set", item_origin(key)),
            ([key, _], "keys") => ("btree_map", item_origin(key)),
            ([_, value], "values") => ("btree_map", item_origin(value)),
            ([key, value], _) => {
                let key = item_origin(key);
                let value = item_origin(value);
                let (Some(key_type), Some(value_type)) = (&key.ty, &value.ty) else {
                    return Err(fail(
                        "the ordered map needs its actual key and value types",
                        location,
                    ));
                };
                let mut pair = SourceOrigin::with_type(
                    Type::Tuple {
                        elements: vec![key_type.clone(), value_type.clone()],
                    },
                    module,
                    Vec::new(),
                );
                pair.copy_evidence = key.copy_evidence.and(value.copy_evidence);
                pair.fields = Some(vec![key, value]);
                pair.source_value_borrow_bindings = regions.clone();
                ("btree_map", pair)
            }
            _ => {
                return Err(fail(
                    "the ordered receiver lost its actual scalar domains",
                    location,
                ));
            }
        };
        let element = item
            .ty
            .clone()
            .ok_or_else(|| fail("the ordered receiver lost its actual item type", location))?;
        let mut iteration = SourceOrigin::with_type(
            Type::Array {
                element: Box::new(element),
                reference: false,
            },
            module,
            Vec::new(),
        );
        iteration.source_owner = Some(SourceOwner {
            kind: "library".to_owned(),
            path: [
                "std",
                "collections",
                collection_module,
                match method {
                    "into_iter" => "IntoIter",
                    "keys" => "Keys",
                    "values" => "Values",
                    _ => "Iter",
                },
            ]
            .map(str::to_owned)
            .to_vec(),
            arguments: carrier.owner.arguments.clone(),
        });
        iteration.array_source_model = None;
        iteration.copy_evidence = None;
        iteration.source_borrowed_place = !owned;
        iteration.source_value_borrow_bindings = regions;
        let lowered = self.lower_standard_collection_method(
            receiver,
            method,
            module,
            location,
            false,
            Some(iteration),
        )?;
        Ok(lowered.map(|expression| (expression, item)))
    }

    fn borrow_ordered_collection_receiver(
        &mut self,
        receiver: &SExpr,
        mut actual: SourceOrigin,
        location: Option<Span>,
    ) -> Result<SourceOrigin> {
        self.origins.check_owned_origin(&actual, location)?;
        let identity = actual
            .source_place_binding_identity
            .as_ref()
            .or(actual.source_binding_identity.as_ref());
        let root = identity.map(|id| {
            let ultimate =
                super::super::owned_record_policy::fields::read_rust_owned_record_ultimate_root(id);
            if ultimate.is_empty() {
                id.clone()
            } else {
                ultimate
            }
        });
        let owning = super::super::owned_record_loops::place(self, receiver)
            .map(|place| place.slot.origin.clone())
            .or_else(|| {
                self.origins
                    .scope
                    .values()
                    .find(|origin| {
                        root.is_some()
                            && origin.source_binding_identity == root
                            && origin.wrappers.is_empty()
                    })
                    .cloned()
            });
        if let Some(owning) = owning {
            self.origins
                .register_mutable_record_loan_root(&owning, location, false)?;
        }
        let regions = super::super::borrowed_iterators::argument_regions(&actual)?;
        if !self.origins.collecting && regions.is_empty() {
            return Err(fail(
                "a shared iterator needs actual live receiver-derived source regions",
                location,
            ));
        }
        actual.wrappers = vec!["reference".to_owned()];
        actual.source_borrowed_place = true;
        actual.copy_evidence = Some(());
        actual.source_value_borrow_bindings = regions;
        self.origins
            .capture_mutable_record_value_loans(actual, location)
    }
}

include!("packet_method_lowering.rs");
