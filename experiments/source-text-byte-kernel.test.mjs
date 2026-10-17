import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {TextEncoder,TextDecoder} from 'node:util';
import * as kernel from './source-text-byte-rules.mjs';
import {sourceTextByteRulesSource} from './generate-source-text-byte-rules.mjs';
import {readSourceTextUnits,sliceSourceTextBytes} from './source-text-byte-adapter.mjs';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';

const encoder=new TextEncoder(),decoder=new TextDecoder('utf-8',{fatal:true});
const points=[0,1,0x7f,0x80,0x7ff,0x800,0xd7ff,0xe000,0xffff,0x10000,0x10ffff,0xfdd0,0x378,0x1f600];
const texts=['',...points.map(point=>String.fromCodePoint(point)),'AΩ😀z','e\u0301','\u0000\u007f\u0080\u07ff\u0800\uffff','🗻∈🌏',String.fromCodePoint(...points)];
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
const invalid=[[-1],[65536],[1.5],[NaN],[Infinity],[-Infinity],[0xd800],[0xdfff],[0xd800,65],[0xd800,0xd800,0xdc00],[0xdc00,0xd800],[65,0xdc00],[0xd800,0xdc00,0xdfff]];
function observe(implementation){
 for(const text of texts){
  const input=units(text),bytes=encoder.encode(text);
  assert.equal(implementation.acceptSourceTextScalarUnits(input),true);
  assert.equal(implementation.readSourceTextByteLength(input),bytes.length);
  const indices=new Map();let index=0;
  for(const character of text){indices.set(index,character.codePointAt(0));index+=character.length;}
  for(let position=0;position<=input.length;position++)assert.equal(implementation.readSourceTextScalar(input,position),indices.get(position)??-1);
  for(let offset=-1;offset<=bytes.length+1;offset++){
   const boundary=offset>=0&&offset<=bytes.length&&(offset===bytes.length||(bytes[offset]&0xc0)!==0x80);
   assert.equal(implementation.readSourceTextCodeUnitOffset(input,offset),boundary?decoder.decode(bytes.subarray(0,offset)).length:-1);
   assert.equal(implementation.readSourceTextByte(input,offset),offset>=0&&offset<bytes.length?bytes[offset]:-1);
   for(let end=-1;end<=bytes.length+1;end++){
    const endBoundary=end>=0&&end<=bytes.length&&(end===bytes.length||(bytes[end]&0xc0)!==0x80);
    assert.equal(implementation.acceptSourceTextByteSlice(input,offset,end),offset<=end&&boundary&&endBoundary);
   }
  }
  for(const offset of [NaN,Infinity,-Infinity,0.5]){
   assert.equal(implementation.readSourceTextCodeUnitOffset(input,offset),-1);
   assert.equal(implementation.readSourceTextByte(input,offset),-1);
   assert.equal(implementation.acceptSourceTextByteSlice(input,offset,0),false);
  }
 }
 for(const input of invalid){
  assert.equal(implementation.acceptSourceTextScalarUnits(input),false);
  assert.equal(implementation.readSourceTextByteLength(input),-1);
  assert.equal(implementation.readSourceTextCodeUnitOffset(input,0),-1);
  assert.equal(implementation.readSourceTextByte(input,0),-1);
  assert.equal(implementation.acceptSourceTextByteSlice(input,0,0),false);
 }
 for(const point of [-1,0xd800,0xdfff,0x110000,0.5,NaN,Infinity])assert.equal(implementation.readSourceTextScalarByteWidth(point),-1);
}

test('linked policy numeric kernel preserves exact UTF-8 bytes and scalar boundaries against independent encoders',()=>{
 assert.equal(readFileSync(new URL('./source-text-byte-rules.mjs',import.meta.url),'utf8'),sourceTextByteRulesSource());
 const model=readProjectData(new URL('./source-text-byte-model.lino',import.meta.url));
 assert.equal(model.constants.maximumUnits,0xffffffff);
 observe(kernel);
 for(const input of [[0,0xd800],[0xd800,0xdc00,0xd800]]){
  assert.equal(kernel.readSourceTextCodeUnitOffset(input,0),-1,'an invalid suffix cannot authorize an otherwise valid prefix boundary');
 }
});

