use std::collections::HashMap;

use crate::configuration::RegionDetectionPolicy;
use crate::language_catalog::canonical_language_name;
use crate::line_index::LineIndex;
use crate::link_network::{LinkId, LinkNetwork, LinkType};
use crate::source::{ByteRange, SourceSpan};

const TXT_LANGUAGE: &str = "txt";

/// Embedded region discovered inside a mixed-language document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbeddedRegion {
    language: String,
    span: SourceSpan,
}

impl EmbeddedRegion {
    pub(crate) const fn new(language: String, span: SourceSpan) -> Self {
        Self { language, span }
    }

    /// Language detected for the embedded region.
    #[must_use]
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Source span covered by the embedded region.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Embedded regions a host document delimits, in source order: the whole
/// buffer of a `txt` document, and for HTML and Markdown hosts the regions
/// their grammar CST (the host's `Syntax` links below `document`) delimits.
/// Mirrors `detectEmbeddedRegionsInTree` in `js/src/regions.js`.
///
/// - HTML: the `raw_text` of a `script_element` in the language its `type`
///   attribute names (JavaScript when absent), the `raw_text` of a
///   `style_element` as CSS, and the `attribute_value` of every `style`
///   attribute as CSS.
/// - Markdown: the `code_fence_content` of a `fenced_code_block` in the
///   language `policy` selects from its info-string `language` and content,
///   every `html_block` as HTML, and every inline `html_tag` as HTML, spanning
///   from an opening tag to the matching closing tag among its siblings.
pub(crate) fn detect_embedded_regions(
    network: &LinkNetwork,
    document: LinkId,
    text: &str,
    language: &str,
    policy: RegionDetectionPolicy,
) -> Vec<EmbeddedRegion> {
    let lines = LineIndex::new(text);
    let host = canonical_language_name(language);
    if host == Some(TXT_LANGUAGE) {
        return vec![region_for(&lines, TXT_LANGUAGE.to_string(), 0, text.len())];
    }
    if !matches!(host, Some("HTML" | "Markdown")) {
        return Vec::new();
    }
    let tree = HostTree::new(network, text);
    let mut found = Vec::new();
    let mut stack = vec![document];
    while let Some(node) = stack.pop() {
        if host == Some("HTML") {
            tree.visit_html(node, &mut found);
        } else {
            tree.visit_markdown(node, policy, &mut found);
        }
        stack.extend(tree.children(node).iter().rev());
    }
    found.sort_by_key(|(_, start, _)| *start);
    found
        .into_iter()
        .map(|(language, start, end)| region_for(&lines, language, start, end))
        .collect()
}

type FoundRegion = (String, usize, usize);

/// The host grammar CST of a network: `Syntax` links in child order below
/// their parent.
struct HostTree<'a> {
    network: &'a LinkNetwork,
    text: &'a str,
    children: HashMap<LinkId, Vec<LinkId>>,
}

impl<'a> HostTree<'a> {
    fn new(network: &'a LinkNetwork, text: &'a str) -> Self {
        let mut children: HashMap<LinkId, Vec<LinkId>> = HashMap::new();
        for link in network.links() {
            if link.metadata().link_type() == Some(LinkType::Syntax) {
                if let [parent] = link.references() {
                    children.entry(*parent).or_default().push(link.id());
                }
            }
        }
        Self {
            network,
            text,
            children,
        }
    }

    fn children(&self, node: LinkId) -> &[LinkId] {
        self.children.get(&node).map_or(&[], Vec::as_slice)
    }

    fn kind(&self, node: LinkId) -> Option<&str> {
        self.network.link(node)?.metadata().term()
    }

    fn range(&self, node: LinkId) -> (usize, usize) {
        self.network
            .link(node)
            .and_then(|link| link.metadata().span())
            .map_or((0, 0), |span| {
                (span.byte_range().start(), span.byte_range().end())
            })
    }

