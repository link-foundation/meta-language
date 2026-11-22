//! Recompose the actual ordinary source modules retained by a library asset.

use std::collections::{HashMap, HashSet};

use super::javascript::test_javascript_source_function_start;
use super::javascript_line_terminator_rules::accept_java_script_line_terminator;
use super::lexer::{Source, Token, TokenKind, tokenize};
use super::source_library_binding_rules::{
    read_source_library_declaration_binding_kind, read_source_library_function_declaration_offset,
    test_java_script_source_function_expression_term,
};
use super::{Language, Span};
use crate::program_representation::{ProgramRepresentation, analyze_program, module_bindings};

#[derive(Default)]
struct ModuleFacts {
    imports: Vec<Import>,
    declarations: Vec<Declaration>,
    exports: HashSet<String>,
    identifiers: HashSet<String>,
}

struct Declaration {
    name: String,
    kind: String,
    expression_name: Option<String>,
}

struct Import {
    span: Span,
    specifier: String,
    bindings: Vec<(String, String)>,
}

pub(super) struct Module<'a> {
    pub path: &'a str,
    pub range: Span,
    pub declarations: HashMap<String, String>,
    pub imports: HashSet<String>,
}

pub(super) struct Closure<'a> {
    pub source: String,
    pub modules: Vec<Module<'a>>,
}

fn word(token: Option<&Token>, name: &str) -> bool {
    token.is_some_and(|token| token.kind == TokenKind::Identifier && token.raw == name)
}

fn punct(token: Option<&Token>, mark: &str) -> bool {
    token.is_some_and(|token| token.kind == TokenKind::Punct && token.value == mark)
}

fn identifier(token: Option<&Token>) -> Result<&Token, String> {
    token
        .filter(|token| token.kind == TokenKind::Identifier)
        .ok_or_else(|| "a source module binding needs its actual identifier".to_owned())
}

fn declaration(
    source: &Source,
    tokens: &[Token],
    at: usize,
    expression_heads: &mut HashSet<usize>,
) -> Option<Declaration> {
    let first = tokens.get(at)?;
    let second = tokens.get(at + 1)?;
    let actual_word = |token: &Token| {
        if token.kind == TokenKind::Identifier && token.value == token.raw {
            token.value.clone()
        } else {
            String::new()
        }
    };
    let line_break = source.units()[first.end..second.start]
        .iter()
        .any(|unit| accept_java_script_line_terminator(f64::from(*unit)));
    let offset = read_source_library_function_declaration_offset(
        &actual_word(first),
        &actual_word(second),
        line_break,
    );
    let head = at + usize::from(offset == 1.0);
    let keyword = actual_word(&tokens[head]);
    let name = identifier(tokens.get(head + 1)).ok()?;
    let callable = keyword == "const"
        && punct(tokens.get(at + 2), "=")
        && test_javascript_source_function_start(tokens, at + 3);
    let mut expression_name = None;
    if callable {
        let mut initializer = at + 3;
        if word(tokens.get(initializer), "async") && !punct(tokens.get(initializer + 1), "=>") {
            initializer += 1;
        }
        if word(tokens.get(initializer), "function") {
            expression_heads.insert(initializer);
            expression_name = identifier(tokens.get(initializer + 1))
                .ok()
                .map(|token| token.value.clone());
        }
    }
    let kind =
        read_source_library_declaration_binding_kind(&keyword, "identifier", &name.value, callable);
    (!kind.is_empty()).then(|| Declaration {
        name: name.value.clone(),
        kind,
        expression_name,
    })
}

fn read_import(tokens: &[Token], start: usize) -> Result<(Import, usize), String> {
    let mut at = start + 1;
    if !punct(tokens.get(at), "{") {
        return Err("source library imports need actual named bindings".to_owned());
    }
    at += 1;
    let mut bindings = Vec::new();
    while let Ok(imported) = identifier(tokens.get(at)) {
        at += 1;
        let local = if word(tokens.get(at), "as") {
            at += 2;
            identifier(tokens.get(at - 1))?
        } else {
            imported
        };
        bindings.push((imported.value.clone(), local.value.clone()));
        if !punct(tokens.get(at), ",") {
            break;
        }
        at += 1;
    }
    let specifier = tokens
        .get(at + 2)
        .filter(|token| token.kind == TokenKind::String);
    let Some(specifier) = specifier.filter(|_| {
        !bindings.is_empty() && punct(tokens.get(at), "}") && word(tokens.get(at + 1), "from")
    }) else {
        return Err("source library import needs a complete named dependency".to_owned());
    };
    at += 3;
    let end = if punct(tokens.get(at), ";") {
        at += 1;
        tokens[at - 1].end
    } else {
        specifier.end
    };
    Ok((
        Import {
            span: Span::new(tokens[start].start, end),
            specifier: specifier.value.clone(),
            bindings,
        },
        at,
    ))
}

