// Generated from complete js/src/translation/array-binding-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Result;
use super::ml_string;
use super::read_array_binding_fact_field;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_binding_default_term_loop9(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut node: f64,
    mut absence: self::ml_string::Text,
    mut kind: f64,
) -> self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Result {
    loop {
        if (kind >= 0f64)
            && ((self::read_required_text_fact(texts, kind))
                == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
        {
            {
                let field = self::read_array_binding_fact_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 99, 101])),
                );
                {
                    let current = if field < 0f64 {
                        self::ml_string::Text::from_units(vec![98, 111, 116, 104])
                    } else {
                        self::read_required_text_fact(texts, field)
                    };
                    {
                        let absence_2 = if (absence == (self::ml_string::Text::from_units(vec![])))
                            || (absence == current)
                        {
                            current
                        } else {
                            self::ml_string::Text::from_units(vec![98, 111, 116, 104])
                        };
                        {
                            let node_2 = self::read_array_binding_fact_field(
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![
                                    101, 108, 101, 109, 101, 110, 116,
                                ])),
                            );
                            {
                                let kind_2 = self::read_array_binding_fact_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    node_2,
                                    &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
                                );
                                {
                                    (node, absence, kind) = (node_2, absence_2, kind_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Result::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Done(node, absence, kind)
        };
    }
}
