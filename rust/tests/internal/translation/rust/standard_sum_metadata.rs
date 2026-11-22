#[cfg(test)]
mod deferred_controls {
    use super::*;
    fn unit() -> SourceOrigin {
        let mut value = SourceOrigin::with_type(Type::Unit, &[], Vec::new());
        value.source_owner = Some(SourceOwner {
            kind: "unit".into(),
            path: Vec::new(),
            arguments: Vec::new(),
        });
        value
    }
    #[test]
    fn independently_authored_semantic_projection_costs() {
        let mut value = unit();
        let cost = measure_origins(&[&value]).unwrap();
        assert_eq!((cost.nodes, cost.units), (20, 203));
        value.path = vec!["𐐀".into()];
        let cost = measure_origins(&[&value]).unwrap();
        assert_eq!((cost.nodes, cost.units), (21, 205));
        value.path.clear();
        value.expanded_source_owner = value.source_owner.clone();
        let cost = measure_origins(&[&value]).unwrap();
        assert_eq!((cost.nodes, cost.units), (23, 224));
    }
    #[test]
    fn copy_reservation_and_depth_boundaries() {
        for (nodes, units, accepted) in [
            (65536, 0, true),
            (65537, 0, false),
            (0, 524288, true),
            (0, 524289, false),
        ] {
            assert_eq!(
                MetadataBudget::default()
                    .reserve(MetadataCost { nodes, units })
                    .is_ok(),
                accepted
            );
        }
        let mut budget = MetadataBudget {
            nodes: 1048560,
            units: 0,
            error: None,
        };
        assert!(budget.reserve(MetadataCost { nodes: 1, units: 0 }).is_ok());
        assert_eq!(budget.nodes, 1048576);
        assert!(budget.reserve(MetadataCost { nodes: 1, units: 0 }).is_err());
        assert!(
            rules::read_rust_standard_sum_source_metadata_count_error(1.0, 0.0, 256.0).is_empty()
        );
        assert!(
            !rules::read_rust_standard_sum_source_metadata_count_error(1.0, 0.0, 257.0).is_empty()
        );
    }
}
