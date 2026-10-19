import assert from 'node:assert/strict';
import test from 'node:test';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {COMPLETE_DECLARATION_RECEIPTS,restoreCompleteDeclarationReceipt,restoreCompleteDeclarationDescriptorSuccessor} from '../scripts/complete-declaration-receipts.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino,structuredDataToLino} from '../src/structured-data-links.js';
import {tokenize} from '../src/translation/lexer.js';
import {parseNative} from '../src/native-grammar-parser.js';

const root=new URL('../../',import.meta.url),completeRegister=readProjectData(COMPLETE_DECLARATION_RECEIPTS);
const register={...completeRegister,receipts:completeRegister.receipts.slice(0,2)};

test('current declaration receipts restore both complete immutable prior artifacts including every source owner',()=>{
 assert.equal(register.receipts.length,2);
 for(const receipt of register.receipts){
  const current=readProjectData(new URL(receipt.current.file,root));
  const previous=execFileSync('git',['show',receipt.previous.revision+':'+receipt.previous.file],{cwd:root,maxBuffer:8*1024*1024});
  const restored=restoreCompleteDeclarationReceipt(current,receipt.identity);
  assert.deepEqual(Buffer.from(structuredDataToLino(restored)),previous);
  assert.deepEqual(restored,readStructuredDataLino(previous.toString()));
  assert.deepEqual(restoreCompleteDeclarationReceipt(restored,receipt.identity),restored);
  const semantic=receipt.identity==='raw consuming'?['completeRust','owners','declarationNames','totals','assumptions','encodings','preludeHashes','replays']:['sourceClosure','emittedTextBytes','emittedTextSha256','library'];
  for(const field of semantic)assert.deepEqual(current[field],restored[field]);
  assert.deepEqual(readFileSync(new URL(receipt.current.file,root)),Buffer.from(structuredDataToLino(current)));
 }
});

test('declaration receipt checks refuse altered semantics endpoints archives descriptors identities and unknown fields',()=>{
 let refused=0;
 for(const receipt of register.receipts){
  const current=readProjectData(new URL(receipt.current.file,root));
  for(const mutate of[
   value=>{value.extra='unreviewed';},
   value=>{value.archive.textSha256='0'.repeat(64);},
   value=>{value.archive.textBytes+=1;},
   value=>{delete value.archive;},
   value=>{if(value.completeRust)value.completeRust+=' ';else value.sourceClosure.source+=' ';},
   value=>{const path=receipt.changes.find(row=>row.path.at(-2)==='sourceSpan').path;let parent=value;for(const key of path.slice(0,-1))parent=parent[key];parent.end+=1;},
  ]){
   const changed=structuredClone(current);mutate(changed);
   assert.throws(()=>restoreCompleteDeclarationReceipt(changed,receipt.identity),assert.AssertionError);refused+=1;
  }
  assert.throws(()=>restoreCompleteDeclarationReceipt(current,receipt.identity+' unknown'),assert.AssertionError);refused+=1;
 }
 assert.equal(refused,14);
});

