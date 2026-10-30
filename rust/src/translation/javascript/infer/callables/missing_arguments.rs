//! Actual omitted inputs and inference owners through source-generated slots.
use super::{Inference, Span, Term};
impl Inference {
    pub(in crate::translation::javascript::infer) fn complete_missing_callable_arguments(
        &mut self,
        parameters: &[Term],
        arguments: &[Term],
        defaults: &[bool],
        place: Option<Span>,
        queue: bool,
    ) -> Vec<Term> {
        let inferred = parameters
            .iter()
            .map(|term| matches!(term, Term::Var(_)))
            .collect::<Vec<_>>();
        let count = serde_json::Number::from(arguments.len())
            .as_f64()
            .expect("actual argument count");
        let plan =
            crate::translation::missing_argument_rules::plan_java_script_missing_argument_inference(
                &inferred, defaults, count,
            );
        if plan.is_empty() {
            return arguments.to_vec();
        }
        let result = plan
            .iter()
            .enumerate()
            .map(|(index, slot)| {
                if *slot >= 0.0 {
                    arguments[crate::translation::frontend_rules::ml_array::number_index(*slot)
                        .expect("actual argument position")]
                    .clone()
                } else if *slot == -2.0 {
                    parameters[index].clone()
                } else {
                    let element = self.fresh(false);
                    super::super::optional_values::optional(element, "undefined")
                }
            })
            .collect::<Vec<_>>();
        if queue {
            self.calls.push(super::super::CallArguments {
                parameters: parameters.to_vec(),
                arguments: result.clone(),
                defaults: vec![false; parameters.len()],
                optional_params: vec![false; parameters.len()],
                place,
            });
        }
        result
    }
}
