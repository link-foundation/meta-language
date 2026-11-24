//! Actual source values into the already generated closed-record Get policy.
use super::{Checker, Expr, Language, Node, Span, Type};
use crate::translation::javascript_record_parameter_rules::{self as rules, ml_string::Text};
impl Checker {
    pub(super) fn javascript_absent_record_property(
        &mut self,
        value: &Expr,
        field: &str,
        expected: Option<&Type>,
        _span: Option<Span>,
    ) -> Option<Expr> {
        if self.language != Language::JavaScript || self.checked_projection {
            return None;
        }
        let (open, present) = match &value.ty {
            Type::Record { fields, open, .. } => {
                (*open, fields.iter().any(|item| item.name == field))
            }
            _ => (false, false),
        };
        let form = rules::read_java_script_record_formal_default_field_form(
            &Text::from_text(value.ty.kind()),
            open,
            present,
            &Text::from_text(field),
        )
        .to_string();
        if form != "absent" {
            return None;
        }
        let ty=expected.filter(|ty|matches!(ty,Type::Option{absence,..} if crate::translation::structured_type_rules::accept_optional_absence(absence,"undefined"))).cloned().unwrap_or_else(||crate::translation::types::option(Type::Unit,"undefined"));
        Some(Expr::new(
            Node::Let {
                name: self.fresh_name("ml_absent_property_receiver"),
                value: Box::new(value.clone()),
                body: Box::new(Expr::new(
                    Node::None {
                        absence: "undefined".to_owned(),
                    },
                    ty.clone(),
                )),
            },
            ty,
        ))
    }
}
