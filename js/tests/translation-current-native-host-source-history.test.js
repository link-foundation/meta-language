import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {restoreCurrentCiRepairSource} from '../scripts/current-ci-repair-source-history.mjs';
import {restoreContinuationHistoricalSource} from '../scripts/continuation-source-history.mjs';
import {restoreArrayRelativeElementSource} from '../scripts/array-relative-element-history.mjs';
import {CURRENT_NATIVE_HOST_FILES,pinCurrentNativeHostSource,readCurrentNativeHostSourceHistory,
  verifyCurrentNativeHostSourceHistory,restoreCurrentNativeHostSource} from '../scripts/current-native-host-source-history.mjs';
import {generateCurrentNativeHostSourceHistory} from '../scripts/generate-current-native-host-source-history.mjs';
import {readRequiredFieldNativeHostSuccession,restoreRequiredFieldNativeHostSource} from '../scripts/required-field-native-host-succession.mjs';
const root=new URL('../../',import.meta.url),read=file=>readFileSync(new URL(file,root));

test('current native source-history controls select an explicitly covered active Node range',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 || >=24 <25 required');
});

test('all four actual native hosts retain complete certified8d source views before their original closed chains',()=>{
  const packet=readCurrentNativeHostSourceHistory();
  assert.deepEqual(packet.sources.map(row=>row.file),CURRENT_NATIVE_HOST_FILES);
  const old=readProjectData(new URL('parity/self-translation/array-relative-element-history.lino',root));
  assert.equal(old.descriptorChanges.length,2);assert.equal(old.sources.length,24);
  for(const row of packet.sources){
    assert.equal(row.current.source,read(row.file).toString('utf8'));
    assert.equal(restoreCurrentNativeHostSource(row.current.source,row.file),row.previous.source);
    assert.equal(restoreCurrentNativeHostSource(row.previous.source,row.file),row.previous.source);
    assert.equal(restoreCurrentCiRepairSource(row.current.source,row.file),restoreCurrentCiRepairSource(row.previous.source,row.file));
    const expected=old.sources.find(source=>source.file===row.file);
    if(expected){
      assert.deepEqual(pinCurrentNativeHostSource(restoreContinuationHistoricalSource(row.current.source,row.file,'relative array')),expected.current);
      assert.equal(restoreArrayRelativeElementSource(row.current.source,row.file),expected.sourceText);
      assert.equal(restoreArrayRelativeElementSource(row.previous.source,row.file),expected.sourceText);
    }else{
      // Module registries and the ASCII host are not entries in the original
      // 24-source array inventory. Retain their actual inputs and check its
      // existing outer source view without inventing an array-specific owner.
      assert.equal(restoreArrayRelativeElementSource(row.current.source,row.file),
        restoreContinuationHistoricalSource(row.current.source,row.file,'relative array'));
    }
  }
});

test('unknown native successors remain visible and original strict source guards refuse them',()=>{
  const packet=readCurrentNativeHostSourceHistory();
  for(const row of packet.sources){
    const changed=row.current.source+'\n';
    assert.equal(restoreCurrentNativeHostSource(changed,row.file),changed);
    assert.equal(restoreCurrentCiRepairSource(changed,row.file),changed);
    if(row.file==='rust/src/self_translation.rs')assert.throws(()=>restoreArrayRelativeElementSource(changed,row.file),assert.AssertionError);
    else assert.equal(restoreArrayRelativeElementSource(changed,row.file),changed,'the unregistered array scope retains unknown source bytes');
  }
  for(const file of ['unregistered.rs','rust/src/translation/check.rs','Rust/src/self_translation.rs'])
    assert.equal(restoreCurrentNativeHostSource('unknown bytes',file),'unknown bytes');
});

test('required-field registration and ASCII readers retain their exact file and profile boundaries',()=>{
  const packet=readCurrentNativeHostSourceHistory(),required=readRequiredFieldNativeHostSuccession();
  for(const[file,reader]of [
    ['rust/src/translation/mod.rs','program string registration'],['rust/tests/unit/mod.rs','program string registration'],
    ['rust/src/translation/javascript/expressions/values.rs','ASCII character host'],
  ]){
    const current=packet.sources.find(row=>row.file===file),old=required.sources.find(row=>row.file===file);
    assert.equal(current.previous.source,old.current.source,'the separate certified8d memberships name the same complete historical host');
    for(const source of [current.current.source,current.previous.source])assert.equal(restoreRequiredFieldNativeHostSource(source,file,reader),old.previous.source);
    const changed=current.current.source+'\n';
    assert.equal(restoreRequiredFieldNativeHostSource(changed,file,reader),changed);
    for(const unknown of ['', 'unregistered reader','current behavior'])assert.equal(restoreRequiredFieldNativeHostSource(current.current.source,file,unknown),current.current.source);
  }
});

test('forged memberships coherent current pins row replacement and interval edits cannot acquire old source identity',()=>{
  const mutations=[
    packet=>{packet.revision='0'.repeat(40);},packet=>{packet.sources.reverse();},
    packet=>{packet.sources[0].file='unregistered.rs';},
    packet=>{packet.sources[0].previous.certificate.path='unregistered.rs';},
    packet=>{packet.sources[0].previous.source+='\n';Object.assign(packet.sources[0].previous,pinCurrentNativeHostSource(packet.sources[0].previous.source));},
    packet=>{packet.sources[1].current.source+='\n';Object.assign(packet.sources[1].current,pinCurrentNativeHostSource(packet.sources[1].current.source));},
    packet=>{packet.sources[2].edits=[];},packet=>{packet.sources[0].edits[0].removed+='\n';},
    packet=>{packet.sources[0].coordinateUnit='bytes';},packet=>{packet.additionalFiles=['unregistered.rs'];},
  ];
  for(const[index,mutate]of mutations.entries()){
    const packet=readCurrentNativeHostSourceHistory();mutate(packet);
    assert.throws(()=>verifyCurrentNativeHostSourceHistory(packet),assert.AssertionError,'tamper '+index);
  }
});

test('complete native host source identities and exact inverses survive both Links codecs',()=>{
  const packet=readCurrentNativeHostSourceHistory(),saved=encodeStructuredDataLinks(packet);
  const canonical=LinkNetwork.fromLino(saved.network.toLino());
  for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(network,saved.root);assert.deepEqual(restored,packet);
    verifyCurrentNativeHostSourceHistory(restored);
  }
});

test('one normal final-source capture changes no actual host or immutable earlier history packet',()=>{
  const files=[...CURRENT_NATIVE_HOST_FILES,'parity/self-translation/array-relative-element-history.lino',
    'parity/self-translation/ci-39389240046-historical-inputs.lino',
    'parity/self-translation/native-compatibility-source-sources-history.lino',
    'parity/self-translation/relative-array-frontend-source-history.lino'];
  const before=files.map(read),packet=generateCurrentNativeHostSourceHistory({check:true});
  assert.deepEqual(packet,readCurrentNativeHostSourceHistory());
  files.forEach((file,index)=>assert.deepEqual(read(file),before[index],file));
});