fn read_module(source: &str) -> Result<ModuleFacts, String> {
    let tokens = tokenize(source, Language::JavaScript)
        .map_err(|error| error.to_string())?
        .tokens;
    let source = Source::new(source);
    let mut facts = ModuleFacts::default();
    let mut depth = 0_usize;
    let mut at = 0;
    let mut declared = HashSet::new();
    let mut imported = HashSet::new();
    let mut expression_heads = HashSet::new();
    while let Some(token) = tokens.get(at) {
        if token.kind == TokenKind::Punct {
            match token.value.as_str() {
                "{" | "[" | "(" => depth += 1,
                "}" | "]" | ")" => {
                    depth = depth
                        .checked_sub(1)
                        .ok_or("unbalanced module punctuation")?
                }
                _ => {}
            }
            at += 1;
            continue;
        }
        if depth == 0 {
            if word(Some(token), "import") {
                let (dependency, next) = read_import(&tokens, at)?;
                for (_, name) in &dependency.bindings {
                    if !imported.insert(name.clone()) {
                        return Err(format!("duplicate actual source import {name}"));
                    }
                }
                facts.imports.push(dependency);
                at = next;
                continue;
            }
            if word(Some(token), "export") {
                let binding = declaration(&source, &tokens, at + 1, &mut expression_heads)
                    .ok_or("a source library export needs an actual named function declaration or constant binding")?;
                facts.exports.insert(binding.name);
            }
            if !expression_heads.contains(&at)
                && (word(Some(token), "function") || word(Some(token), "const"))
                && let Some(binding) = declaration(&source, &tokens, at, &mut expression_heads)
            {
                if !declared.insert(binding.name.clone()) {
                    return Err(format!(
                        "duplicate actual source declaration {}",
                        binding.name
                    ));
                }
                facts.declarations.push(binding);
            }
        }
        at += 1;
    }
    if depth != 0 || imported.iter().any(|name| declared.contains(name)) {
        return Err(
            "source library needs balanced complete modules and distinct local bindings".to_owned(),
        );
    }
    facts.identifiers = tokens
        .into_iter()
        .filter(|token| token.kind == TokenKind::Identifier)
        .map(|token| token.value)
        .collect();
    Ok(facts)
}

fn dependency(path: &str, specifier: &str) -> Result<String, String> {
    if !(specifier.starts_with("./") || specifier.starts_with("../"))
        || !specifier.ends_with(".js")
        || specifier.contains(['?', '#', '\\', '\0'])
    {
        return Err("source library imports need explicit relative JavaScript files".to_owned());
    }
    let mut parts: Vec<_> = path.split('/').collect();
    parts.pop();
    for part in specifier.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts
                    .pop()
                    .ok_or("source dependency escapes its supplied directory")?;
            }
            _ => parts.push(part),
        }
    }
    Ok(parts.join("/"))
}

fn visit<'a>(
    path: &'a str,
    modules: &HashMap<&'a str, ModuleFacts>,
    seen: &mut HashSet<&'a str>,
    order: &mut Vec<&'a str>,
) -> Result<(), String> {
    if !seen.insert(path) {
        return Ok(());
    }
    for import in &modules[path].imports {
        let requested = dependency(path, &import.specifier)?;
        let (provider, facts) = modules
            .get_key_value(requested.as_str())
            .ok_or_else(|| format!("missing actual source dependency {requested}"))?;
        for (name, _) in &import.bindings {
            if !facts.exports.contains(name) {
                return Err(format!(
                    "{name} needs an actual exported owner in {provider}"
                ));
            }
        }
        visit(provider, modules, seen, order)?;
    }
    order.push(path);
    Ok(())
}

fn model(source: &str) -> Result<ProgramRepresentation, String> {
    analyze_program(source, "JavaScript", Default::default()).map_err(|error| error.to_string())
}

