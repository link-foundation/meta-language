#[cfg(test)]
mod tests {
    #[test]
    fn original_container_owner_projection_retains_every_registered_edge_and_depth_bound() {
        let data = crate::project_data::read_project_data_file(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../parity/fixtures/rust-array-cloned-lookup.lino"),
        )
        .unwrap();
        for row in data["leafProjections"].as_array().unwrap() {
            let mut owner: crate::translation::source_signatures::SourceOwner =
                serde_json::from_value(row["leaf"].clone()).unwrap();
            let wrappers = row["wrappers"].as_array().unwrap();
            assert_eq!(wrappers.len() as u64, row["depth"].as_u64().unwrap());
            for wrapper in wrappers {
                let mut outer: crate::translation::source_signatures::SourceOwner =
                    serde_json::from_value(wrapper.clone()).unwrap();
                outer.arguments = vec![owner];
                owner = outer;
            }
            let actual = super::element_owner(Some(&owner));
            assert_eq!(serde_json::to_value(actual).unwrap(), row["expected"]);
        }
    }
}
