//! OPC (Open Packaging Conventions) binary layer for real `.docx` packages.
//!
//! A DOCX file is a ZIP container of OOXML parts. The text content layer in
//! [`super::docx`] models the primary `word/document.xml` part; this module
//! wraps that part — together with the fixed packaging parts a conformant reader
//! requires — into a valid ZIP archive, and reads `word/document.xml` back out
//! of such an archive.
//!
//! # Constrained profile
//!
//! To keep the crate dependency-free, packages are written as **stored**
//! (uncompressed) ZIP entries with a self-implemented CRC-32, and read back the
//! same way. A package produced by [`render_docx_package`] opens in conformant
//! word processors; an arbitrary externally-authored `.docx` that uses DEFLATE
//! compression is out of profile for [`parse_docx_package`] (its
//! `word/document.xml` is not extracted). See `docs/docx-fidelity.md`.

use super::document::FormattingDocument;
use super::docx::{parse_docx_document, render_docx_document};

const DOCUMENT_PART: &str = "word/document.xml";

// --- fixed packaging parts -------------------------------------------------

const CONTENT_TYPES: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\
<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\
<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
<Default Extension=\"xml\" ContentType=\"application/xml\"/>\
<Override PartName=\"/word/document.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml\"/>\
<Override PartName=\"/word/styles.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml\"/>\
<Override PartName=\"/word/numbering.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml\"/>\
</Types>\n";

const ROOT_RELS: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\
<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\
<Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"word/document.xml\"/>\
</Relationships>\n";

const DOCUMENT_RELS: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\
<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\
<Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles\" Target=\"styles.xml\"/>\
<Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering\" Target=\"numbering.xml\"/>\
</Relationships>\n";

const STYLES: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\
<w:styles xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">\
<w:style w:type=\"paragraph\" w:styleId=\"Heading1\"><w:name w:val=\"heading 1\"/><w:pPr><w:outlineLvl w:val=\"0\"/></w:pPr></w:style>\
<w:style w:type=\"paragraph\" w:styleId=\"Heading2\"><w:name w:val=\"heading 2\"/><w:pPr><w:outlineLvl w:val=\"1\"/></w:pPr></w:style>\
<w:style w:type=\"paragraph\" w:styleId=\"Heading3\"><w:name w:val=\"heading 3\"/><w:pPr><w:outlineLvl w:val=\"2\"/></w:pPr></w:style>\
<w:style w:type=\"paragraph\" w:styleId=\"Heading4\"><w:name w:val=\"heading 4\"/><w:pPr><w:outlineLvl w:val=\"3\"/></w:pPr></w:style>\
<w:style w:type=\"paragraph\" w:styleId=\"Heading5\"><w:name w:val=\"heading 5\"/><w:pPr><w:outlineLvl w:val=\"4\"/></w:pPr></w:style>\
<w:style w:type=\"paragraph\" w:styleId=\"Heading6\"><w:name w:val=\"heading 6\"/><w:pPr><w:outlineLvl w:val=\"5\"/></w:pPr></w:style>\
</w:styles>\n";

const NUMBERING: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\
<w:numbering xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">\
<w:abstractNum w:abstractNumId=\"0\"><w:lvl w:ilvl=\"0\"><w:numFmt w:val=\"bullet\"/><w:lvlText w:val=\"\u{2022}\"/></w:lvl></w:abstractNum>\
<w:abstractNum w:abstractNumId=\"1\"><w:lvl w:ilvl=\"0\"><w:numFmt w:val=\"decimal\"/><w:lvlText w:val=\"%1.\"/></w:lvl></w:abstractNum>\
<w:num w:numId=\"1\"><w:abstractNumId w:val=\"0\"/></w:num>\
<w:num w:numId=\"2\"><w:abstractNumId w:val=\"1\"/></w:num>\
</w:numbering>\n";

// --- public API ------------------------------------------------------------

/// Renders a language-free [`FormattingDocument`] into a valid `.docx` (OPC ZIP)
/// package in the documented stored-entry profile.
#[must_use]
pub fn render_docx_package(document: &FormattingDocument) -> Vec<u8> {
    let document_xml = render_docx_document(document);
    let entries: [(&str, &[u8]); 6] = [
        ("[Content_Types].xml", CONTENT_TYPES.as_bytes()),
        ("_rels/.rels", ROOT_RELS.as_bytes()),
        (DOCUMENT_PART, document_xml.as_bytes()),
        ("word/_rels/document.xml.rels", DOCUMENT_RELS.as_bytes()),
        ("word/styles.xml", STYLES.as_bytes()),
        ("word/numbering.xml", NUMBERING.as_bytes()),
    ];
    write_zip(&entries)
}

