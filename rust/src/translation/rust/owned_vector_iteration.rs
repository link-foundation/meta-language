//! Retain the source owner across the consuming vector loop adapter.
use super::{
    Result, RustParser, SExpr, SourceOrigin, Span, Token, Type,
    copy_closure_lifetimes::value_borrows, standard_collections::OrderedIterationHead, unsupported,
};
use crate::translation::source_signatures::SourceOwner;

fn fail(reason: &str, span: Option<Span>) -> super::TranslationError {
    unsupported("Rust owned vector source iteration", reason, span)
}

fn is_vector_owner(owner: &SourceOwner) -> bool {
    owner.kind == "library"
        && matches!(
            owner.path.join("::").as_str(),
            "std::vec::Vec" | "alloc::vec::Vec"
        )
        && owner.arguments.len() == 1
}

fn item_origin(
    ty: Type,
    owner: &SourceOwner,
    path: &[String],
    span: Option<Span>,
    depth: usize,
) -> Result<SourceOrigin> {
    if depth > 128 {
        return Err(fail(
            "an owned vector item needs its bounded complete element owner",
            span,
        ));
    }
    if owner.kind == "library"
        && matches!(
            owner.path.join("::").as_str(),
            "std::boxed::Box" | "alloc::boxed::Box"
        )
    {
        return Err(fail(
            "a Box iterator item requires its actual source ownership layers",
            span,
        ));
    }
    let mut origin = SourceOrigin::with_type(ty, path, Vec::new());
    origin.source_owner = Some(owner.clone());
    if !is_vector_owner(owner) {
        origin.array_source_model = None;
    }
    if let Some(Type::Tuple { elements }) = origin.ty.as_ref() {
        if owner.kind != "tuple" || owner.arguments.len() != elements.len() {
            return Err(fail(
                "an owned tuple item needs every actual element position",
                span,
            ));
        }
        origin.fields = Some(
            elements
                .iter()
                .zip(&owner.arguments)
                .map(|(element, owner)| item_origin(element.clone(), owner, path, span, depth + 1))
                .collect::<Result<_>>()?,
        );
    }
    Ok(origin)
}

impl RustParser {
    pub(super) fn owned_vector_source_iteration(
        &mut self,
        receiver: &SExpr,
        path: &[String],
    ) -> Result<Option<(SExpr, SourceOrigin, SourceOrigin)>> {
        if !self.exclusive_source_enabled {
            return Ok(None);
        }
        let actual = self.origins.origin(receiver, path);
        let Some(Type::Array { element, .. }) = actual.ty.as_ref() else {
            return Ok(None);
        };
        let span = receiver.span;
        let owner = actual
            .expanded_source_owner
            .as_ref()
            .or(actual.source_owner.as_ref())
            .filter(|owner| is_vector_owner(owner))
            .ok_or_else(|| {
                fail(
                    "owned iteration needs its actual direct standard Vec owner",
                    span,
                )
            })?;
        if (!self.origins.collecting && !actual.proven)
            || !actual.wrappers.is_empty()
            || actual.source_borrowed_place
            || actual.array_source_model != Some("owned vector")
            || !value_borrows(&actual).is_empty()
            || actual
                .array_element_position
                .as_ref()
                .is_some_and(|position| {
                    !position.wrappers.is_empty()
                        || position.array_element_position.is_some()
                        || position.owner != owner.arguments[0]
                })
        {
            return Err(fail(
                "owned iteration needs its actual direct standard Vec without borrowed element positions or carried loans",
                span,
            ));
        }
        self.origins.check_owned_origin(&actual, span)?;
        let mut item = item_origin(
            self.record_source_type(element, &actual.path),
            &owner.arguments[0],
            path,
            span,
            0,
        )?;
        let container = self.origins.owned_container_facts(&item);
        let ownership = self.origins.standard_sum_ownership_facts(&item);
        if !self.origins.collecting
            && (!container.closed
                || !matches!(container.profile.as_str(), "copy" | "owned")
                || !matches!(ownership.profile.as_str(), "copy" | "owned"))
        {
            return Err(fail(
                "owned iterator items need their actual closed destructor-free ownership graph",
                span,
            ));
        }
        let mut transferred = self.origins.transfer_owned_result(receiver, path)?;
        if !self.origins.collecting
            && (!self.origins.available_owned_binding(&transferred)
                || !transferred.source_owned_result_proof
                || !self.origins.consume_owned_binding(&transferred, span)?)
        {
            return Err(fail(
                "IntoIterator consumes one actual available transferred Vec owner",
                span,
            ));
        }
        item.copy_evidence = (ownership.profile == "copy").then_some(());
        item.source_owned_result_proof = ownership.profile == "owned";
        transferred.source_binding_identity = None;
        transferred.source_place_binding_identity = None;
        transferred.source_owned_result_proof = false;
        transferred.array_source_model = None;
        transferred.source_owner = Some(SourceOwner {
            kind: "library".to_owned(),
            path: vec!["std".to_owned(), "vec".to_owned(), "IntoIter".to_owned()],
            arguments: owner.arguments.clone(),
        });
        transferred.expanded_source_owner = None;
        self.origins.mark(receiver, transferred);
        if self.origins.collecting {
            self.origins.standard_sum_source_needs_final_admission = true;
        }
        Ok(Some((receiver.clone(), item, actual)))
    }

    pub(super) fn read_owned_vector_iteration_method(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        if !self.exclusive_source_enabled
            || self.ordered_iteration_head.is_none()
            || method.value != "into_iter"
            || !matches!(
                self.origins.origin(receiver, path).ty,
                Some(Type::Array { .. })
            )
        {
            return Ok(None);
        }
        self.cursor.expect("(", Some("owned Vec IntoIterator"))?;
        let end = self
            .cursor
            .expect(")", Some("zero-argument owned Vec IntoIterator"))?;
        let Some((mut expression, item, actual)) =
            self.owned_vector_source_iteration(receiver, path)?
        else {
            return Ok(None);
        };
        let origin = self.origins.origin(&expression, path);
        expression.span = receiver.span.map(|start| Span::new(start.start, end.end));
        self.origins.mark(&expression, origin);
        self.ordered_iteration_head
            .as_mut()
            .expect("active ordered loop head")
            .push(OrderedIterationHead {
                expression: expression.clone(),
                item,
                owned_vector: Some(actual),
                end_token: self.cursor.index,
            });
        Ok(Some(expression))
    }
}
