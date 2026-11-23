use meta_language::link_network::visible_document_text::VisibleDocumentTextOptions;
use meta_language::{ByteRange, LinkNetwork, ParseConfiguration};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};
use super::project_data::read_project_data;

const PDF_FIXTURE: &str = "parity/fixtures/pdf-document.lino";
const WORD_FIXTURE: &str = "parity/fixtures/wordprocessing-package.lino";

fn octets(value: &Value) -> Vec<u8> {
    let source = value.as_str().expect("hexadecimal file");
    (0..source.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&source[index..index + 2], 16).expect("fixture byte"))
        .collect()
}

fn offset(value: &Value) -> usize {
    usize::try_from(value.as_u64().expect("fixture coordinate")).expect("coordinate fits in usize")
}

fn binary_string(bytes: &[u8]) -> String {
    bytes.iter().copied().map(char::from).collect()
}

fn check_reconstruction(network: &LinkNetwork, raw: bool, bytes: &[u8], text: &str) {
    if raw {
        assert_eq!(network.reconstruct_bytes(), bytes);
    } else {
        assert_eq!(network.reconstruct_text(), text);
    }
}

fn check_reading_order(fixture: &Value) {
    let page = fixture["page"].as_str().expect("positioned PDF");
    let text = fixture["text"].as_str().expect("positioned text");
    let ranges = fixture["ranges"].as_array().expect("positioned PDF ranges");
    for (raw, input) in [
        (true, LinkNetwork::parse_bytes(page.as_bytes(), "PDF")),
        (
            false,
            LinkNetwork::parse(page, "PDF", ParseConfiguration::default()),
        ),
    ] {
        let network = LinkNetwork::from_lino(&input.to_lino()).expect("positioned PDF round trip");
        assert!(network.verify_full_match(None).is_clean());
        check_reconstruction(&network, raw, page.as_bytes(), page);
        let projection = network.visible_text(VisibleDocumentTextOptions::default());
        assert_eq!(projection.text, text);
        for range in ranges {
            let selected = range[0].as_str().expect("selected text");
            let anchors = range[1].as_array().expect("source anchors");
            let first = anchors[0].as_str().expect("first operand");
            let last = anchors
                .last()
                .and_then(Value::as_str)
                .expect("last operand");
            let start = projection
                .text
                .find(selected)
                .expect("visible positioned text");
            assert_eq!(
                projection.source_range(start, start + selected.len()),
                Some(ByteRange::new(
                    page.find(first).expect("first source offset"),
                    page.find(last).expect("last source offset") + last.len(),
                )),
            );
        }
    }
}

fn encoded_pdf_stream(entry: &Value) -> (Vec<u8>, usize, usize) {
    let encoded = octets(&entry["data"]);
    let parameters = entry["parameters"].as_str().expect("PDF stream parameters");
    let header = format!(
        "%PDF-1.7\n1 0 obj\n<< /Length {} {parameters} >>\nstream\n",
        encoded.len()
    );
    let start = header.len();
    let end = start + encoded.len();
    let mut bytes = header.into_bytes();
    bytes.extend(encoded);
    bytes.extend_from_slice(b"\nendstream\nendobj\n%%EOF\n");
    (bytes, start, end)
}

