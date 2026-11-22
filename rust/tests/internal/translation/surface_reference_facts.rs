#[cfg(test)]
mod tests {
    use super::create_surface_reference_facts_with_shared_scalars;

    #[test]
    fn shared_scalars_keep_distinct_allocation_facts() {
        let graph = serde_json::json!([
            ["referenceAllocate", null, true, -0.0, 0.0],
            ["referenceAllocate", null, true, -0.0, 0.0]
        ]);
        let facts = create_surface_reference_facts_with_shared_scalars(&graph);
        assert_eq!(
            facts.forms,
            [
                "array", "array", "string", "null", "boolean", "number", "number", "array"
            ]
        );
        assert_eq!(
            facts.targets,
            [2.0, 3.0, 4.0, 5.0, 6.0, 2.0, 3.0, 4.0, 5.0, 6.0, 1.0, 7.0]
        );
    }
}
