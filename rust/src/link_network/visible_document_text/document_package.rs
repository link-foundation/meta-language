use std::collections::HashSet;
use std::str;

use miniz_oxide::inflate::decompress_to_vec_with_limit;

use super::document_container::{
    BinaryStringCoordinates, binary_string_bytes, insert_derived_document, text_source_span,
};
use super::document_container_policy::document_stream_fits_budget;
use crate::line_index::LineIndex;
use crate::{
    LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration, SourceSpan,
};

const LOCAL_HEADER: u32 = 0x04034b50;
const CENTRAL_HEADER: u32 = 0x02014b50;
const END_OF_DIRECTORY: u32 = 0x06054b50;

pub(crate) fn has_zip_header(source: &[u8]) -> bool {
    source.starts_with(b"PK\x03\x04") || source.starts_with(b"PK\x05\x06")
}

pub(crate) fn parse_package_string(
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
) -> LinkNetwork {
    let (mut network, document) = LinkNetwork::new_parse_document(text, language);
    let lines = LineIndex::new(text);
    let full_span = text_source_span(&lines, 0, text.len());
    network.insert_link(
        [document],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_term(text)
            .with_language(language)
            .with_named(true)
            .with_span(full_span),
    );
    if let Some(bytes) = binary_string_bytes(text) {
        let coordinates = BinaryStringCoordinates::new(&bytes);
        attach_wordprocessing_package(
            &mut network,
            document,
            &bytes,
            language,
            |start, end| {
                text_source_span(
                    &lines,
                    coordinates.text_offset(start),
                    coordinates.text_offset(end),
                )
            },
            configuration,
            "binary-string",
        );
    } else {
        network.insert_link(
            [document],
            LinkMetadata::new()
                .with_link_type(LinkType::Syntax)
                .with_term("ERROR")
                .with_language(language)
                .with_named(true)
                .with_span(full_span)
                .with_flags(LinkFlags::error()),
        );
    }
    network
}

struct Entry<'name> {
    name: &'name str,
    start: usize,
    end: usize,
    local: usize,
    method: u16,
    flags: u16,
    checksum: u32,
    uncompressed: usize,
}

struct PackageError {
    message: String,
    start: usize,
    end: usize,
}

impl PackageError {
    fn at(source: &[u8], offset: usize, message: &str) -> Self {
        let start = offset.min(source.len());
        Self {
            message: message.into(),
            start,
            end: (start + 1).min(source.len()),
        }
    }

    fn part(entry: &Entry<'_>, message: &str) -> Self {
        Self {
            message: format!("{}: {message}", entry.name),
            start: entry.start,
            end: entry.end,
        }
    }
}

// ZIP central-directory metadata owns checksums and sizes, including entries
// that were written with data descriptors.
fn directory(source: &[u8]) -> Result<Vec<Entry<'_>>, PackageError> {
    let u16_at = |offset: usize| u16::from_le_bytes([source[offset], source[offset + 1]]);
    let u32_at = |offset: usize| {
        u32::from_le_bytes(source[offset..offset + 4].try_into().expect("four bytes"))
    };
    let number = |offset| usize::from(u16_at(offset));
    let error = |offset, message| PackageError::at(source, offset, message);
    let footer = source
        .len()
        .checked_sub(22)
        .and_then(|latest| {
            (latest.saturating_sub(65535)..=latest)
                .rev()
                .find(|offset| {
                    u32_at(*offset) == END_OF_DIRECTORY && number(*offset + 20) == latest - offset
                })
        })
        .ok_or_else(|| error(source.len(), "missing ZIP end of central directory"))?;
    let count = number(footer + 10);
    let central = u32_at(footer + 16) as usize;
    if u16_at(footer + 4) != 0
        || u16_at(footer + 6) != 0
        || count != number(footer + 8)
        || count == 65535
        || central.checked_add(u32_at(footer + 12) as usize) != Some(footer)
    {
        return Err(error(
            footer,
            "invalid or unsupported ZIP central directory",
        ));
    }
    let mut entries = Vec::new();
    let mut names = HashSet::new();
    let mut offset = central;
    for _ in 0..count {
        if footer.saturating_sub(offset) < 46 || u32_at(offset) != CENTRAL_HEADER {
            return Err(error(offset, "invalid ZIP central entry"));
        }
        let flags = u16_at(offset + 8);
        let method = u16_at(offset + 10);
        let checksum = u32_at(offset + 16);
        let compressed = u32_at(offset + 20) as usize;
        let uncompressed = u32_at(offset + 24) as usize;
        let name_length = number(offset + 28);
        let local = u32_at(offset + 42) as usize;
        let name_start = offset + 46;
        let next =
            name_start.saturating_add(name_length + number(offset + 30) + number(offset + 32));
        if next > footer
            || central.saturating_sub(local) < 30
            || u32_at(local) != LOCAL_HEADER
            || u16_at(offset + 34) != 0
            || flags != u16_at(local + 6)
            || method != u16_at(local + 8)
            || name_length != number(local + 26)
        {
            return Err(error(offset, "invalid ZIP entry header"));
        }
        let name_bytes = &source[name_start..name_start + name_length];
        let local_name = local + 30;
        let name_end = local_name.saturating_add(name_length);
        let start = name_end.saturating_add(number(local + 28));
        let end = start.saturating_add(compressed);
        if end > central || Some(name_bytes) != source.get(local_name..name_end) {
            return Err(error(local, "invalid ZIP entry data range or name"));
        }
        let name = str::from_utf8(name_bytes).map_err(|_| error(local, "invalid OPC part name"))?;
        if name.is_empty()
            || name.contains('\\')
            || name.starts_with('/')
            || name.split('/').any(|part| matches!(part, "." | ".."))
            || !names.insert(name)
        {
            return Err(error(local, "invalid or duplicate OPC part name"));
        }
        entries.push(Entry {
            name,
            start,
            end,
            local,
            method,
            flags,
            checksum,
            uncompressed,
        });
        offset = next;
    }
    if offset != footer {
        return Err(error(offset, "unrecognized ZIP central directory data"));
    }
    let mut ordered = entries.iter().collect::<Vec<_>>();
    ordered.sort_unstable_by_key(|entry| entry.local);
    if ordered.windows(2).any(|pair| pair[1].local < pair[0].end) {
        return Err(error(central, "overlapping ZIP entries"));
    }
    Ok(entries)
}

