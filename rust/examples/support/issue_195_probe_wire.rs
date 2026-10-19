//! Streaming JSON at the explicit runtime-probe process boundary.

use std::io::{self, Write};

use serde_json::{Value, json};

/// Write each section entry separately in the probe's external wire protocol.
pub fn write_runtime_probe_wire_records(
    output: &mut impl Write,
    fixture_digest: &str,
    sections: &[(&str, Vec<Value>)],
) -> io::Result<()> {
    let mut record = |section: &str, index: Value, value: &Value| {
        serde_json::to_writer(
            &mut *output,
            &json!({ "section": section, "index": index, "value": value }),
        )
        .map_err(io::Error::other)?;
        output.write_all(b"\n")
    };
    record("schemaVersion", Value::Null, &json!(1))?;
    record("fixtureDigest", Value::Null, &json!(fixture_digest))?;
    for (section, entries) in sections {
        for (index, value) in entries.iter().enumerate() {
            record(section, json!(index), value)?;
        }
    }
    Ok(())
}
