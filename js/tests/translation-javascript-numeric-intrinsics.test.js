// Deferred numeric intrinsic source controls; no compilation occurs on import.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptNumericIntrinsicFunction,readJavaScriptNumberPredicateNames,readJavaScriptMathMethodNames} from '../src/translation/javascript-numeric-intrinsic-call-source.js';
import {executeJavaScriptCallPrimitive} from '../src/translation/javascript-call-primitives.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
test('Number call distinguishes missing arguments from explicit undefined and uses explicit numeric conversion',()=>{
 const code=compileJavaScriptNumericIntrinsicFunction(1n,2n,'Number').instructions;assert.equal(code[0].$,'argumentCount');assert.equal(code[5].operation,'convert explicit number');assert.deepEqual(code[7],{$:'return',source:2});assert.deepEqual(code[1].value,{$:'number',value:0});
});
test('Number predicates check actual Number type without invoking object conversion',()=>{
 for(const name of readJavaScriptNumberPredicateNames()){const code=compileJavaScriptNumericIntrinsicFunction(1n,2n,'Number.'+name).instructions;assert.equal(code[1].operation,'read semantic type');assert.equal(code.some(row=>row.$==='primitive'&&(row.operation==='coerce primitive number'||row.operation==='convert explicit number')),false);assert.equal(code[7].value.value,false);}
 assert.equal(executeJavaScriptCallPrimitive('test NaN number',[{$:'number',value:NaN}]).value.value,true);assert.equal(executeJavaScriptCallPrimitive('test finite number',[{$:'number',value:Infinity}]).value.value,false);assert.equal(executeJavaScriptCallPrimitive('test finite number',[{$:'number',value:-0}]).value.value,true);
});
test('Math extrema consume every actual argument and preserve signed-zero reduction',()=>{
 for(const name of ['min','max']){const code=compileJavaScriptNumericIntrinsicFunction(1n,2n,'Math.'+name).instructions;assert.equal(code.some(row=>row.$==='argumentAt'),true);assert.equal(code.some(row=>row.$==='arrayAllocate'||row.$==='argumentRest'),false);assert.ok(code.some(row=>row.$==='primitive'&&row.operation==='coerce primitive number'));}
 assert.equal(Object.is(executeJavaScriptCallPrimitive('read minimum number',[{$:'number',value:0},{$:'number',value:-0}]).value.value,-0),true);assert.equal(Object.is(executeJavaScriptCallPrimitive('read maximum number',[{$:'number',value:-0},{$:'number',value:0}]).value.value,0),true);
});
test('numeric catalog entries retain complete branch targets and actual constructor classification',()=>{
 for(const operation of ['Number',...readJavaScriptNumberPredicateNames().map(name=>'Number.'+name),...readJavaScriptMathMethodNames().map(name=>'Math.'+name)]){const fn=compileJavaScriptNumericIntrinsicFunction(1n,2n,operation);for(const row of fn.instructions)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(Number.isInteger(target)&&target>=0&&target<fn.instructions.length);}
 const directory={$:'collectionCallableDirectory',realms:[2n],exoticKey:{$:'symbol',identity:9n},constructors:[],primitiveMethods:[{$:'primitiveMethodOwner',identity:1n,realm:2n,operation:'Number'}]};assert.notEqual(compileJavaScriptCollectionCallDirectory(directory).error,'');assert.equal(compileJavaScriptCollectionCallDirectory({...directory,primitiveMethods:[{...directory.primitiveMethods[0],constructable:true}]}).error,'');
});
