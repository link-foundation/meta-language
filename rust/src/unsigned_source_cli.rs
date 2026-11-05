//! Process/file transport over automatically translated command decisions.
use crate::unsigned_source_command_rules as rules;
use crate::{BinaryLinksOptions, LinkId, ParseConfiguration, SourceModuleRequestCommandOutput};
/// Existing shared process output shape, including exact binary stdout bytes.
pub type UnsignedSourceCommandOutput = SourceModuleRequestCommandOutput;
/// Export a complete exact-ID source graph or emit checked source semantics.
/// Shared argument planning performs no I/O for usage or invalid arguments.
pub fn run_unsigned_source_command<F>(args: &[String], read_file: F) -> UnsignedSourceCommandOutput
where
    F: Fn(&str) -> Result<String, String>,
{
    let arguments = args
        .iter()
        .map(|arg| rules::ml_string::Text::from_text(arg))
        .collect::<Vec<_>>();
    let plan = rules::plan_unsigned_source_command(&arguments)
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let usage = rules::read_unsigned_source_command_usage().to_string();
    if plan.len() == 1 && plan[0] == "help" {
        return UnsignedSourceCommandOutput {
            exit_code: 0,
            stdout: usage.into_bytes(),
            stderr: String::new(),
        };
    }
    if plan.len() != 7 {
        return UnsignedSourceCommandOutput {
            exit_code: 2,
            stdout: Vec::new(),
            stderr: format!("error: invalid exact source arguments\n{usage}"),
        };
    }
    let result = (|| -> Result<Vec<u8>, String> {
        let source = read_file(&plan[1])?;
        let first = LinkId::from_u64(plan[3].parse::<u64>().map_err(|error| error.to_string())?);
        let limits = BinaryLinksOptions::default();
        let (network, root) = if plan[0] == "concrete source" {
            let packet = crate::create_unsigned_source_document_links(
                &source,
                &plan[2],
                ParseConfiguration::default(),
                first,
                limits,
            )
            .map_err(|error| error.to_string())?;
            (packet.network, None)
        } else {
            let packet = if plan[0] == "Rust source owners" {
                if plan[5].is_empty() {
                    crate::create_unsigned_rust_source_owner_program_links(&source, first, limits)
                } else {
                    crate::create_unsigned_rust_source_owner_program_links_with_source_version(
                        &source, &plan[5], first, limits,
                    )
                }
            } else {
                crate::create_unsigned_typed_program_links(&source, &plan[2], first, limits)
            }
            .map_err(|error| error.to_string())?;
            (packet.network, Some(packet.root))
        };
        if !plan[4].is_empty() {
            let root =
                root.ok_or_else(|| "exact source emission needs its actual typed root".to_owned())?;
            return crate::emit_unsigned_typed_program_links(&network, root, &plan[4], limits)
                .map(|emitted| emitted.code.into_bytes())
                .map_err(|error| error.to_string());
        }
        if plan[6] == "binary" {
            crate::encode_unsigned_link_network_binary(&network, limits)
                .map_err(|error| error.to_string())
        } else {
            Ok(network.to_lino().into_bytes())
        }
    })();
    match result {
        Ok(stdout) => UnsignedSourceCommandOutput {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        },
        Err(error) => UnsignedSourceCommandOutput {
            exit_code: 1,
            stdout: Vec::new(),
            stderr: format!("error: {error}\n"),
        },
    }
}
