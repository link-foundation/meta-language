//! Actual inference owners connected by complete generated callback decisions.
use super::{BOOL, FLOAT, Inference, Rc, RefCell, Result, Span, Term, Type, UNIT};
use crate::translation::array_callback_rules as rules;

#[allow(clippy::cast_precision_loss)]
const fn number(value: usize) -> f64 {
    value as f64
}

impl Inference {
    pub(super) fn array_callback_method(
        &mut self,
        receiver: &Term,
        operation: &str,
        arguments: &[Term],
        result: &Term,
        place: Option<Span>,
    ) -> Result<bool> {
        let form = rules::read_array_callback_result_form(operation);
        if form.is_empty() {
            return Ok(false);
        }
        let receiver = match receiver {
            Term::Known(ty @ Type::Array { .. }) => self.declared(Some(ty)),
            term => term.clone(),
        };
        let Term::Array(element) = &receiver else {
            return Ok(false);
        };
        let accumulator = arguments.get(1).unwrap_or(element.as_ref()).clone();
        let context = rules::read_array_callback_expected_result_form(operation);
        let ret = match context.as_str() {
            "bool" => Term::Known(BOOL),
            "accumulator" => accumulator.clone(),
            _ => self.fresh(false),
        };
        if context == "array" {
            let element = self.fresh(false);
            self.unify(&ret, &Term::Array(Box::new(element)), place)?;
        }
        let mut params = Vec::new();
        loop {
            let role = rules::read_array_callback_parameter_role(operation, number(params.len()));
            let term = match role.as_str() {
                "accumulator" => accumulator.clone(),
                "element" => element.as_ref().clone(),
                "index" => Term::Known(FLOAT),
                "receiver" => receiver.clone(),
                _ => break,
            };
            params.push(term);
        }
        if let Some(callback) = arguments.first() {
            self.array_from_callback(callback, &params, &ret, place)?;
        }
        let output = match form.as_str() {
            "mapped array" => Term::Array(Box::new(ret)),
            "receiver" => receiver,
            "optional element" => Term::Option {
                element: element.clone(),
                absence: Rc::new(RefCell::new("undefined".to_owned())),
            },
            "float" => Term::Known(FLOAT),
            "bool" => Term::Known(BOOL),
            "unit" => Term::Known(UNIT),
            "accumulator" => accumulator,
            _ => ret,
        };
        self.unify(result, &output, place)?;
        Ok(true)
    }
}
