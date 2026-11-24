// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

pub fn test_java_script_host_capability_mode(mode: String) -> bool {
    (mode == "qualified host capability")
}

fn ml_main() {
    assert!(((self::test_java_script_host_capability_mode(String::from("qualified host capability"))) == (true)), "assertion 1");
    assert!(((self::test_java_script_host_capability_mode(String::from("Error intrinsic"))) == (false)), "assertion 2");
    assert!(((self::test_java_script_host_capability_mode(String::from("strict"))) == (false)), "assertion 3");
    assert!(((self::test_java_script_host_capability_mode(String::from("lexical"))) == (false)), "assertion 4");
    assert!(((self::test_java_script_host_capability_mode(String::from("Qualified host capability"))) == (false)), "assertion 5");
    assert!(((self::test_java_script_host_capability_mode(String::from(""))) == (false)), "assertion 6");
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
fn all_literal_host_capability_mode_controls() { main(); }
