// Deferred until the complete requirements draft is ready for testing.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readUtf16TextRepetitionLength,readUtf16TextPaddingLength,constructUtf16TextRepetition,constructUtf16TextPadding} from '../src/translation/utf16-text-expansion-rules.js';
import {executeJavaScriptCallPrimitive,readJavaScriptCallPrimitiveArityError} from '../src/translation/javascript-call-primitives.js';
const fixture=readProjectData(new URL('../../parity/fixtures/utf16-text-expansion.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.node);
const text=units=>({$:'text',units}),number=value=>({$:'number',value}),units=value=>Array.from({length:value.length},(_,index)=>value.charCodeAt(index));
const string=value=>value.map(unit=>String.fromCharCode(unit)).join('');
for(const [index,row]of fixture.repetition.entries())test('repetition source/native plan '+index,()=>{
 const count=Number(row.count),original=[...row.text];
 assert.equal(readUtf16TextRepetitionLength(row.text.length,count),row.length);
 const result=executeJavaScriptCallPrimitive('repeat text',[text(row.text),number(count)]);
 if(row.length===-1){
  let expected;try{string(row.text).repeat(count);}catch(error){expected=error;}
  assert.ok(expected);assert.deepEqual(result,{$:'primitiveError',errorClass:expected.name,reason:expected.message});
 }else if(row.length===-2)assert.equal(result.$,'primitiveResourceLimit');
 else{
  const expected=units(string(row.text).repeat(count));
  assert.equal(expected.length,row.length);assert.deepEqual(result,{$:'primitiveValue',value:text(expected)});
  assert.deepEqual(constructUtf16TextRepetition(row.text,row.length),expected);
  if(row.expected)assert.deepEqual(expected,row.expected);else assert.equal(expected.length,row.expectedLength);
 }
 assert.deepEqual(row.text,original);
});
for(const [index,row]of fixture.padding.entries())test('padding source/native plan '+index,()=>{
 const maximum=Number(row.maximum),original=[...row.text],filler=[...row.filler];
 assert.equal(readUtf16TextPaddingLength(row.text.length,maximum,row.filler.length),row.length);
 const operation=row.start?'pad text start':'pad text end',result=executeJavaScriptCallPrimitive(operation,[text(row.text),number(maximum),text(row.filler)]);
 if(row.length<0)assert.equal(result.$,'primitiveResourceLimit');
 else{
  const expected=units(string(row.text)[row.start?'padStart':'padEnd'](maximum,string(row.filler)));
  assert.equal(expected.length,row.length);assert.deepEqual(result,{$:'primitiveValue',value:text(expected)});
  assert.deepEqual(constructUtf16TextPadding(row.text,row.filler,row.length,row.start),expected);
  if(row.expected)assert.deepEqual(expected,row.expected);else assert.equal(expected.length,row.expectedLength);
 }
 assert.deepEqual(row.text,original);assert.deepEqual(row.filler,filler);
});
test('expansion keeps complete operand admission',()=>{
 for(const [operation,values]of [['repeat text',[text([97]),number(2)]],['pad text start',[text([97]),number(3),text([32])]],['pad text end',[text([97]),number(3),text([32])]]]){
  assert.equal(readJavaScriptCallPrimitiveArityError(operation,values.length),'');
  for(const count of [0,1,4])assert.notEqual(readJavaScriptCallPrimitiveArityError(operation,count),'');
  for(let index=0;index<values.length;index+=1){const altered=[...values];altered[index]={$:'undefined'};assert.equal(executeJavaScriptCallPrimitive(operation,altered).$,'primitiveInvalid');}
 }
 for(const length of [-1,0.5,1048577,Infinity,NaN]){
  assert.equal(readUtf16TextRepetitionLength(length,1),-2);
  assert.equal(readUtf16TextPaddingLength(length,0,1),-2);
  assert.equal(readUtf16TextPaddingLength(0,0,length),-2);
 }
 assert.throws(()=>constructUtf16TextRepetition([],1),/complete admitted length/u);
 assert.throws(()=>constructUtf16TextRepetition([97,98],3),/complete admitted length/u);
 assert.throws(()=>constructUtf16TextPadding([97],[],2,true),/complete admitted length/u);
});
