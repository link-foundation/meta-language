//! Source-free function selection under the existing checked primitive model.
use meta_language::{
    FormalConceptArchive, FormalFunctionSelector, LinkId, LinkNetwork, decode_typed_program_links,
    encode_formal_function_definition, prove_formal_concept_equivalence,
    read_formal_concept_definition, read_structured_data_lino, verify_formal_concept_equivalence,
};
use serde_json::{Value, json};

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/formal-function-definitions.lino"
    ))
    .unwrap()
}
fn archive(value: &Value) -> FormalConceptArchive {
    FormalConceptArchive::from_json(value).unwrap()
}
fn program(value: &Value) -> (LinkNetwork, LinkId) {
    (
        LinkNetwork::from_lino(value["programLinks"].as_str().unwrap()).unwrap(),
        LinkId::from_u64(value["programRoot"].as_u64().unwrap()),
    )
}
fn declaration(network: &LinkNetwork, root: LinkId, name: &str) -> LinkId {
    let body = network.link(root).unwrap().references()[0];
    let schema = read_structured_data_lino(include_str!(
        "../../src/data/typed-program-links-schema.lino"
    ))
    .unwrap();
    let label = schema["fields"]["declarations"].as_str().unwrap();
    let mapping = network
        .links()
        .find_map(|link| {
            let [parent, field, child] = link.references() else {
                return None;
            };
            (*parent == body && network.link(*field).unwrap().metadata().term() == Some(label))
                .then_some(*child)
        })
        .unwrap();
    network
        .link(mapping)
        .unwrap()
        .references()
        .iter()
        .find_map(|id| {
            let [key, value] = network.link(*id).unwrap().references() else {
                return None;
            };
            (network.link(*key).unwrap().metadata().definition() == Some(name)).then_some(*value)
        })
        .unwrap()
}

#[test]
fn native_function_selection_preserves_original_ids_and_matches_shared_formal_matrices() {
    let data = fixture();
    for case in data["cases"].as_array().unwrap() {
        let (network, root) = program(case);
        let name = case["declaration"].as_str().unwrap();
        let id = declaration(&network, root, name);
        let provenance = json!({"source": case["name"], "sourceLanguage": case["language"]});
        let selected = encode_formal_function_definition(
            &network,
            root,
            FormalFunctionSelector::Identity(id),
            Some(&provenance),
        )
        .unwrap();
        assert_eq!(
            read_formal_concept_definition(
                &json!({"links": selected.network.to_lino(), "root": selected.root.as_u64()})
            )
            .unwrap(),
            case["expected"]
        );
        let typed_root = selected.network.link(selected.root).unwrap().references()[0];
        assert_eq!(declaration(&selected.network, typed_root, name), id);
        for link in network.links() {
            assert_eq!(
                selected.network.link(link.id()).unwrap().references(),
                link.references()
            );
        }
        let decoded = decode_typed_program_links(&selected.network, typed_root).unwrap();
        assert!(
            decoded
                .declarations
                .iter()
                .any(|entry| entry.full_name() == "selected_function_definition")
        );
        assert!(
            !decoded
                .declarations
                .iter()
                .any(|entry| entry.full_name() == "unused")
        );
        assert!(decoded.main.unwrap().effects.is_empty());
    }
}

#[test]
fn mutual_recursion_and_context_independence_have_closed_reloaded_certificates() {
    let data = fixture();
    for pair in data["equivalent"].as_array().unwrap() {
        let first = archive(&data["cases"][pair[0].as_u64().unwrap() as usize]);
        let second = archive(&data["cases"][pair[1].as_u64().unwrap() as usize]);
        let proof = prove_formal_concept_equivalence(&first, &second).unwrap();
        let restored = FormalConceptArchive {
            network: LinkNetwork::from_lino(&proof.network.to_lino()).unwrap(),
            root: proof.root,
        };
        assert!(verify_formal_concept_equivalence(
            &first, &second, &restored
        ));
    }
    for pair in data["distinct"].as_array().unwrap() {
        let first = archive(&data["cases"][pair[0].as_u64().unwrap() as usize]);
        let second = archive(&data["cases"][pair[1].as_u64().unwrap() as usize]);
        assert!(prove_formal_concept_equivalence(&first, &second).is_none());
    }
}

#[test]
fn foreign_function_id_and_unknown_name_cannot_create_a_formal_basis() {
    let data = fixture();
    let (network, root) = program(&data["cases"][0]);
    assert!(
        encode_formal_function_definition(
            &network,
            root,
            FormalFunctionSelector::Identity(root),
            None
        )
        .unwrap_err()
        .contains("owned function declaration")
    );
    assert!(
        encode_formal_function_definition(
            &network,
            root,
            FormalFunctionSelector::Name("not declared"),
            None
        )
        .unwrap_err()
        .contains("owned function declaration")
    );
}
