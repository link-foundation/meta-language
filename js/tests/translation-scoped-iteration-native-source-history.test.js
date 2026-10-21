import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from '../src/structured-data-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readRequiredFieldNativeHostSuccession} from '../scripts/required-field-native-host-succession.mjs';
import {readCurrentNativeHostSourceHistory,restoreCurrentNativeHostSource} from '../scripts/current-native-host-source-history.mjs';
import {generateBooleanParameterTuples} from '../scripts/generate-boolean-parameter-tuples.mjs';
import {generateRequiredFieldNativeHostSuccession} from '../scripts/generate-required-field-native-host-succession.mjs';
import {SCOPED_ITERATION_NATIVE_SOURCE,SCOPED_ITERATION_NATIVE_LOOP_SOURCE,pinScopedIterationNativeSource,readScopedIterationNativeHostSourceHistory,
  verifyScopedIterationNativeHostSourceHistory,restoreScopedIterationNativeHostSource} from '../scripts/scoped-iteration-native-host-source-history.mjs';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {generateJavaScriptMutableArrayBindings} from '../scripts/generate-javascript-mutable-array-bindings.mjs';
import {generateScopedIterationNativeHostSourceHistory} from '../scripts/generate-scoped-iteration-native-host-source-history.mjs';
const root=new URL('../../',import.meta.url),read=file=>readFileSync(new URL(file,root));

test('source epoch controls select a covered Node runtime and keep the complete current and old declaration hosts',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 || >=24 <25 required');
  const packet=readScopedIterationNativeHostSourceHistory(),specification=SCOPED_ITERATION_NATIVE_SOURCE;
  assert.equal(packet.current.source,read(specification.file).toString('utf8'));
  assert.equal(packet.current.bytes-packet.previous.bytes,57);
  assert.equal(restoreScopedIterationNativeHostSource(packet.current.source,specification.file,'required field native host'),packet.previous.source);
  const old=readRequiredFieldNativeHostSuccession().sources.find(row=>row.file===specification.file);assert.ok(old);
  assert.equal(packet.previous.source,old.current.source,'the whole old required-field input remains independently exact');
  const current=readCurrentNativeHostSourceHistory();assert.equal(current.sources.length,4);
  assert.equal(current.sources.some(row=>row.file===specification.file),false,'the original four-source packet does not acquire an extra path');
});

test('unregistered readers changed source bytes and forged coherent memberships retain original refusals',()=>{
  const packet=readScopedIterationNativeHostSourceHistory(),file=SCOPED_ITERATION_NATIVE_SOURCE.file;
  for(const reader of ['', 'current behavior','ASCII character host','program string registration'])
    assert.equal(restoreScopedIterationNativeHostSource(packet.current.source,file,reader),packet.current.source);
  for(const path of ['unregistered.rs','Rust/src/translation/javascript/declarations.rs','rust/src/self_translation.rs'])
    assert.equal(restoreScopedIterationNativeHostSource(packet.current.source,path,'required field native host'),packet.current.source);
  for(const source of [packet.current.source+'\n',packet.current.source.slice(1),packet.previous.source])
    assert.equal(restoreScopedIterationNativeHostSource(source,file,'required field native host'),source);
  for(const mutate of [
    value=>{value.revision='0'.repeat(40);},value=>{value.previousRevision=value.revision;},
    value=>{value.current.certificate.path='unregistered.rs';},value=>{value.edits[0].start+=1;},
    value=>{value.current.source+='\n';Object.assign(value.current,pinScopedIterationNativeSource(value.current.source));},value=>{value.extra='unrepresented';},
  ]){const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyScopedIterationNativeHostSourceHistory(changed),assert.AssertionError);}
  const current=readCurrentNativeHostSourceHistory().sources.find(row=>row.file==='rust/src/self_translation.rs');assert.ok(current);
  assert.equal(restoreCurrentNativeHostSource(current.current.source+'\n',current.file),current.current.source+'\n','unknown Boolean caller edits acquire no historical identity');
});

