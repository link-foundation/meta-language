//! Actual compiler diagnostics retained beside each unchanged source owner.
use super::{SelfTranslation, SelfTranslationError, SelfTranslationOptions};
use crate::translation::diagnostics::TranslationError;
use serde_json::{Value, json};

/// Independent source translation plus one optional diagnostic per source item.
#[derive(Clone, Debug)]
pub struct SelfTranslationDiagnosticReport {
    /// The ordinary translation, including every carried item.
    pub translation: SelfTranslation,
    /// Actual checked-path errors; null entries mean no error was recorded.
    pub diagnostics: Vec<Option<Value>>,
}

/// Retain actual compiler diagnostics without executing or replacing a product.
///
/// # Errors
/// Returns the ordinary source reconstruction or language diagnostic.
pub fn self_translate_with_diagnostics(
    source: &str,
    source_language: &str,
    target_language: &str,
    options: &SelfTranslationOptions,
) -> Result<SelfTranslationDiagnosticReport, SelfTranslationError> {
    let mut diagnostics = Vec::new();
    let (translation, _) = super::translate_module_diagnostic(
        source,
        source_language,
        target_language,
        options,
        Some(&mut diagnostics),
    )?;
    diagnostics.resize(translation.items.len(), None);
    Ok(SelfTranslationDiagnosticReport {
        translation,
        diagnostics,
    })
}

pub(super) fn retained(error: &TranslationError, start: usize, end: usize) -> Value {
    let mut details = json!({});
    if let Some(construct) = &error.construct {
        details["construct"] = json!(construct);
    }
    json!({"kind":error.kind.as_str(),"reason":error.reason,"message":error.to_string(),
        "span":error.span,"details":details,"sourceRange":{"start":start,"end":end}})
}
