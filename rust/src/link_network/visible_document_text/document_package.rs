use std::collections::{BTreeMap, HashSet};
use std::str;

use miniz_oxide::inflate::decompress_to_vec_with_limit;

use super::document_container::{
    BinaryStringCoordinates, binary_string_bytes, insert_derived_document, text_source_span,
};
use super::document_container_policy::{
    document_stream_fits_budget, wordprocessing_relationship_order,
};
use super::{DocumentSyntax, ReferenceSyntax, SyntaxRecord, character_reference};
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

#[derive(Clone, Copy)]
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

struct Relationship {
    kind: String,
    target: String,
    mode: Option<String>,
}

fn relationship_attributes<'source>(
    syntax: &DocumentSyntax<'source>,
    tag: &SyntaxRecord<'source>,
) -> Result<BTreeMap<&'source str, String>, &'static str> {
    let mut fields = BTreeMap::new();
    for attribute in tag.children.iter().filter_map(|id| syntax.records.get(id)) {
        if attribute.term != "Attribute" {
            continue;
        }
        let Some(name) = syntax.child(attribute, |term| term == "Name") else {
            continue;
        };
        let value = syntax
            .child(attribute, |term| term == "AttValue")
            .map_or("", |value| {
                let text = syntax.read(value.range);
                text.get(1..text.len().saturating_sub(1))
                    .unwrap_or_default()
            });
        let mut decoded = String::new();
        let mut rest = value;
        while let Some(index) = rest.find('&') {
            decoded.push_str(&rest[..index]);
            let (size, reference) = character_reference(&rest[index..], ReferenceSyntax::Xml)
                .ok_or("invalid OPC relationship attribute")?;
            decoded.push_str(&reference);
            rest = &rest[index + size..];
        }
        decoded.push_str(rest);
        fields.insert(syntax.read(name.range), decoded);
    }
    Ok(fields)
}

fn relationship_name<'source>(
    syntax: &DocumentSyntax<'source>,
    tag: &SyntaxRecord<'source>,
) -> (&'source str, String) {
    let name = syntax
        .child(tag, |term| term == "Name")
        .map_or("", |name| syntax.read(name.range));
    name.split_once(':').map_or_else(
        || (name, "xmlns".to_owned()),
        |(prefix, local)| (local, format!("xmlns:{prefix}")),
    )
}

fn package_relationships(
    text: &str,
    configuration: ParseConfiguration,
) -> Result<Vec<Relationship>, &'static str> {
    const NAMESPACE: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
    let network = crate::docx_parser::parse_xml(text, "DOCX", configuration);
    if !network.verify_full_match(None).is_clean() {
        return Err("invalid OPC relationships XML");
    }
    let syntax = DocumentSyntax::new(&network, text);
    let records = syntax.in_language(Some("DOCX"));
    let document = records
        .into_iter()
        .find(|record| record.term == "element")
        .ok_or("missing OPC relationships element")?;
    let open_tag = |term: &str| matches!(term, "STag" | "EmptyElemTag");
    let root = syntax
        .child(document, open_tag)
        .ok_or("missing OPC relationships element")?;
    let (name, binding) = relationship_name(&syntax, root);
    let bindings = relationship_attributes(&syntax, root)?;
    if name != "Relationships"
        || bindings.get(binding.as_str()).map(String::as_str) != Some(NAMESPACE)
    {
        return Err("invalid OPC relationships element");
    }
    let mut result = Vec::new();
    if let Some(content) = syntax.child(document, |term| term == "content") {
        for child in content
            .children
            .iter()
            .filter_map(|id| syntax.records.get(id))
        {
            if child.term != "element" {
                continue;
            }
            let Some(tag) = syntax.child(child, open_tag) else {
                continue;
            };
            let (name, binding) = relationship_name(&syntax, tag);
            let mut fields = relationship_attributes(&syntax, tag)?;
            let binding = binding.as_str();
            if name == "Relationship"
                && fields
                    .get(binding)
                    .or_else(|| bindings.get(binding))
                    .map(String::as_str)
                    == Some(NAMESPACE)
            {
                result.push(Relationship {
                    kind: fields.remove("Type").unwrap_or_default(),
                    target: fields.remove("Target").unwrap_or_default(),
                    mode: fields.remove("TargetMode"),
                });
            }
        }
    }
    Ok(result)
}

