//! Pure queries and explicit evaluation of known JavaScript operands.
use super::{
    Checker, Env, Expr, External, Language, Node, Result, SExpr, STRING, Span, Type, text_lit,
    unsupported,
};
use crate::translation::frontend_rules::{
    accept_checked_type_query_operand, accept_type_query_operand, read_type_query_result,
};

impl Checker {
    pub(super) fn type_query(
        &mut self,
        source: &SExpr,
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let arg = self.expr(source, env, path, None, false)?;
        if let Type::Option { element, .. } = &arg.ty {
            let present = if **element == Type::Unit && self.language == Language::JavaScript {
                "undefined".to_owned()
            } else {
                read_type_query_result(element.kind())
            };
            if present.is_empty() {
                return Err(unsupported(
                    &format!("typeof {}", arg.ty.key()),
                    "type queries require a known JavaScript value type",
                    span,
                ));
            }
            return Ok(Expr::new(
                Node::OptionalTypeOf {
                    arg: Box::new(arg),
                    present,
                },
                STRING,
            ));
        }
        let constant_reference = match &arg.node {
            Node::Call { func, .. } => self.externals.iter().any(
                |external| matches!(external, External::Constant { name, .. } if name == func),
            ),
            _ => false,
        };
        let mut value = read_type_query_result(arg.ty.kind());
        let plan = crate::translation::type_query_evaluation_rules::read_type_query_evaluation_plan(
            self.language == Language::JavaScript,
            !value.is_empty(),
            accept_type_query_operand(source.node.kind()),
            accept_checked_type_query_operand(arg.node.kind(), constant_reference),
            arg.ty == Type::Unit,
        );
        if plan == 3.0 {
            value = "undefined".to_owned();
        }
        if plan == 0.0 {
            return Err(unsupported(
                &format!("typeof {}", arg.ty.key()),
                "type queries require a known JavaScript value type",
                span,
            ));
        }
        let body = text_lit(STRING, value);
        Ok(if plan == 1.0 {
            body
        } else {
            Expr::new(
                Node::Let {
                    name: self.fresh_name("ml_type_query_operand"),
                    value: Box::new(arg),
                    body: Box::new(body),
                },
                STRING,
            )
        })
    }
}
