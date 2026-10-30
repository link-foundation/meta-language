//! Actual inference owners into the shared recursive iterable selection.
use super::{Inference, Result, Term};
use crate::translation::recursive_binding_rules::{self as rules, ml_string::Text};
impl Inference {
    pub(super) fn solve_recursive_binding_iterables(&mut self) -> Result<bool> {
        let mut waiting = Vec::new();
        let mut progress = false;
        for constraint in std::mem::take(&mut self.recursive_binding_iterables) {
            let mut object = self.resolve(&constraint.object);
            let element = match &object {
                Term::Option { element, .. } => {
                    super::optional_values::kind(&self.resolve(element)).to_owned()
                }
                _ => String::new(),
            };
            let mut form = rules::read_java_script_recursive_binding_iterable_form(
                &Text::from_text(super::optional_values::kind(&object)),
                &Text::from_text(&element),
            )
            .to_string();
            if form == "optional" {
                let Term::Option { element, .. } = &object else {
                    unreachable!("shared optional iterable")
                };
                object = self.resolve(element);
                form = rules::read_java_script_recursive_binding_iterable_form(
                    &Text::from_text(super::optional_values::kind(&object)),
                    &Text::from_text(""),
                )
                .to_string();
            }
            match form.as_str() {
                "wait" => {
                    waiting.push(constraint);
                    continue;
                }
                "identity" => self.unify(&constraint.result, &object, constraint.place)?,
                "iterate" => {
                    let element = self.array_from_iterable_element(&object).ok_or_else(|| {
                        super::type_error(
                            "a recursive binding needs a represented iterable",
                            constraint.place,
                        )
                    })?;
                    self.unify(
                        &constraint.result,
                        &Term::Array(Box::new(element)),
                        constraint.place,
                    )?;
                }
                _ => {
                    return Err(super::type_error(
                        "a recursive binding needs a represented iterable",
                        constraint.place,
                    ));
                }
            }
            progress = true;
        }
        self.recursive_binding_iterables = waiting;
        Ok(progress)
    }
}
