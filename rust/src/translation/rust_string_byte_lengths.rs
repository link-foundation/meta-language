//! Actual checked operands into the shared scalar-string byte-count policy.
use super::diagnostics::{Result, unsupported};
use super::ir::{Expr, Node};
use super::{Language, rust_string_byte_length_rules as rules};
pub(super) fn validate(value: &Expr, language: Language, utf16: bool) -> Result<()> {
    let Node::Math { args, .. } = &value.node else {
        unreachable!("actual byte-count operation");
    };
    let error = rules::read_rust_string_byte_length_argument_error(
        language.as_str(),
        utf16,
        serde_json::Number::from(args.len())
            .as_f64()
            .expect("actual argument count"),
        false,
        args.first().map_or("", |arg| arg.ty.kind()),
    );
    if error.is_empty() {
        Ok(())
    } else {
        Err(unsupported("Rust String byte length", error, value.span))
    }
}

#[cfg(test)]
mod tests {
    use super::rules;
    #[test]
    fn scalar_measure_preserves_independent_byte_counts_and_encoding_refusals() {
        let data = crate::read_structured_data_lino(include_str!(
            "../data/rust-string-byte-length-controls.lino"
        ))
        .expect("authored scalar controls");
        for row in data["normCases"].as_array().expect("scalar cases") {
            let units: Vec<f64> = row["units"]
                .as_array()
                .expect("complete units")
                .iter()
                .map(|unit| unit.as_f64().expect("numeric unit"))
                .collect();
            let expected = row["value"].as_str().expect("independent decimal count");
            assert_eq!(
                rules::measure_rust_utf8_byte_length(&units).to_string(),
                expected
            );
            assert_eq!(
                row["text"].as_str().expect("scalar text").len().to_string(),
                expected
            );
        }
        for row in data["invalidUnits"].as_array().expect("invalid encodings") {
            let units: Vec<f64> = row
                .as_array()
                .expect("units")
                .iter()
                .map(|unit| unit.as_f64().expect("numeric unit"))
                .collect();
            let error =
                match std::panic::catch_unwind(|| rules::measure_rust_utf8_byte_length(&units)) {
                    Ok(_) => panic!("invalid encoding acquired a scalar interpretation"),
                    Err(error) => error,
                };
            let (class, message, present) = error
                .downcast_ref::<(String, Vec<u16>, bool)>()
                .expect("generated source exception transport");
            assert_eq!(class, "TypeError");
            assert!(*present);
            assert!(
                String::from_utf16(message)
                    .expect("exception message")
                    .contains("a UTF-8 byte count")
            );
        }
    }
}
