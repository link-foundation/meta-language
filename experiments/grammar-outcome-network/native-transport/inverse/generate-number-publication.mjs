// Publish actual borrowed primitive observations after the original assertions.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';

const root=new URL('../../../../',import.meta.url);
const revision='74ca047cde2c923817096bc189f0d3b3892da512';
const testFile='rust/tests/unit/grammar_outcome_original_numbers.rs';
const modelFile='rust/tests/unit/native_json_number_observation_model.rs';
const outputFile='rust/tests/unit/native_json_number_publication.rs';
const fixtureFile='parity/fixtures/original-native-number-observations.lino';
const testName='original_borrowed_number_predicates_payloads_and_wire_positions_match_every_independent_control';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

/** Derive the complete record projection from the actual native observation type. */
export function generateNativeNumberPublication({check=false}={}) {
  const model=readFileSync(new URL(modelFile,root));
  const parsed=readRustSourceItems(model),declarations=parsed.items.filter(row=>row.kind==='struct_item'&&row.name==='NativeJsonNumberObservation');
  assert.equal(declarations.length,1);
  const declaration=declarations[0],body=declaration.node.children.find(child=>child.node.term==='field_declaration_list')?.node;
  assert.ok(body);
  const fields=body.children.filter(child=>child.node.term==='field_declaration').map(child=>{
    const name=child.node.children.find(item=>item.field==='name')?.node,type=child.node.children.find(item=>item.field==='type')?.node;
    assert.ok(name&&type);const sourceName=model.subarray(name.start,name.end).toString();
    assert.match(sourceName,/^[a-z]+(?:_[a-z]+)*$/u);
    return{sourceName,recordName:sourceName.replace(/_([a-z])/gu,(_,letter)=>letter.toUpperCase()),
      type:model.subarray(type.start,type.end).toString(),sourceText:model.subarray(child.node.start,child.node.end).toString(),start:child.node.start,end:child.node.end};
  });
  assert.equal(fields.length,7);assert.equal(new Set(fields.map(row=>row.recordName)).size,fields.length);
  const source=`// Generated host publication from every actual native observation field.
use super::original::NativeJsonNumberObservation;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::io::Write as _;

fn source_pin(bytes: &[u8]) -> Value {
    let digest = Sha256::digest(bytes).iter().fold(
        String::new(),
        |mut hexadecimal, byte| {
            write!(hexadecimal, "{byte:02x}").expect("writing into a String succeeds");
            hexadecimal
        },
    );
    json!({"bytes": bytes.len(), "sha256": digest})
}

pub(super) fn publish_original_number_controls(observations: &[NativeJsonNumberObservation]) -> Result<(), String> {
    if std::env::var("META_LANGUAGE_SHARD_PROFILE").as_deref() != Ok("evidence") {
        return Ok(());
    }
    if std::env::var("CI").as_deref() != Ok("true") || std::env::var("GITHUB_ACTIONS").as_deref() != Ok("true") {
        return Err("native number publication requires the actual CI evidence profile".to_owned());
    }
    let commit = std::env::var("GITHUB_SHA").map_err(|error| error.to_string())?;
    if commit.len() != 40 || !commit.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        || std::env::var("ISSUE_195_COMMIT").as_deref() != Ok(commit.as_str()) {
        return Err("native number publication requires the exact executed commit".to_owned());
    }
    if observations.len() > 128 {
        return Err("native number publication observation limit exceeded".to_owned());
    }
    let rows: Vec<Value> = observations.iter().map(|observed| json!({
        ${fields.map(row=>JSON.stringify(row.recordName)+': observed.'+row.sourceName).join(',\n        ')}
    })).collect();
    let report = json!({
        "schemaVersion": 1,
        "profile": "original native number controls",
        "testName": "grammar_outcome_original_numbers::${testName}",
        "commit": commit,
        "run": std::env::var("GITHUB_RUN_ID").map_err(|error| error.to_string())?,
        "attempt": std::env::var("GITHUB_RUN_ATTEMPT").map_err(|error| error.to_string())?,
        "targetUsizeBits": usize::BITS,
        "sources": {
            "test": source_pin(include_bytes!("grammar_outcome_original_numbers.rs")),
            "model": source_pin(include_bytes!("native_json_number_observation_model.rs")),
            "visitors": source_pin(include_bytes!("native_json_number_observation_visitors.rs")),
            "publisher": source_pin(include_bytes!("native_json_number_publication.rs")),
            "fixture": source_pin(include_bytes!("../../../${fixtureFile}")),
            "lock": source_pin(include_bytes!("../../Cargo.lock"))
        },
        "observations": rows
    });
    let record = meta_language::structured_data_record_to_lino(&report)?;
    if record.len() > 128 * 1024 || !record.ends_with('\\n') || record[..record.len()-1].contains('\\n') {
        return Err("native number publication frame limit exceeded".to_owned());
    }
    let frame = format!("meta-language original native number controls: {record}");
    std::io::stderr().lock().write_all(frame.as_bytes()).map_err(|error| error.to_string())
}
`;
  const formatted=execFileSync('rustfmt',['--edition','2024','--config','skip_children=true','--emit','stdout'],
    {input:source,encoding:'utf8',timeout:15000,maxBuffer:2*1024*1024});
  const original=captureGitArtifactCertificate(revision,testFile);
  verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:testFile});
  const originalItems=readRustSourceItems(original.blob).items,owners=originalItems.filter(row=>row.name===testName&&row.kind==='function_item');
  assert.equal(owners.length,1);const owner=owners[0];
  assert.equal(original.blob.subarray(owner.node.end-1,owner.node.end).toString(),'}');
  const moduleMarker=Buffer.from('mod original;\n'),moduleAt=original.blob.indexOf(moduleMarker);
  assert.ok(moduleAt>=0&&original.blob.indexOf(moduleMarker,moduleAt+1)===-1);
  const insertions=[{offset:moduleAt+moduleMarker.length,text:'#[path = "native_json_number_publication.rs"]\nmod publication;\n'},
    {offset:owner.node.end-1,text:'    publication::publish_original_number_controls(&actual).unwrap();\n'}];
  let derived=original.blob;
  for(const insertion of [...insertions].reverse())derived=Buffer.concat([derived.subarray(0,insertion.offset),Buffer.from(insertion.text),derived.subarray(insertion.offset)]);
  const actions=[
    {from:'fn original_number_limits()',to:'const fn original_number_limits()'},
    {from:'values.sort();',to:'values.sort_unstable();'},
    {from:'assert!(\n        original::capture_original_json_number_observations(&empty, ordinary)\n            .unwrap()\n            .is_empty()\n    );',
      to:'assert_eq!(\n        original::capture_original_json_number_observations(&empty, ordinary).unwrap(),\n        Vec::<original::NativeJsonNumberObservation>::new()\n    );'},
  ];
  let corrected=derived.toString();for(const action of actions){assert.equal(corrected.split(action.from).length,2);corrected=corrected.replace(action.from,action.to);}
  // Rustfmt can only change layout; the source actions and complete inverse
  // retain the original independent inputs and empty-result assertion.
  derived=Buffer.from(corrected);
  let inverse=derived;
  for(const action of [...actions].reverse()){const text=inverse.toString();assert.equal(text.split(action.to).length,2);inverse=Buffer.from(text.replace(action.to,action.from));}
  for(let index=insertions.length-1;index>=0;index--){const insertion=insertions[index],offset=insertion.offset+insertions.slice(0,index).reduce((sum,row)=>sum+Buffer.byteLength(row.text),0);
    assert.equal(inverse.subarray(offset,offset+Buffer.byteLength(insertion.text)).toString(),insertion.text);
    inverse=Buffer.concat([inverse.subarray(0,offset),inverse.subarray(offset+Buffer.byteLength(insertion.text))]);}
  assert.ok(inverse.equals(original.blob),'every original assertion and test input survives the exact full module inverse');
  const receipt={schemaVersion:1,revision,sources:[{file:testFile,commit:revision,...pin(original.blob),sourceText:original.blob.toString(),certificate:original.certificate,current:pin(derived),insertions,actions}],
    input:{file:modelFile,...pin(model),sourceText:model.toString(),declaration:{name:declaration.name,start:declaration.node.start,end:declaration.node.end,
      sourceText:model.subarray(declaration.node.start,declaration.node.end).toString()},fields},
    outputs:[{file:outputFile,...pin(formatted)},{file:testFile,...pin(derived)}],
    boundary:'Mechanical publication of all actual borrowed observation fields after every original independent assertion. Retained stderr and the actual physical receipt must agree. Compiler feature selection, complete native model reconstruction, public CST ownership and authenticated CI origin remain separate requirements.'};
  const receiptFile=new URL('../../../../parity/self-translation/native-number-publication.lino',import.meta.url);
  if(check){assert.equal(readFileSync(new URL(outputFile,root),'utf8'),formatted);assert.ok(readFileSync(new URL(testFile,root)).equals(derived));checkProjectData(receiptFile,receipt);}
  else{writeFileSync(new URL(outputFile,root),formatted);writeFileSync(new URL(testFile,root),derived);writeProjectData(receiptFile,receipt);}
  return receipt;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateNativeNumberPublication({check:process.argv.includes('--check')});
  console.log('Generated complete actual native observation field publication and preserved every original assertion.');
}
