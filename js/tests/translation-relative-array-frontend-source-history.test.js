import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {readOwnCodebaseRawDescriptorPolicy,readOwnCodebaseRawDescriptorPacket} from '../scripts/own-codebase-raw-source-descriptors.mjs';
import {readStructuredDataLino,encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {createTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {generateRelativeArrayFrontendSourceHistory} from '../scripts/generate-relative-array-frontend-source-history.mjs';
import {restoreRelativeArrayCurrentFrontendSource} from '../scripts/relative-array-current-frontend-history.mjs';
import {RELATIVE_ARRAY_FRONTEND_PROFILE,RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE,replayRelativeArrayNullableInference,relativeArrayFrontendPin,readAuthenticatedCurrentRelativeArrayFrontend,
  verifyRelativeArrayFrontendSourceHistory,readRelativeArrayFrontendSourceHistory,
  restoreRelativeArrayFrontendDescriptorSource} from '../scripts/relative-array-frontend-source-history.mjs';
const root=new URL('../../',import.meta.url);
const read=file=>readFileSync(new URL(file,root));

test('active Node selects explicit relative-array frontend source assertion coverage',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,
    'uncovered Node '+process.versions.node+'; supported >=22.15.0 <23.0.0 || >=24.0.0 <25.0.0');
});

test('one complete certified historical frontend matches the original descriptor while current source retains its own authority',()=>{
  const history=readRelativeArrayFrontendSourceHistory(),policy=readOwnCodebaseRawDescriptorPolicy();
  const current=restoreRelativeArrayCurrentFrontendSource(read(policy.sourceFile).toString('utf8'),policy.sourceFile,RELATIVE_ARRAY_FRONTEND_PROFILE);
  assert.equal(current,readAuthenticatedCurrentRelativeArrayFrontend());
  assert.deepEqual(relativeArrayFrontendPin(current),policy.sourceOwner.current);
  const restored=restoreRelativeArrayFrontendDescriptorSource(current,policy.sourceFile,RELATIVE_ARRAY_FRONTEND_PROFILE);
  assert.equal(restored,history.historical.sourceText);
  assert.equal(restoreRelativeArrayFrontendDescriptorSource(restored,policy.sourceFile,RELATIVE_ARRAY_FRONTEND_PROFILE),restored);
  assert.deepEqual({file:history.historical.file,...relativeArrayFrontendPin(restored)},policy.artifactDescriptorPrevious);
  const predecessor=readStructuredDataLino(readOwnCodebaseRawDescriptorPacket().artifact.sourceText);
  const descriptors=predecessor.decisionSources.filter(row=>row.file===policy.sourceFile);assert.equal(descriptors.length,1);
  assert.deepEqual(descriptors[0],policy.artifactDescriptorPrevious);
  assert.notEqual(current,restored,'distinct complete source definitions are not claimed equal in meaning');
  const detached=readRelativeArrayFrontendSourceHistory();detached.historical.sourceText='caller mutation';
  assert.equal(restoreRelativeArrayFrontendDescriptorSource(current,policy.sourceFile,RELATIVE_ARRAY_FRONTEND_PROFILE),restored);
});

test('unrelated profiles and paths remain identities while selected unknown successors refuse',()=>{
  const policy=readOwnCodebaseRawDescriptorPolicy(),current=read(policy.sourceFile).toString('utf8');
  for(const profile of ['', 'original number source ranges','relative character','current behavior']){
    assert.equal(restoreRelativeArrayFrontendDescriptorSource(current,policy.sourceFile,profile),current);
    assert.equal(restoreRelativeArrayFrontendDescriptorSource(current+'\n',policy.sourceFile,profile),current+'\n');
  }
  for(const file of ['js/src/translation/check.js','unregistered.js','JS/src/translation/javascript.js']){
    assert.equal(restoreRelativeArrayFrontendDescriptorSource(current,file,RELATIVE_ARRAY_FRONTEND_PROFILE),current);
  }
  for(const source of [current+'\n',current.replace('const value = this.expr();','const value = null;'),'unknown complete frontend']){
    assert.throws(()=>restoreRelativeArrayFrontendDescriptorSource(source,policy.sourceFile,RELATIVE_ARRAY_FRONTEND_PROFILE));
  }
});

