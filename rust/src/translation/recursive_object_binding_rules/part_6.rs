// Generated from complete js/src/translation/recursive-object-binding-rules.js. Do not edit by hand.
use super::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result;
use super::contains_java_script_recursive_object_binding;
use super::ml_array;
use super::read_required_number_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_contains_java_script_recursive_object_formals_loop30(
    plan: &[f64],
    mut at: f64,
) -> self::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result {
    loop {
        return if at < (plan.len() as f64) {
            if (self::read_required_number_fact(plan, at)) == 0f64 {
                {
                    let at_2 = at + 3f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let count = self::read_required_number_fact(plan, (at + 2f64));
                    if self::contains_java_script_recursive_object_binding(
                        &(self::ml_array::slice(plan, (at + 3f64), ((at + 3f64) + count))),
                    ) {
                        self::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Return(true)
                    } else {
                        {
                            let at_3 = at + (3f64 + count);
                            {
                                at = at_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Done
        };
    }
}
