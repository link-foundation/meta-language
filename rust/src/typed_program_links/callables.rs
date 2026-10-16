//! Mechanical callable-owner ordinals over the actual record and descriptors.

use serde_json::Value;

pub(super) fn parameter_ordinals(value: &Value) -> Vec<usize> {
    let facts = crate::translation::surface_reference_facts::create_surface_reference_facts(value);
    crate::translation::callable_source_rules::plan_callable_parameter_ownership(
        &facts.starts,
        &facts.counts,
        &facts.labels,
        &facts.targets,
        facts.root,
    )
    .chunks_exact(3)
    .map(|owner| {
        owner[1]
            .to_string()
            .parse()
            .expect("a callable plan retains the original parameter ordinal")
    })
    .collect()
}
