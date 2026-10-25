// Generated from complete js/src/translation/rust-source-constant-rules.js. Do not edit by hand.
/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_source_constant_value(kind: &str, owner: &str, temporary: &str) -> Vec<String> {
    if kind == "const" {
        vec![
            String::from("let"),
            temporary.to_owned(),
            String::from("name"),
            owner.to_owned(),
            String::from("name"),
            temporary.to_owned(),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_callable_value_error(kind: &str, role: &str) -> String {
    if kind == "function" {
        String::new()
    } else {
        format!("the source {} is not callable", role.to_owned())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_constant_pattern_error(declaration: &str) -> String {
    if declaration == "const" {
        String::from("source constant patterns require their represented value and type")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_value_path(module_path: &[String], resolved: &[String]) -> Vec<String> {
    if (resolved.len() as f64) == 1f64 {
        self::ml_array::append(
            self::ml_array::append(vec![String::from("crate")], module_path.to_vec()),
            resolved.to_vec(),
        )
    } else {
        resolved.to_vec()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_call_form(ordinary: bool, count: f64) -> String {
    if ordinary || (count != 0f64) {
        String::from("app")
    } else {
        String::from("name")
    }
}
