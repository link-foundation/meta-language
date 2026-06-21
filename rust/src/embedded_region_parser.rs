use crate::configuration::ParseConfiguration;
use crate::data_format_parser;
use crate::link_network::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::mixed_regions::detect_embedded_regions;
use crate::tree_sitter_adapter;

pub fn attach_embedded_regions(
    network: &mut LinkNetwork,
    document: LinkId,
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
) {
    let policy = configuration.region_detection_policy();
    for region in detect_embedded_regions(text, language, policy) {
        let region_language = region.language().to_string();
        let language_link = network.insert_typed_point(&region_language, LinkType::Language, None);
        let region_link = network.insert_link(
            [document, language_link],
            LinkMetadata::new()
                .with_link_type(LinkType::Region)
                .with_named(true)
                .with_term(format!("{region_language} region"))
                .with_language(region_language)
                .with_span(region.span()),
        );
        let range = region.span().byte_range();
        let region_text = &text[range.start()..range.end()];
        if tree_sitter_adapter::parse_embedded_region_into(
            network,
            region_link,
            region_text,
            region.language(),
            region.span(),
            configuration,
        )
        .is_none()
        {
            let _ = data_format_parser::parse_embedded_region_into(
                network,
                region_link,
                region_text,
                region.language(),
                region.span(),
                configuration,
            );
        }
    }
}