test('scalar loop fixture refresh retains complete historical bytes and only current full declaration endpoints',()=>{
 assert.equal(completeRegister.receipts.length,4);
 assert.deepEqual(register.receipts.map(row=>row.identity),['collection construction','raw consuming']);
 const added=completeRegister.receipts.slice(2);
 assert.deepEqual(added.map(row=>row.identity),['scalar loop values','scalar loop declarations']);
 let endpoints=0,refused=0;
 for(const [index,receipt]of added.entries()){
  assert.equal(receipt.previous.revision,'fdf6b2868d9804bc62eceb9209b8ebce9274b8ed');
  const current=readProjectData(new URL(receipt.current.file,root));
  const previous=execFileSync('git',['show',receipt.previous.revision+':'+receipt.previous.file],{cwd:root,maxBuffer:2*1024*1024});
  const restored=restoreCompleteDeclarationReceipt(current,receipt.identity);
  assert.deepEqual(Buffer.from(structuredDataToLino(restored)),previous);
  assert.deepEqual(restored,readStructuredDataLino(previous.toString()));
  assert.deepEqual(restoreCompleteDeclarationReceipt(restored,receipt.identity),restored);
  assert.deepEqual(readFileSync(new URL(receipt.current.file,root)),Buffer.from(structuredDataToLino(current)));
  const count=index===0?4:6;
  assert.equal(current.cases.length,count);assert.equal(receipt.changes.length,index===0?4:9);
  const changes=[];
  for(const [position,row]of current.cases.entries()){
   if(row.language==='JavaScript'){
    const tokens=tokenize(row.source,'JavaScript').tokens,first=tokens[0],last=tokens.at(-2);
    assert.equal(first.value,'function');assert.equal(last.value,'}');
    assert.deepEqual(row.surface.items[0].span,{start:first.start,end:last.end});
    changes.push({path:['cases',String(position),'surface','items','0','span','end'],before:first.end,after:last.end});endpoints+=1;
   }
   changes.push({path:['cases',String(position),'javaScriptReverseSurface','items','0','span','end'],before:8,after:row.javaScriptDefinitions.length});
   assert.equal(row.javaScriptReverseSurface.items.length,1);
   assert.deepEqual(row.javaScriptReverseSurface.items[0].span,{start:0,end:row.javaScriptDefinitions.length});
   assert.ok(row.javaScriptDefinitions.startsWith('function '));assert.ok(row.javaScriptDefinitions.endsWith('}'));
   endpoints+=1;
  }
  assert.deepEqual(receipt.changes,changes);
  for(const mutate of[
   value=>{value.extra='unreviewed';},
   value=>{value.cases[0].source+=' ';},
   value=>{value.cases[0].javaScriptReverseSurface.items[0].span.end+=1;},
   value=>{delete value.cases[0].javaScriptReverseSurface.items[0].span;},
   value=>{value.cases[0].javaScriptReverseSurface.items[0].body.extra=true;},
   value=>{value.cases[0].reverseRustDefinitions+=' ';},
  ]){
   const changed=structuredClone(current);mutate(changed);
   assert.throws(()=>restoreCompleteDeclarationReceipt(changed,receipt.identity),assert.AssertionError);refused+=1;
  }
  assert.throws(()=>restoreCompleteDeclarationReceipt(current,receipt.identity+' unknown'),assert.AssertionError);refused+=1;
 }
 assert.equal(endpoints,13);assert.equal(refused,14);
});


