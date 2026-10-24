//! Source pattern/ownership transport into one ordinary match per nominal binding.
use super::super::{
    Binding, Result, RustParser, SExpr, SNode, SPattern, SPatternNode, SRow, SourceDeclaration,
    Span, TokenKind, unsupported,
};
use crate::translation::rust_record_rules::accept_rust_irrefutable_pattern_node;

impl RustParser {
    pub(in super::super) fn nominal_binding_pattern(&mut self) -> Result<Option<SPattern>> {
        if self.cursor.peek().kind != TokenKind::Identifier {
            return Ok(None);
        }
        let first = vec![self.cursor.peek().value.clone()];
        let candidate = self
            .origins
            .lookup(&self.resolve(&first, &self.module_path), &self.module_path);
        let known = candidate.is_some_and(|item| {
            matches!(
                item.declaration,
                SourceDeclaration::Data | SourceDeclaration::Constructor { .. }
            )
        });
        if !["::", "(", "{"].contains(&self.cursor.peek_at(1).value.as_str()) && !known {
            return Ok(None);
        }
        let mut aliases = Vec::new();
        let path = self.module_path.clone();
        self.record_pattern_depth += 1;
        let result = self.pattern(&path, &mut aliases);
        self.record_pattern_depth -= 1;
        let pattern = result?;
        if !aliases.is_empty() {
            return Err(unsupported(
                "Rust nominal binding alias",
                "whole-value aliases require their complete move and borrow interpretation",
                pattern.span,
            ));
        }
        Ok(Some(pattern))
    }

    pub(in super::super) fn nominal_binding_names(
        &self,
        pattern: &SPattern,
        validate: bool,
    ) -> Result<Vec<(String, Option<Span>)>> {
        let (kind, entry, args) = match &pattern.node {
            SPatternNode::Ctor { path, args } => (
                "ctor",
                self.origins.lookup(path, &self.module_path),
                args.as_slice(),
            ),
            SPatternNode::BindOrCtor { name } => (
                "bind",
                self.origins.lookup(&[name.clone()], &self.module_path),
                &[][..],
            ),
            SPatternNode::Bind { .. } => ("bind", None, &[][..]),
            SPatternNode::Tuple { args, .. } => ("tuple", None, args.as_slice()),
            SPatternNode::Wild => ("wild", None, &[][..]),
            _ => ("refutable", None, &[][..]),
        };
        let constructor =
            entry.filter(|item| matches!(item.declaration, SourceDeclaration::Constructor { .. }));
        let (variants, fields) = if let Some(item) = &constructor {
            let SourceDeclaration::Constructor { owner, fields, .. } = &item.declaration else {
                unreachable!()
            };
            (self.origins.catalog.values().filter(|candidate| matches!(candidate, SourceDeclaration::Constructor { owner: actual, .. } if actual == owner)).count(), fields.len())
        } else {
            (0, 0)
        };
        let kind = if constructor.is_some() { "ctor" } else { kind };
        let count = |value| {
            f64::from(
                u32::try_from(value).expect("source pattern count fits the shared array limit"),
            )
        };
        if validate
            && !self.origins.collecting
            && !accept_rust_irrefutable_pattern_node(
                kind,
                constructor.is_some(),
                count(variants),
                count(args.len()),
                count(fields),
            )
        {
            return Err(unsupported(
                "refutable product binding",
                "only immutable names, ignored fields and anonymous tuple products are admitted",
                pattern.span,
            ));
        }
        let mut names = Vec::new();
        if kind == "bind" {
            if let SPatternNode::Bind { name } | SPatternNode::BindOrCtor { name } = &pattern.node {
                names.push((name.clone(), pattern.span));
            }
        }
        for item in args {
            names.extend(self.nominal_binding_names(item, validate)?);
        }
        Ok(names)
    }

    pub(in super::super) fn lower_nominal_binding(
        &mut self,
        pattern: SPattern,
        value: SExpr,
        path: &[String],
        bindings: &mut Vec<Binding>,
    ) -> Result<()> {
        let names = self.nominal_binding_names(&pattern, true)?;
        let scope = self
            .origins
            .cases(std::slice::from_ref(&pattern), &value, path)?;
        let range = pattern.span;
        let members = names
            .iter()
            .map(|(name, span)| {
                SExpr::new(
                    SNode::Name {
                        path: vec![name.clone()],
                    },
                    *span,
                )
            })
            .collect::<Vec<_>>();
        let body = match members.len() {
            0 => SExpr::new(SNode::Unit, range),
            1 => members[0].clone(),
            _ => SExpr::new(
                SNode::Tuple {
                    items: members,
                    frozen: false,
                    product_model: None,
                },
                range,
            ),
        };
        let matched = SExpr::new(
            SNode::Match {
                scrutinees: vec![value],
                rows: vec![SRow {
                    patterns: vec![pattern],
                    body,
                    span: range,
                }],
            },
            range,
        );
        let bundle = self.fresh_product_binding();
        bindings.push(Binding {
            name: bundle.clone(),
            ty: None,
            value: matched,
            span: range,
        });
        let single = names.len() == 1;
        for (index, (name, range)) in names.into_iter().enumerate() {
            let object = SExpr::new(
                SNode::Name {
                    path: vec![bundle.clone()],
                },
                range,
            );
            let projection = if single {
                object
            } else {
                SExpr::new(
                    SNode::TupleIndex {
                        object: Box::new(object),
                        index,
                    },
                    range,
                )
            };
            if let Some(origin) = scope.get(&name) {
                self.origins.mark(&projection, origin.clone());
            }
            self.origins.bind(&name, &projection, path, None)?;
            bindings.push(Binding {
                name,
                ty: None,
                value: projection,
                span: range,
            });
        }
        Ok(())
    }
}
