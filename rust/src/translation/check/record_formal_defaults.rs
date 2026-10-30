//! Actual closed-record field facts and existing checked default IR.
use super::{Checker, Env, Expr, Node, Result, SArrayItem, SNode, Span, Type, unsupported};
use crate::translation::javascript_record_parameter_rules::{self as rules, ml_string::Text};
impl Checker {
    pub(super) fn record_formal_default(
        &mut self,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let (value, fallback) = crate::translation::record_formal_defaults::operands(
            args,
            span,
            self.language.as_str(),
        )?;
        let SNode::Field { object, field } = &value.node else {
            unreachable!("actual default field operands")
        };
        let owner = self.expr(object, env, path, None, false)?;
        let (open, present) = match &owner.ty {
            Type::Record { fields, open, .. } => {
                (*open, fields.iter().any(|entry| entry.name == *field))
            }
            _ => (false, false),
        };
        let form = rules::read_java_script_record_formal_default_field_form(
            &Text::from_text(owner.ty.kind()),
            open,
            present,
            &Text::from_text(field),
        )
        .to_string();
        if form == "prototype" {
            return Err(unsupported(
                "record formal inherited property",
                &format!(
                    "an absent own field cannot replace the actual prototype property {field}"
                ),
                span,
            ));
        }
        if form != "absent" {
            return self.javascript_binding_default(&value, fallback, span, env, path);
        }
        let absent = Expr::new(
            Node::None {
                absence: "undefined".to_owned(),
            },
            crate::translation::types::option(Type::Unit, "undefined"),
        );
        let body = self.javascript_binding_default_value(absent, fallback, span, env, path)?;
        let ty = body.ty.clone();
        let name = self.fresh_name("ml_record_default_receiver");
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(owner),
                body: Box::new(body),
            },
            ty,
        ))
    }
}
