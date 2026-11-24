// Generated from js/src/morphology-policy.js.
pub(super) fn russian_number_agreement_kind(integer: f64) -> u8 {
    let ones = integer % 10.0;
    let tens = integer % 100.0;
    if ones == 1.0 && tens != 11.0 {
        0
    } else if (2.0..=4.0).contains(&ones) && !(12.0..=14.0).contains(&tens) {
        1
    } else {
        2
    }
}
