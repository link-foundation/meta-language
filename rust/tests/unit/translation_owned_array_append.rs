//! Fresh source constructions, actual reader identity and source-free value replay.

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
};
use std::process::Command;

fn observe(source: &str) -> String {
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

fn replay(source: &str, expected: &str) {
    assert_eq!(observe(source), expected);
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let javascript = emit_javascript(&program).unwrap().text;
    let rust = emit_rust(&program).unwrap().text;
    assert_eq!(observe(&javascript), expected);
    let packet = encode_typed_program_links(&program).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(restored, program);
        let replayed_javascript = emit_javascript(&restored).unwrap().text;
        let replayed_rust = emit_rust(&restored).unwrap().text;
        assert_eq!(replayed_javascript, javascript);
        assert_eq!(replayed_rust, rust);
        assert_eq!(observe(&replayed_javascript), expected);
        assert!(replayed_rust.contains("crate::ml_array::append("));
    }
}

fn refuses(source: &str) {
    let result = parse_javascript(source).and_then(|surface| check_program(&surface));
    assert!(
        result.is_err(),
        "an unproved original construction remains refused: {source}"
    );
}

#[test]
fn fresh_variadic_constructions_replay_as_ordinary_immutable_values() {
    let source = "/** @param {number} count @returns {number[]} */ function build(count) { const values=[]; for(let index=0;index<count;index+=1) { values.push(index,index+1); } return values; }\nconst result=build(3);console.log(result[0]??-1,result[1]??-1,result[2]??-1,result[3]??-1,result[4]??-1,result[5]??-1);console.log(result.length,build(0).length);";
    replay(source, "0 1 1 2 2 3\n6 0\n");
    let local = "/** @returns {number} */ function build() { let count=0; for(let index=0;index<3;index+=1) { const values=[]; values.push(index); const finished=values; count+=finished.length; } return count; }\nconsole.log(build());";
    replay(local, "3\n");
}

#[test]
fn discarded_private_removals_preserve_empty_arrays_and_loop_state() {
    replay(
        "/** @returns {number[]} */ function build() { const values=[2,3]; values.pop(); values.pop(); values.pop(); values.push(5); for(let index=0;index<3;index+=1){values.push(index);values.pop();} return values; }\nconsole.log(build().join(','));",
        "5\n",
    );
    replay(
        "/** @returns {number[]} */ function build() { const values=[];values.pop();values.pop();values.push(7);return values;}\nconsole.log(build().join(','));",
        "7\n",
    );
}

#[test]
fn observed_removal_results_arguments_and_escaped_arrays_remain_refused() {
    for body in [
        "const alias=values;values.pop();return alias;",
        "const selected=values.pop();return [selected];",
        "values.pop(effect());return values;",
        "const read=()=>values;values.pop();return read();",
        "for(let index=0;index<2;index+=1){values.pop();const alias=values;}return values;",
    ] {
        refuses(&format!(
            "function effect(){{console.log('effect');return 1;}}function build(){{const values=[2,3];{body}}}"
        ));
    }
}

#[test]
fn original_alias_capture_recursive_and_loop_carried_escapes_stay_refused() {
    for body in [
        "const alias=values; values.push(1); return alias;",
        "observe(values); values.push(1); return values;",
        "const capture=()=>values; values.push(1); return capture();",
        "let index=0; while(index<2) { values.push(index); const alias=values; index+=1; } return values;",
        "values.push(values); return values;",
    ] {
        refuses(&format!(
            "/** @param {{number[]}} input */ function observe(input) {{ return input.length; }}\n/** @returns {{number[]}} */ function build() {{ const values=[]; {body} }}"
        ));
    }
}

#[test]
fn inspected_readers_retain_actual_global_identity_across_namespaces_and_shadows() {
    let reader = "/** @param {number[]} values @param {number} index @returns {number} */ function read(values,index) { return values[index] ?? -1; }\n";
    let build = "/** @returns {number[]} */ function build() { const pending=[2]; let index=0; while(index<3) { const next=read(pending,index); pending.push(next+1); index+=1; } return pending; }\n";
    let print = "const result=build();console.log(result[0]??-1,result[1]??-1,result[2]??-1,result[3]??-1);";
    replay(&(reader.to_owned() + build + print), "2 3 4 5\n");
    let loop_reader = reader.replace(
        "return values[index] ?? -1;",
        "let offset=0;while(offset<index){offset+=1;}return values[offset] ?? -1;",
    );
    replay(&(loop_reader + build + print), "2 3 4 5\n");
    let namespace = "const Other={ /** @param {number[]} values @param {number} index @returns {number} */ read(values,index) { const alias=values; return alias.length; } };\n";
    replay(
        &(reader.to_owned() + namespace + build + print),
        "2 3 4 5\n",
    );
    refuses(
        &(reader.to_owned()
            + &build.replace(
                "const pending=[2];",
                "const pending=[2]; const read=(values,index)=>values.length;",
            )),
    );
    refuses(
        &(reader.replace(
            "return values[index] ?? -1;",
            "const alias=values; return alias.length;",
        ) + build),
    );
    refuses(
        &(reader.to_owned()
            + namespace
            + &build.replace("read(pending,index)", "Other.read(pending,index)")),
    );
    let early_namespace = "const Other={ /** @param {number[]} values @param {number} index @returns {number} */ read(values,index) { return values[index] ?? -1; } };\n";
    refuses(
        &(early_namespace.to_owned()
            + build
            + &reader.replace(
                "return values[index] ?? -1;",
                "const alias=values; return alias.length;",
            )),
    );
}

