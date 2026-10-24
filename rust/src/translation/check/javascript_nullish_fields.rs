//! Actual property receiver and type operands into ordinary source error IR.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, INT, Language, LitValue, Node, Result, SExpr, SNode,
    STRING, Span, Type, UnaryOp, binary_node, plain_binary, unsupported,
};
use crate::translation::ir::RecordEntry;
use crate::translation::javascript_nullish_field_rules as rules;
use crate::translation::types::TypeField;

pub(super) enum JavaScriptPropertyAccess<'a> {
    Field(&'a str),
    Method(&'a str, &'a [Option<SExpr>]),
    Index(&'a SExpr),
}

fn primitive_key_text(value: Expr, span: Option<Span>) -> Result<Expr> {
    if let Type::Option { element, .. } = &value.ty {
        if **element == Type::Unit {
            return super::string_search::primitive_text(value, span);
        }
        let condition = Checker::javascript_optional_presence(value.clone(), element, span)?;
        let present = Expr::new(
            Node::UnwrapOption {
                arg: Box::new(value.clone()),
            },
            element.as_ref().clone(),
        );
        let text = primitive_key_text(present, span)?;
        return Ok(Expr::new(
            Node::If {
                cond: Box::new(condition),
                then: Box::new(text),
                otherwise: Box::new(super::string_search::primitive_text(value, span)?),
            },
            STRING,
        ));
    }
    if !matches!(value.ty, Type::Int | Type::Nat) {
        return super::string_search::primitive_text(value, span);
    }
    let integer = super::coerce(value.clone(), &INT, Language::JavaScript, span, None)?;
    let zero = super::text_lit(INT, "0");
    let negative = Expr::new(
        binary_node(
            BinaryOp::Lt,
            Box::new(integer.clone()),
            Box::new(zero.clone()),
            Some(INT),
            None,
            None,
            None,
        ),
        BOOL,
    );
    let magnitude = Expr::new(
        Node::If {
            cond: Box::new(negative),
            then: Box::new(Expr::new(
                Node::Unary {
                    op: UnaryOp::Neg,
                    arg: Box::new(integer.clone()),
                    semantics: None,
                },
                INT,
            )),
            otherwise: Box::new(integer),
        },
        INT,
    );
    let shifted = Expr::new(
        Node::Math {
            op: "shiftBigIntegerBitsRight".to_owned(),
            args: vec![
                magnitude,
                super::text_lit(
                    INT,
                    rules::read_java_script_big_integer_property_description_shift(),
                ),
            ],
        },
        INT,
    );
    let exceeds = Expr::new(
        binary_node(
            BinaryOp::Ne,
            Box::new(shifted),
            Box::new(zero),
            Some(INT),
            None,
            None,
            None,
        ),
        BOOL,
    );
    Ok(Expr::new(
        Node::If {
            cond: Box::new(exceeds),
            then: Box::new(super::text_lit(
                STRING,
                rules::read_java_script_big_integer_property_description_overflow(),
            )),
            otherwise: Box::new(super::string_search::primitive_text(value, span)?),
        },
        STRING,
    ))
}

fn describe_key(value: Expr, source: &SExpr, span: Option<Span>) -> Result<(bool, Expr)> {
    let fields = match &value.ty {
        Type::Record { fields, .. } => fields.as_slice(),
        _ => &[],
    };
    let form = rules::read_java_script_nullish_key_description(
        value.ty.kind(),
        fields.iter().any(|field| field.name == "toString"),
        fields.iter().any(|field| field.name == "constructor"),
    );
    if !rules::accept_java_script_nullish_key_description_origin(&form, source.node.kind()) {
        return Err(unsupported(
            "computed property key diagnostic",
            "the actual key needs its constructor provenance and side-effect-free source description",
            span,
        ));
    }
    match form.as_str() {
        "text" => Ok((true, primitive_key_text(value, span)?)),
        "omitted" => Ok((false, super::text_lit(STRING, ""))),
        "object" | "map" | "set" => Ok((
            true,
            super::text_lit(
                STRING,
                rules::read_java_script_nullish_key_description_text(&form),
            ),
        )),
        _ => Err(unsupported(
            "computed property key diagnostic",
            "the actual key needs its side-effect-free source description and constructor protocol",
            span,
        )),
    }
}

fn property_message(absence: &str, field: &str, description: Option<&(bool, Expr)>) -> Expr {
    let Some((named, text)) = description else {
        return super::text_lit(
            STRING,
            rules::read_java_script_nullish_field_message(absence, field),
        );
    };
    let prefix = super::text_lit(
        STRING,
        rules::read_java_script_nullish_property_message_prefix(absence, *named),
    );
    if !*named {
        return prefix;
    }
    plain_binary(
        BinaryOp::Concat,
        plain_binary(BinaryOp::Concat, prefix, text.clone(), STRING),
        super::text_lit(
            STRING,
            rules::read_java_script_nullish_property_message_suffix(true),
        ),
        STRING,
    )
}
impl Checker {
    pub(super) fn javascript_nullish_field(
        &mut self,
        operand: (&Expr, &SExpr),
        field: &str,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        self.javascript_nullish_access(
            operand,
            JavaScriptPropertyAccess::Field(field),
            env,
            path,
            expected,
            span,
        )
    }
    pub(super) fn javascript_nullish_access(
        &mut self,
        operand: (&Expr, &SExpr),
        access: JavaScriptPropertyAccess<'_>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        let (value, source) = operand;
        let field = match &access {
            JavaScriptPropertyAccess::Field(field) | JavaScriptPropertyAccess::Method(field, _) => {
                *field
            }
            JavaScriptPropertyAccess::Index(_) => "",
        };
        if self.language != Language::JavaScript {
            return Ok(None);
        }
        let element = match &value.ty {
            Type::Option { element, .. } => element.kind(),
            _ => "",
        };
        let form = rules::read_java_script_nullish_field_form(value.ty.kind(), element);
        if form == "direct" {
            return Ok(None);
        }
        let key_source = match &access {
            JavaScriptPropertyAccess::Index(node) => match &node.node {
                SNode::Index { index, .. } => Some(index.as_ref()),
                _ => unreachable!("a computed source access keeps its actual index"),
            },
            _ => None,
        };
        let key_value = key_source
            .map(|key| self.expr(key, env, path, None, false))
            .transpose()?;
        let capture = self.fresh_name("ml_nullish_field_receiver");
        let name = rules::read_java_script_nullish_field_capture();
        let key_field = rules::read_java_script_nullish_index_key_field();
        let mut fields = vec![TypeField {
            name: name.clone(),
            ty: value.ty.clone(),
        }];
        if let Some(key) = &key_value {
            fields.push(TypeField {
                name: key_field.clone(),
                ty: key.ty.clone(),
            });
        }
        let captured_type = Type::Record {
            fields,
            open: false,
            reference: false,
        };
        let key_name = key_value
            .as_ref()
            .map(|_| self.fresh_name("ml_computed_property_key"));
        let key = key_value.as_ref().map(|value| {
            Expr::new(
                Node::RecordField {
                    object: Box::new(Expr::var(capture.clone(), captured_type.clone())),
                    field: key_field.clone(),
                    index: 1,
                },
                value.ty.clone(),
            )
        });
        let mut source_scope = env.clone();
        if let (Some(name), Some(value)) = (&key_name, &key_value) {
            source_scope.bind_local(name, value.ty.clone());
        }
        let receiver = Expr::new(
            Node::RecordField {
                object: Box::new(Expr::var(capture.clone(), captured_type.clone())),
                field: name.clone(),
                index: 0,
            },
            value.ty.clone(),
        );
        let mut ty = expected.cloned().unwrap_or(FLOAT);
        let (present, success) = if form == "guarded" {
            let Type::Option { element, .. } = &value.ty else {
                unreachable!("a shared guarded field has its actual optional receiver")
            };
            let present = element.as_ref().clone();
            let binding = self.fresh_name("ml_nullish_field_present");
            let mut scope = source_scope;
            scope.bind_local(&binding, present.clone());
            let read = SExpr::new(
                SNode::Name {
                    path: vec![binding.clone()],
                },
                source.span,
            );
            let operation = match &access {
                JavaScriptPropertyAccess::Field(_) => SNode::Field {
                    object: Box::new(read),
                    field: field.to_owned(),
                },
                JavaScriptPropertyAccess::Method(_, args) => SNode::Method {
                    object: Box::new(read),
                    op: field.to_owned(),
                    args: args.to_vec(),
                },
                JavaScriptPropertyAccess::Index(node) => {
                    let SNode::Index {
                        index,
                        optional,
                        proven_in_bounds,
                        ..
                    } = &node.node
                    else {
                        unreachable!("the source index is retained")
                    };
                    let literal = matches!(&index.node, SNode::Num { .. } | SNode::Str { .. })
                        || matches!(&index.node, SNode::Unary { op: UnaryOp::Neg, arg } if matches!(&arg.node, SNode::Num { .. }));
                    let actual_key = if literal {
                        index.as_ref().clone()
                    } else {
                        SExpr::new(
                            SNode::Name {
                                path: vec![
                                    key_name
                                        .as_ref()
                                        .expect("a source computed read has its key binding")
                                        .clone(),
                                ],
                            },
                            index.span,
                        )
                    };
                    SNode::Index {
                        object: Box::new(read),
                        index: Box::new(actual_key),
                        optional: optional.clone(),
                        proven_in_bounds: proven_in_bounds.clone(),
                    }
                }
            };
            let field_source = SExpr::new(operation, span);
            let body = self.expr(&field_source, &scope, path, expected, false)?;
            ty = body.ty.clone();
            let actual = Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(receiver.clone()),
                },
                present.clone(),
            );
            let success = Expr::new(
                Node::Let {
                    name: binding,
                    value: Box::new(actual),
                    body: Box::new(body),
                },
                ty.clone(),
            );
            (Some(present), Some(success))
        } else {
            (None, None)
        };
        let policy = match &value.ty {
            Type::Option { absence, .. } => absence.as_str(),
            _ => "undefined",
        };
        let description = match (key.as_ref(), key_source) {
            (Some(key), Some(source)) => {
                Some(describe_key(key.clone(), source, source.span.or(span))?)
            }
            _ => None,
        };
        let null_message = property_message("null", field, description.as_ref());
        let undefined_message = property_message("undefined", field, description.as_ref());
        let message = if policy == "both" {
            let tag = present.as_ref().map_or_else(
                || "undefined".to_owned(),
                |ty| {
                    let tag = crate::translation::frontend_rules::read_type_query_result(ty.kind());
                    if tag.is_empty() {
                        "object".to_owned()
                    } else {
                        tag
                    }
                },
            );
            let query = Expr::new(
                Node::OptionalTypeOf {
                    arg: Box::new(receiver.clone()),
                    present: tag,
                },
                STRING,
            );
            let literal = super::text_lit(STRING, "object");
            let condition = Expr::new(
                binary_node(
                    BinaryOp::Eq,
                    Box::new(query),
                    Box::new(literal),
                    Some(STRING),
                    None,
                    None,
                    None,
                ),
                BOOL,
            );
            Expr::new(
                Node::If {
                    cond: Box::new(condition),
                    then: Box::new(null_message),
                    otherwise: Box::new(undefined_message),
                },
                STRING,
            )
        } else if policy == "null" {
            null_message
        } else {
            undefined_message
        };
        let failure=Expr::new(Node::Math{op:crate::translation::source_exception_rules::read_java_script_source_exception_operation("TypeError"),args:vec![message,Expr::lit(BOOL,LitValue::Bool(true))]},ty.clone());
        let mut body = if let (Some(present), Some(success)) = (present, success) {
            let condition = Self::javascript_optional_presence(receiver, &present, span)?;
            Expr::new(
                Node::If {
                    cond: Box::new(condition),
                    then: Box::new(success),
                    otherwise: Box::new(failure),
                },
                ty.clone(),
            )
        } else {
            failure
        };
        if let (Some(name), Some(key)) = (key_name, key) {
            body = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(key),
                    body: Box::new(body),
                },
                ty.clone(),
            );
        }
        let mut entries = vec![RecordEntry {
            name,
            value: value.clone(),
        }];
        if let Some(key) = key_value {
            entries.push(RecordEntry {
                name: key_field,
                value: key,
            });
        }
        let record = Expr::new(Node::Record { entries }, captured_type);
        Ok(Some(Expr::new(
            Node::Let {
                name: capture,
                value: Box::new(record),
                body: Box::new(body),
            },
            ty,
        )))
    }
}
