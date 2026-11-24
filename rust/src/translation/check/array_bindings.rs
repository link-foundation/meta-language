//! Existing checked constructors selected by the generated binding decisions.
use super::{Checker, Env, Expr, Language, Result, SExpr, SNode, unsupported};
use crate::translation::array_binding_rules::{self as rules, ml_string::Text};
use crate::translation::surface::array_bindings::{offset, validate};

impl Checker {
    pub(super) fn array_binding(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        validate(node)?;
        match &node.node {
            SNode::ArrayBindingRest { object, start, .. } => {
                let object = self.expr(object, env, path, None, false)?;
                let object = Self::reference_value(object);
                if self.language != Language::JavaScript
                    || !rules::accept_java_script_array_rest_receiver(&Text::from_text(
                        object.ty.kind(),
                    ))
                {
                    return Err(unsupported(
                        "array rest binding",
                        "the selected rest binding needs an actual homogeneous array receiver",
                        node.span,
                    ));
                }
                let start = offset(start, node.span)?;
                self.primitive_method_on_checked_receiver(
                    object,
                    "slice",
                    &[Some(start)],
                    env,
                    path,
                    node.span,
                )
            }
            SNode::ArrayBindingDefault {
                value, fallback, ..
            } => self.javascript_binding_default(value, fallback, node.span, env, path),
            _ => unreachable!("the validated array binding has a selected constructor"),
        }
    }
}
