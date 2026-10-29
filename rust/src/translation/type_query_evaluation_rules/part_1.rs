// Generated from complete js/src/translation/type-query-evaluation-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_type_query_evaluation_plan(
    java_script: bool,
    known: bool,
    pure_source: bool,
    pure_value: bool,
    undefined_value: bool,
) -> f64 {
    if java_script && undefined_value {
        3f64
    } else if known {
        if pure_source && pure_value {
            1f64
        } else if java_script {
            2f64
        } else {
            0f64
        }
    } else {
        0f64
    }
}
