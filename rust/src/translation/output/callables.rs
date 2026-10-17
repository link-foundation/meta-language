//! Retain checked declarations and spans while executing the shared effect plans.

use super::{HashSet, Program, Span};
use crate::translation::callable_emission_rules as rules;
use crate::translation::surface_reference_facts::create_surface_reference_facts;
use serde_json::{Value, json};

pub(super) struct CallableOutputAnalysis {
    pub effectful: HashSet<String>,
    pub count: f64,
    pub span: Option<Span>,
}

fn position(value: f64) -> usize {
    value
        .to_string()
        .parse()
        .expect("an effect plan retains an actual fact position")
}

pub(super) fn analyze(program: &Program) -> CallableOutputAnalysis {
    let value = json!({"declarations":program.declarations,"main":program.main});
    let facts = create_surface_reference_facts(&value);
    let root = position(facts.root);
    let start = position(facts.starts[root]);
    let declarations = (start..start + position(facts.counts[root]))
        .find(|edge| facts.labels[*edge] == "declarations")
        .map(|edge| facts.targets[edge])
        .expect("a checked program retains its declaration sequence");
    let selected = rules::read_callable_effectful_declarations(
        &facts.forms,
        &facts.texts,
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        declarations,
    );
    let effectful = selected
        .iter()
        .map(|identity| {
            facts
                .value(*identity)
                .and_then(|value| value.get("fullName"))
                .and_then(Value::as_str)
                .expect("an effectful declaration retains its actual full name")
                .to_owned()
        })
        .collect();
    let obligations = rules::plan_callable_output_obligations(
        &facts.forms,
        &facts.texts,
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        facts.root,
        &selected,
    );
    CallableOutputAnalysis {
        effectful,
        count: serde_json::Number::from(obligations.len())
            .as_f64()
            .expect("an allocated obligation count has a Number representation"),
        span: obligations
            .first()
            .and_then(|identity| facts.value(*identity))
            .and_then(|value| value.get("span"))
            .filter(|value| !value.is_null())
            .map(|value| {
                serde_json::from_value(value.clone())
                    .expect("a checked obligation retains its source span")
            }),
    }
}
