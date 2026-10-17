use std::collections::{HashMap, HashSet};

use crate::configuration::{ParseConfiguration, embedded_region_number as number};
use crate::language_catalog::canonical_language_name;
use crate::link_network::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::mixed_regions::{EmbeddedRegion, HostTreeIndex, detect_embedded_regions_in_scope};
use crate::translation::embedded_region_rules as rules;
use crate::{ByteRange, LinkFlags, SourceSpan, structured_text_parser, tree_sitter_adapter};

struct EmbeddedRegionFrame {
    parent: LinkId,
    language: String,
    bounds: ByteRange,
    depth: f64,
}

struct EmbeddedRegionTraversal {
    frames: Vec<EmbeddedRegionFrame>,
    parents: Vec<f64>,
    languages: Vec<f64>,
    starts: Vec<f64>,
    ends: Vec<f64>,
    language_identifiers: HashMap<String, f64>,
    recorded_languages: HashSet<LinkId>,
    count: f64,
    bytes: f64,
    limits: [f64; 3],
    index: HostTreeIndex,
}

impl EmbeddedRegionTraversal {
    fn new(
        document: LinkId,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> Self {
        Self {
            frames: vec![EmbeddedRegionFrame {
                parent: document,
                language: language.to_owned(),
                bounds: ByteRange::new(0, text.len()),
                depth: 0.0,
            }],
            parents: vec![-1.0],
            languages: vec![0.0],
            starts: vec![0.0],
            ends: vec![number(text.len())],
            language_identifiers: HashMap::from([(
                canonical_language_name(language)
                    .unwrap_or(language)
                    .to_owned(),
                0.0,
            )]),
            recorded_languages: HashSet::new(),
            count: 0.0,
            bytes: 0.0,
            limits: [
                number(configuration.embedded_region_depth_limit()),
                number(configuration.embedded_region_count_limit()),
                number(configuration.embedded_region_byte_limit()),
            ],
            index: HostTreeIndex::new(text),
        }
    }

    fn run(&mut self, network: &mut LinkNetwork, text: &str, configuration: ParseConfiguration) {
        let mut position = 0;
        while position < self.frames.len() {
            let frame = &self.frames[position];
            let regions = detect_embedded_regions_in_scope(
                network,
                &mut self.index,
                frame.parent,
                text,
                &frame.language,
                configuration.region_detection_policy(),
                frame.bounds,
            );
            for region in regions {
                self.attach(network, text, configuration, position, &region);
            }
            position += 1;
        }
    }

    fn attach(
        &mut self,
        network: &mut LinkNetwork,
        text: &str,
        configuration: ParseConfiguration,
        position: usize,
        region: &EmbeddedRegion,
    ) {
        let language = region.language();
        let canonical = canonical_language_name(language).unwrap_or(language);
        let next_language = number(self.language_identifiers.len());
        let selected_language = *self
            .language_identifiers
            .entry(canonical.to_owned())
            .or_insert(next_language);
        let span = region.span();
        let bounds = span.byte_range();
        let start = number(bounds.start());
        let end = number(bounds.end());
        let frame = &self.frames[position];
        let parent = frame.parent;
        let depth = frame.depth + 1.0;
        let ancestor = rules::read_embedded_region_ancestor(
            &self.parents,
            &self.languages,
            &self.starts,
            &self.ends,
            number(position),
            selected_language,
            start,
            end,
        );
        let disposition = rules::read_embedded_region_disposition(
            self.starts[position],
            self.ends[position],
            self.languages[position],
            selected_language,
            start,
            end,
            ancestor,
            depth,
            self.count,
            self.bytes,
            self.limits[0],
            self.limits[1],
            self.limits[2],
            language.eq_ignore_ascii_case(&frame.language),
        );
        if disposition == "reuseAttached" {
            return;
        }
        let language_link = network.insert_typed_point(language, LinkType::Language, None);
        let region_link = network.insert_link(
            [parent, language_link],
            LinkMetadata::new()
                .with_link_type(LinkType::Region)
                .with_named(true)
                .with_term(format!("{language} region"))
                .with_language(language)
                .with_span(span),
        );
        if disposition == "reuse" {
            return;
        }
        if disposition != "parse" {
            let ancestor_parent = rules::ml_array::number_index(ancestor)
                .and_then(|position| self.frames.get(position))
                .map(|frame| frame.parent);
            insert_embedded_region_diagnostic(
                network,
                region_link,
                language,
                span,
                &EmbeddedRegionDiagnostic {
                    reason: &disposition,
                    depth,
                    count: self.count,
                    bytes: self.bytes,
                    observed: rules::read_embedded_region_diagnostic_observed(
                        &disposition,
                        depth,
                        self.count,
                        self.bytes,
                        start,
                        end,
                    ),
                    limit: rules::read_embedded_region_diagnostic_limit(
                        &disposition,
                        self.limits[0],
                        self.limits[1],
                        self.limits[2],
                    ),
                    ancestor: ancestor_parent,
                },
            );
            return;
        }
        self.count = rules::read_embedded_region_charge(self.count, start, end, false);
        self.bytes = rules::read_embedded_region_charge(self.bytes, start, end, true);
        self.parse_child(
            network,
            text,
            configuration,
            position,
            region,
            [region_link, language_link],
        );
    }

    fn parse_child(
        &mut self,
        network: &mut LinkNetwork,
        text: &str,
        configuration: ParseConfiguration,
        position: usize,
        region: &EmbeddedRegion,
        [region_link, language_link]: [LinkId; 2],
    ) {
        let language = region.language();
        let span = region.span();
        let bounds = span.byte_range();
        if self.recorded_languages.insert(language_link) {
            network.record_grammars(language_link, language);
        }
        let region_text = &text[bounds.start()..bounds.end()];
        let child = tree_sitter_adapter::parse_embedded_region_into(
            network,
            region_link,
            region_text,
            language,
            span,
            configuration,
        )
        .or_else(|| {
            structured_text_parser::parse_embedded_region_into(
                network,
                region_link,
                region_text,
                language,
                span,
                configuration,
            )
        });
        if child.is_some() {
            let next_language = canonical_language_name(language).unwrap_or(language);
            let selected_language = self.language_identifiers[next_language];
            let depth = self.frames[position].depth + 1.0;
            self.frames.push(EmbeddedRegionFrame {
                parent: region_link,
                language: language.to_owned(),
                bounds,
                depth,
            });
            self.parents.push(number(position));
            self.languages.push(selected_language);
            self.starts.push(number(bounds.start()));
            self.ends.push(number(bounds.end()));
        }
    }
}

struct EmbeddedRegionDiagnostic<'a> {
    reason: &'a str,
    depth: f64,
    count: f64,
    bytes: f64,
    observed: f64,
    limit: f64,
    ancestor: Option<LinkId>,
}

