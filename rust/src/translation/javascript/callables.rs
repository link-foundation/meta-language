//! Mechanical source facts and typed constructors for generated callable plans.

use super::{
    JavaScriptParser, Result, SExpr, SNode, SParam, Scope, Span, Stmt, TokenKind, imperative, node,
    type_error, unsupported,
};
use crate::translation::callable_source_rules as rules;
use crate::translation::surface_reference_facts::{
    SurfaceReferenceFacts, create_surface_reference_facts,
};
use serde_json::{Value, json};

pub(super) fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("a bounded host position has a Number representation")
}

pub(super) fn position(value: f64) -> usize {
    value
        .to_string()
        .parse()
        .expect("a generated identity names an actual host position")
}

pub(super) fn field(facts: &SurfaceReferenceFacts<'_>, node: f64, name: &str) -> f64 {
    let node = position(node);
    let start = position(facts.starts[node]);
    let count = position(facts.counts[node]);
    (start..start + count)
        .find(|edge| facts.labels[*edge] == name)
        .map_or(-1.0, |edge| facts.targets[edge])
}

pub(super) fn strings(facts: &SurfaceReferenceFacts<'_>, identities: &[f64]) -> Vec<String> {
    identities
        .iter()
        .map(|identity| {
            facts
                .value(*identity)
                .and_then(Value::as_str)
                .expect("a generated source-name identity retains a string")
                .to_owned()
        })
        .collect()
}

/// Transport only: each host statement is encoded with its existing JS fields.
pub(super) fn statements_value(statements: &[Stmt]) -> Value {
    Value::Array(statements.iter().map(statement_value).collect())
}

fn statement_value(statement: &Stmt) -> Value {
    match statement {
        Stmt::Const { name, value, span } => {
            json!({"s":"const","name":name,"value":value,"span":span})
        }
        Stmt::Let { name, value, span } => json!({"s":"let","name":name,"value":value,"span":span}),
        Stmt::Assign { name, value, span } => {
            json!({"s":"assign","name":name,"value":value,"span":span})
        }
        Stmt::Return { expr, span } => json!({"s":"return","expr":expr,"span":span}),
        Stmt::Expr { expr, span } => json!({"s":"expr","expr":expr,"span":span}),
        Stmt::Print { expr, style, span } => {
            json!({"s":"print","expr":expr,"style":style,"span":span})
        }
        Stmt::Throw { message, span } => json!({"s":"throw","message":message,"span":span}),
        Stmt::Break { span } => json!({"s":"break","span":span}),
        Stmt::Continue { span } => json!({"s":"continue","span":span}),
        Stmt::Empty => json!({"s":"empty"}),
        Stmt::Block { body, inline, span } => {
            json!({"s":"block","body":statements_value(body),"inline":inline,"span":span})
        }
        Stmt::If {
            cond,
            then,
            otherwise,
            span,
        } => {
            json!({"s":"if","cond":cond,"then":statements_value(then),"else":otherwise.as_ref().map(|body| statements_value(body)),"span":span})
        }
        Stmt::While { cond, body, span } => {
            json!({"s":"while","cond":cond,"body":statements_value(body),"span":span})
        }
        Stmt::DoWhile { cond, body, span } => {
            json!({"s":"doWhile","cond":cond,"body":statements_value(body),"span":span})
        }
        Stmt::For {
            init,
            cond,
            update,
            body,
            span,
        } => {
            json!({"s":"for","init":statements_value(init),"cond":cond,"update":statements_value(update),"body":statements_value(body),"span":span})
        }
        Stmt::Switch(switch) => {
            json!({"s":"switch","discriminant":switch.scrutinee,"clauses":switch.clauses.iter().map(|clause| json!({"tests":clause.tests.iter().map(case_value).collect::<Vec<_>>(),"body":statements_value(&clause.body),"span":clause.span})).collect::<Vec<_>>(),"span":switch.span})
        }
    }
}

fn case_value(test: &super::CaseTest) -> Value {
    match test {
        super::CaseTest::Default { span } => json!({"k":"default","span":span}),
        super::CaseTest::Tag { tag, span } => json!({"k":"tag","tag":tag,"span":span}),
        super::CaseTest::NumLit {
            value,
            negative,
            span,
        } => json!({"k":"numLit","value":value,"negative":negative,"span":span}),
        super::CaseTest::BoolLit { value, span } => {
            json!({"k":"boolLit","value":value,"span":span})
        }
        super::CaseTest::Value { value, span } => json!({"k":"value","value":value,"span":span}),
    }
}

pub(super) fn statement_usage(statements: &[Stmt], bound: &[String]) -> super::flow::Uses {
    let value = statements_value(statements);
    let facts = create_surface_reference_facts(&value);
    let plan = rules::read_callable_statement_uses(
        &facts.forms,
        &facts.texts,
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        facts.root,
        bound,
    );
    let mut result = super::flow::Uses::default();
    for event in plan.as_chunks::<2>().0 {
        let name = facts
            .value(event[1])
            .and_then(Value::as_str)
            .expect("usage plans select actual source names")
            .to_owned();
        match position(event[0]) {
            0 => {
                result.free.insert(name);
            }
            1 => {
                result.assigned.insert(name);
            }
            _ => {}
        }
    }
    result
}

