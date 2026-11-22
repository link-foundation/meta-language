// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

pub fn read_rust_owned_standard_sum_predicate_result_error(role: String, identity: bool) -> String {
    if (((role == "copy") || (role == "owned")) && identity) {
        String::from("")
    } else {
        String::from("an eager default or first callback must establish its actual complete Copy or supported owned result type")
    }
}

fn ml_main() {
    assert!(((self::read_rust_owned_standard_sum_predicate_result_error(String::from("copy"), true)) == ""), "assertion 1");
    assert!(((self::read_rust_owned_standard_sum_predicate_result_error(String::from("owned"), true)) == ""), "assertion 2");
    assert!(((self::read_rust_owned_standard_sum_predicate_result_error(String::from(""), true)) == "an eager default or first callback must establish its actual complete Copy or supported owned result type"), "assertion 3");
    assert!(((self::read_rust_owned_standard_sum_predicate_result_error(String::from("copy"), false)) == "an eager default or first callback must establish its actual complete Copy or supported owned result type"), "assertion 4");
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
fn all_literal_owned_sum_predicate_admission_controls() { main(); }
