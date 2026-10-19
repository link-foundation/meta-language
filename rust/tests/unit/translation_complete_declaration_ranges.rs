//! Complete actual declaration intervals; native execution belongs to CI.
use std::path::Path;
use std::process::Command;

use meta_language::translation::{
    check::check_program,
    emit_javascript::emit_javascript,
    emit_rust::emit_rust,
    javascript::{parse_javascript, parse_javascript_with_scalar_loop_model},
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
};
#[path = "../../src/typed_program_links/source_span_rules.rs"]
mod complete_declaration_source_span_rules;

#[test]
fn complete_declaration_ranges_preserve_original_owners_values_targets_and_both_archives() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let script = r"import {readProjectData,serializeProjectData} from './js/scripts/project-data.mjs';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitRust} from './js/src/translation/emit-rust.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import vm from 'node:vm';
const fixture=readProjectData(process.argv[1]),observations=[];
for(const row of fixture.cases){
 let surface;
 try{surface=parseJavaScript(row.source,row.context??undefined);}
 catch(error){observations.push({id:row.id,error:{kind:error.kind,reason:error.reason,span:error.span}});continue;}
 const checked=checkProgram(surface),rust=emitRust(checked),javascript=emitJavaScript(checked);
 const original=await import('data:text/javascript;base64,'+Buffer.from(row.source).toString('base64'));
 const call=vm.runInNewContext(javascript.text+'\n;retained;',{}, {timeout:1000});
 const before=original.retained(...row.arguments),after=call(...row.arguments);
 if(!Object.is(before,after)||!Object.is(before,row.expectedValue))throw new Error('the entire original and emitted observations must agree');
 observations.push({id:row.id,surface,rust:rust.text,javascript:javascript.text,mappings:rust.mappings,value:before});
}
process.stdout.write(serializeProjectData({cases:observations},'complete-declaration-contract.lino'));
";
    let fixture_path = root.join("parity/fixtures/complete-declaration-source-ranges.lino");
    let output = Command::new("node")
        .current_dir(root)
        .args(["--input-type=module", "-e", script])
        .arg(&fixture_path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let contract = read_structured_data_lino(&String::from_utf8(output.stdout).unwrap()).unwrap();
    let fixture =
        read_structured_data_lino(&std::fs::read_to_string(fixture_path).unwrap()).unwrap();
    let rows = fixture["cases"].as_array().unwrap();
    let observed = contract["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 14);
    assert_eq!(observed.len(), rows.len());
    let mut accepted = 0;
    let mut refused = 0;
    for (row, javascript) in rows.iter().zip(observed) {
        assert_eq!(row["id"], javascript["id"]);
        let source = row["source"].as_str().unwrap();
        let result = row["context"]["scalarLoopModel"].as_str().map_or_else(
            || parse_javascript(source),
            |model| parse_javascript_with_scalar_loop_model(source, model),
        );
        if !row["expectedError"].is_null() {
            let error = result.unwrap_err();
            assert_eq!(
                error.kind.as_str(),
                javascript["error"]["kind"].as_str().unwrap()
            );
            assert_eq!(
                error.reason,
                javascript["error"]["reason"].as_str().unwrap()
            );
            assert_eq!(
                serde_json::to_value(error.span).unwrap(),
                javascript["error"]["span"]
            );
            assert_eq!(javascript["error"], row["expectedError"]);
            refused += 1;
            continue;
        }
        let surface = result.unwrap();
        let actual = serde_json::to_value(&surface).unwrap();
        assert_eq!(actual, javascript["surface"]);
        let owners = actual["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|item| item["name"] == "retained")
            .collect::<Vec<_>>();
        assert_eq!(owners.len(), 1);
        assert_eq!(owners[0]["span"], row["expectedSpan"]);
        assert_eq!(javascript["value"], row["expectedValue"]);
        let program = check_program(&surface).unwrap();
        let emitted = emit_rust(&program).unwrap();
        assert_eq!(emitted.text, javascript["rust"].as_str().unwrap());
        assert_eq!(
            serde_json::to_value(&emitted.mappings).unwrap(),
            javascript["mappings"]
        );
        assert_eq!(
            emit_javascript(&program).unwrap().text,
            javascript["javascript"].as_str().unwrap()
        );
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
            assert_eq!(emit_rust(&restored).unwrap(), emitted);
            assert_eq!(
                emit_javascript(&restored).unwrap().text,
                javascript["javascript"].as_str().unwrap()
            );
        }
        accepted += 1;
    }
    assert_eq!((accepted, refused), (13, 1));
}

#[test]
fn completed_declaration_interval_uses_the_original_scalar_offset_contract() {
    use complete_declaration_source_span_rules::accept_complete_declaration_source_interval;
    for [start, prefix, end] in [
        [0.0, 6.0, 50.0],
        [25.0, 31.0, 62.0],
        [0.0, 0.0, 0.0],
        [
            9_007_199_254_740_989.0,
            9_007_199_254_740_990.0,
            9_007_199_254_740_991.0,
        ],
    ] {
        assert!(accept_complete_declaration_source_interval(
            start, prefix, end
        ));
    }
    for [start, prefix, end] in [
        [-1.0, 0.0, 1.0],
        [0.0, -1.0, 1.0],
        [2.0, 1.0, 3.0],
        [0.0, 2.0, 1.0],
        [f64::NAN, 2.0, 3.0],
        [0.0, f64::INFINITY, 3.0],
        [0.0, 1.0, f64::INFINITY],
        [0.0, 1.0, 1.5],
        [0.0, 1.0, 9_007_199_254_740_992.0],
    ] {
        assert!(!accept_complete_declaration_source_interval(
            start, prefix, end
        ));
    }
}
