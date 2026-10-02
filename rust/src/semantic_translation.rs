//! Semantic translations produced by the portable-core translator.
//!
//! A program in the portable core is parsed, type-checked and emitted as a native target program
//! that prints the same lines and restates the same theorems. [`SemanticTranslation`] records what
//! that translation relied on: its encodings, assumptions, proof obligations, source mappings,
//! runtime dependencies and provenance.
//!
//! Mirrors the semantic half of `js/src/program-translation.js`.

use sha2::{Digest, Sha256};

use crate::translation::check::check_program;
use crate::translation::diagnostics::TranslationError;
use crate::translation::emit_common::{Assumption, Emitted, Encoding, IN_BOUNDS_READS, Mapping};
use crate::translation::{
    Span, emit_javascript::emit_javascript, emit_lean::emit_lean, emit_rocq::emit_rocq,
    emit_rust::emit_rust, javascript::parse_javascript, lean::parse_lean, rocq::parse_rocq,
    rust::parse_rust,
};

const PROVENANCE_MARKER: &str = "meta-language:translation-provenance:v1";
const TRANSLATOR: &str = "meta-language portable-core translator";

/// What a semantic translation preserves: the printed lines and every stated proposition.
pub const SEMANTIC_OBSERVATION: &str = "the lines main prints, in order, on executions that do not abort, and the proposition of every source theorem and assertion over the translated definitions";

/// The contract encoding of a semantic translation.
pub const SEMANTIC_ENCODING: &str =
    "portable-core translation; the encodings it chose are listed in semantics.encodings";

/// How a proof obligation of the source is met in the target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationObligation {
    /// Source theorem, or `main:assert@<offset>` for an assertion.
    pub source: String,
    /// Target declaration that states it.
    pub target: String,
    /// `theorem` or `assertion`.
    pub kind: String,
    /// Whether the statement has no free variables.
    pub closed_goal: bool,
    /// `target-kernel`, `source-kernel` or `runtime-assertion`.
    pub discharge: String,
    /// The target-side check of a source-kernel proof (`bounded`).
    pub check: Option<String>,
}

/// Where a semantic translation came from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationProvenance {
    /// The translator that produced the artifact.
    pub translator: &'static str,
    /// Canonical source language.
    pub source_language: &'static str,
    /// Lowercase hexadecimal SHA-256 of the UTF-8 source.
    pub source_sha256: String,
    /// Source length in bytes.
    pub source_bytes: usize,
    /// The comment the artifact starts with.
    pub header: String,
}

/// The record of one semantic translation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticTranslation {
    /// The target's entry point, when the source has a main program.
    pub entry: Option<String>,
    /// How the target's observations are made.
    pub observation_procedure: &'static str,
    /// Representation choices for constructs without a direct target equivalent.
    pub encodings: Vec<Encoding>,
    /// Assumptions, with the program-specific details.
    pub assumptions: Vec<Assumption>,
    /// Source theorems and assertions and how the target discharges them.
    pub obligations: Vec<TranslationObligation>,
    /// Where each source declaration went, with its source span in UTF-16 code units.
    pub mappings: Vec<Mapping>,
    /// Libraries and host features the emitted program needs.
    pub runtime_dependencies: Vec<String>,
    /// Where the translation came from.
    pub provenance: TranslationProvenance,
}

/// Why a program stayed outside the portable core.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationDiagnostic {
    /// `syntax`, `type` or `unsupported`.
    pub kind: &'static str,
    /// The message with its span suffix.
    pub message: String,
    /// The source range, in UTF-16 code units.
    pub span: Option<Span>,
}

/// Provenance recovered from an artifact's first line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadTranslationProvenance {
    /// Canonical source language.
    pub source_language: &'static str,
    /// Lowercase hexadecimal SHA-256 of the UTF-8 source.
    pub source_sha256: String,
    /// Source length in bytes.
    pub source_bytes: usize,
}

/// Runs the portable-core pipeline for canonical language names.
pub fn emit(
    source: &str,
    source_language: &str,
    target_language: &str,
) -> Result<Emitted, TranslationDiagnostic> {
    let surface = match source_language {
        "JavaScript" => parse_javascript(source),
        "Rust" => parse_rust(source),
        "Lean" => parse_lean(source),
        _ => parse_rocq(source),
    };
    let emitted = surface
        .and_then(|surface| check_program(&surface))
        .and_then(|program| match target_language {
            "JavaScript" => emit_javascript(&program),
            "Rust" => emit_rust(&program),
            "Lean" => emit_lean(&program),
            _ => emit_rocq(&program),
        });
    emitted.map_err(|error: TranslationError| TranslationDiagnostic {
        kind: error.kind.as_str(),
        message: error.message(),
        span: error.span,
    })
}

const fn comment(target_language: &str) -> (&'static str, &'static str) {
    match target_language.as_bytes() {
        b"Lean" => ("-- ", ""),
        b"Rocq" => ("(* ", " *)"),
        _ => ("// ", ""),
    }
}

