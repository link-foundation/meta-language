//! Module requests extracted from grammar nodes and compared with project declarations.

use super::{unique_facts, ProgramFact, ProgramProjectContext, ProgramSourceMapping};

pub(super) fn module_requests(
    syntax: &[ProgramSourceMapping],
    source: &str,
    language: &str,
) -> Vec<ProgramFact> {
    let mut requests = Vec::new();
    for mapping in syntax {
        let Some(text) = source.get(mapping.range.start..mapping.range.end) else {
            continue;
        };
        let names: Vec<String> = match (language, mapping.term.as_str()) {
            ("JavaScript", "import_statement") => {
                let mut quoted = Vec::new();
                let mut opening = None;
                for (index, character) in text.char_indices() {
                    match opening {
                        Some((quote, start)) if character == quote => {
                            quoted.push(text[start..index].to_string());
                            opening = None;
                        }
                        None if character == '\'' || character == '"' => {
                            opening = Some((character, index + character.len_utf8()));
                        }
                        _ => {}
                    }
                }
                quoted.into_iter().last().into_iter().collect()
            }
            ("Rust", "use_declaration") => text
                .strip_prefix("use ")
                .and_then(|value| value.split("::").next())
                .map(str::to_string)
                .into_iter()
                .collect(),
            ("Lean", "import") => text.strip_prefix("import ").map_or_else(Vec::new, |value| {
                value.split_whitespace().map(str::to_string).collect()
            }),
            ("Rocq", "require_command") => {
                let (prefix, required) = if let Some(from) = text.strip_prefix("From ") {
                    if let Some((prefix, required)) = from.split_once(" Require ") {
                        (Some(prefix), required)
                    } else {
                        continue;
                    }
                } else if let Some(required) = text.strip_prefix("Require ") {
                    (None, required)
                } else {
                    continue;
                };
                let required = required
                    .strip_prefix("Import ")
                    .or_else(|| required.strip_prefix("Export "))
                    .unwrap_or(required);
                required
                    .split_whitespace()
                    .map(|name| {
                        prefix.map_or_else(|| name.to_string(), |prefix| format!("{prefix}.{name}"))
                    })
                    .collect()
            }
            _ => Vec::new(),
        };
        for name in names {
            if !name.is_empty() {
                requests.push(ProgramFact::new("module-import", name, mapping.range));
            }
        }
    }
    unique_facts(requests)
}

pub(super) fn project_has_module(
    project: &ProgramProjectContext,
    name: &str,
    language: &str,
) -> bool {
    let recognized_toolchain_module = match language {
        "JavaScript" => name == "node:fs/promises",
        "Rust" => matches!(name, "std" | "core" | "alloc"),
        "Lean" => name == "Std",
        "Rocq" => name == "Stdlib.Arith",
        _ => false,
    };
    recognized_toolchain_module
        && project
            .dependencies
            .iter()
            .any(|dependency| dependency == name)
}