fn rename_declaration(
    source: &str,
    name: &str,
    kind: &str,
    replacement: &str,
) -> Result<String, String> {
    let program = model(source)?;
    let binding = program
        .bindings()
        .iter()
        .find(|binding| {
            binding.scope() == "scope:0" && binding.kind() == kind && binding.name() == name
        })
        .ok_or_else(|| format!("missing actual resolved binding {name}"))?;
    program
        .rename_binding(binding.id(), replacement)
        .map(|program| program.source().to_owned())
        .map_err(|error| error.to_string())
}

fn normalize_function_expression(
    source: &str,
    outer_name: &str,
    inner_name: &str,
) -> Result<String, String> {
    let program = model(source)?;
    let outer = program.bindings().iter().find(|binding| {
        binding.scope() == "scope:0" && binding.kind() == "const" && binding.name() == outer_name
    });
    let expression = outer.and_then(|outer| {
        let outer = outer.declaration();
        let declarator = program.source_mappings().iter().find(|node| {
            node.term() == "variable_declarator" && node.range().start() == outer.start()
        })?;
        program
            .source_mappings()
            .iter()
            .filter(|node| {
                test_java_script_source_function_expression_term(node.term())
                    && node.range().start() >= outer.end()
                    && node.range().end() <= declarator.range().end()
            })
            .min_by_key(|node| (node.range().start(), std::cmp::Reverse(node.range().end())))
    });
    let inner = expression.and_then(|expression| {
        program.bindings().iter().find(|binding| {
            binding.kind() == "function"
                && binding.name() == inner_name
                && program.scopes().iter().any(|scope| {
                    scope.id() == binding.scope() && scope.range() == expression.range()
                })
        })
    });
    let inner =
        inner.ok_or("source function expression needs its actual private lexical binding")?;
    if inner_name == outer_name {
        return Ok(source.to_owned());
    }
    program
        .rename_binding(inner.id(), outer_name)
        .map(|program| program.source().to_owned())
        .map_err(|error| error.to_string())
}

fn substitute_imports(source: &mut String, imports: &[(&str, &str)]) -> Result<(), String> {
    if imports.is_empty() {
        return Ok(());
    }
    let program = model(source)?;
    let roles =
        module_bindings::read_fields(program.network(), program.source_mappings(), "JavaScript")
            .map_err(|error| error.to_string())?;
    let mut edits = Vec::new();
    for (original, replacement) in imports {
        let binding = program
            .bindings()
            .iter()
            .find(|binding| {
                binding.scope() == "scope:0"
                    && binding.kind() == "import"
                    && binding.name() == *original
            })
            .ok_or_else(|| format!("missing actual resolved source import {original}"))?;
        if program
            .bindings()
            .iter()
            .any(|other| other.id() != binding.id() && other.name() == *replacement)
            || program
                .unresolved_references()
                .iter()
                .any(|reference| reference.name() == *replacement)
        {
            return Err(format!(
                "source alias substitution would capture {replacement}"
            ));
        }
        for range in binding.references() {
            let shorthand = program.source_mappings().iter().any(|node| {
                node.range() == *range
                    && matches!(
                        node.term(),
                        "shorthand_property_identifier" | "shorthand_property_identifier_pattern"
                    )
            });
            let affixes = module_bindings::affixes(
                module_bindings::role(&roles, *range),
                shorthand,
                original,
            )
            .map_err(|error| error.to_string())?;
            edits.push((*range, format!("{}{replacement}{}", affixes[0], affixes[1])));
        }
    }
    edits.sort_unstable_by_key(|(range, _)| std::cmp::Reverse(range.start()));
    let mut previous = source.len();
    for (range, value) in edits {
        if range.end() > previous {
            return Err("alias substitutions need disjoint actual reference ranges".to_owned());
        }
        source.replace_range(range.start()..range.end(), &value);
        previous = range.start();
    }
    Ok(())
}

