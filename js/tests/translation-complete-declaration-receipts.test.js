import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {COMPLETE_DECLARATION_RECEIPTS,restoreCompleteDeclarationReceipt} from '../scripts/complete-declaration-receipts.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino,structuredDataToLino} from '../src/structured-data-links.js';

const root=new URL('../../',import.meta.url),register=readProjectData(COMPLETE_DECLARATION_RECEIPTS);

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
