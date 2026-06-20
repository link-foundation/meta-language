use std::char;

use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind};
use crate::link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::rust_codec::{FromLinks, LinksCodecError, LinksDecoder, LinksEncoder, ToLinks};

const GRAMMAR: &str = "grammar::grammar";
const RULE: &str = "grammar::rule";

const EXPR_EMPTY: &str = "grammar::expr::empty";
const EXPR_TERMINAL: &str = "grammar::expr::terminal";
const EXPR_TERMINAL_INSENSITIVE: &str = "grammar::expr::terminal-insensitive";
const EXPR_CHAR_RANGE: &str = "grammar::expr::char-range";
const EXPR_CHAR_CLASS: &str = "grammar::expr::char-class";
const EXPR_ANY_CHAR: &str = "grammar::expr::any-char";
const EXPR_NON_TERMINAL: &str = "grammar::expr::non-terminal";
const EXPR_CHOICE: &str = "grammar::expr::choice";
const EXPR_SEQUENCE: &str = "grammar::expr::sequence";
const EXPR_OPTIONAL: &str = "grammar::expr::optional";
const EXPR_ZERO_OR_MORE: &str = "grammar::expr::zero-or-more";
const EXPR_ONE_OR_MORE: &str = "grammar::expr::one-or-more";
const EXPR_REPEAT: &str = "grammar::expr::repeat";
const EXPR_AND: &str = "grammar::expr::and";
const EXPR_NOT: &str = "grammar::expr::not";
const EXPR_CAPTURE: &str = "grammar::expr::capture";

const CHAR_CLASS_CHAR: &str = "grammar::char-class-item::char";
const CHAR_CLASS_RANGE: &str = "grammar::char-class-item::range";

const VALUE_NONE: &str = "grammar::value::none";
const VALUE_SOME: &str = "grammar::value::some";
const VALUE_STRING_PREFIX: &str = "grammar::value::string::";
const VALUE_CHAR_PREFIX: &str = "grammar::value::char::";
const VALUE_BOOL_PREFIX: &str = "grammar::value::bool::";
const VALUE_USIZE_PREFIX: &str = "grammar::value::usize::";
const VALUE_RULE_KIND_PREFIX: &str = "grammar::value::rule-kind::";
const VALUE_FORMAT_PREFIX: &str = "grammar::value::format::";

impl ToLinks for Grammar {
    fn to_links(&self, encoder: &mut LinksEncoder) -> LinkId {
        encode_grammar(encoder.network_mut(), self)
    }
}

impl FromLinks for Grammar {
    fn from_links(decoder: &mut LinksDecoder<'_>, link: LinkId) -> Result<Self, LinksCodecError> {
        decode_grammar(decoder.network(), link)
    }
}

fn encode_grammar(network: &mut LinkNetwork, grammar: &Grammar) -> LinkId {
    let mut references = Vec::with_capacity(grammar.rules().len() + 2);
    references.push(encode_option_format(network, grammar.source_format()));
    references.push(encode_option_string(network, grammar.start()));
    references.extend(
        grammar
            .rules()
            .iter()
            .map(|rule| encode_rule(network, rule)),
    );
    insert_grammar_node(network, GRAMMAR, &references)
}

fn decode_grammar(network: &LinkNetwork, link: LinkId) -> Result<Grammar, LinksCodecError> {
    let references = expect_tag(network, link, GRAMMAR)?;
    if references.len() < 2 {
        return Err(malformed(
            link,
            "grammar links must contain source format and start references",
        ));
    }

    let source_format = decode_option_format(network, references[0])?;
    let start = decode_option_string(network, references[1])?;
    let mut grammar = Grammar::new();
    if let Some(source_format) = source_format {
        grammar = grammar.with_source_format(source_format);
    }
    if let Some(start) = start {
        grammar = grammar.with_start(start);
    }

    for rule in &references[2..] {
        grammar.add_rule(decode_rule(network, *rule)?);
    }
    Ok(grammar)
}

