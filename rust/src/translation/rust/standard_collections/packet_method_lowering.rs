impl RustParser {
    fn lower_standard_collection_method(
        &mut self,
        receiver: &SExpr,
        method: &str,
        module: &[String],
        location: Option<Span>,
        read_arguments: bool,
        iteration: Option<SourceOrigin>,
    ) -> Result<Option<SExpr>> {
        let actual = self.origins.origin(receiver, module);
        let Some(carrier) = self.standard_collection_origin(&actual).cloned() else {
            return Ok(None);
        };
        let owner = carrier.owner.path.join("::");
        let owner_text = Text::from_text(&owner);
        let method_text = Text::from_text(method);
        let arity = read_rust_standard_collection_method_arity(&owner_text, &method_text);
        let result_kind =
            read_rust_standard_collection_method_result(&owner_text, &method_text).to_string();
        if matches!(method, "new" | "default") {
            return Err(fail(
                "there is no admitted source method for this collection owner",
                location,
            ));
        }
        let arity = match arity {
            0.0 => 0,
            1.0 => 1,
            2.0 => 2,
            _ => {
                return Err(fail(
                    "there is no admitted source method for this collection owner",
                    location,
                ));
            }
        };
        let mutation = test_rust_standard_collection_mutation(&owner_text, &method_text);
        if mutation && !actual.wrappers.is_empty() {
            return Err(fail(
                "a shared collection reference cannot authorize mutation",
                location,
            ));
        }
        if actual.wrappers.len() > 1 && method == "clone" {
            return Err(fail(
                "nested reference Clone dispatch needs its actual reference-result interpretation",
                location,
            ));
        }
        self.origins.check_owned_name(receiver, module)?;
        if read_arguments {
            self.cursor
                .expect("(", Some("standard collection method"))?;
        }
        let mut arguments = Vec::new();
        let mut argument_facts = Vec::new();
        for index in 0..arity {
            if index != 0 {
                self.cursor
                    .expect(",", Some("collection method arguments"))?;
            }
            let expected = &carrier.arguments[index];
            let mut argument = self
                .with_standard_sum_source_context(Some(expected.clone()), |parser| {
                    parser.expr(module, Options::default())
                })?;
            let source_borrowed = method != "insert";
            let mut source = self.origins.origin(&argument, module);
            infer_integer_argument(&mut argument, expected, &mut source);
            let direct = source.wrappers.is_empty();
            if !source_borrowed && !direct {
                return Err(fail(
                    "collection insertion requires a direct owned source value",
                    argument.span,
                ));
            }
            let borrowed = source.wrappers.as_slice() == ["reference"];
            self.origins.check_owned_origin(&source, argument.span)?;
            let expected_domain = &carrier.domains[index];
            let string_slice = source_borrowed
                && expected_domain == "String"
                && source.ty.as_ref() == Some(&Type::String)
                && source.borrowed_string_slice()
                && (borrowed || matches!(&argument.node, SNode::Str { .. }));
            source.wrappers.clear();
            let domain = if string_slice {
                source.source_owner = Some(SourceOwner {
                    kind: "primitive".to_owned(),
                    path: vec!["str".to_owned()],
                    arguments: Vec::new(),
                });
                source.expanded_source_owner = None;
                "str".to_owned()
            } else {
                if source.expanded_source_owner.is_none()
                    && source.source_owner.is_none()
                    && source.ty.as_ref() == Some(&Type::String)
                    && source.text_source_model == Some("owned string")
                {
                    source.source_owner = Some(SourceOwner {
                        kind: "library".to_owned(),
                        path: ["std", "string", "String"].map(str::to_owned).to_vec(),
                        arguments: Vec::new(),
                    });
                }
                scalar_domain(&source).unwrap_or_default()
            };
            let argument_owner = source
                .expanded_source_owner
                .as_ref()
                .or(source.source_owner.as_ref())
                .filter(|owner| owner.arguments.is_empty())
                .ok_or_else(|| {
                    fail(
                        "collection arguments retain their actual closed scalar owner",
                        argument.span,
                    )
                })?;
            let native_owner = (
                Vec::new(),
                Text::from_text(&argument_owner.kind),
                argument_owner
                    .path
                    .iter()
                    .map(|segment| Text::from_text(segment))
                    .collect(),
            );
            let collecting_local = self.origins.collecting
                && matches!(&argument.node, SNode::Name { path }
                    if path.len() == 1 && self.origins.scope.contains_key(&path[0]));
            let owned_transfer = !source_borrowed
                && direct
                && domain == "String"
                && (self.origins.available_owned_binding(&source)
                    || source.source_owned_result_proof
                    || collecting_local);
            let reason = read_rust_standard_collection_argument_origin_error(
                Some((
                    true,
                    borrowed || string_slice,
                    Text::from_text(&domain),
                    owned_transfer,
                    source.proven,
                    native_owner,
                )),
                &Text::from_text(expected_domain),
                source_borrowed,
            );
            if !reason.0.is_empty() {
                return Err(fail(&reason.to_string(), argument.span));
            }
            argument_facts.push(json!({
                "domain": domain, "proven": source.proven, "available": true,
                "borrowed": borrowed || string_slice, "ownedTransfer": owned_transfer, "sourceOwner": argument_owner,
            }));
            if owned_transfer {
                self.origins
                    .prepare_owned_argument(&argument, Some(expected), module)?;
            }
            self.check_reserved_place_operand_effects(receiver, &argument)?;
            arguments.push(argument);
        }
        if !arguments.is_empty() {
            self.cursor.eat(",");
        }
        if read_arguments {
            self.cursor
                .expect(")", Some("standard collection method"))?;
        }
        let owned_iteration = result_kind == "owned ordered entries" && iteration.is_some();
        let shared_iteration = result_kind == "owned ordered values" && iteration.is_some();
        if owned_iteration {
            if !actual.wrappers.is_empty() || actual.source_borrowed_place {
                return Err(fail(
                    "ordered IntoIterator requires a direct owned BTree receiver",
                    location,
                ));
            }
            self.origins.check_owned_origin(&actual, location)?;
            if !self.origins.collecting {
                let transferred = self.origins.transfer_owned_result(receiver, module)?;
                if !self.origins.available_owned_binding(&transferred)
                    || !transferred.source_owned_result_proof
                    || !self.origins.consume_owned_binding(&transferred, location)?
                {
                    return Err(fail(
                        "ordered IntoIterator requires one actual available owned collection transfer",
                        location,
                    ));
                }
            }
        }
        let place = if mutation {
            Some(self.collection_mutable_state_place(receiver, location)?)
        } else {
            None
        };
        let payload_index = usize::from(carrier.domains.len() == 2);
        let ordered_values = result_kind == "owned ordered values" && !shared_iteration;
        if ordered_values && method == "iter" && payload_index == 1 {
            return Err(fail(
                "ordered map pair iteration requires the actual pair consumer",
                location,
            ));
        }
        let selected_index = if ordered_values || shared_iteration {
            usize::from(method == "values")
        } else {
            payload_index
        };
        let projection = if owned_iteration {
            "owned".to_owned()
        } else if shared_iteration {
            "borrowed".to_owned()
        } else if method == "get" || ordered_values {
            self.cursor
                .expect(".", Some("collection reference projection"))?;
            let selection = self
                .cursor
                .identifier(Some("collection reference projection"))?;
            if !matches!(selection.value.as_str(), "copied" | "cloned")
                || selection.value == "copied" && carrier.domains[selected_index] == "String"
            {
                return Err(fail(
                    "collection borrows require immediate copied or cloned with the actual scalar capability",
                    Some(selection.span()),
                ));
            }
            self.cursor
                .expect("(", Some("collection reference projection"))?;
            self.cursor
                .expect(")", Some("collection reference projection"))?;
            selection.value
        } else {
            String::new()
        };
        let mut result = match result_kind.as_str() {
            "owned collection" => {
                let mut origin = SourceOrigin::with_type(
                    Type::Named {
                        path: library_path(&carrier.data.name),
                        span: None,
                    },
                    module,
                    Vec::new(),
                );
                origin.source_owner = Some(carrier.owner.clone());
                origin.expanded_source_owner = origin.source_owner.clone();
                origin
            }
            "bool" => SourceOrigin::with_type(Type::Bool, module, Vec::new()),
            "unit" => SourceOrigin::with_type(Type::Unit, module, Vec::new()),
            "usize" => SourceOrigin::with_type(
                super::super::rust_fixed_type("usize").expect("usize is represented"),
                module,
                Vec::new(),
            ),
            "borrowed Option" | "owned Option" => self.instantiate_standard_sum_source(
                "std::option::Option",
                vec![carrier.arguments[payload_index].clone()],
                location,
            )?,
            "owned ordered values" if !shared_iteration => {
                self.read_ordered_collection_target(&carrier.arguments[selected_index], location)?
            }
            _ if owned_iteration || shared_iteration => {
                iteration.expect("admitted ordered consumer")
            }
            _ => {
                return Err(fail(
                    "the method requires its actual distinct result interpretation",
                    location,
                ));
            }
        };
        result.source_owned_result_proof = !(owned_iteration || shared_iteration);
        let decoded_kind = if shared_iteration {
            "borrowed ordered entries"
        } else {
            &result_kind
        };
        let library = Rc::clone(
            &self
                .standard_collections
                .as_ref()
                .expect("selected source collection profile")
                .library,
        );
        let program = library.program();
        let [packet_name, _, _] = library.callable_names();
        let packet = callable(program, packet_name, 5, location)?;
        let Type::Array { element, .. } = &packet.params[2].ty else {
            return Err(fail("the packet needs its actual array argument", location));
        };
        let argument_list = SExpr::new(
            SNode::Array {
                element: Some(source_type(element)),
                items: arguments
                    .into_iter()
                    .zip(&carrier.domains)
                    .map(|(argument, domain)| {
                        Ok(SArrayItem::new(
                            packet_leaf(program, element, domain, argument, location)?,
                            false,
                        ))
                    })
                    .collect::<Result<_>>()?,
            },
            location,
        );
        let evaluated_receiver = self.fresh_product_binding();
        let evaluated_arguments = self.fresh_product_binding();
        let facet = carrier
            .data
            .source_standard_collection_carrier
            .as_ref()
            .expect("retained collection facet");
        let clone_method = method == "clone";
        let dispatch = if clone_method {
            "core::clone::Clone"
        } else if owned_iteration {
            "core::iter::IntoIterator"
        } else {
            &owner
        };
        let suffix = format!("/{}", self.fresh_product_binding());
        let mut metadata = json!({
            "$": "rustStandardCollectionCallOrigin", "owner": owner,
            "typeIdentity": facet.concrete_identity, "receiverRoot": "", "method": method,
            "dispatchOwner": dispatch, "proven": true, "shadowed": false,
            "mutable": mutation, "available": true, "overlappingLoan": false, "arguments": argument_facts,
        });
        if owned_iteration {
            metadata["ownedReceiverTransfer"] = json!(true);
        }
        let held = self.fresh_product_binding();
        let state = name(vec![held.clone()], location);
        let state_match =
            self.match_collection_packet(
                program,
                &packet.params[0].ty,
                state.clone(),
                location,
                |parser, tag, fields| {
                    if tag != "rustStandardCollectionState" {
                        return Ok(invalid(
                            library.as_ref(),
                            "the collection carrier lost its actual state variant",
                            location,
                        ));
                    }
                    let root = required_field(&fields, "root", location)?.0.clone();
                    let mut overrides = HashMap::from([("receiverRoot", root.clone())]);
                    if clone_method {
                        let output = SExpr::new(
                            SNode::Binary {
                                op: BinaryOp::Concat,
                                left: Box::new(root),
                                right: Box::new(text(&suffix, location)),
                                rounding: None,
                            },
                            location,
                        );
                        overrides.insert(
                            "outputRoot",
                            SExpr::new(
                                SNode::Some {
                                    value: Box::new(output),
                                    ty: Type::Option {
                                        element: Box::new(Type::String),
                                        absence: "undefined".to_owned(),
                                    },
                                },
                                location,
                            ),
                        );
                    }
                    let call = encode_metadata(
                        program,
                        Some(&metadata),
                        &packet.params[3].ty,
                        0,
                        location,
                        Some(&overrides),
                    )?;
                    let expression = apply(
                        library_path(packet_name),
                        vec![
                            state.clone(),
                            text(method, location),
                            name(vec![evaluated_arguments.clone()], location),
                            call,
                            text(&projection, location),
                        ],
                        location,
                    );
                    parser.match_collection_packet(
                        program,
                        &packet.ret,
                        expression,
                        location,
                        |parser, tag, fields| match tag {
                            "rustCollectionInvalid" => Ok(library
                                .invalid(required_field(&fields, "reason", location)?.0.clone())),
                            "rustCollectionResourceLimit" => Ok(library.resource(
                                required_field(&fields, "state", location)?.0.clone(),
                                required_field(&fields, "reason", location)?.0.clone(),
                            )),
                            "rustCollectionPacketNormal" => {
                                let (values, ty) = required_field(&fields, "values", location)?;
                                let Type::Tuple { elements } = ty else {
                                    return Err(fail(
                                        "the actual packet has no result/state product",
                                        location,
                                    ));
                                };
                                let [value_type, _] = elements.as_slice() else {
                                    return Err(fail(
                                        "the actual packet has no result/state pair",
                                        location,
                                    ));
                                };
                                let value = tuple_index(values.clone(), 0, location);
                                let decoded = parser.decode_collection_result(
                                    library.as_ref(),
                                    &carrier,
                                    value_type,
                                    value,
                                    &result,
                                    decoded_kind,
                                    selected_index,
                                    location,
                                )?;
                                let writeback =
                                    required_field(&fields, "writeback", location)?.0.clone();
                                let correct_writeback = if mutation {
                                    writeback
                                } else {
                                    SExpr::new(
                                        SNode::Unary {
                                            op: UnaryOp::Not,
                                            arg: Box::new(writeback),
                                        },
                                        location,
                                    )
                                };
                                let success = if mutation {
                                    let updated = apply(
                                        carrier_path(&carrier),
                                        vec![tuple_index(values.clone(), 1, location)],
                                        location,
                                    );
                                    SExpr::new(
                                        SNode::Tuple {
                                            items: vec![decoded, updated],
                                            frozen: false,
                                            product_model: None,
                                        },
                                        location,
                                    )
                                } else {
                                    decoded
                                };
                                Ok(SExpr::new(
                                    SNode::If {
                                        cond: Box::new(correct_writeback),
                                        then: Box::new(success),
                                        otherwise: Box::new(invalid(
                                            library.as_ref(),
                                            "collection writeback must match its actual method",
                                            location,
                                        )),
                                    },
                                    location,
                                ))
                            }
                            _ => Err(fail(
                                "the projected packet returned an unsupported constructor",
                                location,
                            )),
                        },
                    )
                },
            )?;
        let expression = SExpr::new(
            SNode::Match {
                scrutinees: vec![name(vec![evaluated_receiver.clone()], location)],
                rows: vec![SRow {
                    patterns: vec![SPattern {
                        node: SPatternNode::Ctor {
                            path: carrier_path(&carrier),
                            args: vec![SPattern {
                                node: SPatternNode::Bind { name: held },
                                span: location,
                            }],
                        },
                        span: location,
                    }],
                    body: state_match,
                    span: location,
                }],
            },
            location,
        );
        let expression = SExpr::new(
            SNode::Let {
                name: evaluated_arguments,
                ty: Some(source_type(&packet.params[2].ty).into()),
                value: Box::new(argument_list),
                body: Box::new(expression),
            },
            location,
        );
        let expression = SExpr::new(
            SNode::Let {
                name: evaluated_receiver,
                ty: Some(
                    actual
                        .ty
                        .clone()
                        .ok_or_else(|| {
                            fail("the receiver needs its actual collection type", location)
                        })?
                        .into(),
                ),
                value: Box::new(receiver.clone()),
                body: Box::new(expression),
            },
            location,
        );
        if let Some(place) = place {
            let ty = Type::Tuple {
                elements: vec![
                    result.ty.clone().expect("actual method result"),
                    actual.ty.expect("actual collection receiver type"),
                ],
            };
            return self
                .owned_record_effect_collection_packet(place, ty, expression, result)
                .map(Some);
        }
        if ordered_values {
            self.origins.mark_vector_constructor(&expression, result)?;
        } else {
            self.origins.mark(&expression, result);
        }
        Ok(Some(expression))
    }
}

include!("packet_result_decoding.rs");
