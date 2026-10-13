//! Adapt the existing JavaScript canonical observation contract to serde.
//! Probe record keys are fixed ASCII names; associative project data keeps
//! its original order, and arrays keep every occurrence in source order.
use serde_json::Value;

pub(super) fn serialize_canonical_observation(value: &Value) -> String {
    let mut canonical = value.clone();
    canonical.sort_all_objects();
    serde_json::to_string(&canonical).expect("an observation value serializes")
}

pub(super) fn sort_observation_values(mut values: Vec<Value>) -> Vec<Value> {
    // The JavaScript fact sorter compares canonical strings as UTF-16 code
    // units. Structural signature strings use UTF-8 ordering elsewhere.
    values.sort_by_cached_key(|value| {
        serialize_canonical_observation(value)
            .encode_utf16()
            .collect::<Vec<_>>()
    });
    values
}
