//! Actual paths and types project the shared standard primitive default terms.
use super::primitive_try_from::origin;
use super::{
    BOOL, FLOAT, Result, RustParser, SExpr, SNode, Span, rust_fixed_type, type_error, unsupported,
};
use crate::translation::rust_primitive_default_rules as rules;
impl RustParser {
    pub(super) fn project_primitive_default(
        &mut self,
        owner: &str,
        path: &[String],
        range: Option<Span>,
    ) -> Result<SExpr> {
        let term = rules::read_rust_primitive_default_term(owner);
        let ty = match term.as_str() {
            "false" => BOOL,
            "positive floating zero" => FLOAT,
            "integer zero" => rust_fixed_type(owner).ok_or_else(|| {
                type_error("the shared primitive default lost its source type", range)
            })?,
            _ => {
                return Err(type_error(
                    "the shared primitive default lost its actual source term",
                    range,
                ));
            }
        };
        let value = SExpr::new(
            if term == "false" {
                SNode::Bool { value: false }
            } else {
                SNode::Num {
                    value: if term == "positive floating zero" {
                        "0.0"
                    } else {
                        "0"
                    }
                    .to_owned(),
                    ty: Some(ty.clone()),
                    negative: false,
                    unit: false,
                }
            },
            range,
        );
        let mut actual = origin(ty, owner, path);
        if term == "positive floating zero" {
            actual.float_source_model = Some("binary64 scalar");
        }
        self.origins.mark(&value, actual);
        Ok(value)
    }
    pub(in super::super) fn primitive_default_import(
        &self,
        segments: &[String],
        path: &[String],
    ) -> bool {
        let root = segments.first().cloned().unwrap_or_default();
        let shadowed = self.origins.lookup(&[root.clone()], path).is_some()
            || self.origins.scope.contains_key(&root)
            || self
                .aliases
                .get(&path.join("."))
                .is_some_and(|aliases| aliases.contains_key(&root));
        rules::accept_rust_primitive_default_import(
            &segments.join("::"),
            self.ordinary_completion_context(),
            shadowed,
        )
    }
    pub(super) fn read_primitive_default(&mut self, path: &[String]) -> Result<Option<SExpr>> {
        let Some(head) = self.primitive_try_head(path) else {
            return Ok(None);
        };
        if head
            .resolved
            .last()
            .is_none_or(|method| method != "default")
        {
            return Ok(None);
        }
        let trait_alias = self
            .aliases
            .get(&path.join("."))
            .and_then(|aliases| aliases.get("Default"));
        let trait_conflict = self.origins.lookup(&["Default".to_owned()], path).is_some()
            || self.origins.scope.contains_key("Default")
            || trait_alias.is_some_and(|segments| {
                !rules::accept_rust_primitive_default_import(&segments.join("::"), true, false)
            });
        let owner = rules::read_rust_primitive_default_owner(
            &head.resolved,
            self.ordinary_completion_context(),
            head.shadowed || trait_conflict,
            head.absolute,
        );
        if owner.is_empty() {
            return Ok(None);
        }
        let range = Some(Span {
            start: head.start.start,
            end: self.cursor.peek_at(head.offset).end,
        });
        if !self.cursor.is_at("(", head.offset) {
            return Err(unsupported(
                "Rust primitive Default",
                "a primitive default function value needs its own callable interpretation",
                range,
            ));
        }
        for _ in 0..=head.offset {
            self.cursor.advance();
        }
        let args = self.call_arguments(path)?;
        let close = self.cursor.expect(")", Some("primitive default call"))?;
        let range = Some(Span {
            start: head.start.start,
            end: close.end,
        });
        if !args.is_empty() {
            return Err(type_error(
                "a primitive Default call needs no actual arguments",
                range,
            ));
        }
        self.project_primitive_default(&owner, path, range)
            .map(Some)
    }
}
