//! The grammar feature union in the native links form: the declaration links
//! `(import NAME)`, `(mode NAME)`, `(extra EXPRESSION)`, `(conflict NAME...)`,
//! `(precedences (name|rule NAME)...)`, `(macro NAME (parameters P...) EXPRESSION)` and `(scanner NAME (tokens
//! T...) (operations OPERATION...))` between the grammar link and the first
//! rule, the rule fields `(parameters P...)`, `(channel NAME)`, `(modes
//! M...)` and `(action OPERATION...)` and the rule metadata `(concept ID)` and
//! `(source-names (SOURCE NAME)...)` before `(doc TEXT)`, and the feature
//! expressions. It mirrors `js/src/grammar-links.js`.

use super::super::feature::class_expression;
use super::super::{
    FeatureExpr, GrammarDeclarations, GrammarExpr, GrammarImportError, GrammarMacro,
    GrammarScanner, GrammarSourceName, Operation, OperationCategory, PrecedenceEntry,
    RuleAttributes, UnicodeClassItem,
};
use super::feature_forms::{
    FormCodec, LinksReader, read_links_feature, read_links_operation, render_feature_form,
    render_operation,
};
use super::links::{
    Node, arity, char_text, decoded_word, link, links_error, parse_node, parts,
    percent_decode_links_text, percent_encode_links_text, render_links_expression, word,
};

/// The links spelling of the feature union forms.
struct LinksCodec;

impl FormCodec for LinksCodec {
    fn name(&self, value: &str) -> String {
        percent_encode_links_text(value)
    }

    fn text(&self, value: &str) -> String {
        percent_encode_links_text(value)
    }

    fn expression(&self, expr: &GrammarExpr) -> String {
        render_links_expression(expr)
    }

    fn call(&self, head: &str, parts: Vec<String>) -> String {
        if parts.is_empty() {
            return head.to_owned();
        }
        let mut all = vec![head.to_owned()];
        all.extend(parts);
        link(&all)
    }

    fn block(&self, word: &str, parts: Vec<String>) -> String {
        let mut all = vec![word.to_owned()];
        all.extend(parts);
        link(&all)
    }

    fn byte_class(&self, negated: bool, items: Vec<String>) -> String {
        let mut all = vec![
            "byteClass".to_owned(),
            if negated { "negated" } else { "plain" }.to_owned(),
        ];
        all.extend(items);
        link(&all)
    }
}

/// Renders one feature union expression of the native links form.
pub(super) fn render_links_feature(feature: &FeatureExpr) -> String {
    match feature {
        FeatureExpr::UnicodeClass { negated, items } => {
            let mut parts = vec![
                "class".to_owned(),
                if *negated { "negated" } else { "plain" }.to_owned(),
            ];
            parts.extend(items.iter().map(|item| match item {
                UnicodeClassItem::Char(value) => link(&["char".to_owned(), char_text(*value)]),
                UnicodeClassItem::Range(start, end) => {
                    link(&["range".to_owned(), char_text(*start), char_text(*end)])
                }
                UnicodeClassItem::Category(value) => {
                    link(&["category".to_owned(), percent_encode_links_text(value)])
                }
                UnicodeClassItem::Script(value) => {
                    link(&["script".to_owned(), percent_encode_links_text(value)])
                }
            }));
            link(&parts)
        }
        FeatureExpr::Call { name, arguments } => {
            let mut parts = vec!["ref".to_owned(), percent_encode_links_text(name)];
            parts.extend(arguments.iter().map(render_links_expression));
            link(&parts)
        }
        FeatureExpr::Form(_) | FeatureExpr::ByteClass { .. } => {
            render_feature_form(feature, &LinksCodec).unwrap_or_default()
        }
    }
}

fn render_names(head: &str, names: &[String]) -> String {
    let mut parts = vec![head.to_owned()];
    parts.extend(names.iter().map(|name| percent_encode_links_text(name)));
    link(&parts)
}

fn render_operations(head: &str, operations: &[Operation]) -> String {
    let mut parts = vec![head.to_owned()];
    parts.extend(
        operations
            .iter()
            .map(|operation| render_operation(operation, &LinksCodec)),
    );
    link(&parts)
}

