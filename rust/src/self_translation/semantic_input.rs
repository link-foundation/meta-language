//! Native item views over the automatically translated shared grouping plan.
use super::{Group, GroupKind, Item, line_break, source_lines};
use crate::translation::semantic_input_rules::{
    plan_semantic_source_groups, read_rust_public_signature_name_position,
    read_semantic_source_role, render_rust_semantic_module_export,
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
    let names = read_rust_public_bindings(source)?;
    read_rust_module_value_signatures(&program, &names)
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

fn read_rust_public_bindings(
    source: &str,
) -> Result<std::collections::BTreeMap<String, String>, super::SelfTranslationError> {
    use crate::translation::{
        Language,
        lexer::{TokenKind, tokenize},
    };
    let network = crate::LinkNetwork::parse(source, "Rust", crate::ParseConfiguration::default());
    if !network.verify_full_match(None).is_clean() {
        return Err(super::SelfTranslationError { message: "independent Rust signature concrete syntax: the complete source needs a clean concrete grammar interpretation".to_owned() });
    }
    let mut names = std::collections::BTreeMap::new();
    for item in super::top_level_items(source, "Rust") {
        let kind = crate::translation::rust_module_value_rules::read_rust_value_module_owner_kind(
            &item.term,
        );
        if kind.is_empty() {
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
        let position = if kind == "const" {
            crate::translation::rust_module_value_rules::read_rust_public_constant_name_position(
                &kinds, &values,
            )
        } else {
            read_rust_public_signature_name_position(&kinds, &values)
        };
        if position >= 0.0 {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let position = position as usize;
            names.insert(values[position].clone(), kind);
        }
    }
    Ok(names)
}

fn read_rust_module_value_signatures(
    program: &crate::translation::ir::Program,
    bindings: &std::collections::BTreeMap<String, String>,
) -> Result<Vec<super::SelfTranslationSignature>, super::SelfTranslationError> {
    use crate::translation::{
        ir::{Decl, Node},
        surface::{External, ExternalParam},
    };
    let mut signatures = Vec::new();
    for entry in &program.declarations {
        let Decl::Fn(function) = entry else {
            continue;
        };
        let Some(kind) = bindings.get(&function.name) else {
            continue;
        };
        if !function.module_path.is_empty() {
            continue;
        }
        signatures.push(if kind == "const" {
            External::Constant {
                name: function.name.clone(),
                ty: function.ret.clone(),
                literal: matches!(function.body.node, Node::Lit { .. }),
            }
        } else {
            External::Function {
                name: function.name.clone(),
                params: function
                    .params
                    .iter()
                    .map(|param| ExternalParam {
                        name: param.name.clone(),
                        ty: param.ty.clone(),
                    })
                    .collect(),
                ret: function.ret.clone(),
            }
        });
    }
    if signatures.len() != bindings.len() {
        return Err(super::SelfTranslationError {message:"independent Rust module interface: every public source value needs its actual checked signature".to_owned()});
    }
    Ok(signatures)
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
            !crate::translation::rust_module_value_rules::accept_rust_value_module_item(
                &item.term,
                item.comment,
                read_semantic_source_role(item.comment, item.text),
            )
        })
    {
        return Ok(None);
    }
    let names = read_rust_public_bindings(source)?;
    let program = read_independent_rust_program(source)?;
    let tree = crate::native_grammar_parser::parse_native("native-rust", source);
    let mut owners = std::collections::HashSet::new();
    for (node, _) in tree.children.iter().filter(|(node, _)| {
        !crate::translation::rust_module_value_rules::read_rust_value_module_owner_kind(&node.term)
            .is_empty()
    }) {
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
        let lexed = crate::translation::lexer::tokenize(name, crate::translation::Language::Rust)
            .map_err(failure)?;
        if lexed.tokens.len() != 2
            || lexed.tokens[0].kind != crate::translation::lexer::TokenKind::Identifier
        {
            return Ok(None);
        }
        if !owners.insert(lexed.tokens[0].value.clone()) {
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
    let reserved = crate::translation::lexer::tokenize(
        &emitted.text,
        crate::translation::Language::JavaScript,
    )
    .map_err(failure)?
    .tokens
    .into_iter()
    .filter(|token| token.kind == crate::translation::lexer::TokenKind::Identifier)
    .map(|token| token.value)
    .collect::<Vec<_>>();
    let kinds = program
        .declarations
        .iter()
        .filter_map(|entry| match entry {
            Decl::Fn(function) => names.get(&function.name).cloned(),
            _ => None,
        })
        .collect::<Vec<_>>();
    let bindings =
        crate::translation::rust_module_value_rules::plan_rust_module_value_export_bindings(
            &kinds, &reserved,
        );
    let mut export_position = 0usize;
    // Emit aliases in actual source declaration order, independently of native
    // hash iteration order. Private declarations remain private target helpers.
    for entry in &program.declarations {
        let Decl::Fn(function) = entry else {
            continue;
        };
        let Some(kind) = names.get(&function.name) else {
            continue;
        };
        let mappings: Vec<_> = emitted
            .mappings
            .iter()
            .filter(|mapping| mapping.kind == "function" && mapping.source == function.full_name)
            .collect();
        if mappings.len() != 1 {
            return Err(super::SelfTranslationError { message:
            "independent Rust module visibility: every public source function needs one actual target mapping".to_owned() });
        }
        let code = if kind == "const" {
            let binding = &bindings[export_position];
            crate::translation::rust_module_value_rules::render_rust_module_value_export(
                &function.name,
                &mappings[0].target,
                binding,
            )
        } else {
            render_rust_semantic_module_export(&function.name, &mappings[0].target)
        };
        emitted.definitions.push(code);
        export_position += 1;
    }
    let signatures = read_rust_module_value_signatures(&program, &names)?;
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
