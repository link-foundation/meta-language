import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptArrayCopyFunction} from '../src/translation/javascript-array-copy-call-source.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
import {executeJavaScriptCallPrimitive} from '../src/translation/javascript-call-primitives.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)},arrayPrototype=object(10n),number=value=>({$:'number',value});
for(const operation of ['Array.toReversed','Array.with'])test('copy bytecode retains actual '+operation+' operation and realm roots',()=>{
 const fn=compileJavaScriptArrayCopyFunction(7n,8n,operation,roots,arrayPrototype);assert.ok(fn);assert.equal(fn.instructions[0].operation,'require object coercible');assert.equal(fn.instructions.some(row=>row.destination===0),false);assert.equal(fn.instructions.some(row=>row.$==='ordinaryHas'),false);assert.equal(fn.instructions.filter(row=>row.$==='arrayAllocate').length,1);
 for(const row of fn.instructions){if(row.$==='branch')for(const target of [row.yes,row.no])assert.ok(target>=0&&target<fn.instructions.length);if(row.$==='jump')assert.ok(row.target>=0&&row.target<fn.instructions.length);}
 const directory={$:'collectionCallableDirectory',realms:[8n],exoticKey:{$:'symbol',identity:12n},constructors:[],primitiveMethods:[{$:'primitiveMethodOwner',identity:7n,realm:8n,operation,wrapperRoots:roots,arrayPrototype}]};assert.equal(compileJavaScriptCollectionCallDirectory(directory).error,'');
 assert.equal(compileJavaScriptArrayCopyFunction(7n,8n,operation,undefined,arrayPrototype),undefined);assert.equal(compileJavaScriptArrayCopyFunction(7n,8n,operation,roots,undefined),undefined);
});
test('copy scalar rules retain exact bounds conversion and Error classes',()=>{
 for(const [length,index,result]of [[3,-1.9,2],[3,NaN,0],[3,-0,0],[3,-3,0],[3,2.9,2]])assert.deepEqual(executeJavaScriptCallPrimitive('read checked array replacement index',[number(length),number(index)]),{$:'primitiveValue',value:number(result)});
 for(const [length,index]of [[0,0],[3,3],[3,-4],[3,Infinity],[3,-Infinity]])assert.equal(executeJavaScriptCallPrimitive('read checked array replacement index',[number(length),number(index)]).errorClass,'RangeError');
 assert.equal(executeJavaScriptCallPrimitive('require Array creation length',[number(4294967295)]).$,'primitiveValue');assert.equal(executeJavaScriptCallPrimitive('require Array creation length',[number(4294967296)]).errorClass,'RangeError');
});
