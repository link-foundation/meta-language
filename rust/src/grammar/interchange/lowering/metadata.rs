//! The reconstruction metadata of a lowering as links, the reconstruction of
//! the original grammar from the executable and the metadata, and the check
//! that reports every feature a lowering does not carry.

use std::collections::HashMap;

use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};

use super::super::super::round_trip::canonical_rule_definition;
use super::super::super::runtime::GrammarParser;
use super::super::super::{
    Grammar, GrammarDeclarations, GrammarExpr, GrammarFormat, GrammarImportError, GrammarRule,
    RuleAttributes, RuleKind,
};
use super::super::links::{
    parse_grammar_links, parse_node, percent_decode_links_text, percent_encode_links_text,
    render_links_expression,
};
use super::super::{grammar_importer, render_declaration_links, render_rule_fields};
use super::{
    GrammarImportFn, GrammarLowering, GrammarLoweringEncoding, GrammarLoweringError,
    GrammarLoweringMetadata, GrammarLoweringOptions, GrammarLoweringStatus, GrammarLoweringStep,
    assemble, check_format, lower_grammar, map_children, rule,
};

type Node = LiNo<String>;

/// Renders the reconstruction metadata as links, one link per line.
#[must_use]
pub fn render_lowering_metadata(metadata: &GrammarLoweringMetadata) -> String {
    let text = |value: &str| percent_encode_links_text(value);
    let mut lines = vec![
        format!(
            "(lowering {} {})",
            metadata.format,
            metadata.status.as_str()
        ),
        metadata.source.map_or_else(
            || "(source)".to_owned(),
            |source| format!("(source {})", text(source.as_str())),
        ),
        format!("(start {})", text(&metadata.start)),
        format!(
            "(rules {})",
            metadata
                .order
                .iter()
                .map(|name| text(name))
                .collect::<Vec<_>>()
                .join(" ")
        ),
    ];
    for step in &metadata.steps {
        lines.push(match step {
            GrammarLoweringStep::Helper {
                helper,
                owner,
                construct,
                encoding,
                note,
                original,
            } => format!(
                "(helper {} {} {construct} {} {} {})",
                text(helper),
                text(owner),
                encoding.as_str(),
                text(note),
                render_links_expression(original)
            ),
            GrammarLoweringStep::Rename { rule, value } => {
                format!("(rename {} {})", text(rule), text(value))
            }
            GrammarLoweringStep::Kind { rule, kind } => {
                format!("(kind {} {})", text(rule), text(kind.as_str()))
            }
            GrammarLoweringStep::Doc { rule, doc } => {
                format!("(doc {} {})", text(rule), text(doc))
            }
            GrammarLoweringStep::Declarations { declarations } => {
                format!("(declarations {})", text(declarations))
            }
            GrammarLoweringStep::Attributes { rule, attributes } => {
                format!("(attributes {} {})", text(rule), text(attributes))
            }
        });
    }
    lines.iter().fold(String::new(), |mut text, line| {
        text.push_str(line);
        text.push('\n');
        text
    })
}

/// The declarations as native links (a grammar link with the matching and
/// the settling, then one link per declaration), or `None` for a grammar without declarations.
pub(super) fn render_declarations_text(declarations: &GrammarDeclarations) -> Option<String> {
    let lines = render_declaration_links(declarations);
    if declarations.matching.is_none() && declarations.settling.is_none() && lines.is_empty() {
        return None;
    }
    let fields = declarations
        .matching
        .iter()
        .map(|matching| format!(" (matching {matching})"))
        .chain(
            declarations
                .settling
                .iter()
                .map(|settling| format!(" (settling {})", settling.join(" "))),
        )
        .collect::<String>();
    let header = format!("(grammar{fields})");
    Some(
        std::iter::once(header)
            .chain(lines)
            .fold(String::new(), |mut text, line| {
                text.push_str(&line);
                text.push('\n');
                text
            }),
    )
}

/// Reads a declarations step back through the native links reader, which
/// needs one rule after the declarations.
fn parse_declarations_text(text: &str) -> Result<GrammarDeclarations, GrammarLoweringError> {
    Ok(
        parse_grammar_links(&format!("{text}(rule _ normal empty)\n"))?
            .declarations()
            .clone(),
    )
}

