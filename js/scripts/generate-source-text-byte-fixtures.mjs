import assert from 'node:assert/strict';
import {TextEncoder} from 'node:util';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import * as rules from '../src/translation/source-text-byte-rules.js';
const encoder=new TextEncoder();
const points=[0,1,0x7f,0x80,0x7ff,0x800,0xd7ff,0xe000,0xffff,0x10000,0x10ffff,0xfdd0,0x378,0x1f600];
const texts=['',...points.map(point=>String.fromCodePoint(point)),'AΩ😀z','e\u0301','🗻∈🌏'];
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
const cases=texts.map((text,index)=>{
 const input=units(text),bytes=[...encoder.encode(text)],positions=Array(bytes.length+1).fill(-1);
 let byte=0,position=0;positions[0]=0;
 for(const character of text){byte+=encoder.encode(character).length;position+=character.length;positions[byte]=position;}
 assert.equal(rules.readSourceTextByteLength(input),bytes.length);
 for(let offset=0;offset<=bytes.length;offset++){
  assert.equal(rules.readSourceTextByte(input,offset),bytes[offset]??-1);
  assert.equal(rules.readSourceTextCodeUnitOffset(input,offset),positions[offset]);
 }
 return{name:index===0?'empty text':`Unicode scalar text ${index}`,units:input,bytes,positions};
});
const invalidUnits=[['-1'],['65536'],['1.5'],['NaN'],['Infinity'],['-Infinity'],['55296'],['57343'],['55296','65'],['55296','55296','56320'],['56320','55296'],['65','56320'],['55296','56320','57343']];
const fixture={schemaVersion:1,model:readProjectData(new URL('../src/translation/data/source-text-byte-model.lino',import.meta.url)).model,
 oracle:'Valid bytes come from the independent standard TextEncoder; scalar boundaries accumulate complete original Unicode scalars. Invalid unit vectors are refused before any replacement encoder is used.',
 cases,invalidUnits,invalidScalars:['-1','0.5','55296','57343','1114112','NaN','Infinity','-Infinity'],invalidOffsets:['-1','0.5','NaN','Infinity','-Infinity'],zeroOffsets:['0','-0']};
const file=new URL('../../parity/fixtures/source-text-bytes.lino',import.meta.url);
if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);
console.log(`Source-text byte fixture: ${cases.length} exact encodings and ${invalidUnits.length} malformed numeric unit vectors`);