fn resolve_package_target(source_name: &str, target: &str) -> Option<String> {
    if target.is_empty()
        || target
            .bytes()
            .any(|byte| byte <= 0x20 || matches!(byte, 0x7f | b'\\' | b':' | b'#' | b'?'))
    {
        return None;
    }
    let mut segments = Vec::new();
    if !target.starts_with('/')
        && let Some((directory, _)) = source_name.rsplit_once('/')
    {
        segments.extend(directory.split('/'));
    }
    for segment in target.strip_prefix('/').unwrap_or(target).split('/') {
        match segment {
            "" => return None,
            "." => continue,
            ".." => {
                segments.pop()?;
            }
            _ => segments.push(segment),
        }
    }
    (!segments.is_empty()).then(|| segments.join("/"))
}

fn relationship_targets<'entries, 'source>(
    source: &[u8],
    entries: &'entries BTreeMap<&'source str, Entry<'source>>,
    entry: &Entry<'_>,
    source_name: &str,
    configuration: ParseConfiguration,
    decoded_bytes: &mut usize,
    errors: &mut Vec<PackageError>,
) -> Vec<(u8, &'entries Entry<'source>)> {
    let relationships = decode_part(source, entry, decoded_bytes).and_then(|text| {
        package_relationships(&text, configuration)
            .map_err(|message| PackageError::part(entry, message))
    });
    let relationships = match relationships {
        Ok(relationships) => relationships,
        Err(error) => {
            errors.push(error);
            return Vec::new();
        }
    };
    let mut result = Vec::new();
    for relation in relationships {
        let order = wordprocessing_relationship_order(&relation.kind);
        let mode = relation.mode.as_deref();
        if order >= 4 || (order == 0) != source_name.is_empty() || mode == Some("External") {
            continue;
        }
        let target = if matches!(mode, None | Some("Internal")) {
            resolve_package_target(source_name, &relation.target)
        } else {
            None
        };
        if let Some(selected) = target.as_deref().and_then(|name| entries.get(name)) {
            result.push((order, selected));
        } else {
            errors.push(PackageError::part(
                entry,
                &format!(
                    "missing or invalid WordprocessingML relationship target: {}",
                    relation.target
                ),
            ));
        }
    }
    result
}

fn select_wordprocessing_parts<'entries, 'source>(
    source: &[u8],
    entries: &'entries BTreeMap<&'source str, Entry<'source>>,
    configuration: ParseConfiguration,
    decoded_bytes: &mut usize,
    errors: &mut Vec<PackageError>,
) -> Vec<(u8, &'entries Entry<'source>)> {
    let package = entries.get("_rels/.rels");
    let main = if let Some(package) = package {
        let root = relationship_targets(
            source,
            entries,
            package,
            "",
            configuration,
            decoded_bytes,
            errors,
        );
        if root.len() > 1 {
            errors.push(PackageError::part(
                package,
                "multiple WordprocessingML document relationships",
            ));
        }
        if let [(_, main)] = root.as_slice() {
            Some(*main)
        } else {
            None
        }
    } else {
        entries.get("word/document.xml")
    };
    let mut selected = Vec::new();
    if let Some(main) = main {
        selected.push((0, main));
        let (directory, name) = main
            .name
            .rsplit_once('/')
            .map_or(("", main.name), |(directory, name)| (directory, name));
        let relationship = if directory.is_empty() {
            format!("_rels/{name}.rels")
        } else {
            format!("{directory}/_rels/{name}.rels")
        };
        if let Some(entry) = entries.get(relationship.as_str()) {
            let mut names = HashSet::from([main.name]);
            selected.extend(
                relationship_targets(
                    source,
                    entries,
                    entry,
                    main.name,
                    configuration,
                    decoded_bytes,
                    errors,
                )
                .into_iter()
                .filter(|(_, candidate)| names.insert(candidate.name)),
            );
            return selected;
        }
    }
    if package.is_none() {
        selected.extend(entries.values().filter_map(|entry| {
            part_order(entry.name)
                .filter(|order| *order > 0)
                .map(|order| (order, entry))
        }));
    }
    selected
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
            let entries = entries
                .into_iter()
                .map(|entry| (entry.name, entry))
                .collect();
            let mut selected = select_wordprocessing_parts(
                source,
                &entries,
                configuration,
                &mut decoded_bytes,
                &mut errors,
            );
            selected.sort_by_key(|(order, entry)| (*order, entry.start));
            for (order, entry) in selected {
                match decode_part(source, entry, &mut decoded_bytes) {
                    Ok(text) => {
                        let document = crate::docx_parser::parse_xml(&text, "DOCX", configuration);
                        let clean = document.verify_full_match(None).is_clean();
                        parts.push((order, *entry, document, clean));
                    }
                    Err(error) => errors.push(error),
                }
            }
            if !parts.iter().any(|(order, ..)| *order == 0) {
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
