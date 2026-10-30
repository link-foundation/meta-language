//! The same retained arithmetic input and independent observations as JavaScript.
use super::super::issue_195_translation_pairs::{TemporaryDirectory, run};
use std::{fs, process::Command};

pub(super) fn check_original_arithmetic_binding(source: &str, translated: &str) {
    assert_eq!(
        source,
        "const LIMIT = 1 + 2;\n\n/** @returns {number} */\nexport function capture() { return LIMIT; }\n"
    );
    for (tool, native) in [("node", false), ("rustc", true)] {
        let active = run(Command::new(tool).arg("--version"));
        assert!(active.ok, "{}", active.stderr);
        let version = if native {
            active.stdout.split_whitespace().nth(1).unwrap()
        } else {
            active.stdout.trim().trim_start_matches('v')
        };
        let parts = version
            .split('.')
            .map(|part| part.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        assert!(
            if native {
                parts[0] == 1 && (90..100).contains(&parts[1])
            } else {
                parts[0] == 22 && parts[1] >= 15 || parts[0] == 24
            },
            "uncovered {tool} {version}; Rust >=1.90 <1.100, Node >=22.15 <23 or >=24 <25"
        );
    }
    let original = run(Command::new("node").args([
        "--input-type=module",
        "-e",
        &(source.to_owned() + "\nconsole.log(capture());\n"),
    ]));
    assert!(original.ok, "{}", original.stderr);
    assert_eq!(original.stdout, "3\n");
    let directory = TemporaryDirectory::new("retained-arithmetic-binding");
    let file = directory.0.join("capture.rs");
    let binary = directory
        .0
        .join(format!("capture{}", std::env::consts::EXE_SUFFIX));
    fs::write(
        &file,
        translated.to_owned() + "\nfn main(){println!(\"{}\",capture());}\n",
    )
    .unwrap();
    let mut command = Command::new("rustc");
    command.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        command.arg("-C").arg(format!("linker={linker}"));
    }
    let built = run(command.arg(&file).arg("-o").arg(&binary));
    assert!(built.ok, "{}", built.stderr);
    let observed = run(&mut Command::new(binary));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, original.stdout);
}
