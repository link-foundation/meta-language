// Generated from complete js/src/translation/conditional-constructor-sharing-rules.js. Do not edit by hand.
use super::MlTemporaryPlanConditionalConstructorFieldSelectionLoop4Result;
use super::ml;
use super::ml_array;
use super::ml_math;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_conditional_constructor_field_selection_loop4(
    classes: &[f64],
    source_ml_temporary_values2: &[f64],
    mut source_ml_temporary_index2: self::ml::Big,
) -> self::MlTemporaryPlanConditionalConstructorFieldSelectionLoop4Result {
    loop {
        return if source_ml_temporary_index2
            < (self::ml::Big::from_u128(source_ml_temporary_values2.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_temporary_values2,
                    source_ml_temporary_index2.to_index(),
                );
                if (!(self::ml_math::is_safe_integer(value)) || (value < 0f64))
                    || (value >= (classes.len() as f64))
                {
                    self::MlTemporaryPlanConditionalConstructorFieldSelectionLoop4Result::MlTemporaryPlanConditionalConstructorFieldSelectionLoop4Return(Vec::<f64>::new())
                } else {
                    {
                        let source_ml_temporary_index2_2 =
                            source_ml_temporary_index2.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index2 = source_ml_temporary_index2_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanConditionalConstructorFieldSelectionLoop4Result::MlTemporaryPlanConditionalConstructorFieldSelectionLoop4Done
        };
    }
}
