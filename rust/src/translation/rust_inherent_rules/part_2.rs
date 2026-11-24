// Generated from complete js/src/translation/rust-inherent-rules.js. Do not edit by hand.
use super::MlPlanRustInherentReceiverLayersJoin14;
use super::MlPlanRustInherentReceiverLayersLoop12Result;
use super::ml;
use super::ml_array;
use super::ml_plan_rust_inherent_receiver_layers_loop15;

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustInherentReceiverLayersJoin13 {
    MlPlanRustInherentReceiverLayersJoin13Next,
    MlPlanRustInherentReceiverLayersJoin13Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_inherent_receiver_layers_loop16(
    actual: &[String],
    expected: &[String],
    mut skipped: f64,
    mut remaining: f64,
    mut same: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < remaining {
            if !(match (
                self::ml_array::optional_at(actual, self::ml_array::number_index(skipped + index)),
                self::ml_array::optional_at(expected, self::ml_array::number_index(index)),
            ) {
                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                (None, None) => true,
                _ => false,
            }) {
                {
                    let same_2 = false;
                    {
                        let index_2 = index + 1f64;
                        {
                            (same, index) = (same_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { same };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_inherent_receiver_layers_loop12(
    actual: &[String],
    expected: &[String],
    result: &[f64],
    mut through_reference: bool,
    mut skipped: f64,
) -> self::MlPlanRustInherentReceiverLayersLoop12Result {
    loop {
        return if skipped <= (actual.len() as f64) {
            {
                let remaining = (actual.len() as f64) - skipped;
                {
                    let ml_s6 = if (remaining == (expected.len() as f64))
                        && (!through_reference
                            || (((expected.len() as f64) > 0f64)
                                && match (
                                    self::ml_array::optional_at(
                                        expected,
                                        self::ml_array::number_index(0f64),
                                    ),
                                    String::from("reference"),
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                })) {
                        {
                            let same = true;
                            {
                                let index = 0f64;
                                {
                                    let same_2 = self::ml_plan_rust_inherent_receiver_layers_loop16(
                                        actual, expected, skipped, remaining, same, index,
                                    );
                                    if same_2 {
                                        self::MlPlanRustInherentReceiverLayersJoin13::MlPlanRustInherentReceiverLayersJoin13Return(result.to_vec())
                                    } else {
                                        self::MlPlanRustInherentReceiverLayersJoin13::MlPlanRustInherentReceiverLayersJoin13Next
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlPlanRustInherentReceiverLayersJoin13::MlPlanRustInherentReceiverLayersJoin13Next
                    };
                    match ml_s6 {
                        self::MlPlanRustInherentReceiverLayersJoin13::MlPlanRustInherentReceiverLayersJoin13Next => {
                            {
                                let ml_s7 = if ((remaining + 1f64) == (expected.len() as f64)) && match (self::ml_array::optional_at(expected, self::ml_array::number_index(0f64)), String::from("reference")) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                    {
                                        let same_3 = true;
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let same_4 = self::ml_plan_rust_inherent_receiver_layers_loop15(actual, expected, skipped, remaining, same_3, index_2);
                                                if same_4 {
                                                    self::MlPlanRustInherentReceiverLayersJoin14::MlPlanRustInherentReceiverLayersJoin14Return(result.to_vec())
                                                } else {
                                                    self::MlPlanRustInherentReceiverLayersJoin14::MlPlanRustInherentReceiverLayersJoin14Next
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlPlanRustInherentReceiverLayersJoin14::MlPlanRustInherentReceiverLayersJoin14Next
                                };
                                match ml_s7 {
                                    self::MlPlanRustInherentReceiverLayersJoin14::MlPlanRustInherentReceiverLayersJoin14Next => {
                                        if (skipped < (actual.len() as f64)) && match (self::ml_array::optional_at(actual, self::ml_array::number_index(skipped)), String::from("reference")) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                            {
                                                let through_reference_2 = true;
                                                {
                                                    let skipped_2 = skipped + 1f64;
                                                    {
                                                        (through_reference, skipped) = (through_reference_2, skipped_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
{
                                            {
                                                let skipped_3 = skipped + 1f64;
                                                {
                                                    skipped = skipped_3;
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlPlanRustInherentReceiverLayersJoin14::MlPlanRustInherentReceiverLayersJoin14Return(ml_result14) => {
                                        self::MlPlanRustInherentReceiverLayersLoop12Result::MlPlanRustInherentReceiverLayersLoop12Return(ml_result14)
                                    }
                                }
                            }
                        }
                        self::MlPlanRustInherentReceiverLayersJoin13::MlPlanRustInherentReceiverLayersJoin13Return(ml_result15) => {
                            self::MlPlanRustInherentReceiverLayersLoop12Result::MlPlanRustInherentReceiverLayersLoop12Return(ml_result15)
                        }
                    }
                }
            }
        } else {
            self::MlPlanRustInherentReceiverLayersLoop12Result::MlPlanRustInherentReceiverLayersLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustInherentReceiverLayersLoop11Result {
    MlPlanRustInherentReceiverLayersLoop11Done(Vec<f64>),
    MlPlanRustInherentReceiverLayersLoop11Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_inherent_receiver_layers_loop11(
    mut result: Vec<f64>,
    ml_values9: &[String],
    mut ml_index9: self::ml::Big,
) -> self::MlPlanRustInherentReceiverLayersLoop11Result {
    loop {
        return if ml_index9 < (self::ml::Big::from_u128(ml_values9.len() as u128)) {
            {
                let layer = self::ml_array::at(ml_values9, ml_index9.to_index());
                if (layer != "reference") && (layer != "box") {
                    self::MlPlanRustInherentReceiverLayersLoop11Result::MlPlanRustInherentReceiverLayersLoop11Return(vec![0f64])
                } else {
                    {
                        let result_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![if layer == "reference" { 1f64 } else { 2f64 }], result);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let ml_index9_2 = ml_index9.add(&self::ml::Big::from_i128(1));
                            {
                                (result, ml_index9) = (result_2, ml_index9_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlPlanRustInherentReceiverLayersLoop11Result::MlPlanRustInherentReceiverLayersLoop11Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustInherentReceiverLayersLoop10Result {
    MlPlanRustInherentReceiverLayersLoop10Done,
    MlPlanRustInherentReceiverLayersLoop10Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_inherent_receiver_layers_loop10(
    ml_values8: &[String],
    mut ml_index8: self::ml::Big,
) -> self::MlPlanRustInherentReceiverLayersLoop10Result {
    loop {
        return if ml_index8 < (self::ml::Big::from_u128(ml_values8.len() as u128)) {
            {
                let layer = self::ml_array::at(ml_values8, ml_index8.to_index());
                if (layer != "reference") && (layer != "box") {
                    self::MlPlanRustInherentReceiverLayersLoop10Result::MlPlanRustInherentReceiverLayersLoop10Return(vec![0f64])
                } else {
                    {
                        let ml_index8_2 = ml_index8.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index8 = ml_index8_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlPlanRustInherentReceiverLayersLoop10Result::MlPlanRustInherentReceiverLayersLoop10Done
        };
    }
}
