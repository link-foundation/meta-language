use std::collections::BTreeSet;

use crate::grammar::{Grammar, GrammarExpr, GrammarRule};
use crate::rust_codec::{RustFieldShape, RustTypeKind, RustTypeShape};

use super::pest::emit_pest;
use super::{EmitReport, GrammarEmitError};

/// Bundled Rust parser codegen output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustParserArtifacts {
    /// `.pest` grammar source accepted by `pest_meta` and `pest_derive`.
    pub pest_grammar: String,
    /// `pest_derive` parser struct stub using `#[grammar_inline = "..."]`.
    pub parser_struct: String,
    /// Rendered Rust AST type declarations, one declaration per grammar rule.
    pub ast_types: String,
    /// Queryable Rust type shapes used to render `ast_types`.
    pub ast_shapes: Vec<RustTypeShape>,
}

/// Emits a runnable Rust parser bundle from the grammar IR.
///
/// The generated parser uses `pest_derive` with an inline pest grammar. AST
/// declarations are derived from [`RustTypeShape`] so the generated type model
/// can also be registered in the links codec.
///
/// # Errors
///
/// Returns [`GrammarEmitError`] when the underlying pest grammar emitter cannot
/// represent an invalid IR construct, such as empty choices or invalid repeat
/// bounds.
pub fn emit_rust_parser(
    grammar: &Grammar,
) -> Result<(RustParserArtifacts, EmitReport), GrammarEmitError> {
    let (pest_grammar, report) = emit_pest(grammar)?;
    let ast_shapes = ast_shapes_for_grammar(grammar);
    let ast_types = render_rust_types(&ast_shapes);
    let parser_struct = render_parser_struct(&parser_struct_name(grammar), &pest_grammar);

    Ok((
        RustParserArtifacts {
            pest_grammar,
            parser_struct,
            ast_types,
            ast_shapes,
        },
        report,
    ))
}

/// Renders a [`RustTypeShape`] as a Rust declaration.
#[must_use]
pub fn render_rust_type(shape: &RustTypeShape) -> String {
    match shape.kind() {
        RustTypeKind::Struct => render_struct(shape),
        RustTypeKind::Enum => render_enum(shape),
        RustTypeKind::Primitive => {
            format!(
                "#[derive(Debug, Clone)]\npub struct {}(pub String);\n",
                shape.name()
            )
        }
        RustTypeKind::Trait => render_trait(shape),
        RustTypeKind::Sequence => render_sequence(shape),
        RustTypeKind::Option => render_option(shape),
        RustTypeKind::Map => render_map(shape),
    }
}

fn ast_shapes_for_grammar(grammar: &Grammar) -> Vec<RustTypeShape> {
    grammar.rules().iter().map(shape_for_rule).collect()
}

fn shape_for_rule(rule: &GrammarRule) -> RustTypeShape {
    let type_name = rust_type_name(rule.name());
    if let GrammarExpr::Choice { alternatives, .. } = rule.expr() {
        return RustTypeShape::enumeration(type_name, enum_variants(alternatives));
    }

    let fields = ast_fields(rule.expr());
    if fields.is_empty() && is_terminal_only(rule.expr()) {
        RustTypeShape::structure(type_name, [RustFieldShape::new("0", "String")])
    } else {
        RustTypeShape::structure(type_name, fields)
    }
}

fn enum_variants(alternatives: &[GrammarExpr]) -> Vec<RustFieldShape> {
    let mut used = BTreeSet::new();
    alternatives
        .iter()
        .enumerate()
        .map(|(index, alternative)| {
            let name = unique_name(variant_name(alternative, index), &mut used);
            RustFieldShape::new(name, variant_type(alternative))
        })
        .collect()
}

fn variant_name(expr: &GrammarExpr, index: usize) -> String {
    match expr {
        GrammarExpr::NonTerminal(name) => rust_type_name(name),
        GrammarExpr::Capture {
            label: Some(label), ..
        } => rust_type_name(label),
        GrammarExpr::Capture { expr, .. } => variant_name(expr, index),
        GrammarExpr::Terminal(value) | GrammarExpr::TerminalInsensitive(value) => {
            rust_type_name_with_fallback(value, &format!("Literal{}", index + 1))
        }
        GrammarExpr::Empty => "Empty".to_string(),
        GrammarExpr::CharRange(_, _) | GrammarExpr::CharClass { .. } => {
            format!("Character{}", index + 1)
        }
        GrammarExpr::AnyChar => format!("Any{}", index + 1),
        GrammarExpr::Sequence(_) => format!("Sequence{}", index + 1),
        GrammarExpr::Choice { .. } => format!("Choice{}", index + 1),
        GrammarExpr::Optional(_) => format!("Optional{}", index + 1),
        GrammarExpr::ZeroOrMore(_) | GrammarExpr::OneOrMore(_) | GrammarExpr::Repeat { .. } => {
            format!("Repeated{}", index + 1)
        }
        GrammarExpr::And(_) | GrammarExpr::Not(_) => format!("Predicate{}", index + 1),
    }
}

