//! Prints the tree-sitter-cmake tree of the issue 195 CMake recovery row for
//! the scanner initial state selected at build time.
//!   INITIAL_TOKEN=zero|garbage|none cargo run -q
use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_cmake() -> *const ();
}

fn main() {
    let language: Language = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_cmake) }.into();
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    for source in ["project(Demo\nset(X 1)\n", "set(X [[a]])\n#[[c]]\n"] {
        let tree = parser.parse(source, None).unwrap();
        println!("{:?} => {}", source, tree.root_node().to_sexp());
    }
}
