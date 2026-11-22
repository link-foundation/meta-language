use std::fmt::Write as _;

use crate::link_network::visible_document_text::{
    byte_source_span, document_package, pdf_document,
};
use crate::{
    ByteRange, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration, Point, SourceSpan,
};

const BYTE_CHUNK_SIZE: usize = 4096;

impl LinkNetwork {
    /// Stores an arbitrary file as a lossless byte-token network.
    ///
    /// Binary document formats can attach decoded parts to the lossless layer.
    /// Other format identifiers are stored as opaque names.
    #[must_use]
    pub fn parse_bytes(bytes: &[u8], format: &str) -> Self {
        Self::parse_bytes_with_configuration(bytes, format, ParseConfiguration::default())
    }

    /// Stores arbitrary bytes and applies the supplied document options.
    #[must_use]
    pub fn parse_bytes_with_configuration(
        bytes: &[u8],
        format: &str,
        configuration: ParseConfiguration,
    ) -> Self {
        let mut network = Self::self_describing();
        let format_link = network.insert_typed_point(format, LinkType::Language, None);
        let document_span = SourceSpan::new(
            ByteRange::new(0, bytes.len()),
            Point::new(0, 0),
            Point::new(0, bytes.len()),
        );
        let document = network.insert_link(
            [format_link],
            LinkMetadata::new()
                .with_link_type(LinkType::Document)
                .with_named(true)
                .with_term(format!("{format} document"))
                .with_language(format)
                .with_span(document_span),
        );

        for (chunk_index, chunk) in bytes.chunks(BYTE_CHUNK_SIZE).enumerate() {
            let offset = chunk_index * BYTE_CHUNK_SIZE;
            let encoded = encode_bytes(chunk);
            network.insert_link(
                [document],
                LinkMetadata::new()
                    .with_link_type(LinkType::Token)
                    .with_named(true)
                    .with_term(format!("bytes:{encoded}"))
                    .with_language(format)
                    .with_span(SourceSpan::new(
                        ByteRange::new(offset, offset + chunk.len()),
                        Point::new(0, offset),
                        Point::new(0, offset + chunk.len()),
                    )),
            );
        }
        match crate::language_catalog::canonical_language_name(format) {
            Some("DOCX") if document_package::has_zip_header(bytes) => {
                document_package::attach_wordprocessing_package(
                    &mut network,
                    document,
                    bytes,
                    format,
                    byte_source_span,
                    configuration,
                    "bytes",
                );
            }
            Some("PDF") if !bytes.is_empty() => {
                pdf_document::attach_binary_pdf(
                    &mut network,
                    document,
                    bytes,
                    format,
                    configuration,
                );
            }
            _ => {}
        }
        network
    }

    /// Reconstructs bytes stored by [`Self::parse_bytes`] in source order.
    #[must_use]
    pub fn reconstruct_bytes(&self) -> Vec<u8> {
        let mut tokens = self
            .links()
            .filter(|link| link.metadata().link_type() == Some(LinkType::Token))
            .filter_map(|link| {
                Some((
                    link.metadata().span()?.byte_range().start(),
                    link.id().as_u64(),
                    decode_bytes(link.metadata().term()?.strip_prefix("bytes:")?)?,
                ))
            })
            .collect::<Vec<_>>();
        tokens.sort_by_key(|(offset, id, _bytes)| (*offset, *id));
        tokens
            .into_iter()
            .flat_map(|(_offset, _id, bytes)| bytes)
            .collect()
    }
}

fn encode_bytes(bytes: &[u8]) -> String {
    bytes.iter().fold(
        String::with_capacity(bytes.len() * 2),
        |mut encoded, byte| {
            write!(encoded, "{byte:02x}").expect("writing to a String cannot fail");
            encoded
        },
    )
}

fn decode_bytes(encoded: &str) -> Option<Vec<u8>> {
    if !encoded.len().is_multiple_of(2) {
        return None;
    }
    (0..encoded.len())
        .step_by(2)
        .map(|offset| u8::from_str_radix(&encoded[offset..offset + 2], 16))
        .collect::<Result<_, _>>()
        .ok()
}
