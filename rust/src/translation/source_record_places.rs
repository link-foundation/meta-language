//! Structural checks for actual Rust declaration-order field places.
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::diagnostics::{Result, type_error};
use super::ir::{DataDecl, Expr, Node, Pattern};
use super::rust_record_place_owner_rules as rules;
use super::source_expression_positions::check_position;
use super::source_loop_folds::SourceFoldSpan;
use super::source_signatures::{SourceOwner, SourceTypeSignature, source_policy_column};
use super::types::Type;

/// The original parent position and actual selected field declaration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RustSourceRecordPlace {
    pub model: String,
    pub root: SourceTypeSignature,
    pub field: usize,
    pub value: SourceTypeSignature,
    pub span: SourceFoldSpan,
}

fn canonical_box(owner: &SourceOwner) -> bool {
    super::rust_owned_recursive_datatype_rules::accept_rust_owned_box_type_node(
        source_policy_column(&["box".into()]),
        &owner.kind,
        &owner.path.join("::"),
        owner.arguments.len() as f64,
    )
}

pub(super) fn owner_view(
    position: &SourceTypeSignature,
    allow_references: bool,
) -> Option<(&SourceOwner, usize)> {
    if position.wrappers.len() > 256 {
        return None;
    }
    let mut owner = &position.owner;
    let mut boxes = Vec::new();
    for layer in &position.wrappers {
        if layer == "box" {
            if !canonical_box(owner) {
                return None;
            }
            boxes.push(true);
            owner = &owner.arguments[0];
        }
    }
    let references = rules::read_rust_record_place_reference_count(
        &position.wrappers,
        position.lifetimes.len() as f64,
        allow_references,
        &boxes,
        canonical_box(owner),
    );
    (references >= 0.0).then_some((owner, references as usize))
}

pub(super) fn box_projection_depth(
    source: &SourceTypeSignature,
    target: &SourceTypeSignature,
) -> Option<usize> {
    owner_view(source, false)?;
    owner_view(target, false)?;
    if source.array_element_position.is_some() || target.array_element_position.is_some() {
        return None;
    }
    let depth = source.wrappers.len().checked_sub(target.wrappers.len())?;
    let mut owner = &source.owner;
    for _ in 0..depth {
        owner = &owner.arguments[0];
    }
    let mut pending = vec![(owner, &target.owner)];
    let mut visits = 0;
    while let Some((left, right)) = pending.pop() {
        visits += 1;
        if visits > 65_536
            || left.kind != right.kind
            || left.path != right.path
            || left.arguments.len() != right.arguments.len()
        {
            return None;
        }
        pending.extend(left.arguments.iter().zip(&right.arguments));
    }
    Some(depth)
}

fn plain(node: &Expr) -> bool {
    node.source_position.is_none()
        && node.source_state_transition.is_none()
        && node.source_loop_fold.is_none()
        && node.source_record_place.is_none()
}

pub(super) fn check_record_place<'node, 'data>(
    facet: &RustSourceRecordPlace,
    node: &'node Expr,
    language: &str,
    mut resolve_type: impl FnMut(&Type) -> Result<Type>,
    mut resolve_data: impl FnMut(&Type) -> Option<&'data DataDecl>,
) -> Result<(RustSourceRecordPlace, &'node Expr)> {
    if !rules::accept_rust_source_record_place(
        language,
        &facet.model,
        facet.field as f64,
        facet.span.start as f64,
        facet.span.end as f64,
    ) || !node
        .span
        .is_some_and(|span| span.start == facet.span.start && span.end == facet.span.end)
        || node.source_state_transition.is_some()
        || node.source_loop_fold.is_some()
    {
        return Err(type_error(
            "a source record place retains its exact model, original field span and actual declaration index",
            node.span,
        ));
    }
    let invalid_projection = || {
        type_error(
            "a source record place retains its actual one-constructor projection",
            node.span,
        )
    };
    let mut projected = node;
    let mut original = None;
    if let Node::Let { name, value, body } = &node.node {
        let Node::Match { scrutinee, .. } = &body.node else {
            return Err(invalid_projection());
        };
        if !plain(body)
            || !plain(scrutinee)
            || !matches!(&scrutinee.node, Node::Var { name: actual } if actual == name)
            || scrutinee.ty != value.ty
        {
            return Err(invalid_projection());
        }
        projected = body;
        original = Some(value.as_ref());
    }
    let Node::Match { scrutinee, cases } = &projected.node else {
        return Err(invalid_projection());
    };
    let [row] = cases.as_slice() else {
        return Err(invalid_projection());
    };
    let base = original.unwrap_or(scrutinee.as_ref());
    let root = check_position(
        &facet.root,
        &base.ty,
        language,
        &mut resolve_type,
        &mut resolve_data,
    )?;
    let value = check_position(
        &facet.value,
        &node.ty,
        language,
        &mut resolve_type,
        &mut resolve_data,
    )?;
    let invalid_owner = || {
        type_error(
            "a source field place retains its actual complete closed record owner and declared field",
            node.span,
        )
    };
    if root.array_element_position.is_some()
        || owner_view(&root, true).is_none()
        || owner_view(&value, false).is_none()
    {
        return Err(invalid_owner());
    }
    let data = resolve_data(&root.ty).ok_or_else(invalid_owner)?;
    let [constructor] = data.ctors.as_slice() else {
        return Err(invalid_owner());
    };
    let fields = data
        .source_field_signatures
        .as_ref()
        .and_then(|signature| signature.ctors.first())
        .filter(|entry| entry.fields.len() == constructor.fields.len())
        .ok_or_else(invalid_owner)?;
    if value.array_element_position.is_some() || fields.fields.get(facet.field) != Some(&value) {
        return Err(invalid_owner());
    }
    let Pattern::Ctor {
        data: selected,
        ctor,
        binds,
    } = &row.pattern
    else {
        return Err(invalid_projection());
    };
    let binding = binds.get(facet.field).and_then(Option::as_ref);
    let mut unique = HashSet::new();
    if selected != &data.full_name
        || ctor != &constructor.name
        || binds.len() != fields.fields.len()
        || !binds
            .iter()
            .flatten()
            .all(|name| !name.is_empty() && unique.insert(name))
        || !matches!(&row.body.node, Node::Var { name } if Some(name) == binding)
        || !plain(&row.body)
        || projected.ty != value.ty
    {
        return Err(invalid_projection());
    }
    Ok((
        RustSourceRecordPlace {
            model: facet.model.clone(),
            root,
            field: facet.field,
            value,
            span: facet.span.clone(),
        },
        base,
    ))
}
