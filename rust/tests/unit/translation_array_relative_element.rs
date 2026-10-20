//! Complete relative-array programs execute through both translators in CI only.
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_typed_program_links, decode_link_network_binary,
    decode_typed_program_links, emit_typed_program_links, encode_link_network_binary,
};
use std::fs;
use std::process::Command;

fn array_relative_javascript(source: &str) -> String {
    let output = Command::new("node")
        .args(["--input-type=module", "-e", source])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn array_relative_native(source: &str, index: usize) -> String {
    assert_eq!(
        std::env::var("CI").as_deref(),
        Ok("true"),
        "Rust compilation is CI-only"
    );
    assert_eq!(
        std::env::var("GITHUB_ACTIONS").as_deref(),
        Ok("true"),
        "native execution requires the actual CI runner"
    );
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "meta-language-relative-array-{}-{nonce}-{index}",
        std::process::id()
    ));
    fs::create_dir(&directory).unwrap();
    let file = directory.join("program.rs");
    let executable = directory.join(format!("program{}", std::env::consts::EXE_SUFFIX));
    fs::write(&file, source).unwrap();
    let compiled = Command::new("rustc")
        .arg("--edition=2024")
        .arg(&file)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{}; source retained at {}",
        String::from_utf8_lossy(&compiled.stderr),
        file.display()
    );
    let output = Command::new(&executable).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    fs::remove_dir_all(directory).unwrap();
    stdout
}

#[test]
fn relative_arrays_preserve_complete_original_programs_through_native_execution_and_both_archives()
{
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/array-relative-element-observations.lino"
    ))
    .unwrap();
    let programs = fixture["programs"].as_array().unwrap();
    assert_eq!(programs.len(), 2);
    assert_eq!(fixture["positions"].as_array().unwrap().len(), 22);
    for (index, row) in programs.iter().enumerate() {
        let source = row["source"].as_str().unwrap();
        let expected = row["expectedStdout"].as_str().unwrap();
        assert_eq!(array_relative_javascript(source), expected);
        let program = check_program(&parse_javascript(source).unwrap()).unwrap();
        assert_eq!(
            array_relative_javascript(&emit_javascript(&program).unwrap().text),
            expected
        );
        let native = emit_rust(&program).unwrap().text;
        assert_eq!(array_relative_native(&native, index), expected);
        let packet = create_typed_program_links(source, "JavaScript").unwrap();
        for network in [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
                    .unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            let restored = decode_typed_program_links(&network, packet.root).unwrap();
            assert!(
                !serde_json::to_string(&restored)
                    .unwrap()
                    .contains("\"op\":\"at\"")
            );
            assert_eq!(
                emit_typed_program_links(&network, packet.root, "Rust")
                    .unwrap()
                    .code,
                native
            );
            assert_eq!(
                array_relative_javascript(
                    &emit_typed_program_links(&network, packet.root, "JavaScript")
                        .unwrap()
                        .code
                ),
                expected
            );
        }
        let translated = meta_language::self_translate(source, "JavaScript", "Rust").unwrap();
        assert_eq!(
            translated
                .items
                .iter()
                .filter(|item| item.status == "carried")
                .count(),
            0
        );
        assert_eq!(
            meta_language::self_translate(&translated.code, "Rust", "JavaScript")
                .unwrap()
                .code,
            source
        );
    }
}