fn variant_type(expr: &GrammarExpr) -> String {
    match expr {
        GrammarExpr::NonTerminal(name) => rust_type_name(name),
        GrammarExpr::Capture { expr, .. } => variant_type(expr),
        GrammarExpr::Empty => "()".to_string(),
        _ => "String".to_string(),
    }
}

fn ast_fields(expr: &GrammarExpr) -> Vec<RustFieldShape> {
    let mut fields = Vec::new();
    collect_fields(expr, Quantifier::Single, &mut fields);
    fields
        .into_iter()
        .map(|field| {
            let type_name = field.type_name();
            RustFieldShape::new(field.name, type_name)
        })
        .collect()
}

fn collect_fields(expr: &GrammarExpr, quantifier: Quantifier, fields: &mut Vec<AstField>) {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            push_field(
                fields,
                AstField::new(rust_field_name(name), rust_type_name(name), quantifier),
            );
        }
        GrammarExpr::Capture {
            label: Some(label),
            expr,
        } => {
            let (type_name, quantifier) = captured_type(expr, quantifier);
            push_field(
                fields,
                AstField::new(rust_field_name(label), type_name, quantifier),
            );
        }
        GrammarExpr::Capture { label: None, expr } => collect_fields(expr, quantifier, fields),
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_fields(item, quantifier, fields);
            }
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_fields(
                    alternative,
                    combine_quantifier(quantifier, Quantifier::Optional),
                    fields,
                );
            }
        }
        GrammarExpr::Optional(inner) => {
            collect_fields(
                inner,
                combine_quantifier(quantifier, Quantifier::Optional),
                fields,
            );
        }
        GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::Repeat { expr: inner, .. } => {
            collect_fields(
                inner,
                combine_quantifier(quantifier, Quantifier::Repeated),
                fields,
            );
        }
        GrammarExpr::And(_)
        | GrammarExpr::Not(_)
        | GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

fn captured_type(expr: &GrammarExpr, quantifier: Quantifier) -> (String, Quantifier) {
    match expr {
        GrammarExpr::NonTerminal(name) => (rust_type_name(name), quantifier),
        GrammarExpr::Capture { expr, .. } => captured_type(expr, quantifier),
        GrammarExpr::Optional(inner) => {
            captured_type(inner, combine_quantifier(quantifier, Quantifier::Optional))
        }
        GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::Repeat { expr: inner, .. } => {
            captured_type(inner, combine_quantifier(quantifier, Quantifier::Repeated))
        }
        _ => ("String".to_string(), quantifier),
    }
}