pub(super) fn mutable_cells(statements: &[Stmt]) -> std::collections::HashSet<String> {
    let value = statements_value(statements);
    let facts = create_surface_reference_facts(&value);
    let plan = rules::plan_callable_mutable_cells(
        &facts.forms,
        &facts.texts,
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        facts.root,
    );
    strings(&facts, &plan).into_iter().collect()
}

pub(super) fn validate_closure_captures(statements: &[Stmt], references: bool) -> Result<()> {
    let value = statements_value(statements);
    let facts = create_surface_reference_facts(&value);
    let plan = rules::read_callable_capture_conflicts(
        &facts.forms,
        &facts.texts,
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        facts.root,
    );
    for conflict in plan.as_chunks::<2>().0 {
        let capture = facts
            .value(conflict[1])
            .and_then(Value::as_str)
            .expect("a conflict retains an actual capture name");
        if !capture.is_empty() && !references {
            let span = facts
                .value(conflict[0])
                .and_then(|closure| closure.get("span"))
                .map(|span| {
                    serde_json::from_value::<Span>(span.clone())
                        .expect("a closure retains its source span")
                });
            return Err(unsupported(
                &format!("mutable closure capture {capture}"),
                "captured assignments require a shared cell across calls",
                span,
            ));
        }
    }
    Ok(())
}

impl JavaScriptParser {
    pub(super) fn read_closure(&mut self, method: bool) -> Result<SExpr> {
        self.with_lexical_scope(&[], 1, |parser| parser.read_closure_in_scope(method))
    }

    fn read_closure_in_scope(&mut self, method: bool) -> Result<SExpr> {
        let start = self.peek();
        let expression = method || self.cursor.eat("function").is_some();
        let token = self.peek();
        let token_kind = if token.kind == TokenKind::Identifier {
            "identifier"
        } else {
            ""
        };
        let error = rules::read_callable_closure_prefix_error(expression, token_kind, &token.value);
        match error.as_str() {
            "named function expression" => {
                return Err(unsupported(
                    &error,
                    "the function name needs a recursive local binding",
                    Some(token.span()),
                ));
            }
            "generator function" => {
                return Err(unsupported(
                    &error,
                    "generators require a suspended execution state",
                    Some(token.span()),
                ));
            }
            _ => {}
        }
        let tokens = if self.cursor.eat("(").is_some() {
            self.parameters("closure")?
        } else {
            vec![(self.cursor.identifier(Some("closure parameter"))?, None)]
        };
        if !expression {
            self.cursor.expect("=>", Some("closure"))?;
        }
        let names: Vec<_> = tokens
            .iter()
            .map(|(token, _)| token.value.clone())
            .collect();
        let error = rules::read_callable_closure_parameter_error(
            &names,
            &(tokens
                .iter()
                .map(|(_, value)| value.is_some())
                .collect::<Vec<_>>()),
        );
        match error.as_str() {
            "duplicate closure parameter" => return Err(type_error(error, Some(start.span()))),
            "closure parameter default" => {
                return Err(unsupported(
                    &error,
                    "a function value needs a callable default-argument signature",
                    Some(start.span()),
                ));
            }
            _ => {}
        }
        let token_values: Vec<_> = tokens
            .iter()
            .map(|(token, _)| json!({"value":token.value,"start":token.start,"end":token.end}))
            .collect();
        let scope_value = json!({"outer":{"locals":self.scope.locals,"tdz":self.scope.tdz,"mutable":self.scope.mutable},"tokens":token_values});
        let facts = create_surface_reference_facts(&scope_value);
        let tokens_node = field(&facts, facts.root, "tokens");
        let scope_plan = rules::plan_callable_closure_scope(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            field(&facts, facts.root, "outer"),
            tokens_node,
        );
        let locals = position(scope_plan[0]);
        let temporal = position(scope_plan[1]);
        let mutable = position(scope_plan[2]);
        let scope = Scope {
            locals: strings(&facts, &scope_plan[3..3 + locals])
                .into_iter()
                .collect(),
            tdz: strings(&facts, &scope_plan[3 + locals..3 + locals + temporal])
                .into_iter()
                .collect(),
            mutable: strings(
                &facts,
                &scope_plan[3 + locals + temporal..3 + locals + temporal + mutable],
            )
            .into_iter()
            .collect(),
        };
        let parameter_plan = rules::plan_callable_surface_parameters(
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            tokens_node,
        );
        let params = parameter_plan
            .as_chunks::<3>()
            .0
            .iter()
            .map(|entry| SParam {
                optional: false,
                name: facts
                    .value(entry[0])
                    .and_then(Value::as_str)
                    .expect("a parameter retains its token spelling")
                    .to_owned(),
                span: Some(Span::new(
                    facts
                        .value(entry[1])
                        .and_then(Value::as_u64)
                        .and_then(|value| usize::try_from(value).ok())
                        .expect("a token start is a host offset"),
                    facts
                        .value(entry[2])
                        .and_then(Value::as_u64)
                        .and_then(|value| usize::try_from(value).ok())
                        .expect("a token end is a host offset"),
                )),
                ty: None,
                default_value: None,
                guard: None,
                rocq_type: None,
            })
            .collect::<Vec<_>>();
        for parameter in &params {
            self.lexical.binding(&parameter.name, None);
        }
        let outer = std::mem::replace(&mut self.scope, scope);
        let outer_jumps = std::mem::take(&mut self.jumps);
        let outer_top = std::mem::replace(&mut self.top_level, false);
        let outer_async = std::mem::replace(&mut self.in_async, false);
        let result = (|| {
            let statements = if self.cursor.is("{") {
                self.block_statements()?
            } else {
                vec![Stmt::Return {
                    expr: self.expr()?,
                    span: start.span(),
                }]
            };
            let place = Span::new(start.start, self.cursor.peek().start);
            let value = statements_value(&statements);
            let facts = create_surface_reference_facts(&value);
            let plan = rules::plan_callable_statement_captures(
                &facts.forms,
                &facts.texts,
                &facts.starts,
                &facts.counts,
                &facts.labels,
                &facts.targets,
                facts.root,
                &names,
                &(outer.locals.iter().cloned().collect::<Vec<_>>()),
            );
            let count = position(plan[0]);
            let assigned = strings(&facts, &plan[1..=count]);
            let captures = strings(&facts, &plan[1 + count..]);
            if !assigned.is_empty() && !self.reference_state.enabled {
                return Err(unsupported(
                    "mutable closure capture",
                    "captured assignments require a shared cell across calls",
                    Some(place),
                ));
            }
            let body = if imperative(&statements) {
                self.generated_count += 1;
                let name = format!("closure{}", self.generated_count);
                super::imperative::lower_imperative_with_captures(
                    self,
                    &name,
                    &params,
                    &statements,
                    place,
                    &assigned,
                    &captures,
                )?
            } else {
                super::lower(&statements, place, &mut self.generated_count)?
            };
            let assigned_captures =
                rules::require_callable_assigned_capture_field(number(assigned.len()))
                    .then_some(assigned);
            Ok(node(
                SNode::Lambda {
                    params,
                    ret: None,
                    captures,
                    assigned_captures,
                    body: Box::new(body),
                },
                place,
            ))
        })();
        self.scope = outer;
        self.jumps = outer_jumps;
        self.top_level = outer_top;
        self.in_async = outer_async;
        result
    }
}

