#[cfg(test)]
mod tests {
    use super::rules;
    #[test]
    fn complete_groups_retain_atomic_tokens_and_refuse_invalid_boundaries() {
        let data = crate::read_structured_data_lino(include_str!(
            "../../../../src/data/rust-declaration-header-controls.lino"
        ))
        .expect("authored header controls");
        for row in data["parenthesisCases"]
            .as_array()
            .expect("parenthesis controls")
        {
            let syntax = row["syntax"]
                .as_array()
                .expect("actual syntax roles")
                .iter()
                .map(|value| value.as_str().expect("actual role").to_owned())
                .collect::<Vec<_>>();
            assert_eq!(crate::translation::rust_contextual_closure_rules::find_rust_contextual_closure_parenthesis_count(&syntax), row["depth"].as_f64().expect("independent prefix depth"));
        }
        for row in data["headerCases"]
            .as_array()
            .expect("complete header cases")
        {
            let kinds = row["kinds"]
                .as_array()
                .expect("actual token kinds")
                .iter()
                .map(|value| value.as_str().expect("kind").to_owned())
                .collect::<Vec<_>>();
            let values = row["values"]
                .as_array()
                .expect("actual token values")
                .iter()
                .map(|value| value.as_str().expect("value").to_owned())
                .collect::<Vec<_>>();
            assert_eq!(
                rules::find_rust_declaration_body_boundary(&kinds, &values),
                row["end"].as_f64().expect("independent boundary")
            );
        }
    }
}