/// Reads an attributes step back as the rule fields of a placeholder rule.
fn parse_rule_fields(fields: &str) -> Result<RuleAttributes, GrammarLoweringError> {
    let grammar = parse_grammar_links(&format!("(grammar)\n(rule _ normal empty {fields})\n"))?;
    Ok(grammar
        .rule("_")
        .map(|placeholder| placeholder.attributes.clone())
        .unwrap_or_default())
}

fn metadata_error(detail: impl AsRef<str>) -> GrammarLoweringError {
    GrammarLoweringError::Import(GrammarImportError::Parse {
        format: GrammarFormat::MetaLanguage,
        message: format!("lowering metadata: {}", detail.as_ref()),
    })
}

fn as_word(node: &Node) -> Option<&str> {
    match node {
        LiNo::Ref(word) => Some(word),
        LiNo::Link {
            id: Some(word),
            values,
        } if values.is_empty() => Some(word),
        LiNo::Link { .. } => None,
    }
}

fn word(node: Option<&Node>) -> Result<&str, GrammarLoweringError> {
    node.and_then(as_word)
        .ok_or_else(|| metadata_error("expected a word"))
}

fn decoded(node: Option<&Node>) -> Result<String, GrammarLoweringError> {
    Ok(percent_decode_links_text(word(node)?)?)
}

/// A metadata statement as its head word and arguments; a link of one word
/// reads as a word.
fn head(statement: &Node) -> Result<(&str, &[Node]), GrammarLoweringError> {
    if let Some(word) = as_word(statement) {
        return Ok((word, &[]));
    }
    let LiNo::Link { id: None, values } = statement else {
        return Err(metadata_error("every metadata entry is a link"));
    };
    let Some((first, rest)) = values.split_first() else {
        return Err(metadata_error("every metadata entry is a link"));
    };
    as_word(first)
        .map(|first| (first, rest))
        .ok_or_else(|| metadata_error("a metadata link starts with a word"))
}

fn step(kind: &str, args: &[Node]) -> Result<GrammarLoweringStep, GrammarLoweringError> {
    match (kind, args.len()) {
        ("helper", 6) => {
            let encoding = match word(args.get(3))? {
                "exact" => GrammarLoweringEncoding::Exact,
                "approximate" => GrammarLoweringEncoding::Approximate,
                other => return Err(metadata_error(format!("unknown encoding {other}"))),
            };
            Ok(GrammarLoweringStep::Helper {
                helper: decoded(args.first())?,
                owner: decoded(args.get(1))?,
                construct: word(args.get(2))?.to_owned(),
                encoding,
                note: decoded(args.get(4))?,
                original: parse_node(&args[5])?,
            })
        }
        ("declarations", 1) => {
            let declarations = decoded(args.first())?;
            parse_declarations_text(&declarations)?;
            Ok(GrammarLoweringStep::Declarations { declarations })
        }
        ("attributes", 2) => {
            let attributes = decoded(args.get(1))?;
            parse_rule_fields(&attributes)?;
            Ok(GrammarLoweringStep::Attributes {
                rule: decoded(args.first())?,
                attributes,
            })
        }
        ("rename" | "kind" | "doc", 2) => {
            let value = decoded(args.get(1))?;
            if kind == "kind" {
                let Some(kind) = RuleKind::from_tag(&value) else {
                    return Err(metadata_error(format!("unknown rule kind {value}")));
                };
                return Ok(GrammarLoweringStep::Kind {
                    rule: decoded(args.first())?,
                    kind,
                });
            }
            let rule = decoded(args.first())?;
            Ok(if kind == "rename" {
                GrammarLoweringStep::Rename { rule, value }
            } else {
                GrammarLoweringStep::Doc { rule, doc: value }
            })
        }
        _ => Err(metadata_error(format!("unexpected link {kind}"))),
    }
}

