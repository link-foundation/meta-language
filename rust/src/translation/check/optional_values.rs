//! Host materialization for the checked shared optional value decisions.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, Node, Result, SExpr, SNode, Span, Type, binary_node,
    plain_binary, type_error, unsupported,
};
use crate::translation::optional_value_rules::{
    read_optional_boolean_operation, read_optional_field_operation, read_optional_number_operation,
    require_optional_coalescing, require_optional_flattening,
};
use crate::translation::structured_type_rules::merge_optional_absence;

/// Materialize every actual intermediate flatten node, retaining its operands.
pub(super) fn flatten_option(mut value: Expr) -> Expr {
    loop {
        let element_kind = match &value.ty {
            Type::Option { element, .. } => element.kind(),
            _ => "",
        };
        if !require_optional_flattening(value.ty.kind(), element_kind) {
            return value;
        }
        let Type::Option {
            element,
            absence: outer,
        } = value.ty.clone()
        else {
            unreachable!("the generated flatten plan requires Option")
        };
        let Type::Option {
            element,
            absence: inner,
        } = *element
        else {
            unreachable!("the generated flatten plan requires its actual inner Option")
        };
        let ty = Type::Option {
            element,
            absence: merge_optional_absence(&inner, &outer),
        };
        value = Expr::new(
            Node::FlattenOption {
                value: Box::new(value),
            },
            ty,
        );
    }
}

