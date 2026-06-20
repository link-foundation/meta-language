mod grammar_pipeline_support;

use grammar_pipeline_support::{
    assert_runtime_accepts_all, assert_runtime_rejects, compile_and_run_rust_parser,
    infer_valid_grammar, ARITH_EXAMPLES,
};
use meta_language::{emit_rust_parser, import_pest, write_grammar_surface, GrammarRule};

fn main() -> Result<(), String> {
    let grammar = infer_valid_grammar(ARITH_EXAMPLES);
    let (artifacts, report) = emit_rust_parser(&grammar).map_err(|error| error.to_string())?;

    pest_meta::parse_and_optimize(&artifacts.pest_grammar)
        .map_err(|errors| format!("emitted pest grammar did not validate: {errors:?}"))?;
    let emitted_grammar =
        import_pest(&artifacts.pest_grammar).map_err(|error| error.to_string())?;
    assert_runtime_accepts_all(&emitted_grammar, ARITH_EXAMPLES);
    assert_runtime_rejects(&emitted_grammar, "1++");

    let start_rule = grammar
        .start_rule()
        .map(GrammarRule::name)
        .ok_or_else(|| "inferred grammar has no start rule".to_string())?;
    let examples = ARITH_EXAMPLES
        .iter()
        .map(|example| (*example).to_string())
        .collect::<Vec<_>>();
    compile_and_run_rust_parser(&artifacts, start_rule, &examples, Some("1++"))?;

    println!("corpus: arith");
    for example in ARITH_EXAMPLES {
        println!("  {example}");
    }
    println!("\ninferred grammar:\n{}", write_grammar_surface(&grammar));
    println!("emitted pest grammar:\n{}", artifacts.pest_grammar);
    println!(
        "emitted Rust parser:\n{}{}",
        artifacts.parser_struct, artifacts.ast_types
    );
    if !report.lossy.is_empty() {
        println!("fidelity notes:");
        for note in report.lossy {
            println!("  {note}");
        }
    }
    println!("re-parse: ok");
    Ok(())
}