/// The artifact text and its semantic record.
pub fn record(
    source: &str,
    source_language: &'static str,
    target_language: &str,
    emitted: Emitted,
) -> (String, SemanticTranslation) {
    let source_sha256 = Sha256::digest(source.as_bytes()).iter().fold(
        String::with_capacity(64),
        |mut hex, byte| {
            use std::fmt::Write as _;
            let _ = write!(hex, "{byte:02x}");
            hex
        },
    );
    let (open, close) = comment(target_language);
    let header = format!(
        "{open}{PROVENANCE_MARKER} source={source_language} sha256={source_sha256} bytes={}{close}",
        source.len()
    );
    let kernel_target = matches!(target_language, "Lean" | "Rocq");
    let obligations = emitted
        .theorems
        .into_iter()
        .map(|theorem| TranslationObligation {
            source: theorem.source,
            target: theorem.target,
            kind: theorem.kind,
            closed_goal: theorem.closed_goal,
            discharge: theorem.discharge.unwrap_or_else(|| {
                if kernel_target {
                    "target-kernel"
                } else {
                    "runtime-assertion"
                }
                .to_owned()
            }),
            check: theorem.check,
        })
        .collect();
    let semantics = SemanticTranslation {
        entry: emitted.entry,
        observation_procedure: observation_procedure(target_language),
        encodings: emitted.encodings,
        assumptions: emitted.assumptions,
        obligations,
        mappings: emitted.mappings,
        runtime_dependencies: runtime_dependencies(target_language, &emitted.text),
        provenance: TranslationProvenance {
            translator: TRANSLATOR,
            source_language,
            source_sha256,
            source_bytes: source.len(),
            header: header.clone(),
        },
    };
    (format!("{header}\n{}", emitted.text), semantics)
}

/// The fixed assumption statements a contract lists for these assumptions, in their order.
pub fn assumption_statements(assumptions: &[Assumption]) -> &'static [&'static str] {
    const NONE: &[&str] = &[];
    const READS: &[&str] = &[IN_BOUNDS_READS.statement];
    let ids: Vec<&str> = assumptions
        .iter()
        .map(|assumption| assumption.id.as_str())
        .collect();
    match ids.as_slice() {
        [] => NONE,
        [first] if *first == IN_BOUNDS_READS.id => READS,
        _ => unreachable!("unknown assumptions {ids:?}"),
    }
}

const fn observation_procedure(target_language: &str) -> &'static str {
    match target_language.as_bytes() {
        b"JavaScript" => {
            "run the module with node; --ml-check-theorems evaluates the theorem properties instead of main"
        }
        b"Rust" => {
            "compile with rustc and run the binary; --ml-check-theorems evaluates the theorem properties instead of main"
        }
        b"Lean" => {
            "lean --run executes main after the Lean kernel has checked every definition and theorem"
        }
        _ => {
            "rocq compile checks every definition and proof; Eval vm_compute in main prints the list of output lines"
        }
    }
}

/// The libraries and host features the emitted program needs, read from its text.
fn runtime_dependencies(target_language: &str, code: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    match target_language {
        "JavaScript" => {
            dependencies.push("ECMAScript 2026 host with BigInt".to_owned());
            if code.contains("process.argv") {
                dependencies.push("Node.js process.argv (selects --ml-check-theorems)".to_owned());
            }
        }
        "Rust" => {
            dependencies.push("Rust 1.98.1 standard library".to_owned());
            if code.contains("\npub mod ml {") {
                dependencies.push(
                    "ml::Big arbitrary-precision integers, defined inside the artifact".to_owned(),
                );
            }
            if code.contains("\npub mod ml_number {") {
                dependencies.push(
                    "ml_number JavaScript Number formatting and SameValue, defined inside the artifact"
                        .to_owned(),
                );
            }
        }
        "Lean" => {
            dependencies.push("Lean 4.34.1 core library".to_owned());
            dependencies.extend(
                code.lines()
                    .filter_map(|line| line.strip_prefix("import "))
                    .map(|module| module.trim().to_owned()),
            );
        }
        _ => {
            dependencies.push("Rocq 9.2".to_owned());
            for line in code.lines() {
                let Some(rest) = line.strip_prefix("From ") else {
                    continue;
                };
                let Some((library, modules)) = rest.split_once(" Require Import ") else {
                    continue;
                };
                let Some(modules) = modules.strip_suffix('.') else {
                    continue;
                };
                if library.is_empty() || !library.chars().all(|c| c.is_ascii_alphabetic()) {
                    continue;
                }
                dependencies.extend(
                    modules
                        .split_whitespace()
                        .map(|module| format!("{library}.{module}")),
                );
            }
        }
    }
    dependencies
}

/// Reads the provenance comment a semantic translation starts with.
pub fn read_provenance(
    code: &str,
    target_language: &str,
) -> Result<ReadTranslationProvenance, String> {
    let (open, close) = comment(target_language);
    let first_line = code.split('\n').next().unwrap_or_default();
    let fields = first_line
        .strip_prefix(open)
        .and_then(|line| line.strip_prefix(PROVENANCE_MARKER))
        .and_then(|line| line.strip_prefix(' '))
        .and_then(|line| line.strip_suffix(close))
        .ok_or_else(|| format!("missing {target_language} provenance comment"))?;
    let field = |name: &str| {
        fields.split(' ').find_map(|field| {
            field
                .strip_prefix(name)
                .and_then(|rest| rest.strip_prefix('='))
        })
    };
    let invalid = || "expected source, sha256 and bytes fields".to_owned();
    let source_language = field("source")
        .and_then(crate::language_support)
        .ok_or_else(invalid)?
        .name;
    let source_sha256 = field("sha256")
        .filter(|hash| {
            hash.len() == 64
                && hash
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
        .ok_or_else(invalid)?
        .to_owned();
    let source_bytes = field("bytes")
        .and_then(|bytes| bytes.parse::<usize>().ok())
        .ok_or_else(invalid)?;
    Ok(ReadTranslationProvenance {
        source_language,
        source_sha256,
        source_bytes,
    })
}
