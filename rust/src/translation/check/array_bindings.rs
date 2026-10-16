//! Existing checked constructors selected by the generated binding decisions.
use super::{
    Checker, Env, Expr, Language, Node, Result, SExpr, SNode, Type, type_error, unsupported,
};
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
                let object = self.reference_value(object);
                if self.language != Language::JavaScript
                    || !rules::accept_java_script_array_rest_receiver(Text::from_text(
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
            } => {
                let left = self.expr(value, env, path, None, false)?;
                let (absence, element) = match &left.ty {
                    Type::Option { absence, element } => (absence.as_str(), Some(element.as_ref())),
                    _ => ("", None),
                };
                let operation = rules::read_java_script_array_default_operation(
                    Text::from_text(self.language.as_str()),
                    Text::from_text(left.ty.kind()),
                    Text::from_text(absence),
                    Text::from_text(element.map_or("", Type::kind)),
                    true,
                );
                if operation.to_string() != "coalesce" {
                    return Err(unsupported(
                        "array binding default",
                        "the selected default needs an undefined-only primitive element read",
                        node.span,
                    ));
                }
                let element = element
                    .expect("the shared default selected an actual optional element")
                    .clone();
                let right = self.expr(fallback, env, path, Some(&element), false)?;
                if rules::read_java_script_array_default_operation(
                    Text::from_text(self.language.as_str()),
                    Text::from_text(left.ty.kind()),
                    Text::from_text(absence),
                    Text::from_text(element.kind()),
                    element.same(&right.ty),
                )
                .to_string()
                    != "coalesce"
                {
                    return Err(type_error(
                        "array binding default and present element need exactly the same type",
                        node.span,
                    ));
                }
                Ok(Expr::new(
                    Node::Coalesce {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    element,
                ))
            }
            _ => unreachable!("the validated array binding has a selected constructor"),
        }
    }
}