fn encode_rule(network: &mut LinkNetwork, rule: &GrammarRule) -> LinkId {
    let expr = encode_expr(network, rule.expr());
    let references = [
        encode_string_value(network, rule.name()),
        expr,
        encode_rule_kind_value(network, rule.kind()),
        encode_option_string(network, rule.concept()),
        encode_option_string(network, rule.doc()),
    ];
    insert_grammar_node(network, RULE, &references)
}

fn decode_rule(network: &LinkNetwork, link: LinkId) -> Result<GrammarRule, LinksCodecError> {
    let references = expect_tag(network, link, RULE)?;
    let [name, expr, kind, concept, doc] = references else {
        return Err(malformed(
            link,
            "rule links must contain name, expression, kind, concept, and doc references",
        ));
    };
    let mut rule = GrammarRule::new(
        decode_string_value(network, *name)?,
        decode_expr(network, *expr)?,
    )
    .with_kind(decode_rule_kind_value(network, *kind)?);
    if let Some(concept) = decode_option_string(network, *concept)? {
        rule = rule.with_concept(concept);
    }
    if let Some(doc) = decode_option_string(network, *doc)? {
        rule = rule.with_doc(doc);
    }
    Ok(rule)
}

fn encode_expr(network: &mut LinkNetwork, expr: &GrammarExpr) -> LinkId {
    match expr {
        GrammarExpr::Empty => insert_grammar_node(network, EXPR_EMPTY, &[]),
        GrammarExpr::Terminal(value) => {
            let references = [encode_string_value(network, value)];
            insert_grammar_node(network, EXPR_TERMINAL, &references)
        }
        GrammarExpr::TerminalInsensitive(value) => {
            let references = [encode_string_value(network, value)];
            insert_grammar_node(network, EXPR_TERMINAL_INSENSITIVE, &references)
        }
        GrammarExpr::CharRange(start, end) => {
            let references = [
                encode_char_value(network, *start),
                encode_char_value(network, *end),
            ];
            insert_grammar_node(network, EXPR_CHAR_RANGE, &references)
        }
        GrammarExpr::CharClass { negated, items } => {
            let mut references = Vec::with_capacity(items.len() + 1);
            references.push(encode_bool_value(network, *negated));
            references.extend(
                items
                    .iter()
                    .map(|item| encode_char_class_item(network, item)),
            );
            insert_grammar_node(network, EXPR_CHAR_CLASS, &references)
        }
        GrammarExpr::AnyChar => insert_grammar_node(network, EXPR_ANY_CHAR, &[]),
        GrammarExpr::NonTerminal(value) => {
            let references = [encode_string_value(network, value)];
            insert_grammar_node(network, EXPR_NON_TERMINAL, &references)
        }
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => {
            let mut references = Vec::with_capacity(alternatives.len() + 1);
            references.push(encode_bool_value(network, *ordered));
            references.extend(
                alternatives
                    .iter()
                    .map(|alternative| encode_expr(network, alternative)),
            );
            insert_grammar_node(network, EXPR_CHOICE, &references)
        }
        GrammarExpr::Sequence(items) => {
            let references = items
                .iter()
                .map(|item| encode_expr(network, item))
                .collect::<Vec<_>>();
            insert_grammar_node(network, EXPR_SEQUENCE, &references)
        }
        GrammarExpr::Optional(expr) => encode_unary_expr(network, EXPR_OPTIONAL, expr),
        GrammarExpr::ZeroOrMore(expr) => encode_unary_expr(network, EXPR_ZERO_OR_MORE, expr),
        GrammarExpr::OneOrMore(expr) => encode_unary_expr(network, EXPR_ONE_OR_MORE, expr),
        GrammarExpr::Repeat { expr, min, max } => {
            let references = [
                encode_expr(network, expr),
                encode_usize_value(network, *min),
                encode_option_usize(network, *max),
            ];
            insert_grammar_node(network, EXPR_REPEAT, &references)
        }
        GrammarExpr::And(expr) => encode_unary_expr(network, EXPR_AND, expr),
        GrammarExpr::Not(expr) => encode_unary_expr(network, EXPR_NOT, expr),
        GrammarExpr::Capture { label, expr } => {
            let references = [
                encode_option_string(network, label.as_deref()),
                encode_expr(network, expr),
            ];
            insert_grammar_node(network, EXPR_CAPTURE, &references)
        }
    }
}

