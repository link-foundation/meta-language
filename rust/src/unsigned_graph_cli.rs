//! Saved graph/file transport over the shared exact root and command policies.
use crate::unsigned_graph_command_rules as rules;
use crate::{
    BinaryLinksOptions, LinkId, LinkNetwork, SourceModuleRequestCommandOutput,
    TypedProgramLinksError, UnsignedLinksError,
};
fn number(value: usize) -> f64 {
    value.to_string().parse().unwrap_or(f64::INFINITY)
}
fn text(value: &str) -> rules::ml_string::Text {
    rules::ml_string::Text::from_text(value)
}
fn graph_error(message: impl ToString) -> UnsignedLinksError {
    crate::unsigned_link_network::fail(message)
}
/// Read canonical exact-ID Links text, retaining the complete graph.
/// # Errors
/// Rejects invalid metadata/text and configured byte/record/directory bounds.
pub fn read_unsigned_graph_snapshot_text(
    source: &str,
    options: BinaryLinksOptions,
) -> Result<LinkNetwork, UnsignedLinksError> {
    let bytes = number(source.len());
    let reason = rules::read_unsigned_graph_read_bounds_error(
        bytes,
        0.0,
        number(options.byte_limit),
        number(options.record_limit),
    )
    .to_string();
    if !reason.is_empty() {
        return Err(graph_error(reason));
    }
    let reason = rules::read_unsigned_graph_text_surface_error(&text(source)).to_string();
    if !reason.is_empty() {
        return Err(graph_error(reason));
    }
    let graph = LinkNetwork::from_lino(source).map_err(graph_error)?;
    let reason = rules::read_unsigned_graph_read_bounds_error(
        bytes,
        number(graph.len()),
        number(options.byte_limit),
        number(options.record_limit),
    )
    .to_string();
    if !reason.is_empty() {
        return Err(graph_error(reason));
    }
    crate::create_dense_unsigned_link_network(&graph, options)?;
    Ok(graph)
}
/// Read the existing complete exact-ID binary snapshot without source parsing.
/// # Errors
/// Retains every existing binary metadata, framing and resource refusal.
pub fn read_unsigned_graph_snapshot_binary(
    source: &[u8],
    options: BinaryLinksOptions,
) -> Result<LinkNetwork, UnsignedLinksError> {
    crate::decode_unsigned_link_network_binary(source, options)
}
/// Select an actual schema-1 typed root from exact stored metadata columns.
/// # Errors
/// Refuses missing/ambiguous roots, absent explicit roots and record limits.
pub fn select_unsigned_typed_program_root(
    network: &LinkNetwork,
    requested: Option<LinkId>,
    record_limit: usize,
) -> Result<LinkId, TypedProgramLinksError> {
    let reason =
        rules::read_unsigned_typed_root_bounds_error(number(network.len()), number(record_limit))
            .to_string();
    if !reason.is_empty() {
        return Err(TypedProgramLinksError { message: reason });
    }
    let links = network.links().collect::<Vec<_>>();
    let ids = links
        .iter()
        .map(|link| text(&link.id().as_u64().to_string()))
        .collect::<Vec<_>>();
    let types = links
        .iter()
        .map(|link| {
            text(
                &link
                    .metadata()
                    .link_type()
                    .map(|kind| format!("{kind:?}"))
                    .unwrap_or_default(),
            )
        })
        .collect::<Vec<_>>();
    let languages = links
        .iter()
        .map(|link| text(link.metadata().language().unwrap_or("")))
        .collect::<Vec<_>>();
    let terms = links
        .iter()
        .map(|link| text(link.metadata().term().unwrap_or("")))
        .collect::<Vec<_>>();
    let definitions = links
        .iter()
        .map(|link| text(link.metadata().definition().unwrap_or("")))
        .collect::<Vec<_>>();
    let requested = requested
        .map(|id| id.as_u64().to_string())
        .unwrap_or_default();
    let plan = rules::plan_unsigned_typed_graph_root(
        &ids,
        &types,
        &languages,
        &terms,
        &definitions,
        &text(&requested),
        number(record_limit),
    )
    .iter()
    .map(ToString::to_string)
    .collect::<Vec<_>>();
    if plan.first().map(String::as_str) != Some("root") {
        return Err(TypedProgramLinksError {
            message: plan
                .get(1)
                .cloned()
                .unwrap_or_else(|| "an exact typed root plan needs its complete result".to_owned()),
        });
    }
    let value = plan[1]
        .parse::<u64>()
        .map_err(|error| TypedProgramLinksError {
            message: error.to_string(),
        })?;
    Ok(LinkId::from_u64(value))
}
/// Existing shared process output shape, with exact binary stdout.
pub type UnsignedGraphCommandOutput = SourceModuleRequestCommandOutput;
/// Read current saved Links and optionally rebase or emit checked semantics.
/// Help and invalid shared argument plans do not access the selected file.
pub fn run_unsigned_graph_command<F, B>(
    args: &[String],
    read_file: F,
    read_bytes: B,
) -> UnsignedGraphCommandOutput
where
    F: Fn(&str) -> Result<String, String>,
    B: Fn(&str) -> Result<Vec<u8>, String>,
{
    let args = args.iter().map(|arg| text(arg)).collect::<Vec<_>>();
    let plan = rules::plan_unsigned_graph_command(&args)
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let usage = rules::read_unsigned_graph_command_usage().to_string();
    if plan.len() == 1 && plan[0] == "help" {
        return UnsignedGraphCommandOutput {
            exit_code: 0,
            stdout: usage.into_bytes(),
            stderr: String::new(),
        };
    }
    if plan.len() != 6 {
        return UnsignedGraphCommandOutput {
            exit_code: 2,
            stdout: Vec::new(),
            stderr: format!("error: invalid exact graph arguments\n{usage}"),
        };
    }
    let result = (|| -> Result<Vec<u8>, String> {
        let options = BinaryLinksOptions::default();
        let mut network = if plan[1] == "binary" {
            read_unsigned_graph_snapshot_binary(&read_bytes(&plan[0])?, options)
        } else {
            read_unsigned_graph_snapshot_text(&read_file(&plan[0])?, options)
        }
        .map_err(|error| error.to_string())?;
        let requested = if plan[4].is_empty() {
            None
        } else {
            Some(LinkId::from_u64(
                plan[4].parse::<u64>().map_err(|error| error.to_string())?,
            ))
        };
        let mut root = if plan[3].is_empty() {
            None
        } else {
            Some(
                select_unsigned_typed_program_root(&network, requested, options.record_limit)
                    .map_err(|error| error.to_string())?,
            )
        };
        if !plan[5].is_empty() {
            let first =
                LinkId::from_u64(plan[5].parse::<u64>().map_err(|error| error.to_string())?);
            let rebased = crate::rebase_unsigned_link_network(&network, first, options)
                .map_err(|error| error.to_string())?;
            if let Some(actual) = root {
                let index = rebased
                    .source_identities
                    .binary_search(&actual)
                    .map_err(|_| {
                        "a saved typed graph lost its actual root during coordinate allocation"
                            .to_owned()
                    })?;
                root = Some(rebased.identities[index]);
            }
            network = rebased.network;
        }
        if !plan[3].is_empty() {
            return crate::emit_unsigned_typed_program_links(
                &network,
                root.ok_or_else(|| "a saved typed graph needs its selected root".to_owned())?,
                &plan[3],
                options,
            )
            .map(|emitted| emitted.code.into_bytes())
            .map_err(|error| error.to_string());
        }
        if plan[2] == "binary" {
            crate::encode_unsigned_link_network_binary(&network, options)
                .map_err(|error| error.to_string())
        } else {
            Ok(network.to_lino().into_bytes())
        }
    })();
    match result {
        Ok(stdout) => UnsignedGraphCommandOutput {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        },
        Err(error) => UnsignedGraphCommandOutput {
            exit_code: 1,
            stdout: Vec::new(),
            stderr: format!("error: {error}\n"),
        },
    }
}
