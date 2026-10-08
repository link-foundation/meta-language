//! Layout of emitted Rust that does not fit 100 columns (issue #217).

use meta_language::translation::rust_layout::{RUST_WIDTH, wrap_rust};

#[test]
fn a_short_line_is_kept() {
    assert_eq!(
        wrap_rust("let x = vec![1, 2];", RUST_WIDTH),
        "let x = vec![1, 2];"
    );
}

#[test]
fn a_long_list_goes_one_item_per_line() {
    let line = format!(
        "    let words = vec![{}];",
        (0..30)
            .map(|index| format!("\"word{index}\""))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let wrapped = wrap_rust(&line, RUST_WIDTH);
    assert!(wrapped.starts_with("    let words = vec![\n        \"word0\",\n"));
    assert!(wrapped.ends_with("\n        \"word29\",\n    ];"));
    assert!(
        wrapped
            .lines()
            .all(|line| line.chars().count() <= RUST_WIDTH)
    );
}

#[test]
fn a_parenthesized_expression_never_becomes_a_tuple() {
    let line = "    let total = (alpha_value_long_name + beta_value_long_name + gamma_value_long_name + delta_value) * 2;";
    let wrapped = wrap_rust(line, RUST_WIDTH);
    assert_eq!(
        wrapped,
        "    let total = (\n        alpha_value_long_name + beta_value_long_name + gamma_value_long_name + delta_value\n    ) * 2;"
    );
}

#[test]
fn brackets_and_commas_inside_literals_are_text() {
    let line = format!("    let text = \"{}\";", "a, (b) [c], ".repeat(12));
    assert_eq!(wrap_rust(&line, RUST_WIDTH), line);
}
