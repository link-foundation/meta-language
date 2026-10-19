import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {TextEncoder,TextDecoder} from 'node:util';
import {readProjectData} from '../scripts/project-data.mjs';
import * as rules from '../src/translation/source-text-byte-rules.js';
import {readSourceTextUnits,readSourceTextByteLength,readSourceTextByte,sliceSourceTextBytes} from '../src/translation/source-text-bytes.js';
import {sourceTextByteRulesSource} from '../../experiments/generate-source-text-byte-rules.mjs';
const fixture=readProjectData(new URL('../../parity/fixtures/source-text-bytes.lino',import.meta.url));
const encoder=new TextEncoder(),decoder=new TextDecoder('utf-8',{fatal:true});
const source=row=>String.fromCharCode(...row.units);

test('both runtime models preserve the frozen seven algorithms and their exact linked assumptions',()=>{
 const text=readFileSync(new URL('../src/translation/source-text-byte-rules.js',import.meta.url),'utf8');
 assert.ok(text.startsWith(sourceTextByteRulesSource()));
 const original=readFileSync(new URL('../../experiments/source-text-byte-model.lino',import.meta.url));
 for(const path of ['../src/translation/data/source-text-byte-model.lino','../../rust/src/data/source-text-byte-model.lino'])assert.deepEqual(readFileSync(new URL(path,import.meta.url)),original);
 for(const count of [0,0xffffffff])assert.equal(rules.acceptSourceTextUnitCount(count),true);
 for(const count of [-1,0.5,0x100000000,NaN,Infinity])assert.equal(rules.acceptSourceTextUnitCount(count),false);
 const native=readFileSync(new URL('../../rust/src/translation/source_text_byte_rules.rs',import.meta.url),'utf8');
 assert.ok(!native.includes('#[allow('));assert.ok(!native.includes('.mul_add('));assert.ok(!native.includes(' as f64'));
 assert.match(native,/let scaled = \(first - 55_296f64\) \* 1024f64/u);
});

test('actual host facades and private numeric rules match all frozen independent byte and boundary rows',()=>{
 assert.equal(fixture.cases.length,18);assert.equal(fixture.invalidUnits.length,13);
 for(const row of fixture.cases){
  const text=source(row),bytes=encoder.encode(text);
  assert.deepEqual([...bytes],row.bytes,row.name);assert.deepEqual(readSourceTextUnits(text),row.units,row.name);
  assert.equal(readSourceTextByteLength(text),row.bytes.length,row.name);
  assert.equal(rules.readSourceTextByteLength(row.units),row.bytes.length,row.name);
  for(let offset=0;offset<=bytes.length+1;offset++){
   assert.equal(readSourceTextByte(text,offset),row.bytes[offset]??null,row.name);
   assert.equal(rules.readSourceTextByte(row.units,offset),row.bytes[offset]??-1,row.name);
   assert.equal(rules.readSourceTextCodeUnitOffset(row.units,offset),row.positions[offset]??-1,row.name);
   for(let end=0;end<=bytes.length+1;end++){
    const accepted=offset<=end&&(row.positions[offset]??-1)>=0&&(row.positions[end]??-1)>=0;
    assert.equal(rules.acceptSourceTextByteSlice(row.units,offset,end),accepted,row.name);
    assert.equal(sliceSourceTextBytes(text,offset,end),accepted?decoder.decode(bytes.subarray(offset,end)):null,row.name);
   }
  }
  for(const spelling of fixture.invalidOffsets){const offset=Number(spelling);assert.equal(readSourceTextByte(text,offset),null);assert.equal(sliceSourceTextBytes(text,offset,0),null);}
  for(const spelling of fixture.zeroOffsets){const offset=Number(spelling);assert.equal(sliceSourceTextBytes(text,offset,offset),'');assert.equal(readSourceTextByte(text,offset),row.bytes[0]??null);}
 }
});

test('malformed complete numeric unit vectors and surrogate text refuse before replacement',()=>{
 for(const encoded of fixture.invalidUnits){const units=encoded.map(Number);
  assert.equal(rules.acceptSourceTextScalarUnits(units),false);assert.equal(rules.readSourceTextByteLength(units),-1);
  assert.equal(rules.readSourceTextCodeUnitOffset(units,0),-1);assert.equal(rules.readSourceTextByte(units,0),-1);
  assert.equal(rules.acceptSourceTextByteSlice(units,0,0),false);
 }
 for(const spelling of fixture.invalidScalars)assert.equal(rules.readSourceTextScalarByteWidth(Number(spelling)),-1);
 for(const text of ['\ud800','\udfff','A\ud800','\ud800\ud800\udc00'])for(const read of [readSourceTextUnits,readSourceTextByteLength,text=>readSourceTextByte(text,0),text=>sliceSourceTextBytes(text,0,0)])assert.throws(()=>read(text),/unpaired Unicode surrogate/u);
});

test('primitive text and offset admission never coerces boxed strings or foreign index values',()=>{
 for(const value of [null,undefined,1,{},new String('boxed')])for(const read of [readSourceTextUnits,readSourceTextByteLength,text=>readSourceTextByte(text,0),text=>sliceSourceTextBytes(text,0,0)])assert.throws(()=>read(value),/primitive string/u);
 let calls=0;const foreign={valueOf(){calls++;return 0;}};
 for(const offset of [foreign,'0',null,undefined,Symbol('offset')]){assert.equal(readSourceTextByte('A',offset),null);assert.equal(sliceSourceTextBytes('A',offset,1),null);}
 assert.equal(calls,0);
});