test('the descriptor successor retains all four receipt rows and restores the complete preceding raw artifact before its original inverse',()=>{
 const completeRegister=readStructuredDataLino(execFileSync('git',['show','1a69d1b33be71ca7421eb8a7a2e4dba9b599ef26:parity/self-translation/complete-declaration-receipts.lino'],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}));
 const baseline='191f214903cc123cfb1566d82a90e18819be6b4d';
 const previousAdapter=execFileSync('git',['show',baseline+':js/scripts/complete-declaration-receipts.mjs'],{cwd:root,maxBuffer:1024*1024});
 let inverseAdapter=execFileSync('git',['show','18a8e0a04bab43f6572be311ceb75c6596fdc147:js/scripts/complete-declaration-receipts.mjs'],{cwd:root,encoding:'utf8',maxBuffer:1024*1024});
 const additionsStart=inverseAdapter.indexOf('// A source revision authenticates source descriptors;');
 const originalStart=inverseAdapter.indexOf('/** Restore only a recorded source endpoint, archive receipt or source descriptor. */');
 assert.ok(additionsStart>0&&originalStart>additionsStart);
 inverseAdapter=inverseAdapter.slice(0,additionsStart)+inverseAdapter.slice(originalStart);
 const newOpening='export function restoreCompleteDeclarationReceipt(artifact,identity,{register=readProjectData(COMPLETE_DECLARATION_RECEIPTS)}={}){\n const successor=readCompleteDeclarationDescriptorSuccessor(artifact,identity,register);\n artifact=successor.artifact;';
 assert.equal(inverseAdapter.split(newOpening).length,2);
 inverseAdapter=inverseAdapter.replace(newOpening,'export function restoreCompleteDeclarationReceipt(artifact,identity){\n const register=readProjectData(COMPLETE_DECLARATION_RECEIPTS);');
 inverseAdapter=inverseAdapter.replace('const receipt=matches[0],bytes=successor.bytes,actual=digest(bytes);','const receipt=matches[0],bytes=Buffer.from(structuredDataToLino(artifact)),actual=digest(bytes);');
 assert.deepEqual(Buffer.from(inverseAdapter),previousAdapter,'all original source bytes and integrity assertions restore exactly after removing only the appended adapter and internal-buffer seam');

 const {successors,...originalRegister}=completeRegister;
 const originalRegisterBytes=execFileSync('git',['show',baseline+':parity/self-translation/complete-declaration-receipts.lino'],{cwd:root,maxBuffer:2*1024*1024});
 assert.deepEqual(Buffer.from(structuredDataToLino(originalRegister)),originalRegisterBytes);
 assert.equal(originalRegister.receipts.length,4);assert.equal(successors.length,1);
 const edge=successors[0];assert.equal(edge.identity,'raw consuming');
 assert.equal(edge.previous.revision,baseline);
 assert.equal(edge.current.sourceRevision,'b4b92d47ee0a186dbe3920c1844eb907afe56aeb');
 const current=readStructuredDataLino(execFileSync('git',['show','1a69d1b33be71ca7421eb8a7a2e4dba9b599ef26:'+edge.current.file],{cwd:root,encoding:'utf8',maxBuffer:8*1024*1024}));
 const priorBytes=execFileSync('git',['show',baseline+':'+edge.previous.file],{cwd:root,maxBuffer:8*1024*1024});
 const prior=readStructuredDataLino(priorBytes.toString());
 const restored=restoreCompleteDeclarationDescriptorSuccessor(current,edge.identity,completeRegister);
 assert.deepEqual(restored,prior);assert.deepEqual(Buffer.from(structuredDataToLino(restored)),priorBytes);
 assert.deepEqual(restoreCompleteDeclarationDescriptorSuccessor(prior,edge.identity,completeRegister),prior);
 assert.deepEqual(restoreCompleteDeclarationReceipt(current,edge.identity),restoreCompleteDeclarationReceipt(prior,edge.identity));
 assert.equal(edge.changes.length,4);
 assert.deepEqual(edge.sources.map(row=>row.file),['js/src/translation/check.js','js/src/translation/emit-rust.js']);
 for(const source of edge.sources){
  for(const [revision,expected]of [[baseline,source.previous],[edge.current.sourceRevision,source.current]]){
   const bytes=execFileSync('git',['show',revision+':'+source.file],{cwd:root,maxBuffer:1024*1024});
   assert.equal(bytes.length,expected.bytes);
   assert.equal(createHash('sha256').update(bytes).digest('hex'),expected.sha256);
  }
  const descriptor=current.decisionSources.find(row=>row.file===source.file);
  assert.deepEqual(descriptor,{file:source.file,...source.current});
 }
 const inverse=structuredClone(current);inverse.decisionSources=structuredClone(prior.decisionSources);
 assert.deepEqual(inverse,prior);assert.deepEqual(Buffer.from(structuredDataToLino(inverse)),priorBytes);
 assert.equal(current.definitionCount,106);assert.equal(current.owners.length,106);
 for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','preludeHashes','replays','archive'])assert.deepEqual(current[field],prior[field]);
 const borrowedFile='experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';
 const borrowed=readStructuredDataLino(execFileSync('git',['show','1a69d1b33be71ca7421eb8a7a2e4dba9b599ef26:'+borrowedFile],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}));
 const oldBorrowedBytes=execFileSync('git',['show',baseline+':'+borrowedFile],{cwd:root,maxBuffer:2*1024*1024});
 const oldBorrowed=readStructuredDataLino(oldBorrowedBytes.toString());
 assert.equal(borrowed.predecessor.bytes,Buffer.byteLength(structuredDataToLino(current)));
 assert.equal(borrowed.predecessor.sha256,createHash('sha256').update(structuredDataToLino(current)).digest('hex'));
 assert.deepEqual(borrowed.predecessor.source,oldBorrowed.predecessor.source);
 const borrowedInverse={...borrowed,predecessor:oldBorrowed.predecessor};
 assert.deepEqual(borrowedInverse,oldBorrowed);assert.deepEqual(Buffer.from(structuredDataToLino(borrowedInverse)),oldBorrowedBytes);
});

