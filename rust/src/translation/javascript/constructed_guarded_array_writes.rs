//! Actual source occurrence transport for the generated joint construction plan.

use super::callables::{position, statements_value};
use super::{BinaryOp, HashMap, HashSet, SArrayItem, SExpr, SNode, SParam, Span, Stmt};
use crate::translation::guarded_private_array_write_rules as rules;
use crate::translation::surface_reference_facts::{
    SurfaceReferenceFacts, create_surface_reference_facts,
};
use serde_json::{Value, json};

#[derive(Clone, Copy)]
enum ChildList {
    Body,
    Then,
    Otherwise,
    Initializers,
    Updates,
    Clause(usize),
}

#[derive(Clone)]
struct StatementPath {
    parents: Vec<(usize, ChildList)>,
    index: usize,
}

/// Join existing owned statement slots to the actual serialized statement nodes.
/// These are constructor/field relationships, not planner admission decisions.
fn statement_paths(
    statements: &[Stmt],
    values: &Value,
    parents: &mut Vec<(usize, ChildList)>,
    paths: &mut HashMap<*const Value, StatementPath>,
) {
    let values = values
        .as_array()
        .expect("actual statement lists serialize as arrays");
    assert_eq!(statements.len(), values.len());
    for (index, statement) in statements.iter().enumerate() {
        let value = &values[index];
        assert!(
            paths
                .insert(
                    std::ptr::from_ref(value),
                    StatementPath {
                        parents: parents.clone(),
                        index,
                    }
                )
                .is_none()
        );
        let mut child = |slot, statements: &[Stmt], values: &Value| {
            parents.push((index, slot));
            statement_paths(statements, values, parents, paths);
            parents.pop();
        };
        match statement {
            Stmt::Block { body, .. } | Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
                child(ChildList::Body, body, &value["body"])
            }
            Stmt::If {
                then, otherwise, ..
            } => {
                child(ChildList::Then, then, &value["then"]);
                if let Some(otherwise) = otherwise {
                    child(ChildList::Otherwise, otherwise, &value["else"]);
                }
            }
            Stmt::For {
                init, body, update, ..
            } => {
                child(ChildList::Initializers, init, &value["init"]);
                child(ChildList::Body, body, &value["body"]);
                child(ChildList::Updates, update, &value["update"]);
            }
            Stmt::Switch(switch) => {
                assert_eq!(
                    switch.clauses.len(),
                    value["clauses"]
                        .as_array()
                        .expect("actual switch clauses serialize as an array")
                        .len()
                );
                for (clause, item) in switch.clauses.iter().enumerate() {
                    child(
                        ChildList::Clause(clause),
                        &item.body,
                        &value["clauses"][clause]["body"],
                    );
                }
            }
            _ => {}
        }
    }
}

/// Resolve an owned path afresh; no references into mutable vectors are retained.
fn statement_at_mut<'a>(mut statements: &'a mut [Stmt], path: &StatementPath) -> &'a mut Stmt {
    for (index, slot) in &path.parents {
        let statement = statements
            .get_mut(*index)
            .expect("an actual parent slot remains present");
        statements = match (slot, statement) {
            (
                ChildList::Body,
                Stmt::Block { body, .. }
                | Stmt::While { body, .. }
                | Stmt::DoWhile { body, .. }
                | Stmt::For { body, .. },
            ) => body,
            (ChildList::Then, Stmt::If { then, .. }) => then,
            (ChildList::Otherwise, Stmt::If { otherwise, .. }) => otherwise
                .as_mut()
                .expect("an actual else list remains present"),
            (ChildList::Initializers, Stmt::For { init, .. }) => init,
            (ChildList::Updates, Stmt::For { update, .. }) => update,
            (ChildList::Clause(clause), Stmt::Switch(switch)) => {
                &mut switch
                    .clauses
                    .get_mut(*clause)
                    .expect("an actual clause remains present")
                    .body
            }
            _ => unreachable!("an owned statement path retains its original constructors"),
        };
    }
    statements
        .get_mut(path.index)
        .expect("the actual selected statement remains present")
}

