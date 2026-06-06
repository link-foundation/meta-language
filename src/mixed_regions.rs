use crate::configuration::RegionDetectionPolicy;
use crate::source::{ByteRange, Point, SourceSpan};

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

pub(crate) fn detect_embedded_regions(
    text: &str,
    language: &str,
    policy: RegionDetectionPolicy,
) -> Vec<EmbeddedRegion> {
    let mut regions = Vec::new();
    match language.to_ascii_lowercase().as_str() {
        TXT_LANGUAGE => regions.push(region_for(text, TXT_LANGUAGE.to_string(), 0, text.len())),
        "markdown" => {
            regions.extend(detect_markdown_fenced_regions(text, policy));
            regions.extend(detect_markdown_html_regions(text));
        }
        "html" => {
            regions.extend(detect_html_element_regions(text, "script", "JavaScript"));
            regions.extend(detect_html_element_regions(text, "style", "CSS"));
            regions.extend(detect_html_style_attributes(text));
        }
        _ => {}
    }
    regions
}

fn detect_markdown_fenced_regions(
    text: &str,
    policy: RegionDetectionPolicy,
) -> Vec<EmbeddedRegion> {
    let mut regions = Vec::new();
    let mut offset = 0;
    let mut open_fence: Option<(String, usize)> = None;

    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\r', '\n']).trim_start();
        if let Some((language_tag, content_start)) = open_fence.take() {
            if trimmed.starts_with("```") {
                if let Some(language) = region_language_from_tag_or_content(
                    &language_tag,
                    &text[content_start..offset],
                    policy,
                ) {
                    regions.push(region_for(text, language, content_start, offset));
                }
            } else {
                open_fence = Some((language_tag, content_start));
            }
        } else if let Some(rest) = trimmed.strip_prefix("```") {
            let language_tag = rest
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_string();
            open_fence = Some((language_tag, offset + line.len()));
        }
        offset += line.len();
    }

    if let Some((language_tag, content_start)) = open_fence {
        if let Some(language) =
            region_language_from_tag_or_content(&language_tag, &text[content_start..], policy)
        {
            regions.push(region_for(text, language, content_start, text.len()));
        }
    }

    regions
}

fn region_language_from_tag_or_content(
    language_tag: &str,
    content: &str,
    policy: RegionDetectionPolicy,
) -> Option<String> {
    match policy {
        RegionDetectionPolicy::NameDriven => {
            (!language_tag.is_empty()).then(|| language_tag.to_string())
        }
        RegionDetectionPolicy::ContentDriven => {
            Some(sniff_language(content).unwrap_or(TXT_LANGUAGE).to_string())
        }
        RegionDetectionPolicy::Both => {
            if language_tag.is_empty() {
                Some(sniff_language(content).unwrap_or(TXT_LANGUAGE).to_string())
            } else {
                Some(language_tag.to_string())
            }
        }
    }
}

fn detect_markdown_html_regions(text: &str) -> Vec<EmbeddedRegion> {
    let mut regions = Vec::new();
    let mut search_start = 0;

    while let Some(relative_start) = text[search_start..].find('<') {
        let start = search_start + relative_start;
        let Some(next) = text[start + 1..].chars().next() else {
            break;
        };
        if !next.is_ascii_alphabetic() {
            search_start = start + 1;
            continue;
        }

        let Some(close) = text[start..].find('>') else {
            break;
        };
        let first_tag_end = start + close + 1;
        let tag_name = text[start + 1..first_tag_end - 1]
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .trim_matches('/')
            .to_ascii_lowercase();
        if tag_name.is_empty() {
            search_start = first_tag_end;
            continue;
        }

        let closing_tag = format!("</{tag_name}>");
        let region_end = text[first_tag_end..]
            .to_ascii_lowercase()
            .find(&closing_tag)
            .map_or(first_tag_end, |relative_end| {
                first_tag_end + relative_end + closing_tag.len()
            });
        regions.push(region_for(text, "HTML".to_string(), start, region_end));
        search_start = region_end;
    }

    regions
}

fn detect_html_element_regions(text: &str, element: &str, language: &str) -> Vec<EmbeddedRegion> {
    let mut regions = Vec::new();
    let lower = text.to_ascii_lowercase();
    let open = format!("<{element}");
    let close = format!("</{element}>");
    let mut search_start = 0;

    while let Some(relative_start) = lower[search_start..].find(&open) {
        let start = search_start + relative_start;
        let Some(open_end_relative) = lower[start..].find('>') else {
            break;
        };
        let content_start = start + open_end_relative + 1;
        let Some(close_relative) = lower[content_start..].find(&close) else {
            break;
        };
        let content_end = content_start + close_relative;
        regions.push(region_for(
            text,
            language.to_string(),
            content_start,
            content_end,
        ));
        search_start = content_end + close.len();
    }

    regions
}

fn detect_html_style_attributes(text: &str) -> Vec<EmbeddedRegion> {
    let mut regions = Vec::new();
    let lower = text.to_ascii_lowercase();
    let mut search_start = 0;

    while let Some(relative_start) = lower[search_start..].find("style=\"") {
        let value_start = search_start + relative_start + "style=\"".len();
        let Some(value_end_relative) = text[value_start..].find('"') else {
            break;
        };
        let value_end = value_start + value_end_relative;
        regions.push(region_for(text, "CSS".to_string(), value_start, value_end));
        search_start = value_end + 1;
    }

    regions
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
        Some("SQL")
    } else {
        None
    }
}

fn region_for(text: &str, language: String, start: usize, end: usize) -> EmbeddedRegion {
    EmbeddedRegion::new(
        language,
        SourceSpan::new(
            ByteRange::new(start, end),
            point_at_byte(text, start),
            point_at_byte(text, end),
        ),
    )
}

fn point_at_byte(text: &str, byte: usize) -> Point {
    let mut row = 0;
    let mut column = 0;

    for (index, character) in text.char_indices() {
        if index >= byte {
            break;
        }
        if character == '\n' {
            row += 1;
            column = 0;
        } else {
            column += 1;
        }
    }

    Point::new(row, column)
}
