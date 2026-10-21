// Diagnostic aggregation of retained report objects only. No source program,
// grammar, source reconstruction, translator or native toolchain runs here.
import assert from 'node:assert/strict';
import {requireReportSourcePath} from './self-translation-report-blockers.mjs';

const ITEM_KINDS=['translated','carried','comment','kept','restored','provenance'];
const natural=(value,label)=>{assert.ok(Number.isSafeInteger(value)&&value>=0,label);return value;};
const hash=(value,label)=>assert.match(value??'',/^[a-f0-9]{64}$/u,label);

function readRetainedItemMeasurement(row,mode){
  const measured=mode==='generic'?row:row.decorated,coverage=measured?.coverage,translation=measured?.translation;
  assert.ok(coverage&&translation,'both retained mode measurements are required');
  assert.equal(translation.sourceLanguage,'JavaScript');assert.equal(translation.targetLanguage,'Rust');
  assert.ok(Array.isArray(translation.items));
  for(const field of ['sourceBytes','itemBytes','layoutBytes','unrepresentedBytes'])natural(coverage[field],field);
  assert.equal(coverage.unrepresentedBytes,0,'successful measured rows retain no omitted source bytes');
  assert.deepEqual(coverage.unrepresentedRanges,[]);
  assert.equal(coverage.itemBytes+coverage.layoutBytes,coverage.sourceBytes,'recorded complete byte accounting');
  const items=Object.fromEntries(ITEM_KINDS.map(kind=>[kind,0])),bytes={};let end=0;
  for(const item of translation.items){
    assert.ok(ITEM_KINDS.includes(item.status),'known retained item outcome');
    natural(item.start,'item start');natural(item.end,'item end');
    assert.ok(item.start>=end&&item.end>item.start&&item.end<=coverage.sourceBytes,'ordered nonoverlapping complete item ranges');
    items[item.status]+=1;bytes[item.status]=(bytes[item.status]??0)+item.end-item.start;end=item.end;
  }
  assert.deepEqual(bytes,coverage.bytesByStatus,'retained source ranges and byte counters agree');
  assert.equal(Object.values(bytes).reduce((sum,value)=>sum+value,0),coverage.itemBytes);
  if(mode==='generic')for(const kind of ['translated','carried','comment'])assert.equal(row.items?.[kind],items[kind]);
  return{sourceBytes:coverage.sourceBytes,itemBytes:coverage.itemBytes,layoutBytes:coverage.layoutBytes,items,bytes};
}

/** Keep the exact original source inventory and generic/decorated denominators.
 * A zero carried-item counter is not a whole-module semantic observation. */
