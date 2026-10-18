use meta_language::{
    BinaryLinksOptions, FeatureParseOptions, LinkNetwork, OperationValue, SyntaxTree,
    compile_feature_grammar, decode_link_network_binary, decode_structured_data_links,
    encode_link_network_binary, encode_structured_data_links, parse_grammar_links,
};
use serde_json::{Map, Value, json};

#[test]
fn actual_grammar_actions_preserve_attribute_positions_values_and_utf16_rendering() {
    let fixture = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/grammar-attribute-order.lino"
    ))
    .expect("shared attribute order fixture");
    for case in fixture["cases"].as_array().expect("attribute cases") {
        let grammar = parse_grammar_links(case["grammar"].as_str().expect("grammar")).unwrap();
        let parser =
            compile_feature_grammar(&grammar, None, FeatureParseOptions::default()).unwrap();
        let outcome = parser
            .parse_tree(
                case["source"].as_str().unwrap().as_bytes(),
                &FeatureParseOptions::default(),
            )
            .unwrap();
        assert!(outcome.ok, "{:?}", outcome.rejection);
        let tree = outcome.tree.expect("concrete tree");
        let SyntaxTree::Node {
            attributes: Some(attributes),
            ..
        } = &tree
        else {
            panic!("actual root attributes are required");
        };
        assert_eq!(json!(attributes.keys().collect::<Vec<_>>()), case["order"]);
        assert_eq!(tree.render(), case["rendered"].as_str().unwrap());
        let values: Map<String, Value> = attributes
            .iter()
            .map(|(name, value)| {
                let value = match value {
                    OperationValue::Integer(value) => json!(value),
                    OperationValue::Text(value) => json!(value),
                };
                (name.clone(), value)
            })
            .collect();
        let entries = Value::Array(
            values
                .iter()
                .map(|(key, value)| json!([key, value]))
                .collect(),
        );
        assert_eq!(entries, case["entries"]);
        let original = Value::Object(values);
        let graph = encode_structured_data_links(&original).unwrap();
        let binary =
            encode_link_network_binary(&graph.network, BinaryLinksOptions::default()).unwrap();
        for saved in [
            graph.network.clone(),
            LinkNetwork::from_lino(&graph.network.to_lino()).unwrap(),
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ] {
            let restored = decode_structured_data_links(&saved, graph.root).unwrap();
            assert_eq!(restored, original);
            assert_eq!(
                json!(restored.as_object().unwrap().keys().collect::<Vec<_>>()),
                case["order"]
            );
        }
    }
}
