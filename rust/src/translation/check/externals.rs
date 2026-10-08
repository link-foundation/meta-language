//! The other items of the module, which a program calls and reads by name
//! without declaring them.

use super::{Checker, Env, Expr, External, Node, Param, Result, SExpr, Span, type_error};

impl Checker {
    /// A call of another item's function of the module, or a read of one of
    /// its constants: a `call` of its name.
    pub(super) fn external_application(
        &mut self,
        name: &str,
        segments: &[String],
        args: &[SExpr],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let external = self
            .externals
            .iter()
            .find(|external| external.name() == name)
            .cloned();
        match external {
            Some(External::Function { params, ret, .. }) => {
                let params: Vec<Param> = params
                    .into_iter()
                    .map(|param| Param {
                        name: param.name,
                        ty: param.ty,
                        guard: None,
                        default_value: None,
                    })
                    .collect();
                self.checked_call(name.to_owned(), &params, ret, &[], args, env, path, span)
            }
            Some(External::Constant { ty, .. }) if args.is_empty() => Ok(Expr::new(
                Node::Call {
                    func: name.to_owned(),
                    args: Vec::new(),
                },
                ty,
            )),
            _ => Err(type_error(
                format!("{} is not a function", segments.join(".")),
                span,
            )),
        }
    }
}
