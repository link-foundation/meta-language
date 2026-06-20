mod grammar_pipeline_support;

use grammar_pipeline_support::{
    infer_valid_grammar, run_javascript_parser, ExternalRun, CSV_ROW_EXAMPLES,
};
use meta_language::{emit_javascript_parser, write_grammar_surface};

fn main() -> Result<(), String> {
    let grammar = infer_valid_grammar(CSV_ROW_EXAMPLES);
    let (artifacts, report) =
        emit_javascript_parser(&grammar).map_err(|error| error.to_string())?;

    let examples = CSV_ROW_EXAMPLES
        .iter()
        .map(|example| (*example).to_string())
        .collect::<Vec<_>>();
    match run_javascript_parser(&artifacts.module, &examples, Some("a,,"))? {
        ExternalRun::Ran => println!("generated JavaScript parser: ok"),
        ExternalRun::Skipped(reason) => println!("generated JavaScript parser: skipped ({reason})"),
    }

    println!("corpus: csv-row");
    for example in CSV_ROW_EXAMPLES {
        println!("  {example}");
    }
    println!("\ninferred grammar:\n{}", write_grammar_surface(&grammar));
    println!("emitted Peggy grammar:\n{}", artifacts.peggy_grammar);
    println!("emitted JavaScript module:\n{}", artifacts.module);
    if !report.lossy.is_empty() {
        println!("fidelity notes:");
        for note in report.lossy {
            println!("  {note}");
        }
    }
    Ok(())
}