fn insert_embedded_region_diagnostic(
    network: &mut LinkNetwork,
    region: LinkId,
    language: &str,
    span: SourceSpan,
    counters: &EmbeddedRegionDiagnostic<'_>,
) -> LinkId {
    let diagnostic = network.insert_link(
        [region],
        LinkMetadata::new()
            .with_link_type(LinkType::Relation)
            .with_term("embedded region diagnostic")
            .with_language(language)
            .with_definition(counters.reason)
            .with_span(span)
            .with_flags(LinkFlags::error()),
    );
    for (label, value) in [
        ("depth", counters.depth),
        ("region count", counters.count),
        ("child parse bytes", counters.bytes),
        ("observed", counters.observed),
        ("limit", counters.limit),
    ] {
        if value < 0.0 {
            continue;
        }
        let scalar = network.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Object)
                .with_term(value.to_string())
                .with_definition("nonnegative integer"),
        );
        network.insert_field(diagnostic, label, scalar);
    }
    if let Some(ancestor) = counters.ancestor {
        network.insert_field(diagnostic, "ancestor", ancestor);
    }
    diagnostic
}

pub fn attach_embedded_regions(
    network: &mut LinkNetwork,
    document: LinkId,
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
) {
    EmbeddedRegionTraversal::new(document, text, language, configuration).run(
        network,
        text,
        configuration,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;

    #[test]
    fn synthetic_embedding_cycles_keep_the_actual_ancestor_and_error_after_reload() {
        let mut network = LinkNetwork::new();
        let span = SourceSpan::new(ByteRange::new(0, 2), Point::new(0, 0), Point::new(0, 2));
        let outer = network.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Region)
                .with_language("Markdown")
                .with_span(span),
        );
        let inner = network.insert_link(
            [outer],
            LinkMetadata::new()
                .with_link_type(LinkType::Region)
                .with_language("HTML")
                .with_span(span),
        );
        let rejected = network.insert_link(
            [inner],
            LinkMetadata::new()
                .with_link_type(LinkType::Region)
                .with_language("Markdown")
                .with_span(span),
        );
        let ancestor = rules::read_embedded_region_ancestor(
            &[-1.0, 0.0],
            &[0.0, 1.0],
            &[0.0, 0.0],
            &[2.0, 2.0],
            1.0,
            0.0,
            0.0,
            2.0,
        );
        assert_eq!(rules::ml_array::number_index(ancestor), Some(0));
        let reason = rules::read_embedded_region_disposition(
            0.0,
            2.0,
            1.0,
            0.0,
            0.0,
            2.0,
            ancestor,
            2.0,
            1.0,
            2.0,
            128.0,
            4096.0,
            16_777_216.0,
            true,
        );
        assert_eq!(reason, "nonConsumingEmbeddingCycle");
        let diagnostic = insert_embedded_region_diagnostic(
            &mut network,
            rejected,
            "Markdown",
            span,
            &EmbeddedRegionDiagnostic {
                reason: &reason,
                depth: 2.0,
                count: 1.0,
                bytes: 2.0,
                observed: -1.0,
                limit: -1.0,
                ancestor: Some(outer),
            },
        );
        network.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Token)
                .with_term("π")
                .with_language("Markdown")
                .with_span(span),
        );
        let restored = LinkNetwork::from_lino(&network.to_lino()).unwrap();
        for current in [&network, &restored] {
            assert!(!current.verify_full_match(None).is_clean());
            assert_eq!(
                current.link(diagnostic).unwrap().metadata().definition(),
                Some(reason.as_str())
            );
            let mut fields = current
                .links()
                .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
                .filter(|link| link.references().first() == Some(&diagnostic));
            assert!(fields.any(|field| {
                let references = field.references();
                current.link(references[1]).unwrap().metadata().term() == Some("ancestor")
                    && references[2] == outer
            }));
            assert!(
                !current
                    .links()
                    .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
                    .filter(|link| link.references().first() == Some(&diagnostic))
                    .any(|field| current
                        .link(field.references()[1])
                        .unwrap()
                        .metadata()
                        .term()
                        == Some("observed"))
            );
            assert_eq!(current.reconstruct_text(), "π");
        }
    }

    #[test]
    fn generated_region_policy_keeps_malformed_ancestry_zero_width_and_current_host_reuse_distinct()
    {
        assert!(rules::accept_embedded_region_bounds(10.0, 20.0, 20.0, 20.0));
        assert!(!rules::accept_embedded_region_bounds(10.0, 20.0, 9.0, 20.0));
        assert!(!rules::accept_embedded_region_limit(f64::NAN));
        assert!(!rules::accept_embedded_region_limit(f64::INFINITY));
        assert!(!rules::accept_embedded_region_limit(-1.0));
        assert!(!rules::accept_embedded_region_limit(0.5));
        let invalid = rules::read_embedded_region_ancestor(
            &[0.0],
            &[0.0],
            &[0.0],
            &[2.0],
            0.0,
            0.0,
            0.0,
            2.0,
        );
        assert_eq!(invalid.to_bits(), (-2.0_f64).to_bits());
        assert_eq!(
            rules::read_embedded_region_disposition(
                0.0, 2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, true
            ),
            "reuseAttached"
        );
        assert_eq!(
            rules::read_embedded_region_disposition(
                0.0, 2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, true
            ),
            "reuse"
        );
        assert_eq!(
            rules::read_embedded_region_disposition(
                0.0,
                2.0,
                0.0,
                0.0,
                0.0,
                2.0,
                0.0,
                1.0,
                0.0,
                0.0,
                128.0,
                4096.0,
                16_777_216.0,
                false
            ),
            "parse"
        );
    }
}