#[derive(Clone, Copy)]
enum Operation {
    Guard,
    Append,
}

struct Selection<'a> {
    operation: Operation,
    path: StatementPath,
    owner: String,
    source_operation: &'a Value,
}

fn actual_operation(statement: &Stmt, operation: Operation) -> &SExpr {
    match (operation, statement) {
        (Operation::Append, Stmt::Expr { expr, .. }) => expr,
        (Operation::Guard, Stmt::If { then, .. }) => {
            assert_eq!(
                then.len(),
                1,
                "the selected guard retains its complete then list"
            );
            let Stmt::Expr { expr, .. } = &then[0] else {
                unreachable!("the native selected guard retains its actual expression wrapper");
            };
            expr
        }
        _ => unreachable!("the generated statement identity retains its constructor"),
    }
}

fn temporary_name(
    role: &str,
    generated_count: &mut usize,
    facts: &SurfaceReferenceFacts<'_>,
) -> String {
    loop {
        *generated_count = generated_count
            .checked_add(1)
            .expect("a bounded parsed function has a finite generated-name census");
        let name = format!("ml_constructed_guarded_{role}_{}", *generated_count);
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

const fn parameter(name: String) -> SParam {
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

fn replace_append(statement: &mut Stmt, owner: String, source_operation: &Value) -> Span {
    assert_eq!(
        serde_json::to_value(actual_operation(statement, Operation::Append))
            .expect("an actual selected operation serializes"),
        *source_operation
    );
    let original = std::mem::replace(statement, Stmt::Empty);
    let Stmt::Expr { expr, span } = original else {
        unreachable!("the generated append identity retains its expression statement");
    };
    let place = expr.span;
    let SNode::Method { object, args, op } = expr.node else {
        unreachable!("the selected append retains its actual method constructor");
    };
    assert_eq!(op, "push");
    assert_eq!(object.simple_name(), Some(owner.as_str()));
    let first = args
        .first()
        .and_then(Option::as_ref)
        .expect("the selected actual push has its first argument");
    let operation_span = Span::new(
        object
            .span
            .expect("the actual push receiver retains its span")
            .end,
        first
            .span
            .expect("the actual first push argument retains its span")
            .start,
    );
    // Exactly the existing owned-array-append AST: preserve argument order and
    // omission of each nonspread item's serialized flag, without cloning operands.
    let mut items = vec![SArrayItem::new(*object, true)];
    items.extend(args.into_iter().map(|argument| {
        SArrayItem::without_spread(
            argument.expect("each selected actual push argument retains its expression"),
        )
    }));
    *statement = Stmt::Assign {
        name: owner,
        value: SExpr::new(
            SNode::Array {
                items,
                element: None,
            },
            place,
        ),
        span,
    };
    operation_span
}

fn replace_guard(
    statement: &mut Stmt,
    owner: String,
    source_operation: &Value,
    generated_count: &mut usize,
    facts: &SurfaceReferenceFacts<'_>,
) -> Span {
    assert_eq!(
        serde_json::to_value(actual_operation(statement, Operation::Guard))
            .expect("an actual selected operation serializes"),
        *source_operation
    );
    let Stmt::If { then, .. } = statement else {
        unreachable!("the generated guard identity retains its actual if constructor");
    };
    let original = std::mem::replace(&mut then[0], Stmt::Empty);
    let Stmt::Expr { expr, .. } = original else {
        unreachable!("the native selected write retains its actual expression wrapper");
    };
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
        unreachable!("the selected write retains its complete native constructor");
    };
    assert_eq!(op, "=");
    assert_eq!(object.simple_name(), Some(owner.as_str()));
    let operation_span = Span::new(
        object
            .span
            .expect("the actual write receiver retains its span")
            .end,
        value
            .span
            .expect("the actual right operand retains its span")
            .start,
    );
    let position = temporary_name("position", generated_count, facts);
    let replacement = temporary_name("value", generated_count, facts);
    let element = temporary_name("element", generated_count, facts);
    let ordinal = temporary_name("ordinal", generated_count, facts);
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
    operation_span
}

/// Invoke before legacy append/guard lowering, inside the existing unforced
/// reference gate. The generated shared plan owns every admission decision.
pub(super) fn lower_constructed_guarded_array_writes(
    statements: &mut [Stmt],
    parameters: &[SParam],
    generated_count: &mut usize,
) -> Vec<Span> {
    let source = json!({"statements":statements_value(statements),"parameters":parameters});
    let facts = create_surface_reference_facts(&source);
    let plan = rules::plan_constructed_guarded_array_writes(
        &facts.columns,
        &facts.rows,
        facts.size,
        facts.root,
    );
    if plan.is_empty() {
        return Vec::new();
    }
    let mut paths = HashMap::new();
    statement_paths(
        statements,
        &source["statements"],
        &mut Vec::new(),
        &mut paths,
    );
    let (rows, remainder) = plan.as_chunks::<4>();
    assert!(
        remainder.is_empty(),
        "every selected operation retains four actual plan fields"
    );
    let mut selections = Vec::new();
    let mut selected_statements = HashSet::new();
    // Resolve and validate every original ID/path/operation before any mutation.
    for [code, statement, operation, allocation] in rows {
        let statement_value = facts
            .value(*statement)
            .expect("the planner selects an actual statement node");
        let path = paths
            .get(&std::ptr::from_ref(statement_value))
            .expect("the selected statement has an owned source path")
            .clone();
        assert!(
            selected_statements.insert(std::ptr::from_ref(statement_value)),
            "each actual statement is replaced exactly once"
        );
        let allocation_value = facts
            .value(*allocation)
            .expect("the planner selects an actual allocation statement");
        let allocation_path = paths
            .get(&std::ptr::from_ref(allocation_value))
            .expect("the selected allocation has an owned statement path");
        assert!(
            allocation_path.parents.is_empty(),
            "the actual allocation retains its top-level slot"
        );
        let owner = match &statements[allocation_path.index] {
            Stmt::Const { name, .. } | Stmt::Let { name, .. } => name.clone(),
            _ => unreachable!("the generated allocation ID retains its actual constructor"),
        };
        assert_eq!(allocation_value["name"].as_str(), Some(owner.as_str()));
        let source_operation = facts
            .value(*operation)
            .expect("the planner selects an actual source operation node");
        let (operation, pointer) = match position(*code) {
            0 => {
                assert!(
                    path.parents.is_empty(),
                    "the selected guard retains its top-level source slot"
                );
                (Operation::Guard, "/then/0/expr")
            }
            1 => (Operation::Append, "/expr"),
            _ => unreachable!("only actual shared plan operation codes are transported"),
        };
        assert!(std::ptr::eq(
            source_operation,
            statement_value
                .pointer(pointer)
                .expect("the selected operation retains its actual native wrapper")
        ));
        assert_eq!(
            serde_json::to_value(actual_operation(
                statement_at_mut(statements, &path),
                operation
            ))
            .expect("the actual selected operation serializes"),
            *source_operation
        );
        selections.push(Selection {
            operation,
            path,
            owner,
            source_operation,
        });
    }
    let mut operations = Vec::new();
    for selection in selections {
        let statement = statement_at_mut(statements, &selection.path);
        let span = match selection.operation {
            Operation::Guard => replace_guard(
                statement,
                selection.owner,
                selection.source_operation,
                generated_count,
                &facts,
            ),
            Operation::Append => {
                replace_append(statement, selection.owner, selection.source_operation)
            }
        };
        operations.push(span);
    }
    operations
}
