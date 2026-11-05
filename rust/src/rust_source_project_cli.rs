//! Process transport over shared argument and module-composition definitions.
use crate::rust_project_source_rules::{self as rules, ml_string::Text};
use crate::{
    BinaryLinksOptions, create_rust_source_project_program_links, emit_typed_program_links,
    encode_link_network_binary, read_structured_data_lino,
};
use serde_json::json;
const USAGE: &str = "usage: meta-language source-project <inventory.lino> --entry <path> [--library] [--to <JavaScript|Rust|Lean|Rocq> | --binary]\nThe inventory contains inputs (original path/source records) and optional resource options. Default output retains checked semantics and complete original concrete graphs as Links Notation.\n";
/// Public UTF8/binary CLI process output.
pub struct RustSourceProjectCommandOutput {
    /// Process exit code.
    pub exit_code: i32,
    /// Exact output bytes.
    pub stdout: Vec<u8>,
    /// Diagnostic text.
    pub stderr: String,
}
fn failure(code: i32, message: String) -> RustSourceProjectCommandOutput {
    RustSourceProjectCommandOutput {
        exit_code: code,
        stdout: Vec::new(),
        stderr: message,
    }
}
/// Draft a source inventory into complete graph data or selected target code.
pub fn run_rust_source_project_command<F>(
    args: &[String],
    read_file: F,
) -> RustSourceProjectCommandOutput
where
    F: Fn(&str) -> Result<String, String>,
{
    let args = args
        .iter()
        .map(|value| Text::from_text(value))
        .collect::<Vec<_>>();
    let plan =
        match std::panic::catch_unwind(|| rules::read_rust_source_project_command_plan(&args)) {
            Ok(plan) => plan,
            Err(payload) => {
                let message = payload
                    .downcast_ref::<(String, Vec<u16>, bool)>()
                    .map_or_else(
                        || "invalid source project arguments".to_owned(),
                        |(_, units, _)| String::from_utf16_lossy(units),
                    );
                return failure(2, format!("error: {message}\n{USAGE}"));
            }
        };
    if plan.is_empty() {
        return RustSourceProjectCommandOutput {
            exit_code: 0,
            stdout: USAGE.as_bytes().to_vec(),
            stderr: String::new(),
        };
    }
    let result = (|| -> Result<Vec<u8>, String> {
        let values = plan
            .iter()
            .map(|value| {
                String::from_utf16(&value.0)
                    .map_err(|_| "invalid source project argument text".to_owned())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let value = read_structured_data_lino(&read_file(&values[0])?)
            .map_err(|error| error.to_string())?;
        if !value.is_object() {
            return Err("Rust source project: expected a source inventory record".to_owned());
        }
        let mut options = value.get("options").cloned().unwrap_or_else(|| json!({}));
        if options.is_null() {
            options = json!({});
        }
        if !options.is_object() {
            return Err("Rust source project: expected a resource options record".to_owned());
        }
        if values[4] == "true" {
            options
                .as_object_mut()
                .ok_or_else(|| {
                    "Rust source project: expected a resource options record".to_owned()
                })?
                .insert("library".to_owned(), json!(true));
        }
        let packet =
            create_rust_source_project_program_links(&value["inputs"], &values[1], &options)?;
        if !values[2].is_empty() {
            return emit_typed_program_links(&packet.network, packet.root, &values[2])
                .map(|emitted| emitted.code.into_bytes())
                .map_err(|error| error.to_string());
        }
        if values[3] == "true" {
            encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
                .map_err(|error| error.to_string())
        } else {
            Ok(packet.network.to_lino().into_bytes())
        }
    })();
    match result {
        Ok(stdout) => RustSourceProjectCommandOutput {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        },
        Err(message) => failure(1, format!("error: {message}\n")),
    }
}
