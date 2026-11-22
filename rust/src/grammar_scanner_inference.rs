//! Native grammar storage and validation around the shared scanner planner.
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::grammar::{GrammarScanner, parse_grammar_links, percent_encode_links_text};
use crate::grammar_scanner_character_class_rules as classes;
use crate::grammar_scanner_inference_rules as rules;
use crate::grammar_scanner_mode_stack_rules as mode_stack;
use crate::{TypedProgramLinks, create_source_definition_program_links};

pub(crate) const SCANNER_ALGORITHM: &str = "regular-discovery-scanner-algorithm";
pub(crate) const SCANNER_CLASS_ALGORITHM: &str =
    "regular-discovery-scanner-character-class-algorithm";
pub(crate) const SCANNER_MODE_STACK_ALGORITHM: &str =
    "regular-discovery-scanner-mode-stack-algorithm";

pub(crate) fn create_grammar_scanner_inference_program_links() -> Result<TypedProgramLinks, String>
{
    create_source_definition_program_links(
        "meta-language/source-program/grammar-scanner-inference-program",
    )
}

pub(crate) fn create_grammar_scanner_character_class_program_links()
-> Result<TypedProgramLinks, String> {
    create_source_definition_program_links(
        "meta-language/source-program/grammar-scanner-character-class-program",
    )
}

pub(crate) fn create_grammar_scanner_mode_stack_program_links() -> Result<TypedProgramLinks, String>
{
    create_source_definition_program_links(
        "meta-language/source-program/grammar-scanner-mode-stack-program",
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

/// One literal token observation with complete mode stacks, ordered bottom first.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarScannerModeStackObservation {
    /// The modes before the token request.
    pub before: Vec<String>,
    /// The exact nonempty consumed text.
    pub spelling: String,
    /// The external token selected by the scanner.
    pub token: String,
    /// The modes after emitting the token.
    pub after: Vec<String>,
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
    infer_scanner(observations, name, false)
}

/// Infer scanner character classes from observed single Unicode scalars.
///
/// # Errors
/// Refuses excessive, malformed or contradictory observations.
pub fn infer_grammar_scanner_classes(
    observations: &[GrammarScannerObservation],
    name: Option<&str>,
) -> Result<InferredGrammarScanner, String> {
    infer_scanner(observations, name, true)
}

/// Infer a native scanner from single-operation changes to complete mode stacks.
///
/// # Errors
/// Refuses conflicting, malformed, excessive or unsupported mode transitions.
pub fn infer_grammar_scanner_mode_stack(
    observations: &[GrammarScannerModeStackObservation],
    name: Option<&str>,
    character_classes: bool,
) -> Result<InferredGrammarScanner, String> {
    let invalid = || fail("expected bounded nonempty scanner mode stack observations");
    #[allow(clippy::cast_precision_loss)]
    if observations.is_empty()
        || observations.len() as f64 > rules::read_grammar_scanner_observation_limit()
    {
        return Err(invalid());
    }
    let name = name.unwrap_or("inferred_scanner");
    if name.is_empty() || name.len() > 4096 {
        return Err(fail("invalid scanner name"));
    }
    let mut bytes = 0usize;
    let mut encode = |value: &str| {
        if value.is_empty() {
            return Err(invalid());
        }
        bytes = bytes
            .checked_add(value.len())
            .ok_or_else(|| fail("observation byte limit exceeded"))?;
        #[allow(clippy::cast_precision_loss)]
        if bytes as f64 > rules::read_grammar_scanner_observation_byte_limit() {
            return Err(fail("observation byte limit exceeded"));
        }
        Ok(percent_encode_links_text(value))
    };
    let [mut modes, mut spellings, mut tokens, mut effects]: [Vec<String>; 4] =
        std::array::from_fn(|_| Vec::new());
    let mut code_points = Vec::new();
    let mut observed_modes = Vec::new();
    let mut seen_modes = BTreeSet::new();
    for row in observations {
        let mut encode_stack = |stack: &[String]| {
            #[allow(clippy::cast_precision_loss)]
            if stack.is_empty()
                || stack.len() as f64 > mode_stack::read_grammar_scanner_mode_stack_limit()
            {
                return Err(invalid());
            }
            stack
                .iter()
                .map(|mode| {
                    let value = encode(mode)?;
                    if seen_modes.insert(value.clone()) {
                        observed_modes.push(value.clone());
                    }
                    Ok(value)
                })
                .collect::<Result<Vec<_>, String>>()
        };
        let before = encode_stack(&row.before)?;
        let after = encode_stack(&row.after)?;
        spellings.push(encode(&row.spelling)?);
        tokens.push(encode(&row.token)?);
        let effect = mode_stack::read_grammar_scanner_mode_stack_operation(&before, &after);
        if effect.is_empty() {
            return Err(fail(
                "a token observation requires at most one native scanner mode operation",
            ));
        }
        modes.push(before.last().ok_or_else(invalid)?.clone());
        effects.push(effect);
        code_points.push(scanner_spelling_code_point(&row.spelling));
    }
    let plan = rules::plan_grammar_scanner_transitions(&modes, &spellings, &tokens, &effects);
    let reason = rules::read_grammar_scanner_inference_error(plan[0]);
    if !reason.is_empty() {
        return Err(fail(&reason));
    }
    let matchers = if character_classes {
        classes::plan_grammar_scanner_character_classes(
            &modes,
            &spellings,
            &tokens,
            &effects,
            &code_points,
            &plan,
        )
    } else {
        Vec::new()
    };
    read_scanner_listing(mode_stack::render_grammar_scanner_mode_stack_definition(
        &modes,
        &spellings,
        &tokens,
        &effects,
        &observed_modes,
        &code_points,
        &plan,
        &matchers,
        &percent_encode_links_text(name),
    ))
}

fn scanner_spelling_code_point(spelling: &str) -> f64 {
    let mut scalars = spelling.chars();
    match (scalars.next(), scalars.next()) {
        (Some(scalar), None) => f64::from(u32::from(scalar)),
        _ => -1.0,
    }
}

fn infer_scanner(
    observations: &[GrammarScannerObservation],
    name: Option<&str>,
    character_classes: bool,
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
    let mut code_points = Vec::new();
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
        if character_classes {
            code_points.push(scanner_spelling_code_point(&row.spelling));
        }
    }
    let [modes, spellings, tokens, next_modes] = columns;
    let plan = rules::plan_grammar_scanner_transitions(&modes, &spellings, &tokens, &next_modes);
    let reason = rules::read_grammar_scanner_inference_error(plan[0]);
    if !reason.is_empty() {
        return Err(fail(&reason));
    }
    let name = percent_encode_links_text(name);
    let listing = if character_classes {
        let matchers = classes::plan_grammar_scanner_character_classes(
            &modes,
            &spellings,
            &tokens,
            &next_modes,
            &code_points,
            &plan,
        );
        classes::render_grammar_scanner_character_class_definition(
            &modes,
            &tokens,
            &next_modes,
            &code_points,
            &plan,
            &matchers,
            &name,
        )
    } else {
        rules::render_grammar_scanner_definition(
            &modes,
            &spellings,
            &tokens,
            &next_modes,
            &plan,
            &name,
        )
    };
    read_scanner_listing(listing)
}

fn read_scanner_listing(listing: String) -> Result<InferredGrammarScanner, String> {
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