/// The declaration links between the grammar link and the first rule.
pub fn render_declaration_links(declarations: &GrammarDeclarations) -> Vec<String> {
    let mut lines = Vec::new();
    for name in &declarations.imports {
        lines.push(format!("(import {})", percent_encode_links_text(name)));
    }
    for name in &declarations.modes {
        lines.push(format!("(mode {})", percent_encode_links_text(name)));
    }
    for extra in &declarations.extras {
        lines.push(format!("(extra {})", render_links_expression(extra)));
    }
    for group in &declarations.conflicts {
        lines.push(render_names("conflict", group));
    }
    for order in &declarations.precedences {
        let entries = order.iter().map(|entry| {
            format!(
                "({} {})",
                entry.kind(),
                percent_encode_links_text(entry.value())
            )
        });
        lines.push(format!(
            "(precedences {})",
            entries.collect::<Vec<_>>().join(" ")
        ));
    }
    for declared in &declarations.macros {
        lines.push(format!(
            "(macro {} {} {})",
            percent_encode_links_text(&declared.name),
            render_names("parameters", &declared.parameters),
            render_links_expression(&declared.expression)
        ));
    }
    for scanner in &declarations.scanners {
        lines.push(format!(
            "(scanner {} {} {})",
            percent_encode_links_text(&scanner.name),
            render_names("tokens", &scanner.tokens),
            render_operations("operations", &scanner.operations)
        ));
    }
    lines
}

/// The rule fields before the `(doc TEXT)` field.
pub fn render_rule_fields(attributes: &RuleAttributes) -> Vec<String> {
    let mut fields = Vec::new();
    if !attributes.parameters.is_empty() {
        fields.push(render_names("parameters", &attributes.parameters));
    }
    if let Some(channel) = &attributes.channel {
        fields.push(format!("(channel {})", percent_encode_links_text(channel)));
    }
    if let Some(modes) = &attributes.modes {
        fields.push(render_names("modes", modes));
    }
    if let Some(action) = &attributes.action {
        fields.push(render_operations("action", action));
    }
    fields
}

/// The links reader of the shared feature form tables.
struct Helpers;

impl LinksReader for Helpers {
    type Node = Node;

    fn parts<'a>(&self, node: &'a Node) -> Result<(&'a str, &'a [Node]), GrammarImportError> {
        parts(node)
    }

    fn word<'a>(&self, node: &'a Node, what: &str) -> Result<&'a str, GrammarImportError> {
        word(Some(node), what)
    }

    fn decoded_word(&self, node: &Node, what: &str) -> Result<String, GrammarImportError> {
        decoded_word(Some(node), what)
    }

    fn expression(&self, node: &Node) -> Result<GrammarExpr, GrammarImportError> {
        parse_node(node)
    }

    fn fail(&self, detail: String) -> GrammarImportError {
        links_error(detail)
    }
}

/// Reads a feature union expression headed by `head`, or `None`.
pub(super) fn read_feature(
    head: &str,
    args: &[Node],
) -> Result<Option<GrammarExpr>, GrammarImportError> {
    read_links_feature(&Helpers, head, args)
}

/// Reads a `(class plain|negated ITEM...)` link's items.
pub(super) fn read_class(negated: bool, items: &[Node]) -> Result<GrammarExpr, GrammarImportError> {
    let items = items
        .iter()
        .map(|item| {
            let (head, args) = parts(item)?;
            match head {
                "char" => {
                    arity(head, args, 1)?;
                    Ok(UnicodeClassItem::Char(super::links::character(
                        args.first(),
                        "a class character",
                    )?))
                }
                "range" => {
                    arity(head, args, 2)?;
                    Ok(UnicodeClassItem::Range(
                        super::links::character(args.first(), "a range start")?,
                        super::links::character(args.get(1), "a range end")?,
                    ))
                }
                "category" | "script" => {
                    arity(head, args, 1)?;
                    let value = decoded_word(args.first(), &format!("a Unicode {head}"))?;
                    Ok(if head == "category" {
                        UnicodeClassItem::Category(value)
                    } else {
                        UnicodeClassItem::Script(value)
                    })
                }
                other => Err(links_error(format!("unknown class item {other}"))),
            }
        })
        .collect::<Result<_, _>>()?;
    Ok(class_expression(negated, items))
}

fn names(node: &Node, head: &str) -> Result<Vec<String>, GrammarImportError> {
    let (found, args) = parts(node)?;
    if found != head {
        return Err(links_error(format!("expected ({head} ...), not {found}")));
    }
    args.iter()
        .map(|item| decoded_word(Some(item), &format!("a {head} name")))
        .collect()
}

fn operations(node: &Node, head: &str) -> Result<Vec<Operation>, GrammarImportError> {
    let (found, args) = parts(node)?;
    if found != head {
        return Err(links_error(format!("expected ({head} ...), not {found}")));
    }
    args.iter()
        .map(|item| read_links_operation(&Helpers, item, OperationCategory::Statement))
        .collect()
}

