import assert from 'node:assert/strict';
import test from 'node:test';
import {summarizeRetainedSelfTranslationPhases,retainedSelfTranslationPhaseMarkdown} from '../scripts/self-translation-phase-summary.mjs';

const hash='a'.repeat(64),commit='1'.repeat(40),catalog=['js/src/a.js','js/src/b.js','js/src/c.js'];
const inventory=(module,rust=null)=>({module,sourceSha256:hash,rust,rustSha256:rust===null?null:'b'.repeat(64)});
function measurement(items,sourceBytes=10){
  const retained=structuredClone(items),bytesByStatus={};for(const item of retained)bytesByStatus[item.status]=(bytesByStatus[item.status]??0)+item.end-item.start;
  const itemBytes=Object.values(bytesByStatus).reduce((sum,value)=>sum+value,0);
  return{coverage:{sourceBytes,itemBytes,layoutBytes:sourceBytes-itemBytes,unrepresentedBytes:0,unrepresentedRanges:[],bytesByStatus},
    translation:{sourceLanguage:'JavaScript',targetLanguage:'Rust',items:retained}};
}
function row(module,items,decorated=items,rust=null){
  return{...inventory(module,rust),...measurement(items),items:Object.fromEntries(['translated','carried','comment'].map(kind=>[kind,items.filter(item=>item.status===kind).length])),
    decorated:measurement(decorated)};
}
const report=(modules,failures=[],selected=[...modules,...failures].map(row=>inventory(row.module,row.rust??null)))=>({
  schemaVersion:1,commit,decorators:['one'],sourceCatalog:catalog.slice(),inventory:selected,modules,failures});
const translated={term:'function_declaration',start:0,end:4,status:'translated',reason:null};
const carried={term:'class_declaration',start:5,end:9,status:'carried',reason:'unsupported'};

test('active Node selects explicit retained-report counter assertion versions',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; supported >=22.15.0 <23 || >=24 <25');
});

test('two retained modes keep complete shard denominators byte counters item kinds and exact counterpart subset',()=>{
  const first=report([row(catalog[0],[translated,carried],[translated],'rust/src/a.rs')]);
  const second=report([row(catalog[1],[translated],[translated,carried],'rust/src/a.rs')]);
  const summary=summarizeRetainedSelfTranslationPhases([first,second]);
  assert.equal(summary.commit,commit);assert.equal(summary.catalogModules,3);assert.equal(summary.selectedModules,2);
  assert.equal(summary.measuredModules,2);assert.equal(summary.failedModules,0);assert.equal(summary.unreportedCatalogModules,1);
  assert.equal(summary.rustCounterpartSubset.comparedSourceFiles,1);
  assert.equal(summary.rustCounterpartSubset.completeRustSourceInventorySize,null);
  assert.deepEqual(summary.modes.map(mode=>[mode.mode,mode.moduleMeasurements,mode.sourceBytes,mode.itemBytes,mode.layoutBytes,
    mode.items.translated,mode.items.carried,mode.bytesByItemKind.translated,mode.bytesByItemKind.carried,mode.modulesWithoutCarriedItems]),
    [['generic',2,20,12,8,2,1,8,4,1],['decorated',2,20,12,8,2,1,8,4,1]]);
  const isolated=summarizeRetainedSelfTranslationPhases([first]);
  assert.deepEqual(isolated.modes.map(mode=>[mode.mode,mode.items.carried,mode.itemBytes,mode.modulesWithoutCarriedItems]),
    [['generic',1,8,0],['decorated',0,4,1]],'the retained modes must not alias their actual different measurements');
  const before=structuredClone([first,second]);summarizeRetainedSelfTranslationPhases([first,second]);assert.deepEqual([first,second],before);
});

