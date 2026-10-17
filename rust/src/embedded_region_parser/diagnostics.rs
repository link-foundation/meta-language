use super::{LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, SourceSpan};

pub(super) struct EmbeddedRegionDiagnostic<'a> {
    pub(super) reason: &'a str,
    pub(super) depth: f64,
    pub(super) count: f64,
    pub(super) bytes: f64,
    pub(super) observed: f64,
    pub(super) limit: f64,
    pub(super) ancestor: Option<LinkId>,
}

pub(super) fn insert_embedded_region_diagnostic(
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
