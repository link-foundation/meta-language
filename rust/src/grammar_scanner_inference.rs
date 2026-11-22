//! Native grammar storage and validation around the shared scanner planner.
use serde::{Deserialize, Serialize};

use crate::grammar::{GrammarScanner, parse_grammar_links, percent_encode_links_text};
use crate::grammar_scanner_inference_rules as rules;
use crate::{TypedProgramLinks, create_source_definition_program_links};

pub(crate) const SCANNER_ALGORITHM: &str = "regular-discovery-scanner-algorithm";

pub(crate) fn create_grammar_scanner_inference_program_links() -> Result<TypedProgramLinks, String>
{
    create_source_definition_program_links(
        "meta-language/source-program/grammar-scanner-inference-program",
    )
}

/// One literal spelling, emitted token and explicit mode transition.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GrammarScannerObservation {
    /// The mode before the scanner request. The initial mode is `default`.
    pub mode: String,
    /// The exact nonempty consumed text.
    pub spelling: String,
    /// The external token selected by the scanner.
    pub token: String,
    /// The mode after emitting the token.
    pub next_mode: String,
}

/// Native, inspectable declarations that can be composed into a grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferredGrammarScanner {
    /// Native Links Notation declarations.
    pub listing: String,
    /// Additional modes, with the implicit `default` omitted.
    pub modes: Vec<String>,
    /// The generated operation program and its emitted external token names.
    pub scanner: GrammarScanner,
}

fn fail(reason: &str) -> String {
    format!("grammar scanner inference: {reason}")
}

/// Infer a finite scanner with an explicit mode, longest valid spelling and
/// constant mode changes from the original ordered trace.
///
/// # Errors
/// Refuses resource exhaustion, malformed observations and conflicting
/// outputs for the same state and spelling.
pub fn infer_grammar_scanner(
    observations: &[GrammarScannerObservation],
    name: Option<&str>,
) -> Result<InferredGrammarScanner, String> {
    #[allow(clippy::cast_precision_loss)]
    if observations.len() as f64 > rules::read_grammar_scanner_observation_limit() {
        return Err(fail(&rules::read_grammar_scanner_inference_error(-1.0)));
    }
    let name = name.unwrap_or("inferred_scanner");
    if name.is_empty() || name.len() > 4096 {
        return Err(fail("invalid scanner name"));
    }
    let mut columns: [Vec<String>; 4] = std::array::from_fn(|_| Vec::new());
    let mut bytes = 0usize;
    for row in observations {
        let values = [&row.mode, &row.spelling, &row.token, &row.next_mode];
        for (column, value) in columns.iter_mut().zip(values) {
            if value.is_empty() {
                return Err(fail(&rules::read_grammar_scanner_inference_error(-1.0)));
            }
            bytes = bytes
                .checked_add(value.len())
                .ok_or_else(|| fail("observation byte limit exceeded"))?;
            #[allow(clippy::cast_precision_loss)]
            if bytes as f64 > rules::read_grammar_scanner_observation_byte_limit() {
                return Err(fail("observation byte limit exceeded"));
            }
            column.push(percent_encode_links_text(value));
        }
    }
    let [modes, spellings, tokens, next_modes] = columns;
    let plan = rules::plan_grammar_scanner_transitions(&modes, &spellings, &tokens, &next_modes);
    let reason = rules::read_grammar_scanner_inference_error(plan[0]);
    if !reason.is_empty() {
        return Err(fail(&reason));
    }
    let listing = rules::render_grammar_scanner_definition(
        &modes,
        &spellings,
        &tokens,
        &next_modes,
        &plan,
        &percent_encode_links_text(name),
    );
    let grammar = parse_grammar_links(&format!(
        "(grammar (start inferred_scanner_empty))\n{listing}(rule inferred_scanner_empty normal empty)\n"
    ))
    .map_err(|error| fail(&error.to_string()))?;
    let declarations = grammar.declarations();
    Ok(InferredGrammarScanner {
        listing,
        modes: declarations.modes.clone(),
        scanner: declarations
            .scanners
            .first()
            .ok_or_else(|| fail("the generated scanner declaration is missing"))?
            .clone(),
    })
}
