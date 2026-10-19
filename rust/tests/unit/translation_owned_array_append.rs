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
