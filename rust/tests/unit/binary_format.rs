use meta_language::{LinkNetwork, LinkType};

#[test]
fn arbitrary_file_bytes_round_trip_without_utf8_loss() {
    let source = [0x00, 0x25, 0x50, 0x44, 0x46, 0xff, 0x80, 0x0a];

    let network = LinkNetwork::parse_bytes(&source, "application/pdf");

    assert_eq!(network.reconstruct_bytes(), source);
    let tokens = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Token))
        .collect::<Vec<_>>();
    assert_eq!(tokens.len(), 1);
    assert!(tokens
        .iter()
        .all(|link| link.metadata().language() == Some("application/pdf")));
}

#[test]
fn empty_files_round_trip_and_keep_their_format() {
    let network = LinkNetwork::parse_bytes(&[], "application/octet-stream");

    assert!(network.reconstruct_bytes().is_empty());
    assert!(network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Document)
            && link.metadata().language() == Some("application/octet-stream")
    }));
}

#[test]
fn files_round_trip_across_storage_chunk_boundaries() {
    let source = (0..5000)
        .map(|index| u8::try_from(index % 251).expect("modulo result fits in a byte"))
        .collect::<Vec<_>>();

    let network = LinkNetwork::parse_bytes(&source, "application/octet-stream");

    assert_eq!(network.reconstruct_bytes(), source);
    assert_eq!(
        network
            .links()
            .filter(|link| link.metadata().link_type() == Some(LinkType::Token))
            .count(),
        2
    );
}
