// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::ml_array;
use super::read_array_from_element;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_array_from_callback_constraints_loop2(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut params: f64,
    mut declared_params: f64,
    mut count: f64,
    mut pairs: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let pairs_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::read_array_from_element(
                                starts,
                                counts,
                                targets,
                                declared_params,
                                index,
                            ),
                            self::read_array_from_element(starts, counts, targets, params, index),
                        ],
                        pairs,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (pairs, index) = (pairs_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { pairs };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_array_from_map_callback_loop3(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut params: f64,
    mut declared: f64,
    mut count: f64,
    mut plan: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let plan_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::read_array_from_element(starts, counts, targets, declared, index),
                            self::read_array_from_element(starts, counts, targets, params, index),
                        ],
                        plan,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (plan, index) = (plan_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { plan };
    }
}
