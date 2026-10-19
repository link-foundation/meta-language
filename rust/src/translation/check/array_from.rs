//! Mechanical checked-value construction from the complete Array.from plans.
use super::{
    Checker, Env, Expr, FLOAT, Node, Result, SExpr, Span, Type, array, type_error, unsupported,
};
use crate::translation::array_from_rules as rules;
use crate::translation::frontend_rules::{accept_argument_count, ml_array::number_index};
use crate::translation::surface_reference_facts::{
    SurfaceReferenceFacts, create_surface_reference_facts,
};
use serde_json::json;

fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("a host arity has a Number representation")
}

fn selected_type(facts: &SurfaceReferenceFacts<'_>, identity: f64) -> Type {
    serde_json::from_value(
        facts
            .value(identity)
            .expect("the shared plan selects an actual type")
            .clone(),
    )
    .expect("the retained type remains serializable")
}

impl Checker {
    pub(super) fn array_from(
        &mut self,
        args: &[SExpr],
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
        span: Option<Span>,
    ) -> Result<Expr> {
        let error = rules::read_array_from_argument_error(number(args.len()));
        if !error.is_empty() {
            return Err(type_error(error, span));
        }
        let object = self.expr(&args[0], env, path, None, false)?;
        let value =
            serde_json::to_value(&object.ty).expect("a checked receiver type is serializable");
        let facts = create_surface_reference_facts(&value);
        let plan = rules::plan_array_from_iterable_type(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.root,
        );
        let result = match plan.first().and_then(|code| number_index(*code)) {
            Some(0) => object,
            Some(1) => Expr::new(
                Node::StringPoints {
                    object: Box::new(object),
                },
                array(Type::String),
            ),
            Some(form @ (2 | 3)) => {
                let element = if form == 2 {
                    selected_type(&facts, plan[1])
                } else {
                    Type::Tuple {
                        elements: vec![
                            selected_type(&facts, plan[1]),
                            selected_type(&facts, plan[2]),
                        ],
                    }
                };
                Expr::new(
                    Node::CollectionItems {
                        object: Box::new(object),
                        op: if form == 2 { "values" } else { "entries" }.to_owned(),
                    },
                    array(element),
                )
            }
            _ => {
                return Err(unsupported(
                    "Array.from",
                    &rules::read_array_from_receiver_error(object.ty.kind()),
                    span,
                ));
            }
        };
        let Some(mapper) = args.get(1) else {
            return Ok(result);
        };
        let params = [
            result
                .ty
                .element()
                .expect("the selected iterable has an array result")
                .clone(),
            FLOAT,
        ];
        let expected_value =
            serde_json::to_value(expected).expect("an optional expected type is serializable");
        let expected_facts = create_surface_reference_facts(&expected_value);
        let expected_plan = rules::plan_array_from_expected_element(
            &expected_facts.texts,
            &expected_facts.starts,
            &expected_facts.counts,
            &expected_facts.labels,
            &expected_facts.targets,
            expected_facts.root,
        );
        let ret = if number_index(expected_plan[0]) == Some(1) {
            Some(selected_type(&expected_facts, expected_plan[1]))
        } else {
            None
        };
        let callback =
            self.expr_with_callable_expectation(mapper, env, path, &params, ret.as_ref())?;
        let Type::Function { ret, .. } = &callback.ty else {
            return Err(type_error("Array.from mapper has no function result", span));
        };
        let ty = array(ret.as_ref().clone());
        Ok(Expr::new(
            Node::Method {
                object: Box::new(result),
                op: "map".to_owned(),
                args: vec![Some(callback)],
            },
            ty,
        ))
    }

    pub(super) fn array_map_on_checked_receiver(
        &mut self,
        object: Expr,
        args: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        if !accept_argument_count(vec![false], number(args.len())) {
            return Err(unsupported(
                "method call .map()",
                ".map() expects 1 arguments",
                span,
            ));
        }
        let params = [
            object
                .ty
                .element()
                .expect("the map receiver retains its array type")
                .clone(),
            FLOAT,
            object.ty.clone(),
        ];
        let callback = self.expr_with_callable_expectation(
            args[0]
                .as_ref()
                .ok_or_else(|| type_error(".map() requires a function", span))?,
            env,
            path,
            &params,
            None,
        )?;
        let value = json!({"callback":callback.ty,"params":params});
        let facts = create_surface_reference_facts(&value);
        let child = |name| {
            rules::read_array_from_field(
                &facts.starts,
                &facts.counts,
                &facts.labels,
                &facts.targets,
                facts.root,
                name,
            )
        };
        let plan = rules::plan_array_from_map_callback(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            child("callback"),
            child("params"),
        );
        let equal = plan[2..]
            .as_chunks::<2>()
            .0
            .iter()
            .all(|pair| selected_type(&facts, pair[0]).same(&selected_type(&facts, pair[1])));
        let error = rules::read_array_from_map_callback_error(plan[0], equal);
        if !error.is_empty() {
            return Err(type_error(error, span));
        }
        let ty = array(selected_type(&facts, plan[1]));
        Ok(Expr::new(
            Node::Method {
                object: Box::new(object),
                op: "map".to_owned(),
                args: vec![Some(callback)],
            },
            ty,
        )
        .with_span(span))
    }
}