test('the complete numeric kernel replays from checked Links without its source and automatically translates to Rust',()=>{
 const source=sourceTextByteRulesSource(),program=checkProgram(parseJavaScript(source));
 const encoded=encodeTypedProgramLinks(program),loaded=LinkNetwork.fromLino(encoded.network.toLino()),decoded=decodeTypedProgramLinks(loaded,encoded.root);
 assert.ok(loaded.links().every(link=>link.metadata().definition!==source));
 const emitted=emitJavaScript(decoded),context={};vm.runInNewContext(emitted.text,context,{timeout:1000});
 const implementation=Object.fromEntries(Object.keys(kernel).map(name=>[name,context[emitted.mappings.find(row=>row.source===name).target]]));
 observe(implementation);
 const rust=emitRust(decoded);
 assert.equal(rust.definitions.length,program.declarations.size);
 for(const name of Object.keys(kernel))assert.ok(rust.mappings.some(row=>row.source===name));
 assert.ok(rust.definitions.some(value=>value.includes('pub fn read_source_text_byte(')));
 assert.ok(rust.definitions.some(value=>value.includes('pub fn read_source_text_code_unit_offset(')));
});

test('source-free changes alter the actual byte-width algorithm rather than an opaque source envelope',()=>{
 const encoded=encodeTypedProgramLinks(checkProgram(parseJavaScript(sourceTextByteRulesSource())));
 const network=LinkNetwork.fromLino(encoded.network.toLino()),program=decodeTypedProgramLinks(network,encoded.root);
 const pending=[program.declarations.get('readSourceTextScalarByteWidth').body];let changed=false;
 while(pending.length){const value=pending.pop();if(!value||typeof value!=='object')continue;
  if(value.k==='binary'&&value.op==='lt'&&value.right?.k==='lit'&&value.right.value==='128'){value.right.value='64';changed=true;break;}
  for(const child of Object.values(value))if(Array.isArray(child))pending.push(...child);else if(child&&typeof child==='object')pending.push(child);
 }
 assert.equal(changed,true);
 const edited=encodeTypedProgramLinks(program),restored=decodeTypedProgramLinks(LinkNetwork.fromLino(edited.network.toLino()),edited.root);
 const emitted=emitJavaScript(restored),context={};vm.runInNewContext(emitted.text,context,{timeout:1000});
 const name=emitted.mappings.find(row=>row.source==='readSourceTextByteLength').target;
 assert.equal(context[name]([65]),2);assert.equal(encoder.encode('A').length,1);
});

test('thin primitive-string adapter preserves exact valid byte slices and refuses replacement or coercion',()=>{
 for(const text of texts){const bytes=encoder.encode(text);assert.deepEqual(readSourceTextUnits(text),units(text));
  for(let start=0;start<=bytes.length;start++)for(let end=0;end<=bytes.length;end++){
   let expected=null;
   if(start<=end&&(start===bytes.length||(bytes[start]&0xc0)!==0x80)&&(end===bytes.length||(bytes[end]&0xc0)!==0x80))expected=decoder.decode(bytes.subarray(start,end));
   assert.equal(sliceSourceTextBytes(text,start,end),expected);
  }
 }
 for(const text of ['\ud800','\udfff','A\ud800','\ud800\ud800\udc00']){
  assert.ok(encoder.encode(text).length>0,'TextEncoder replacement is excluded from valid-input evidence');
  assert.throws(()=>readSourceTextUnits(text),/unpaired Unicode surrogate/u);
  assert.throws(()=>sliceSourceTextBytes(text,0,0),/unpaired Unicode surrogate/u);
 }
 for(const value of [null,undefined,42,{},new String('boxed')])assert.throws(()=>readSourceTextUnits(value),/primitive string/u);
 let coerced=0;const object={valueOf(){coerced++;return 0;}};
 for(const offset of ['0',object,null,undefined,Symbol('offset')])assert.equal(sliceSourceTextBytes('A',offset,1),null);
 assert.equal(coerced,0);
});
