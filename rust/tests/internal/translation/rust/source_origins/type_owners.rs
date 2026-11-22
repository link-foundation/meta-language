#[cfg(test)]
mod deferred_controls {
    use super::super::SourceOrigin;
    use super::*;
    fn names(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }
    #[test]
    fn independent_namespace_entries_survive_both_source_orders_and_passes() {
        for reverse in [false, true] {
            let mut first = SourceOrigins::new(None, "");
            let function = SourceDeclaration::Function {
                parameters: Vec::new(),
                result: SourceOrigin::value(),
            };
            let entries = if reverse {
                vec![function, SourceDeclaration::Module]
            } else {
                vec![SourceDeclaration::Module, function]
            };
            for entry in entries {
                first.declaration("std", &[], entry).unwrap();
            }
            let mut next = SourceOrigins::new(Some(first.catalog), "");
            next.type_catalog = first.type_catalog;
            next.value_catalog = first.value_catalog;
            assert!(matches!(
                next.lookup_type_owner(&names(&["std"]), &[])
                    .unwrap()
                    .declaration,
                SourceDeclaration::Module
            ));
            assert!(matches!(
                next.lookup_value_owner(&names(&["std"]), &[])
                    .unwrap()
                    .declaration,
                SourceDeclaration::Function { .. }
            ));
        }
    }
    #[test]
    fn nearer_type_prefix_stops_outer_child_and_data_filter() {
        let mut origins = SourceOrigins::new(None, "");
        origins.register_catalog_entry("X".into(), SourceDeclaration::Data);
        origins.register_catalog_entry(
            "X.C".into(),
            SourceDeclaration::Constructor {
                owner: "X".into(),
                fields: Vec::new(),
                result: SourceOrigin::value(),
            },
        );
        origins.register_catalog_entry("inner.X".into(), SourceDeclaration::Module);
        assert!(
            origins
                .lookup_literal_match_owner(&names(&["X"]), &names(&["inner"]))
                .is_none()
        );
        assert!(
            origins
                .lookup_value_owner(&names(&["X", "C"]), &names(&["inner"]))
                .is_none()
        );
        assert!(matches!(
            origins
                .lookup_type_owner(&names(&["X"]), &names(&["unrelated"]))
                .unwrap()
                .declaration,
            SourceDeclaration::Data
        ));
    }
}
