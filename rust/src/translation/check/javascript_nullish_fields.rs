//! Actual property receiver and type operands into ordinary source error IR.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, Language, LitValue, Node, Result, SExpr, SNode,
    STRING, Span, Type, binary_node,
};
use crate::translation::ir::RecordEntry;
use crate::translation::javascript_nullish_field_rules as rules;
use crate::translation::types::TypeField;

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
        let (value, source) = operand;
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
        let capture = self.fresh_name("ml_nullish_field_receiver");
        let name = rules::read_java_script_nullish_field_capture();
        let captured_type = Type::Record {
            fields: vec![TypeField {
                name: name.clone(),
                ty: value.ty.clone(),
            }],
            open: false,
            reference: false,
        };
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
            let mut scope = env.clone();
            scope.bind_local(&binding, present.clone());
            let read = SExpr::new(
                SNode::Name {
                    path: vec![binding.clone()],
                },
                source.span,
            );
            let field_source = SExpr::new(
                SNode::Field {
                    object: Box::new(read),
                    field: field.to_owned(),
                },
                span,
            );
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
        let null_message = Expr::lit(
            STRING,
            LitValue::Text(rules::read_java_script_nullish_field_message("null", field)),
        );
        let undefined_message = Expr::lit(
            STRING,
            LitValue::Text(rules::read_java_script_nullish_field_message(
                "undefined",
                field,
            )),
        );
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
        let body = if let (Some(present), Some(success)) = (present, success) {
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
        let record = Expr::new(
            Node::Record {
                entries: vec![RecordEntry {
                    name,
                    value: value.clone(),
                }],
            },
            captured_type,
        );
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
