use crate::line_index::LineIndex;
use crate::{ByteRange, LinkId, LinkMetadata, LinkNetwork, LinkType, Point, SourceSpan};

const DERIVED_DOCUMENT_HEADING: &str = "derived document\n";
const CHECKPOINT_WIDTH: usize = 4096;

/// One decoded part and its link to the physical container.
pub struct DerivedDocument {
    /// Stream kind or package part name.
    pub name: String,
    /// Language of the decoded content.
    pub language: String,
    /// Extent of the encoded bytes in the original document.
    pub source_range: ByteRange,
    /// Whether the original was parsed as bytes or as a binary string.
    pub encoding: String,
    /// Filters applied to the part, or its ZIP storage method.
    pub compression: String,
    /// Independent lossless network in decoded coordinates.
    pub network: LinkNetwork,
}

impl LinkNetwork {
    /// Reads the decoded documents stored as links in a PDF or DOCX network.
    #[must_use]
    pub fn derived_documents(&self) -> Vec<DerivedDocument> {
        self.links()
            .filter_map(|link| {
                let metadata = link.metadata();
                if metadata.link_type() != Some(LinkType::Document) {
                    return None;
                }
                let (encoding, rest) = metadata
                    .definition()?
                    .strip_prefix(DERIVED_DOCUMENT_HEADING)?
                    .split_once('\n')?;
                let (compression, notation) = rest.split_once('\n')?;
                Some(DerivedDocument {
                    name: metadata.term()?.into(),
                    language: metadata.language()?.into(),
                    source_range: metadata.span()?.byte_range(),
                    encoding: encoding.into(),
                    compression: compression.into(),
                    network: Self::from_lino(notation).ok()?,
                })
            })
            .collect()
    }
}

pub(super) fn insert_derived_document(
    container: &mut LinkNetwork,
    parent: LinkId,
    metadata: LinkMetadata,
    encoding: &str,
    compression: &str,
    content: &LinkNetwork,
) {
    let notation = content.to_lino();
    container.insert_link(
        [parent],
        metadata
            .with_link_type(LinkType::Document)
            .with_named(true)
            .with_definition(format!(
                "{DERIVED_DOCUMENT_HEADING}{encoding}\n{compression}\n{notation}"
            )),
    );
}

pub(crate) fn binary_string_bytes(text: &str) -> Option<Vec<u8>> {
    text.chars()
        .map(|character| u8::try_from(u32::from(character)).ok())
        .collect()
}

pub(super) fn binary_string(bytes: &[u8]) -> String {
    bytes.iter().copied().map(char::from).collect()
}

pub(crate) fn byte_source_span(start: usize, end: usize) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(start, end),
        Point::new(0, start),
        Point::new(0, end),
    )
}

pub(crate) fn text_source_span(lines: &LineIndex, start: usize, end: usize) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(start, end),
        lines.byte_point(start),
        lines.byte_point(end),
    )
}

/// Translates offsets between original bytes and their Latin-1 view in UTF-8.
pub(super) struct BinaryStringCoordinates<'bytes> {
    source: &'bytes [u8],
    checkpoints: Vec<usize>,
}

impl<'bytes> BinaryStringCoordinates<'bytes> {
    pub(super) fn new(source: &'bytes [u8]) -> Self {
        let mut checkpoints = vec![0];
        let mut length = 0;
        for block in source.chunks_exact(CHECKPOINT_WIDTH) {
            length += block.iter().map(|byte| byte_width(*byte)).sum::<usize>();
            checkpoints.push(length);
        }
        Self {
            source,
            checkpoints,
        }
    }

    pub(super) fn text_offset(&self, raw_offset: usize) -> usize {
        let block_number = raw_offset / CHECKPOINT_WIDTH;
        let block_start = block_number * CHECKPOINT_WIDTH;
        self.checkpoints[block_number]
            + self.source[block_start..raw_offset]
                .iter()
                .map(|byte| byte_width(*byte))
                .sum::<usize>()
    }

    pub(super) fn byte_offset(&self, text_offset: usize) -> usize {
        let block_number = self
            .checkpoints
            .partition_point(|offset| *offset <= text_offset)
            - 1;
        let mut current = self.checkpoints[block_number];
        let mut raw_offset = block_number * CHECKPOINT_WIDTH;
        while current < text_offset && raw_offset < self.source.len() {
            current += byte_width(self.source[raw_offset]);
            raw_offset += 1;
        }
        raw_offset
    }
}

const fn byte_width(byte: u8) -> usize {
    if byte < 128 { 1 } else { 2 }
}
