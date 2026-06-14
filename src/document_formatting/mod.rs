//! Shared, language-free document-formatting concept ontology.
//!
//! Documents in different container formats (Markdown, HTML, and — through the
//! issues that build on this substrate — PDF and DOCX) express the *same*
//! formatting concepts with different surface syntax. A Markdown `**bold**`, an
//! HTML `<strong>bold</strong>`, and a DOCX run with the `<w:b/>` property all
//! denote one language-free `strong` concept.
//!
//! This module seeds that concept set into a [`LinkNetwork`] with per-format
//! syntax mappings, and provides data-driven resolution and reconstruction so
//! the *same* concept link round-trips across formats. The per-format mapping
//! is stored as a small template string whose `{}` placeholder marks the
//! formatted content and whose `{name}` placeholders mark named attributes
//! (`{href}`, `{src}`, `{lang}`) or the heading level (`{markers}` for the
//! Markdown `#` run, `{level}` for the HTML digit).

mod document;

pub use document::{parse_markup_document, BlockNode, FormattingDocument, InlineNode};

use std::collections::BTreeMap;

use crate::link_network::{LinkId, LinkNetwork, LinkType};

/// A single document-formatting concept with its per-format templates.
struct FormattingConcept {
    id: &'static str,
    definition: &'static str,
    /// `(language, template)` pairs. The template uses `{}` for content and
    /// `{name}` for named holes; see the module documentation.
    templates: &'static [(&'static str, &'static str)],
}

/// The shared document-formatting concept set required by issue #83.
const DOCUMENT_FORMATTING_CONCEPTS: &[FormattingConcept] = &[
    // --- inline concepts ---
    FormattingConcept {
        id: "emphasis",
        definition: "Inline emphasis (italic) applied to a text fragment.",
        templates: &[("Markdown", "*{}*"), ("HTML", "<em>{}</em>")],
    },
    FormattingConcept {
        id: "strong",
        definition: "Inline strong importance (bold) applied to a text fragment.",
        templates: &[("Markdown", "**{}**"), ("HTML", "<strong>{}</strong>")],
    },
    FormattingConcept {
        id: "strikethrough",
        definition: "Inline strikethrough (deleted) text fragment.",
        templates: &[("Markdown", "~~{}~~"), ("HTML", "<del>{}</del>")],
    },
    FormattingConcept {
        id: "inline-code",
        definition: "Inline monospaced code span.",
        templates: &[("Markdown", "`{}`"), ("HTML", "<code>{}</code>")],
    },
    FormattingConcept {
        id: "hyperlink",
        definition: "Inline hyperlink wrapping text and targeting a destination.",
        templates: &[
            ("Markdown", "[{}]({href})"),
            ("HTML", "<a href=\"{href}\">{}</a>"),
        ],
    },
    FormattingConcept {
        id: "image",
        definition: "Inline image with alternative text and a source reference.",
        templates: &[
            ("Markdown", "![{}]({src})"),
            ("HTML", "<img src=\"{src}\" alt=\"{}\" />"),
        ],
    },
    FormattingConcept {
        id: "line-break",
        definition: "Explicit inline line break inside a block.",
        templates: &[("Markdown", "  \n"), ("HTML", "<br />")],
    },
    // --- block concepts ---
    FormattingConcept {
        id: "heading",
        definition: "Section heading carrying a level from 1 (most significant) downward.",
        templates: &[
            ("Markdown", "{markers} {}"),
            ("HTML", "<h{level}>{}</h{level}>"),
        ],
    },
    FormattingConcept {
        id: "paragraph",
        definition: "Block of running text.",
        templates: &[("Markdown", "{}"), ("HTML", "<p>{}</p>")],
    },
    FormattingConcept {
        id: "blockquote",
        definition: "Quoted block set off from the surrounding text.",
        templates: &[
            ("Markdown", "> {}"),
            ("HTML", "<blockquote>{}</blockquote>"),
        ],
    },
    FormattingConcept {
        id: "bullet-list",
        definition: "Unordered list container.",
        templates: &[("Markdown", "{}"), ("HTML", "<ul>{}</ul>")],
    },
    FormattingConcept {
        id: "ordered-list",
        definition: "Ordered list container.",
        templates: &[("Markdown", "{}"), ("HTML", "<ol>{}</ol>")],
    },
    FormattingConcept {
        id: "list-item",
        definition: "Single item within a list.",
        templates: &[("Markdown", "- {}"), ("HTML", "<li>{}</li>")],
    },
    FormattingConcept {
        id: "code-block",
        definition: "Fenced block of preformatted code carrying an optional language.",
        templates: &[
            ("Markdown", "```{lang}\n{}\n```"),
            (
                "HTML",
                "<pre><code class=\"language-{lang}\">{}</code></pre>",
            ),
        ],
    },
    FormattingConcept {
        id: "thematic-break",
        definition: "Thematic break (horizontal rule) between sections.",
        templates: &[("Markdown", "---"), ("HTML", "<hr />")],
    },
    FormattingConcept {
        id: "table",
        definition: "Tabular data container.",
        templates: &[("Markdown", "{}"), ("HTML", "<table>{}</table>")],
    },
    FormattingConcept {
        id: "table-row",
        definition: "Row within a table.",
        templates: &[("Markdown", "{}"), ("HTML", "<tr>{}</tr>")],
    },
    FormattingConcept {
        id: "table-cell",
        definition: "Cell within a table row.",
        templates: &[("Markdown", "{}"), ("HTML", "<td>{}</td>")],
    },
];

