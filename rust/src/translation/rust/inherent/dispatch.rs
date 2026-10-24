//! Actual nominal receiver lookup and ordinary ordered function application.
use super::super::{
    Result, RustParser, SExpr, SNode, SourceDeclaration, Span, Token, Type, unsupported,
};
use crate::translation::rust_inherent_rules::{
    accept_rust_inherent_receiver, require_rust_inherent_receiver_borrow,
};

impl RustParser {
    pub(in super::super) fn inherent_method(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let origin = self.origins.origin(receiver, path);
        let owner = if let Some(Type::Named { path: names, .. }) = &origin.ty {
            self.origins
                .lookup(names, &origin.path)
                .filter(|entry| matches!(entry.declaration, SourceDeclaration::Data))
        } else {
            None
        };
        let method_path = owner.as_ref().map(|owner| {
            owner
                .full_name
                .split('.')
                .map(str::to_owned)
                .chain(std::iter::once(method.value.clone()))
                .collect::<Vec<_>>()
        });
        let selected = method_path
            .as_ref()
            .and_then(|path| self.inherent_methods.get(&path.join(".")))
            .cloned();
        if let Some(selected) = selected.filter(|selected| selected.mode != 0.0) {
            let owner = owner.unwrap();
            let wrappers = f64::from(
                u32::try_from(origin.wrappers.len())
                    .expect("source indirection fits the shared array limit"),
            );
            if !accept_rust_inherent_receiver(
                selected.mode,
                wrappers,
                origin.wrappers.iter().all(|wrapper| wrapper == "reference"),
                &owner.full_name,
                &selected.owner,
            ) {
                return Err(unsupported(
                    "Rust inherent receiver",
                    "the method needs its actual owned or single immutable borrowed receiver",
                    range,
                ));
            }
            let argument = receiver.clone();
            if require_rust_inherent_receiver_borrow(selected.mode, wrappers) {
                self.origins.wrap(&argument, path, "reference");
            }
            let mut values = vec![argument];
            values.extend_from_slice(args);
            let mut rooted = vec!["crate".to_owned()];
            rooted.extend(method_path.unwrap());
            self.origins.call(&rooted, &values, path, range)?;
            return Ok(Some(SExpr::new(
                SNode::App {
                    func: Box::new(SExpr::new(SNode::Name { path: rooted }, range)),
                    args: values,
                },
                range,
            )));
        }
        if self.inherent_discovery {
            let path = method_path
                .unwrap_or_else(|| vec!["unknown_inherent_owner".to_owned(), method.value.clone()]);
            let mut values = vec![receiver.clone()];
            values.extend_from_slice(args);
            return Ok(Some(SExpr::new(
                SNode::App {
                    func: Box::new(SExpr::new(SNode::Name { path }, range)),
                    args: values,
                },
                range,
            )));
        }
        Ok(None)
    }
}
