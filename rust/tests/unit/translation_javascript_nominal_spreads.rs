//! Deferred shared-policy, whole-frontend and both native-product controls.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::read_structured_data_lino;
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    javascript::parse_javascript,
};
use std::{fs, process::Command};
#[path = "../../src/translation/javascript_nominal_spread_rules.rs"]
mod rules;
#[test]
fn nominal_copies_keep_owners_effect_order_guards_and_complete_products() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/nominal-spread-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let data = read_structured_data_lino(&producer.stdout).unwrap();
    let strings = |value: &serde_json::Value| {
        value
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>()
    };
    for row in data["policies"].as_array().unwrap() {
        assert_eq!(
            rules::read_java_script_nominal_spread_error(
                row["language"].as_str().unwrap(),
                row["constructors"].as_f64().unwrap(),
                &strings(&row["declared"]),
                &strings(&row["kinds"]),
                &strings(&row["names"]),
                &strings(&row["owners"]),
                row["owner"].as_str().unwrap(),
                row["prototype"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in data["guards"].as_array().unwrap() {
        assert_eq!(
            rules::read_java_script_nominal_spread_guard_arm(
                row["kind"].as_str().unwrap(),
                row["operation"].as_str().unwrap(),
                row["absence"].as_str().unwrap()
            ),
            row["expected"].as_f64().unwrap()
        );
    }
    for (index, row) in data["rows"].as_array().unwrap().iter().enumerate() {
        let surface = parse_javascript(row["source"].as_str().unwrap()).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), row["surface"]);
        let program = check_program(&surface).unwrap();
        let native = emit_rust(&program).unwrap();
        assert_eq!(serde_json::to_value(&native).unwrap(), row["native"]);
        assert_eq!(
            serde_json::to_value(emit_javascript(&program).unwrap()).unwrap(),
            row["javascript"]
        );
        let entry = native
            .mappings
            .iter()
            .find(|mapping| mapping.kind == "function" && mapping.source == "compute")
            .unwrap()
            .target
            .clone();
        let javascript_product = row["native"]["preludes"]
            .as_array()
            .unwrap()
            .iter()
            .chain(row["native"]["definitions"].as_array().unwrap())
            .map(|value| value.as_str().unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        for (owner, code) in [
            (
                "native",
                native.preludes.join("\n") + "\n" + &native.definitions.join("\n"),
            ),
            ("javascript", javascript_product),
        ] {
            let directory = TemporaryDirectory::new(&format!("nominal-spread-{index}-{owner}"));
            let file = directory.0.join("product.rs");
            let binary = directory.0.join(if cfg!(windows) {
                "product.exe"
            } else {
                "product"
            });
            fs::write(
                &file,
                code + &format!(
                    "\nfn main(){{assert_eq!({entry}(),{}f64);}}\n",
                    row["expected"].as_f64().unwrap()
                ),
            )
            .unwrap();
            let mut compiler = Command::new("rustc");
            compiler.args(["--edition", "2024"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                compiler.arg("-C").arg(format!("linker={linker}"));
            }
            let built = run(compiler.arg(file).arg("-o").arg(&binary));
            assert!(built.ok, "{} {owner}: {}", row["name"], built.stderr);
            let observed = run(&mut Command::new(binary));
            assert!(observed.ok, "{}", observed.stderr);
        }
    }
    for row in data["refusals"].as_array().unwrap() {
        assert!(
            parse_javascript(row["source"].as_str().unwrap())
                .and_then(|surface| check_program(&surface))
                .is_err(),
            "{}",
            row["name"]
        );
    }
}
