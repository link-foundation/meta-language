// Generated host publication from every actual native observation field.
use super::original::NativeJsonNumberObservation;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::io::Write as _;

fn source_pin(bytes: &[u8]) -> Value {
    let digest = Sha256::digest(bytes)
        .iter()
        .fold(String::new(), |mut hexadecimal, byte| {
            write!(hexadecimal, "{byte:02x}").expect("writing into a String succeeds");
            hexadecimal
        });
    json!({"bytes": bytes.len(), "sha256": digest})
}

pub(super) fn publish_original_number_controls(
    observations: &[NativeJsonNumberObservation],
) -> Result<(), String> {
    if std::env::var("META_LANGUAGE_SHARD_PROFILE").as_deref() != Ok("evidence") {
        return Ok(());
    }
    if std::env::var("CI").as_deref() != Ok("true")
        || std::env::var("GITHUB_ACTIONS").as_deref() != Ok("true")
    {
        return Err("native number publication requires the actual CI evidence profile".to_owned());
    }
    let commit = std::env::var("GITHUB_SHA").map_err(|error| error.to_string())?;
    if commit.len() != 40
        || !commit
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        || std::env::var("ISSUE_195_COMMIT").as_deref() != Ok(commit.as_str())
    {
        return Err("native number publication requires the exact executed commit".to_owned());
    }
    if observations.len() > 128 {
        return Err("native number publication observation limit exceeded".to_owned());
    }
    let rows: Vec<Value> = observations
        .iter()
        .map(|observed| {
            json!({
                "wireOccurrence": observed.wire_occurrence,
                "signed": observed.signed,
                "unsigned": observed.unsigned,
                "floating": observed.floating,
                "signedDecimal": observed.signed_decimal,
                "unsignedDecimal": observed.unsigned_decimal,
                "floatingBits": observed.floating_bits
            })
        })
        .collect();
    let report = json!({
        "schemaVersion": 1,
        "profile": "original native number controls",
        "testName": "grammar_outcome_original_numbers::original_borrowed_number_predicates_payloads_and_wire_positions_match_every_independent_control",
        "commit": commit,
        "run": std::env::var("GITHUB_RUN_ID").map_err(|error| error.to_string())?,
        "attempt": std::env::var("GITHUB_RUN_ATTEMPT").map_err(|error| error.to_string())?,
        "targetUsizeBits": usize::BITS,
        "sources": {
            "test": source_pin(include_bytes!("grammar_outcome_original_numbers.rs")),
            "model": source_pin(include_bytes!("native_json_number_observation_model.rs")),
            "visitors": source_pin(include_bytes!("native_json_number_observation_visitors.rs")),
            "publisher": source_pin(include_bytes!("native_json_number_publication.rs")),
            "fixture": source_pin(include_bytes!("../../../parity/fixtures/original-native-number-observations.lino")),
            "lock": source_pin(include_bytes!("../../Cargo.lock"))
        },
        "observations": rows
    });
    let record = meta_language::structured_data_record_to_lino(&report)?;
    if record.len() > 128 * 1024
        || !record.ends_with('\n')
        || record[..record.len() - 1].contains('\n')
    {
        return Err("native number publication frame limit exceeded".to_owned());
    }
    let frame = format!("meta-language original native number controls: {record}");
    std::io::stderr()
        .lock()
        .write_all(frame.as_bytes())
        .map_err(|error| error.to_string())
}
