use meta_language::translation::rust::parse_rust_with_fn_mut_source_model;
use meta_language::translation::surface::{SItem, SNode};

#[test]
fn direct_callback_calls_replay_every_call_in_the_original_declaration() {
    let source = "
        fn transform(value: u8, mut map: impl FnMut(u8) -> u8) -> u8 {
            map(map(value))
        }
        fn main() {
            assert!(transform(2u8, |value| value + 1u8) == 4u8);
        }
    ";
    let surface = parse_rust_with_fn_mut_source_model(
        source,
        "Rust closed source FnMut specializations 1",
        "Rust exclusive source loans 1",
        "1.93.2",
    )
    .unwrap();
    let function = surface.items.iter().find_map(|item| match item {
        SItem::Fn(function) if function.source_fn_mut_specialization.is_some() => Some(function),
        _ => None,
    });
    let function = function.expect("the selected original declaration is specialized");
    let facet = function.source_fn_mut_specialization.as_ref().unwrap();
    assert_eq!(facet.source_version, "1.93.2");
    assert_eq!(facet.declaration.path, ["transform"]);
    assert_eq!(facet.callback.parameter, 1);
    assert!(facet.packet.is_empty());
    assert!(matches!(&function.body.node, SNode::Tuple { items, .. } if items.len() == 1));
    assert!(!surface.items.iter().any(|item| item.name() == "transform"));
}
