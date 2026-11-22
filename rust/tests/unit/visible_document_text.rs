use meta_language::link_network::visible_document_text::VisibleDocumentTextOptions;
use meta_language::{LinkNetwork, ParseConfiguration};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/fixtures/visible-document-text.lino";

#[test]
fn visible_document_text_keeps_source_ranges() {
    let fixtures: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/visible-document-text.lino"
    ))
    .expect("visible document text fixtures");
    for (language, group, requirement_id, assertions) in [
        (
            "HTML",
            "html",
            "I195-I238-HTML-VISIBLE-TEXT",
            [
                "htmlVisibleTextAndOptions",
                "htmlVisibleTextKeepsSourceRanges",
            ],
        ),
        (
            "Markdown",
            "markdown",
            "I195-I238-MARKDOWN-VISIBLE-TEXT",
            [
                "markdownVisibleText",
                "markdownVisibleTextKeepsSourceRanges",
            ],
        ),
        (
            "DOCX",
            "docx",
            "I195-I238-WORDPROCESSINGML-VISIBLE-TEXT",
            [
                "wordprocessingmlVisibleText",
                "wordprocessingmlVisibleTextKeepsSourceRanges",
            ],
        ),
    ] {
        for case in fixtures[group].as_array().expect("document fixture cases") {
            let source = case["source"].as_str().expect("source document");
            let expected = case["text"].as_str().expect("visible text");
            let opts = &case["options"];
            let options = VisibleDocumentTextOptions {
                block_separator: opts["blockSeparator"].as_str().unwrap_or("\n"),
                decode_entities: opts["decodeEntities"].as_bool().unwrap_or(true),
                collapse_whitespace: opts["collapseWhitespace"].as_bool().unwrap_or(true),
                keep_nonbreaking_space: opts["nbsp"].as_str() == Some("keep"),
                list_bullet: opts["listBullet"].as_str().unwrap_or_default(),
            };
            let original = LinkNetwork::parse(source, language, ParseConfiguration::default());
            let network =
                LinkNetwork::from_lino(&original.to_lino()).expect("document network round trip");
            let projection = network.visible_text(options);
            assert_eq!(projection.text, expected, "{language}: {source}");
            assert_eq!(network.reconstruct_text(), source);
            assert_eq!(projection, original.visible_text(options));
            let mut text_end = 0;
            let mut source_end = 0;
            for span in &projection.map {
                assert_eq!(span.text.start(), text_end);
                assert!(
                    span.text.end() > span.text.start()
                        && projection
                            .text
                            .get(span.text.start()..span.text.end())
                            .is_some()
                );
                assert!(
                    span.source.start() >= source_end
                        && source.get(span.source.start()..span.source.end()).is_some(),
                    "{span:?}"
                );
                text_end = span.text.end();
                source_end = span.source.end();
            }
            assert_eq!(text_end, expected.len());
            for selection in case["selections"].as_array().expect("visible selections") {
                let visible = selection[0].as_str().expect("visible fragment");
                let original = selection[1].as_str().expect("source spelling");
                let start = expected.find(visible).expect("fragment occurs");
                let mapped = projection
                    .source_range(start, start + visible.len())
                    .expect("exact selection range");
                assert_eq!(&source[mapped.start()..mapped.end()], original);
            }
            assert_eq!(projection.source_range(0, 0), None);
            assert_eq!(
                projection.source_range(expected.len(), expected.len() + 1),
                None
            );
        }
        record(&Observation {
            requirement_id,
            suffix: "behavior",
            fixture_id: &requirement_id.to_ascii_lowercase(),
            fixture_file: FIXTURE,
            assertions: &assertions,
            test_name: "visible_document_text_keeps_source_ranges",
        });
    }
}
