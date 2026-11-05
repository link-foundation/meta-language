// Generated from complete js/src/translation/array-classification-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_classification(kind: &str) -> bool {
    (kind == "array") || (kind == "tuple")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_classification_plan(kind: &str, element: &str) -> Vec<bool> {
    {
        let selected = if kind == "option" {
            element.to_owned()
        } else {
            kind.to_owned()
        };
        if self::read_java_script_array_classification(&(selected)) {
            vec![true, (kind == "option")]
        } else if (((((selected == "unit") || (selected == "absence")) || (selected == "bool"))
            || (selected == "string"))
            || (((((selected == "float") || (selected == "int")) || (selected == "nat"))
                || (selected == "fixed"))
                || (selected == "literal")))
            || ((((((selected == "record") || (selected == "data")) || (selected == "set"))
                || (selected == "map"))
                || (selected == "cell"))
                || (selected == "function"))
        {
            vec![false, false]
        } else {
            Vec::<bool>::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_classification_builtin_operation(name: &str) -> String {
    if name == "Array.isArray" {
        String::from("testJavaScriptArrayValue")
    } else {
        String::new()
    }
}