test('descriptor successor admission refuses omitted edges altered policies and any non-descriptor artifact mutation',()=>{
 const completeRegister=readStructuredDataLino(execFileSync('git',['show','1a69d1b33be71ca7421eb8a7a2e4dba9b599ef26:parity/self-translation/complete-declaration-receipts.lino'],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}));
 const current=readStructuredDataLino(execFileSync('git',['show','1a69d1b33be71ca7421eb8a7a2e4dba9b599ef26:'+completeRegister.successors[0].current.file],{cwd:root,encoding:'utf8',maxBuffer:8*1024*1024}));
 let refused=0;
 for(const mutate of [
  value=>{delete value.successors;},value=>{value.successors=[];},
  value=>{value.successors.push(structuredClone(value.successors[0]));},
  value=>{value.receipts[1].current.sha256='0'.repeat(64);},
  value=>{value.successors[0].identity='collection construction';},
  value=>{value.successors[0].current.sha256='0'.repeat(64);},
  value=>{value.successors[0].previous.sha256='0'.repeat(64);},
  value=>{value.successors[0].current.sourceRevision='0'.repeat(40);},
  value=>{value.successors[0].sources.reverse();},
  value=>{value.successors[0].sources[0].file='unowned.js';},
  value=>{value.successors[0].sources[0].current.bytes+=1;},
  value=>{value.successors[0].changes.pop();},
  value=>{value.successors[0].changes[0].path=['completeRust'];},
  value=>{value.successors[0].changes[0].after+=1;},
  value=>{value.successors[0].changes.reverse();},
  value=>{value.extra='unreviewed';},
 ]){
  const changed=structuredClone(completeRegister);mutate(changed);
  assert.throws(()=>restoreCompleteDeclarationReceipt(current,'raw consuming',{register:changed}),assert.AssertionError);refused+=1;
 }
 for(const mutate of [
  value=>{value.decisionSources[3].file='unowned.js';},
  value=>{value.decisionSources[3].bytes+=1;},
  value=>{value.decisionSources[4].sha256='0'.repeat(64);},
  value=>{value.decisionSources[2].sha256='0'.repeat(64);},
  value=>{value.completeRust+=' ';},value=>{value.owners.pop();},
  value=>{value.archive.textBytes+=1;},value=>{value.replays.reverse();},
  value=>{value.extra='unreviewed';},
 ]){
  const changed=structuredClone(current);mutate(changed);
  assert.throws(()=>restoreCompleteDeclarationReceipt(changed,'raw consuming'),assert.AssertionError);refused+=1;
 }
 assert.equal(refused,25);
});

