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
                // The module specifier is the statement's last string outside its import attributes.
                let within = |inner: &ProgramSourceMapping, outer: &ProgramSourceMapping| {
                    inner.range.start >= outer.range.start && inner.range.end <= outer.range.end
                };
                let attributes = syntax
                    .iter()
                    .filter(|inner| inner.term == "import_attribute" && within(inner, mapping))
                    .collect::<Vec<_>>();
                syntax
                    .iter()
                    .rfind(|inner| {
                        inner.term == "string"
                            && within(inner, mapping)
                            && !attributes.iter().any(|attribute| within(inner, attribute))
                    })
                    .filter(|specifier| specifier.range.end - specifier.range.start > 2)
                    .and_then(|specifier| {
                        source.get(specifier.range.start + 1..specifier.range.end - 1)
                    })
                    .map(str::to_string)
                    .into_iter()
                    .collect()
            }
            ("Rust", "use_declaration") => text
                .strip_prefix("use")
                .filter(|rest| rest.starts_with(char::is_whitespace))
                .map(str::trim_start)
                .filter(|rest| {
                    rest.starts_with(|character: char| {
                        character.is_ascii_alphabetic() || character == '_'
                    })
                })
                .map(|rest| {
                    rest.split(|character: char| {
                        !(character.is_ascii_alphanumeric() || character == '_')
                    })
                    .next()
                    .unwrap_or_default()
                    .to_string()
                })
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
        "JavaScript" => matches!(
            name,
            "node:fs/promises" | "node:assert" | "node:assert/strict"
        ),
        "Rust" => matches!(name, "std" | "core" | "alloc"),
        "Lean" => name == "Std",
        "Rocq" => name == "Stdlib.Arith",
        _ => false,
    };
    recognized_toolchain_module
        && project
            .dependencies()
            .iter()
            .any(|dependency| dependency == name)
}
