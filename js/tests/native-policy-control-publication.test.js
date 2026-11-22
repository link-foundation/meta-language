// Deferred pure ownership/publication planning controls; no producer is run.
import assert from 'node:assert/strict';
import test from 'node:test';
import {createHash} from 'node:crypto';
import {encodeNativePolicyPublicationReceipt,readNativePolicyPublicationReceipt,planNativePolicyControlPublication,rustBaselineNativePolicyRegistration,rustBaselineNativePolicyPublicationManifest,rustBaselineMixedPredicateControlGroups} from '../scripts/draft-rust-baseline-native-policy-controls.mjs';
import {readStructuredDataLino,structuredDataToLino} from '../src/structured-data-links.js';
const first={path:'rust/tests/generated/authored_test_controls.rs',source:'fn main() {}\n#[test]\nfn controls() { main(); }\n'},receipt={path:'parity/self-translation/rust-baseline-native-policy-controls/authored_test_controls.lino',source:'()\n'};
const state=file=>({kind:'file',bytes:Buffer.byteLength(file.source),sha256:createHash('sha256').update(file.source).digest('hex')});
test('first publication claims only absent task-owned files and retains an exact bounded manifest',()=>{
 const files=[first,receipt],before=structuredClone(files),current=new Map(files.map(file=>[file.path,{kind:'missing'}]));
 const plan=planNativePolicyControlPublication({files,current});assert.deepEqual(plan.writes,files.map(file=>file.path));assert.equal(plan.manifest.files.length,2);assert.deepEqual(files,before);
 const repeat=planNativePolicyControlPublication({files,previous:plan.manifest,current:new Map(files.map(file=>[file.path,state(file)]))});assert.deepEqual(repeat.writes,[]);assert.deepEqual(repeat.manifest,plan.manifest);
});
test('changed generation updates only exact predecessor-owned contents and retains obsolete files without deletion',()=>{
 const initial=planNativePolicyControlPublication({files:[first,receipt],current:new Map([[first.path,{kind:'missing'}],[receipt.path,{kind:'missing'}]])});
 const next={...first,source:first.source+'// complete newly generated source\n'},plan=planNativePolicyControlPublication({files:[next],previous:initial.manifest,current:new Map([[first.path,state(first)],[receipt.path,state(receipt)]])});
 assert.deepEqual(plan.writes,[first.path]);assert.equal(plan.manifest.files.length,2);assert.deepEqual(plan.manifest.files.find(file=>file.path===receipt.path),initial.manifest.files.find(file=>file.path===receipt.path));
});
test('unexpected identical files edits missing predecessors and symlinks all remain preserved refusals',()=>{
 assert.throws(()=>planNativePolicyControlPublication({files:[first],current:new Map([[first.path,state(first)]])}),/unexpected unowned/u);
 const initial=planNativePolicyControlPublication({files:[first],current:new Map([[first.path,{kind:'missing'}]])});
 for(const changed of [state({...first,source:'// unrelated edited source'}),{kind:'missing'},{kind:'unsafe'}])assert.throws(()=>planNativePolicyControlPublication({files:[first],previous:initial.manifest,current:new Map([[first.path,changed]])}),/preserve/u);
 assert.throws(()=>planNativePolicyControlPublication({files:[first],previous:{schemaVersion:99,files:[]},current:new Map([[first.path,{kind:'missing'}]])}),/unrecognized/u);
});
test('output namespace and manifest ownership cannot be widened by a prepared plan',()=>{
 for(const path of ['rust/tests/unit/mod.rs','../outside.rs','rust/tests/generated/../unit/mod.rs',rustBaselineNativePolicyPublicationManifest])assert.throws(()=>planNativePolicyControlPublication({files:[{path,source:'new'}],current:new Map([[path,{kind:'missing'}]])}),/task-owned/u);
 assert.equal(rustBaselineNativePolicyRegistration,'rust/tests/generated/rust_baseline_native_policy_assertions.rs');
 assert.equal(rustBaselineMixedPredicateControlGroups.length,2);assert.deepEqual(rustBaselineMixedPredicateControlGroups[0].fixtureSelections[0].operations,['readRustOwnedStandardSumPredicateResultError']);
 assert.equal(rustBaselineMixedPredicateControlGroups[0].fixtureSelections[0].path,rustBaselineMixedPredicateControlGroups[1].fixtureSelections[0].path);assert.equal(rustBaselineMixedPredicateControlGroups[0].fixtureSelections[0].exportName,rustBaselineMixedPredicateControlGroups[1].fixtureSelections[0].exportName);
});
test('publication retains and authenticates the complete canonical receipt within its byte budget',()=>{
 const functionSource = 'pub fn exact_output() -> &' + "'static str { \"λ\" }\n";
 const receipt = { source: functionSource.repeat(10_000), coordinates: [0, 1] };
 const original = structuredDataToLino(receipt);
 const encoded = encodeNativePolicyPublicationReceipt(original);
 assert.ok(Buffer.byteLength(encoded) < Buffer.byteLength(original) / 10);
 assert.deepEqual(readNativePolicyPublicationReceipt(encoded), receipt);
 assert.deepEqual(readNativePolicyPublicationReceipt(original), receipt);
 const envelope = readStructuredDataLino(encoded);
 assert.throws(()=>readNativePolicyPublicationReceipt(structuredDataToLino({...envelope, sha256:'0'.repeat(64)})), /does not match its exact source/u);
 assert.throws(()=>readNativePolicyPublicationReceipt(structuredDataToLino({...envelope, bytes:32*1024*1024+1})), /invalid compressed native policy receipt/u);
});
