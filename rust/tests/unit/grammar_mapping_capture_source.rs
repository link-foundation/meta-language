//! Host file/hash assertions around the same Links decorators used by JavaScript.
use meta_language::{DecoratorLevel, DecoratorRecord, DecoratorSet};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::OnceLock;

const FIXTURE_PATH: &str = "parity/fixtures/grammar-selection-json-string.lino";
const FIXTURE_SHA256: &str = "ee8105fd4aa66df19feb1abd20d1012f696abb712cbd339d5e2816f60fcfaba9";
const DEPENDENCY_CAPTURES: [(&str, &str); 3] = [
    (
        "parity/language-grammar-inventory.lino",
        "ec9f4efb0e9ef936114e09823981352f6db13700",
    ),
    (
        "parity/fixtures/default-cst-expected.lino",
        "bb6722e623fabc6a8be8ce6c0c5a4679602f1711",
    ),
    (
        "parity/fixtures/native-default-cst-expected.lino",
        "ec9f4efb0e9ef936114e09823981352f6db13700",
    ),
];

fn sha256(bytes: impl AsRef<[u8]>) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut hexadecimal, byte| {
            use std::fmt::Write as _;
            write!(hexadecimal, "{byte:02x}").expect("writing into a String succeeds");
            hexadecimal
        })
}

fn dependency_capture(root: &Path, relative: &str, mut bytes: Vec<u8>) -> Vec<u8> {
    static CAPTURES: OnceLock<Vec<Value>> = OnceLock::new();
    let captures = CAPTURES.get_or_init(|| {
        let archive: Value = meta_language::project_data::read_project_data_file(
            root.join("parity/self-translation/dependency-workflow-source-history.lino"),
        )
        .unwrap();
        assert_eq!(archive["schemaVersion"], 2);
        archive["sources"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| {
                DEPENDENCY_CAPTURES
                    .iter()
                    .any(|(file, revision)| row["file"] == *file && row["revision"] == *revision)
            })
            .cloned()
            .collect()
    });
    assert_eq!(captures.len(), DEPENDENCY_CAPTURES.len());
    let row = captures.iter().find(|row| row["file"] == relative).unwrap();
    assert_eq!(Value::from(bytes.len()), row["currentBytes"]);
    assert_eq!(sha256(&bytes), row["currentDigest"].as_str().unwrap());
    let edit = &row["edit"];
    let offset = usize::try_from(edit["offset"].as_u64().unwrap()).unwrap();
    let length = usize::try_from(edit["length"].as_u64().unwrap()).unwrap();
    let end = offset.checked_add(length).unwrap();
    assert!(end <= bytes.len());
    bytes.splice(offset..end, edit["predecessor"].as_str().unwrap().bytes());
    assert_eq!(
        Value::from(bytes.len()),
        row["certificate"]["blob"]["bytes"]
    );
    assert_eq!(
        sha256(&bytes),
        row["certificate"]["blob"]["sha256"].as_str().unwrap()
    );
    bytes
}

pub fn original_capture(relative: &str, bytes: Vec<u8>) -> Vec<u8> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let profile: Value = meta_language::project_data::read_project_data_file(
        root.join("parity/fixtures/grammar-selection-capture-source-profile.lino"),
    )
    .unwrap();
    assert_eq!(profile["schemaVersion"], 1);
    assert_eq!(profile["historicalFixture"]["path"], FIXTURE_PATH);
    assert_eq!(profile["historicalFixture"]["sha256"], FIXTURE_SHA256);
    let fixture_bytes = std::fs::read(root.join(FIXTURE_PATH)).unwrap();
    assert_eq!(sha256(&fixture_bytes), FIXTURE_SHA256);
    if DEPENDENCY_CAPTURES
        .iter()
        .any(|(file, _)| *file == relative)
    {
        return dependency_capture(&root, relative, bytes);
    }
    let originals = profile["sources"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|source| source["path"] == relative)
        .collect::<Vec<_>>();
    assert!(originals.len() <= 1);
    let Some(original) = originals.first() else {
        return bytes;
    };
    let fixture: Value =
        meta_language::project_data::read_project_data_file(root.join(FIXTURE_PATH)).unwrap();
    let capture = fixture["captures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|source| source["path"] == relative)
        .unwrap();
    let original_source = original["source"].as_str().unwrap();
    assert_eq!(Value::from(original_source.len()), capture["bytes"]);
    assert_eq!(sha256(original_source), capture["sha256"].as_str().unwrap());
    assert_eq!(Value::from(bytes.len()), original["current"]["bytes"]);
    assert_eq!(
        sha256(&bytes),
        original["current"]["sha256"].as_str().unwrap()
    );
    let current_source = std::str::from_utf8(&bytes).unwrap();
    let original_record = DecoratorRecord::from([
        ("capture".to_owned(), relative.to_owned()),
        ("source".to_owned(), original_source.to_owned()),
    ]);
    let current_record = DecoratorRecord::from([
        ("capture".to_owned(), relative.to_owned()),
        ("source".to_owned(), current_source.to_owned()),
    ]);
    let forward = DecoratorSet::from_lino(profile["forwardDecorators"].as_str().unwrap())
        .unwrap()
        .decorate(DecoratorLevel::Emitter, &original_record);
    assert_eq!(forward.as_ref(), Some(&current_record));
    let reverse = DecoratorSet::from_lino(profile["reverseDecorators"].as_str().unwrap())
        .unwrap()
        .decorate(DecoratorLevel::Emitter, &current_record);
    assert_eq!(reverse.as_ref(), Some(&original_record));
    original_source.as_bytes().to_vec()
}
