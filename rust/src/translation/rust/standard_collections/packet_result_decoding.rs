impl RustParser {
    fn read_ordered_collection_target(
        &mut self,
        element: &SourceOrigin,
        location: Option<Span>,
    ) -> Result<SourceOrigin> {
        self.cursor
            .expect(".", Some("immediate ordered collection"))?;
        self.cursor
            .expect("collect", Some("immediate ordered collection"))?;
        self.cursor
            .expect("::", Some("explicit ordered Vec target"))?;
        self.cursor
            .expect("<", Some("explicit ordered Vec target"))?;
        let vector = self.parse_type()?;
        let mut origin = self.origins.current_type.clone();
        self.split_type_closing_token();
        self.cursor
            .expect(">", Some("explicit ordered Vec target"))?;
        self.cursor
            .expect("(", Some("immediate ordered collection"))?;
        self.cursor
            .expect(")", Some("immediate ordered collection"))?;
        let expected_owner = element
            .expanded_source_owner
            .as_ref()
            .or(element.source_owner.as_ref());
        let Type::Array {
            element: vector_element,
            reference: false,
        } = vector
        else {
            return Err(fail(
                "ordered collection needs its actual owned Vec target",
                location,
            ));
        };
        let owner = origin
            .expanded_source_owner
            .as_ref()
            .or(origin.source_owner.as_ref())
            .filter(|owner| {
                owner.kind == "library"
                    && matches!(
                        owner.path.join("::").as_str(),
                        "std::vec::Vec" | "alloc::vec::Vec"
                    )
            });
        if !origin.wrappers.is_empty()
            || element.ty.as_ref() != Some(vector_element.as_ref())
            || !owner.is_some_and(|owner| {
                owner.arguments.len() == 1 && owner.arguments.first() == expected_owner
            })
        {
            return Err(fail(
                "ordered collection needs its actual same-domain owned Vec target",
                location,
            ));
        }
        origin.array_source_model = Some("owned vector");
        origin.array_element_copy_evidence = element.copy_evidence.is_some();
        Ok(origin)
    }

    fn decode_ordered_collection_entry(
        &mut self,
        library: &dyn CollectionSourceDeclarations,
        carrier: &SourceCarrier,
        input: SExpr,
        packet_element: &Type,
        output: &Type,
        selected_index: usize,
        location: Option<Span>,
    ) -> Result<SExpr> {
        let Type::Record { fields, .. } = packet_element else {
            return Err(fail(
                "ordered entries need their actual checked record layout",
                location,
            ));
        };
        let fields = fields
            .iter()
            .map(|field| {
                let ty = if field.optional {
                    crate::translation::javascript_constructor_presence::field_value_type(&field.ty)
                        .ok_or_else(|| {
                            fail(
                                "an optional entry field lost its actual presence carrier",
                                location,
                            )
                        })?
                } else {
                    &field.ty
                };
                let expression = SExpr::new(
                    SNode::Field {
                        object: Box::new(input.clone()),
                        field: field.name.clone(),
                    },
                    location,
                );
                Ok((field.name.clone(), (expression, ty.clone())))
            })
            .collect::<Result<Fields>>()?;
        let mut decode_field = |field: &str, index: usize, output: &Type| {
            let (expression, ty) = required_field(&fields, field, location)?;
            let (selected, scalar_type) = if let Type::Option { element, .. } = ty {
                (
                    SExpr::new(
                        SNode::Coalesce {
                            left: Box::new(expression.clone()),
                            right: Box::new(invalid(
                                library,
                                "the ordered collection entry lost its selected scalar",
                                location,
                            )),
                        },
                        location,
                    ),
                    element.as_ref(),
                )
            } else {
                (expression.clone(), ty)
            };
            self.decode_collection_scalar(
                library,
                selected,
                scalar_type,
                &carrier.domains[index],
                output,
                location,
            )
        };
        match (carrier.domains.as_slice(), output) {
            ([_], _) => decode_field("key", 0, output),
            ([_, _], Type::Tuple { elements }) => {
                let [key, value] = elements.as_slice() else {
                    return Err(fail(
                        "an ordered map needs its actual key/value pair",
                        location,
                    ));
                };
                Ok(SExpr::new(
                    SNode::Tuple {
                        items: vec![
                            decode_field("key", 0, key)?,
                            decode_field("value", 1, value)?,
                        ],
                        frozen: false,
                        product_model: None,
                    },
                    location,
                ))
            }
            ([_, _], _) => {
                let field = if selected_index == 0 { "key" } else { "value" };
                decode_field(field, selected_index, output)
            }
            _ => Err(fail(
                "the ordered entry lost its actual collection item type",
                location,
            )),
        }
    }

    fn decode_collection_result(
        &mut self,
        library: &dyn CollectionSourceDeclarations,
        carrier: &SourceCarrier,
        ty: &Type,
        value: SExpr,
        result: &SourceOrigin,
        kind: &str,
        selected_index: usize,
        location: Option<Span>,
    ) -> Result<SExpr> {
        let result_type = result.ty.as_ref().ok_or_else(|| {
            fail(
                "the collection result has no concrete source type",
                location,
            )
        })?;
        match kind {
            "bool" | "usize" => {
                self.decode_collection_scalar(library, value, ty, kind, result_type, location)
            }
            "unit" => {
                self.match_collection_packet(library.program(), ty, value, location, |_, tag, _| {
                    Ok(if tag == "rustCollectionUnit" {
                        SExpr::new(SNode::Unit, location)
                    } else {
                        invalid(
                            library,
                            "the collection clear needs its actual Unit result",
                            location,
                        )
                    })
                })
            }
            "owned ordered values" | "owned ordered entries" | "borrowed ordered entries" => self
                .match_collection_packet(
                    library.program(),
                    ty,
                    value,
                    location,
                    |parser, tag, fields| {
                        let field = match (kind, tag) {
                            ("owned ordered values", "rustCollectionOrderedValues") => "values",
                            (
                                "owned ordered entries" | "borrowed ordered entries",
                                "rustCollectionOrderedEntries",
                            ) => "entries",
                            _ => "",
                        };
                        if field.is_empty() {
                            return Ok(invalid(
                                library,
                                "the ordered collection needs its actual complete sequence",
                                location,
                            ));
                        }
                        let (values, values_type) = required_field(&fields, field, location)?;
                        let Type::Array {
                            element: packet_element,
                            ..
                        } = values_type
                        else {
                            return Err(fail(
                                "the ordered collection lost its actual sequence type",
                                location,
                            ));
                        };
                        let Type::Array { element, .. } = result_type else {
                            return Err(fail(
                                "the ordered result needs its actual owned Vec target",
                                location,
                            ));
                        };
                        let accumulator = parser.fresh_product_binding();
                        let item = parser.fresh_product_binding();
                        let input = name(vec![item.clone()], location);
                        let decoded = if field == "values" {
                            parser.decode_collection_scalar(
                                library,
                                input,
                                packet_element,
                                &carrier.domains[selected_index],
                                element,
                                location,
                            )?
                        } else {
                            parser.decode_ordered_collection_entry(
                                library,
                                carrier,
                                input,
                                packet_element,
                                element,
                                selected_index,
                                location,
                            )?
                        };
                        let body = SExpr::new(
                            SNode::Array {
                                element: Some(element.as_ref().clone()),
                                items: vec![
                                    SArrayItem::new(
                                        name(vec![accumulator.clone()], location),
                                        true,
                                    ),
                                    SArrayItem::new(decoded, false),
                                ],
                            },
                            location,
                        );
                        let reduce = SExpr::new(
                            SNode::Lambda {
                                params: vec![
                                    parameter(accumulator, result_type.clone(), location),
                                    parameter(item, source_type(packet_element), location),
                                ],
                                ret: Some(result_type.clone()),
                                captures: Vec::new(),
                                assigned_captures: None,
                                body: Box::new(body),
                            },
                            location,
                        );
                        let initial = SExpr::new(
                            SNode::Array {
                                element: Some(element.as_ref().clone()),
                                items: Vec::new(),
                            },
                            location,
                        );
                        Ok(SExpr::new(
                            SNode::Method {
                                object: Box::new(values.clone()),
                                op: "reduce".to_owned(),
                                args: vec![Some(reduce), Some(initial)],
                            },
                            location,
                        ))
                    },
                ),
            "owned collection" => self.match_collection_packet(
                library.program(),
                ty,
                value,
                location,
                |_, tag, fields| {
                    if tag != "rustStandardCollectionState" {
                        return Ok(invalid(
                            library,
                            "the cloned collection needs its actual state result",
                            location,
                        ));
                    }
                    let Type::Data { name } = ty else {
                        return Err(fail(
                            "the cloned collection state needs its complete datatype",
                            location,
                        ));
                    };
                    let Some(Decl::Data(data)) = library.program().declaration(name) else {
                        return Err(fail(
                            "the cloned collection state has no actual declaration",
                            location,
                        ));
                    };
                    let ctor = data
                        .ctors
                        .iter()
                        .find(|constructor| constructor.name == tag)
                        .ok_or_else(|| {
                            fail("the cloned state constructor is unavailable", location)
                        })?;
                    let arguments = ctor
                        .fields
                        .iter()
                        .map(|field| {
                            required_field(&fields, &field.name, location)
                                .map(|value| value.0.clone())
                        })
                        .collect::<Result<_>>()?;
                    let (state, _) = library.state_and_ordering_names();
                    let projected =
                        apply(library_path(&format!("{state}.{tag}")), arguments, location);
                    Ok(apply(carrier_path(carrier), vec![projected], location))
                },
            ),
            "borrowed Option" | "owned Option" => {
                let sum = result.standard_sum_source.as_deref().ok_or_else(|| {
                    fail(
                        "collection get retains its actual standard Option owner",
                        location,
                    )
                })?;
                let some = sum.constructor_paths.get("Some").ok_or_else(|| {
                    fail(
                        "collection get requires the actual Some constructor",
                        location,
                    )
                })?;
                let none = sum.constructor_paths.get("None").ok_or_else(|| {
                    fail(
                        "collection get requires the actual None constructor",
                        location,
                    )
                })?;
                let index = usize::from(carrier.domains.len() == 2);
                let domain = &carrier.domains[index];
                let scalar = carrier.arguments[index]
                    .ty
                    .as_ref()
                    .expect("actual scalar argument");
                self.match_collection_packet(
                    library.program(),
                    ty,
                    value,
                    location,
                    |parser, tag, fields| {
                        if tag != "rustCollectionOption" {
                            return Ok(invalid(
                                library,
                                "collection get lost its actual Option result",
                                location,
                            ));
                        }
                        let (optional, optional_type) = required_field(&fields, "value", location)?;
                        let Type::Option { element, .. } = optional_type else {
                            return Err(fail(
                                "collection Option payload has no actual presence representation",
                                location,
                            ));
                        };
                        let selected = SExpr::new(
                            SNode::Coalesce {
                                left: Box::new(optional.clone()),
                                right: Box::new(invalid(
                                    library,
                                    "present collection Option lost its actual payload",
                                    location,
                                )),
                            },
                            location,
                        );
                        let decoded = parser.decode_collection_scalar(
                            library, selected, element, domain, scalar, location,
                        )?;
                        Ok(SExpr::new(
                            SNode::If {
                                cond: Box::new(
                                    required_field(&fields, "present", location)?.0.clone(),
                                ),
                                then: Box::new(apply(some.clone(), vec![decoded], location)),
                                otherwise: Box::new(name(none.clone(), location)),
                            },
                            location,
                        ))
                    },
                )
            }
            _ => Err(fail(
                "collection decoding requires its actual source result interpretation",
                location,
            )),
        }
    }
}
