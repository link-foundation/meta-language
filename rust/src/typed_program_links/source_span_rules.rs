// Generated through the checked JavaScript translator. Do not edit.
/// Apply the shared source-span offset policy.
#[must_use]
pub fn accept_source_span_offset(value: f64, negative_zero: bool) -> bool {
    !negative_zero && (0.0_f64..=9_007_199_254_740_991_f64).contains(&value) && value % 1f64 == 0f64
}

/// Apply the shared source-span offset policy.
#[must_use]
pub const fn accept_source_span_interval(start: f64, end: f64) -> bool {
    start <= end
}

/// Apply the shared source-span offset policy.
#[must_use]
pub fn accept_complete_declaration_source_interval(
    start: f64,
    prefix_end: f64,
    consumed_end: f64,
) -> bool {
    (((self::accept_source_span_offset(start, false)
        && self::accept_source_span_offset(prefix_end, false))
        && self::accept_source_span_offset(consumed_end, false))
        && self::accept_source_span_interval(start, prefix_end))
        && self::accept_source_span_interval(prefix_end, consumed_end)
}
