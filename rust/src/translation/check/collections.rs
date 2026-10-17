//! Checked-value transport for the complete shared collection construction plans.

use super::{Checker, Env, Expr, Node, Result, SExpr, SNode, Type, array, type_error, unsupported};
use crate::translation::collection_construction_rules as rules;
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::optional_value_rules::accept_optional_primitive_kind;
use crate::translation::surface_reference_facts::{
    SurfaceReferenceFacts, create_surface_reference_facts,
};

fn selected_type(facts: &SurfaceReferenceFacts<'_>, identity: f64) -> Type {
    serde_json::from_value(
        facts
            .value(identity)
            .expect("the collection plan selects an actual type")
            .clone(),
    )
    .expect("the retained collection type is serializable")
}

impl Checker {
    pub(super) fn collection_construction(
        &mut self,
        node: &SExpr,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        let (items, declared) = match &node.node {
            SNode::Set { items, ty } => (items.as_ref(), ty.as_ref()),
            SNode::Map { entries, ty } => (entries.as_ref(), ty.as_ref()),
            _ => unreachable!("only actual collection constructors reach this adapter"),
        };
        let want = if declared.is_some() {
            Some(self.resolve_type(declared, path, node.span)?)
        } else {
            expected.cloned()
        };
        let value =
            serde_json::to_value(want).expect("an expected collection type is serializable");
        let facts = create_surface_reference_facts(&value);
        let context = rules::plan_collection_construction_inference(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            node.node.kind(),
            facts.root,
        );
        let form = number_index(context[0]);
        let expected_items = if number_index(context[1]) == Some(1) {
            let first = selected_type(&facts, context[2]);
            Some(array(match form {
                Some(1) => first,
                Some(2) => Type::Tuple {
                    elements: vec![first, selected_type(&facts, context[3])],
                },
                _ => unreachable!("the collection plan selects a constructor form"),
            }))
        } else {
            None
        };
        let items = self.expr(items, env, path, expected_items.as_ref(), false)?;
        let value =
            serde_json::to_value(&items.ty).expect("a checked collection input is serializable");
        let facts = create_surface_reference_facts(&value);
        let plan = rules::plan_collection_construction_checked(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            node.node.kind(),
            facts.root,
        );
        if number_index(plan[0]) != Some(0) {
            return Err(type_error(
                rules::read_collection_construction_diagnostic(plan[0], "message"),
                node.span,
            ));
        }
        let first = selected_type(&facts, plan[1]);
        let error = rules::read_collection_construction_key_admission(
            node.node.kind(),
            accept_optional_primitive_kind(first.kind()),
        );
        if number_index(error) != Some(0) {
            return Err(unsupported(
                &rules::read_collection_construction_diagnostic(error, "construct"),
                &rules::read_collection_construction_diagnostic(error, "message"),
                node.span,
            ));
        }
        let (node, ty) = match form {
            Some(1) => (
                Node::Set {
                    items: Box::new(items),
                },
                Type::Set {
                    element: Box::new(first),
                    reference: false,
                },
            ),
            Some(2) => (
                Node::Map {
                    entries: Box::new(items),
                },
                Type::Map {
                    key: Box::new(first),
                    value: Box::new(selected_type(&facts, plan[2])),
                    reference: false,
                },
            ),
            _ => unreachable!("the collection plan selects a constructor form"),
        };
        Ok(Expr::new(node, ty))
    }
}
