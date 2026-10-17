//! Interpret actual source identities selected by the generated shared planner.

use super::callables::statements_value;
use super::{BinaryOp, HashMap, SExpr, SNode, SParam, Span, Stmt};
use crate::translation::guarded_private_array_write_rules as rules;
use crate::translation::surface_reference_facts::{
    SurfaceReferenceFacts, create_surface_reference_facts,
};
use serde_json::{Value, json};

fn temporary_name(
    role: &str,
    generated_count: &mut usize,
    facts: &SurfaceReferenceFacts<'_>,
) -> String {
    loop {
        *generated_count = generated_count
            .checked_add(1)
            .expect("a bounded parsed function has a finite generated-name census");
        let name = format!("ml_guarded_write_{role}_{}", *generated_count);
        if !facts.texts.contains(&name) {
            return name;
        }
    }
}

fn local(name: &str, span: Span) -> SExpr {
    SExpr::new(
        SNode::Name {
            path: vec![name.to_owned()],
        },
        Some(span),
    )
}

fn parameter(name: String) -> SParam {
    SParam {
        name,
        optional: false,
        default_value: None,
        ty: None,
        span: None,
        guard: None,
        rocq_type: None,
    }
}

/// Interpret only selected source IDs. No ownership, guard, scalar-expression,
/// allocation, alias or escape classification is implemented in this adapter.
pub(super) fn lower_guarded_private_array_writes(
    statements: &mut [Stmt],
    parameters: &[SParam],
    generated_count: &mut usize,
) -> Vec<Span> {
    let source = json!({"statements":statements_value(statements),"parameters":parameters});
    let facts = create_surface_reference_facts(&source);
    let positions: HashMap<*const Value, usize> = source["statements"]
        .as_array()
        .expect("the actual body is an array")
        .iter()
        .enumerate()
        .map(|(index, value)| (std::ptr::from_ref(value), index))
        .collect();
    let plan = rules::plan_guarded_private_array_writes(
        &facts.columns,
        &facts.rows,
        facts.size,
        facts.root,
    );
    let (triplets, remainder) = plan.as_chunks::<3>();
    assert!(
        remainder.is_empty(),
        "every selection contains three actual IDs"
    );
    let mut operations = Vec::new();
    for [conditional, write, allocation] in triplets {
        let selected = facts
            .value(*conditional)
            .expect("the planner selects an actual if");
        let statement = *positions
            .get(&std::ptr::from_ref(selected))
            .expect("the selected if is an actual top-level statement");
        let allocated = facts
            .value(*allocation)
            .expect("the planner selects an actual allocation");
        let allocation_position = *positions
            .get(&std::ptr::from_ref(allocated))
            .expect("the allocation is an actual top-level statement");
        let owner = match &statements[allocation_position] {
            Stmt::Const { name, .. } | Stmt::Let { name, .. } => name.clone(),
            _ => unreachable!("the generated allocation ID retains its actual constructor"),
        };
        let selected_write = facts
            .value(*write)
            .expect("the planner selects an actual operation");
        assert!(std::ptr::eq(
            selected_write,
            selected
                .pointer("/then/0/expr")
                .expect("the selected native operation retains its actual wrapper"),
        ));
        assert_eq!(allocated["name"].as_str(), Some(owner.as_str()));
        let Stmt::If { then, .. } = &mut statements[statement] else {
            unreachable!("the generated if ID retains its actual constructor");
        };
        assert_eq!(
            then.len(),
            1,
            "the generated plan selects the complete then body"
        );
        let original = std::mem::replace(&mut then[0], Stmt::Empty);
        let Stmt::Expr { expr, .. } = original else {
            unreachable!("the selected native write remains an expression statement");
        };
        assert_eq!(
            serde_json::to_value(&expr).expect("an actual expression serializes"),
            *selected_write
        );
        let span = expr
            .span
            .expect("a parsed write retains its actual operation span");
        let SNode::ReferenceWrite {
            object,
            index: Some(index),
            field: None,
            value: Some(value),
            op,
        } = expr.node
        else {
            unreachable!("the selected operation retains its complete native source fields");
        };
        assert_eq!(op, "=");
        assert_eq!(object.simple_name(), Some(owner.as_str()));
        let operation_span = Span::new(
            object
                .span
                .expect("the actual receiver retains its span")
                .end,
            value
                .span
                .expect("the actual right operand retains its span")
                .start,
        );
        let position = temporary_name("position", generated_count, &facts);
        let replacement = temporary_name("value", generated_count, &facts);
        let element = temporary_name("element", generated_count, &facts);
        let ordinal = temporary_name("ordinal", generated_count, &facts);
        let select = SExpr::new(
            SNode::Lambda {
                params: vec![parameter(element.clone()), parameter(ordinal.clone())],
                ret: None,
                captures: Vec::new(),
                assigned_captures: None,
                body: Box::new(SExpr::new(
                    SNode::If {
                        cond: Box::new(SExpr::new(
                            SNode::Binary {
                                op: BinaryOp::Eq,
                                left: Box::new(local(&ordinal, span)),
                                right: Box::new(local(&position, span)),
                                rounding: None,
                            },
                            Some(span),
                        )),
                        then: Box::new(local(&replacement, span)),
                        otherwise: Box::new(local(&element, span)),
                    },
                    Some(span),
                )),
            },
            Some(span),
        );
        *then = vec![
            Stmt::Const {
                name: position,
                value: *index,
                span,
            },
            Stmt::Const {
                name: replacement,
                value: *value,
                span,
            },
            Stmt::Assign {
                name: owner,
                value: SExpr::new(
                    SNode::Method {
                        object,
                        op: "map".to_owned(),
                        args: vec![Some(select)],
                    },
                    Some(span),
                ),
                span,
            },
        ];
        operations.push(operation_span);
    }
    operations
}
