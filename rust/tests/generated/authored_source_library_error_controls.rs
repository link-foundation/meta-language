// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
    }

    /// The index a Number names: a non-negative integer, -0 included.
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> f64 {
        if values.is_empty() { return -1.0; }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) { return -1.0; }
            let start = if from < 0.0 { (values.len() as f64 + from) as usize } else { from.min((values.len() - 1) as f64) as usize };
            (0..=start).rev().find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 { (values.len() as f64 + from).max(0.0) as usize } else { from.min(values.len() as f64) as usize };
            (start..values.len()).find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        }
    }
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
        }
        out
    }
}

pub fn read_source_library_error_profile_error(profile: String, dependencies: bool, aliases: bool, ordinary_classes: String) -> String {
    if (profile == "") {
        String::from("")
    } else {
        if ((((profile == "resolved supplied Error source modules 1") && dependencies) && aliases) && (ordinary_classes == "")) {
            String::from("")
        } else {
            String::from("Error source linkage requires its exact supplied modules, resolved aliases and separate semantic class profile")
        }
    }
}

pub fn accept_source_library_error_declaration_owner(source_name: String, compiled_name: String, count: f64) -> bool {
    (((source_name != "") && (compiled_name != "")) && (count == (1f64)))
}

pub fn accept_source_library_error_entry_owner(exports: Vec<String>, entry: String, count: f64) -> bool {
    (((entry != "") && self::ml_array::includes(&exports, &entry.clone(), 0f64, false, |a, b| a == b)) && (count == (1f64)))
}

fn ml_main() {
    assert!(((self::read_source_library_error_profile_error(String::from(""), false, false, String::from(""))) == ""), "assertion 1");
    assert!(((self::read_source_library_error_profile_error(String::from("resolved supplied Error source modules 1"), true, true, String::from(""))) == ""), "assertion 2");
    assert!(((self::read_source_library_error_profile_error(String::from("resolved supplied Error source modules 1"), false, true, String::from(""))) == "Error source linkage requires its exact supplied modules, resolved aliases and separate semantic class profile"), "assertion 3");
    assert!(((self::read_source_library_error_profile_error(String::from("resolved supplied Error source modules 1"), true, false, String::from(""))) == "Error source linkage requires its exact supplied modules, resolved aliases and separate semantic class profile"), "assertion 4");
    assert!(((self::read_source_library_error_profile_error(String::from("resolved supplied Error source modules 1"), true, true, String::from("closed local class declarations 1"))) == "Error source linkage requires its exact supplied modules, resolved aliases and separate semantic class profile"), "assertion 5");
    assert!(((self::accept_source_library_error_declaration_owner(String::from("unsupported"), String::from("dependencyFunction1"), 1f64)) == (true)), "assertion 6");
    assert!(((self::accept_source_library_error_declaration_owner(String::from("unsupported"), String::from("dependencyFunction1"), 2f64)) == (false)), "assertion 7");
    assert!(((self::accept_source_library_error_declaration_owner(String::from(""), String::from("dependencyFunction1"), 1f64)) == (false)), "assertion 8");
    assert!(((self::accept_source_library_error_entry_owner(vec![String::from("run")], String::from("run"), 1f64)) == (true)), "assertion 9");
    assert!(((self::accept_source_library_error_entry_owner(vec![String::from("run")], String::from("unsupported"), 1f64)) == (false)), "assertion 10");
    assert!(((self::accept_source_library_error_entry_owner(vec![String::from("run")], String::from("run"), 2f64)) == (false)), "assertion 11");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_source_library_error_controls() { main(); }