fn part_order(name: &str) -> Option<u8> {
    let part = name.strip_prefix("word/")?.strip_suffix(".xml")?;
    match part {
        "document" => Some(0),
        "footnotes" | "endnotes" => Some(3),
        part => {
            let (digits, order) = part
                .strip_prefix("header")
                .map(|digits| (digits, 1))
                .or_else(|| part.strip_prefix("footer").map(|digits| (digits, 2)))?;
            digits
                .bytes()
                .all(|digit| digit.is_ascii_digit())
                .then_some(order)
        }
    }
}

fn decode_part(
    source: &[u8],
    entry: &Entry<'_>,
    decoded_bytes: &mut usize,
) -> Result<String, PackageError> {
    let error = |message| PackageError::part(entry, message);
    if entry.flags & 0x2041 != 0 || !matches!(entry.method, 0 | 8) {
        return Err(error("unsupported OPC part compression or encryption"));
    }
    if !document_stream_fits_budget(entry.end - entry.start, entry.uncompressed, *decoded_bytes) {
        return Err(error("OPC part exceeds the decoded size limit"));
    }
    *decoded_bytes += entry.uncompressed;
    let raw = &source[entry.start..entry.end];
    let decoded;
    let data = if entry.method == 8 {
        decoded = decompress_to_vec_with_limit(raw, entry.uncompressed + 1)
            .map_err(|_| error("invalid or oversized deflated OPC part"))?;
        decoded.as_slice()
    } else {
        raw
    };
    if data.len() != entry.uncompressed || crc32fast::hash(data) != entry.checksum {
        return Err(error("OPC part size or checksum mismatch"));
    }
    str::from_utf8(data)
        .map(str::to_owned)
        .map_err(|_| error("invalid UTF-8 OPC part"))
}

pub(crate) fn attach_wordprocessing_package(
    network: &mut LinkNetwork,
    parent: LinkId,
    source: &[u8],
    language: &str,
    span: impl Fn(usize, usize) -> SourceSpan,
    configuration: ParseConfiguration,
    encoding: &str,
) {
    let mut parts = Vec::new();
    let mut errors = Vec::new();
    match directory(source) {
        Ok(entries) => {
            let mut decoded_bytes = 0;
            for entry in entries {
                let Some(order) = part_order(entry.name) else {
                    continue;
                };
                match decode_part(source, &entry, &mut decoded_bytes) {
                    Ok(text) => {
                        let document = crate::docx_parser::parse_xml(&text, "DOCX", configuration);
                        let clean = document.verify_full_match(None).is_clean();
                        parts.push((order, entry, document, clean));
                    }
                    Err(error) => errors.push(error),
                }
            }
            if !parts.iter().any(|(order, _, _, _)| *order == 0) {
                errors.push(PackageError::at(
                    source,
                    0,
                    "missing WordprocessingML document part",
                ));
            }
        }
        Err(error) => errors.push(error),
    }
    let has_error = !errors.is_empty() || parts.iter().any(|(_, _, _, clean)| !clean);
    let container = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_term("docx_package")
            .with_language(language)
            .with_named(true)
            .with_span(span(0, source.len()))
            .with_flags(if has_error {
                LinkFlags::containing_error()
            } else {
                LinkFlags::clean()
            }),
    );
    for error in errors {
        network.insert_link(
            [container],
            LinkMetadata::new()
                .with_link_type(LinkType::Syntax)
                .with_term("ERROR")
                .with_language(language)
                .with_named(true)
                .with_span(span(error.start, error.end))
                .with_definition(error.message)
                .with_flags(LinkFlags::error()),
        );
    }
    parts.sort_by_key(|(order, entry, _, _)| (*order, entry.start));
    for (_, entry, document, clean) in parts {
        let compression = if entry.method == 8 {
            "deflate"
        } else {
            "stored"
        };
        let metadata = LinkMetadata::new()
            .with_term(entry.name)
            .with_language("DOCX")
            .with_span(span(entry.start, entry.end))
            .with_flags(if clean {
                LinkFlags::clean()
            } else {
                LinkFlags::error()
            });
        insert_derived_document(
            network,
            container,
            metadata,
            encoding,
            compression,
            &document,
        );
    }
}
