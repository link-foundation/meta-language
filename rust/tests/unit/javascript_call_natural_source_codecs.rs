// Generated from actual checked source declarations and target mappings.
// Transport only; no source operation semantics are implemented here.
use super::native;
use serde_json::{Value as Data, json};
fn wire_number(value: f64) -> Data {
    if value.to_bits() == 0x8000_0000_0000_0000 {
        return Data::Number(serde_json::Number::from_f64(value).unwrap());
    }
    Data::Number(value.to_string().parse().expect("finite model Number"))
}
fn wire_text_units(value: &Data) -> Vec<u16> {
    let units = if let Some(text) = value.as_str() {
        text.encode_utf16().collect::<Vec<u16>>()
    } else {
        let object = value.as_object().expect("qualified model text units");
        assert_eq!(object.len(), 2, "exact model text keys");
        assert_eq!(
            object.get("textEncoding").and_then(Data::as_str),
            Some("UTF16 code units 1"),
            "model text encoding"
        );
        let input = object
            .get("units")
            .and_then(Data::as_array)
            .expect("model text units");
        assert!(input.len() <= 16777216, "model text unit budget");
        input
            .iter()
            .map(|unit| {
                u16::try_from(unit.as_u64().expect("model UTF16 unit"))
                    .expect("model UTF16 unit range")
            })
            .collect()
    };
    assert!(units.len() <= 16777216, "model text unit budget");
    units
}
fn wire_text(units: &[u16]) -> Data {
    assert!(units.len() <= 16777216, "model text unit budget");
    match String::from_utf16(units) {
        Ok(text) => Data::String(text),
        Err(_) => json!({"textEncoding":"UTF16 code units 1","units":units}),
    }
}
fn wire_undefined() -> Data {
    json!({"fieldPresence":"undefined"})
}
fn is_wire_undefined(value: &Data) -> bool {
    value.as_object().is_some_and(|fields| {
        fields.len() == 1 && fields.get("fieldPresence").and_then(Data::as_str) == Some("undefined")
    })
}

#[path = "javascript_call_natural_source_codecs/part_1.rs"]
mod part_1;
pub(super) use part_1::*;
#[path = "javascript_call_natural_source_codecs/part_2.rs"]
mod part_2;
pub(super) use part_2::*;
#[path = "javascript_call_natural_source_codecs/part_3.rs"]
mod part_3;
pub(super) use part_3::*;
#[path = "javascript_call_natural_source_codecs/part_4.rs"]
mod part_4;
pub(super) use part_4::*;
