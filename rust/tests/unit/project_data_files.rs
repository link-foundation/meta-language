//! Native filesystem adapters preserve the tested project-data boundaries.
use std::fs;

use meta_language::project_data::{
    ProjectDataFormat, read_project_data_file, read_project_data_file_with_format,
    read_project_file_bytes, resolve_project_data_path, resolve_project_data_path_with_format,
};
use meta_language::structured_data_to_lino;
use serde_json::{Value, json};

use super::issue_195_translation_pairs::TemporaryDirectory;

#[test]
fn logical_identity_resolves_exact_linked_data_and_digest_bytes() {
    let directory = TemporaryDirectory::new("project-data-logical");
    let logical = directory.0.join("fixture with spaces.json");
    let physical = logical.with_extension("lino");
    let value = json!({
        "fixtureFile": "parity/fixtures/native-grammars/elixir.json",
        "cases": [{"id": "café 😀", "rows": [[0, "", true, null]]}],
        "__proto__": {"owned": true},
    });
    let bytes = structured_data_to_lino(&value).unwrap();
    fs::write(&physical, &bytes).unwrap();
    assert_eq!(resolve_project_data_path(&logical).unwrap(), physical);
    assert!(!logical.exists());
    assert_eq!(read_project_data_file::<Value>(&logical).unwrap(), value);
    assert_eq!(read_project_file_bytes(&logical).unwrap(), bytes.as_bytes());
    assert_eq!(
        read_project_data_file::<Value>(&logical).unwrap()["fixtureFile"],
        "parity/fixtures/native-grammars/elixir.json"
    );
}

#[test]
fn coexisting_storage_refuses_except_at_an_explicit_external_json_boundary() {
    let directory = TemporaryDirectory::new("project-data-duplicate");
    let logical = directory.0.join("fixture.json");
    let physical = logical.with_extension("lino");
    fs::write(&logical, r#"{"current":false}"#).unwrap();
    fs::write(
        &physical,
        structured_data_to_lino(&json!({"current": true})).unwrap(),
    )
    .unwrap();
    for file in [&logical, &physical] {
        assert!(
            resolve_project_data_path(file)
                .unwrap_err()
                .contains("duplicate JSON and LiNo")
        );
        assert!(
            read_project_data_file::<Value>(file)
                .unwrap_err()
                .contains("duplicate JSON and LiNo")
        );
    }
    assert_eq!(
        resolve_project_data_path_with_format(&logical, ProjectDataFormat::Json).unwrap(),
        logical
    );
    assert_eq!(
        read_project_data_file_with_format::<Value>(&logical, ProjectDataFormat::Json).unwrap(),
        json!({"current": false})
    );
}

#[test]
fn legacy_json_and_non_data_bytes_retain_their_requested_boundaries() {
    let directory = TemporaryDirectory::new("project-data-legacy");
    let package = directory.0.join("package.json");
    fs::write(&package, r#"{"name":"external-package","version":"1.0.0"}"#).unwrap();
    assert_eq!(resolve_project_data_path(&package).unwrap(), package);
    assert_eq!(
        read_project_data_file::<Value>(&package).unwrap()["name"],
        "external-package"
    );
    assert!(!package.with_extension("lino").exists());
    let source = directory.0.join("source.rs");
    let bytes = "fn café() { /* 😀 */ }\r\n";
    fs::write(&source, bytes).unwrap();
    assert_eq!(resolve_project_data_path(&source).unwrap(), source);
    assert_eq!(read_project_file_bytes(&source).unwrap(), bytes.as_bytes());
    assert!(
        read_project_data_file::<Value>(&source)
            .unwrap_err()
            .contains("unsupported storage extension")
    );
    let selected = resolve_project_data_path_with_format(&source, ProjectDataFormat::Lino).unwrap();
    assert_eq!(selected, directory.0.join("source.rs.lino"));
}

#[test]
fn malformed_linked_data_never_falls_back_and_missing_files_keep_physical_errors() {
    let directory = TemporaryDirectory::new("project-data-invalid");
    let logical = directory.0.join("fixture.json");
    let physical = logical.with_extension("lino");
    fs::write(&physical, "this is not a structured data graph").unwrap();
    let error = read_project_data_file::<Value>(&logical).unwrap_err();
    assert!(error.contains("fixture.lino"), "{error}");
    fs::remove_file(&physical).unwrap();
    assert_eq!(resolve_project_data_path(&logical).unwrap(), physical);
    assert!(
        read_project_data_file::<Value>(&logical)
            .unwrap_err()
            .contains("fixture.lino")
    );
}
