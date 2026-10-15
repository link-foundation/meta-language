import assert from 'node:assert/strict';
import {mkdtempSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import test from 'node:test';
import {TextEncoder,TextDecoder} from 'node:util';
import {publicDeclarationConsumers} from '../scripts/check-public-declaration-contracts.mjs';
import {readProjectData} from '../scripts/project-data.mjs';

const modulePath=process.env.META_LANGUAGE_SOURCE_TEXT_BYTE_PUBLIC_MODULE??fileURLToPath(new URL('../src/index.js',import.meta.url));
const api=await import(pathToFileURL(modulePath).href);
const fixture=readProjectData(new URL('../../parity/fixtures/source-text-bytes.lino',import.meta.url));
const consumers=new Map(publicDeclarationConsumers(modulePath).map(entry=>[entry.name,entry]));
const encoder=new TextEncoder(),decoder=new TextDecoder('utf-8',{fatal:true});
async function runConsumer(name,context){
 const consumer=consumers.get(name);assert.ok(consumer,'the actual declared byte consumer exists: '+name);
 const directory=mkdtempSync(join(tmpdir(),'meta-language-source-text-byte-public-consumer-'));context.after(()=>rmSync(directory,{recursive:true,force:true}));
 const file=join(directory,'consumer.mjs');writeFileSync(file,api.eraseTypeScript(consumer.source).code);return import(pathToFileURL(file).href);
}

for(const [index,row]of fixture.cases.entries()){
 test('the exact public byte consumer retains '+row.name+' and every original scalar boundary',async context=>{
  const consumer=await runConsumer('valid-source-text-byte-facades-'+index,context),evidence=consumer.contractEvidence,bytes=encoder.encode(evidence.text);
  assert.deepEqual(evidence.units,row.units);assert.deepEqual([...bytes],row.bytes);assert.equal(evidence.length,row.bytes.length);
  assert.equal(evidence.firstByte,row.bytes[0]??null);assert.equal(evidence.whole,evidence.text);
  for(let start=0;start<=bytes.length+1;start++){
   assert.equal(consumer.byteAt(start),row.bytes[start]??null);
   for(let end=0;end<=bytes.length+1;end++){
    const accepted=start<=end&&(row.positions[start]??-1)>=0&&(row.positions[end]??-1)>=0;
    assert.equal(consumer.sliceAt(start,end),accepted?decoder.decode(bytes.subarray(start,end)):null,row.name+' '+start+'..'+end);
   }
  }
  for(const spelling of fixture.invalidOffsets){const offset=Number(spelling);assert.equal(consumer.byteAt(offset),null);assert.equal(consumer.sliceAt(offset,0),null);}
  for(const spelling of fixture.zeroOffsets){const offset=Number(spelling);assert.equal(consumer.byteAt(offset),row.bytes[0]??null);assert.equal(consumer.sliceAt(offset,offset),'');}
 });
}

test('the exact typed byte boundary consumer refuses nonprimitive and unpaired text without coercing foreign positions',async context=>{
 const consumer=await runConsumer('valid-source-text-byte-runtime-boundaries',context);
 for(const text of[null,undefined,1,{},new String('boxed')])for(const read of[consumer.units,consumer.length,value=>consumer.byte(value,0),value=>consumer.slice(value,0,0)])assert.throws(()=>read(text),/primitive string/u);
 for(const text of['\ud800','\udfff','A\ud800','\ud800\ud800\udc00'])for(const read of[consumer.units,consumer.length,value=>consumer.byte(value,0),value=>consumer.slice(value,0,0)])assert.throws(()=>read(text),/unpaired Unicode surrogate/u);
 let calls=0;const foreign={valueOf(){calls++;return 0;}};
 for(const offset of[foreign,'0',null,undefined,Symbol('offset')]){assert.equal(consumer.byte('A',offset),null);assert.equal(consumer.slice('A',offset,1),null);}
 assert.equal(calls,0);
});

for(const selection of['units','length','byte','slice']){
 test('the exact nonstring '+selection+' byte consumer rejects its illegal text at runtime',async context=>{
  await assert.rejects(runConsumer('reject-source-text-byte-nonstring-'+selection,context),{name:'TypeError',message:'source text must be a primitive string'});
 });
}

for(const selection of['byte','slice']){
 test('the exact foreign '+selection+' offset consumer returns its declared nullable boundary',async context=>{
  const consumer=await runConsumer('reject-source-text-byte-foreign-offset-'+selection,context);assert.equal(consumer.contractEvidence.value,null);
 });
}

for(const selection of['number','string']){
 test('the exact byte consumer rejected for nullable '+selection+' actually returns null',async context=>{
  const consumer=await runConsumer('reject-source-text-byte-null-as-'+selection,context);assert.equal(consumer.contractEvidence.value,null);
 });
}