    fn text(&self, node: LinkId) -> &'a str {
        let (start, end) = self.range(node);
        &self.text[start..end]
    }

    fn child_of_kind(&self, node: LinkId, kind: &str) -> Option<LinkId> {
        self.children(node)
            .iter()
            .copied()
            .find(|child| self.kind(*child) == Some(kind))
    }

    fn visit_html(&self, node: LinkId, found: &mut Vec<FoundRegion>) {
        let content = self.child_of_kind(node, "raw_text");
        match (self.kind(node), content) {
            (Some("script_element"), Some(content)) => {
                let type_value = self.attribute_value(node, "type");
                if let Some(language) = script_language(type_value.unwrap_or_default()) {
                    let (start, end) = self.range(content);
                    found.push((language.to_string(), start, end));
                }
            }
            (Some("style_element"), Some(content)) => {
                let essence = mime_essence(self.attribute_value(node, "type").unwrap_or_default());
                if essence.is_empty() || essence == "text/css" {
                    let (start, end) = self.range(content);
                    found.push(("CSS".to_string(), start, end));
                }
            }
            (Some("attribute"), _) if self.attribute_name(node).as_deref() == Some("style") => {
                if let Some(value) = self.attribute_value_node(node) {
                    let (start, end) = self.range(value);
                    found.push(("CSS".to_string(), start, end));
                }
            }
            _ => {}
        }
    }

    fn visit_markdown(
        &self,
        node: LinkId,
        policy: RegionDetectionPolicy,
        found: &mut Vec<FoundRegion>,
    ) {
        match self.kind(node) {
            Some("fenced_code_block") => {
                if let Some(content) = self.child_of_kind(node, "code_fence_content") {
                    let tag = self
                        .child_of_kind(node, "info_string")
                        .and_then(|info| self.child_of_kind(info, "language"))
                        .map_or("", |tag| self.text(tag));
                    if let Some(language) = fence_language(tag, self.text(content), policy) {
                        let (start, end) = self.range(content);
                        found.push((language, start, end));
                    }
                }
            }
            Some("html_block") => {
                let (start, end) = self.range(node);
                found.push(("HTML".to_string(), start, end));
            }
            _ => {}
        }
        let tags: Vec<LinkId> = self
            .children(node)
            .iter()
            .copied()
            .filter(|child| self.kind(*child) == Some("html_tag"))
            .collect();
        let texts: Vec<&str> = tags.iter().map(|tag| self.text(*tag)).collect();
        for (first, last) in pair_html_tags(&texts) {
            found.push((
                "HTML".to_string(),
                self.range(tags[first]).0,
                self.range(tags[last]).1,
            ));
        }
    }

    fn attribute_name(&self, attribute: LinkId) -> Option<String> {
        self.child_of_kind(attribute, "attribute_name")
            .map(|name| self.text(name).to_ascii_lowercase())
    }

    fn attribute_value_node(&self, attribute: LinkId) -> Option<LinkId> {
        self.child_of_kind(attribute, "attribute_value")
            .or_else(|| {
                self.child_of_kind(attribute, "quoted_attribute_value")
                    .and_then(|quoted| self.child_of_kind(quoted, "attribute_value"))
            })
    }

    /// The value of an element's start-tag attribute, `""` when valueless.
    fn attribute_value(&self, element: LinkId, name: &str) -> Option<&'a str> {
        let start_tag = self.child_of_kind(element, "start_tag")?;
        let attribute = self.children(start_tag).iter().copied().find(|child| {
            self.kind(*child) == Some("attribute")
                && self.attribute_name(*child).as_deref() == Some(name)
        })?;
        Some(
            self.attribute_value_node(attribute)
                .map_or("", |value| self.text(value)),
        )
    }
}

/// Void elements never have a closing tag.
const HTML_VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

struct InlineTag {
    name: Option<String>,
    opening: bool,
    closing: bool,
}