fn decode_expr(network: &LinkNetwork, link: LinkId) -> Result<GrammarExpr, LinksCodecError> {
    let (term, references) = grammar_link(network, link)?;
    match term {
        EXPR_EMPTY => {
            expect_count(link, references, 0)?;
            Ok(GrammarExpr::Empty)
        }
        EXPR_TERMINAL => {
            let [value] = references else {
                return Err(expected_count(link, 1, references.len()));
            };
            Ok(GrammarExpr::Terminal(decode_string_value(network, *value)?))
        }
        EXPR_TERMINAL_INSENSITIVE => {
            let [value] = references else {
                return Err(expected_count(link, 1, references.len()));
            };
            Ok(GrammarExpr::TerminalInsensitive(decode_string_value(
                network, *value,
            )?))
        }
        EXPR_CHAR_RANGE => {
            let [start, end] = references else {
                return Err(expected_count(link, 2, references.len()));
            };
            Ok(GrammarExpr::CharRange(
                decode_char_value(network, *start)?,
                decode_char_value(network, *end)?,
            ))
        }
        EXPR_CHAR_CLASS => {
            let Some((negated, items)) = references.split_first() else {
                return Err(malformed(
                    link,
                    "character class links must contain the negated flag",
                ));
            };
            Ok(GrammarExpr::CharClass {
                negated: decode_bool_value(network, *negated)?,
                items: items
                    .iter()
                    .map(|item| decode_char_class_item(network, *item))
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
        EXPR_ANY_CHAR => {
            expect_count(link, references, 0)?;
            Ok(GrammarExpr::AnyChar)
        }
        EXPR_NON_TERMINAL => {
            let [value] = references else {
                return Err(expected_count(link, 1, references.len()));
            };
            Ok(GrammarExpr::NonTerminal(decode_string_value(
                network, *value,
            )?))
        }
        EXPR_CHOICE => {
            let Some((ordered, alternatives)) = references.split_first() else {
                return Err(malformed(
                    link,
                    "choice links must contain the ordered flag",
                ));
            };
            Ok(GrammarExpr::Choice {
                ordered: decode_bool_value(network, *ordered)?,
                alternatives: alternatives
                    .iter()
                    .map(|alternative| decode_expr(network, *alternative))
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
        EXPR_SEQUENCE => Ok(GrammarExpr::Sequence(
            references
                .iter()
                .map(|item| decode_expr(network, *item))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        EXPR_OPTIONAL => decode_unary_expr(network, link, references).map(GrammarExpr::optional),
        EXPR_ZERO_OR_MORE => {
            decode_unary_expr(network, link, references).map(GrammarExpr::zero_or_more)
        }
        EXPR_ONE_OR_MORE => {
            decode_unary_expr(network, link, references).map(GrammarExpr::one_or_more)
        }
        EXPR_REPEAT => {
            let [expr, min, max] = references else {
                return Err(expected_count(link, 3, references.len()));
            };
            Ok(GrammarExpr::repeat(
                decode_expr(network, *expr)?,
                decode_usize_value(network, *min)?,
                decode_option_usize(network, *max)?,
            ))
        }
        EXPR_AND => decode_unary_expr(network, link, references).map(GrammarExpr::and),
        EXPR_NOT => decode_unary_expr(network, link, references).map(GrammarExpr::not),
        EXPR_CAPTURE => {
            let [label, expr] = references else {
                return Err(expected_count(link, 2, references.len()));
            };
            Ok(GrammarExpr::Capture {
                label: decode_option_string(network, *label)?,
                expr: Box::new(decode_expr(network, *expr)?),
            })
        }
        _ => Err(malformed(
            link,
            format!("unexpected grammar expression tag {term:?}"),
        )),
    }
}

fn encode_unary_expr(network: &mut LinkNetwork, tag: &str, expr: &GrammarExpr) -> LinkId {
    let references = [encode_expr(network, expr)];
    insert_grammar_node(network, tag, &references)
}

fn decode_unary_expr(
    network: &LinkNetwork,
    link: LinkId,
    references: &[LinkId],
) -> Result<GrammarExpr, LinksCodecError> {
    let [expr] = references else {
        return Err(expected_count(link, 1, references.len()));
    };
    decode_expr(network, *expr)
}

fn encode_char_class_item(network: &mut LinkNetwork, item: &CharClassItem) -> LinkId {
    match item {
        CharClassItem::Char(value) => {
            let references = [encode_char_value(network, *value)];
            insert_grammar_node(network, CHAR_CLASS_CHAR, &references)
        }
        CharClassItem::Range(start, end) => {
            let references = [
                encode_char_value(network, *start),
                encode_char_value(network, *end),
            ];
            insert_grammar_node(network, CHAR_CLASS_RANGE, &references)
        }
    }
}

fn decode_char_class_item(
    network: &LinkNetwork,
    link: LinkId,
) -> Result<CharClassItem, LinksCodecError> {
    let (term, references) = grammar_link(network, link)?;
    match term {
        CHAR_CLASS_CHAR => {
            let [value] = references else {
                return Err(expected_count(link, 1, references.len()));
            };
            Ok(CharClassItem::Char(decode_char_value(network, *value)?))
        }
        CHAR_CLASS_RANGE => {
            let [start, end] = references else {
                return Err(expected_count(link, 2, references.len()));
            };
            Ok(CharClassItem::Range(
                decode_char_value(network, *start)?,
                decode_char_value(network, *end)?,
            ))
        }
        _ => Err(malformed(
            link,
            format!("unexpected character class item tag {term:?}"),
        )),
    }
}

fn encode_option_string(network: &mut LinkNetwork, value: Option<&str>) -> LinkId {
    match value {
        Some(value) => {
            let references = [encode_string_value(network, value)];
            insert_grammar_node(network, VALUE_SOME, &references)
        }
        None => insert_grammar_node(network, VALUE_NONE, &[]),
    }
}

fn decode_option_string(
    network: &LinkNetwork,
    link: LinkId,
) -> Result<Option<String>, LinksCodecError> {
    decode_option(network, link, decode_string_value)
}

fn encode_option_usize(network: &mut LinkNetwork, value: Option<usize>) -> LinkId {
    match value {
        Some(value) => {
            let references = [encode_usize_value(network, value)];
            insert_grammar_node(network, VALUE_SOME, &references)
        }
        None => insert_grammar_node(network, VALUE_NONE, &[]),
    }
}

fn decode_option_usize(
    network: &LinkNetwork,
    link: LinkId,
) -> Result<Option<usize>, LinksCodecError> {
    decode_option(network, link, decode_usize_value)
}

fn encode_option_format(network: &mut LinkNetwork, value: Option<GrammarFormat>) -> LinkId {
    match value {
        Some(value) => {
            let references = [encode_format_value(network, value)];
            insert_grammar_node(network, VALUE_SOME, &references)
        }
        None => insert_grammar_node(network, VALUE_NONE, &[]),
    }
}

fn decode_option_format(
    network: &LinkNetwork,
    link: LinkId,
) -> Result<Option<GrammarFormat>, LinksCodecError> {
    decode_option(network, link, decode_format_value)
}

fn decode_option<T>(
    network: &LinkNetwork,
    link: LinkId,
    decode_value: fn(&LinkNetwork, LinkId) -> Result<T, LinksCodecError>,
) -> Result<Option<T>, LinksCodecError> {
    let (term, references) = grammar_link(network, link)?;
    match term {
        VALUE_NONE => {
            expect_count(link, references, 0)?;
            Ok(None)
        }
        VALUE_SOME => {
            let [value] = references else {
                return Err(expected_count(link, 1, references.len()));
            };
            decode_value(network, *value).map(Some)
        }
        _ => Err(malformed(
            link,
            format!("expected optional value tag, found {term:?}"),
        )),
    }
}

fn encode_string_value(network: &mut LinkNetwork, value: &str) -> LinkId {
    insert_grammar_value(
        network,
        &format!("{VALUE_STRING_PREFIX}{}", hex_encode(value)),
    )
}

fn decode_string_value(network: &LinkNetwork, link: LinkId) -> Result<String, LinksCodecError> {
    let value = prefixed_value(network, link, VALUE_STRING_PREFIX, "String")?;
    hex_decode(link, "String", value)
}

fn encode_char_value(network: &mut LinkNetwork, value: char) -> LinkId {
    insert_grammar_value(
        network,
        &format!("{VALUE_CHAR_PREFIX}{:x}", u32::from(value)),
    )
}

fn decode_char_value(network: &LinkNetwork, link: LinkId) -> Result<char, LinksCodecError> {
    let value = prefixed_value(network, link, VALUE_CHAR_PREFIX, "char")?;
    let code = u32::from_str_radix(value, 16)
        .map_err(|error| invalid_value(link, "char", Some(value), &error.to_string()))?;
    char::from_u32(code).ok_or_else(|| invalid_value(link, "char", Some(value), "invalid char"))
}

fn encode_bool_value(network: &mut LinkNetwork, value: bool) -> LinkId {
    insert_grammar_value(network, &format!("{VALUE_BOOL_PREFIX}{value}"))
}

fn decode_bool_value(network: &LinkNetwork, link: LinkId) -> Result<bool, LinksCodecError> {
    let value = prefixed_value(network, link, VALUE_BOOL_PREFIX, "bool")?;
    value.parse().map_err(|error: std::str::ParseBoolError| {
        invalid_value(link, "bool", Some(value), &error.to_string())
    })
}

fn encode_usize_value(network: &mut LinkNetwork, value: usize) -> LinkId {
    insert_grammar_value(network, &format!("{VALUE_USIZE_PREFIX}{value}"))
}

fn decode_usize_value(network: &LinkNetwork, link: LinkId) -> Result<usize, LinksCodecError> {
    let value = prefixed_value(network, link, VALUE_USIZE_PREFIX, "usize")?;
    value.parse().map_err(|error: std::num::ParseIntError| {
        invalid_value(link, "usize", Some(value), &error.to_string())
    })
}

fn encode_rule_kind_value(network: &mut LinkNetwork, value: RuleKind) -> LinkId {
    insert_grammar_value(
        network,
        &format!("{VALUE_RULE_KIND_PREFIX}{}", value.as_str()),
    )
}

fn decode_rule_kind_value(
    network: &LinkNetwork,
    link: LinkId,
) -> Result<RuleKind, LinksCodecError> {
    let value = prefixed_value(network, link, VALUE_RULE_KIND_PREFIX, "RuleKind")?;
    RuleKind::from_tag(value)
        .ok_or_else(|| invalid_value(link, "RuleKind", Some(value), "unknown rule kind"))
}

fn encode_format_value(network: &mut LinkNetwork, value: GrammarFormat) -> LinkId {
    insert_grammar_value(network, &format!("{VALUE_FORMAT_PREFIX}{}", value.as_str()))
}

fn decode_format_value(
    network: &LinkNetwork,
    link: LinkId,
) -> Result<GrammarFormat, LinksCodecError> {
    let value = prefixed_value(network, link, VALUE_FORMAT_PREFIX, "GrammarFormat")?;
    GrammarFormat::from_tag(value)
        .ok_or_else(|| invalid_value(link, "GrammarFormat", Some(value), "unknown grammar format"))
}

fn insert_grammar_node(network: &mut LinkNetwork, term: &str, references: &[LinkId]) -> LinkId {
    network.insert_dynamic_link(
        references,
        LinkMetadata::new()
            .with_link_type(LinkType::Grammar)
            .with_term(term),
    )
}

fn insert_grammar_value(network: &mut LinkNetwork, term: &str) -> LinkId {
    insert_grammar_node(network, term, &[])
}

fn expect_tag<'network>(
    network: &'network LinkNetwork,
    link: LinkId,
    expected: &str,
) -> Result<&'network [LinkId], LinksCodecError> {
    let (term, references) = grammar_link(network, link)?;
    if term == expected {
        Ok(references)
    } else {
        Err(malformed(
            link,
            format!("expected grammar tag {expected:?}, found {term:?}"),
        ))
    }
}

fn grammar_link(network: &LinkNetwork, link: LinkId) -> Result<(&str, &[LinkId]), LinksCodecError> {
    let link = network
        .link(link)
        .ok_or(LinksCodecError::MissingLink(link))?;
    expect_grammar_type(link)?;
    let Some(term) = link.metadata().term() else {
        return Err(malformed(link.id(), "grammar link is missing its term tag"));
    };
    Ok((term, link.references()))
}

fn expect_grammar_type(link: &Link) -> Result<(), LinksCodecError> {
    if link.metadata().link_type() == Some(LinkType::Grammar) {
        return Ok(());
    }
    Err(malformed(
        link.id(),
        format!(
            "expected LinkType::Grammar, found {:?}",
            link.metadata().link_type()
        ),
    ))
}

fn prefixed_value<'network>(
    network: &'network LinkNetwork,
    link: LinkId,
    prefix: &str,
    type_name: &str,
) -> Result<&'network str, LinksCodecError> {
    let (term, _) = grammar_link(network, link)?;
    term.strip_prefix(prefix)
        .ok_or_else(|| invalid_value(link, type_name, Some(term), "wrong value prefix"))
}

fn expect_count(
    link: LinkId,
    references: &[LinkId],
    expected: usize,
) -> Result<(), LinksCodecError> {
    if references.len() == expected {
        Ok(())
    } else {
        Err(expected_count(link, expected, references.len()))
    }
}

fn expected_count(link: LinkId, expected: usize, actual: usize) -> LinksCodecError {
    malformed(
        link,
        format!("expected {expected} references, found {actual}"),
    )
}

fn malformed(link: LinkId, reason: impl Into<String>) -> LinksCodecError {
    LinksCodecError::MalformedObject {
        object: link,
        reason: reason.into(),
    }
}

fn invalid_value(
    link: LinkId,
    type_name: &str,
    value: Option<&str>,
    reason: &str,
) -> LinksCodecError {
    LinksCodecError::InvalidLiteral {
        object: link,
        type_name: type_name.to_string(),
        value: value.map(ToString::to_string),
        reason: reason.to_string(),
    }
}

fn hex_encode(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn hex_decode(link: LinkId, type_name: &str, value: &str) -> Result<String, LinksCodecError> {
    if value.len() % 2 != 0 {
        return Err(invalid_value(
            link,
            type_name,
            Some(value),
            "hex payload has odd length",
        ));
    }

    let mut bytes = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().chunks_exact(2) {
        let high = hex_digit(pair[0])
            .ok_or_else(|| invalid_value(link, type_name, Some(value), "invalid hex digit"))?;
        let low = hex_digit(pair[1])
            .ok_or_else(|| invalid_value(link, type_name, Some(value), "invalid hex digit"))?;
        bytes.push((high << 4) | low);
    }

    String::from_utf8(bytes).map_err(|error| {
        invalid_value(
            link,
            type_name,
            Some(value),
            &format!("invalid UTF-8: {error}"),
        )
    })
}

const fn hex_digit(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}