test('reference renderer descriptor refresh preserves complete raw output and every earlier receipt edge',()=>{
 const completeRegister=readStructuredDataLino(execFileSync('git',['show','18a8e0a04bab43f6572be311ceb75c6596fdc147:parity/self-translation/complete-declaration-receipts.lino'],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}));
 const edge=completeRegister.descriptorSuccessors[0],baseline='1a69d1b33be71ca7421eb8a7a2e4dba9b599ef26';
 assert.equal(completeRegister.descriptorSuccessors.length,1);
 assert.equal(edge.identity,'raw consuming');assert.equal(edge.sources.length,1);assert.equal(edge.changes.length,2);
 assert.deepEqual(edge.changes.map(change=>change.path),[['decisionSources','4','bytes'],['decisionSources','4','sha256']]);
 const previousRegisterBytes=execFileSync('git',['show',baseline+':parity/self-translation/complete-declaration-receipts.lino'],{cwd:root,maxBuffer:2*1024*1024});
 const {descriptorSuccessors,...previousRegister}=completeRegister;
 assert.deepEqual(Buffer.from(structuredDataToLino(previousRegister)),previousRegisterBytes);
 const beforeBytes=execFileSync('git',['show',baseline+':'+edge.previous.file],{cwd:root,maxBuffer:8*1024*1024});
 const before=readStructuredDataLino(beforeBytes.toString()),current=readStructuredDataLino(execFileSync('git',['show','18a8e0a04bab43f6572be311ceb75c6596fdc147:'+edge.current.file],{cwd:root,encoding:'utf8',maxBuffer:8*1024*1024}));
 assert.equal(beforeBytes.length,edge.previous.bytes);assert.equal(createHash('sha256').update(beforeBytes).digest('hex'),edge.previous.sha256);
 assert.equal(Buffer.byteLength(structuredDataToLino(current)),edge.current.bytes);
 assert.equal(createHash('sha256').update(structuredDataToLino(current)).digest('hex'),edge.current.sha256);
 const inverse=structuredClone(current);
 assert.deepEqual(inverse.decisionSources[4],{file:edge.sources[0].file,...edge.sources[0].current});
 inverse.decisionSources[4]={file:edge.sources[0].file,...edge.sources[0].previous};
 assert.deepEqual(inverse,before);
 assert.deepEqual(Buffer.from(structuredDataToLino(inverse)),beforeBytes,'all complete original raw artifact bytes remain recoverable');
 assert.equal(current.definitionCount,106);assert.equal(current.owners.length,106);
 for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','preludeHashes','replays','archive'])
  assert.deepEqual(current[field],before[field]);
 assert.deepEqual(restoreCompleteDeclarationReceipt(current,'raw consuming'),restoreCompleteDeclarationReceipt(before,'raw consuming',{register:previousRegister}));
 assert.deepEqual(restoreCompleteDeclarationDescriptorSuccessor(current,'raw consuming',completeRegister),
  restoreCompleteDeclarationDescriptorSuccessor(before,'raw consuming',previousRegister));
 const source=edge.sources[0],sourceBytes=readFileSync(new URL(source.file,root));
 assert.equal(sourceBytes.length,source.current.bytes);assert.equal(createHash('sha256').update(sourceBytes).digest('hex'),source.current.sha256);
 assert.deepEqual(sourceBytes,execFileSync('git',['show',edge.current.sourceRevision+':'+source.file],{cwd:root,maxBuffer:1024*1024}));
 const reviewed=readProjectData(new URL('../../experiments/rust-text-certificate-adapter/data/provider-revisions.lino',import.meta.url))
  .revisions.find(revision=>revision.sourceCommit===edge.current.sourceRevision);
 assert.ok(reviewed);assert.equal(reviewed.inverse.length,2);
 let restored=sourceBytes.toString();
 for(const action of reviewed.inverse){
  assert.equal(action.module,'translation/emit-rust.js');assert.equal(restored.split(action.from).length,2);
  restored=restored.replace(action.from,()=>action.to);
 }
 assert.equal(Buffer.byteLength(restored),source.previous.bytes);
 assert.equal(createHash('sha256').update(restored).digest('hex'),source.previous.sha256);
 assert.deepEqual(Buffer.from(restored),execFileSync('git',['show',reviewed.sourceCommit+'^:'+source.file],{cwd:root,maxBuffer:1024*1024}));
 const borrowedFile='experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';
 const borrowed=readStructuredDataLino(execFileSync('git',['show','18a8e0a04bab43f6572be311ceb75c6596fdc147:'+borrowedFile],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}));
 const oldBorrowedBytes=execFileSync('git',['show',baseline+':'+borrowedFile],{cwd:root,maxBuffer:2*1024*1024});
 const oldBorrowed=readStructuredDataLino(oldBorrowedBytes.toString());
 assert.deepEqual(borrowed.predecessor,{...oldBorrowed.predecessor,bytes:edge.current.bytes,sha256:edge.current.sha256});
 assert.deepEqual(borrowed.predecessor.source,oldBorrowed.predecessor.source);
 const borrowedInverse={...borrowed,predecessor:oldBorrowed.predecessor};
 assert.deepEqual(borrowedInverse,oldBorrowed);assert.deepEqual(Buffer.from(structuredDataToLino(borrowedInverse)),oldBorrowedBytes);
 const adapterBefore=execFileSync('git',['show',baseline+':js/scripts/complete-declaration-receipts.mjs'],{cwd:root,encoding:'utf8'});
 let adapter=execFileSync('git',['show','18a8e0a04bab43f6572be311ceb75c6596fdc147:js/scripts/complete-declaration-receipts.mjs'],{cwd:root,encoding:'utf8',maxBuffer:1024*1024});
 const begin=adapter.indexOf('// This second finite edge authenticates source descriptors only.');
 const end=adapter.indexOf('/** The public standalone gate retains its cloned-data result and all checks. */',begin);
 assert.ok(begin>0&&end>begin);
 adapter=(adapter.slice(0,begin)+adapter.slice(end)).replace('function readHistoricalCompleteDeclarationDescriptorSuccessor(','function readCompleteDeclarationDescriptorSuccessor(');
 assert.equal(adapter,adapterBefore,'the entire previous reader and every assertion survive the new wrapper');
 // An absent new field retains the original unextended-register domain.
 const originalRevision='191f214903cc123cfb1566d82a90e18819be6b4d';
 const originalRegister=readStructuredDataLino(execFileSync('git',['show',originalRevision+':parity/self-translation/complete-declaration-receipts.lino'],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}));
 const originalCurrent=readStructuredDataLino(execFileSync('git',['show',originalRevision+':'+edge.current.file],{cwd:root,encoding:'utf8',maxBuffer:8*1024*1024}));
 assert.deepEqual(restoreCompleteDeclarationReceipt(originalCurrent,'raw consuming',{register:originalRegister}),
  restoreCompleteDeclarationReceipt(originalCurrent,'raw consuming'));
});

