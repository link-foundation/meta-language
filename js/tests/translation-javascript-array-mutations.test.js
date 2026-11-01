// Deferred mutation bytecode and primitive controls; no behavior at import.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptArrayMutationFunction} from '../src/translation/javascript-array-mutation-call-source.js';
import {executeJavaScriptCallPrimitive} from '../src/translation/javascript-call-primitives.js';
test('push uses actual argument count indexed arguments and ordered shared Set operations',()=>{
 const fn=compileJavaScriptArrayMutationFunction(1n,2n,'Array.push'),code=fn.instructions;assert.equal(fn.receiverMode,'primitive intrinsic');const count=code.findIndex(row=>row.$==='argumentCount'),check=code.findIndex(row=>row.$==='primitive'&&row.operation==='read checked array append length'),read=code.findIndex(row=>row.$==='argumentAt'),writes=code.flatMap((row,index)=>row.$==='ordinarySet'?[index]:[]);assert.ok(count<check&&check<read&&read<writes[0]);assert.equal(writes.length,2);assert.equal(code[writes[0]].key,6);assert.equal(code[writes[1]].key,1);assert.equal(code.some(row=>row.$==='arrayAllocate'),false);assert.equal(compileJavaScriptArrayMutationFunction(1n,2n,'Array.unknown'),undefined);
});
test('pop holds its selected value before strict deletion and final length Set',()=>{
 const code=compileJavaScriptArrayMutationFunction(1n,2n,'Array.pop').instructions,read=code.findIndex(row=>row.$==='ordinaryGet'&&row.destination===5),remove=code.findIndex(row=>row.$==='ordinaryDelete'),check=code.findIndex(row=>row.$==='primitive'&&row.operation==='require delete success'),write=code.findIndex(row=>row.$==='ordinarySet');assert.ok(read<remove&&remove<check&&check<write);assert.equal(code[code.length-1].source,5);const zero=code.find(row=>row.$==='branch');assert.equal(zero.yes,write);
});
test('mutation primitive boundaries retain ToLength safe append and strict delete outcomes',()=>{
 const number=value=>({$:'number',value});for(const [input,expected]of [[NaN,0],[-1,0],[3.9,3],[Infinity,9007199254740991]])assert.equal(executeJavaScriptCallPrimitive('read array like length',[number(input)]).value.value,expected);
 assert.equal(executeJavaScriptCallPrimitive('read checked array append length',[number(9007199254740991),number(0)]).value.value,9007199254740991);assert.equal(executeJavaScriptCallPrimitive('read checked array append length',[number(9007199254740991),number(1)]).errorClass,'TypeError');assert.equal(executeJavaScriptCallPrimitive('require delete success',[{$:'boolean',value:false}]).errorClass,'TypeError');assert.equal(executeJavaScriptCallPrimitive('require array method object',[{$:'text',units:[]}]).$,'primitiveInvalid');
});