/// Summary returned after seeding the document-formatting concept set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DocumentFormattingSeedReport {
    concepts: usize,
    syntax_mappings: usize,
}

impl DocumentFormattingSeedReport {
    /// Number of language-free formatting concepts seeded.
    #[must_use]
    pub const fn concepts(self) -> usize {
        self.concepts
    }

    /// Number of per-format syntax mappings attached to the formatting concepts.
    #[must_use]
    pub const fn syntax_mappings(self) -> usize {
        self.syntax_mappings
    }
}

/// A formatting fragment resolved to its language-free concept.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentFormatMatch {
    /// Exact concept id (for example `strong`).
    pub concept: String,
    /// Concept link in the network the fragment maps onto.
    pub link: LinkId,
    /// Formatted content captured from the `{}` placeholder.
    pub content: String,
    /// Heading level, when the concept carries one.
    pub level: Option<u8>,
    /// Named attribute captures (`href`, `src`, `lang`).
    pub attributes: BTreeMap<String, String>,
}

/// A concept instance ready to be rendered into a target format.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentFormatInstance {
    /// Formatted content for the `{}` placeholder.
    pub content: String,
    /// Heading level, when the concept carries one.
    pub level: Option<u8>,
    /// Named attribute values (`href`, `src`, `lang`).
    pub attributes: BTreeMap<String, String>,
}

impl DocumentFormatInstance {
    /// Creates an instance carrying only formatted content.
    #[must_use]
    pub fn from_content(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            level: None,
            attributes: BTreeMap::new(),
        }
    }

    fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(String::as_str)
    }
}

/// A single segment of a compiled template.
enum Segment {
    Literal(String),
    Hole(Hole),
}

/// The kind of value a template hole carries.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Hole {
    /// Formatted content (`{}`).
    Content,
    /// Markdown heading markers rendered as a run of `#`.
    Markers,
    /// HTML heading level rendered as a decimal digit.
    Level,
    /// Named attribute hole such as `href`, `src`, or `lang`.
    Attribute(&'static str),
}

fn hole_for_name(name: &str) -> Hole {
    match name {
        "" => Hole::Content,
        "markers" => Hole::Markers,
        "level" => Hole::Level,
        "href" => Hole::Attribute("href"),
        "src" => Hole::Attribute("src"),
        "lang" => Hole::Attribute("lang"),
        other => panic!("unsupported document-formatting template hole {{{other}}}"),
    }
}

