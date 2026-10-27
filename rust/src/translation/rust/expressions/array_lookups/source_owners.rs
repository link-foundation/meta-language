//! Bounded original owner-tree transport through shared registered edges only.
use crate::translation::rust_pointer_integer_owner_rules::accept_rust_pointer_integer_owner_child;
use crate::translation::source_signatures::SourceOwner;
pub(super) fn element_owner(mut owner: Option<&SourceOwner>) -> Option<&SourceOwner> {
    for _ in 0..128 {
        let value = owner?;
        if !accept_rust_pointer_integer_owner_child(
            &value.kind,
            &value.path.join("::"),
            super::count(value.arguments.len()),
        ) {
            return owner;
        }
        owner = value.arguments.first();
    }
    None
}

pub(super) fn clone_facts(
    element: Option<&super::Type>,
    owner: Option<&SourceOwner>,
    cloned: bool,
) -> (String, bool) {
    if !cloned {
        return (String::new(), false);
    }
    let (bits, signed) = match element {
        Some(super::Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
        _ => (String::new(), false),
    };
    let error =
        crate::translation::rust_array_lookup_rules::read_rust_array_lookup_cloned_element_error(
            element.map_or("", super::Type::kind),
            &bits,
            signed,
            owner.map_or("", |owner| owner.kind.as_str()),
            &owner.map_or_else(String::new, |owner| owner.path.join("::")),
            owner.map_or(-1.0, |owner| super::count(owner.arguments.len())),
        );
    let string = element.is_some_and(|element| element.kind() == "string") && error.is_empty();
    (error, string)
}
pub(super) fn scalar_owner(source: &super::SourceOrigin) -> String {
    if let Some(expanded) = &source.expanded_source_owner {
        super::SourceOrigin {
            source_owner: Some(expanded.clone()),
            ..super::SourceOrigin::unknown()
        }
        .pointer_integer_owner()
    } else {
        source.pointer_integer_owner()
    }
}
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
