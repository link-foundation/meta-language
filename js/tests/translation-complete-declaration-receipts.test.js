import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {COMPLETE_DECLARATION_RECEIPTS,restoreCompleteDeclarationReceipt} from '../scripts/complete-declaration-receipts.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino,structuredDataToLino} from '../src/structured-data-links.js';
import {tokenize} from '../src/translation/lexer.js';

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
