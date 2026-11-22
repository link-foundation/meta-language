// Generated from js/src/document-container-policy.js.
pub(super) fn document_stream_fits_budget(encoded: usize, decoded: usize, previous: usize) -> bool {
    encoded <= 16_777_216 && decoded <= 16_777_216 && previous + decoded <= 33_554_432
}

pub(super) fn pdf_stream_filter_kind(name: &str) -> u8 {
    match name {
        "FlateDecode" | "Fl" => 1,
        "ASCIIHexDecode" | "AHx" => 2,
        "ASCII85Decode" | "A85" => 3,
        "LZWDecode" | "LZW" => 4,
        _ => 0,
    }
}

pub(super) fn pdf_lzw_code_width(width: usize, next: usize, early_change: usize) -> usize {
    let threshold = next + early_change;
    match (width, threshold) {
        (9, 512) => 10,
        (10, 1024) => 11,
        (11, 2048) => 12,
        _ => width,
    }
}

pub(super) fn pdf_text_operator_kind(operator: &str) -> u8 {
    match operator {
        "Tf" => 1,
        "Do" => 2,
        "Tj" => 3,
        "TJ" => 4,
        "'" | "\"" => 5,
        "BT" | "ET" | "T*" => 6,
        "Td" | "TD" | "Tm" => 7,
        _ => 0,
    }
}

pub(super) fn pdf_drawing_operator(operator: &str) -> bool {
    matches!(
        operator,
        "q" | "Q"
            | "cm"
            | "m"
            | "l"
            | "c"
            | "re"
            | "S"
            | "s"
            | "f"
            | "f*"
            | "F"
            | "B"
            | "B*"
            | "b"
            | "b*"
            | "n"
            | "sh"
    )
}

pub(super) fn wordprocessing_relationship_order(kind: &str) -> u8 {
    match kind {
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
        | "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument" => 0,
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header"
        | "http://purl.oclc.org/ooxml/officeDocument/relationships/header" => 1,
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer"
        | "http://purl.oclc.org/ooxml/officeDocument/relationships/footer" => 2,
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes"
        | "http://purl.oclc.org/ooxml/officeDocument/relationships/footnotes"
        | "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes"
        | "http://purl.oclc.org/ooxml/officeDocument/relationships/endnotes" => 3,
        _ => 4,
    }
}
