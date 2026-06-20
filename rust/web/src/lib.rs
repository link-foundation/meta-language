//! WebAssembly bindings powering the interactive demo on the meta-language
//! website (<https://link-foundation.github.io/meta-language>).
//!
//! The demo is a "Links Notation playground": the visitor types
//! [links-notation](https://crates.io/crates/links-notation) — the structural
//! substrate the meta language uses to serialize its links network — and sees
//! it parsed into a links tree and re-formatted, entirely client-side.
//!
//! Only `links-notation` is wrapped here because it is pure Rust and compiles
//! to `wasm32-unknown-unknown`. The full `meta-language` crate links native
//! tree-sitter grammars and cannot target wasm, so the rich language parsing is
//! showcased through pre-rendered example output on the website instead.

use links_notation::format_config::FormatConfig;
use links_notation::{parse_lino_to_links, LiNo};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

/// Parse links-notation `input` and return a JSON string describing the result.
///
/// Success shape:
/// ```json
/// { "ok": true, "count": 2, "formatted": "...", "tree": [ { ... } ] }
/// ```
///
/// Failure shape:
/// ```json
/// { "ok": false, "error": "Syntax error: ..." }
/// ```
///
/// The function never panics: parse errors are reported in the JSON payload so
/// the browser can render them inline.
#[wasm_bindgen]
#[must_use]
pub fn parse_links_notation(input: &str) -> String {
    let result = match parse_lino_to_links(input) {
        Ok(statements) => {
            let config = FormatConfig::new();
            let tree: Vec<Value> = statements.iter().map(lino_to_json).collect();
            let formatted: Vec<String> = statements
                .iter()
                .map(|statement| statement.format_with_config(&config))
                .collect();
            json!({
                "ok": true,
                "count": statements.len(),
                "formatted": formatted.join("\n"),
                "tree": tree,
            })
        }
        Err(error) => json!({
            "ok": false,
            "error": error.to_string(),
        }),
    };
    result.to_string()
}

/// Convert a single [`LiNo`] node into a JSON value for the browser tree view.
fn lino_to_json(node: &LiNo<String>) -> Value {
    match node {
        LiNo::Ref(value) => json!({ "kind": "ref", "value": value }),
        LiNo::Link { id, values } => json!({
            "kind": "link",
            "id": id,
            "values": values.iter().map(lino_to_json).collect::<Vec<_>>(),
        }),
    }
}
