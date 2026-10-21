import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {executePhysicalHarness,readPhysicalShardPolicy,selectPhysicalTests} from '../scripts/rust-test-shards.mjs';
import {structuredDataToLino,encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {structuredDataRecordToLino} from '../src/structured-data-records.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {parseNative} from '../src/native-grammar-parser.js';
import {generateNativeNumberPublication} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-number-publication.mjs';
import {readNativeNumberControlPublication} from '../../experiments/grammar-outcome-network/native-transport/inverse/number-publication.mjs';

const root=new URL('../../',import.meta.url);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const testName='grammar_outcome_original_numbers::original_borrowed_number_predicates_payloads_and_wire_positions_match_every_independent_control';
const marker='meta-language original native number controls: ';
const fixture=readProjectData(new URL('parity/fixtures/original-native-number-observations.lino',root));

// Explicitly synthetic transport controls never publish ledger observations.
async function numberPublicationControl(change=()=>{}) {
  const identity={commit:'a'.repeat(40),tree:'b'.repeat(40),run:'123',attempt:'2',profile:'evidence',platform:'ubuntu-latest',suite:'all'};
  const report={schemaVersion:1,profile:'original native number controls',testName,commit:identity.commit,run:identity.run,attempt:identity.attempt,
    targetUsizeBits:64,sources:Object.fromEntries(Object.entries({test:'rust/tests/unit/grammar_outcome_original_numbers.rs',model:'rust/tests/unit/native_json_number_observation_model.rs',
      visitors:'rust/tests/unit/native_json_number_observation_visitors.rs',publisher:'rust/tests/unit/native_json_number_publication.rs',
      fixture:'parity/fixtures/original-native-number-observations.lino',lock:'rust/Cargo.lock'}).map(([key,file])=>[key,pin(readFileSync(new URL(file,root)))])),
    observations:structuredClone(fixture.observations)};
  change(report);
  const standardError='retained native diagnostic\n'+marker+structuredDataRecordToLino(report);
  const standardOutput='test '+testName+' ... ok\n';
  const inventory=[{name:testName,kind:'test'}],policy=readPhysicalShardPolicy();
  const shard=Array.from({length:policy.evidence.shards},(_,index)=>index+1)
    .find(index=>selectPhysicalTests(inventory,index,policy.evidence.shards,policy.evidence.sharedScopes).length===1);
  const receipt=await executePhysicalHarness({binary:'/synthetic/rust/target/debug/deps/unit-abcdef01',arguments:['--nocapture'],identity,shard,count:policy.evidence.shards,
    invoke:async(_binary,_arguments,listing)=>({stdout:listing?testName+': test\n':standardOutput,stderr:listing?'':standardError,exitCode:0,signal:null})});
  return{identity,report,standardError,standardOutput,receipt};
}

function readNumberPublicationControl(control) {
  return readNativeNumberControlPublication(control.standardError,control.standardOutput,structuredDataToLino(control.receipt),structuredDataToLino(control.identity));
}

test('complete original native assertions and source contexts survive the additive generated publication',()=>{
  const receipt=generateNativeNumberPublication({check:true});
  const original=receipt.sources[0];
  verifyGitArtifactCertificate(original.certificate,Buffer.from(original.sourceText),{revision:original.commit,path:original.file});
  let current=readFileSync(new URL(original.file,root));assert.deepEqual(pin(current),original.current);
  for(const action of [...original.actions].reverse()){
    const source=current.toString();assert.equal(source.split(action.to).length,2);current=Buffer.from(source.replace(action.to,action.from));
  }
  for(let index=original.insertions.length-1;index>=0;index--){
    const insertion=original.insertions[index],offset=insertion.offset+original.insertions.slice(0,index).reduce((sum,row)=>sum+Buffer.byteLength(row.text),0);
    assert.equal(current.subarray(offset,offset+Buffer.byteLength(insertion.text)).toString(),insertion.text);
    current=Buffer.concat([current.subarray(0,offset),current.subarray(offset+Buffer.byteLength(insertion.text))]);
  }
  assert.equal(current.toString(),original.sourceText);assert.deepEqual(pin(current),{bytes:original.bytes,sha256:original.sha256});
  const source=readFileSync(new URL(receipt.outputs[0].file,root)),parsed=parseNative('native-rust',source.toString());
  assert.equal(parsed.hasError,false);assert.equal(parsed.end,source.length);
  const actual=source.toString();
  for(const field of receipt.input.fields){
    assert.equal(Buffer.from(receipt.input.sourceText).subarray(field.start,field.end).toString(),field.sourceText);
    assert.equal(actual.split('"'+field.recordName+'": observed.'+field.sourceName).length,2);
  }
  assert.equal(receipt.input.fields.length,7);assert.ok(actual.includes('usize::BITS'));assert.match(actual,/stderr\(\)\s*\.lock\(\)\s*\.write_all/u);
  assert.ok(!actual.includes('println!')&&!actual.includes('as_f64')&&!actual.includes('Number::from'));
  assert.ok(actual.includes('META_LANGUAGE_SHARD_PROFILE')&&actual.includes('GITHUB_ACTIONS')&&actual.includes('ISSUE_195_COMMIT'));
});

test('all actual primitive fields bind to complete successful physical logs and reload from both Links encodings',async()=>{
  const control=await numberPublicationControl(),result=readNumberPublicationControl(control);
  assert.deepEqual(result.observations,fixture.observations);assert.equal(result.targetUsizeBits,64);
  assert.deepEqual(result.identity,control.identity);assert.equal(result.execution.testName,testName);
  assert.deepEqual(result.execution.standardError,pin(control.standardError));assert.match(result.boundary,/does not authenticate/u);
  const packet=encodeStructuredDataLinks(result),network=LinkNetwork.fromLino(packet.network.toLino());
  for(const restored of [network,decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))])assert.deepEqual(decodeStructuredDataLinks(restored,packet.root),result);
  const narrow=await numberPublicationControl(report=>{report.targetUsizeBits=32;});assert.equal(readNumberPublicationControl(narrow).targetUsizeBits,32);
});

