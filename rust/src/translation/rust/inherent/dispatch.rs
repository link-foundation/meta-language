//! Actual nominal receiver lookup and ordinary ordered function application.
use super::super::{
    Result, RustParser, SExpr, SNode, SourceDeclaration, Span, Token, Type, unsupported,
};
use crate::translation::rust_inherent_rules::plan_rust_inherent_receiver_layers;

impl RustParser {
    pub(in super::super) fn capture_inherent_receiver_value_loan(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<()> {
        if self.origins.collecting{return Ok(());}
        let origin=self.origins.origin(receiver,path);
        let Some(Type::Named{path:names,..})=origin.ty.as_ref() else{return Ok(());};
        let Some(owner)=self.origins.lookup(names,&origin.path).filter(|entry|matches!(entry.declaration,SourceDeclaration::Data))else{return Ok(());};
        let Some(selected)=self.inherent_methods.get(&format!("{}.{}",owner.full_name,method.value)).filter(|selected|selected.mode!=0.0&&selected.owner==owner.full_name)else{return Ok(());};
        let plan=plan_rust_inherent_receiver_layers(&origin.wrappers,&selected.receiver);if plan.first()!=Some(&1.0){return Ok(());}
        let wrappers=plan.iter().skip(1).map(|layer|if *layer==1.0{"reference".into()}else{"box".into()}).collect::<Vec<String>>();if !wrappers.iter().any(|layer|layer=="reference"){return Ok(());}
        let mut borrowed=origin.clone();borrowed.wrappers=wrappers;let borrowed=self.origins.capture_mutable_record_value_loans(borrowed,receiver.span)?;
        let mut actual=origin;actual.source_value_borrow_bindings=borrowed.source_value_borrow_bindings;self.origins.mark(receiver,actual);Ok(())
    }
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
            let plan = plan_rust_inherent_receiver_layers(&origin.wrappers, &selected.receiver);
            if owner.full_name != selected.owner || plan.first() != Some(&1.0) {
                return Err(unsupported(
                    "Rust inherent receiver",
                    "the method needs its actual nominal receiver and supported immutable dereference or borrow layers",
                    range,
                ));
            }
            let mut argument = receiver.clone();
            let wrappers = plan
                .iter()
                .skip(1)
                .map(|layer| {
                    if *layer == 1.0 {
                        "reference".to_owned()
                    } else {
                        "box".to_owned()
                    }
                })
                .collect();
            let origin=self.prepare_borrowed_inherent_receiver_position(origin,wrappers,range)?;
            self.origins.mark(&argument, origin);
            self.note_borrowed_expression_source_position(&mut argument,path)?;
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
