// Generated through the checked JavaScript translator. Do not edit.
/// Apply the shared integral metadata role policy.
#[must_use]
pub fn accept_program_integral_metadata(
    kind: &str,
    field: &str,
    value: f64,
    negative_zero: bool,
) -> bool {
    kind == "fixed"
        && field == "bits"
        && [8f64, 16f64, 32f64, 64f64, 128f64].contains(&value)
        && !negative_zero
        && (0.0_f64..=9_007_199_254_740_991_f64).contains(&value)
        && value % 1f64 == 0f64
}
