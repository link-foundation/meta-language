// Deferred source controls: execute after the complete requirement draft.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileJavaScriptArrayMutationFunction} from '../src/translation/javascript-array-mutation-call-source.js';
import {compileJavaScriptBoxedArrayMutationFunction} from '../src/translation/javascript-boxed-array-mutation-call-source.js';
import {javascriptArrayMutationBoxingOriginals,javascriptArrayMutationBoxingAssertionPolicy} from './fixtures/javascript-array-mutation-boxing.mjs';

const object=identity=>({$:'object',identity,callable:false});
const roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)};
const operations=['Array.push','Array.pop','Array.shift','Array.unshift','Array.reverse','Array.fill','Array.copyWithin'];

test('boxed mutation compilation requires authenticated complete realm wrapper roots',()=>{
 assert.equal(compileJavaScriptBoxedArrayMutationFunction(7n,8n,'Array.unknown',roots),undefined);
 assert.equal(compileJavaScriptBoxedArrayMutationFunction(7n,8n,'Array.push',undefined),undefined);
 for(const key of ['objectPrototype','stringPrototype','numberPrototype','booleanPrototype','integerPrototype','symbolPrototype']){
  assert.equal(compileJavaScriptBoxedArrayMutationFunction(7n,8n,'Array.push',{...roots,[key]:{$:'null'}}),undefined);
  assert.equal(compileJavaScriptBoxedArrayMutationFunction(7n,8n,'Array.push',{...roots,[key]:{...roots[key],callable:true}}),undefined);
 }
});

for(const operation of operations)test(operation+' composes ToObject without changing the original mutation algorithm',()=>{
 const original=compileJavaScriptArrayMutationFunction(7n,8n,operation);
 const fn=compileJavaScriptBoxedArrayMutationFunction(7n,8n,operation,roots),held=original.registers;
 const offset=fn.instructions.length-original.instructions.length,prefix=fn.instructions.slice(0,offset),body=fn.instructions.slice(offset);
 assert.equal(fn.registers,held+5);assert.equal(fn.receiverMode,'primitive intrinsic');
 assert.deepEqual(fn.captures,original.captures);assert.equal(fn.identity,7n);assert.equal(fn.realm,8n);
 assert.deepEqual(prefix[0],{$:'primitive',destination:held+3,operation:'require object coercible',arguments:[0]});
 assert.deepEqual(prefix.find(row=>row.$==='primitiveWrapper'),{$:'primitiveWrapper',destination:held,input:0,prototype:held+4});
 assert.ok(prefix.some(row=>row.$==='copy'&&row.destination===held&&row.source===0));
 assert.deepEqual(body[0],{...original.instructions[0],arguments:[held]});
 const firstGet=body.find(row=>row.$==='ordinaryGet');assert.equal(firstGet.object,held);assert.equal(firstGet.receiver,held);
 assert.equal(fn.instructions.some(row=>'destination' in row&&row.destination===0),false);
 // Compare every instruction with its original after undoing only the allowed
 // register/coordinate relocation. This retains strict checks and loop bodies.
 const fields={ordinaryGet:['object','key','receiver'],ordinarySet:['object','key','value','receiver'],ordinaryHas:['object','key'],ordinaryDelete:['object','key'],copy:['source'],return:['source'],numberBinary:['left','right'],strictEqual:['left','right'],propertyKey:['input'],requireWrite:['success'],argumentAt:['index'],branch:['condition']};
 for(let index=0;index<body.length;index++){
  const row={...body[index]};
  for(const field of fields[row.$]??[])if(row[field]===held)row[field]=0;
  if(row.$==='primitive')row.arguments=row.arguments.map(value=>value===held?0:value);
  if(row.$==='branch'){row.yes-=offset;row.no-=offset;}
  if(row.$==='jump')row.target-=offset;
  assert.deepEqual(row,original.instructions[index],operation+' instruction '+index);
 }
 for(const row of fn.instructions)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(Number.isInteger(target)&&target>=0&&target<fn.instructions.length);
 for(const row of body)if(row.$==='branch'||row.$==='jump')for(const target of row.$==='branch'?[row.yes,row.no]:[row.target])assert.ok(target>=offset);
 assert.deepEqual(compileJavaScriptArrayMutationFunction(7n,8n,operation),original);
});

test('receiver-returning mutations return the held wrapper and length-writing mutations retain strict Set',()=>{
 for(const operation of operations){
  const original=compileJavaScriptArrayMutationFunction(7n,8n,operation),fn=compileJavaScriptBoxedArrayMutationFunction(7n,8n,operation,roots);
  if(['Array.reverse','Array.fill','Array.copyWithin'].includes(operation))assert.equal(fn.instructions.at(-1).source,original.registers);
  for(let index=0;index<fn.instructions.length;index++)if(fn.instructions[index].$==='ordinarySet')assert.equal(fn.instructions[index+1].$,'requireWrite');
 }
});

test('boxing original source obligations remain independent expected-true version bounded inputs',()=>{
 assert.ok(javascriptArrayMutationBoxingOriginals.length>=14);
 assert.equal(new Set(javascriptArrayMutationBoxingOriginals.map(row=>row.name)).size,javascriptArrayMutationBoxingOriginals.length);
 for(const fixture of javascriptArrayMutationBoxingOriginals){assert.equal(fixture.expected,true);assert.equal(typeof fixture.source,'string');assert.deepEqual(fixture.entries,['read']);assert.equal(fixture.entry,'read');assert.deepEqual(fixture.arguments,[]);}
 assert.deepEqual(javascriptArrayMutationBoxingAssertionPolicy.expectations.map(row=>[row.minimum,row.maximum]),[['22.15.0','23.0.0'],['24.0.0','25.0.0']]);
});
