// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_number_predicate_argument_form(
    language: &str,
    operation: &str,
    kind: &str,
    element: &str,
) -> String {
    if (language != "JavaScript")
        || !((((operation == "isInteger") || (operation == "isSafeInteger"))
            || (operation == "isFinite"))
            || (operation == "isNaN"))
    {
        String::new()
    } else if kind == "float" {
        String::from("number")
    } else if (kind == "option") && (element == "float") {
        String::from("optional-number")
    } else if ((((((((((((kind == "bool") || (kind == "string")) || (kind == "unit"))
        || (kind == "int"))
        || (kind == "nat"))
        || (kind == "array"))
        || (kind == "tuple"))
        || (kind == "record"))
        || (kind == "map"))
        || (kind == "set"))
        || (kind == "function"))
        || (kind == "data"))
        || ((kind == "option")
            && ((((((((((((element == "bool") || (element == "string"))
                || (element == "unit"))
                || (element == "int"))
                || (element == "nat"))
                || (element == "array"))
                || (element == "tuple"))
                || (element == "record"))
                || (element == "map"))
                || (element == "set"))
                || (element == "function"))
                || (element == "data")))
    {
        String::from("false")
    } else {
        String::from("unsupported")
    }
}
