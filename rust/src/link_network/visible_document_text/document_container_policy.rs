// Generated from js/src/document-container-policy.js.
pub(super) fn document_stream_fits_budget(encoded: usize, decoded: usize, previous: usize) -> bool {
    encoded <= 16_777_216 && decoded <= 16_777_216 && previous + decoded <= 33_554_432
}

pub(super) fn pdf_stream_filter_kind(name: &str) -> u8 {
    match name {
        "FlateDecode" | "Fl" => 1,
        "ASCIIHexDecode" | "AHx" => 2,
        "ASCII85Decode" | "A85" => 3,
        _ => 0,
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