fn check_lzw_decoding(fixture: &Value) {
    let padding = offset(&fixture["padding"]);
    let contents = fixture["contents"].as_str().expect("LZW content");
    let expected = format!("{}{contents}", " ".repeat(padding));
    let text = fixture["text"].as_str().expect("LZW visible text");
    for entry in fixture["streams"].as_array().expect("LZW streams") {
        let (bytes, start, end) = encoded_pdf_stream(entry);
        let source = binary_string(&bytes);
        for (raw, input) in [
            (true, LinkNetwork::parse_bytes(&bytes, "PDF")),
            (
                false,
                LinkNetwork::parse(&source, "PDF", ParseConfiguration::default()),
            ),
        ] {
            let network = LinkNetwork::from_lino(&input.to_lino()).expect("LZW PDF round trip");
            assert!(network.verify_full_match(None).is_clean());
            check_reconstruction(&network, raw, &bytes, &source);
            let end = if raw {
                end
            } else {
                binary_string(&bytes[..end]).len()
            };
            let range = ByteRange::new(start, end);
            let documents = network.derived_documents();
            assert_eq!(documents.len(), 1);
            assert_eq!(
                documents[0].compression,
                entry["compression"].as_str().expect("LZW filter chain")
            );
            assert_eq!(documents[0].source_range, range);
            assert_eq!(documents[0].network.reconstruct_text(), expected);
            let projection = network.visible_text(VisibleDocumentTextOptions::default());
            assert_eq!(projection.text, text);
            assert_eq!(projection.source_range(0, text.len()), Some(range));
        }
    }
    for entry in fixture["invalid"].as_array().expect("broken LZW streams") {
        let (bytes, start, end) = encoded_pdf_stream(entry);
        let network = LinkNetwork::parse_bytes(&bytes, "PDF");
        assert!(!network.verify_full_match(None).is_clean());
        assert_eq!(network.reconstruct_bytes(), bytes);
        assert!(network.derived_documents().is_empty());
        assert!(network.links().any(|link| {
            link.metadata().flags().is_error()
                && link
                    .metadata()
                    .span()
                    .is_some_and(|span| span.byte_range() == ByteRange::new(start, end))
        }));
    }
}

fn check_declared_content(fixture: &Value) {
    for entry in fixture.as_array().expect("declared content documents") {
        let bytes = octets(&entry["encoded"]);
        let source = binary_string(&bytes);
        let expected = entry["streams"].as_array().expect("declared streams");
        for (raw, original) in [
            (true, LinkNetwork::parse_bytes(&bytes, "PDF")),
            (
                false,
                LinkNetwork::parse(&source, "PDF", ParseConfiguration::default()),
            ),
        ] {
            let network = LinkNetwork::from_lino(&original.to_lino()).expect("PDF round trip");
            assert!(network.verify_full_match(None).is_clean());
            check_reconstruction(&network, raw, &bytes, &source);
            assert!(
                network
                    .visible_text(VisibleDocumentTextOptions::default())
                    .text
                    .is_empty()
            );
            let decoded = network.derived_documents();
            assert_eq!(decoded.len(), expected.len());
            let coordinate = |position| {
                if raw {
                    position
                } else {
                    binary_string(&bytes[..position]).len()
                }
            };
            for (stream, expected) in decoded.iter().zip(expected) {
                let content = expected["content"].as_str().expect("decoded content");
                assert_eq!(stream.name, "content_stream");
                assert_eq!(stream.compression, "FlateDecode");
                assert_eq!(stream.network.reconstruct_text(), content);
                assert_eq!(
                    stream.source_range,
                    ByteRange::new(
                        coordinate(offset(&expected["start"])),
                        coordinate(offset(&expected["end"])),
                    )
                );
                assert!(
                    stream
                        .network
                        .links()
                        .any(|link| link.metadata().term() == Some("content_stream"))
                );
                if content.starts_with("BI ") {
                    assert!(
                        stream
                            .network
                            .links()
                            .any(|link| link.metadata().term() == Some("inline_image"))
                    );
                }
            }
        }
    }
}

