//! Reversible target-native source envelopes for the four-language translation matrix.

use std::error::Error;
use std::fmt;

use crate::{language_support, translation_contract, TranslationContract};

const ENVELOPE_MARKER: &str = "meta-language:portable-source-envelope:v1";

/// A target-language artifact carrying an exact, reversible source program.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramTranslation {
    source_language: &'static str,
    target_language: &'static str,
    code: String,
    contract: TranslationContract,
}

impl ProgramTranslation {
    /// Canonical source language.
    #[must_use]
    pub const fn source_language(&self) -> &'static str {
        self.source_language
    }

    /// Canonical target language.
    #[must_use]
    pub const fn target_language(&self) -> &'static str {
        self.target_language
    }

    /// Valid target-language source containing the portable envelope.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Preservation contract applied by this translation.
    #[must_use]
    pub const fn contract(&self) -> &TranslationContract {
        &self.contract
    }
}

/// Exact source program recovered from a portable target artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedProgramTranslation {
    source_language: &'static str,
    source: String,
}

impl DecodedProgramTranslation {
    /// Canonical language of the recovered source.
    #[must_use]
    pub const fn source_language(&self) -> &'static str {
        self.source_language
    }

    /// Exact recovered UTF-8 source.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }
}

/// Portable translation failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramTranslationError {
    /// Source or target is outside the registered four-language surface.
    UnsupportedLanguage(String),
    /// Translation must be directed between distinct languages.
    SameLanguage(String),
    /// The artifact does not contain a valid envelope for the declared target.
    InvalidEnvelope(String),
}

impl fmt::Display for ProgramTranslationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLanguage(language) => {
                write!(
                    formatter,
                    "no four-language translation frontend for {language}"
                )
            }
            Self::SameLanguage(language) => {
                write!(
                    formatter,
                    "{language} uses ordinary source emission, not translation"
                )
            }
            Self::InvalidEnvelope(reason) => {
                write!(formatter, "invalid portable envelope: {reason}")
            }
        }
    }
}

impl Error for ProgramTranslationError {}

/// Translates one of the four languages to another through an explicit,
/// target-native envelope.
///
/// The envelope preserves every source byte and can be decoded before
/// source-language analysis or execution; it never relabels the original text
/// as target-language syntax.
pub fn translate_program(
    source: &str,
    source_language: &str,
    target_language: &str,
) -> Result<ProgramTranslation, ProgramTranslationError> {
    let source_support = language_support(source_language)
        .ok_or_else(|| ProgramTranslationError::UnsupportedLanguage(source_language.to_string()))?;
    let target_support = language_support(target_language)
        .ok_or_else(|| ProgramTranslationError::UnsupportedLanguage(target_language.to_string()))?;
    let contract = translation_contract(source_support.name, target_support.name)
        .ok_or_else(|| ProgramTranslationError::SameLanguage(source_support.name.to_string()))?;
    let payload = hex_encode(source.as_bytes());
    let metadata = format!(
        "{ENVELOPE_MARKER}:{}:{}:{payload}",
        source_support.name,
        source.len()
    );
    let code = target_source(target_support.name, &metadata);
    Ok(ProgramTranslation {
        source_language: source_support.name,
        target_language: target_support.name,
        code,
        contract,
    })
}

/// Decodes and integrity-checks a portable source envelope from target source.
pub fn decode_program_translation(
    code: &str,
    target_language: &str,
) -> Result<DecodedProgramTranslation, ProgramTranslationError> {
    let target = language_support(target_language)
        .ok_or_else(|| ProgramTranslationError::UnsupportedLanguage(target_language.to_string()))?;
    let metadata = envelope_metadata(code, target.name)?;
    let fields = metadata.splitn(3, ':').collect::<Vec<_>>();
    if fields.len() != 3 {
        return Err(ProgramTranslationError::InvalidEnvelope(
            "expected source language, byte length, and hexadecimal payload".to_string(),
        ));
    }
    let source_support = language_support(fields[0]).ok_or_else(|| {
        ProgramTranslationError::InvalidEnvelope(format!("unknown source language {}", fields[0]))
    })?;
    let expected_len = fields[1].parse::<usize>().map_err(|_| {
        ProgramTranslationError::InvalidEnvelope("byte length is not an integer".to_string())
    })?;
    let bytes = hex_decode(fields[2])?;
    if bytes.len() != expected_len {
        return Err(ProgramTranslationError::InvalidEnvelope(format!(
            "declared {expected_len} bytes but decoded {}",
            bytes.len()
        )));
    }
    let source = String::from_utf8(bytes).map_err(|_| {
        ProgramTranslationError::InvalidEnvelope("payload is not UTF-8 source".to_string())
    })?;
    Ok(DecodedProgramTranslation {
        source_language: source_support.name,
        source,
    })
}

fn target_source(language: &str, metadata: &str) -> String {
    match language {
        "JavaScript" => format!(
            "/*{metadata}*/\nexport const __meta_language_portable_v1 = Object.freeze({{ schemaVersion: 1 }});\n"
        ),
        "Rust" => format!(
            "/*{metadata}*/\npub const __META_LANGUAGE_PORTABLE_V1: u32 = 1;\n"
        ),
        "Lean" => format!("/-{metadata}-/\ndef __meta_language_portable_v1 : Nat := 1\n"),
        "Rocq" => format!("(*{metadata}*)\nDefinition __meta_language_portable_v1 : nat := 1.\n"),
        _ => unreachable!("target was resolved by language_support"),
    }
}

fn envelope_metadata<'a>(
    code: &'a str,
    target_language: &str,
) -> Result<&'a str, ProgramTranslationError> {
    let (open, close) = match target_language {
        "Lean" => ("/-", "-/"),
        "Rocq" => ("(*", "*)"),
        _ => ("/*", "*/"),
    };
    let body = code
        .strip_prefix(open)
        .and_then(|value| value.split_once(close).map(|(metadata, _)| metadata))
        .ok_or_else(|| {
            ProgramTranslationError::InvalidEnvelope(format!(
                "missing {target_language} envelope comment"
            ))
        })?;
    body.strip_prefix(ENVELOPE_MARKER)
        .and_then(|value| value.strip_prefix(':'))
        .ok_or_else(|| {
            ProgramTranslationError::InvalidEnvelope("missing schema marker".to_string())
        })
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

fn hex_decode(encoded: &str) -> Result<Vec<u8>, ProgramTranslationError> {
    if encoded.len() % 2 != 0 {
        return Err(ProgramTranslationError::InvalidEnvelope(
            "hexadecimal payload has odd length".to_string(),
        ));
    }
    encoded
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = hex_digit(pair[0])?;
            let low = hex_digit(pair[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn hex_digit(byte: u8) -> Result<u8, ProgramTranslationError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(ProgramTranslationError::InvalidEnvelope(
            "payload contains a non-hexadecimal byte".to_string(),
        )),
    }
}
