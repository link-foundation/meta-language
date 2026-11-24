// Deferred protected bytecode controls. Never execute during source drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileJavaScriptReflectConstructionFunction} from '../src/translation/javascript-reflect-construction-call-source.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';

test('Reflect.construct checks target then default or supplied newTarget before argument Object reads',()=>{
 const fn=compileJavaScriptReflectConstructionFunction(7n,8n),code=fn.instructions;
 const checks=code.map((row,index)=>({row,index})).filter(({row})=>row.$==='constructorCheck');
 assert.equal(checks.length,2);assert.deepEqual(checks.map(({row})=>[row.input,row.required]),[[1,true],[3,true]]);
 const requireObject=code.findIndex(row=>row.$==='primitive'&&row.operation==='require array like arguments Object');
 assert.ok(checks.every(({index})=>index<requireObject));
 const count=code.findIndex(row=>row.$==='argumentCount'),branch=code[count+3];
 assert.equal(branch.$,'branch');assert.deepEqual(code[branch.yes],{$:'copy',destination:3,source:1});
 assert.deepEqual(code[branch.no],{$:'argument',destination:3,index:2});
 assert.equal(code.findIndex(row=>row.$==='ordinaryGet')>requireObject,true);
 assert.equal(code.filter(row=>row.$==='argumentListOpen').length,1);
 assert.equal(code.filter(row=>row.$==='argumentListAppend').length,1);
 assert.equal(code.some(row=>row.$==='iteratorOpen'||row.$==='ordinaryHas'),false);
 const invoke=code.find(row=>row.$==='argumentListCall');assert.deepEqual(invoke,{$:'argumentListCall',destination:9,slot:0,callee:1,receiver:3,mode:'construct'});
 for(const row of code)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(Number.isSafeInteger(target)&&target>=0&&target<code.length);
});

test('Reflect.construct catalog compilation uses actual identity and shared realm',()=>{
 const directory={$:'collectionCallableDirectory',realms:[8n],exoticKey:{$:'symbol',identity:9n},constructors:[],primitiveMethods:[{$:'primitiveMethodOwner',identity:7n,realm:8n,operation:'Reflect.construct'}]};
 const result=compileJavaScriptCollectionCallDirectory(directory);assert.equal(result.error,'');
 assert.deepEqual(result.program.functions,[compileJavaScriptReflectConstructionFunction(7n,8n)]);
 assert.notEqual(compileJavaScriptCollectionCallDirectory({...directory,primitiveMethods:[{...directory.primitiveMethods[0],realm:10n}]}).error,'');
});