#[test]
fn pdf_streams_and_font_unicode_keep_the_original_ranges() {
    let fixture = read_project_data(include_str!("../../../parity/fixtures/pdf-document.lino"))
        .expect("PDF document fixtures");
    let bytes = octets(&fixture["encoded"]);
    let binary = binary_string(&bytes);
    let settings = ParseConfiguration::default();
    let expected = fixture["text"].as_str().expect("projected text");
    let start = offset(&fixture["start"]);
    let end = offset(&fixture["end"]);
    for (raw, original) in [
        (true, LinkNetwork::parse_bytes(&bytes, "PDF")),
        (false, LinkNetwork::parse(&binary, "PDF", settings)),
    ] {
        let network = LinkNetwork::from_lino(&original.to_lino()).expect("PDF round trip");
        assert!(network.verify_full_match(None).is_clean());
        check_reconstruction(&network, raw, &bytes, &binary);
        let coordinate = |position| {
            if raw {
                position
            } else {
                binary_string(&bytes[..position]).len()
            }
        };
        let range = ByteRange::new(coordinate(start), coordinate(end));
        let attached = network.derived_documents();
        assert_eq!(attached.len(), 1);
        let content = &attached[0];
        assert_eq!(content.name, "content_stream");
        assert_eq!(content.compression, "FlateDecode");
        assert_eq!(
            content.encoding,
            if raw { "bytes" } else { "binary-string" }
        );
        assert_eq!(content.source_range, range);
        assert_eq!(
            content.network.reconstruct_text(),
            fixture["contents"].as_str().expect("decoded operators")
        );
        assert!(
            content
                .network
                .links()
                .any(|link| link.metadata().term() == Some("operation"))
        );
        let projection = network.visible_text(VisibleDocumentTextOptions::default());
        assert_eq!(projection.text, expected);
        assert_eq!(projection.source_range(0, "чай 🧪B".len()), Some(range));
        let shown = r"(A\200\201)";
        let literal = bytes
            .windows(shown.len())
            .position(|window| window == shown.as_bytes())
            .expect("literal shown text");
        let visible = projection.text.find('A').expect("visible page text");
        assert_eq!(
            projection.source_range(visible, projection.text.len()),
            Some(ByteRange::new(
                coordinate(literal),
                coordinate(literal + shown.len())
            ))
        );
        assert_eq!(
            network
                .visible_text(VisibleDocumentTextOptions {
                    block_separator: "|",
                    ..Default::default()
                })
                .text,
            expected.replace('\n', "|")
        );
    }
    for undecoded in [
        LinkNetwork::parse(
            &binary,
            "PDF",
            settings.with_document_stream_decoding(false),
        ),
        LinkNetwork::parse_bytes_with_configuration(
            &bytes,
            "PDF",
            settings.with_document_stream_decoding(false),
        ),
    ] {
        assert!(undecoded.verify_full_match(None).is_clean());
        assert!(undecoded.derived_documents().is_empty());
        assert_eq!(
            undecoded
                .visible_text(VisibleDocumentTextOptions::default())
                .text,
            "A€é"
        );
    }
    let damaged = octets(&fixture["corrupt"]);
    let network = LinkNetwork::parse_bytes(&damaged, "PDF");
    assert!(!network.verify_full_match(None).is_clean());
    assert_eq!(network.reconstruct_bytes(), damaged);
    assert!(network.links().any(|link| {
        link.metadata().flags().is_error()
            && link
                .metadata()
                .span()
                .is_some_and(|span| span.byte_range() == ByteRange::new(start, end))
    }));
    assert_eq!(
        network
            .visible_text(VisibleDocumentTextOptions::default())
            .text,
        "A€é"
    );
    check_reading_order(&fixture["readingOrder"]);
    check_reading_order(&fixture["transformedReadingOrder"]);
    check_reading_order(&fixture["taggedReadingOrder"]);
    check_lzw_decoding(&fixture["lzw"]);
    check_declared_content(&fixture["declaredContent"]);
    for (requirement_id, assertions) in [
        (
            "I195-I239-PDF-CONTENT-STREAMS",
            &[
                "pdfDerivedOperatorsKeepEncodedRange",
                "pdfFiltersCanBeDisabled",
                "corruptPdfFilterKeepsLocatedError",
                "pdfLzwCodeWidthsAndFilterChainsKeepEncodedRange",
                "corruptPdfLzwKeepsLocatedError",
                "pdfDeclaredContentWithoutTextKeepsEncodedRanges",
            ][..],
        ),
        (
            "I195-I239-PDF-FONT-UNICODE",
            &[
                "pdfTextUsesFontMappingsAndPageOrder",
                "pdfVisibleTextKeepsEncodedAndLiteralRanges",
                "pdfPositionedLinesKeepReadingOrderAndRanges",
                "pdfTransformedLinesKeepReadingOrderAndRanges",
                "pdfTaggedSequencesKeepReadingOrderAndRanges",
            ][..],
        ),
    ] {
        record(&Observation {
            requirement_id,
            suffix: "behavior",
            fixture_id: &requirement_id.to_ascii_lowercase(),
            fixture_file: PDF_FIXTURE,
            assertions,
            test_name: "pdf_streams_and_font_unicode_keep_the_original_ranges",
        });
    }
}