/// Reads one declaration link into `declarations`; `false` when `head` names none.
pub(super) fn read_declaration(
    head: &str,
    args: &[Node],
    declarations: &mut GrammarDeclarations,
) -> Result<bool, GrammarImportError> {
    let one = || arity(head, args, 1).map(|()| &args[0]);
    match head {
        "import" => declarations
            .imports
            .push(decoded_word(Some(one()?), "an imported grammar")?),
        "mode" => declarations
            .modes
            .push(decoded_word(Some(one()?), "a mode")?),
        "extra" => declarations.extras.push(parse_node(one()?)?),
        "conflict" => {
            if args.is_empty() {
                return Err(links_error("conflict names at least one rule"));
            }
            declarations.conflicts.push(
                args.iter()
                    .map(|item| decoded_word(Some(item), "a rule name"))
                    .collect::<Result<_, _>>()?,
            );
        }
        "precedences" => {
            if args.len() < 2 {
                return Err(links_error("precedences orders at least two entries"));
            }
            let mut order = Vec::new();
            for item in args {
                let (kind, values) = parts(item)?;
                let what = match kind {
                    "name" => "a precedence name",
                    "rule" => "a rule name",
                    other => {
                        return Err(links_error(format!(
                            "a precedence entry is (name NAME) or (rule NAME), not {other}"
                        )));
                    }
                };
                arity(kind, values, 1)?;
                let value = decoded_word(values.first(), what)?;
                order.extend(PrecedenceEntry::from_kind(kind, value));
            }
            declarations.precedences.push(order);
        }
        "macro" => {
            arity(head, args, 3)?;
            declarations.macros.push(GrammarMacro {
                name: decoded_word(args.first(), "a macro name")?,
                parameters: names(&args[1], "parameters")?,
                expression: parse_node(&args[2])?,
            });
        }
        "scanner" => {
            arity(head, args, 3)?;
            declarations.scanners.push(GrammarScanner {
                name: decoded_word(args.first(), "a scanner name")?,
                tokens: names(&args[1], "tokens")?,
                operations: operations(&args[2], "operations")?,
            });
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/// The rule fields in order: the attributes, the metadata `(concept ID)` and
/// `(source-names (SOURCE NAME)...)`, and `(doc TEXT)`.
const RULE_FIELDS: [&str; 7] = [
    "parameters",
    "channel",
    "modes",
    "action",
    "concept",
    "source-names",
    "doc",
];

/// The optional fields of a rule link.
#[derive(Default)]
pub(super) struct RuleFields {
    pub attributes: RuleAttributes,
    pub concept: Option<String>,
    pub source_names: Vec<GrammarSourceName>,
    pub doc: Option<String>,
}

/// Reads the `(SOURCE NAME)...` items of `(source-names (SOURCE NAME)...)`.
pub(super) fn read_source_names(
    args: &[Node],
) -> Result<Vec<GrammarSourceName>, GrammarImportError> {
    if args.is_empty() {
        return Err(links_error("source-names lists at least one source name"));
    }
    args.iter()
        .map(|item| {
            let (source, name) = parts(item)?;
            if name.len() != 1 {
                return Err(links_error("a source name is (SOURCE NAME)"));
            }
            Ok(GrammarSourceName {
                source: percent_decode_links_text(source)?,
                name: decoded_word(name.first(), "a source name")?,
            })
        })
        .collect()
}

/// Reads the optional fields of a rule link: its attributes, its metadata and
/// its doc.
pub(super) fn read_rule_fields(fields: &[Node]) -> Result<RuleFields, GrammarImportError> {
    let mut read = RuleFields::default();
    let mut order = 0;
    for field in fields {
        let (head, args) = parts(field)?;
        let Some(position) = RULE_FIELDS.iter().position(|name| *name == head) else {
            return Err(links_error(format!("unexpected rule field {head}")));
        };
        if position < order {
            return Err(links_error(format!("rule field {head} is out of order")));
        }
        order = position + 1;
        match head {
            "parameters" => read.attributes.parameters = names(field, "parameters")?,
            "modes" => read.attributes.modes = Some(names(field, "modes")?),
            "action" => read.attributes.action = Some(operations(field, "action")?),
            "source-names" => read.source_names = read_source_names(args)?,
            "channel" => {
                arity(head, args, 1)?;
                read.attributes.channel = Some(decoded_word(args.first(), "a channel name")?);
            }
            "concept" => {
                arity(head, args, 1)?;
                read.concept = Some(decoded_word(args.first(), "a concept id")?);
            }
            _ => {
                arity(head, args, 1)?;
                read.doc = Some(decoded_word(args.first(), "a doc text")?);
            }
        }
    }
    Ok(read)
}