/// Groups sibling inline HTML tags into elements: an opening tag extends to
/// its matching closing tag (same name, nesting counted); a void,
/// self-closing, unmatched, comment, or declaration tag stands alone.
fn pair_html_tags(texts: &[&str]) -> Vec<(usize, usize)> {
    let tags: Vec<InlineTag> = texts
        .iter()
        .map(|text| {
            let rest = &text[1.min(text.len())..];
            let closing = rest.starts_with('/');
            let rest = rest.strip_prefix('/').unwrap_or(rest);
            let name: String = rest
                .chars()
                .take_while(|character| character.is_ascii_alphanumeric() || *character == '-')
                .collect();
            if !text.starts_with('<') || !name.starts_with(|c: char| c.is_ascii_alphabetic()) {
                return InlineTag {
                    name: None,
                    opening: false,
                    closing: false,
                };
            }
            let name = name.to_ascii_lowercase();
            let opening =
                !closing && !text.ends_with("/>") && !HTML_VOID_ELEMENTS.contains(&name.as_str());
            InlineTag {
                name: Some(name),
                opening,
                closing,
            }
        })
        .collect();
    let mut groups = Vec::new();
    let mut index = 0;
    while index < tags.len() {
        let mut last = index;
        if tags[index].opening {
            let mut depth = 0_usize;
            for (next, tag) in tags.iter().enumerate().skip(index) {
                if tag.name != tags[index].name {
                    continue;
                }
                if tag.opening {
                    depth += 1;
                }
                if tag.closing {
                    depth -= 1;
                }
                if depth == 0 {
                    last = next;
                    break;
                }
            }
        }
        groups.push((index, last));
        index = last + 1;
    }
    groups
}

/// <https://html.spec.whatwg.org/multipage/scripting.html#javascript-mime-type>
const JAVASCRIPT_SCRIPT_TYPES: &[&str] = &[
    "",
    "module",
    "application/ecmascript",
    "application/javascript",
    "application/x-ecmascript",
    "application/x-javascript",
    "text/ecmascript",
    "text/javascript",
    "text/javascript1.0",
    "text/javascript1.1",
    "text/javascript1.2",
    "text/javascript1.3",
    "text/javascript1.4",
    "text/javascript1.5",
    "text/jscript",
    "text/livescript",
    "text/x-ecmascript",
    "text/x-javascript",
];

/// The language of a script element's content from its `type` attribute.
///
/// JavaScript for classic scripts and modules, JSON for import maps,
/// speculation rules and JSON types, otherwise the registered language the
/// MIME subtype names (such as `text/typescript`); `None` for other data
/// blocks.
#[must_use]
pub fn script_language(type_attribute: &str) -> Option<&'static str> {
    let essence = mime_essence(type_attribute);
    if JAVASCRIPT_SCRIPT_TYPES.contains(&essence.as_str()) {
        return Some("JavaScript");
    }
    if essence == "importmap"
        || essence == "speculationrules"
        || essence.ends_with("/json")
        || essence.ends_with("+json")
    {
        return Some("JSON");
    }
    let (_, subtype) = essence.split_once('/')?;
    canonical_language_name(subtype.strip_prefix("x-").unwrap_or(subtype))
}

fn mime_essence(type_attribute: &str) -> String {
    type_attribute
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
}

fn fence_language(tag: &str, content: &str, policy: RegionDetectionPolicy) -> Option<String> {
    let named = (!tag.is_empty()).then(|| canonical_language_name(tag).unwrap_or(tag).to_string());
    let sniffed = || {
        let language = sniff_language(content).unwrap_or(TXT_LANGUAGE);
        canonical_language_name(language)
            .unwrap_or(language)
            .to_string()
    };
    match policy {
        RegionDetectionPolicy::NameDriven => named,
        RegionDetectionPolicy::ContentDriven => Some(sniffed()),
        RegionDetectionPolicy::Both => Some(named.unwrap_or_else(sniffed)),
    }
}

fn sniff_language(content: &str) -> Option<&'static str> {
    let trimmed = content.trim_start();
    let upper = trimmed.to_ascii_uppercase();

    if trimmed.contains("fn main") {
        Some("rust")
    } else if trimmed.starts_with("def ") {
        Some("Python")
    } else if trimmed.starts_with('<') {
        Some("HTML")
    } else if trimmed.contains("function ")
        || trimmed.contains("const ")
        || trimmed.contains("let ")
    {
        Some("JavaScript")
    } else if upper.starts_with("SELECT ") {
        Some("sql-ansi")
    } else {
        None
    }
}

fn region_for(lines: &LineIndex, language: String, start: usize, end: usize) -> EmbeddedRegion {
    EmbeddedRegion::new(
        language,
        SourceSpan::new(
            ByteRange::new(start, end),
            lines.byte_point(start),
            lines.byte_point(end),
        ),
    )
}
