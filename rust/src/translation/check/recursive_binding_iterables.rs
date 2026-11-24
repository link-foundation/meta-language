//! Typed source operands into the existing iterator and exception operations.
use super::{
    BOOL, Checker, Env, Expr, Language, LitValue, Node, Result, SArrayItem, STRING, Span, Type,
    type_error, unsupported,
};
use crate::translation::recursive_binding_rules::{self as rules, ml_string::Text};
impl Checker {
    fn recursive_binding_present_iterable(value: Expr, span: Option<Span>) -> Result<Expr> {
        if matches!(value.ty, Type::Tuple { .. }) {
            Ok(value)
        } else {
            Self::for_of_iterable_value(value, span)
        }
    }
    pub(super) fn recursive_binding_iterable(
        &mut self,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript || args.len() != 1 || args[0].spread {
            return Err(type_error(
                "a recursive binding iterator needs one actual operand",
                span,
            ));
        }
        let value = self.expr(&args[0].value, env, path, None, false)?;
        let element = match &value.ty {
            Type::Option { element, .. } => element.kind(),
            _ => "",
        };
        let form = rules::read_java_script_recursive_binding_iterable_form(
            &Text::from_text(value.ty.kind()),
            &Text::from_text(element),
        )
        .to_string();
        if form == "wait" || form == "refuse" {
            return Err(unsupported(
                "recursive binding iterator",
                "the actual source value needs its complete operational iterator",
                span,
            ));
        }
        if form != "optional" {
            return Self::recursive_binding_present_iterable(value, span);
        }
        let Type::Option { element, absence } = &value.ty else {
            unreachable!("shared optional iterable selection")
        };
        let present = element.as_ref().clone();
        let absence = absence.clone();
        let name = self.fresh_name("ml_recursive_binding_iterable");
        let bound = Expr::new(Node::Var { name: name.clone() }, value.ty.clone());
        let success = Self::recursive_binding_present_iterable(
            Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(bound.clone()),
                },
                present.clone(),
            ),
            span,
        )?;
        let message = |absence: &str| {
            super::text_lit(
                STRING,
                rules::read_java_script_recursive_binding_iterable_error(&Text::from_text(absence))
                    .to_string(),
            )
        };
        let mut text = message(if absence == "null" {
            "null"
        } else {
            "undefined"
        });
        if absence == "both" {
            let condition = Expr::new(
                Node::StrictEqual {
                    left: Box::new(bound.clone()),
                    right: Box::new(Expr::new(
                        Node::None {
                            absence: "null".to_owned(),
                        },
                        value.ty.clone(),
                    )),
                    negate: false,
                },
                BOOL,
            );
            text = Expr::new(
                Node::If {
                    cond: Box::new(condition),
                    then: Box::new(message("null")),
                    otherwise: Box::new(text),
                },
                STRING,
            );
        }
        let ty = success.ty.clone();
        let failure=Expr::new(Node::Math{op:crate::translation::source_exception_rules::read_java_script_source_exception_operation("TypeError"),args:vec![text,Expr::new(Node::Lit{value:LitValue::Bool(true)},BOOL)]},ty.clone());
        let condition = Self::javascript_optional_presence(bound, &present, span)?;
        let body = Expr::new(
            Node::If {
                cond: Box::new(condition),
                then: Box::new(success),
                otherwise: Box::new(failure),
            },
            ty.clone(),
        );
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            ty,
        )
        .with_span(span))
    }
}
