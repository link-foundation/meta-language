//! Deferred actual binary dispatch on current exact saved graph files.
use std::process::Command;
#[test]
fn exact_graph_binary_uses_current_snapshots_and_returns_the_declared_process_results() {
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
    let product = Command::new("node")
        .arg("--max-old-space-size=256")
        .arg("../experiments/unsigned-graph/produce.mjs")
        .output()
        .unwrap();
    assert!(
        product.status.success(),
        "{}",
        String::from_utf8_lossy(&product.stderr)
    );
    let observed =
        meta_language::read_structured_data_lino(&String::from_utf8(product.stdout).unwrap())
            .unwrap();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../parity/fixtures/unsigned-graph-command.lino"
    ))
    .unwrap();
    assert_eq!(
        observed["io"].as_array().unwrap().len(),
        fixture["io"].as_array().unwrap().len()
    );
    struct Scratch(std::path::PathBuf);
    impl Drop for Scratch {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let path = std::env::temp_dir().join(format!(
        "meta-language-exact-graph-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&path).unwrap();
    let scratch = Scratch(path);
    for (row, actual) in fixture["io"]
        .as_array()
        .unwrap()
        .iter()
        .zip(observed["io"].as_array().unwrap())
    {
        assert_eq!(row["name"], actual["name"]);
        let args = actual["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let input = if args.iter().any(|arg| arg == "--input-binary") {
            actual["inputBytes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| u8::try_from(value.as_u64().unwrap()).unwrap())
                .collect::<Vec<_>>()
        } else {
            actual["inputText"].as_str().unwrap().as_bytes().to_vec()
        };
        std::fs::write(scratch.0.join("saved.graph"), input).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_meta-language"))
            .arg("exact-graph")
            .args(&args)
            .current_dir(&scratch.0)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code().unwrap(),
            i32::try_from(row["exitCode"].as_i64().unwrap()).unwrap()
        );
        assert_eq!(output.stderr, actual["stderr"].as_str().unwrap().as_bytes());
        let expected = if args.iter().any(|arg| arg == "--binary") && row["exitCode"] == 0 {
            actual["bytes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| u8::try_from(value.as_u64().unwrap()).unwrap())
                .collect::<Vec<_>>()
        } else {
            actual["stdout"].as_str().unwrap().as_bytes().to_vec()
        };
        assert_eq!(output.stdout, expected);
    }
}
