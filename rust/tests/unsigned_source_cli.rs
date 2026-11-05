//! Deferred actual native binary dispatch, file input and binary/text stdout.
use std::process::Command;
#[test]
fn exact_source_binary_dispatch_retains_every_declared_io_case() {
    let version = Command::new("rustc").arg("--version").output().unwrap();
    assert!(version.status.success());
    let version = String::from_utf8(version.stdout).unwrap();
    let fields = version
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split('.')
        .collect::<Vec<_>>();
    assert!(
        fields[0] == "1" && (90..100).contains(&fields[1].parse::<u32>().unwrap()),
        "uncovered Rust {version}; require >=1.90.0 <1.100.0"
    );
    struct Scratch(std::path::PathBuf);
    impl Drop for Scratch {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let path = std::env::temp_dir().join(format!(
        "meta-language-exact-source-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&path).unwrap();
    let scratch = Scratch(path);
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../parity/fixtures/unsigned-source-command.lino"
    ))
    .unwrap();
    for row in fixture["io"].as_array().unwrap() {
        std::fs::write(
            scratch.0.join("input.source"),
            row["source"].as_str().unwrap(),
        )
        .unwrap();
        let args = row["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let expected = meta_language::run_unsigned_source_command(&args, |path| {
            std::fs::read_to_string(scratch.0.join(path)).map_err(|error| error.to_string())
        });
        assert_eq!(expected.exit_code, 0, "{}", expected.stderr);
        let actual = Command::new(env!("CARGO_BIN_EXE_meta-language"))
            .arg("exact-source")
            .args(&args)
            .current_dir(&scratch.0)
            .output()
            .unwrap();
        assert!(
            actual.status.success(),
            "{}",
            String::from_utf8_lossy(&actual.stderr)
        );
        assert_eq!(actual.stdout, expected.stdout);
        assert_eq!(actual.stderr, expected.stderr.as_bytes());
    }
}