test('actual current frontend still executes Unicode string spread after both semantic codecs independently of its descriptor view',()=>{
  const source='/** @param {string} value @returns {string[]} */function points(value){return [...value];}';
  const values=['','A','Aπ','😀','A😀Z','\ud800','\udc00','\ud800x\udc00'];
  const saved=createTypedProgramLinks(source,'JavaScript');
  for(const network of [LinkNetwork.fromLino(saved.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]){
    const program=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,saved.root),{wholeProgram:true});
    const points=vm.runInNewContext(emitJavaScript(program).text+'\npoints;',{}, {timeout:1000});
    for(const value of values)assert.deepEqual(Array.from(points(value)),[...value]);
  }
  const wrong='/** @param {number} value */function points(value){return [...value];}';
  assert.throws(()=>createTypedProgramLinks(wrong,'JavaScript'),/iterable|iterator|array|spread/u);
});

test('coherent caller pins changed historical definitions forged membership and broader scopes cannot authenticate',()=>{
  const changes=[
    value=>{value.profile='current behavior';},value=>{value.sourceArchive.sha256='0'.repeat(64);},
    value=>{value.historical.file='unregistered.js';},value=>{value.historical.commit='0'.repeat(40);},
    value=>{value.historical.sourceText+='\n';Object.assign(value.historical,relativeArrayFrontendPin(value.historical.sourceText));},
    value=>{value.historical.certificate.path='unregistered.js';},
    value=>{value.currentAuthority.current.sha256='0'.repeat(64);},value=>{value.currentAuthority.file='unregistered.js';},
    value=>{value.host.sourceText+='\n';Object.assign(value.host,relativeArrayFrontendPin(value.host.sourceText));},
    value=>{value.host.edits.pop();},value=>{value.host.edits[1].inserted=value.host.edits[1].inserted.replace('relative array','current behavior');},
    value=>{value.host.certificate.revision='0'.repeat(40);},value=>{value.additionalProfiles=['current behavior'];},
  ];
  for(const[index,mutate]of changes.entries()){
    const value=readRelativeArrayFrontendSourceHistory();mutate(value);
    assert.throws(()=>verifyRelativeArrayFrontendSourceHistory(value),assert.AssertionError,'tamper '+index);
  }
});

test('the complete compact projection retains both source codecs and original two-change twenty-four-source receipt scope',()=>{
  const history=readRelativeArrayFrontendSourceHistory(),saved=encodeStructuredDataLinks(history);
  const canonical=LinkNetwork.fromLino(saved.network.toLino());
  for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(network,saved.root);assert.deepEqual(restored,history);
    assert.deepEqual(verifyRelativeArrayFrontendSourceHistory(restored),verifyRelativeArrayFrontendSourceHistory(history));
  }
  const old=readProjectData(new URL('parity/self-translation/array-relative-element-history.lino',root));
  assert.equal(old.sources.length,24);assert.equal(old.descriptorChanges.length,2);
  assert.deepEqual(old.descriptorChanges.map(change=>change.path.at(-1)),['bytes','sha256']);
  assert.deepEqual(old.descriptorChanges[0].path.slice(0,-1),old.descriptorChanges[1].path.slice(0,-1));
  assert.equal(old.descriptorChanges[0].path[0],'decisionSources');
});