test('reference renderer descriptor refresh refuses forged edges altered old registers and semantic artifact edits',()=>{
 const completeRegister=readStructuredDataLino(execFileSync('git',['show','18a8e0a04bab43f6572be311ceb75c6596fdc147:parity/self-translation/complete-declaration-receipts.lino'],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}));
 const current=readStructuredDataLino(execFileSync('git',['show','18a8e0a04bab43f6572be311ceb75c6596fdc147:'+completeRegister.descriptorSuccessors[0].current.file],{cwd:root,encoding:'utf8',maxBuffer:8*1024*1024}));
 let refused=0;
 for(const mutate of [
  value=>{delete value.descriptorSuccessors;},value=>{value.descriptorSuccessors=[];},
  value=>{value.descriptorSuccessors.push(structuredClone(value.descriptorSuccessors[0]));},
  value=>{value.successors[0].identity='changed';},value=>{value.receipts[1].current.bytes+=1;},
  value=>{value.descriptorSuccessors[0].identity='collection construction';},
  value=>{value.descriptorSuccessors[0].previous.sha256='0'.repeat(64);},
  value=>{value.descriptorSuccessors[0].current.sha256='0'.repeat(64);},
  value=>{value.descriptorSuccessors[0].current.sourceRevision='0'.repeat(40);},
  value=>{value.descriptorSuccessors[0].sources[0].file='unowned.js';},
  value=>{value.descriptorSuccessors[0].sources[0].current.bytes+=1;},
  value=>{value.descriptorSuccessors[0].changes.reverse();},
  value=>{value.descriptorSuccessors[0].changes[0].path=['completeRust'];},
  value=>{value.descriptorSuccessors[0].changes.pop();},value=>{value.extra='unreviewed';},
 ]){
  const changed=structuredClone(completeRegister);mutate(changed);
  assert.throws(()=>restoreCompleteDeclarationReceipt(current,'raw consuming',{register:changed}),assert.AssertionError);refused+=1;
 }
 for(const mutate of [
  value=>{value.decisionSources[4].file='unowned.js';},value=>{value.decisionSources[4].bytes+=1;},
  value=>{value.decisionSources[4].sha256='0'.repeat(64);},value=>{value.decisionSources[3].bytes+=1;},
  value=>{value.completeRust+=' ';},value=>{value.owners.pop();},value=>{value.replays.reverse();},
  value=>{value.archive.textBytes+=1;},value=>{value.extra='unreviewed';},
 ]){
  const changed=structuredClone(current);mutate(changed);
  assert.throws(()=>restoreCompleteDeclarationReceipt(changed,'raw consuming'),assert.AssertionError);refused+=1;
 }
 assert.equal(refused,24);
});