/// Parses the `word/document.xml` part of a stored-profile `.docx` package back
/// into the language-free concept layer.
///
/// Returns an empty document when the package has no extractable
/// `word/document.xml` (for example a DEFLATE-compressed external file), so
/// out-of-profile packages degrade gracefully rather than panicking.
#[must_use]
pub fn parse_docx_package(bytes: &[u8]) -> FormattingDocument {
    read_zip_entry(bytes, DOCUMENT_PART)
        .and_then(|data| String::from_utf8(data).ok())
        .map_or_else(FormattingDocument::default, |document_xml| {
            parse_docx_document(&document_xml)
        })
}

/// Whether `bytes` is a stored-profile `.docx` package carrying at least one
/// recognized block in its `word/document.xml` part.
#[must_use]
pub fn docx_package_is_recognized(bytes: &[u8]) -> bool {
    !parse_docx_package(bytes).blocks.is_empty()
}

// --- ZIP writer (stored entries) -------------------------------------------

const LOCAL_HEADER_SIGNATURE: u32 = 0x0403_4b50;
const CENTRAL_HEADER_SIGNATURE: u32 = 0x0201_4b50;
const END_OF_CENTRAL_SIGNATURE: u32 = 0x0605_4b50;
const VERSION: u16 = 20;

fn write_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut central = Vec::new();
    let mut offsets = Vec::with_capacity(entries.len());

    for (name, data) in entries {
        offsets.push(u32::try_from(output.len()).expect("archive offset fits in u32"));
        let crc = crc32(data);
        let size = u32::try_from(data.len()).expect("entry length fits in u32");
        let name_len = u16::try_from(name.len()).expect("name length fits in u16");

        // Local file header.
        push_u32(&mut output, LOCAL_HEADER_SIGNATURE);
        push_u16(&mut output, VERSION);
        push_u16(&mut output, 0); // general purpose flags
        push_u16(&mut output, 0); // method: stored
        push_u16(&mut output, 0); // mod time
        push_u16(&mut output, 0); // mod date
        push_u32(&mut output, crc);
        push_u32(&mut output, size); // compressed size
        push_u32(&mut output, size); // uncompressed size
        push_u16(&mut output, name_len);
        push_u16(&mut output, 0); // extra length
        output.extend_from_slice(name.as_bytes());
        output.extend_from_slice(data);

        // Central directory header (deferred).
        push_u32(&mut central, CENTRAL_HEADER_SIGNATURE);
        push_u16(&mut central, VERSION); // version made by
        push_u16(&mut central, VERSION); // version needed
        push_u16(&mut central, 0);
        push_u16(&mut central, 0); // method: stored
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u32(&mut central, crc);
        push_u32(&mut central, size);
        push_u32(&mut central, size);
        push_u16(&mut central, name_len);
        push_u16(&mut central, 0); // extra
        push_u16(&mut central, 0); // comment
        push_u16(&mut central, 0); // disk number
        push_u16(&mut central, 0); // internal attrs
        push_u32(&mut central, 0); // external attrs
        push_u32(&mut central, *offsets.last().expect("offset pushed above"));
        central.extend_from_slice(name.as_bytes());
    }

    let central_offset = u32::try_from(output.len()).expect("central offset fits in u32");
    let central_size = u32::try_from(central.len()).expect("central size fits in u32");
    let count = u16::try_from(entries.len()).expect("entry count fits in u16");
    output.extend_from_slice(&central);

    // End of central directory record.
    push_u32(&mut output, END_OF_CENTRAL_SIGNATURE);
    push_u16(&mut output, 0); // disk number
    push_u16(&mut output, 0); // disk with central dir
    push_u16(&mut output, count);
    push_u16(&mut output, count);
    push_u32(&mut output, central_size);
    push_u32(&mut output, central_offset);
    push_u16(&mut output, 0); // comment length

    output
}

// --- ZIP reader (stored entries) -------------------------------------------

/// Extracts the stored data of `name` by scanning local file headers.
fn read_zip_entry(bytes: &[u8], name: &str) -> Option<Vec<u8>> {
    let mut cursor = 0usize;
    while cursor + 30 <= bytes.len() {
        if read_u32(bytes, cursor)? != LOCAL_HEADER_SIGNATURE {
            break;
        }
        let method = read_u16(bytes, cursor + 8)?;
        let compressed = read_u32(bytes, cursor + 18)? as usize;
        let name_len = read_u16(bytes, cursor + 26)? as usize;
        let extra_len = read_u16(bytes, cursor + 28)? as usize;
        let name_start = cursor + 30;
        let data_start = name_start + name_len + extra_len;
        let data_end = data_start + compressed;
        if data_end > bytes.len() {
            break;
        }
        let entry_name = bytes.get(name_start..name_start + name_len)?;
        if entry_name == name.as_bytes() && method == 0 {
            return Some(bytes[data_start..data_end].to_vec());
        }
        cursor = data_end;
    }
    None
}

// --- little-endian helpers -------------------------------------------------

fn push_u16(buffer: &mut Vec<u8>, value: u16) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let slice = bytes.get(offset..offset + 2)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset + 4)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

// --- CRC-32 (IEEE 802.3, polynomial 0xEDB88320) ----------------------------

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}