pub(super) fn compose<'a>(
    entry: &'a str,
    inputs: &[(&'a str, &'a str)],
) -> Result<Closure<'a>, String> {
    if inputs.is_empty() || inputs.len() > 128 {
        return Err("a source library needs at most 128 supplied modules".to_owned());
    }
    let mut modules = HashMap::new();
    let mut sources = HashMap::new();
    let mut input_bytes = 0;
    for &(path, source) in inputs {
        input_bytes += source.len();
        if input_bytes > 16 * 1024 * 1024
            || path.contains(['\\', '\0'])
            || path.split('/').any(|part| matches!(part, "" | "." | ".."))
            || sources.insert(path, source).is_some()
        {
            return Err(
                "source library needs bounded unique normalized module paths and texts".to_owned(),
            );
        }
        modules.insert(path, read_module(source)?);
    }
    let mut order = Vec::new();
    if !modules.contains_key(entry) {
        return Err("source library needs the actual entry module".to_owned());
    }
    visit(entry, &modules, &mut HashSet::new(), &mut order)?;
    if order.len() != modules.len() {
        return Err("exactly every reachable actual source module must be supplied".to_owned());
    }
    let mut counts = HashMap::new();
    let mut names = HashSet::new();
    let mut aliased = HashSet::new();
    for &path in &order {
        for Declaration { name, .. } in &modules[path].declarations {
            *counts.entry(name.as_str()).or_insert(0) += 1;
        }
        names.extend(modules[path].identifiers.iter().cloned());
        for import in &modules[path].imports {
            let provider = dependency(path, &import.specifier)?;
            for (original, local) in &import.bindings {
                if original != local {
                    aliased.insert((provider.clone(), original.clone()));
                }
            }
        }
    }
    let mut ordinal = 0;
    let mut selected = HashMap::new();
    for &path in &order {
        for Declaration { name, .. } in &modules[path].declarations {
            let identity = (path.to_owned(), name.clone());
            let replacement =
                if path != entry && (counts[name.as_str()] > 1 || aliased.contains(&identity)) {
                    loop {
                        ordinal += 1;
                        let candidate = format!("dependencyFunction{ordinal}");
                        if names.insert(candidate.clone()) {
                            break candidate;
                        }
                    }
                } else {
                    name.clone()
                };
            selected.insert(identity, replacement);
        }
    }
    let mut source = String::new();
    let mut offset = 0;
    let mut output = Vec::new();
    let mut rewrites = 0;
    for path in order {
        let mut body = sources[path].to_owned();
        let mut declarations = HashMap::new();
        let mut imports = HashSet::new();
        let mut substitutions = Vec::new();
        for Declaration { name, kind, .. } in &modules[path].declarations {
            let replacement = &selected[&(path.to_owned(), name.clone())];
            if replacement != name {
                rewrites += 1;
                if rewrites > 1024 {
                    return Err(
                        "source alias composition exceeds 1024 actual binding rewrites".to_owned(),
                    );
                }
                body = rename_declaration(&body, name, kind, replacement)?;
            }
            declarations.insert(replacement.clone(), name.clone());
        }
        for Declaration {
            name,
            expression_name,
            ..
        } in &modules[path].declarations
        {
            if let Some(inner_name) = expression_name {
                let replacement = &selected[&(path.to_owned(), name.clone())];
                body = normalize_function_expression(&body, replacement, inner_name)?;
            }
        }
        for import in &modules[path].imports {
            let provider = dependency(path, &import.specifier)?;
            for (original, local) in &import.bindings {
                let replacement = selected
                    .get(&(provider.clone(), original.clone()))
                    .ok_or_else(|| format!("missing actual provider declaration {original}"))?;
                imports.insert(replacement.clone());
                if replacement != local {
                    substitutions.push((local.as_str(), replacement.as_str()));
                }
            }
        }
        rewrites += substitutions.len();
        if rewrites > 1024 {
            return Err("source alias composition exceeds 1024 actual binding rewrites".to_owned());
        }
        substitute_imports(&mut body, &substitutions)?;
        let transformed = read_module(&body)?;
        let units = Source::new(&body);
        let mut start = 0;
        let mut compiled = String::new();
        for import in transformed.imports {
            compiled.push_str(&units.slice(start, import.span.start));
            start = import.span.end;
        }
        compiled.push_str(&units.slice(start, units.units().len()));
        let end = offset + compiled.encode_utf16().count();
        if !output.is_empty() {
            source.push_str("\n\n");
        }
        source.push_str(&compiled);
        output.push(Module {
            path,
            range: Span::new(offset, end),
            declarations,
            imports,
        });
        offset = end + 2;
    }
    Ok(Closure {
        source,
        modules: output,
    })
}
