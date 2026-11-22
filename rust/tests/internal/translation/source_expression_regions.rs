#[cfg(test)]
mod source_controls {
    use super::*;
    use serde_json::{Value, json};
    fn fixture() -> Value {
        let scalar = json!({"kind":"fixed","bits":8,"signed":false});
        let owner = json!({"kind":"primitive","path":["u8"],"arguments":[]});
        let element =
            json!({"type":scalar,"owner":owner,"wrappers":["reference"],"lifetimes":["a"]});
        let array = json!({"kind":"array","element":scalar,"reference":false});
        let result = json!({"type":array,"owner":{"kind":"library","path":["std","vec","Vec"],"arguments":[owner]},"wrappers":[],"lifetimes":[],"arrayElementPosition":element});
        json!({"schemaVersion":1,"sourceLanguage":"Rust","items":[],"main":{"effects":[]},"declarations":[{"k":"fn","name":"children","fullName":"children","modulePath":[],"params":[{"name":"value","type":scalar}],"ret":array,"recursive":false,"decreasing":null,"sourceSignature":{"model":MODEL,"language":"Rust","parameters":[element],"result":result,"lifetimeBindings":["a"]},"body":{"k":"array","type":array,"sourcePosition":{"model":"Rust borrowed expression source position 1","position":result},"items":[{"k":"var","name":"value","type":scalar,"sourcePosition":{"model":"Rust borrowed expression source position 1","position":element}}]}}]})
    }
    #[test]
    fn source_free_reload_uses_actual_binder() {
        let program: Program =
            serde_json::from_value(fixture()).expect("authored checked source fixture");
        validate_borrowed_vector_source_regions(&program).expect("actual parameter loan");
    }
    #[test]
    fn forged_static_body_cannot_upgrade_parameter() {
        let mut source = fixture();
        source["declarations"][0]["body"]["items"][0]["sourcePosition"]["position"]["lifetimes"] =
            json!(["static"]);
        let program: Program = serde_json::from_value(source).expect("authored source fixture");
        assert!(validate_borrowed_vector_source_regions(&program).is_err());
    }
    #[test]
    fn unknown_actual_binder_refuses() {
        let mut source = fixture();
        source["declarations"][0]["body"]["items"][0]["name"] = json!("unbound");
        let program: Program = serde_json::from_value(source).expect("authored source fixture");
        assert!(validate_borrowed_vector_source_regions(&program).is_err());
    }
    #[test]
    fn assertion_only_facet_does_not_bypass_detection() {
        let mut source = fixture();
        let expression = source["declarations"][0]["body"]["items"][0].clone();
        source["declarations"] = json!([]);
        source["main"]["effects"] = json!([{"k":"assert","prop":{"p":"bool","expr":expression}}]);
        let program: Program = serde_json::from_value(source).expect("authored source fixture");
        assert!(validate_borrowed_vector_source_regions(&program).is_err());
    }
    #[test]
    fn runtime_shell_conflict_refuses_before_body_proof() {
        let mut source = fixture();
        source["sourceRuntimeShell"] = json!({});
        let program: Program = serde_json::from_value(source).expect("authored source fixture");
        assert!(validate_borrowed_vector_source_regions(&program).is_err());
    }
    fn match_fixture() -> Value {
        let mut source = fixture();
        let scalar = json!({"kind":"fixed","bits":8,"signed":false});
        let owner = json!({"kind":"primitive","path":["u8"],"arguments":[]});
        let field = json!({"type":scalar,"owner":owner,"wrappers":[],"lifetimes":[]});
        let data = json!({"kind":"data","name":"Node"});
        let parameter = json!({"type":data,"wrappers":["reference"],"lifetimes":["a"],"owner":{"kind":"declaration","path":["Node"],"arguments":[]}});
        source["declarations"][0]["params"][0]["type"] = data.clone();
        source["declarations"][0]["sourceSignature"]["parameters"] = json!([parameter]);
        let mut body = source["declarations"][0]["body"].clone();
        body["items"][0]["name"] = json!("child");
        source["declarations"][0]["body"] = json!({"k":"match","type":body["type"],"sourcePosition":body["sourcePosition"],"scrutinee":{"k":"var","name":"value","type":data,"sourcePosition":{"model":"Rust borrowed expression source position 1","position":parameter}},"cases":[{"pattern":{"k":"ctor","data":"Node","ctor":"Node","binds":["child"]},"body":body}]});
        source["declarations"].as_array_mut().expect("authored declarations").push(json!({"k":"data","name":"Node","fullName":"Node","modulePath":[],"ctors":[{"name":"Node","fields":[{"name":"value","type":scalar}]}],"sourceFieldSignatures":{"model":"Rust data type field signatures 1","language":"Rust","dataTypeName":"Node","ctors":[{"name":"Node","fields":[field]}]}}));
        source
    }
    #[test]
    fn borrowed_nominal_match_preserves_parent_region() {
        let program: Program =
            serde_json::from_value(match_fixture()).expect("authored source fixture");
        validate_borrowed_vector_source_regions(&program).expect("actual borrowed field region");
    }
    #[test]
    fn unknown_pattern_field_binder_cannot_create_loan() {
        let mut source = match_fixture();
        source["declarations"][0]["body"]["cases"][0]["body"]["items"][0]["name"] =
            json!("missing");
        let program: Program = serde_json::from_value(source).expect("authored source fixture");
        assert!(validate_borrowed_vector_source_regions(&program).is_err());
    }
    #[test]
    fn standard_box_projection_preserves_parameter_region() {
        let mut source = fixture();
        source["declarations"][0]["sourceSignature"]["parameters"][0]["wrappers"] =
            json!(["reference", "box"]);
        source["declarations"][0]["sourceSignature"]["parameters"][0]["owner"] = json!({"kind":"library","path":["std","boxed","Box"],"arguments":[{"kind":"primitive","path":["u8"],"arguments":[]}]});
        let program: Program = serde_json::from_value(source).expect("authored source fixture");
        validate_borrowed_vector_source_regions(&program)
            .expect("actual canonical Box parameter region");
    }
}