impl Checker {
    pub(super) fn optional_expr(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        match &node.node {
            SNode::Some { value, ty } => {
                let ty = self.resolve_type(Some(ty), path, node.span)?;
                let Type::Option { element, .. } = &ty else {
                    return Err(type_error(
                        "a present optional value requires its actual optional type",
                        node.span,
                    ));
                };
                let value = self.expr(value, env, path, Some(element), false)?;
                let value = self.coerce(value, element, node.span)?;
                Ok(Expr::new(
                    Node::Some {
                        value: Box::new(value),
                    },
                    ty,
                ))
            }
            SNode::FlattenOption { value } => {
                let value = self.expr(value, env, path, None, false)?;
                let element_kind = match &value.ty {
                    Type::Option { element, .. } => element.kind(),
                    _ => "",
                };
                if !require_optional_flattening(value.ty.kind(), element_kind) {
                    return Err(type_error(
                        "optional flattening requires two actual optional layers",
                        node.span,
                    ));
                }
                let Type::Option {
                    element,
                    absence: outer,
                } = value.ty.clone()
                else {
                    unreachable!("the generated plan validates the outer Option")
                };
                let Type::Option {
                    element,
                    absence: inner,
                } = *element
                else {
                    unreachable!("the generated plan validates the inner Option")
                };
                let ty = Type::Option {
                    element,
                    absence: merge_optional_absence(&inner, &outer),
                };
                Ok(Expr::new(
                    Node::FlattenOption {
                        value: Box::new(value),
                    },
                    ty,
                ))
            }
            SNode::Coalesce { left, right } => {
                let mut source = (**left).clone();
                if let SNode::Index { optional, .. } = &mut source.node {
                    *optional = Some(true.into());
                }
                let left = self.expr(&source, env, path, None, false)?;
                if !require_optional_coalescing(left.ty.kind()) {
                    return Ok(left);
                }
                let Type::Option { element, .. } = &left.ty else {
                    unreachable!("the generated coalescing plan requires Option")
                };
                let ty = (**element).clone();
                let right = self.expr(right, env, path, Some(&ty), false)?;
                let right = self.coerce(right, &ty, node.span)?;
                Ok(Expr::new(
                    Node::Coalesce {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    ty,
                ))
            }
            SNode::OptionalField {
                object: source,
                field,
            } => {
                let object = self.expr(source, env, path, None, false)?;
                let element_kind = match &object.ty {
                    Type::Option { element, .. } => element.kind(),
                    _ => "",
                };
                let operation =
                    read_optional_field_operation(object.ty.kind(), element_kind, field);
                match operation.as_str() {
                    "field" => self.expr(
                        &SExpr::new(
                            SNode::Field {
                                object: source.clone(),
                                field: field.clone(),
                            },
                            node.span,
                        ),
                        env,
                        path,
                        expected,
                        false,
                    ),
                    "project" => Ok(Expr::new(
                        Node::OptionalProject {
                            object: Box::new(object),
                            field: field.clone(),
                        },
                        Type::Option {
                            element: Box::new(FLOAT),
                            absence: "undefined".to_owned(),
                        },
                    )),
                    "optional-field" => {
                        let Type::Option { element, .. } = &object.ty else {
                            unreachable!("the generated optional-field plan requires Option")
                        };
                        let Type::Record { fields, .. } = &**element else {
                            unreachable!(
                                "the generated optional-field plan requires the actual record layout"
                            )
                        };
                        let (index, declared) = fields
                            .iter()
                            .enumerate()
                            .find(|(_, candidate)| candidate.name == *field)
                            .ok_or_else(|| {
                                type_error(format!("record has no field {field}"), node.span)
                            })?;
                        let ty = Type::Option {
                            element: Box::new(declared.ty.clone()),
                            absence: "undefined".to_owned(),
                        };
                        Ok(flatten_option(Expr::new(
                            Node::OptionalField {
                                object: Box::new(object),
                                field: field.clone(),
                                index,
                            },
                            ty,
                        )))
                    }
                    _ => Err(unsupported(
                        "optional field access",
                        "this property needs a known structural type",
                        node.span,
                    )),
                }
            }
            _ => unreachable!("only optional Surface nodes enter their host adapter"),
        }
    }

    pub(super) fn logical_values(
        &mut self,
        op: BinaryOp,
        left: Expr,
        right: Expr,
        span: Option<Span>,
    ) -> Result<Expr> {
        let left_element = match &left.ty {
            Type::Option { element, .. } => element.key() == right.ty.key(),
            _ => false,
        };
        let right_element = match &right.ty {
            Type::Option { element, .. } => element.key() == left.ty.key(),
            _ => false,
        };
        let operation = crate::translation::optional_value_rules::read_optional_logical_operation(
            self.language.as_str(),
            left.ty.kind(),
            right.ty.kind(),
            left_element,
            right_element,
            left.ty.key() == right.ty.key(),
            op.name(),
        );
        if operation == "boolean" {
            return Ok(plain_binary(op, left, right, BOOL));
        }
        if operation == "language-error" {
            return Err(type_error("logical operands must be booleans", span));
        }
        if operation == "union-error" {
            return Err(unsupported(
                "logical value union",
                "these logical operands need a tagged union of their distinct value types",
                span,
            ));
        }
        let name = self.fresh_name("ml_logical");
        let bound = Expr::var(name.clone(), left.ty.clone());
        let mut retained = bound.clone();
        let mut other = right;
        let mut ty = left.ty.clone();
        match operation.as_str() {
            "unwrap-left" => {
                ty = other.ty.clone();
                retained = Expr::new(
                    Node::UnwrapOption {
                        arg: Box::new(bound.clone()),
                    },
                    ty.clone(),
                );
            }
            "wrap-right" => other = self.coerce(other, &ty, span)?,
            "wrap-left" => {
                ty = other.ty.clone();
                retained = self.coerce(bound.clone(), &ty, span)?;
            }
            _ => {}
        }
        let cond = self.boolean_value(bound, span)?;
        let (then, otherwise) = if op == BinaryOp::Or {
            (retained, other)
        } else {
            (other, retained)
        };
        let body = Expr::new(
            Node::If {
                cond: Box::new(cond),
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            ty.clone(),
        );
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(left),
                body: Box::new(body),
            },
            ty,
        ))
    }

    pub(super) fn optional_comparison(
        &mut self,
        op: BinaryOp,
        left: Expr,
        right: Expr,
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        let first = match &left.ty {
            Type::Option { element, .. } => element.as_ref(),
            ty => ty,
        };
        let second = match &right.ty {
            Type::Option { element, .. } => element.as_ref(),
            ty => ty,
        };
        let operation =
            crate::translation::optional_value_rules::read_optional_comparison_operation(
                self.language.as_str(),
                left.ty.kind(),
                right.ty.kind(),
                op.name(),
                first.key() == second.key(),
                crate::translation::optional_value_rules::accept_optional_primitive_kind(
                    first.kind(),
                ),
            );
        match operation.as_str() {
            "object-error" => Err(unsupported(
                "nullable object equality",
                "equality of these values requires shared reference identities",
                span,
            )),
            "strict" => Ok(Some(Expr::new(
                Node::StrictEqual {
                    left: Box::new(left),
                    right: Box::new(right),
                    negate: op == BinaryOp::Ne,
                },
                BOOL,
            ))),
            "number" => {
                let left = self.number_value(left, span)?;
                let right = self.number_value(right, span)?;
                let (left, right) = self.unify(left, right, span)?;
                let domain = left.ty.clone();
                Ok(Some(Expr::new(
                    binary_node(
                        op,
                        Box::new(left),
                        Box::new(right),
                        Some(domain),
                        None,
                        None,
                        None,
                    ),
                    BOOL,
                )))
            }
            _ => Ok(None),
        }
    }

    pub(super) fn number_value(&self, value: Expr, span: Option<Span>) -> Result<Expr> {
        let element = match &value.ty {
            Type::Option { element, .. } => element.kind(),
            _ => "",
        };
        match read_optional_number_operation(self.language.as_str(), value.ty.kind(), element)
            .as_str()
        {
            "identity" => Ok(value),
            "number" => Ok(Expr::new(
                Node::NullishNumber {
                    arg: Box::new(value),
                },
                FLOAT,
            )),
            _ => Err(unsupported(
                "nullable arithmetic operand",
                "this operand needs its source numeric coercion and error behavior represented",
                span,
            )),
        }
    }

    pub(super) fn boolean_value(&self, value: Expr, span: Option<Span>) -> Result<Expr> {
        match read_optional_boolean_operation(self.language.as_str(), value.ty.kind()).as_str() {
            "identity" => Ok(value),
            "truthy" => Ok(Expr::new(
                Node::Truthy {
                    arg: Box::new(value),
                },
                BOOL,
            )),
            _ => self.coerce(value, &BOOL, span),
        }
    }
}

pub(super) fn annotation_compatible(actual: &Type, declared: &Type) -> bool {
    let root = serde_json::json!([actual, declared]);
    let facts = crate::translation::surface_reference_facts::create_surface_reference_facts(&root);
    fn visit(
        facts: &crate::translation::surface_reference_facts::SurfaceReferenceFacts<'_>,
        actual: f64,
        declared: f64,
    ) -> bool {
        let plan =
            crate::translation::optional_inference_rules::plan_optional_annotation_constraints(
                &facts.texts,
                &facts.starts,
                &facts.counts,
                &facts.labels,
                &facts.targets,
                actual,
                declared,
            );
        !plan.is_empty()
            && plan[1..]
                .chunks_exact(2)
                .all(|pair| visit(facts, pair[0], pair[1]))
    }
    let root: usize = facts
        .root
        .to_string()
        .parse()
        .expect("a fact root is an actual host position");
    let start: usize = facts.starts[root]
        .to_string()
        .parse()
        .expect("a fact start is an actual host position");
    visit(&facts, facts.targets[start], facts.targets[start + 1])
}