test('one certified nullable inference successor restores both historical descriptor scopes and refuses unknown owners or source successors',()=>{
  const history=readRelativeArrayFrontendSourceHistory(),owner=RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE;
  const current=read(owner.path).toString('utf8'),previous=history.nullableInference.sourceText;
  assert.deepEqual(relativeArrayFrontendPin(previous),owner.previous);
  assert.deepEqual(relativeArrayFrontendPin(current),owner.current);
  assert.equal(replayRelativeArrayNullableInference(previous).source,current);
  assert.notEqual(current,previous);
  for(const source of [current,previous])assert.equal(restoreRelativeArrayFrontendDescriptorSource(source,owner.path,RELATIVE_ARRAY_FRONTEND_PROFILE),previous);
  for(const profile of ['', 'original number source ranges','relative character','current behavior']){
    assert.equal(restoreRelativeArrayFrontendDescriptorSource(current,owner.path,profile),current);
    assert.equal(restoreRelativeArrayFrontendDescriptorSource(current+'\n',owner.path,profile),current+'\n');
  }
  for(const source of [current+'\n',previous+'\n',current.replace('this.collections.set(node, object);','this.collections.set(node, result);')]){
    assert.throws(()=>restoreRelativeArrayFrontendDescriptorSource(source,owner.path,RELATIVE_ARRAY_FRONTEND_PROFILE));
  }
  for(const file of ['unregistered.js','js/src/translation/check.js','JS/src/translation/javascript-infer.js']){
    assert.equal(restoreRelativeArrayFrontendDescriptorSource(current,file,RELATIVE_ARRAY_FRONTEND_PROFILE),current);
  }
  const changes=[
    value=>{value.nullableInference.path='unregistered.js';},value=>{value.nullableInference.revision='0'.repeat(40);},
    value=>{value.nullableInference.sourceText+='\n';Object.assign(value.nullableInference,relativeArrayFrontendPin(value.nullableInference.sourceText));},
    value=>{value.nullableInference.current.sha256='0'.repeat(64);},value=>{value.nullableInference.certificate.path='unregistered.js';},
    value=>{value.nullableInference.edits[0].inserted+='\n';},value=>{value.nullableInference.edits=[];},
  ];
  for(const[index,mutate]of changes.entries()){
    const value=readRelativeArrayFrontendSourceHistory();mutate(value);
    assert.throws(()=>verifyRelativeArrayFrontendSourceHistory(value),assert.AssertionError,'nullable source tamper '+index);
  }
});

test('actual newer nullable spread inference retains every original CI output through both semantic codecs',()=>{
  const source=`function numberFirst() { return [1, null, undefined, -0].join('|'); }
function absenceFirst() { return [null, undefined, 1, -0].join('|'); }
function spreadAbsence() { return [1, ...[null, undefined], -0].join('|'); }
function nullableBigints() { return [1n, null, 3n].join('|'); }
console.log(numberFirst());
console.log(absenceFirst());
console.log(spreadAbsence());
console.log(nullableBigints());`;
  const expected=['1|||0','||1|0','1|||0','1||3'],saved=createTypedProgramLinks(source,'JavaScript');
  const printed=source=>{const lines=[];vm.runInNewContext(source,{console:{log:value=>lines.push(String(value))}},{timeout:1000});return lines;};
  assert.deepEqual(printed(source),expected);
  for(const network of [LinkNetwork.fromLino(saved.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]){
    const program=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,saved.root),{wholeProgram:true});
    assert.deepEqual(printed(emitJavaScript(program).text),expected);
  }
});

test('normal projection capture changes no existing frontend descriptor receipt or semantic artifact',()=>{
  const paths=['js/src/translation/javascript.js','js/src/translation/javascript-infer.js','parity/self-translation/array-relative-element-history.lino',
    'parity/self-translation/own-codebase-raw-source-descriptors.lino','parity/self-translation/complete-declaration-receipts.lino',
    'experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino',
    'experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino'];
  const before=paths.map(read),history=generateRelativeArrayFrontendSourceHistory({check:true});
  assert.deepEqual(history,readRelativeArrayFrontendSourceHistory());
  paths.forEach((file,index)=>assert.deepEqual(read(file),before[index]));
});
