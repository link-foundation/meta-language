use std::collections::{HashMap, HashSet};

use crate::configuration::{ParseConfiguration, embedded_region_number as number};
use crate::language_catalog::canonical_language_name;
use crate::link_network::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::mixed_regions::{EmbeddedRegion, HostTreeIndex, detect_embedded_regions_in_scope};
use crate::translation::embedded_region_rules as rules;
use crate::{ByteRange, LinkFlags, SourceSpan, structured_text_parser, tree_sitter_adapter};

mod diagnostics;

use diagnostics::{EmbeddedRegionDiagnostic, insert_embedded_region_diagnostic};

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
