//! Expression checking: operators, arithmetic, applications and constructors.

use super::arrays::{array_text, immutable_product_value};
use super::expectation::ExpectedType;
use super::{
    BOOL, BinaryOp, Checker, Decimal, Env, Expr, FLOAT, INT, Language, LitValue, Node, Result,
    Rounding, SExpr, SNode, STRING, Semantics, Span, Type, UNIT, UnaryOp, arithmetic_semantics,
    binary_node, cast_to, coerce, comparison, fixed_bounds, negate_number, op_name, plain_binary,
    text_lit, type_error, unsupported,
};
use crate::translation::frontend_rules::accept_java_script_optional_tuple_context;
use crate::translation::rust_character_rules::accept_rust_numeric_literal_negation;
use crate::translation::surface::SurfaceBindingTypeAnnotation;
use crate::translation::typed_program_link_rules::accept_program_tuple_product_model;

mod branches;
mod calls;

impl Checker {
    pub(super) fn coerce(&self, value: Expr, ty: &Type, span: Option<Span>) -> Result<Expr> {
        coerce(value, ty, self.language, span, None)
    }

    pub(super) fn expr(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        self.expr_with_context(
            node,
            env,
            path,
            ExpectedType::Concrete(expected),
            allow_literal,
        )
    }

    pub(super) fn expr_with_callable_expectation(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        params: &[Type],
        ret: Option<&Type>,
    ) -> Result<Expr> {
        if let Some(ret) = ret {
            let expected = Type::Function {
                params: params.to_vec(),
                ret: Box::new(ret.clone()),
            };
            return self.expr(node, env, path, Some(&expected), false);
        }
        self.expr_with_context(
            node,
            env,
            path,
            ExpectedType::Callable { params, ret },
            false,
        )
    }

