use std::fmt::Write as _;

use crate::{ByteRange, LinkMetadata, LinkNetwork, LinkType, Point, SourceSpan};

const BYTE_CHUNK_SIZE: usize = 4096;

impl LinkNetwork {
    /// Stores an arbitrary file as a lossless byte-token network.
    ///
    /// Unlike [`Self::parse`], this boundary does not require UTF-8 and does not
    /// interpret the file. `format` is an opaque media type, extension, or other
    /// caller-defined format identifier. Format-specific parsers can enrich the
    /// returned network without changing its lossless byte layer.
    #[must_use]
    pub fn parse_bytes(bytes: &[u8], format: &str) -> Self {
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
    if encoded.len() % 2 != 0 {
        return None;
    }
    (0..encoded.len())
        .step_by(2)
        .map(|offset| u8::from_str_radix(&encoded[offset..offset + 2], 16))
        .collect::<Result<_, _>>()
        .ok()
}
