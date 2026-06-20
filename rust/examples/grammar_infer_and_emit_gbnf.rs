mod grammar_pipeline_support;

use grammar_pipeline_support::{assert_runtime_accepts_all, infer_valid_grammar, JSONISH_EXAMPLES};
use meta_language::{emit_gbnf, import_gbnf, write_grammar_surface};

fn main() -> Result<(), String> {
    let grammar = infer_valid_grammar(JSONISH_EXAMPLES);
    let (gbnf, report) = emit_gbnf(&grammar).map_err(|error| error.to_string())?;
    let reparsed = import_gbnf(&gbnf).map_err(|error| error.to_string())?;
    assert!(
        reparsed.undefined_nonterminals().is_empty(),
        "GBNF re-import has undefined non-terminals: {:#?}",
        reparsed.undefined_nonterminals()
    );
    assert_runtime_accepts_all(&reparsed, JSONISH_EXAMPLES);
    let (re_emitted, _) = emit_gbnf(&reparsed).map_err(|error| error.to_string())?;
    assert_eq!(gbnf, re_emitted);

    println!("corpus: json-ish");
    for example in JSONISH_EXAMPLES {
        println!("  {example}");
    }
    println!("\ninferred grammar:\n{}", write_grammar_surface(&grammar));
    println!("emitted GBNF grammar:\n{gbnf}");
    if !report.lossy.is_empty() {
        println!("fidelity notes:");
        for note in report.lossy {
            println!("  {note}");
        }
    }
    println!("GBNF re-import: ok");
    Ok(())
}