/// Reads the metadata written by [`render_lowering_metadata`].
///
/// # Errors
///
/// Returns a [`GrammarLoweringError`] for malformed links, an unsupported
/// format or an unknown status, encoding, rule kind or source format.
pub fn parse_lowering_metadata(
    source: &str,
) -> Result<GrammarLoweringMetadata, GrammarLoweringError> {
    let statements = parse_lino_to_links_with_config(source, &ParserConfig::without_comments())
        .map_err(|error| metadata_error(error.to_string()))?;
    let links = statements.iter().map(head).collect::<Result<Vec<_>, _>>()?;
    let [
        (header, header_args),
        (source_head, source_args),
        (start_head, start_args),
        (rules_head, rules_args),
        rest @ ..,
    ] = links.as_slice()
    else {
        return Err(metadata_error(
            "the metadata starts with the lowering, source, start and rules links",
        ));
    };
    if *header != "lowering" || header_args.len() != 2 {
        return Err(metadata_error(
            "the first link must be (lowering FORMAT STATUS)",
        ));
    }
    let format = word(header_args.first())?;
    check_format(format)?;
    let status = match word(header_args.get(1))? {
        "exact" => GrammarLoweringStatus::Exact,
        "approximate" => GrammarLoweringStatus::Approximate,
        other => return Err(metadata_error(format!("unknown status {other}"))),
    };
    if *source_head != "source" || source_args.len() > 1 {
        return Err(metadata_error("the second link must be (source [FORMAT])"));
    }
    if *start_head != "start" || start_args.len() != 1 {
        return Err(metadata_error("the third link must be (start NAME)"));
    }
    if *rules_head != "rules" {
        return Err(metadata_error("the fourth link must be (rules NAME...)"));
    }
    let steps = rest
        .iter()
        .map(|(kind, args)| step(kind, args))
        .collect::<Result<Vec<_>, _>>()?;
    let source_format = if source_args.is_empty() {
        None
    } else {
        let tag = decoded(source_args.first())?;
        let Some(format) = GrammarFormat::from_tag(&tag) else {
            return Err(metadata_error(format!("unknown source format {tag}")));
        };
        Some(format)
    };
    Ok(GrammarLoweringMetadata {
        format: format.to_owned(),
        status,
        source: source_format,
        start: decoded(start_args.first())?,
        order: rules_args
            .iter()
            .map(|value| decoded(Some(value)))
            .collect::<Result<_, _>>()?,
        steps,
    })
}

/// Reconstructs the original grammar from an executable text and its
/// lowering metadata.
///
/// The executable is imported, every renamed rule gets its
/// original name back, every helper reference is replaced with the original
/// expression the metadata records, the helpers are removed and the rule
/// kinds and documentation the target did not keep, the rule fields and the
/// grammar declarations are restored. `import`
/// replaces the format's own importer.
///
/// # Errors
///
/// Returns a [`GrammarLoweringError`] for malformed metadata, an importer
/// error or an executable without a rule the metadata names.
pub fn reconstruct_grammar(
    executable: &str,
    metadata: &str,
    import: Option<GrammarImportFn<'_>>,
) -> Result<Grammar, GrammarLoweringError> {
    let metadata = parse_lowering_metadata(metadata)?;
    let own_import = grammar_importer(&metadata.format)
        .ok_or_else(|| GrammarLoweringError::UnsupportedFormat(metadata.format.clone()))?;
    let imported = import.unwrap_or(&own_import)(executable)?;
    let mut helpers: HashMap<&str, &GrammarExpr> = HashMap::new();
    let mut renames: HashMap<&str, &str> = HashMap::new();
    let mut originals: HashMap<&str, &str> = HashMap::new();
    let mut kinds: HashMap<&str, RuleKind> = HashMap::new();
    let mut docs: HashMap<&str, &str> = HashMap::new();
    let mut attributes: HashMap<&str, RuleAttributes> = HashMap::new();
    let mut declarations = None;
    for step in &metadata.steps {
        match step {
            GrammarLoweringStep::Helper {
                helper, original, ..
            } => {
                helpers.insert(helper, original);
            }
            GrammarLoweringStep::Rename { rule, value } => {
                renames.insert(rule, value);
                originals.insert(value, rule);
            }
            GrammarLoweringStep::Kind { rule, kind } => {
                kinds.insert(rule, *kind);
            }
            GrammarLoweringStep::Doc { rule, doc } => {
                docs.insert(rule, doc);
            }
            GrammarLoweringStep::Declarations { declarations: text } => declarations = Some(text),
            GrammarLoweringStep::Attributes {
                rule,
                attributes: fields,
            } => {
                attributes.insert(rule, parse_rule_fields(fields)?);
            }
        }
    }
    let restore = |expr: &GrammarExpr| restore(expr, &helpers, &originals);
    let mut rules = Vec::new();
    for name in &metadata.order {
        let executable_name = renames.get(name.as_str()).copied().unwrap_or(name);
        let Some(back) = imported.rule(executable_name) else {
            return Err(GrammarLoweringError::MissingRule(
                executable_name.to_owned(),
            ));
        };
        let doc = docs
            .get(name.as_str())
            .map(|doc| (*doc).to_owned())
            .or_else(|| back.doc.clone());
        let mut restored = rule(
            name,
            kinds.get(name.as_str()).copied().unwrap_or(back.kind),
            restore(&back.expr),
            doc,
        );
        if let Some(fields) = attributes.remove(name.as_str()) {
            restored.attributes = fields;
        }
        rules.push(restored);
    }
    let mut grammar = assemble(&metadata.start, rules, metadata.source);
    if let Some(text) = declarations {
        grammar.set_declarations(parse_declarations_text(text)?);
    }
    Ok(grammar)
}

