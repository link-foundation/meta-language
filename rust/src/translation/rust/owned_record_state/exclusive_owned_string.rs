//! Checked String mutations reuse the ordered mutable state transport.
use super::super::{BinaryOp, STRING, Token};
use super::{Result, RustParser, SExpr, SNode, SourceOrigin, Span, fail, rules};
use crate::translation::rust_exclusive_source_string_rules as string_rules;

impl RustParser {
    pub(in super::super) fn read_exclusive_owned_string_mutation(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        if !self.exclusive_source_enabled {
            return Ok(None);
        }
        let arity = string_rules::read_rust_exclusive_source_string_method_arity(&method.value);
        if arity < 0.0 {
            return Ok(None);
        }
        let actual = self.origins.origin(receiver, path);
        if actual.ty != Some(STRING) {
            return Ok(None);
        }
        let span = Some(Span::new(
            receiver.span.map_or(method.start, |span| span.start),
            method.end,
        ));
        let owner = actual
            .expanded_source_owner
            .as_ref()
            .or(actual.source_owner.as_ref());
        if !self.origins.collecting
            && !string_rules::accept_rust_exclusive_source_string_receiver(
                "string",
                actual.text_source_model.unwrap_or(""),
                actual.wrappers.len() as f64,
                actual.proven,
                owner.map_or("", |owner| owner.kind.as_str()),
                &owner.map_or_else(String::new, |owner| owner.path.join("::")),
                owner.map_or(-1.0, |owner| owner.arguments.len() as f64),
            )
        {
            return Err(fail(
                "String mutation requires an actual direct standard owned String receiver",
                span,
            ));
        }

        let place = super::super::owned_record_loops::place(self, receiver).ok_or_else(|| {
            fail(
                "String mutation requires its actual mutable lexical place",
                span,
            )
        })?;
        let slot = place.slot.clone();
        let field = place.origin.clone();
        let indices = place.indices;
        let source_fields = self.record_state_layout_indices(&slot.origin, &indices, span)?;
        let target = rules::read_rust_owned_record_state_field_identity(
            &slot.root,
            &indices
                .iter()
                .map(|index| *index as f64)
                .collect::<Vec<_>>(),
        );
        self.cursor.expect("(", Some("String mutation"))?;
        let temporaries = self.origins.owned_temporary_frame();
        let suffix = if arity == 1.0 {
            let suffix = self.with_standard_sum_source_context(None, |parser| {
                parser.expr(path, Default::default())
            })?;
            let mut origin = self.origins.origin(&suffix, path);
            let owner = origin
                .expanded_source_owner
                .clone()
                .or(origin.source_owner.clone())
                .unwrap_or_else(|| {
                    self.named_source_type_owner(
                        &[if origin.text_source_model == Some("string slice") {
                            "str"
                        } else {
                            "String"
                        }
                        .to_owned()],
                        Vec::new(),
                    )
                });
            if origin.source_owner.is_none() {
                origin.source_owner = Some(owner.clone());
            }
            if !self.origins.collecting
                && !string_rules::accept_rust_exclusive_source_string_suffix(
                    origin.ty.as_ref().map_or("", super::Type::kind),
                    origin.text_source_model.unwrap_or(""),
                    origin.wrappers.len() as f64,
                    origin.proven,
                    origin.wrappers.iter().all(|layer| layer == "reference"),
                    &owner.kind,
                    &owner.path.join("::"),
                    owner.arguments.len() as f64,
                )
            {
                return Err(fail(
                    "push_str requires one actual standard shared str or String argument",
                    suffix.span,
                ));
            }
            self.origins.check_owned_origin(&origin, suffix.span)?;
            self.cursor.eat(",");
            Some((suffix, origin))
        } else {
            None
        };
        let end = self.cursor.expect(")", Some("String mutation"))?;
        let span = span.map(|span| Span::new(span.start, end.end));
        if !self.origins.collecting
            && (slot.callable != self.owned_record_state.current_callable
                || self.record_state_overlaps(&slot, &target)
                || super::super::owned_record_updates::field_state(
                    &self.origins.snapshot_owned_bindings(),
                    &target,
                ) != "available")
        {
            return Err(fail(
                "String mutation requires an available source place with no overlapping loan",
                span,
            ));
        }
        if !source_fields.is_empty() && slot.source_binder.is_some() {
            let result = self.owned_record_effect_string_replacement(
                super::CollectionPlace {
                    slot,
                    layouts: source_fields,
                    target,
                },
                field,
                suffix,
                path,
                span,
            )?;
            self.origins
                .finish_owned_expression(&result, path, &temporaries, false);
            return Ok(Some(result));
        }
        let mut origin = SourceOrigin::with_type(STRING, path, Vec::new());
        origin.source_owner = field.source_owner.clone();
        origin.expanded_source_owner = field.expanded_source_owner.clone();
        let value = if let Some((suffix, _)) = suffix {
            let suffix_name = self.fresh_product_binding();
            let prior = self.owned_record_effect_read(slot.clone(), span)?;
            let current = self.project_record_state(&source_fields, prior, span);
            let mut append = SExpr::new(
                SNode::Binary {
                    op: BinaryOp::Concat,
                    left: Box::new(current),
                    right: Box::new(super::name(&suffix_name)),
                    rounding: None,
                },
                span,
            );
            self.origins.mark_owned_string_constructor(&append, origin);
            self.note_borrowed_expression_source_position(&mut append, path)?;
            SExpr::new(
                SNode::Let {
                    name: suffix_name,
                    ty: Some(STRING.into()),
                    value: Box::new(suffix),
                    body: Box::new(append),
                },
                span,
            )
        } else {
            let mut empty = SExpr::new(
                SNode::Str {
                    value: String::new(),
                },
                span,
            );
            self.origins.mark_owned_string_constructor(&empty, origin);
            self.note_borrowed_expression_source_position(&mut empty, path)?;
            empty
        };
        let result =
            self.owned_record_effect_write(slot, source_fields, field, target, value, path, span)?;
        self.origins
            .finish_owned_expression(&result, path, &temporaries, false);
        Ok(Some(result))
    }
}