// New tests below are CI-only: local contributors never compile Rust.
#[path = "../../src/translation/owned_array_append_rules.rs"]
mod self_slice_generated_rules;

fn self_slice_policy() -> serde_json::Value {
    meta_language::read_structured_data_lino(include_str!("../../src/data/owned-array-append.lino"))
        .unwrap()
}

fn self_slice_versions() {
    let policy = self_slice_policy();
    assert_eq!(
        policy["assertionVersions"]["Node"],
        ">=22.15.0 <23.0.0 || >=24.0.0 <25.0.0"
    );
    assert_eq!(policy["assertionVersions"]["Rust"], ">=1.90.0 <1.100.0");
    let node = super::issue_195_translation_pairs::run(Command::new("node").arg("--version"));
    assert!(node.ok, "{}", node.stderr);
    let node_version = node.stdout.trim().trim_start_matches('v');
    let parts: Vec<_> = node_version.split('.').collect();
    let major: u32 = parts[0].parse().unwrap();
    let minor: u32 = parts[1].parse().unwrap();
    assert!(
        (major == 22 && minor >= 15) || major == 24,
        "uncovered Node {node_version}"
    );
    let rust = super::issue_195_translation_pairs::run(Command::new("rustc").arg("--version"));
    assert!(rust.ok, "{}", rust.stderr);
    let rust_version = rust.stdout.split_whitespace().nth(1).unwrap();
    let parts: Vec<_> = rust_version.split('.').collect();
    let major: u32 = parts[0].parse().unwrap();
    let minor: u32 = parts[1].parse().unwrap();
    assert!(
        major == 1 && (90..100).contains(&minor),
        "uncovered Rust {rust_version}"
    );
}

#[test]
fn self_slice_generated_decision_retains_exact_owner_shape_and_arity_bounds() {
    self_slice_versions();
    assert!(
        self_slice_generated_rules::accept_owned_array_self_slice_rebinding(
            "copySlice",
            "",
            "name",
            1.0,
            true,
            0.0
        )
    );
    assert!(
        !self_slice_generated_rules::accept_owned_array_self_slice_rebinding(
            "copySlice",
            "slice",
            "name",
            1.0,
            true,
            0.0
        )
    );
    assert!(
        !self_slice_generated_rules::accept_owned_array_self_slice_rebinding(
            "copySlice",
            "",
            "name",
            1.0,
            false,
            0.0
        )
    );
    for count in [0.0, 1.0, 2.0] {
        assert!(
            self_slice_generated_rules::accept_owned_array_self_slice_rebinding(
                "method", "slice", "name", 1.0, true, count
            )
        );
    }
    for (expression, operation, receiver, length, same_owner, count) in [
        ("app", "slice", "name", 1.0, true, 1.0),
        ("method", "concat", "name", 1.0, true, 1.0),
        ("method", "slice", "field", 1.0, true, 1.0),
        ("method", "slice", "name", 2.0, true, 1.0),
        ("method", "slice", "name", 1.0, false, 1.0),
        ("method", "slice", "name", 1.0, true, -1.0),
        ("method", "slice", "name", 1.0, true, 0.5),
        ("method", "slice", "name", 1.0, true, 3.0),
        ("method", "slice", "name", 1.0, true, f64::NAN),
        ("method", "slice", "name", 1.0, true, f64::INFINITY),
    ] {
        assert!(
            !self_slice_generated_rules::accept_owned_array_self_slice_rebinding(
                expression, operation, receiver, length, same_owner, count
            )
        );
    }
}

