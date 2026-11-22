use meta_language::translation::check::check_program;
use meta_language::translation::rust::parse_rust_with_fn_mut_source_model;
use meta_language::translation::surface::{SItem, SNode};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_typed_program_links_with_rust_fn_mut_source_model,
    decode_link_network_binary, decode_typed_program_links, encode_link_network_binary,
};

const FN_MUT_MODEL: &str = "Rust closed source FnMut specializations 1";
const EXCLUSIVE_MODEL: &str = "Rust exclusive source loans 1";

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
    let surface =
        parse_rust_with_fn_mut_source_model(source, FN_MUT_MODEL, EXCLUSIVE_MODEL, "1.93.2")
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

    let checked = check_program(&surface).unwrap();
    let packet = create_typed_program_links_with_rust_fn_mut_source_model(
        source,
        FN_MUT_MODEL,
        EXCLUSIVE_MODEL,
        "1.93.2",
        None,
    )
    .unwrap();
    let binary =
        encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
    for restored in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
    ] {
        assert_eq!(
            decode_typed_program_links(&restored, packet.root).unwrap(),
            checked
        );
    }
}

#[test]
fn repeated_callbacks_retain_the_original_mutable_capture() {
    let source = "
        fn twice(value: u8, mut map: impl FnMut(u8) -> u8) -> u8 {
            let first = map(value);
            map(first)
        }
        fn main() {
            let mut count: u8 = 0;
            let result = twice(4, |value| { count += 1; value + count });
            println!(\"{}\", result == 7 && count == 2);
        }
    ";
    let surface =
        parse_rust_with_fn_mut_source_model(source, FN_MUT_MODEL, EXCLUSIVE_MODEL, "1.93.2")
            .unwrap();
    let facet = surface.items.iter().find_map(|item| match item {
        SItem::Fn(function) => function.source_fn_mut_specialization.as_ref(),
        _ => None,
    }).expect("the original callback function has a checked specialization");
    assert_eq!(facet.packet, [2]);
    let [capture] = facet.callback.captures.as_slice() else {
        panic!("the original closure captures one mutable root");
    };
    assert_eq!(capture.parameter, 2);
    assert_eq!(&source[capture.binder.start..capture.binder.end], "count");
    check_program(&surface).unwrap();
}
