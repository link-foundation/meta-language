//! Native item views over the automatically translated shared grouping plan.
use super::{Group, GroupKind, Item, line_break, source_lines};
use crate::translation::semantic_input_rules::{
    accept_rust_semantic_module_item, plan_semantic_source_groups,
    read_rust_public_signature_name_position, read_semantic_source_role,
    render_rust_semantic_module_export,
};

pub(super) fn read_independent_rust_module_signatures(
    source: &str,
    options: &super::SelfTranslationOptions,
) -> Result<Vec<super::SelfTranslationSignature>, super::SelfTranslationError> {
    if !options.imports.is_empty() {
        return Err(super::SelfTranslationError { message: "independent Rust signature imports: a source module with external signature assumptions requires a composed Rust namespace interpretation".to_owned() });
    }
    let program = read_independent_rust_program(source)?;
    crate::translation::emit_javascript::emit_javascript(&program).map_err(failure)?;
    let names = read_rust_public_names(source)?;
    Ok(super::binding::declared_signatures(&program, false)
        .into_iter()
        .filter(|signature| names.contains(signature.name()))
        .collect())
}

fn failure(
    error: crate::translation::diagnostics::TranslationError,
) -> super::SelfTranslationError {
    super::SelfTranslationError {
        message: error.to_string(),
    }
}

fn read_independent_rust_program(
    source: &str,
) -> Result<crate::translation::ir::Program, super::SelfTranslationError> {
    use crate::translation::{check::check_program, rust::parse_rust};
    let surface = match parse_rust(source) {
        Ok(surface) => surface,
        Err(error)
            if error.kind == crate::translation::diagnostics::ErrorKind::Syntax
                && error.reason == "a Rust program needs fn main" =>
        {
            parse_rust(&format!("{source}\nfn main() {{}}\n")).map_err(failure)?
        }
        Err(error) => return Err(failure(error)),
    };
    check_program(&surface).map_err(failure)
}

fn read_rust_public_names(
    source: &str,
) -> Result<std::collections::HashSet<String>, super::SelfTranslationError> {
    use crate::translation::{
        Language,
        lexer::{TokenKind, tokenize},
    };
    let network = crate::LinkNetwork::parse(source, "Rust", crate::ParseConfiguration::default());
    if !network.verify_full_match(None).is_clean() {
        return Err(super::SelfTranslationError { message: "independent Rust signature concrete syntax: the complete source needs a clean concrete grammar interpretation".to_owned() });
    }
    let mut names = std::collections::HashSet::new();
    for item in super::top_level_items(source, "Rust") {
        if item.term != "function_item" {
            continue;
        }
        let lexed = tokenize(item.text, Language::Rust).map_err(failure)?;
        let kinds: Vec<String> = lexed
            .tokens
            .iter()
            .map(|token| {
                match token.kind {
                    TokenKind::Identifier => "identifier",
                    TokenKind::Punct => "punct",
                    _ => "other",
                }
                .to_owned()
            })
            .collect();
        let values: Vec<String> = lexed
            .tokens
            .iter()
            .map(|token| token.value.clone())
            .collect();
        let position = read_rust_public_signature_name_position(&kinds, &values);
        if position >= 0.0 {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let position = position as usize;
            names.insert(values[position].clone());
        }
    }
    Ok(names)
}

pub(super) fn read_independent_rust_module_publication(
    source: &str,
    items: &[Item<'_>],
) -> Result<
    Option<(
        crate::translation::emit_common::Emitted,
        Vec<super::SelfTranslationSignature>,
    )>,
    super::SelfTranslationError,
> {
    use crate::translation::{emit_javascript::emit_javascript, ir::Decl};
    if items.is_empty()
        || items.iter().any(|item| {
            !accept_rust_semantic_module_item(
                &item.term,
                item.comment,
                read_semantic_source_role(item.comment, item.text),
            )
        })
    {
        return Ok(None);
    }
    let names = read_rust_public_names(source)?;
    let program = read_independent_rust_program(source)?;
    let tree = crate::native_grammar_parser::parse_native("native-rust", source);
    let mut owners = std::collections::HashSet::new();
    for (node, _) in tree
        .children
        .iter()
        .filter(|(node, _)| node.term == "function_item")
    {
        let declarations: Vec<_> = node
            .children
            .iter()
            .filter(|(_, field)| field.as_deref() == Some("name"))
            .collect();
        if declarations.len() != 1 {
            return Ok(None);
        }
        let name = &declarations[0].0;
        let Some(name) = source.get(name.start..name.end) else {
            return Ok(None);
        };
        if !owners.insert(name.to_owned()) {
            return Ok(None);
        }
    }
    if program
        .main
        .as_ref()
        .is_some_and(|main| !main.effects.is_empty())
        || owners.len() != program.declarations.len() + usize::from(owners.contains("main"))
        || program
            .declarations
            .iter()
            .any(|entry| !matches!(entry, Decl::Fn(function) if function.module_path.is_empty() && owners.contains(&function.name)))
    {
        return Ok(None);
    }
    let mut emitted = emit_javascript(&program).map_err(failure)?;
    // Emit aliases in actual source declaration order, independently of native
    // hash iteration order. Private declarations remain private target helpers.
    for entry in &program.declarations {
        let Decl::Fn(function) = entry else {
            continue;
        };
        if !names.contains(&function.name) {
            continue;
        }
        let mappings: Vec<_> = emitted
            .mappings
            .iter()
            .filter(|mapping| mapping.kind == "function" && mapping.source == function.full_name)
            .collect();
        if mappings.len() != 1 {
            return Err(super::SelfTranslationError { message:
            "independent Rust module visibility: every public source function needs one actual target mapping".to_owned() });
        }
        emitted.definitions.push(render_rust_semantic_module_export(
            &function.name,
            &mappings[0].target,
        ));
    }
    let signatures = super::binding::declared_signatures(&program, false)
        .into_iter()
        .filter(|signature| names.contains(signature.name()))
        .collect();
    Ok(Some((emitted, signatures)))
}

pub(super) fn group_semantic_items<'a>(
    items: &[Item<'a>],
    text: &'a str,
    source_language: &str,
) -> Vec<Group<'a>> {
    let roles: Vec<f64> = items
        .iter()
        .map(|item| read_semantic_source_role(item.comment, item.text))
        .collect();
    let line_breaks: Vec<bool> = items.iter().map(|item| line_break(item.after)).collect();
    let plan = plan_semantic_source_groups(&roles, &line_breaks);
    plan.chunks_exact(3)
        .map(|record| {
            // These positions are produced by the complete shared source algorithm,
            // bounded by the actual native item slice supplied above.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let first = record[0] as usize;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let last = record[1] as usize;
            let selected = &items[first..=last];
            let kind = if record[2] == 0.0 {
                GroupKind::Provenance
            } else if record[2] == 1.0 {
                GroupKind::Carried {
                    language: source_language.to_owned(),
                    lines: source_lines(&items[first + 1..=last]),
                }
            } else if record[2] == 2.0 {
                GroupKind::Comment {
                    text: text[items[first].start..items[last].end].to_owned(),
                    term: items[last].term.clone(),
                }
            } else {
                GroupKind::Item {
                    text: text[items[first].start..items[last].end].to_owned(),
                    term: items[last].term.clone(),
                }
            };
            Group {
                kind,
                items: selected.to_vec(),
                after: items[last].after,
            }
        })
        .collect()
}
