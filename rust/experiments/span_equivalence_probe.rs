// Dumps every source span a parse produces, for a battery of sources chosen to
// stress point resolution: CRLF line endings, multi-byte characters, missing
// trailing newlines, embedded regions, and the CSS path that appends a
// synthetic semicolon before parsing.
//
// Issue #193 replaced the per-lookup rescan of the source with a shared line
// index. Running this probe on the commit before that change and on the commit
// after it must produce byte-identical output:
//
//   git worktree add /tmp/pre-193 <commit-before>
//   cp experiments/span_equivalence_probe.rs examples/
//   cargo run --quiet --example span_equivalence_probe > /tmp/spans-after.txt
//   (repeat inside /tmp/pre-193/rust) > /tmp/spans-before.txt
//   diff /tmp/spans-before.txt /tmp/spans-after.txt
use meta_language::{LinkNetwork, ParseConfiguration};

const CASES: &[(&str, &str)] = &[
    ("Rust", "fn main() {\n    let s = \"héllo wörld\";\n}\n"),
    ("Rust", "fn main() {\r\n    let s = \"héllo\";\r\n}"),
    ("Rust", ""),
    ("Rust", "\n\n\n"),
    ("Python", "def ítem(value):\n    return value\n"),
    ("JavaScript", "const ünit = 1;\nconst other = ünit + 2;\n"),
    ("JSON", "{\n  \"kéy\": [1, 2, 3],\n  \"other\": null\n}\n"),
    ("CSV", "náme,value\n\"quoted, cell\",2\n"),
    ("Lino", "(itém: source target)\n(other: a b)\n"),
    ("Markdown", "# Título\n\nTexto ção aquí.\n\n```rust\nfn ítem() {}\n```\n"),
    ("Markdown", "Inline <b>bóld</b> html.\n"),
    (
        "HTML",
        "<p style=\"color: réd\">Té</p>\n<style>\n.a { color: blue }\n</style>\n",
    ),
    ("English", "Hawaii is a state.\nThe naïve parser reads it.\n"),
    ("Russian", "Гавайи это штат.\nВторая строка здесь.\n"),
    ("Mandarin Chinese", "这是一个测试。\n第二行。\n"),
];

fn main() {
    for (language, source) in CASES {
        println!("=== {language} ({} bytes) ===", source.len());
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        println!("round-trip: {}", network.reconstruct_text() == *source);
        for link in network.links() {
            let Some(span) = link.metadata().span() else {
                continue;
            };
            println!(
                "{:?} lang={:?} term={:?} bytes={}..{} start=({},{}) end=({},{})",
                link.metadata().link_type(),
                link.metadata().language(),
                link.metadata().term(),
                span.byte_range().start(),
                span.byte_range().end(),
                span.start_point().row(),
                span.start_point().column(),
                span.end_point().row(),
                span.end_point().column(),
            );
        }
    }
}
