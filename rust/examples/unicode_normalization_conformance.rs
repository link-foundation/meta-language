//! Future full official NFC corpus runner. Compile and execute only in CI.
//! The caller supplies the independently pinned digest of the temporary source.
use meta_language::{SOURCE_IDENTIFIER_UNICODE_VERSION, normalize_unicode_text};
use sha2::{Digest, Sha256};

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    assert_eq!(
        arguments.len(),
        2,
        "use <Unicode NormalizationTest.txt> <pinned sha256>"
    );
    let bytes = std::fs::read(&arguments[0]).expect("read temporary Unicode corpus");
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        arguments[1],
        "Unicode conformance source digest differs"
    );
    let source = String::from_utf8(bytes).expect("Unicode corpus is UTF-8");
    assert!(
        source.lines().take(20).any(|line| line.contains(&format!(
            "NormalizationTest-{SOURCE_IDENTIFIER_UNICODE_VERSION}.txt"
        ))),
        "uncovered Unicode conformance version"
    );
    let mut assertions = 0;
    for (line_index, line) in source.lines().enumerate() {
        let content = line.split('#').next().unwrap().trim();
        if content.is_empty() || content.starts_with('@') {
            continue;
        }
        let columns = content
            .split(';')
            .take(5)
            .map(|column| {
                column
                    .split_whitespace()
                    .map(|point| {
                        char::from_u32(
                            u32::from_str_radix(point, 16).expect("hexadecimal Unicode scalar"),
                        )
                        .expect("Unicode scalar")
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        assert_eq!(columns.len(), 5, "incomplete row {}", line_index + 1);
        for (index, input) in columns.iter().enumerate() {
            let expected = &columns[if index < 3 { 1 } else { 3 }];
            assert_eq!(
                &normalize_unicode_text(input).unwrap(),
                expected,
                "row {} column {}",
                line_index + 1,
                index + 1
            );
            assertions += 1;
        }
    }
    assert!(assertions > 0, "Unicode conformance corpus is empty");
    println!("Unicode {SOURCE_IDENTIFIER_UNICODE_VERSION}: {assertions} NFC assertions");
}
