//! Render typed callable fields using the checked shared source kernels.

use super::{Expr, Result, RustEmitter, Type, own};
use crate::translation::callable_source_rules as rules;
use crate::translation::ir::LambdaParam;
use crate::translation::surface_reference_facts::create_surface_reference_facts;

impl RustEmitter<'_> {
    pub(super) fn callable_lambda(
        &mut self,
        expression: &Expr,
        params: &[LambdaParam],
        body: &Expr,
    ) -> Result<String> {
        let value = serde_json::to_value(body).expect("a checked lambda body is serializable");
        let facts = create_surface_reference_facts(&value);
        let names = params
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect::<Vec<_>>();
        let plan = rules::read_callable_free_references(
            &facts.forms,
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.root,
            &names,
            true,
        );
        let selected = crate::translation::callable_emission_rules::plan_callable_capture_bindings(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            &plan,
        );
        let mut captures: Vec<(String, Type)> = Vec::new();
        for identity in selected {
            let node = facts
                .value(identity)
                .expect("a capture plan retains an actual checked variable");
            let name = node["name"]
                .as_str()
                .expect("a checked variable has a name");
            let ty: Type = serde_json::from_value(node["type"].clone())
                .expect("a checked variable has its actual type");
            captures.push((name.to_owned(), ty));
        }
        let copies: Vec<String> = captures
            .iter()
            .map(|(name, ty)| format!("let {name} = {};", own(name, ty)))
            .collect();
        let types = params
            .iter()
            .map(|parameter| self.ty(&parameter.ty))
            .collect::<Result<Vec<_>>>()?;
        let Type::Function { ret, .. } = &expression.ty else {
            unreachable!("a checked Lambda has its Function type")
        };
        let ret = self.ty(ret)?;
        let body = self.expr(body)?;
        Ok(rules::render_callable_lambda(
            "Rust", &names, &types, &ret, &body, &copies,
        ))
    }
}