#[cfg(test)]
mod state_controls {
    use super::*;
    use serde_json::{Value, json};

    fn position() -> Value {
        json!({
            "type":{"kind":"fixed","bits":8,"signed":false},
            "owner":{"kind":"primitive","path":["u8"],"arguments":[]},
            "wrappers":[],
            "lifetimes":[]
        })
    }

    fn transition(operation: &str, numeral: &str) -> Value {
        let position = position();
        json!({
            "k":"lit","type":position["type"],"value":numeral,
            "sourceStateTransition":{
                "model":"Rust source state transition 1",
                "operation":operation,
                "binder":{"name":"count","identity":"actual count","start":18,"end":23},
                "position":position
            }
        })
    }

    fn binding(name: &str, value: Value) -> Value {
        json!({"k":"let","name":name,"value":value})
    }

    fn inspect(effects: Vec<Value>) -> super::super::diagnostics::Result<()> {
        let program = serde_json::from_value(json!({
            "schemaVersion":1,"sourceLanguage":"Rust","items":[],"declarations":[],
            "main":{"effects":effects}
        }))
        .expect("well-shaped checked source control");
        validate_borrowed_vector_source_regions(&program)
    }

    #[test]
    fn only_the_current_source_storage_revision_is_readable() {
        let initial = binding("count", transition("initialize", "1"));
        let write = binding("updated", transition("write", "2"));
        let print = |name: &str| json!({"k":"print","expr":{"k":"var","name":name,"type":position()["type"]}});
        inspect(vec![initial.clone(), write.clone(), print("updated")])
            .expect("the actual published state is available");
        let error = inspect(vec![initial, write, print("count")])
            .expect_err("the superseded lexical value cannot supply a current state read");
        assert!(
            error
                .reason
                .contains("actual live selected storage revision")
        );
    }

    #[test]
    fn a_forged_storage_binder_cannot_publish_a_write() {
        let mut write = transition("write", "2");
        write["sourceStateTransition"]["binder"]["identity"] = json!("other count");
        let error = inspect(vec![
            binding("count", transition("initialize", "1")),
            binding("updated", write),
        ])
        .expect_err("the write must name its actual initialized binder");
        assert!(error.reason.contains("actual initialized binder"));
    }

    #[test]
    fn a_diverging_alternative_cannot_poison_the_continuing_state() {
        let scalar = position()["type"].clone();
        let reselect = json!({
            "k":"if","type":scalar,
            "cond":{"k":"lit","type":{"kind":"bool"},"value":false},
            "then":{"k":"let","name":"discarded","type":scalar,
                "value":transition("write","2"),
                "body":{"k":"abort","type":scalar,"message":"halt"}},
            "else":{"k":"var","name":"count","type":scalar}
        });
        inspect(vec![
            binding("count", transition("initialize", "1")),
            binding("result", reselect),
            json!({"k":"print","expr":{"k":"var","name":"count","type":scalar}}),
        ])
        .expect("the original storage survives its continuing alternative");
    }
}