export function summarizeRetainedSelfTranslationPhases(reports){
  assert.ok(Array.isArray(reports)&&reports.length>0);
  const commit=reports[0].commit,decorators=reports[0].decorators??null;
  assert.match(commit??'',/^[a-f0-9]{40}$/u);
  const inventory=new Map(),counterparts=new Map(),comparedCounterparts=new Set();let catalog=null,measuredModules=0,failedModules=0,withoutCounterpart=0;
  const modes=['generic','decorated'].map(mode=>({mode,moduleMeasurements:0,sourceBytes:0,itemBytes:0,layoutBytes:0,
    items:Object.fromEntries(ITEM_KINDS.map(kind=>[kind,0])),bytesByItemKind:Object.fromEntries(ITEM_KINDS.map(kind=>[kind,0])),
    modulesWithTranslatedItems:0,modulesWithoutCarriedItems:0}));
  for(const report of reports){
    assert.equal(report.schemaVersion,1);assert.equal(report.commit,commit);assert.deepEqual(report.decorators??null,decorators);
    assert.ok(Array.isArray(report.modules)&&Array.isArray(report.failures));
    if(report.sourceCatalog!==undefined){
      assert.ok(Array.isArray(report.sourceCatalog));const current=report.sourceCatalog.map(requireReportSourcePath).slice().sort();
      assert.equal(new Set(current).size,current.length,'distinct complete JavaScript source catalog');
      if(catalog)assert.deepEqual(current,catalog);catalog=current;
    }
    const outcomes=[...report.modules,...report.failures],selected=report.inventory??outcomes;
    assert.ok(Array.isArray(selected));const remaining=new Set();
    for(const source of selected){
      requireReportSourcePath(source.module);assert.ok(!inventory.has(source.module),'a module belongs to one retained shard');
      hash(source.sourceSha256,'actual selected JavaScript source hash');
      if(source.rust===null){assert.equal(source.rustSha256,null);withoutCounterpart+=1;}
      else{
        assert.ok(typeof source.rust==='string'&&source.rust.startsWith('rust/src/')&&source.rust.endsWith('.rs')
          &&!source.rust.includes('\\')&&!source.rust.includes('\0')&&source.rust.split('/').every(part=>part&&part!=='.'&&part!=='..'),'actual Rust counterpart subset path');
        hash(source.rustSha256,'actual compared Rust source hash');
        if(counterparts.has(source.rust))assert.equal(counterparts.get(source.rust),source.rustSha256,'shared counterpart has one actual source');
        counterparts.set(source.rust,source.rustSha256);
      }
      inventory.set(source.module,source);remaining.add(source.module);
    }
    for(const row of outcomes){
      assert.ok(remaining.delete(row.module),'one retained outcome for every selected module');
      const selected=inventory.get(row.module);
      for(const field of ['sourceSha256','rust','rustSha256'])if(row[field]!==undefined)assert.equal(row[field],selected[field]);
    }
    assert.equal(remaining.size,0,'selected modules cannot disappear from the denominator');
    failedModules+=report.failures.length;
    for(const failure of report.failures)assert.equal(typeof failure.error,'string');
    for(const row of report.modules){
      measuredModules+=1;
      const sourceOwner=inventory.get(row.module);if(sourceOwner.rust!==null)comparedCounterparts.add(sourceOwner.rust);
      for(const total of modes){
        const measured=readRetainedItemMeasurement(row,total.mode);total.moduleMeasurements+=1;
        for(const field of ['sourceBytes','itemBytes','layoutBytes'])total[field]+=measured[field];
        for(const kind of ITEM_KINDS){total.items[kind]+=measured.items[kind];total.bytesByItemKind[kind]+=measured.bytes[kind]??0;}
        if(measured.items.translated>0)total.modulesWithTranslatedItems+=1;
        if(measured.items.carried===0)total.modulesWithoutCarriedItems+=1;
      }
      assert.equal(row.coverage.sourceBytes,row.decorated.coverage.sourceBytes,'both modes measured one complete JavaScript source');
    }
  }
  if(catalog)for(const module of inventory.keys())assert.ok(catalog.includes(module),'selected module belongs to the complete report source catalog');
  const unknown=(executionRuntime,sourceLanguage,phase,reason)=>({executionRuntime,sourceLanguage,phase,moduleCount:null,reason});
  return{schemaVersion:1,kind:'retained self translation phase summary',commit,
    executionRuntime:'JavaScript',sourceLanguage:'JavaScript',targetLanguage:'Rust',
    catalogModules:catalog?.length??null,selectedModules:inventory.size,measuredModules,failedModules,
    unreportedCatalogModules:catalog?catalog.length-inventory.size:null,
    rustCounterpartSubset:{selectedSourceFiles:counterparts.size,comparedSourceFiles:comparedCounterparts.size,selectedJavaScriptModulesWithoutCounterpart:withoutCounterpart,
      completeRustSourceInventorySize:null,scope:'Only corresponding rust/src files recorded by JavaScript report rows; no complete Rust-source denominator or native-runtime observation.'},
    modes,
    unrecordedPhases:[
      unknown('JavaScript','JavaScript','complete CST syntax recovery observations','Schema1 retains no complete CST ERROR/MISSING/root recovery observation; source byte accounting does not establish syntax acceptance.'),
      unknown('JavaScript','JavaScript','complete Links source reconstruction observations','The producer checks source reconstruction during selfTranslate, but schema1 retains no separate per-phase observation object.'),
      unknown('JavaScript','JavaScript','whole-module semantic parse check and Rust emission','Translated item counts do not retain one complete module parse/check/emit observation, even when no items are carried.'),
      unknown('JavaScript','Rust','complete Rust source CST and Links reconstruction','Counterpart function comparison retains no lossless Rust Links reconstruction, accepted/recovered CST flags or complete Rust inventory.'),
      unknown('JavaScript','Rust','whole-module Rust semantic parse check and JavaScript emission','The report does not translate its Rust counterparts into JavaScript.'),
      unknown('Rust','JavaScript','native runtime whole-module CST and semantic translation','The JavaScript worker report is not a Rust-runtime corpus measurement.'),
      unknown('Rust','Rust','native runtime whole-module CST and semantic translation','Native corpus phases are not recorded in these reports.'),
    ],
    boundary:'Aggregate analysis of original retained report objects only; no source reparse, reconstruction, translation, execution, native compilation, requirement observation or completion claim. Both modes remain distinct. Carried originals, comments and provenance do not establish executable translation.'};
}

export function retainedSelfTranslationPhaseMarkdown(summary){
  const generic=summary.modes.find(row=>row.mode==='generic'),decorated=summary.modes.find(row=>row.mode==='decorated');
  return'\n## Recorded coverage and unmeasured phases\n\n'
    +`${summary.measuredModules} JavaScript modules have complete recorded item-range byte accounting; ${summary.failedModules} module measurements failed. `
    +`${generic.modulesWithoutCarriedItems} generic and ${decorated.modulesWithoutCarriedItems} decorated modules have zero carried items; whole-module semantic success remains unmeasured.\n\n`
    +`${summary.rustCounterpartSubset.comparedSourceFiles} distinct Rust counterpart files were compared by the JavaScript report. `
    +'A complete Rust inventory, Rust lossless reconstruction and native-runtime corpus phases are not measured here. '
    +'CST recovery flags and complete-module semantic parse/check/emit observations are not retained by report schema1.\n';
}
