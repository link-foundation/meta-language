use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use meta_language::benchmark::{
    render_competitor_report, run_competitor_suite, run_competitor_suite_from_paths,
    PUBLISHED_NATGI_AVG_F1,
};
use meta_language::SampleConfig;

#[test]
fn competitor_bar_runs_included_corpora_and_reports_skips() {
    let report = run_competitor_suite(&test_config()).expect("suite loads");
    let rendered = render_competitor_report(&report);

    assert!(report.failures.is_empty(), "{rendered}");
    assert!(!report.runs.is_empty(), "{rendered}");
    assert!(
        report
            .runs
            .iter()
            .all(|run| run.scores.f1 >= PUBLISHED_NATGI_AVG_F1),
        "{rendered}"
    );
    assert!(
        rendered.contains("SKIPPED treevada/bc-example:"),
        "{rendered}"
    );
    assert!(rendered.contains("GBNF emit"), "{rendered}");
}

#[test]
fn manifest_integrity_fails_when_vendored_subject_is_missing_from_manifest() {
    let temp = TempSuite::new("missing-manifest-entry");
    temp.write_subject("tool", "subject", "examples/one.txt", "a");
    temp.write_manifest(
        r#"{
  "schema": 1,
  "corpus": []
}"#,
    );

    let report = run_competitor_suite_from_paths(&temp.manifest, &temp.corpora, &test_config())
        .expect("suite report");
    let rendered = render_competitor_report(&report);

    assert!(
        rendered.contains("tool/subject: vendored subject is missing from manifest"),
        "{rendered}"
    );
}

#[test]
fn manifest_integrity_fails_when_excluded_subject_has_no_reason() {
    let temp = TempSuite::new("empty-skip-reason");
    temp.write_subject("tool", "subject", "examples/one.txt", "a");
    temp.write_manifest(
        r#"{
  "schema": 1,
  "corpus": [
    {
      "tool": "tool",
      "subject": "subject",
      "source": "example.invalid/tool",
      "commit": "abc123",
      "license": "MIT",
      "files": 1,
      "bytes": 1,
      "included": false,
      "exclude_reason": "",
      "example_paths": ["examples"],
      "golden": "external_oracle_pending"
    }
  ]
}"#,
    );

    let report = run_competitor_suite_from_paths(&temp.manifest, &temp.corpora, &test_config())
        .expect("suite report");
    let rendered = render_competitor_report(&report);

    assert!(
        rendered.contains("excluded corpus must provide a non-empty exclude_reason"),
        "{rendered}"
    );
}

const fn test_config() -> SampleConfig {
    SampleConfig {
        seed: 0xE3C0_0017,
        count: 64,
        max_depth: 8,
        repeat_cap: 3,
    }
}

struct TempSuite {
    root: PathBuf,
    corpora: PathBuf,
    manifest: PathBuf,
}

impl TempSuite {
    fn new(name: &str) -> Self {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "meta-language-{name}-{}-{stamp}",
            std::process::id()
        ));
        let corpora = root.join("corpora");
        let manifest = root.join("manifest.json");
        fs::create_dir_all(&corpora).expect("temp corpora root");
        Self {
            root,
            corpora,
            manifest,
        }
    }

    fn write_subject(&self, tool: &str, subject: &str, relative_file: &str, text: &str) {
        let path = self.corpora.join(tool).join(subject).join(relative_file);
        fs::create_dir_all(path.parent().expect("relative file has parent"))
            .expect("subject directory");
        fs::write(path, text).expect("subject fixture");
    }

    fn write_manifest(&self, text: &str) {
        fs::write(&self.manifest, text).expect("manifest fixture");
    }
}

impl Drop for TempSuite {
    fn drop(&mut self) {
        let _ = remove_dir_all_if_exists(&self.root);
    }
}

fn remove_dir_all_if_exists(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)
    } else {
        Ok(())
    }
}
