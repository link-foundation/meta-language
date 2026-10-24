//! Actual IR transport for source numeric failures selected by shared rules.
use super::{BOOL, Checker, Expr, FLOAT, Language, LitValue, Node, STRING, Type};
use crate::translation::source_exception_rules as rules;

impl Checker {
    pub(super) fn materialize_numeric_failure(
        &mut self,
        operation: &str,
        args: &[Expr],
        expected: Option<&Type>,
    ) -> Option<Expr> {
        if self.language != Language::JavaScript || args.len() != 2 {
            return None;
        }
        let failure = rules::read_java_script_numeric_operation_failure(
            operation,
            args[0].ty.kind(),
            args[1].ty.kind(),
        );
        if failure.is_empty() {
            return None;
        }
        let ty = expected.cloned().unwrap_or(FLOAT);
        let message = Expr::lit(
            STRING,
            LitValue::Text(rules::read_java_script_numeric_operation_message(&failure)),
        );
        let mut body = Expr::new(
            Node::Math {
                op: "raiseJavaScriptTypeError".to_owned(),
                args: vec![message, Expr::lit(BOOL, LitValue::Bool(true))],
            },
            ty.clone(),
        );
        let names = args
            .iter()
            .map(|_| self.fresh_name("ml_numeric_failure_operand"))
            .collect::<Vec<_>>();
        for (name, value) in names.into_iter().zip(args.iter()).rev() {
            body = Expr::new(
                Node::Let {
                    name,
                    value: Box::new(value.clone()),
                    body: Box::new(body),
                },
                ty.clone(),
            );
        }
        Some(body)
    }
}