/// Compiles a template string into ordered literal and hole segments.
fn compile_template(template: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut literal = String::new();
    let mut chars = template.chars();

    while let Some(character) = chars.next() {
        if character == '{' {
            if !literal.is_empty() {
                segments.push(Segment::Literal(std::mem::take(&mut literal)));
            }
            let mut name = String::new();
            for inner in chars.by_ref() {
                if inner == '}' {
                    break;
                }
                name.push(inner);
            }
            segments.push(Segment::Hole(hole_for_name(&name)));
        } else {
            literal.push(character);
        }
    }

    if !literal.is_empty() {
        segments.push(Segment::Literal(literal));
    }

    segments
}

/// Renders a compiled template from an instance, or `None` when a required
/// attribute is missing.
fn render_segments(segments: &[Segment], instance: &DocumentFormatInstance) -> Option<String> {
    let mut output = String::new();
    for segment in segments {
        match segment {
            Segment::Literal(text) => output.push_str(text),
            Segment::Hole(Hole::Content) => output.push_str(&instance.content),
            Segment::Hole(Hole::Markers) => {
                let level = instance.level.unwrap_or(1).max(1);
                output.push_str(&"#".repeat(usize::from(level)));
            }
            Segment::Hole(Hole::Level) => {
                output.push_str(&instance.level.unwrap_or(1).max(1).to_string());
            }
            Segment::Hole(Hole::Attribute(name)) => output.push_str(instance.attribute(name)?),
        }
    }
    Some(output)
}

/// Matches a fragment against a compiled template, capturing hole values.
fn match_segments(segments: &[Segment], fragment: &str) -> Option<DocumentFormatInstance> {
    let mut instance = DocumentFormatInstance::default();
    let mut cursor = 0usize;

    for (index, segment) in segments.iter().enumerate() {
        match segment {
            Segment::Literal(text) => {
                if !fragment[cursor..].starts_with(text.as_str()) {
                    return None;
                }
                cursor += text.len();
            }
            Segment::Hole(hole) => {
                let rest = &fragment[cursor..];
                let captured = match segments.get(index + 1) {
                    Some(Segment::Literal(next)) => {
                        let relative = rest.find(next.as_str())?;
                        &rest[..relative]
                    }
                    // Two holes cannot sit next to each other in our templates.
                    Some(Segment::Hole(_)) => return None,
                    None => rest,
                };
                store_capture(&mut instance, *hole, captured)?;
                cursor += captured.len();
            }
        }
    }

    if cursor == fragment.len() {
        Some(instance)
    } else {
        None
    }
}

fn store_capture(instance: &mut DocumentFormatInstance, hole: Hole, captured: &str) -> Option<()> {
    match hole {
        Hole::Content => instance.content = captured.to_string(),
        Hole::Markers => {
            if captured.is_empty() || !captured.bytes().all(|byte| byte == b'#') {
                return None;
            }
            let level = u8::try_from(captured.len()).ok()?;
            assign_level(instance, level)?;
        }
        Hole::Level => {
            let level: u8 = captured.parse().ok()?;
            assign_level(instance, level)?;
        }
        Hole::Attribute(name) => {
            instance
                .attributes
                .insert(name.to_string(), captured.to_string());
        }
    }
    Some(())
}

/// Stores a heading level, rejecting a fragment whose repeated level holes
/// disagree (for example an HTML `<h2>...</h1>`).
fn assign_level(instance: &mut DocumentFormatInstance, level: u8) -> Option<()> {
    match instance.level {
        Some(existing) if existing != level => None,
        _ => {
            instance.level = Some(level);
            Some(())
        }
    }
}

fn template_for(concept: &str, language: &str) -> Option<&'static str> {
    DOCUMENT_FORMATTING_CONCEPTS
        .iter()
        .find(|entry| entry.id == concept)?
        .templates
        .iter()
        .find(|(lang, _)| *lang == language)
        .map(|(_, template)| *template)
}

