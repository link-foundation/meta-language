//! Type queries over known bindings and literal operands.
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
        let hint = "type queries currently require a bound value or literal; evaluating other operands must retain their effects and exceptions";
        let arg = self.expr(source, env, path, None, false)?;
        if let Type::Option { element, .. } = &arg.ty {
            let present = if **element == Type::Unit && self.language == Language::JavaScript {
                "undefined".to_owned()
            } else {
                read_type_query_result(element.kind())
            };
            return Ok(Expr::new(
                Node::OptionalTypeOf {
                    arg: Box::new(arg),
                    present,
                },
                STRING,
            ));
        }
        if !accept_type_query_operand(source.node.kind()) {
            return Err(unsupported("typeof operand", hint, span));
        }
        let constant_reference = match &arg.node {
            Node::Call { func, .. } => self.externals.iter().any(
                |external| matches!(external, External::Constant { name, .. } if name == func),
            ),
            _ => false,
        };
        if !accept_checked_type_query_operand(arg.node.kind(), constant_reference) {
            return Err(unsupported("typeof operand", hint, span));
        }
        let value = read_type_query_result(arg.ty.kind());
        if value.is_empty() {
            return Err(unsupported(
                &format!("typeof {}", arg.ty.key()),
                "type queries require a known JavaScript value type",
                span,
            ));
        }
        Ok(text_lit(STRING, value))
    }
}
