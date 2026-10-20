// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_owned_outcome_native_number_kind(
    profile_available: bool,
    version: &str,
    arbitrary_precision: bool,
    signed: bool,
    unsigned: bool,
    floating: bool,
) -> String {
    if !profile_available || version != "1.0.151" || arbitrary_precision {
        String::new()
    } else if floating {
        if signed || unsigned {
            String::new()
        } else {
            String::from("floating point")
        }
    } else if unsigned {
        String::from("unsigned integer")
    } else if signed {
        String::from("signed integer")
    } else {
        String::new()
    }
}