test('neutral type descriptor successor preserves every earlier artifact register reader and closed source declaration',()=>{
 const baseline='18a8e0a04bab43f6572be311ceb75c6596fdc147',edge=completeRegister.typeNeutralDescriptorSuccessors[0];
 assert.equal(completeRegister.typeNeutralDescriptorSuccessors.length,1);
 assert.equal(edge.identity,'raw consuming');assert.equal(edge.sources.length,1);assert.equal(edge.changes.length,2);
 assert.deepEqual(edge.changes.map(change=>change.path),[['decisionSources','9','bytes'],['decisionSources','9','sha256']]);
 const previousRegisterBytes=execFileSync('git',['show',baseline+':parity/self-translation/complete-declaration-receipts.lino'],{cwd:root,maxBuffer:2*1024*1024});
 const {typeNeutralDescriptorSuccessors,...previousRegister}=completeRegister;
 assert.deepEqual(Buffer.from(structuredDataToLino(previousRegister)),previousRegisterBytes);
 const previousBytes=execFileSync('git',['show',baseline+':'+edge.previous.file],{cwd:root,maxBuffer:8*1024*1024});
 const previous=readStructuredDataLino(previousBytes.toString()),current=readProjectData(new URL(edge.current.file,root));
 const inverse=structuredClone(current),source=edge.sources[0];
 assert.deepEqual(inverse.decisionSources[9],{file:source.file,...source.current});
 inverse.decisionSources[9]={file:source.file,...source.previous};
 assert.deepEqual(inverse,previous);assert.deepEqual(Buffer.from(structuredDataToLino(inverse)),previousBytes);
 assert.equal(previousBytes.length,edge.previous.bytes);assert.equal(createHash('sha256').update(previousBytes).digest('hex'),edge.previous.sha256);
 const currentBytes=Buffer.from(structuredDataToLino(current));
 assert.equal(currentBytes.length,edge.current.bytes);assert.equal(createHash('sha256').update(currentBytes).digest('hex'),edge.current.sha256);
 assert.equal(current.definitionCount,106);assert.equal(current.owners.length,106);
 for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','preludeHashes','replays','archive'])assert.deepEqual(current[field],previous[field]);
 assert.deepEqual(restoreCompleteDeclarationReceipt(current,'raw consuming'),restoreCompleteDeclarationReceipt(previous,'raw consuming',{register:previousRegister}));
 assert.deepEqual(restoreCompleteDeclarationDescriptorSuccessor(current,'raw consuming',completeRegister),restoreCompleteDeclarationDescriptorSuccessor(previous,'raw consuming',previousRegister));
 const currentSource=readFileSync(new URL(source.file,root)),insertion=edge.sourceExtension;
 assert.equal(currentSource.length,source.current.bytes);assert.equal(createHash('sha256').update(currentSource).digest('hex'),source.current.sha256);
 assert.equal(currentSource.subarray(insertion.start,insertion.end).toString('utf8'),insertion.source);
 const previousSource=Buffer.concat([currentSource.subarray(0,insertion.start),currentSource.subarray(insertion.end)]);
 assert.equal(previousSource.length,source.previous.bytes);assert.equal(createHash('sha256').update(previousSource).digest('hex'),source.previous.sha256);
 assert.deepEqual(previousSource,execFileSync('git',['show','06d96082^:'+source.file],{cwd:root,maxBuffer:1024*1024}));
 const originalTree=parseNative('native-javascript',previousSource.toString('utf8')),currentTree=parseNative('native-javascript',currentSource.toString('utf8'));
 assert.equal(originalTree.hasError,false);assert.equal(currentTree.hasError,false);
 const originalNodes=originalTree.children.map(({node})=>({kind:node.term,source:previousSource.subarray(node.start,node.end).toString('utf8')}));
 const remaining=currentTree.children.filter(({node})=>node.end<=insertion.start||node.start>=insertion.end)
  .map(({node})=>({kind:node.term,source:currentSource.subarray(node.start,node.end).toString('utf8')}));
 assert.deepEqual(remaining,originalNodes);assert.equal(originalNodes.length,47);
 assert.equal(originalNodes.filter(node=>node.kind==='export_statement').length,23);
 assert.equal(originalNodes.filter(node=>node.kind==='import_statement').length,0);
 let reader=readFileSync(new URL('../scripts/complete-declaration-receipts.mjs',import.meta.url),'utf8');
 const begin=reader.indexOf('// This third finite edge records only the generated neutral type declaration.');
 const end=reader.indexOf('/** The public standalone gate retains its cloned-data result and all checks. */',begin);
 assert.ok(begin>0&&end>begin);
 reader=(reader.slice(0,begin)+reader.slice(end)).replace("import {readFileSync} from 'node:fs';\n",'')
  .replace('return readTypeNeutralCompleteDeclarationDescriptorSuccessor(artifact,identity,register).artifact;','return readCompleteDeclarationDescriptorSuccessor(artifact,identity,register).artifact;')
  .replace('const successor=readTypeNeutralCompleteDeclarationDescriptorSuccessor(artifact,identity,register);','const successor=readCompleteDeclarationDescriptorSuccessor(artifact,identity,register);');
 assert.deepEqual(Buffer.from(reader),execFileSync('git',['show',baseline+':js/scripts/complete-declaration-receipts.mjs'],{cwd:root,maxBuffer:1024*1024}),
  'the complete previous reader and every strict assertion restore exactly');
});