fn push_field(fields: &mut Vec<AstField>, field: AstField) {
    if let Some(existing) = fields
        .iter_mut()
        .find(|existing| existing.name == field.name && existing.base_type == field.base_type)
    {
        existing.quantifier = Quantifier::Repeated;
        return;
    }

    if fields.iter().any(|existing| existing.name == field.name) {
        let mut used = fields
            .iter()
            .map(|existing| existing.name.clone())
            .collect::<BTreeSet<_>>();
        let mut renamed = field;
        renamed.name = unique_name(renamed.name, &mut used);
        fields.push(renamed);
    } else {
        fields.push(field);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AstField {
    name: String,
    base_type: String,
    quantifier: Quantifier,
}

impl AstField {
    const fn new(name: String, base_type: String, quantifier: Quantifier) -> Self {
        Self {
            name,
            base_type,
            quantifier,
        }
    }

    fn type_name(&self) -> String {
        match self.quantifier {
            Quantifier::Single => self.base_type.clone(),
            Quantifier::Optional => format!("Option<{}>", self.base_type),
            Quantifier::Repeated => format!("Vec<{}>", self.base_type),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Quantifier {
    Single,
    Optional,
    Repeated,
}

const fn combine_quantifier(outer: Quantifier, inner: Quantifier) -> Quantifier {
    match (outer, inner) {
        (Quantifier::Repeated, _) | (_, Quantifier::Repeated) => Quantifier::Repeated,
        (Quantifier::Optional, _) | (_, Quantifier::Optional) => Quantifier::Optional,
        (Quantifier::Single, Quantifier::Single) => Quantifier::Single,
    }
}

fn is_terminal_only(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => true,
        GrammarExpr::NonTerminal(_) => false,
        GrammarExpr::Choice { alternatives, .. } | GrammarExpr::Sequence(alternatives) => {
            alternatives.iter().all(is_terminal_only)
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => is_terminal_only(expr),
    }
}

fn render_rust_types(shapes: &[RustTypeShape]) -> String {
    let mut output = String::new();
    for (index, shape) in shapes.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }
        output.push_str(&render_rust_type(shape));
    }
    output
}

fn render_parser_struct(name: &str, pest_grammar: &str) -> String {
    format!(
        "#[derive(pest_derive::Parser)]\n#[grammar_inline = {pest_grammar:?}]\npub struct {name};\n"
    )
}

fn render_struct(shape: &RustTypeShape) -> String {
    if shape.fields().is_empty() {
        return format!("#[derive(Debug, Clone)]\npub struct {};\n", shape.name());
    }

    if tuple_fields(shape.fields()) {
        let fields = shape
            .fields()
            .iter()
            .map(|field| format!("pub {}", field.type_name()))
            .collect::<Vec<_>>()
            .join(", ");
        return format!(
            "#[derive(Debug, Clone)]\npub struct {}({fields});\n",
            shape.name()
        );
    }

    let mut output = format!("#[derive(Debug, Clone)]\npub struct {} {{\n", shape.name());
    for field in shape.fields() {
        output.push_str("    pub ");
        output.push_str(field.name());
        output.push_str(": ");
        output.push_str(field.type_name());
        output.push_str(",\n");
    }
    output.push_str("}\n");
    output
}

fn render_enum(shape: &RustTypeShape) -> String {
    let mut output = format!("#[derive(Debug, Clone)]\npub enum {} {{\n", shape.name());
    for field in shape.fields() {
        output.push_str("    ");
        output.push_str(field.name());
        if field.type_name() == "()" {
            output.push_str(",\n");
        } else {
            output.push('(');
            output.push_str(field.type_name());
            output.push_str("),\n");
        }
    }
    output.push_str("}\n");
    output
}

fn render_trait(shape: &RustTypeShape) -> String {
    let mut output = format!("pub trait {} {{\n", shape.name());
    for field in shape.fields() {
        output.push_str("    fn ");
        output.push_str(field.name());
        output.push_str("(&self) -> ");
        output.push_str(field.type_name());
        output.push_str(";\n");
    }
    output.push_str("}\n");
    output
}

fn render_sequence(shape: &RustTypeShape) -> String {
    let element_type = shape
        .fields()
        .first()
        .map_or("()", RustFieldShape::type_name);
    format!(
        "#[derive(Debug, Clone)]\npub struct {}(pub Vec<{element_type}>);\n",
        shape.name()
    )
}

fn render_option(shape: &RustTypeShape) -> String {
    let some_type = shape
        .fields()
        .first()
        .map_or("()", RustFieldShape::type_name);
    format!(
        "#[derive(Debug, Clone)]\npub enum {} {{\n    Some({some_type}),\n    None,\n}}\n",
        shape.name()
    )
}

fn render_map(shape: &RustTypeShape) -> String {
    let key_type = shape
        .fields()
        .iter()
        .find(|field| field.name() == "key")
        .map_or("()", RustFieldShape::type_name);
    let value_type = shape
        .fields()
        .iter()
        .find(|field| field.name() == "value")
        .map_or("()", RustFieldShape::type_name);
    format!(
        "#[derive(Debug, Clone)]\npub struct {}(pub std::collections::BTreeMap<{key_type}, {value_type}>);\n",
        shape.name()
    )
}

fn tuple_fields(fields: &[RustFieldShape]) -> bool {
    fields.iter().enumerate().all(|(index, field)| {
        field
            .name()
            .parse::<usize>()
            .is_ok_and(|field_index| field_index == index)
    })
}

fn parser_struct_name(grammar: &Grammar) -> String {
    let base = grammar.start_rule().map_or("Generated", GrammarRule::name);
    format!("{}Parser", rust_type_name(base))
}

fn rust_type_name(value: &str) -> String {
    rust_type_name_with_fallback(value, "Generated")
}

fn rust_type_name_with_fallback(value: &str, fallback: &str) -> String {
    let mut output = String::new();
    let mut next_upper = true;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            if output.is_empty() && character.is_ascii_digit() {
                output.push_str(fallback);
            }
            if next_upper {
                output.push(character.to_ascii_uppercase());
            } else {
                output.push(character.to_ascii_lowercase());
            }
            next_upper = false;
        } else {
            next_upper = true;
        }
    }

    if output.is_empty() {
        fallback.to_string()
    } else {
        output
    }
}

fn rust_field_name(value: &str) -> String {
    let mut output = String::new();
    let mut previous_was_separator = false;

    for (index, character) in value.chars().enumerate() {
        if character.is_ascii_alphanumeric() {
            if output.is_empty() && character.is_ascii_digit() {
                output.push_str("field_");
            }
            if character.is_ascii_uppercase()
                && !output.is_empty()
                && index > 0
                && !previous_was_separator
            {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !output.is_empty() && !previous_was_separator {
            output.push('_');
            previous_was_separator = true;
        }
    }

    while output.ends_with('_') {
        output.pop();
    }

    if output.is_empty() {
        output.push_str("field");
    }

    if is_rust_keyword(&output) {
        format!("r#{output}")
    } else {
        output
    }
}

fn unique_name(name: String, used: &mut BTreeSet<String>) -> String {
    if used.insert(name.clone()) {
        return name;
    }

    for index in 2.. {
        let candidate = format!("{name}{index}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
    }

    unreachable!("unbounded unique-name loop always returns")
}

fn is_rust_keyword(value: &str) -> bool {
    matches!(
        value,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
    )
}
