impl SourceOrigins {
    /// Immutable host type inspection avoids cloning a retained metadata tree.
    pub(super) fn recorded_expression_origin(&self, node: &SExpr) -> Option<&SourceOrigin> {
        node.span.and_then(|span| self.expressions.get(&span))
    }
    pub(super) fn recorded_expression_origin_at_span(
        &self,
        span: Option<Span>,
    ) -> Option<&SourceOrigin> {
        span.and_then(|span| self.expressions.get(&span))
    }
    pub(super) fn origin(&self, node: &SExpr, path: &[String]) -> SourceOrigin {
        if let Some(origin) = node.span.and_then(|span| self.expressions.get(&span)) {
            return origin.clone();
        }
        if let SNode::Name { path: names } = &node.node
            && names.len() == 1
            && let Some(origin) = self.scope.get(&names[0])
        {
            return origin.clone();
        }
        let names = match &node.node {
            SNode::Name { path } => Some(path),
            SNode::App { func, .. } => match &func.node {
                SNode::Name { path } => Some(path),
                _ => None,
            },
            _ => None,
        };
        if let Some(entry) = names.and_then(|names| self.lookup_value_owner(names, path)) {
            match entry.declaration {
                SourceDeclaration::Function {
                    parameters,
                    mut result,
                } => {
                    result.fresh_array_value = node.span.filter(|span| {
                        crate::translation::rust_fresh_vector_call_result_rules::accept_rust_fresh_vector_result_call(
                            node.kind(), result.array_source_model.unwrap_or(""),
                            u32::try_from(result.wrappers.len()).map_or(f64::INFINITY, f64::from),
                            result.fresh_array_value.is_some(), result.fresh_vector_result_contract, self.fresh_calls.contains(span),
                        )
                    });
                    return self.borrowed_vector_call_result(
                        node,
                        path,
                        &parameters,
                        self.standard_sum_owned_call_result(node, path, result),
                    );
                }
                SourceDeclaration::Constant { result } => return result,
                SourceDeclaration::Constructor { result, .. } => return result,
                SourceDeclaration::ExternalCrate
                | SourceDeclaration::Module
                | SourceDeclaration::FnMutTemplate(_)
                | SourceDeclaration::Data => {}
            }
        }
        if let SNode::Name { path: names } = &node.node
            && names.len() == 2
            && self.lookup_type_owner(&names[..1], path).is_none()
            && super::rust_fixed_type(&names[0]).is_some()
            && ["MIN", "MAX"].contains(&names[1].as_str())
        {
            let mut origin = SourceOrigin::scalar_copy_value(super::rust_fixed_type(&names[0]));
            origin.source_owner = Some(crate::translation::source_signatures::SourceOwner {
                kind: "primitive".to_owned(),
                path: vec![names[0].clone()],
                arguments: Vec::new(),
            });
            return origin;
        }
        if let SNode::If {
            then, otherwise, ..
        } = &node.node
        {
            return self.join(&[self.origin(then, path), self.origin(otherwise, path)]);
        }
        let operation = match &node.node {
            SNode::Binary { op, .. } => op.name(),
            _ => "",
        };
        let form = read_rust_source_expression_origin(node.node.kind(), operation);
        match form.as_str() {
            "value" => match &node.node {
                SNode::Num { ty, .. } => SourceOrigin::scalar_copy_value(ty.clone()),
                SNode::Bool { .. } => SourceOrigin::scalar_copy_value(Some(super::BOOL)),
                SNode::Unit => {
                    let mut origin = SourceOrigin::scalar_copy_value(Some(super::UNIT));
                    origin.source_owner = Some(super::super::source_signatures::SourceOwner {
                        kind: "unit".to_owned(),
                        path: Vec::new(),
                        arguments: Vec::new(),
                    });
                    origin
                }
                _ => SourceOrigin::value(),
            },
            "diverging" => SourceOrigin {
                diverging: true,
                ..SourceOrigin::value()
            },
            "derived-value"
            | "numeric-cast"
            | "primitive-operation"
            | "comparison"
            | "boolean-operation" => {
                let nodes: Vec<&SExpr> = match &node.node {
                    SNode::Unary { arg, .. }
                    | SNode::Cast { arg, .. }
                    | SNode::ToString { arg }
                    | SNode::Show { arg, .. } => vec![arg],
                    SNode::Binary { left, right, .. } => vec![left, right],
                    SNode::StringMap { object, .. } => vec![object],
                    SNode::StringTest { object, search, .. } => vec![object, search],
                    _ => Vec::new(),
                };
                let operands: Vec<_> = nodes
                    .iter()
                    .map(|operand| self.origin(operand, path))
                    .collect();
                if operands
                    .iter()
                    .any(|operand| operand.integer_source_owner_conflict)
                {
                    return SourceOrigin::integer_conflict();
                }
                if operands.iter().any(|operand| !operand.proven) {
                    let uncertain = operands.iter().any(|operand| {
                        operand.uncertain
                            || (form != "derived-value" && !operand.wrappers.is_empty())
                    });
                    return if uncertain {
                        SourceOrigin::uncertain()
                    } else {
                        SourceOrigin::unknown()
                    };
                }
                if !self.collecting
                    && ((form == "comparison"
                        && operands[0].wrappers != operands[1].wrappers
                        && !accept_rust_source_text_equality(
                            operation,
                            (
                                operands[0].direct_string_value(),
                                operands[1].direct_string_value(),
                                operands[0].borrowed_string_slice(),
                                operands[1].borrowed_string_slice(),
                            ),
                        ))
                        || (form == "primitive-operation"
                            && operands.iter().any(|operand| {
                                operand.wrappers.iter().any(|layer| layer == "box")
                            }))
                        || (["numeric-cast", "boolean-operation"].contains(&form.as_str())
                            && operands.iter().any(|operand| !operand.wrappers.is_empty())))
                {
                    return SourceOrigin::uncertain();
                }
                if form == "derived-value" {
                    if matches!(node.node, SNode::StringTest { .. }) {
                        return SourceOrigin {
                            source_owner: Some(
                                crate::translation::source_signatures::SourceOwner {
                                    kind: "primitive".to_owned(),
                                    path: vec!["bool".to_owned()],
                                    arguments: Vec::new(),
                                },
                            ),
                            ..SourceOrigin::scalar_copy_value(Some(super::BOOL))
                        };
                    }
                    return SourceOrigin {
                        source_owned_result_proof: true,
                        ..SourceOrigin::text_value()
                    };
                }
                let owners = operands
                    .iter()
                    .map(SourceOrigin::pointer_integer_owner)
                    .collect::<Vec<_>>();
                if (form == "primitive-operation" || form == "comparison")
                    && !read_rust_pointer_integer_owner_join_error(&owners).is_empty()
                {
                    return SourceOrigin::integer_conflict();
                }
                let owner = if form == "primitive-operation" {
                    select_rust_pointer_integer_result_owner(&owners)
                } else {
                    String::new()
                };
                let primitive = if form == "primitive-operation" {
                    primitive_expressions::select_primitive_expression_origin(
                        operation, &nodes, &operands,
                    )
                } else {
                    None
                };
                let boolean = crate::translation::rust_primitive_expression_rules::accept_rust_primitive_boolean_result(
                    &form, operation,
                    operands.len() == 1 && operands[0].ty == Some(super::BOOL)
                        && operands[0].proven && operands[0].wrappers.is_empty(),
                );
                let ty = match &node.node {
                    SNode::Cast { to, .. } => Some(to.clone()),
                    _ if boolean => Some(super::BOOL),
                    _ if operation == "concat" => Some(super::STRING),
                    _ => {
                        if let Some(value) = &primitive {
                            value.ty.clone()
                        } else if owner.is_empty() {
                            None
                        } else {
                            operands
                                .iter()
                                .zip(&owners)
                                .find(|(_, candidate)| *candidate == &owner)
                                .and_then(|(origin, _)| origin.ty.clone())
                        }
                    }
                };
                let copy_evidence = form != "primitive-operation"
                    || (operation != "concat"
                        && operands
                            .iter()
                            .all(|operand| operand.copy_evidence.is_some()));
                SourceOrigin {
                    ty,
                    float_source_model: primitive
                        .as_ref()
                        .and_then(|origin| origin.float_source_model),
                    source_owner: if boolean {
                        Some(crate::translation::source_signatures::SourceOwner {
                            kind: "primitive".to_owned(),
                            path: vec!["bool".to_owned()],
                            arguments: Vec::new(),
                        })
                    } else {
                        primitive
                            .as_ref()
                            .and_then(|origin| origin.source_owner.clone())
                            .or_else(|| {
                                (!owner.is_empty()).then(|| {
                                    crate::translation::source_signatures::SourceOwner {
                                        kind: "primitive".to_owned(),
                                        path: vec![owner],
                                        arguments: Vec::new(),
                                    }
                                })
                            })
                    },
                    copy_evidence: copy_evidence.then_some(()),
                    text_source_model: (operation == "concat").then_some("owned string"),
                    ..SourceOrigin::value()
                }
            }
            _ => SourceOrigin::unknown(),
        }
    }
}
