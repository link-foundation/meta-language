#[cfg(test)]
mod deferred_controls {
    use super::*;
    #[test]
    fn actual_nominal_function_fields_require_acyclic_constructive_results() {
        let mut data:DataDecl=serde_json::from_value(serde_json::json!({"name":"Callback","fullName":"Callback","modulePath":[],"ctors":[{"name":"Value","fields":[{"name":"read","type":{"kind":"function","params":[{"kind":"bool"}],"ret":{"kind":"bool"}}}]}]})).unwrap();
        let ty = Type::Data {
            name: "Callback".into(),
        };
        let facts = read_callable_data_facts(&ty, &|name| {
            if name == "Callback" {
                Some(&data)
            } else {
                None
            }
        })
        .unwrap();
        assert!(facts.callable && facts.constructive && facts.acyclic);
        data.ctors[0].fields[0].ty = Type::Function {
            params: vec![ty.clone()],
            ret: Box::new(Type::Bool),
        };
        let facts = read_callable_data_facts(&ty, &|name| {
            if name == "Callback" {
                Some(&data)
            } else {
                None
            }
        })
        .unwrap();
        assert!(facts.callable && !facts.acyclic);
        assert!(
            !rules::read_callable_data_profile_error(
                facts.callable,
                facts.acyclic,
                facts.constructive
            )
            .is_empty()
        );
    }
}
