import assert from 'node:assert/strict';
import test from 'node:test';
import {relocateCheckedModuleImports} from './overlay.mjs';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {readProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {readReviewedOverlayProviderSources,restoreReviewedOverlayProviderSources,reviewedOverlayProviderPolicy} from './reviewed-providers.mjs';
import {createCheckedModuleOverlay} from './overlay.mjs';
import {restoreReviewedOverlayProviderPolicy,restoreReviewedOverlayProviderHostSource} from './provider-succession.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';

test('lexical module relocation preserves active imports and resource bases without changing inert expressions',()=>{
 const source='import value from "./provider.js";\nexport {value} from "./export.js";\nexport const data=new URL("./data.lino",import.meta.url);\nconst inert="import.meta.url";\nfrom\n"./inert.js";\nconst property=object.import.meta.url;\nconst lazy=import("./lazy.js");';
 const origin='file:///original/module.js',linked=relocateCheckedModuleImports(source,origin,url=>url==='file:///original/provider.js'?'file:///overlay/provider.js':url);
 assert.ok(linked.includes('from "file:///overlay/provider.js"'));assert.ok(linked.includes('new URL("./data.lino","file:///original/module.js")'));assert.ok(linked.includes('inert="import.meta.url"'));
 assert.ok(linked.includes('from "file:///original/export.js"'));assert.ok(linked.includes('from\n"./inert.js"'));assert.ok(linked.includes('object.import.meta.url'));assert.ok(linked.includes('import("file:///original/lazy.js")'));
});

test('reviewed provider revisions preserve historical receipts and refuse every unreviewed byte change',async()=>{
 const modules=reviewedOverlayProviderPolicy.revisions[0].providers.map(row=>row.module),sourceRoot=new URL('../../js/src/',import.meta.url);
 for(const historical of reviewedOverlayProviderPolicy.historicalOverlays){
  const file=new URL('../'+historical.experiment+'/data/overlays.lino',import.meta.url),before=readFileSync(file);
  assert.equal(createHash('sha256').update(before).digest('hex'),historical.sourceSha256);
  const selected=readReviewedOverlayProviderSources({experiment:historical.experiment,modules});
  assert.ok(reviewedOverlayProviderPolicy.revisions.includes(selected.revision));
  for(const changed of modules)assert.throws(()=>readReviewedOverlayProviderSources({experiment:historical.experiment,modules,
   readSource:module=>readFileSync(new URL(module,sourceRoot),'utf8')+(module===changed?'\n':'')}),/no unique reviewed revision/u,changed);
  if(selected.revision!==reviewedOverlayProviderPolicy.revisions[0])await assert.rejects(createCheckedModuleOverlay({records:readProjectData(file)}),/exact overlay provider changed/u);
  assert.deepEqual(readFileSync(file),before);
 }
});

test('the collection renderer revision restores all five complete preceding providers by its exact inverse',()=>{
 const current=reviewedOverlayProviderPolicy.revisions.find(row=>row.sourceCommit==='b51d48140220bf2ac071334d79d28aa77273c00e'),previous=reviewedOverlayProviderPolicy.revisions.find(row=>row.sourceCommit==='b87d3499746d56e8f303464343c8edfecef84a8a');
 assert.equal(current.sourceCommit,'b51d48140220bf2ac071334d79d28aa77273c00e');
 assert.equal(previous.sourceCommit,'b87d3499746d56e8f303464343c8edfecef84a8a');
 assert.deepEqual(current.providers.map(row=>row.module),previous.providers.map(row=>row.module));
 const root=new URL('../../',import.meta.url);
 const policySource=execFileSync('git',['show','fdf6b2868d9804bc62eceb9209b8ebce9274b8ed:experiments/rust-text-certificate-adapter/reviewed-providers.mjs'],{cwd:root,encoding:'utf8'});
 const marker="    { sourceCommit: 'b51d48140220bf2ac071334d79d28aa77273c00e',\n";
 assert.equal(policySource.split(marker).length,2);
 const start=policySource.indexOf(marker),end=policySource.indexOf('\n  ],\n};',start);
 assert.ok(end>start);
 assert.equal(createHash('sha256').update(policySource.slice(0,start)+policySource.slice(end+1)).digest('hex'),
  '6cc35c56132e1da0715302046535a25258bfa1b7b998d918c367e357e8913018','all preceding provider registrations and the full reader remain byte exact');
 const collectionTests=readFileSync(new URL('../../js/tests/translation-collection-construction.test.js',import.meta.url),'utf8');
 const branchStart='const originalBranches=',branchEnd='\n};\nconst originalSources=';
 assert.equal(collectionTests.split(branchStart).length,2);assert.equal(collectionTests.split(branchEnd).length,2);
 const branches=JSON.parse(collectionTests.slice(collectionTests.indexOf(branchStart)+branchStart.length,collectionTests.indexOf(branchEnd)+2));
 for(const[target,pin]of Object.entries({
  JavaScript:'7b0bd63e09ce0aa72bb077aa0cdce0b9daaab036e2c0d1e739c59b830b59df5b',Rust:'9d7d395b1fac905a4455c472eeb81939b33e4d1781a457744913425498dd22e1',
  Lean:'59d6c5001f117178abc22962f9063fdb4ec22fefe8512452a013cbc3064e4294',Rocq:'c03707727d898497d151101c8ff4e748f6bef2e6ee5b3afbc1ffa8eae58dc267',
 }))assert.equal(createHash('sha256').update(branches[target]).digest('hex'),pin,'the original '+target+' constructor template stays independently frozen');
 const collectionPolicy=readProjectData(new URL('../../js/src/translation/data/collection-construction-rules.lino',import.meta.url));
 assert.deepEqual(readProjectData(new URL('../../rust/src/data/collection-construction-rules.lino',import.meta.url)),collectionPolicy);
 assert.equal(collectionPolicy.sourceSha256,'e9e21ed1c12c208190ec62595d2f3f52fd6a1329a82df133832f7add8f683502');
 const collectionSource=readFileSync(new URL('../../'+collectionPolicy.sourcePath,import.meta.url),'utf8');
 assert.equal(collectionPolicy.source,collectionSource);
 for(const input of collectionPolicy.sourceClosure.inputs){
  const bytes=readFileSync(new URL('../../'+input.path,import.meta.url));
  assert.equal(bytes.toString('utf8'),input.source);assert.equal(bytes.length,input.bytes);
  assert.equal(createHash('sha256').update(bytes).digest('hex'),input.sourceSha256);
 }
 assert.equal(collectionPolicy.sourceClosure.inputs.length,2);
 assert.equal(collectionPolicy.sourceClosure.removedImports.length,1);
 assert.equal(collectionSource.split(collectionPolicy.sourceClosure.removedImports[0]).length,2);
 assert.equal(collectionPolicy.sourceClosure.source,collectionPolicy.sourceClosure.inputs[0].source+'\n'+collectionSource.replace(collectionPolicy.sourceClosure.removedImports[0],''));
 const sourceSurface=parseJavaScript(collectionPolicy.sourceClosure.source,{referenceSemantics:false});
 const sourceInventory=sourceSurface.items.map((item,ordinal)=>({ordinal,kind:item.k,name:item.name,generated:item.generated===true}));
 assert.deepEqual(sourceInventory.filter(item=>!item.generated).map(item=>[item.kind,item.name]),[
  ['fn','refuseRequiredNumberFact'],['fn','refuseRequiredTextFact'],['fn','refuseRequiredBooleanFact'],
  ['fn','readRequiredNumberFact'],['fn','readRequiredTextFact'],['fn','readRequiredBooleanFact'],
  ['fn','readCollectionConstructionField'],['fn','readCollectionConstructionElement'],['fn','readCollectionConstructionKind'],
  ['fn','planCollectionConstructionInference'],['fn','planCollectionConstructionChecked'],['fn','readCollectionConstructionKeyAdmission'],
  ['fn','readCollectionConstructionDiagnostic'],['fn','readCollectionConstructionBinding'],
  ['fn','renderCollectionConstructionType'],['fn','renderCollectionConstruction'],
 ],'all ten construction definitions, three required readers and three private refusal helpers retain their actual source owners');
 assert.deepEqual(sourceInventory.filter(item=>item.generated).map(item=>[item.kind,item.name]),[
  ['data','ml_readCollectionConstructionField_loop1_result'],['fn','ml_readCollectionConstructionField_loop1'],
 ],'both actual generated data/function owners remain represented');
 const artifact=readProjectData(new URL('../../parity/self-translation/collection-construction-rules-native.lino',import.meta.url));
 assert.deepEqual(artifact.sourceClosure,collectionPolicy.sourceClosure);
 assert.deepEqual(artifact.inventory,sourceInventory);
 assert.deepEqual(artifact.checkedNamedSourceInventory.map(({ordinal,kind,name,generated})=>({ordinal,kind,name,generated})),sourceInventory);
 for(const replay of artifact.replays)assert.deepEqual(replay.checkedNamedSourceInventory,artifact.checkedNamedSourceInventory);
 const replacements={
  'translation/emit-javascript.js': {original:branches.JavaScript,current:"      case 'set':\n      case 'map':\n        return renderCollectionConstruction('JavaScript', e.k, '', this.expr(e.k === 'set' ? e.items : e.entries), '', '', '');\n"},
  'translation/emit-rust.js': {original:branches.Rust,current:"      case 'set':\n      case 'map': {\n        this.usesArray = true;\n        this.usesStandardArray = true;\n        const key = e.k === 'set' ? e.type.element : e.type.key;\n        const values = this.expr(e.k === 'set' ? e.items : e.entries);\n        return renderCollectionConstruction('Rust', e.k, key.kind, values, '', '', '');\n      }\n"},
 };
 const changed=[],sourceRoot=new URL('../../js/src/',import.meta.url);
 for(const[index,row]of current.providers.entries()){
  const source=execFileSync('git',['show',current.sourceCommit+':js/src/'+row.module],{cwd:root,encoding:'utf8'});
  assert.equal(createHash('sha256').update(source).digest('hex'),row.sourceSha256);
  let restored=source;
  if(Object.hasOwn(replacements,row.module)){
   changed.push(row.module);
   const imported="import { renderCollectionConstruction } from './collection-construction-rules.js';\n",replacement=replacements[row.module];
   assert.equal(restored.split(imported).length,2);assert.equal(restored.split(replacement.current).length,2);
   restored=restored.replace(imported,'').replace(replacement.current,replacement.original);
  }
  assert.equal(createHash('sha256').update(restored).digest('hex'),previous.providers[index].sourceSha256,
   row.module+': the exact contextual inverse restores the complete preceding source');
 }
 assert.deepEqual(changed,['translation/emit-javascript.js','translation/emit-rust.js']);
 // The current providers additionally retain exact inverses to that complete
 // collection-renderer revision; no historical input or comparison is removed.
 const reviewed=reviewedOverlayProviderPolicy.revisions.find(row=>row.sourceCommit==='fdf6b2868d9804bc62eceb9209b8ebce9274b8ed');
 assert.equal(reviewed.sourceCommit,'fdf6b2868d9804bc62eceb9209b8ebce9274b8ed');
 assert.equal(reviewed.inverse.length,3);
 assert.deepEqual(reviewed.providers.map(row=>row.module),current.providers.map(row=>row.module));
 assert.deepEqual([...new Set(reviewed.inverse.map(row=>row.module))],['typed-program-checked-projection.js','translation/emit-rust.js']);
 for(const[index,row]of reviewed.providers.entries()){
  const source=execFileSync('git',['show',reviewed.sourceCommit+':js/src/'+row.module],{cwd:root,encoding:'utf8'});
  assert.equal(createHash('sha256').update(source).digest('hex'),row.sourceSha256);
  let restored=source;
  for(const inverse of reviewed.inverse.filter(inverse=>inverse.module===row.module)){
   assert.equal(restored.split(inverse.from).length,2,'each exact inverse has one complete contextual match');
   restored=restored.replace(inverse.from,inverse.to);
  }
  const bytes=execFileSync('git',['show',current.sourceCommit+':js/src/'+row.module],{cwd:root});
  assert.deepEqual(Buffer.from(restored),bytes,'every byte of the original reviewed provider is restored');
  assert.equal(createHash('sha256').update(restored).digest('hex'),current.providers[index].sourceSha256);
 }
 const actualPolicy=execFileSync('git',['show','191f214903cc123cfb1566d82a90e18819be6b4d:experiments/rust-text-certificate-adapter/reviewed-providers.mjs'],{cwd:root,encoding:'utf8'});
 const appendedMarker="    { sourceCommit: 'fdf6b2868d9804bc62eceb9209b8ebce9274b8ed',\n";
 assert.equal(actualPolicy.split(appendedMarker).length,2);
 const appendedStart=actualPolicy.indexOf(appendedMarker),appendedEnd=actualPolicy.indexOf('\n  ],\n};',appendedStart);
 assert.ok(appendedEnd>appendedStart);
 const restoredPolicy=actualPolicy.slice(0,appendedStart)+actualPolicy.slice(appendedEnd+1);
 assert.equal(createHash('sha256').update(restoredPolicy).digest('hex'),'5874eb2978ba2cf2ba2bded4073e567e2a03c89705ded1229f4fb9b55684bcea');
 assert.equal(restoredPolicy,policySource,'the complete historical registrations and admission reader remain byte exact');
 for(const historical of reviewedOverlayProviderPolicy.historicalOverlays){
  const bytes=readFileSync(new URL('../'+historical.experiment+'/data/overlays.lino',import.meta.url));
  assert.equal(createHash('sha256').update(bytes).digest('hex'),historical.sourceSha256);
 }
});

test('callback renderer revision restores all preceding providers and the complete historical policy',()=>{
 const root=new URL('../../',import.meta.url),sourceRoot=new URL('../../js/src/',import.meta.url);
 const previous=reviewedOverlayProviderPolicy.revisions.find(row=>row.sourceCommit==='fdf6b2868d9804bc62eceb9209b8ebce9274b8ed');
 const current=reviewedOverlayProviderPolicy.revisions.find(row=>row.sourceCommit==='b4b92d47ee0a186dbe3920c1844eb907afe56aeb');
 assert.equal(current.sourceCommit,'b4b92d47ee0a186dbe3920c1844eb907afe56aeb');
 assert.equal(current.inverse.length,2);
 assert.deepEqual(current.providers.map(row=>row.module),previous.providers.map(row=>row.module));
 assert.deepEqual([...new Set(current.inverse.map(row=>row.module))],['translation/emit-rust.js']);
 const changed=[];
 for(const[index,row]of current.providers.entries()){
  const bytes=execFileSync('git',['show',current.sourceCommit+':js/src/'+row.module],{cwd:root}),source=bytes.toString('utf8');
  assert.equal(createHash('sha256').update(bytes).digest('hex'),row.sourceSha256);
  const preceding=execFileSync('git',['show','191f214903cc123cfb1566d82a90e18819be6b4d:js/src/'+row.module],{cwd:root});
  if(!bytes.equals(preceding))changed.push(row.module);
  let inverse=source;
  for(const action of current.inverse.filter(action=>action.module===row.module)){
   assert.equal(inverse.split(action.from).length,2,'each actual whole contextual edit has exactly one inverse');
   inverse=inverse.replace(action.from,()=>action.to);
  }
  assert.deepEqual(Buffer.from(inverse),preceding,'every original provider byte remains recoverable');
  assert.equal(createHash('sha256').update(inverse).digest('hex'),previous.providers[index].sourceSha256);
 }
 assert.deepEqual(changed,['translation/emit-rust.js']);
 const policy=execFileSync('git',['show','42023d402abd577ae935f2686eca3f39f5e0b9dd:experiments/rust-text-certificate-adapter/reviewed-providers.mjs'],{cwd:root,encoding:'utf8'});
 const previousPolicy=execFileSync('git',['show','191f214903cc123cfb1566d82a90e18819be6b4d:experiments/rust-text-certificate-adapter/reviewed-providers.mjs'],{cwd:root,encoding:'utf8'});
 const marker='    {\n      "sourceCommit": "b4b92d47ee0a186dbe3920c1844eb907afe56aeb",\n';
 assert.equal(policy.split(marker).length,2);
 const start=policy.indexOf(marker),end=policy.indexOf('\n  ],\n};',start);assert.ok(end>start);
 assert.equal(policy.slice(0,start)+policy.slice(end+1),previousPolicy,'the entire original policy and reader remain exact');
 for(const historical of reviewedOverlayProviderPolicy.historicalOverlays){
  const bytes=readFileSync(new URL('../'+historical.experiment+'/data/overlays.lino',import.meta.url));
  assert.equal(createHash('sha256').update(bytes).digest('hex'),historical.sourceSha256);
 }
});


test('reference callback provider revision restores every preceding source and historical policy',()=>{
 const root=new URL('../../',import.meta.url),sourceRoot=new URL('../../js/src/',import.meta.url);
 const historicalPolicy=restoreReviewedOverlayProviderPolicy(reviewedOverlayProviderPolicy);
 assert.equal(historicalPolicy.revisions.pop().sourceCommit,'478fb5873ddfd6ab7e4c4b6849654e2c2cef3fa8');
 assert.equal(historicalPolicy.revisions.pop().sourceCommit,'9f867bfac32f59a3341dabf54e8c0c02fbd83314');
 const previous=historicalPolicy.revisions.find(row=>row.sourceCommit==='b4b92d47ee0a186dbe3920c1844eb907afe56aeb');
 const current=historicalPolicy.revisions.at(-1);
 assert.equal(current.sourceCommit,'113a7747f799cbce5167167a783eb88427bd2825');
 assert.equal(current.inverse.length,2);
 assert.deepEqual(current.providers.map(row=>row.module),previous.providers.map(row=>row.module));
 assert.deepEqual([...new Set(current.inverse.map(row=>row.module))],['translation/emit-rust.js']);
 const restoredSources=restoreReviewedOverlayProviderSources({experiment:'rust-text-certificate-adapter',modules:current.providers.map(row=>row.module),sourceCommit:current.sourceCommit}).sources;
 const changed=[];
 for(const[index,row]of current.providers.entries()){
  const bytes=Buffer.from(restoredSources.get(row.module));
  assert.deepEqual(bytes,execFileSync('git',['show',current.sourceCommit+':js/src/'+row.module],{cwd:root}));
  assert.equal(createHash('sha256').update(bytes).digest('hex'),row.sourceSha256);
  const preceding=execFileSync('git',['show',previous.sourceCommit+':js/src/'+row.module],{cwd:root});
  if(!bytes.equals(preceding))changed.push(row.module);
  let restored=bytes.toString('utf8');
  for(const action of current.inverse.filter(action=>action.module===row.module)){
   assert.equal(restored.split(action.from).length,2,'every exact reviewed edit has one whole contextual inverse');
   restored=restored.replace(action.from,()=>action.to);
  }
  assert.deepEqual(Buffer.from(restored),preceding,'all preceding provider bytes are recovered');
  assert.equal(createHash('sha256').update(restored).digest('hex'),previous.providers[index].sourceSha256);
 }
 assert.deepEqual(changed,['translation/emit-rust.js']);
 const dependency='js/src/translation/array-callback-rules.js';
 const before=execFileSync('git',['show',previous.sourceCommit+':'+dependency],{cwd:root});
 const actual=readFileSync(new URL('../../'+dependency,import.meta.url));
 assert.equal(before.length,7249);
 assert.equal(createHash('sha256').update(before).digest('hex'),'a5fa80c7683dc2a127b2500b464e049ccf23d29979ff596aa14999c369e5c554');
 assert.deepEqual(actual,execFileSync('git',['show',current.sourceCommit+':'+dependency],{cwd:root}));
 assert.deepEqual(actual.subarray(0,before.length),before,'every previous callback function and byte remains exact');
 assert.deepEqual([...actual.subarray(before.length).toString('utf8').matchAll(/^export function (\w+)/gmu)].map(match=>match[1]),['renderArrayCallbackReferenceExpression']);
 const fixtureFile='parity/fixtures/array-callbacks.lino';
 const originalBytes=execFileSync('git',['show',previous.sourceCommit+':'+fixtureFile],{cwd:root,maxBuffer:8*1024*1024});
 const original=readStructuredDataLino(originalBytes.toString('utf8'));
 const fixture=readProjectData(new URL('../../'+fixtureFile,import.meta.url)),inverse=structuredClone(fixture);
 assert.equal(fixture.cases.length,18);
 const rows=inverse.cases.filter(row=>row.id==='array-callback-flatMap-reference-value-parameters');
 assert.equal(rows.length,1);assert.equal(rows[0].profiles.length,1);assert.equal(rows[0].profiles[0].referenceSemantics,true);
 const emission=rows[0].profiles[0].targets.Rust.emitted;
 const old='ml_values1.iter().cloned().enumerate().flat_map(|(ml_index1, ml_value1)| ml_callback1(ml_value1, (ml_index1 as f64))).collect::<Vec<_>>()';
 const replacement='std::rc::Rc::new(std::cell::RefCell::new(ml_values1.iter().cloned().enumerate().flat_map(|(ml_index1, ml_value1)| { let ml_callback_values1 = ml_callback1(ml_value1, (ml_index1 as f64)).borrow().clone(); ml_callback_values1 }).collect::<Vec<_>>()))';
 assert.equal(emission.text.split(replacement).length,2);
 emission.text=emission.text.replace(replacement,()=>old);
 let definitions=0;
 emission.definitions=emission.definitions.map(text=>{
  if(!text.includes(replacement))return text;
  assert.equal(text.split(replacement).length,2);definitions+=1;return text.replace(replacement,()=>old);
 });
 assert.equal(definitions,1);
 assert.deepEqual(inverse,original,'all original cases, source inputs, expected values, traces, archives and target fields are recovered');
 assert.deepEqual(Buffer.from(serializeProjectData(inverse,new URL('../../'+fixtureFile,import.meta.url))),originalBytes,
  'the complete original canonical fixture bytes remain independently recoverable');
 let policy=restoreCurrentCiRepairSource(restoreReviewedOverlayProviderHostSource(readFileSync(new URL('./reviewed-providers.mjs',import.meta.url),'utf8'),'experiments/rust-text-certificate-adapter/reviewed-providers.mjs'),'experiments/rust-text-certificate-adapter/reviewed-providers.mjs');
 const restoredPolicy=execFileSync('git',['show','9f867bfac32f59a3341dabf54e8c0c02fbd83314:experiments/rust-text-certificate-adapter/reviewed-providers.mjs'],{cwd:root,encoding:'utf8'});
 const appended='    {\n      "sourceCommit": "9f867bfac32f59a3341dabf54e8c0c02fbd83314",\n';
 const added=policy.indexOf(appended),close=policy.indexOf('\n  ],\n};',added);
 assert.ok(added>0&&close>added);
 policy=policy.slice(0,added)+policy.slice(close+1);
 const helper=policy.indexOf('\n/** Restore only recorded complete source copies; unsupported history refuses. */');
 assert.ok(helper>0);policy=policy.slice(0,helper);
 assert.equal(policy,restoredPolicy,'only the new complete provider record and checked history reader were appended');
 const previousPolicy=execFileSync('git',['show','42023d402abd577ae935f2686eca3f39f5e0b9dd:experiments/rust-text-certificate-adapter/reviewed-providers.mjs'],{cwd:root,encoding:'utf8'});
 const marker='    {\n      "sourceCommit": "113a7747f799cbce5167167a783eb88427bd2825",\n';
 assert.equal(policy.split(marker).length,2);
 const start=policy.indexOf(marker),end=policy.indexOf('\n  ],\n};',start);assert.ok(end>start);
 assert.equal(policy.slice(0,start)+policy.slice(end+1),previousPolicy,'all earlier revisions and the entire admission reader stay byte exact');
 const withoutCurrent=structuredClone(historicalPolicy);withoutCurrent.revisions.pop();
 const previousData=readStructuredDataLino(execFileSync('git',['show','42023d402abd577ae935f2686eca3f39f5e0b9dd:experiments/rust-text-certificate-adapter/data/provider-revisions.lino'],{cwd:root,encoding:'utf8'}));
 assert.deepEqual(withoutCurrent,previousData,'all prior policy records retain their original values and order');
 for(const historical of historicalPolicy.historicalOverlays){
  const bytes=readFileSync(new URL('../'+historical.experiment+'/data/overlays.lino',import.meta.url));
  assert.equal(createHash('sha256').update(bytes).digest('hex'),historical.sourceSha256);
 }
});

import {restoreCurrentCiRepairSource} from '../../js/scripts/current-ci-repair-source-history.mjs';