test('module failures preserve original selected denominator and missing catalogs stay unknown',()=>{
  const failure={module:catalog[1],error:'retained exact failure'},source=report([row(catalog[0],[translated])],[failure]);
  let summary=summarizeRetainedSelfTranslationPhases([source]);
  assert.equal(summary.selectedModules,2);assert.equal(summary.measuredModules,1);assert.equal(summary.failedModules,1);
  assert.equal(summary.unreportedCatalogModules,1);assert.equal(summary.modes[0].moduleMeasurements,1);
  const failedCounterpart=inventory(catalog[1],'rust/src/failed.rs');
  const selected=report([row(catalog[0],[translated])],[failure],[inventory(catalog[0]),failedCounterpart]);
  const subset=summarizeRetainedSelfTranslationPhases([selected]).rustCounterpartSubset;
  assert.equal(subset.selectedSourceFiles,1);assert.equal(subset.comparedSourceFiles,0,'a selected failing module has not compared its Rust counterpart');
  delete source.sourceCatalog;summary=summarizeRetainedSelfTranslationPhases([source]);
  assert.equal(summary.catalogModules,null);assert.equal(summary.unreportedCatalogModules,null);
});

test('zero carried items preserve every unknown CST complete-module and native-runtime phase',()=>{
  const summary=summarizeRetainedSelfTranslationPhases([report([row(catalog[0],[translated])])]);
  assert.equal(summary.modes[0].modulesWithoutCarriedItems,1);assert.equal(summary.unrecordedPhases.length,7);
  for(const phase of summary.unrecordedPhases)assert.equal(phase.moduleCount,null);
  assert.deepEqual(new Set(summary.unrecordedPhases.map(phase=>phase.executionRuntime)),new Set(['JavaScript','Rust']));
  assert.deepEqual(new Set(summary.unrecordedPhases.map(phase=>phase.sourceLanguage)),new Set(['JavaScript','Rust']));
  const markdown=retainedSelfTranslationPhaseMarkdown(summary);
  assert.match(markdown,/whole-module semantic success remains unmeasured/u);
  assert.match(markdown,/native-runtime corpus phases are not measured/u);
  assert.equal(Object.hasOwn(summary,'completion'),false);assert.equal(Object.hasOwn(summary,'status'),false);
});

test('comments carried originals empty modules and copied source never become translated item counters',()=>{
  const comment={term:'comment',start:0,end:4,status:'comment'},source=report([row(catalog[0],[comment,carried]),row(catalog[1],[])]);
  const summary=summarizeRetainedSelfTranslationPhases([source]);
  assert.equal(summary.modes[0].items.translated,0);assert.equal(summary.modes[0].modulesWithTranslatedItems,0);
  assert.equal(summary.modes[0].items.carried,1);assert.equal(summary.modes[0].items.comment,1);
  assert.equal(summary.modes[0].modulesWithoutCarriedItems,1,'an empty source item inventory is not semantic success');
});

test('missing outcomes duplicate shards changed commits invented source owners inconsistent ranges and counters refuse',()=>{
  // Build each complete retained control from a valid object, independently.
  const fresh=()=>report([row(catalog[0],[translated,carried])]);
  const mutations=[
    value=>{value.schemaVersion=2;},value=>{value.commit='invalid';},value=>{value.sourceCatalog.push(catalog[0]);},
    value=>{value.inventory.push(inventory(catalog[1]));},value=>{value.modules.push(structuredClone(value.modules[0]));},
    value=>{value.inventory[0].sourceSha256='x';},value=>{value.modules[0].sourceSha256='c'.repeat(64);},
    value=>{value.inventory[0].rust='rust/src/../outside.rs';value.inventory[0].rustSha256=hash;},
    value=>{value.inventory[0].module='js/src/../outside.js';},value=>{value.modules[0].translation.items[1].start=3;},
    value=>{value.modules[0].coverage.bytesByStatus.translated+=1;},value=>{value.modules[0].items.carried=0;},
    value=>{value.modules[0].coverage.unrepresentedBytes=1;},value=>{value.modules[0].decorated.translation.sourceLanguage='Rust';},
    value=>{delete value.modules[0].decorated;},value=>{value.sourceCatalog=[catalog[1],catalog[2]];},
  ];
  for(const[index,mutate]of mutations.entries()){
    const changed=fresh();mutate(changed);assert.throws(()=>summarizeRetainedSelfTranslationPhases([changed]),assert.AssertionError,'retained counter tamper '+index);
  }
  const source=fresh();assert.throws(()=>summarizeRetainedSelfTranslationPhases([source,structuredClone(source)]));
  const other=report([row(catalog[1],[translated])]);other.commit='2'.repeat(40);
  assert.throws(()=>summarizeRetainedSelfTranslationPhases([source,other]));
});
