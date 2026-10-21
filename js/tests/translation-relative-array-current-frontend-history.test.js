import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {createTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {generateRelativeArrayCurrentFrontendHistory} from '../scripts/generate-relative-array-current-frontend-history.mjs';
import {RELATIVE_ARRAY_CURRENT_FRONTEND,RELATIVE_ARRAY_CURRENT_FRONTEND_ANCHORS,
  pinRelativeArrayCurrentFrontend,readRelativeArrayCurrentFrontendHistory,
  verifyRelativeArrayCurrentFrontendHistory,restoreRelativeArrayCurrentFrontendSource} from '../scripts/relative-array-current-frontend-history.mjs';
import {readRelativeArrayFrontendSourceHistory,restoreRelativeArrayFrontendDescriptorSource} from '../scripts/relative-array-frontend-source-history.mjs';
const root=new URL('../../',import.meta.url),read=path=>readFileSync(new URL(path,root));
const archives=network=>{
  // Structured-data encoding returns a bootstrap store. The public binary
  // codec accepts LinkNetwork, so materialize the same complete Links first.
  const concrete=network instanceof LinkNetwork?network:LinkNetwork.fromLino(network.toLino());
  return [LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))];
};

test('current frontend source views select an explicitly covered active Node range',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 || >=24 <25 required');
});

test('both certified commits and complete actual frontend compose into the unchanged older relative-array inverse',()=>{
  const packet=readRelativeArrayCurrentFrontendHistory(),old=readRelativeArrayFrontendSourceHistory();
  assert.equal(packet.current.source,read(RELATIVE_ARRAY_CURRENT_FRONTEND).toString('utf8'));
  assert.deepEqual(packet.anchors.map(({revision,bytes,sha256})=>({revision,bytes,sha256})),RELATIVE_ARRAY_CURRENT_FRONTEND_ANCHORS);
  for(const row of [...packet.anchors,packet.current]){
    assert.equal(restoreRelativeArrayCurrentFrontendSource(row.source,packet.file,'relative array'),packet.anchors[0].source);
    assert.equal(restoreRelativeArrayFrontendDescriptorSource(row.source,packet.file,'relative array'),old.historical.sourceText);
  }
  assert.equal(restoreRelativeArrayCurrentFrontendSource(old.historical.sourceText,packet.file,'relative array'),old.historical.sourceText,'already historical complete source stays byte-identical');
  assert.equal(restoreRelativeArrayFrontendDescriptorSource(old.historical.sourceText,packet.file,'relative array'),old.historical.sourceText);
  const detached=readRelativeArrayCurrentFrontendHistory();detached.current.source='caller mutation';
  assert.equal(readRelativeArrayCurrentFrontendHistory().current.source,packet.current.source);
});

test('current frontend succession keeps other scopes visible and refuses every unknown selected source',()=>{
  const packet=readRelativeArrayCurrentFrontendHistory(),source=packet.current.source;
  for(const profile of ['', 'relative character','current behavior']){
    assert.equal(restoreRelativeArrayCurrentFrontendSource(source,packet.file,profile),source);
    assert.equal(restoreRelativeArrayCurrentFrontendSource(source+'\n',packet.file,profile),source+'\n');
  }
  for(const path of ['unregistered.js','js/src/translation/check.js','JS/src/translation/javascript.js'])
    assert.equal(restoreRelativeArrayCurrentFrontendSource(source,path,'relative array'),source);
  for(const changed of [source+'\n',packet.anchors[0].source+'\n',packet.anchors[1].source+'\n','unknown complete source']){
    assert.throws(()=>restoreRelativeArrayCurrentFrontendSource(changed,packet.file,'relative array'),assert.AssertionError);
    assert.throws(()=>restoreRelativeArrayFrontendDescriptorSource(changed,packet.file,'relative array'),assert.AssertionError);
  }
});

test('coherent pins cannot forge either Git owner the actual source or any complete inverse',()=>{
  const mutations=[
    packet=>{packet.profile='current behavior';},packet=>{packet.file='unregistered.js';},
    packet=>{packet.sourceArchive.sha256='0'.repeat(64);},
    packet=>{packet.anchors.reverse();},packet=>{packet.anchors[0].revision='0'.repeat(40);},
    packet=>{packet.anchors[0].certificate.path='unregistered.js';},
    packet=>{packet.anchors[1].certificate.revision=packet.anchors[0].revision;},
    packet=>{packet.anchors[1].source+='\n';Object.assign(packet.anchors[1],pinRelativeArrayCurrentFrontend(packet.anchors[1].source));},
    packet=>{packet.current.source+='\n';Object.assign(packet.current,pinRelativeArrayCurrentFrontend(packet.current.source));},
    packet=>{packet.edges[0].edits.pop();},packet=>{packet.edges[1].edits[0].inserted+='\n';},
    packet=>{packet.edges[1].coordinateUnit='bytes';},packet=>{packet.additionalProfiles=['current behavior'];},
  ];
  for(const[index,mutate]of mutations.entries()){
    const packet=readRelativeArrayCurrentFrontendHistory();mutate(packet);
    assert.throws(()=>verifyRelativeArrayCurrentFrontendHistory(packet),assert.AssertionError,'tamper '+index);
  }
});

test('the complete current succession survives both Links codecs without changing any older source definition',()=>{
  const packet=readRelativeArrayCurrentFrontendHistory(),saved=encodeStructuredDataLinks(packet);
  for(const network of archives(saved.network)){
    const restored=decodeStructuredDataLinks(network,saved.root);
    assert.deepEqual(restored,packet);
    assert.equal(verifyRelativeArrayCurrentFrontendHistory(restored),packet.anchors[0].source);
  }
});

test('actual required parameters still parse check and emit independently of every historical frontend view',()=>{
  const source='function currentRequiredLength({length}){return length;}function observeCurrentRequiredLength(){return currentRequiredLength("A😀");}';
  assert.equal(vm.runInNewContext(source+'\nobserveCurrentRequiredLength();'),3);
  const packet=createTypedProgramLinks(source,'JavaScript');
  let native;
  for(const network of archives(packet.network)){
    const program=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    assert.equal(vm.runInNewContext(emitJavaScript(program).text+'\nobserveCurrentRequiredLength();'),3);
    const output=emitRust(program).text;
    if(native===undefined)native=output;else assert.equal(output,native);
  }
});

test('one normal current-source refresh preserves all original immutable packets descriptors and source bytes',()=>{
  const paths=[RELATIVE_ARRAY_CURRENT_FRONTEND,'parity/self-translation/relative-array-frontend-source-history.lino',
    'parity/self-translation/array-relative-element-history.lino','parity/self-translation/own-codebase-raw-source-descriptors.lino',
    'parity/self-translation/complete-declaration-receipts.lino',
    'experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino',
    'experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino'];
  const before=paths.map(read);
  const packet=generateRelativeArrayCurrentFrontendHistory({check:true});
  assert.deepEqual(packet,readRelativeArrayCurrentFrontendHistory());
  paths.forEach((path,index)=>assert.deepEqual(read(path),before[index],path));
});