/// Interpret the generic reconstruction instructions over retained source nodes.
pub(super) fn rename_expression(expression: &SExpr, scope: &Value) -> Result<SExpr> {
    fn materialize(plan: &[f64], at: &mut usize, facts: &SurfaceReferenceFacts<'_>) -> Value {
        let operation = position(plan[*at]);
        *at += 1;
        match operation {
            0 => {
                let identity = plan[*at];
                *at += 1;
                facts
                    .value(identity)
                    .expect("a copy instruction retains an actual value")
                    .clone()
            }
            1 => {
                let count = position(plan[*at]);
                *at += 1;
                Value::Array((0..count).map(|_| materialize(plan, at, facts)).collect())
            }
            2 => {
                let source = plan[*at];
                *at += 1;
                let count = position(plan[*at]);
                *at += 1;
                let mut object = if source < 0.0 {
                    serde_json::Map::new()
                } else {
                    facts
                        .value(source)
                        .and_then(Value::as_object)
                        .expect("a record-copy instruction retains a record")
                        .clone()
                };
                for _ in 0..count {
                    let field = plan[*at];
                    *at += 1;
                    let label = if field < 0.0 {
                        rules::read_callable_renaming_field(-field - 1.0)
                    } else {
                        facts.labels[position(field)].clone()
                    };
                    assert!(!label.is_empty(), "a reconstruction field is declared");
                    object.insert(label, materialize(plan, at, facts));
                }
                Value::Object(object)
            }
            3 => {
                let constant = plan[*at];
                *at += 1;
                Value::String(rules::read_callable_renaming_constant(constant))
            }
            _ => unreachable!("the generated reconstruction plan has a known instruction"),
        }
    }
    let value = json!({"node":expression,"scope":scope});
    let facts = create_surface_reference_facts(&value);
    let plan = rules::plan_callable_source_renaming(
        &facts.forms,
        &facts.texts,
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        field(&facts, facts.root, "node"),
        field(&facts, facts.root, "scope"),
    );
    let mut at = 0;
    let value = materialize(&plan, &mut at, &facts);
    assert_eq!(
        at,
        plan.len(),
        "a reconstruction plan consumes every instruction"
    );
    serde_json::from_value(value).map_err(|error| {
        type_error(
            format!("a reconstructed source expression is malformed: {error}"),
            expression.span,
        )
    })
}