test('neutral type descriptor successor refuses forged source insertions lost ancestors and semantic changes',()=>{
 const current=readProjectData(new URL(completeRegister.typeNeutralDescriptorSuccessors[0].current.file,root));
 let refused=0;
 for(const mutate of [
  value=>{delete value.typeNeutralDescriptorSuccessors;},value=>{value.typeNeutralDescriptorSuccessors=[];},
  value=>{value.typeNeutralDescriptorSuccessors.push(structuredClone(value.typeNeutralDescriptorSuccessors[0]));},
  value=>{value.descriptorSuccessors[0].current.sha256='0'.repeat(64);},value=>{value.successors[0].identity='changed';},
  value=>{value.receipts[0].previous.bytes+=1;},value=>{value.typeNeutralDescriptorSuccessors[0].identity='changed';},
  value=>{value.typeNeutralDescriptorSuccessors[0].current.sha256='0'.repeat(64);},
  value=>{value.typeNeutralDescriptorSuccessors[0].previous.sha256='0'.repeat(64);},
  value=>{value.typeNeutralDescriptorSuccessors[0].sourceExtension.source+=' ';},
  value=>{value.typeNeutralDescriptorSuccessors[0].sourceExtension.start+=1;},
  value=>{value.typeNeutralDescriptorSuccessors[0].sourceExtension.end-=1;},
  value=>{value.typeNeutralDescriptorSuccessors[0].sources[0].file='unowned.js';},
  value=>{value.typeNeutralDescriptorSuccessors[0].sources[0].current.bytes+=1;},
  value=>{value.typeNeutralDescriptorSuccessors[0].changes.reverse();},
  value=>{value.typeNeutralDescriptorSuccessors[0].changes[0].path=['completeRust'];},
  value=>{value.extra='unreviewed';},
 ]){
  const changed=structuredClone(completeRegister);mutate(changed);
  assert.throws(()=>restoreCompleteDeclarationReceipt(current,'raw consuming',{register:changed}),assert.AssertionError);refused+=1;
 }
 for(const mutate of [
  value=>{value.decisionSources[9].file='unowned.js';},value=>{value.decisionSources[9].bytes+=1;},
  value=>{value.decisionSources[9].sha256='0'.repeat(64);},value=>{value.decisionSources.reverse();},
  value=>{value.completeRust+=' ';},value=>{value.owners.pop();},value=>{value.replays.reverse();},
  value=>{value.archive.textBytes+=1;},value=>{value.extra='unreviewed';},
 ]){
  const changed=structuredClone(current);mutate(changed);
  assert.throws(()=>restoreCompleteDeclarationReceipt(changed,'raw consuming'),assert.AssertionError);refused+=1;
 }
 assert.equal(refused,26);
});
