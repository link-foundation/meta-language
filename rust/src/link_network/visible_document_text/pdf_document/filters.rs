//! PDF stream filters and cumulative decode budgets.

use miniz_oxide::inflate::decompress_to_vec_zlib_with_limit;

use super::super::document_container::{binary_string, binary_string_bytes};
use super::super::document_container_policy::{document_stream_fits_budget, pdf_lzw_code_width};
use super::super::pdf_text_fonts::{hex_bytes, pdf_whitespace};
use super::{GrammarNode, MAXIMUM_DECODED_BYTES, MAXIMUM_STREAM_BYTES, PdfReader, text_of};

impl PdfReader<'_> {
    pub(super) fn decode(
        &mut self,
        source: &str,
        kinds: &[u8],
        options: &[Option<&GrammarNode>],
    ) -> Result<String, &'static str> {
        const SIZE_LIMIT: &str = "PDF stream exceeds the decoded size limit";
        let encoded_size = source.chars().count();
        if self.decoded_bytes >= MAXIMUM_DECODED_BYTES
            || !document_stream_fits_budget(encoded_size, 0, self.decoded_bytes)
            || kinds.len() > 8
        {
            return Err(SIZE_LIMIT);
        }
        let mut encoded = binary_string_bytes(source).ok_or("invalid binary PDF stream")?;
        for (index, &kind) in kinds.iter().enumerate() {
            let limit = MAXIMUM_STREAM_BYTES.min(MAXIMUM_DECODED_BYTES - self.decoded_bytes);
            if limit == 0 {
                return Err(SIZE_LIMIT);
            }
            encoded = match kind {
                1 => decompress_to_vec_zlib_with_limit(&encoded, limit)
                    .map_err(|_| "invalid or oversized FlateDecode PDF stream")?,
                2 => decode_ascii_hex(&encoded)?,
                3 => decode_ascii85(&encoded, limit)?,
                _ => {
                    let parameter = self.get(options.get(index).copied().flatten(), "EarlyChange");
                    let early_change = if let Some(node) = parameter {
                        let change = text_of(self.text, Some(node)).parse::<i64>();
                        match (node.term, change) {
                            ("integer", Ok(0)) => 0,
                            ("integer", Ok(1)) => 1,
                            _ => return Err("invalid LZWDecode PDF EarlyChange"),
                        }
                    } else {
                        1
                    };
                    decode_lzw(&encoded, early_change, limit)?
                }
            };
            if !document_stream_fits_budget(0, encoded.len(), self.decoded_bytes) {
                return Err(SIZE_LIMIT);
            }
            self.decoded_bytes += encoded.len();
        }
        Ok(binary_string(&encoded))
    }
}

fn decode_lzw(source: &[u8], early_change: usize, limit: usize) -> Result<Vec<u8>, &'static str> {
    const INVALID: &str = "invalid or oversized LZWDecode PDF stream";
    const DICTIONARY_SIZE: usize = 4096;
    let mut prefix = [0_u16; DICTIONARY_SIZE];
    let mut suffix = [0_u8; DICTIONARY_SIZE];
    let mut sequence = [0_u8; DICTIONARY_SIZE];
    let mut output = Vec::new();
    let mut next = 258;
    let mut width = 9;
    let mut previous = None;
    let mut bits = 0;
    let mut accumulator = 0_u32;
    let mut bytes = source.iter();
    loop {
        while bits < width {
            accumulator = (accumulator << 8) | u32::from(*bytes.next().ok_or(INVALID)?);
            bits += 8;
        }
        bits -= width;
        let code = ((accumulator >> bits) & ((1 << width) - 1)) as usize;
        if code == 256 {
            next = 258;
            width = 9;
            previous = None;
            continue;
        }
        if code == 257 {
            return Ok(output);
        }
        if code > next || (code >= 258 && previous.is_none()) {
            return Err(INVALID);
        }
        // The next entry is the previous entry followed by its first byte.
        let next_entry = code == next;
        let mut entry = if next_entry {
            previous.ok_or(INVALID)?
        } else {
            code
        };
        let mut length = usize::from(next_entry);
        while entry >= 258 {
            sequence[length] = suffix[entry];
            length += 1;
            entry = usize::from(prefix[entry]);
        }
        sequence[length] = entry as u8;
        length += 1;
        if next_entry {
            sequence[0] = entry as u8;
        }
        if output.len() + length > limit {
            return Err(INVALID);
        }
        output.extend(sequence[..length].iter().rev().copied());
        if let Some(previous) = previous.filter(|_| next < DICTIONARY_SIZE) {
            prefix[next] = previous as u16;
            suffix[next] = entry as u8;
            next += 1;
            width = pdf_lzw_code_width(width, next, early_change);
        }
        previous = Some(code);
    }
}

fn decode_ascii_hex(source: &[u8]) -> Result<Vec<u8>, &'static str> {
    let digits = source
        .iter()
        .copied()
        .filter(|byte| !pdf_whitespace(*byte))
        .map(char::from)
        .collect::<String>();
    let digits = digits.strip_suffix('>').unwrap_or(&digits);
    if !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("invalid ASCIIHexDecode PDF stream");
    }
    Ok(hex_bytes(digits))
}

fn decode_ascii85(source: &[u8], limit: usize) -> Result<Vec<u8>, &'static str> {
    let digits = source
        .iter()
        .copied()
        .filter(|byte| !pdf_whitespace(*byte))
        .collect::<Vec<_>>();
    let digits = digits.strip_prefix(b"<~").unwrap_or(&digits);
    let digits = digits.strip_suffix(b"~>").unwrap_or(digits);
    let mut result = Vec::new();
    let mut value = 0u64;
    let mut count = 0;
    let mut append = |value, count| {
        let value =
            u32::try_from(value).map_err(|_| "invalid or oversized ASCII85Decode PDF stream")?;
        if result.len() + count > limit + 1 {
            return Err("invalid or oversized ASCII85Decode PDF stream");
        }
        result.extend_from_slice(&value.to_be_bytes()[..count]);
        Ok(())
    };
    for &digit in digits {
        if digit == b'z' && count == 0 {
            append(0, 4)?;
        } else {
            if !(b'!'..=b'u').contains(&digit) {
                return Err("invalid ASCII85Decode PDF stream");
            }
            value = value * 85 + u64::from(digit - 33);
            count += 1;
            if count == 5 {
                append(value, 4)?;
                value = 0;
                count = 0;
            }
        }
    }
    if count == 1 {
        return Err("incomplete ASCII85Decode PDF stream");
    }
    if count > 1 {
        for _ in count..5 {
            value = value * 85 + 84;
        }
        append(value, count - 1)?;
    }
    Ok(result)
}