    pub(super) fn expr_with_context(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: ExpectedType<'_>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let expression = if let SNode::ReferencePayload { value, extra } = &node.node {
            if !extra.is_empty()
                || node.block
                || node.tag_test.is_some()
                || node.parenthesized.is_some()
                || matches!(value.node, SNode::ReferencePayload { .. })
            {
                return Err(type_error(
                    "a reference payload needs its closed owned expression",
                    node.span,
                ));
            }
            value.as_ref()
        } else {
            node
        };
        let payload = !crate::translation::surface_reference_analysis_rules::read_surface_reference_payload_operand(node.node.kind()).is_empty();
        let mut result = self.expr_inner(expression, env, path, expected, allow_literal)?;
        if payload {
            let refusal = crate::translation::surface_reference_analysis_rules::read_surface_reference_payload_error(result.ty.kind(), result.ty.is_reference());
            if !refusal.is_empty() {
                return Err(type_error(refusal, node.span));
            }
        }
        if crate::translation::surface_reference_analysis_rules::select_surface_reference_allocation(
            self.reference_semantics && !payload,
            result.ty.kind(),
            result.ty.is_reference(),
        ) {
            let ty = result.ty.with_reference(true);
            result = Expr::new(
                Node::ReferenceAllocate {
                    value: Box::new(result),
                },
                ty,
            );
        }
        let span = node.span.or(if payload { expression.span } else { None });
        if span.is_some() && result.span.is_none() {
            result.span = span;
        }
        Ok(result)
    }

    pub(super) fn reference_value(value: Expr) -> Expr {
        if crate::translation::surface_reference_analysis_rules::select_surface_reference_read(
            value.ty.is_reference(),
        ) {
            let ty = value.ty.with_reference(false);
            Expr::new(
                Node::ReferenceRead {
                    reference: Box::new(value),
                },
                ty,
            )
        } else {
            value
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn expr_inner(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        context: ExpectedType<'_>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let span = node.span;
        let expected = context.concrete();
        match &node.node {
            SNode::ArrayBindingRest { .. } | SNode::ArrayBindingDefault { .. } => {
                self.array_binding(node, env, path)
            }
            SNode::Num { .. }
            | SNode::Bool { .. }
            | SNode::Str { .. }
            | SNode::Unit
            | SNode::None { .. } => {
                self.check_literal_expression(node, env, path, context, allow_literal)
            }
            SNode::BooleanValue { arg } => {
                let value = self.expr(arg, env, path, None, false)?;
                self.boolean_value(value, span)
            }
            SNode::NumberValue { arg } => {
                let value = self.expr(arg, env, path, None, false)?;
                let value = self.number_value(value, span)?;
                self.coerce(value, &FLOAT, span)
            }
            SNode::Some { .. }
            | SNode::FlattenOption { .. }
            | SNode::Coalesce { .. }
            | SNode::OptionalField { .. } => self.optional_expr(node, env, path, expected),
            SNode::Name { path: segments } => {
                if let Some(name) = env.local(node) {
                    return Ok(Expr::var(name, env.vars[name].clone()));
                }
                if let Some(reference) = self.callable_reference(node, segments, path, context)? {
                    return Ok(reference);
                }
                self.application(node, &[], env, path, expected, span)
            }
            SNode::FunctionValue { .. }
            | SNode::Lambda { .. }
            | SNode::CellAllocate { .. }
            | SNode::CellRead { .. }
            | SNode::CellWrite { .. } => self.callable_expression(node, env, path, context),
            SNode::DotCtor { .. } => self.application(node, &[], env, path, expected, span),
            SNode::App { func, args } => self.application(func, args, env, path, expected, span),
            SNode::Field { field, object } => {
                self.record_source_field(object, field, env, path, span)
            }
            SNode::TypeOf { arg } => self.type_query(arg, env, path, span),
            SNode::ArrayIsArray { arg } => {
                let value = self.expr(arg, env, path, None, false)?;
                let kind = value.ty.kind();
                let element = match &value.ty {
                    Type::Option { element, .. } => element.kind(),
                    _ => "",
                };
                let plan = crate::translation::array_classification_rules::read_java_script_array_classification_plan(kind, element);
                let name = self.fresh_name("ml_array_test");
                let body = match plan.as_slice() {
                    [false, false] => Expr::lit(BOOL, LitValue::Bool(false)),
                    [true, false] => Expr::lit(BOOL, LitValue::Bool(true)),
                    [true, true] => {
                        self.boolean_value(Expr::var(name.clone(), value.ty.clone()), span)?
                    }
                    _ => {
                        return Err(unsupported(
                            "Array.isArray operand",
                            "the checked value needs a closed represented kind or a flat optional array/tuple; unknown union/proxy domains need an explicit operational model",
                            span,
                        ));
                    }
                };
                Ok(Expr::new(
                    Node::Let {
                        name,
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    BOOL,
                ))
            }

            SNode::Unary {
                op: UnaryOp::Not,
                arg,
            } => {
                let arg = self.expr(arg, env, path, None, false)?;
                let arg = self.boolean_value(arg, span)?;
                Ok(Expr::new(
                    Node::Unary {
                        op: UnaryOp::Not,
                        arg: Box::new(arg),
                        semantics: None,
                    },
                    BOOL,
                ))
            }
            SNode::Unary {
                op: UnaryOp::Neg,
                arg,
            } => {
                if let SNode::Num {
                    value,
                    ty,
                    negative,
                    ..
                } = &arg.node
                {
                    if !accept_rust_numeric_literal_negation(
                        self.language.as_str(),
                        ty.as_ref().map_or("", Type::kind),
                        matches!(ty, Some(Type::Fixed { signed: true, .. })),
                    ) {
                        let name = ty
                            .as_ref()
                            .map_or_else(|| "untyped literal".to_owned(), Type::key);
                        return Err(type_error(format!("negation of {name}"), span));
                    }
                    let flipped = SExpr {
                        node: SNode::Num {
                            value: value.clone(),
                            ty: ty.clone(),
                            negative: !negative,
                            unit: false,
                        },
                        span,
                        parenthesized: arg.parenthesized.clone(),
                        block: arg.block,
                        tag_test: None,
                    };
                    let literal = self.expr(&flipped, env, path, expected, allow_literal)?;
                    if !accept_rust_numeric_literal_negation(
                        self.language.as_str(),
                        literal.ty.kind(),
                        matches!(literal.ty, Type::Fixed { signed: true, .. }),
                    ) {
                        return Err(type_error(
                            format!("negation of {}", literal.ty.key()),
                            span,
                        ));
                    }
                    return Ok(literal);
                }
                let numeric = expected.filter(|ty| ty.is_numeric());
                let arg = self.expr(arg, env, path, numeric, false)?;
                let arg = self.number_value(arg, span)?;
                if arg.ty.is_float() {
                    return Ok(Expr::new(
                        Node::Unary {
                            op: UnaryOp::Neg,
                            arg: Box::new(arg),
                            semantics: Some(Semantics::Ieee),
                        },
                        FLOAT,
                    ));
                }
                if arg.ty == Type::Nat && self.language == Language::JavaScript {
                    // A guarded natural is still a BigInt, and its negation an integer.
                    return Ok(Expr::new(
                        Node::Unary {
                            op: UnaryOp::Neg,
                            arg: Box::new(self.coerce(arg, &INT, span)?),
                            semantics: Some(Semantics::Exact),
                        },
                        INT,
                    ));
                }
                if !matches!(arg.ty, Type::Int | Type::Fixed { signed: true, .. }) {
                    return Err(type_error(format!("negation of {}", arg.ty.key()), span));
                }
                let ty = arg.ty.clone();
                let semantics = if matches!(ty, Type::Fixed { .. }) {
                    Semantics::Checked
                } else {
                    Semantics::Exact
                };
                Ok(Expr::new(
                    Node::Unary {
                        op: UnaryOp::Neg,
                        arg: Box::new(arg),
                        semantics: Some(semantics),
                    },
                    ty,
                ))
            }
            SNode::ReferenceIdentity {
                op,
                left,
                right,
                extra,
            } => {
                if !extra.is_empty()
                    || node.parenthesized.is_some()
                    || node.block
                    || node.tag_test.is_some()
                {
                    return Err(type_error(
                        "explicit reference identity needs its closed owned operands",
                        span,
                    ));
                }
                let (left, right) = self.operands(left, right, env, path, span)?;
                let form = crate::translation::rust_shared_array_handle_rules::read_rust_shared_array_handle_comparison_projection(
                    self.language.as_str(), op_name(*op), left.ty.kind(), left.ty.is_reference(),
                );
                if !self.reference_semantics || form != "referenceIdentity"
                    || !crate::translation::reference_identity_rules::accept_reference_identity_comparison(
                        op_name(*op), left.ty.kind(), right.ty.kind(), left.ty.is_reference(), right.ty.is_reference(), left.ty == right.ty,
                    ) {
                    return Err(unsupported("explicit Rust array identity", "only the owned array reference domain admits this operation", span));
                }
                let domain = left.ty.clone();
                Ok(Expr::new(
                    binary_node(
                        *op,
                        Box::new(left),
                        Box::new(right),
                        Some(domain),
                        None,
                        None,
                        None,
                    ),
                    BOOL,
                ))
            }
            SNode::Binary {
                op,
                left,
                right,
                rounding,
            } => self.binary(
                *op,
                left,
                right,
                *rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
            ),
            SNode::If { .. } => {
                self.check_conditional_expression(node, env, path, context, allow_literal)
            }
            SNode::Let { .. } => {
                self.check_binding_expression(node, env, path, context, allow_literal)
            }
            SNode::Match { scrutinees, rows } => {
                let compiled = self.compile_match(scrutinees, rows, span, env, path)?;
                self.expr_with_context(&compiled, env, path, context, allow_literal)
            }
            SNode::Match1 { scrutinee, cases } => {
                self.match_cases(scrutinee, cases, span, env, path, context)
            }
            SNode::StringMap { .. } => self.string_map(node, env, path),
            SNode::CopySlice { object } => self.copy_slice(object, env, path, span),
            SNode::Method { object, op, args } => {
                let form =
                    crate::translation::array_callback_rules::read_array_callback_result_form(op);
                if form.is_empty() || form == "mapped array" {
                    self.primitive_method(object, op, args, env, path, span)
                } else {
                    self.array_callback_method(node, env, path, context, &form)
                }
            }
            SNode::StringTest { .. } => self.string_test(node, env, path),
            SNode::ToString { arg } => {
                let arg = self.expr(arg, env, path, None, false)?;
                let refusal =
                    crate::translation::primitive_value_rules::read_number_display_refusal(
                        self.language.as_str(),
                        arg.ty.kind(),
                    );
                if !refusal.is_empty() {
                    return Err(unsupported("Rust Float display", &refusal, span));
                }
                if arg.ty == Type::String {
                    return Ok(arg);
                }
                if let Type::Option { element, .. } = &arg.ty
                    && crate::translation::optional_value_rules::accept_optional_primitive_kind(
                        element.kind(),
                    )
                {
                    return Ok(Expr::new(
                        Node::OptionalText {
                            arg: Box::new(arg),
                            console: false,
                        },
                        STRING,
                    ));
                }
                if arg.ty.element().is_some() {
                    return Err(array_text(span));
                }
                if !arg.ty.is_ordered() && arg.ty != Type::Bool {
                    return Err(type_error(format!("toString of {}", arg.ty.key()), span));
                }
                Ok(Expr::new(
                    Node::ToString {
                        arg: Box::new(arg),
                        console: false,
                    },
                    STRING,
                ))
            }
            SNode::Show { arg, style } => self.show(arg, env, path, *style),
            SNode::Cast {
                arg,
                to,
                from,
                flavor,
            } => {
                let to = self.resolve_type(Some(to), path, span)?;
                let arg = self.expr(arg, env, path, from.as_ref(), false)?;
                cast_to(arg, &to, *flavor, span)
            }
            SNode::Abort { message } => {
                let Some(expected) = expected else {
                    return Err(type_error("abort needs a known result type", span));
                };
                Ok(Expr::new(
                    Node::Abort {
                        message: message.clone(),
                    },
                    expected.clone(),
                ))
            }
            SNode::CtorObject {
                tag,
                fields,
                frozen,
                prototype_definition,
            } => {
                if let Some(definition) = prototype_definition {
                    return Err(unsupported(
                        "object prototype definition",
                        "an uncomputed __proto__: entry is prototype-definition syntax rather than an own data property; the portable core does not represent object prototypes",
                        Some(definition.span),
                    ));
                }
                let mut value = self.ctor_object(tag, fields, span, env, path, expected)?;
                if let Node::Ctor {
                    frozen: checked_frozen,
                    ..
                } = &mut value.node
                {
                    *checked_frozen = *frozen;
                }
                Ok(value)
            }
            SNode::Print { expr, style, body } => {
                let text = self.show(expr, env, path, *style)?;
                let body = self.expr_with_context(body, env, path, context, allow_literal)?;
                let ty = body.ty.clone();
                Ok(Expr::new(
                    Node::Print {
                        text: Box::new(text),
                        body: Box::new(body),
                    },
                    ty,
                ))
            }
            SNode::Length { object, integer } => self.length(object, *integer, span, env, path),
            SNode::Array { items, element } => {
                let context = match expected {
                    Some(ty @ Type::Option { element, .. })
                        if accept_java_script_optional_tuple_context(
                            self.language.as_str(),
                            node.node.kind(),
                            ty.kind(),
                            element.kind(),
                        ) =>
                    {
                        Some(element.as_ref())
                    }
                    _ => expected,
                };
                self.array_literal(items, element.as_ref(), span, env, path, context)
            }
            SNode::Record {
                entries,
                record_projection,
                prototype_definition,
                ..
            } => {
                if let Some(definition) = prototype_definition {
                    return Err(unsupported(
                        "object prototype definition",
                        "an uncomputed __proto__: entry is prototype-definition syntax rather than an own data property; the portable core does not represent object prototypes",
                        Some(definition.span),
                    ));
                }
                self.record_literal(
                    entries,
                    record_projection.as_ref(),
                    env,
                    path,
                    expected,
                    span,
                )
            }
            SNode::Tuple {
                items,
                frozen,
                product_model,
            } => {
                if product_model
                    .as_ref()
                    .is_some_and(|model| !accept_program_tuple_product_model(model, *frozen))
                {
                    return Err(type_error(
                        "unknown or contradictory tuple product model",
                        span,
                    ));
                }
                let context = match expected {
                    Some(ty @ Type::Option { element, .. })
                        if accept_java_script_optional_tuple_context(
                            self.language.as_str(),
                            node.node.kind(),
                            ty.kind(),
                            element.kind(),
                        ) =>
                    {
                        Some(element.as_ref())
                    }
                    _ => expected,
                };
                let mut value = self.tuple_literal(items, *frozen, span, env, path, context)?;
                if let Node::Tuple {
                    product_model: checked_model,
                    ..
                } = &mut value.node
                {
                    checked_model.clone_from(product_model);
                }
                Ok(value)
            }
            SNode::TupleIndex { object, index } => {
                self.tuple_index(object, *index, span, env, path)
            }
            SNode::Index { .. } => self.index(node, env, path, expected),
            SNode::ForOfIterable { object, .. } => self.for_of_iterable(object, span, env, path),
            SNode::ArrayFrom { args, .. } => self.array_from(args, env, path, expected, span),
            SNode::Set { .. } | SNode::Map { .. } => {
                self.collection_construction(node, env, path, expected)
            }
            SNode::StringPoints { object } => {
                let value = self.expr(object, env, path, Some(&STRING), false)?;
                let object = self.coerce(value, &STRING, span)?;
                Ok(Expr::new(
                    Node::StringPoints {
                        object: Box::new(object),
                    },
                    crate::translation::types::array(STRING),
                ))
            }
            SNode::ReferenceWrite { .. } => Err(unsupported(
                &crate::translation::array_value_rules::read_reference_mutation_diagnostic(
                    "feature",
                ),
                &crate::translation::array_value_rules::read_reference_mutation_diagnostic(
                    "reason",
                ),
                span,
            )),
            SNode::Math { op, name, args } => self.math(op, name, args, span, env, path),
            other => Err(type_error(
                format!("unknown expression {}", other.kind()),
                span,
            )),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn binary(
        &mut self,
        op: BinaryOp,
        left: &SExpr,
        right: &SExpr,
        rounding: Option<Rounding>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        match op {
            BinaryOp::And | BinaryOp::Or => {
                let left = self.expr(left, env, path, None, false)?;
                let right = self.expr(right, env, path, None, false)?;
                self.logical_values(op, left, right, span)
            }
            BinaryOp::Concat => {
                let left = self.expr(left, env, path, Some(&STRING), false)?;
                let left = self.coerce(left, &STRING, span)?;
                let right = self.expr(right, env, path, Some(&STRING), false)?;
                let right = self.coerce(right, &STRING, span)?;
                Ok(plain_binary(op, left, right, STRING))
            }
            _ if comparison(op) => {
                if self.language == Language::JavaScript {
                    let mut checked_left = self.expr(left, env, path, None, true)?;
                    let mut checked_right = self.expr(right, env, path, None, true)?;
                    if checked_left.ty.is_literal() {
                        checked_left =
                            self.expr(left, env, path, Some(&self.default_number()), false)?;
                    }
                    if checked_right.ty.is_literal() {
                        checked_right =
                            self.expr(right, env, path, Some(&self.default_number()), false)?;
                    }
                    if let Some(value) =
                        self.optional_comparison(op, checked_left, checked_right, span)?
                    {
                        return Ok(value);
                    }
                }
                let (left, right) = self.operands(left, right, env, path, span)?;
                if self.language == Language::JavaScript && crate::translation::reference_identity_rules::accept_reference_identity_comparison(
                    op_name(op), left.ty.kind(), right.ty.kind(),
                    left.ty.is_reference(), right.ty.is_reference(), left.ty == right.ty,
                ) {
                    let domain = left.ty.clone();
                    return Ok(Expr::new(binary_node(op, Box::new(left), Box::new(right), Some(domain), None, None, None), BOOL));
                }
                if matches!(left.ty, Type::Tuple { .. }) {
                    return Err(unsupported(
                        "comparison of tuples",
                        "tuple value equality and JavaScript reference identity need an explicit operational comparison model; compare positional elements",
                        span,
                    ));
                }
                if matches!(left.ty, Type::Data { .. } | Type::Unit) {
                    return Err(unsupported(
                        "structural equality of data values",
                        "comparisons of data values are not in the portable core",
                        span,
                    ));
                }
                if left.ty.element().is_some() {
                    return Err(unsupported(
                        "comparison of arrays",
                        "=== of two arrays compares which array each is, which a value translation does not keep; compare their elements",
                        span,
                    ));
                }
                let string_order =
                    crate::translation::string_order_rules::read_string_order_operation(
                        self.language.as_str(),
                        op_name(op),
                        if left.ty == STRING { "string" } else { "" },
                    );
                if !string_order.is_empty() {
                    return Ok(Expr::new(
                        Node::StringTest {
                            op: string_order,
                            string: Box::new(left),
                            search: Box::new(right),
                        },
                        BOOL,
                    ));
                }
                if !matches!(op, BinaryOp::Eq | BinaryOp::Ne) && !left.ty.is_ordered() {
                    return Err(type_error(format!("ordering on {}", left.ty.key()), span));
                }
                let domain = left.ty.clone();
                Ok(Expr::new(
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
                ))
            }
            BinaryOp::Plus => self.plus(
                left,
                right,
                rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
            ),
            _ => self.arithmetic(
                op,
                left,
                right,
                rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
                None,
            ),
        }
    }

    /// JavaScript's `+` concatenates when either operand is a string, reading
    /// the other as `String(value)`, and adds numbers otherwise.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn plus(
        &mut self,
        left_surface: &SExpr,
        right_surface: &SExpr,
        rounding: Option<Rounding>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
    ) -> Result<Expr> {
        let left = self.expr(left_surface, env, path, None, true)?;
        let right = self.expr(right_surface, env, path, None, true)?;
        if left.ty != Type::String && right.ty != Type::String {
            return self.arithmetic(
                BinaryOp::Add,
                left_surface,
                right_surface,
                rounding,
                span,
                env,
                path,
                expected,
                allow_literal,
                Some((left, right)),
            );
        }
        let left = self.text(left, left_surface, env, path)?;
        let right = self.text(right, right_surface, env, path)?;
        Ok(plain_binary(BinaryOp::Concat, left, right, STRING))
    }

    pub(super) fn text(
        &mut self,
        value: Expr,
        surface: &SExpr,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if value.ty == Type::String {
            return Ok(value);
        }
        let checked = if value.ty.is_literal() {
            self.expr(surface, env, path, Some(&self.default_number()), false)?
        } else {
            value
        };
        if matches!(checked.ty, Type::Data { .. } | Type::Unit) {
            return Err(unsupported(
                "string conversion of structured values",
                &format!("String({}) has no portable textual form", checked.ty.key()),
                surface.span,
            ));
        }
        if let Type::Option { element, .. } = &checked.ty {
            if !crate::translation::optional_value_rules::accept_optional_primitive_kind(
                element.kind(),
            ) {
                return Err(unsupported(
                    "string conversion of nullable structured values",
                    "this conversion needs its source structured-value formatter",
                    surface.span,
                ));
            }
            return Ok(Expr::new(
                Node::OptionalText {
                    arg: Box::new(checked),
                    console: false,
                },
                STRING,
            ));
        }
        if checked.ty.element().is_some() {
            return Err(array_text(surface.span));
        }
        Ok(Expr::new(
            Node::ToString {
                arg: Box::new(checked),
                console: false,
            },
            STRING,
        ))
    }

    /// `checked` holds operands already checked without an expected type, as `plus` does.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn arithmetic(
        &mut self,
        op: BinaryOp,
        left_surface: &SExpr,
        right_surface: &SExpr,
        rounding: Option<Rounding>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        allow_literal: bool,
        checked: Option<(Expr, Expr)>,
    ) -> Result<Expr> {
        if !matches!(
            op,
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem
        ) {
            return Err(type_error(
                format!("unknown operator {}", op_name(op)),
                span,
            ));
        }
        let numeric_expected = expected.filter(|ty| ty.is_numeric()).cloned();
        let (checked_left, checked_right) = match checked {
            Some((left, right)) => (Some(left), Some(right)),
            None => (None, None),
        };
        let mut left = match checked_left {
            Some(left) if !left.ty.is_literal() => left,
            _ => self.expr(left_surface, env, path, numeric_expected.as_ref(), true)?,
        };
        let mut right = match checked_right {
            Some(right) if !right.ty.is_literal() => right,
            _ => {
                let right_expected = if left.ty.is_literal() {
                    numeric_expected.clone()
                } else {
                    Some(left.ty.clone())
                };
                self.expr(right_surface, env, path, right_expected.as_ref(), true)?
            }
        };
        if left.ty.is_literal() {
            let left_expected = if right.ty.is_literal() {
                numeric_expected.clone()
            } else {
                Some(right.ty.clone())
            };
            left = self.expr(left_surface, env, path, left_expected.as_ref(), true)?;
        }
        if left.ty.is_literal() && right.ty.is_literal() {
            if allow_literal {
                return Ok(text_lit(Type::Literal, "0"));
            }
            left = self.expr(left_surface, env, path, Some(&self.default_number()), false)?;
            right = self.expr(right_surface, env, path, Some(&left.ty), false)?;
        }
        if right.ty.is_literal() {
            right = self.expr(right_surface, env, path, Some(&left.ty), false)?;
        }
        if left.ty.is_literal() {
            left = self.expr(left_surface, env, path, Some(&right.ty), false)?;
        }
        if self.language == Language::JavaScript {
            left = self.number_value(left, span)?;
            right = self.number_value(right, span)?;
        }
        if self.language == Language::JavaScript && (left.ty == Type::Nat || right.ty == Type::Nat)
        {
            // BigInt arithmetic is integer arithmetic even on guarded naturals: `n - 1n` is -1n for n = 0n.
            let widen = |value: Expr| match &value.node {
                Node::Lit { value } => Ok(Expr::lit(INT, value.clone())),
                _ => coerce(value, &INT, Language::JavaScript, span, None),
            };
            left = widen(left)?;
            right = widen(right)?;
        }
        let (left, right) = self.unify(left, right, span)?;
        if !left.ty.is_ordered() {
            return Err(type_error(format!("arithmetic on {}", left.ty.key()), span));
        }
        let ty = left.ty.clone();
        let (semantics, rounding, by_zero) = arithmetic_semantics(self.language, op, &ty, rounding);
        Ok(Expr::new(
            binary_node(
                op,
                Box::new(left),
                Box::new(right),
                Some(ty.clone()),
                Some(semantics),
                rounding,
                by_zero,
            ),
            ty,
        ))
    }
}