/// `expr` with every renamed reference given its original name back and every
/// helper reference replaced with the expression the helper stands for.
fn restore(
    expr: &GrammarExpr,
    helpers: &HashMap<&str, &GrammarExpr>,
    originals: &HashMap<&str, &str>,
) -> GrammarExpr {
    if let GrammarExpr::NonTerminal(name) = expr {
        let name = originals.get(name.as_str()).copied().unwrap_or(name);
        return helpers.get(name).map_or_else(
            || GrammarExpr::NonTerminal(name.to_owned()),
            |original| (*original).clone(),
        );
    }
    map_children::<std::convert::Infallible>(expr, &mut |item| {
        Ok(restore(item, helpers, originals))
    })
    .unwrap_or_else(|never| match never {})
}

/// The way a lowering check failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarLoweringFailureKind {
    /// The emitter noted a construct it could not write.
    LossyEmission,
    /// The executable does not read back as the lowered grammar.
    NotExecutable,
    /// The reconstruction lost a feature of the original grammar.
    FeatureDropped,
    /// An exact lowering rejected an accept sample.
    SampleRejected,
    /// An exact lowering accepted a reject sample.
    SampleAccepted,
}

impl GrammarLoweringFailureKind {
    /// Stable tag shared with the JavaScript runtime.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LossyEmission => "lossy-emission",
            Self::NotExecutable => "not-executable",
            Self::FeatureDropped => "feature-dropped",
            Self::SampleRejected => "sample-rejected",
            Self::SampleAccepted => "sample-accepted",
        }
    }
}

/// One failure of a lowering check.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarLoweringFailure {
    /// The way the check failed.
    pub kind: GrammarLoweringFailureKind,
    /// A readable detail.
    pub detail: String,
}

impl GrammarLoweringFailure {
    fn new(kind: GrammarLoweringFailureKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }
}

/// The outcome of [`check_grammar_lowering`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarLoweringReport {
    /// The lowering status, or broken when any check failed.
    pub status: GrammarLoweringStatus,
    /// Every failed check.
    pub failures: Vec<GrammarLoweringFailure>,
    /// The lowering under check.
    pub lowering: GrammarLowering,
    /// The grammar reconstructed from the package, when it could be.
    pub reconstructed: Option<Grammar>,
}

fn body(expr: &GrammarExpr) -> Result<String, GrammarLoweringError> {
    Ok(canonical_rule_definition(&GrammarRule::new(
        "",
        expr.clone(),
    ))?)
}

fn accepts(grammar: &Grammar, text: &str) -> bool {
    GrammarParser::new(grammar.clone()).accepts(text)
}