fn check_wordprocessing_parts(fixture: &Value, visible: &str, relationship_selected: bool) {
    let cases = fixture["cases"].as_array().expect("ZIP cases");
    let names = fixture["names"].as_array().expect("part names");
    let parts = fixture["parts"].as_array().expect("part texts");
    for case in cases {
        let bytes = octets(&case["archive"]);
        let text = binary_string(&bytes);
        let ranges = case["ranges"].as_array().expect("encoded ranges");
        let compression = case["compression"].as_str().expect("ZIP compression");
        for (raw, original) in [
            (true, LinkNetwork::parse_bytes(&bytes, "DOCX")),
            (
                false,
                LinkNetwork::parse(&text, "DOCX", ParseConfiguration::default()),
            ),
        ] {
            let network = LinkNetwork::from_lino(&original.to_lino()).expect("package round trip");
            assert!(network.verify_full_match(None).is_clean());
            check_reconstruction(&network, raw, &bytes, &text);
            let documents = network.derived_documents();
            assert_eq!(documents.len(), names.len());
            let coordinate = |position| {
                if raw {
                    position
                } else {
                    binary_string(&bytes[..position]).len()
                }
            };
            for (index, document) in documents.iter().enumerate() {
                assert_eq!(document.name, names[index].as_str().expect("part name"));
                assert_eq!(
                    document.network.reconstruct_text(),
                    parts[index].as_str().expect("part text")
                );
                assert_eq!(document.compression, compression);
                assert_eq!(
                    document.encoding,
                    if raw { "bytes" } else { "binary-string" }
                );
                assert_eq!(
                    document.source_range,
                    ByteRange::new(
                        coordinate(offset(&ranges[index][0])),
                        coordinate(offset(&ranges[index][1]))
                    )
                );
            }
            let projection = network.visible_text(VisibleDocumentTextOptions::default());
            assert_eq!(
                projection.text,
                fixture["expectedText"].as_str().expect("package text")
            );
            let part_start = offset(&ranges[0][0]);
            let (start, end) = if compression == "stored" {
                let start = part_start
                    + parts[0]
                        .as_str()
                        .expect("main part")
                        .find(visible)
                        .expect("word in part");
                (start, start + visible.len())
            } else {
                (part_start, offset(&ranges[0][1]))
            };
            assert_eq!(
                projection.source_range(0, visible.len()),
                Some(ByteRange::new(coordinate(start), coordinate(end)))
            );
            let all = projection
                .source_range(0, projection.text.len())
                .expect("covering range");
            if relationship_selected {
                for range in ranges {
                    assert!(
                        all.start() < coordinate(offset(&range[1]))
                            && all.end() > coordinate(offset(&range[0]))
                    );
                }
            } else {
                assert!(
                    all.start() <= coordinate(offset(&ranges[1][0]) + 200)
                        && all.end() >= coordinate(offset(&ranges[3][0]))
                );
            }
        }
    }
}

