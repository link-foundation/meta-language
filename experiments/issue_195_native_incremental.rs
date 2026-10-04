//! Does native tree-sitter's incremental reparse give the same tree as a
//! fresh parse for the edit sequence the issue 195 generative run-time pass
//! found (seed "d", JavaScript case 141)? Run by copying into rust/examples/
//! and `cargo run --example issue_195_native_incremental`.
use tree_sitter::{InputEdit, Parser, Point, Tree};

fn point(text: &str, offset: usize) -> Point {
    let before = &text.as_bytes()[..offset];
    let row = before.iter().filter(|byte| **byte == b'\n').count();
    let column = offset - before.iter().rposition(|byte| *byte == b'\n').map_or(0, |at| at + 1);
    Point { row, column }
}

fn main() {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_javascript::LANGUAGE.into()).unwrap();
    let mut text = "\nif (x)\n  y;\nelse if (a)\n  b;\n\nif (a) {\n  c;\n  d;\n} else {\n  e;\n}\n".to_string();
    let edits: [(usize, usize, &str); 6] = [(61, 66, "日本"), (35, 67, ""), (12, 12, "`"), (29, 29, "/*"), (29, 29, "=>"), (27, 40, "")];
    // Chained: each step reuses the previous tree (the old tree is reparsed each step in meta-language).
    let mut chained: Tree = parser.parse(&text, None).unwrap();
    let mut per_step_ok = true;
    for (start, end, replacement) in edits {
        let old = text.clone();
        text = format!("{}{}{}", &old[..start], replacement, &old[end..]);
        let edit = InputEdit {
            start_byte: start,
            old_end_byte: end,
            new_end_byte: start + replacement.len(),
            start_position: point(&old, start),
            old_end_position: point(&old, end),
            new_end_position: point(&text, start + replacement.len()),
        };
        // meta-language style: fresh parse of old text, edit, reparse.
        let mut old_tree = parser.parse(&old, None).unwrap();
        old_tree.edit(&edit);
        let single = parser.parse(&text, Some(&old_tree)).unwrap();
        chained.edit(&edit);
        chained = parser.parse(&text, Some(&chained)).unwrap();
        let fresh = parser.parse(&text, None).unwrap();
        let same = single.root_node().to_sexp() == fresh.root_node().to_sexp();
        per_step_ok &= same;
        println!("{text:?}\n  single-step incremental == fresh: {same}\n  chained == fresh: {}", chained.root_node().to_sexp() == fresh.root_node().to_sexp());
        if !same {
            println!("  incremental {}\n  fresh       {}", single.root_node().to_sexp(), fresh.root_node().to_sexp());
        }
    }
    println!("all single steps agree: {per_step_ok}");
}