test('both Links codecs retain the complete independent source certificates and exact source inverse',()=>{
  const packet=readScopedIterationNativeHostSourceHistory(),saved=encodeStructuredDataLinks(packet);
  const canonical=LinkNetwork.fromLino(saved.network.toLino());
  for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(network,saved.root);assert.deepEqual(restored,packet);
    verifyScopedIterationNativeHostSourceHistory(restored);
  }
});

test('normal source authors and the default historical tuple writer preserve every actual runtime and old packet byte',()=>{
  const files=[SCOPED_ITERATION_NATIVE_SOURCE.file,'rust/src/self_translation.rs','rust/tests/unit/grammar_outcome_number_kind_rules.rs',
    'parity/self-translation/boolean-parameter-tuples.lino','parity/self-translation/required-field-native-host-succession.lino',
    'parity/self-translation/ci-39539903548-native-host-source-history.lino'];
  const before=files.map(read);
  generateScopedIterationNativeHostSourceHistory({check:true});
  generateRequiredFieldNativeHostSuccession({check:true});
  generateBooleanParameterTuples({check:true});
  generateBooleanParameterTuples();
  files.forEach((file,index)=>assert.deepEqual(read(file),before[index],file));
});


test('the additional loop source preserves its original mutable-array host and the complete first declaration packet',()=>{
  const packet=readScopedIterationNativeHostSourceHistory(),row=packet.additionalSources[0],file=SCOPED_ITERATION_NATIVE_LOOP_SOURCE.file;
  assert.equal(row.current.source,read(file).toString('utf8'));assert.equal(row.current.bytes-row.previous.bytes,104);
  const {predecessor,additionalSources,...original}=packet;
  assert.deepEqual(pinScopedIterationNativeSource(serializeProjectData(original,'history.lino')),{bytes:predecessor.bytes,sha256:predecessor.sha256},'the entire committed first packet remains byte exact');
  const hosts=readProjectData(new URL('../../parity/self-translation/javascript-mutable-array-binding-hosts.lino',import.meta.url)),host=hosts.hosts.find(owner=>owner.path===file);assert.ok(host);
  let expected=host.source;for(const edit of host.edits){assert.equal(expected.slice(edit.start,edit.start+edit.removed.length),edit.removed);expected=expected.slice(0,edit.start)+edit.inserted+expected.slice(edit.start+edit.removed.length);}
  assert.equal(row.previous.source,expected,'the additional inverse reaches the exact original whole host proposal');
  assert.equal(restoreScopedIterationNativeHostSource(row.current.source,file,'mutable array binding host'),expected);
  for(const reader of['required field native host','ASCII character host','program string registration',''])assert.equal(restoreScopedIterationNativeHostSource(row.current.source,file,reader),row.current.source);
  for(const path of['unregistered.rs',packet.file,'Rust/src/translation/javascript/loops.rs'])assert.equal(restoreScopedIterationNativeHostSource(row.current.source,path,'mutable array binding host'),row.current.source);
  for(const source of[row.current.source+'\n',row.current.source.slice(1),row.previous.source])assert.equal(restoreScopedIterationNativeHostSource(source,file,'mutable array binding host'),source);
  for(const edit of[value=>{value.additionalSources[0].current.certificate.path=packet.file;},value=>{value.additionalSources[0].edits[0].start+=1;},value=>{value.additionalSources.push(structuredClone(value.additionalSources[0]));},value=>{value.predecessor.revision='0'.repeat(40);}]){const changed=structuredClone(packet);edit(changed);assert.throws(()=>verifyScopedIterationNativeHostSourceHistory(changed),assert.AssertionError);}
});

test('both mutable-array historical author modes preserve the complete current loop and original six-host packet',()=>{
  const file=SCOPED_ITERATION_NATIVE_LOOP_SOURCE.file,artifact='parity/self-translation/javascript-mutable-array-binding-hosts.lino',before=[read(file),read(artifact)];
  generateJavaScriptMutableArrayBindings({check:true});generateJavaScriptMutableArrayBindings();
  assert.deepEqual(read(file),before[0]);assert.deepEqual(read(artifact),before[1]);
});