#[test]
fn wordprocessing_packages_keep_parts_and_character_ranges() {
    let fixture = read_project_data(include_str!(
        "../../../parity/fixtures/wordprocessing-package.lino"
    ))
    .expect("WordprocessingML packages");
    check_wordprocessing_parts(&fixture, "Кошка", false);
    let damaged = octets(&fixture["corrupt"]);
    let network = LinkNetwork::parse_bytes(&damaged, "DOCX");
    assert!(!network.verify_full_match(None).is_clean());
    assert_eq!(network.reconstruct_bytes(), damaged);
    let range = &fixture["cases"][0]["ranges"][0];
    assert!(network.links().any(|link| {
        link.metadata().flags().is_error()
            && link
                .metadata()
                .definition()
                .is_some_and(|text| text.contains("OPC part size or checksum mismatch"))
            && link.metadata().span().is_some_and(|span| {
                span.byte_range() == ByteRange::new(offset(&range[0]), offset(&range[1]))
            })
    }));
    let surviving = network.derived_documents();
    let names = fixture["names"].as_array().expect("part names");
    assert_eq!(surviving.len(), names.len() - 1);
    assert!(
        surviving
            .iter()
            .zip(&names[1..])
            .all(|(part, name)| name.as_str() == Some(part.name.as_str()))
    );
    record(&Observation {
        requirement_id: "I195-I239-WORDPROCESSING-PACKAGE",
        suffix: "behavior",
        fixture_id: "i195-i239-wordprocessing-package",
        fixture_file: WORD_FIXTURE,
        assertions: &[
            "opcPackageKeepsStoredAndDeflatedParts",
            "opcVisibleTextKeepsEncodedAndStoredRanges",
            "corruptOpcPartKeepsLocatedError",
        ],
        test_name: "wordprocessing_packages_keep_parts_and_character_ranges",
    });
}

#[test]
fn wordprocessing_relationships_select_named_parts_and_resolve_internal_targets() {
    const FIXTURE: &str = "parity/fixtures/wordprocessing-relationships.lino";
    let fixture = read_project_data(include_str!(
        "../../../parity/fixtures/wordprocessing-relationships.lino"
    ))
    .expect("WordprocessingML relationships");
    check_wordprocessing_parts(&fixture, "Корпус", true);
    let missing = &fixture["missing"];
    let bytes = octets(&missing["archive"]);
    let network = LinkNetwork::parse_bytes(&bytes, "DOCX");
    assert!(!network.verify_full_match(None).is_clean());
    assert_eq!(network.reconstruct_bytes(), bytes);
    let range = &missing["relationshipRange"];
    assert!(network.links().any(|link| {
        let metadata = link.metadata();
        metadata.flags().is_error()
            && metadata.term() == Some("ERROR")
            && metadata
                .definition()
                .is_some_and(|text| text.contains("../../../outside.xml"))
            && metadata.span().is_some_and(|span| {
                span.byte_range() == ByteRange::new(offset(&range[0]), offset(&range[1]))
            })
    }));
    let names: Vec<_> = fixture["names"]
        .as_array()
        .expect("part names")
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != 1)
        .map(|(_, name)| name.as_str().expect("part name"))
        .collect();
    assert_eq!(
        network
            .derived_documents()
            .iter()
            .map(|document| document.name.as_str())
            .collect::<Vec<_>>(),
        names
    );
    record(&Observation {
        requirement_id: "I195-I239-PACKAGE-RELATIONSHIPS",
        suffix: "behavior",
        fixture_id: "i195-i239-package-relationships",
        fixture_file: FIXTURE,
        assertions: &[
            "opcRelationshipsResolveInternalParts",
            "opcRelationshipsKeepSourceRanges",
            "invalidOpcTargetKeepsRelationshipRange",
        ],
        test_name: "wordprocessing_relationships_select_named_parts_and_resolve_internal_targets",
    });
}
