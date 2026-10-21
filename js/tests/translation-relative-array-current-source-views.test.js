import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {restoreRelativeArrayFrontendDescriptorSource} from '../scripts/relative-array-frontend-source-history.mjs';
import {restoreArrayRelativeElementSource} from '../scripts/array-relative-element-history.mjs';
import {RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE,RELATIVE_ARRAY_OWNED_RULE_SOURCE,RELATIVE_ARRAY_OWNED_RULE_OUTPUT,
  pinRelativeArrayCurrentSource,readRelativeArrayOwnedRuleSourceHistory,verifyRelativeArrayOwnedRuleSourceHistory,
  restoreRelativeArrayCurrentSourceView} from '../scripts/relative-array-current-source-views.mjs';
import {generateRelativeArrayOwnedRuleSourceHistory} from '../scripts/generate-relative-array-owned-rule-source-history.mjs';
const root=new URL('../../',import.meta.url),read=file=>readFileSync(new URL(file,root));
const targets=[
  ['js/src/translation/emit-javascript.js',35562,'b9967bc6024d7967e1c429ffdc5171d32db30042354104164c64ef1ba155414d'],
  ['js/src/typed-program-link-rules.js',18923,'fbffe8ff099b43b8f23fd5c35a0dda83ab711db4735ef3dc1a8239a87d5e5110'],
  [RELATIVE_ARRAY_OWNED_RULE_SOURCE,4830,'eb349133f29d2e5944ace3b96975d508a92d265231518ac2b99a604129c7b48a'],
];
test('the historical source view selects an explicitly covered active Node range',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 || >=24 <25 required');
});
test('three complete actual source owners recover their independent original relative-array descriptor pins',()=>{
  for(const[file,bytes,sha256]of targets){
    const current=read(file).toString('utf8'),previous=restoreRelativeArrayCurrentSourceView(current,file,RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE);
    assert.deepEqual(pinRelativeArrayCurrentSource(previous),{bytes,sha256},file);assert.notEqual(current,previous);
    assert.equal(restoreRelativeArrayCurrentSourceView(previous,file,RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE),previous);
    assert.equal(restoreRelativeArrayFrontendDescriptorSource(current,file,RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE),previous,'the unchanged historical generator enters this same view before its older frontend inverse');
    assert.equal(restoreArrayRelativeElementSource(current,file),restoreArrayRelativeElementSource(previous,file),'the original reader uses the same complete source view');
    assert.equal(read(file).toString('utf8'),current,'runtime source remains current');
  }
  const checker='js/src/translation/check.js',current=read(checker).toString('utf8');
  assert.equal(restoreRelativeArrayCurrentSourceView(current,checker,RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE),current,'the intended original two checker descriptor changes remain visible');
});
test('unknown sources profiles and paths cannot acquire a historical owner',()=>{
  for(const[file]of targets){
    const current=read(file).toString('utf8'),previous=restoreRelativeArrayCurrentSourceView(current,file,RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE);
    for(const source of [current+'\n',previous+'\n','unknown complete source'])
      assert.equal(restoreRelativeArrayCurrentSourceView(source,file,RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE),source);
    for(const profile of ['', 'original number source ranges','relative character','current behavior'])
      for(const source of [current,current+'\n'])assert.equal(restoreRelativeArrayCurrentSourceView(source,file,profile),source);
  }
  for(const file of ['unregistered.js','js/src/translation/check.js','JS/src/translation/owned-array-append-rules.js'])
    assert.equal(restoreRelativeArrayCurrentSourceView('unknown complete source',file,RELATIVE_ARRAY_CURRENT_SOURCE_PROFILE),'unknown complete source');
});
test('coherent pins foreign memberships altered source intervals and wider scopes cannot forge the owned-rule inverse',()=>{
  const mutations=[
    value=>{value.revision='0'.repeat(40);},value=>{value.source='unregistered.js';},
    value=>{value.previous.certificate.path='unregistered.js';},
    value=>{value.previous.source+='\n';Object.assign(value.previous,pinRelativeArrayCurrentSource(value.previous.source));},
    value=>{value.current.source+='\n';Object.assign(value.current,pinRelativeArrayCurrentSource(value.current.source));},
    value=>{value.edits=[];},value=>{value.edits[0].inserted+='\n';},
    value=>{value.coordinateUnit='bytes';},value=>{value.additionalFiles=['unregistered.js'];},
  ];
  for(const[index,mutate]of mutations.entries()){
    const value=readRelativeArrayOwnedRuleSourceHistory();mutate(value);
    assert.throws(()=>verifyRelativeArrayOwnedRuleSourceHistory(value),assert.AssertionError,'tamper '+index);
  }
  const changed=readRelativeArrayOwnedRuleSourceHistory();changed.current.source='caller mutation';
  assert.equal(readRelativeArrayOwnedRuleSourceHistory().current.source,read(RELATIVE_ARRAY_OWNED_RULE_SOURCE).toString('utf8'));
});
test('the complete whole-source proof survives both Links codecs without changing old source histories',()=>{
  const value=readRelativeArrayOwnedRuleSourceHistory(),saved=encodeStructuredDataLinks(value),canonical=LinkNetwork.fromLino(saved.network.toLino());
  for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(network,saved.root);assert.deepEqual(restored,value);verifyRelativeArrayOwnedRuleSourceHistory(restored);
  }
  const files=[...targets.map(([file])=>file),'js/scripts/generate-array-relative-element-history.mjs',
    'parity/self-translation/array-relative-element-history.lino','parity/self-translation/relative-array-frontend-source-history.lino',
    'parity/self-translation/current-own-codebase-raw-source-descriptors.lino','parity/self-translation/reviewed-overlay-provider-succession.lino',RELATIVE_ARRAY_OWNED_RULE_OUTPUT];
  const before=files.map(read);assert.deepEqual(generateRelativeArrayOwnedRuleSourceHistory({check:true}),value);
  files.forEach((file,index)=>assert.ok(read(file).equals(before[index]),file));
});