test('stale source commit run width and number facts refuse even with matching fabricated receipt hashes',async()=>{
  readNumberPublicationControl(await numberPublicationControl());
  for(const mutate of [report=>{report.commit='c'.repeat(40);},report=>{report.run='124';},report=>{report.attempt='3';},report=>{report.targetUsizeBits=16;},
    report=>{report.sources.test.sha256='0'.repeat(64);},report=>{report.sources.publisher.bytes++;},report=>{report.sources.lock.sha256='0'.repeat(64);},
    report=>{report.observations.pop();},report=>{report.observations.reverse();},report=>{report.observations[3].floatingBits='0000000000000000';},
    report=>{report.observations[2].signedDecimal='1152921504606846977';},report=>{report.observations[1].wireOccurrence=23;},
    report=>{report.observations[0].floating=true;},report=>{report.sources.foreign=report.sources.test;},report=>{report.extra=true;}]){
    const altered=await numberPublicationControl(mutate);assert.throws(()=>readNumberPublicationControl(altered));
  }
});

test('missing duplicate changed ignored or failed physical events and incomplete diagnostic frames refuse',async()=>{
  const original=await numberPublicationControl();readNumberPublicationControl(original);
  for(const mutate of [control=>{control.standardError+='changed';},control=>{control.standardOutput+='changed';},
    control=>{control.receipt.outcomes[0].outcome='ignored';},control=>{control.receipt.outcomes[0].outcome='FAILED';},
    control=>{control.receipt.outcomes=[];},control=>{control.receipt.outcomes.push(control.receipt.outcomes[0]);},
    control=>{control.receipt.selected=[];},control=>{control.receipt.identity.commit='c'.repeat(40);},
    control=>{control.receipt.exitCode=101;},control=>{control.receipt.signal='SIGTERM';},control=>{control.receipt.auxiliary=true;},
    control=>{control.receipt.arguments=[];},control=>{control.receipt.binary='foreign-abcdef';},
    control=>{control.identity.profile='test';},control=>{control.identity.platform='windows-latest';},
    control=>{control.standardError='unpublished\n';control.receipt.stderrDigest=pin(control.standardError).sha256;},
    control=>{control.standardOutput=control.standardOutput.replace(' ... ok',' ... FAILED');control.receipt.stdoutDigest=pin(control.standardOutput).sha256;},
    control=>{control.standardError+=marker+structuredDataRecordToLino(control.report);control.receipt.stderrDigest=pin(control.standardError).sha256;},
    control=>{control.standardError=control.standardError.replace(marker,'interleaved '+marker);control.receipt.stderrDigest=pin(control.standardError).sha256;},
    control=>{control.standardError=control.standardError.slice(0,-20);control.receipt.stderrDigest=pin(control.standardError).sha256;}]){
    const altered=structuredClone(original);mutate(altered);assert.throws(()=>readNumberPublicationControl(altered));
  }
  assert.throws(()=>readNativeNumberControlPublication({},'', '', ''),/bounded/u);
  assert.throws(()=>readNativeNumberControlPublication('\ud800','', '', ''),/bounded/u);
  assert.throws(()=>readNativeNumberControlPublication('x'.repeat(16*1024*1024+1),'', '', ''),/bounded/u);
});
