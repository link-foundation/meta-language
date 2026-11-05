//! Actual source App heads into the existing computed application pipeline.
use super::{Checker, Entry, Env, External, Result, SExpr, SNode, Span, Type};
use crate::translation::rust_module_value_rules as rules;
impl Checker {
    pub(super) fn capture_callable_constant_application_head(
        &mut self,
        head: &SExpr,
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if self.checked_projection {
            return Ok(None);
        }
        let local = env.local(head).is_some();
        let mut constant = None;
        if let SNode::Name { path: names } = &head.node
            && !local
            && let Some(Entry::External(name)) = self.lookup(names, path, span)?
        {
            constant = self.externals.iter().find_map(|entry| match entry {
                External::Constant {
                    name: actual, ty, ..
                } if actual == &name => Some(ty.clone()),
                _ => None,
            });
        }
        if !rules::require_callable_constant_application_capture(
            head.node.kind(),
            local,
            if constant.is_some() { "const" } else { "" },
            constant.as_ref().map_or("", Type::kind),
        ) {
            return Ok(None);
        }
        let name = self.fresh_name("ml_callable_constant");
        Ok(Some(SExpr::new(
            SNode::Let {
                name: name.clone(),
                ty: None,
                value: Box::new(head.clone()),
                body: Box::new(SExpr::new(SNode::Name { path: vec![name] }, span)),
            },
            span,
        )))
    }
}