/// Inline concepts are resolved most-specific-first so that, for example, an
/// image is not mistaken for a hyperlink and bold is not mistaken for italic.
const INLINE_RESOLUTION_ORDER: &[&str] = &[
    "image",
    "hyperlink",
    "strong",
    "emphasis",
    "strikethrough",
    "inline-code",
];

impl LinkNetwork {
    /// Seeds the shared document-formatting concept set with per-format syntax
    /// mappings.
    ///
    /// Each concept becomes a language-free [`LinkType::Concept`] link, and each
    /// `(language, template)` pair becomes a semantic syntax mapping so the same
    /// concept reconstructs as `**…**` in Markdown, `<strong>…</strong>` in
    /// HTML, and so on.
    pub fn seed_document_formatting_concepts(&mut self) -> DocumentFormattingSeedReport {
        let mut syntax_mappings = 0;
        for concept in DOCUMENT_FORMATTING_CONCEPTS {
            let concept_link = self.intern_concept(concept.id, Some(concept.definition));
            for (language, template) in concept.templates {
                self.insert_concept_syntax_mapping(
                    concept_link,
                    concept.id,
                    language,
                    template,
                    true,
                );
                syntax_mappings += 1;
            }
        }

        DocumentFormattingSeedReport {
            concepts: DOCUMENT_FORMATTING_CONCEPTS.len(),
            syntax_mappings,
        }
    }

    /// Resolves a formatting `fragment` written in `language` to the shared,
    /// language-free concept it denotes.
    ///
    /// Both Markdown `**bold**` and HTML `<strong>bold</strong>` resolve to the
    /// one seeded `strong` concept link. Returns `None` when the fragment is not
    /// a known formatting construct or the concept set has not been seeded.
    #[must_use]
    pub fn resolve_document_format(
        &self,
        language: &str,
        fragment: &str,
    ) -> Option<DocumentFormatMatch> {
        for concept in INLINE_RESOLUTION_ORDER
            .iter()
            .copied()
            .chain(DOCUMENT_FORMATTING_CONCEPTS.iter().map(|entry| entry.id))
        {
            let Some(template) = template_for(concept, language) else {
                continue;
            };
            let segments = compile_template(template);
            if let Some(instance) = match_segments(&segments, fragment) {
                let Some(link) = self.find_term(concept) else {
                    continue;
                };
                return Some(DocumentFormatMatch {
                    concept: concept.to_string(),
                    link,
                    content: instance.content,
                    level: instance.level,
                    attributes: instance.attributes,
                });
            }
        }
        None
    }

    /// Renders a concept instance into `language` surface syntax.
    ///
    /// Returns `None` when the concept has no template for the language or a
    /// required attribute is missing from the instance.
    #[must_use]
    pub fn render_document_format(
        &self,
        concept: &str,
        language: &str,
        instance: &DocumentFormatInstance,
    ) -> Option<String> {
        let template = template_for(concept, language)?;
        render_segments(&compile_template(template), instance)
    }

    /// Translates a single formatting fragment from `source_language` to
    /// `target_language` through the shared concept layer.
    #[must_use]
    pub fn translate_document_format(
        &self,
        source_language: &str,
        target_language: &str,
        fragment: &str,
    ) -> Option<String> {
        let resolved = self.resolve_document_format(source_language, fragment)?;
        let instance = DocumentFormatInstance {
            content: resolved.content,
            level: resolved.level,
            attributes: resolved.attributes,
        };
        self.render_document_format(&resolved.concept, target_language, &instance)
    }

    /// Returns the seeded concept link for a formatting concept id, when present.
    #[must_use]
    pub fn document_formatting_concept(&self, concept: &str) -> Option<LinkId> {
        let _ = template_for(concept, "Markdown")?;
        self.find_term(concept)
            .filter(|link| self.is_concept_link(*link))
    }

    fn is_concept_link(&self, link: LinkId) -> bool {
        self.link(link)
            .is_some_and(|link| link.metadata().link_type() == Some(LinkType::Concept))
    }
}