#[test]
fn complete_self_slice_inventory_preserves_original_effects_both_codecs_and_native_execution() {
    self_slice_versions();
    let policy = self_slice_policy();
    let mut source = String::new();
    let mut expected = String::new();
    let fixtures = policy["selfSliceFixtures"].as_array().unwrap();
    assert_eq!(fixtures.len(), 7);
    for fixture in fixtures {
        let original = fixture["source"].as_str().unwrap();
        let output = fixture["expected"].as_str().unwrap();
        assert_eq!(observe(original), output, "{}", fixture["id"]);
        source.push_str(original);
        source.push('\n');
        expected.push_str(output);
    }
    assert_eq!(observe(&source), expected);
    let program = check_program(&parse_javascript(&source).unwrap()).unwrap();
    let native = emit_rust(&program).unwrap().text;
    assert!(native.contains("crate::ml_array::append("));
    let packet = encode_typed_program_links(&program).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(restored, program);
        assert_eq!(observe(&emit_javascript(&restored).unwrap().text), expected);
        assert_eq!(emit_rust(&restored).unwrap().text, native);
    }
    // One CI-only native compilation for the complete inventory. Both codec
    // products above are byte-identical, so no duplicate native work is needed.
    let directory =
        super::issue_195_translation_pairs::TemporaryDirectory::new("owned-array-self-slice");
    let input = directory.0.join("program.rs");
    let binary = directory
        .0
        .join(format!("program{}", std::env::consts::EXE_SUFFIX));
    std::fs::write(&input, &native).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled =
        super::issue_195_translation_pairs::run(compiler.arg(&input).arg("-o").arg(&binary));
    assert!(compiled.ok, "{}\n{}", compiled.stdout, compiled.stderr);
    let executed = super::issue_195_translation_pairs::run(&mut Command::new(binary));
    assert!(executed.ok, "{}", executed.stderr);
    assert_eq!(executed.stdout, expected);
}

#[test]
fn self_slice_never_erases_prior_later_identity_capture_or_argument_escapes() {
    self_slice_versions();
    let policy = self_slice_policy();
    let fixtures = policy["selfSliceRefusals"].as_array().unwrap();
    assert_eq!(fixtures.len(), 8);
    for fixture in fixtures {
        let source = fixture["source"].as_str().unwrap();
        assert_eq!(
            observe(source),
            fixture["expected"].as_str().unwrap(),
            "{}",
            fixture["id"]
        );
        refuses(source);
    }
    refuses(
        "/** @param {number[]} values */ function selfSliceParameter(values){values=values.slice(1);values.push(2);return values;}\nconsole.log(selfSliceParameter([1]).join(','));",
    );
}

#[test]
fn inspected_scalar_iteration_admits_self_copy_while_live_iteration_still_escapes() {
    self_slice_versions();
    let reader = "/** @param {number[]} values @param {number} wanted @returns {boolean} */ function contains(values,wanted){for(const value of values){if(value===wanted)return true;}return false;}";
    let build = "/** @returns {number[]} */ function build(){let values=[1];while(values.length<4){values=values.slice();if(!contains(values,values.length+1))values.push(values.length+1);}return values;}\nconsole.log(build().join(\",\"));";
    let source = format!("{reader}\n{build}");
    assert_eq!(observe(&source), "1,2,3,4\n");
    let program = check_program(&parse_javascript(&source).unwrap()).unwrap();
    let native = emit_rust(&program).unwrap().text;
    let packet = encode_typed_program_links(&program).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let replay = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(replay, program);
        assert_eq!(
            observe(&emit_javascript(&replay).unwrap().text),
            "1,2,3,4\n"
        );
        assert_eq!(emit_rust(&replay).unwrap().text, native);
    }
    let escaping = source.replace(
        "for(const value of values)",
        "const alias=values;for(const value of alias)",
    );
    assert_eq!(observe(&escaping), "1,2,3,4\n");
    refuses(&escaping);
    let live = "function live(){let values=[1];for(const value of values){if(values.length<3)values.push(value+1);}return values.join(\",\");}\nconsole.log(live());";
    assert_eq!(observe(live), "1,2,3\n");
    refuses(live);
}

#[test]
fn actual_completed_string_iteration_borrows_keep_source_semantics_and_live_alias_refusals() {
    self_slice_versions();
    let policy = self_slice_policy();
    let mut source = String::new();
    let mut expected = String::new();
    for fixture in policy["scopedIterationFixtures"].as_array().unwrap() {
        let original = fixture["source"].as_str().unwrap();
        let output = fixture["expected"].as_str().unwrap();
        assert_eq!(observe(original), output, "{}", fixture["id"]);
        replay(original, output);
        source.push_str(original);
        source.push('\n');
        expected.push_str(output);
    }
    let program = check_program(&parse_javascript(&source).unwrap()).unwrap();
    let native = emit_rust(&program).unwrap().text;
    let directory =
        super::issue_195_translation_pairs::TemporaryDirectory::new("owned-array-scoped-iteration");
    let input = directory.0.join("program.rs");
    let binary = directory
        .0
        .join(format!("program{}", std::env::consts::EXE_SUFFIX));
    std::fs::write(&input, native).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled =
        super::issue_195_translation_pairs::run(compiler.arg(&input).arg("-o").arg(&binary));
    assert!(compiled.ok, "{}", compiled.stderr);
    let executed = super::issue_195_translation_pairs::run(&mut Command::new(binary));
    assert!(executed.ok, "{}", executed.stderr);
    assert_eq!(executed.stdout, expected);
    for fixture in policy["scopedIterationRefusals"].as_array().unwrap() {
        let original = fixture["source"].as_str().unwrap();
        assert_eq!(
            observe(original),
            fixture["expected"].as_str().unwrap(),
            "{}",
            fixture["id"]
        );
        refuses(original);
    }
}
