#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::MODEL;
    use crate::{
        translation::{check::check_program, surface::SProgram},
        typed_program_links::{decode_typed_program_links, encode_typed_program_links},
    };

    #[test]
    fn clear_keeps_the_same_checked_facet_through_links() {
        let vector =
            json!({"kind": "array", "element": {"kind": "fixed", "bits": 32, "signed": false}});
        let unit = json!({"kind": "unit"});
        let span = json!({"start": 3, "end": 13});
        let element = json!({"kind": "primitive", "path": ["u32"], "arguments": []});
        let position =
            |ty, owner| json!({"type": ty, "owner": owner, "wrappers": [], "lifetimes": []});
        let helper = json!({
            "k": "fn", "name": "clear_vector", "generated": true, "span": span,
            "params": [{"name": "input", "type": vector}],
            "ret": {"kind": "tuple", "elements": [unit, vector]},
            "body": {"k": "tuple", "items": [{"k": "unit"}, {"k": "array", "items": []}]},
            "sourceVectorMutation": {
                "model": MODEL, "operation": "clear", "span": span,
                "parameters": [position(vector.clone(), json!({
                    "kind": "library", "path": ["std", "vec", "Vec"], "arguments": [element]
                }))],
                "element": position(vector["element"].clone(), element),
                "result": position(unit, json!({"kind": "unit", "path": [], "arguments": []}))
            }
        });
        let surface = |helper| {
            serde_json::from_value::<SProgram>(json!({"language": "Rust", "items": [helper]}))
                .expect("complete generated Vec helper surface")
        };
        let checked = check_program(&surface(helper.clone())).expect("actual clear");
        let links = encode_typed_program_links(&checked).expect("complete Vec facet");
        let replay =
            decode_typed_program_links(&links.network, links.root).expect("rechecked Vec facet");
        assert_eq!(replay, checked);

        let mut changed = helper;
        changed["body"]["items"][1] = json!({"k": "name", "path": ["input"]});
        assert!(check_program(&surface(changed)).is_err());
    }
}