/// Lowers `grammar` into `format`, reconstructs it from the package and
/// reports every way the lowering failed.
///
/// The failures are an emission note, an executable
/// that does not read back as the lowered grammar, a feature the
/// reconstruction lost, and for an exact lowering an accept or reject sample
/// on which the executable disagrees.
///
/// # Errors
///
/// Returns a [`GrammarLoweringError`] when the lowering itself cannot be
/// carried out or a rule definition cannot be normalized.
pub fn check_grammar_lowering(
    grammar: &Grammar,
    format: &str,
    options: &GrammarLoweringOptions<'_>,
) -> Result<GrammarLoweringReport, GrammarLoweringError> {
    use GrammarLoweringFailureKind as Kind;
    let lowering = lower_grammar(grammar, format, options)?;
    let mut failures: Vec<GrammarLoweringFailure> = lowering
        .report
        .lossy
        .iter()
        .map(|note| GrammarLoweringFailure::new(Kind::LossyEmission, note.clone()))
        .collect();
    // The executable must read back as the lowered grammar, which already
    // carries the target's rule names: the same rules, start and definitions.
    let expected = lowering.grammar.rule_names();
    let actual = lowering.imported.rule_names();
    if expected != actual {
        failures.push(GrammarLoweringFailure::new(
            Kind::NotExecutable,
            format!(
                "rules [{}] read back as [{}]",
                expected.join(", "),
                actual.join(", ")
            ),
        ));
    }
    let start = lowering
        .grammar
        .start_rule()
        .map_or("", |start| start.name.as_str());
    if lowering
        .imported
        .start_rule()
        .map(|back| back.name.as_str())
        != Some(start)
    {
        failures.push(GrammarLoweringFailure::new(
            Kind::NotExecutable,
            format!("the executable does not start at rule {start}"),
        ));
    }
    for lowered in lowering.grammar.rules() {
        if let Some(back) = lowering.imported.rule(&lowered.name)
            && body(&back.expr)? != body(&lowered.expr)?
        {
            failures.push(GrammarLoweringFailure::new(
                Kind::NotExecutable,
                format!(
                    "rule {} reads back with a different definition",
                    lowered.name
                ),
            ));
        }
    }

    let metadata = options.edit_metadata.map_or_else(
        || lowering.metadata.clone(),
        |edit| edit(&lowering.metadata),
    );
    let reconstructed = match reconstruct_grammar(&lowering.executable, &metadata, options.import) {
        Ok(reconstructed) => Some(reconstructed),
        Err(error) => {
            failures.push(GrammarLoweringFailure::new(
                Kind::FeatureDropped,
                error.to_string(),
            ));
            None
        }
    };
    if let Some(reconstructed) = &reconstructed {
        failures.extend(
            dropped_grammar_features(grammar, reconstructed)?
                .into_iter()
                .map(|detail| GrammarLoweringFailure::new(Kind::FeatureDropped, detail)),
        );
    }
    if lowering.status == GrammarLoweringStatus::Exact {
        for text in options.accepts {
            if !accepts(&lowering.imported, text) {
                failures.push(GrammarLoweringFailure::new(
                    Kind::SampleRejected,
                    text.clone(),
                ));
            }
        }
        for text in options.rejects {
            if accepts(&lowering.imported, text) {
                failures.push(GrammarLoweringFailure::new(
                    Kind::SampleAccepted,
                    text.clone(),
                ));
            }
        }
    }
    Ok(GrammarLoweringReport {
        status: if failures.is_empty() {
            lowering.status
        } else {
            GrammarLoweringStatus::Broken
        },
        failures,
        lowering,
        reconstructed,
    })
}

/// The features of `expected` that `actual` does not carry, as readable
/// details: the rule names and their order, the start rule, and every rule's
/// kind, definition and documentation.
///
/// # Errors
///
/// Returns a [`GrammarLoweringError`] when a rule definition cannot be
/// normalized for comparison.
pub fn dropped_grammar_features(
    expected: &Grammar,
    actual: &Grammar,
) -> Result<Vec<String>, GrammarLoweringError> {
    let mut details = Vec::new();
    if expected.rule_names() != actual.rule_names() {
        details.push(format!(
            "rules [{}] became [{}]",
            expected.rule_names().join(", "),
            actual.rule_names().join(", ")
        ));
    }
    let start = |grammar: &Grammar| grammar.start_rule().map(|start| start.name.clone());
    if start(expected) != start(actual) {
        details.push("the start rule changed".to_owned());
    }
    if render_declarations_text(expected.declarations())
        != render_declarations_text(actual.declarations())
    {
        details.push("the grammar declarations changed".to_owned());
    }
    for original in expected.rules() {
        let Some(other) = actual.rule(&original.name) else {
            continue;
        };
        if other.kind != original.kind {
            details.push(format!(
                "rule {} lost its kind {}",
                original.name,
                original.kind.as_str()
            ));
        }
        if render_rule_fields(&other.attributes) != render_rule_fields(&original.attributes) {
            details.push(format!(
                "rule {} changed its parameters, channel, modes or action",
                original.name
            ));
        }
        if canonical_rule_definition(other)? != canonical_rule_definition(original)? {
            details.push(format!("rule {} changed its definition", original.name));
        }
        if other.doc != original.doc {
            details.push(format!("rule {} changed its documentation", original.name));
        }
    }
    Ok(details)
}
