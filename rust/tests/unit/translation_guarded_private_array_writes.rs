//! Actual source/frontend and source-free transport parity; native CI only.
use std::path::Path;
use std::process::Command;

use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    javascript::parse_javascript,
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
};
use serde_json::Value;

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

#[test]
fn guarded_writes_keep_actual_source_shape_target_emission_and_both_link_archives() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let script = "import {parseJavaScript} from './js/src/translation/javascript.js';import {checkProgram} from './js/src/translation/check.js';import {emitRust} from './js/src/translation/emit-rust.js';const surface=parseJavaScript(process.argv[1]);process.stdout.write(JSON.stringify({surface,rust:emitRust(checkProgram(surface)).text}));";
    let guards = [
        "Number.isInteger(position)",
        "position>=0",
        "position<data.length",
    ];
    let mut sources = Vec::new();
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let guard = order.map(|index| guards[index]).join("&&");
        sources.push((format!("/** @param {{number}} value @param {{number}} position @returns {{number[]}} */ function updated(value,position){{const data=[2,4,6];if({guard}){{data[position]=Math.abs(value);}}return data;}}console.log(updated(-7,1).join(','));console.log(updated(7,-1).join(','));console.log(updated(7,0.5).join(','));console.log(updated(7,NaN).join(','));"),"2,7,6\n2,4,6\n2,4,6\n2,4,6\n"));
    }
    sources.push(("/** @param {number} value @param {number} position @returns {number[]} */ function updated(value,position){const data=[2,4,6];if(Number.isInteger(position)&&position>=0&&position<data.length){data[position]=value;}if(Number.isSafeInteger(position)&&position>=0&&position<data.length){data[position]=value+data.length;}return data;}console.log(updated(7,1).join(','));".to_owned(),"2,10,6\n"));
    sources.push(("/** @param {boolean} value @param {number} position @returns {boolean[]} */ function updated(value,position){const data=[false,true];if(Number.isInteger(position)&&position>=0&&position<data.length){data[position]=value;}return data;}console.log(updated(true,0).join(','));".to_owned(),"true,true\n"));
    sources.push(("/** @param {string} value @param {number} position @returns {string[]} */ function updated(value,position){const data=['first','second'];if(Number.isInteger(position)&&position>=0&&position<data.length){data[position]=value;}return data;}console.log(updated('前😀',1).join(','));".to_owned(),"first,前😀\n"));
    for (source, expected) in sources {
        assert_eq!(observe(&source), expected, "independent unchanged source");
        let output = Command::new("node")
            .current_dir(root)
            .args(["--input-type=module", "-e", script, &source])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let javascript: Value = serde_json::from_slice(&output.stdout).unwrap();
        let surface = parse_javascript(&source).unwrap();
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            javascript["surface"]
        );
        let program = check_program(&surface).unwrap();
        let rust = emit_rust(&program).unwrap().text;
        assert_eq!(rust, javascript["rust"].as_str().unwrap());
        assert_eq!(observe(&emit_javascript(&program).unwrap().text), expected);
        let packet = encode_typed_program_links(&program).unwrap();
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
            assert_eq!(restored, program);
            assert_eq!(emit_rust(&restored).unwrap().text, rust);
            assert_eq!(observe(&emit_javascript(&restored).unwrap().text), expected);
        }
    }
}

#[test]
fn unproved_guard_alias_capture_call_and_loop_profiles_remain_translation_gaps() {
    let guard = "Number.isInteger(position)&&position>=0&&position<data.length";
    for body in [
        "const data=[2,4,6];data[position]=value;return data;".to_owned(),
        "const data=[2,4,6];if(position>=0&&position<data.length){data[position]=value;}return data;".to_owned(),
        "const data=[2,4,6];if(Number.isInteger(position)&&position>=0){data[position]=value;}return data;".to_owned(),
        format!("const data=[2,4,6],alias=data;if({guard}){{data[position]=value;}}return alias;"),
        format!("const data=[2,4,6];function capture(){{return data;}}if({guard}){{data[position]=value;}}return capture();"),
        format!("const data=[2,4,6];while(value>0){{if({guard}){{data[position]=value;}}value-=1;}}return data;"),
        format!("const data=[];if({guard}){{data[position]=value;}}return data;"),
        format!("const data=[2,4,6];function shift(){{position=100;return true;}}if({guard}&&shift()){{data[position]=value;}}return data;"),
        format!("const data=[2,4,6];function next(){{return value+1;}}if({guard}){{data[position]=next();}}return data;"),
    ] {
        let source=format!("/** @param {{number}} value @param {{number}} position @returns {{number[]}} */ function updated(value,position){{{body}}}");
        let result=parse_javascript(&source).and_then(|surface|check_program(&surface)).and_then(|program|emit_rust(&program));
        assert!(result.is_err(),"an unproved original write stays refused: {source}");
    }
}
