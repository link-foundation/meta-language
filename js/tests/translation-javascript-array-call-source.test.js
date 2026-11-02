// Deferred Array Call linkage controls; no execution during source drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileJavaScriptArrayCallFunction} from '../src/translation/javascript-array-call-source.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';

test('Array Call uses its actual intrinsic owner for target and newTarget independently of thisArgument',()=>{
 const fn=compileJavaScriptArrayCallFunction(7n,8n),code=fn.instructions;
 assert.equal(fn.identity,7n);assert.equal(fn.realm,8n);
 assert.deepEqual(code[0],{$:'constant',destination:1,value:{$:'object',identity:7n,callable:true}});
 const invoke=code.find(row=>row.$==='argumentListCall');assert.equal(invoke.mode,'construct');assert.equal(invoke.callee,1);assert.equal(invoke.receiver,1);
 assert.equal(code.some(row=>row.$==='copy'&&row.source===0),false);
 assert.equal(code.filter(row=>row.$==='argumentListAppend').length,1);
 assert.equal(code.find(row=>row.$==='argumentCount').destination,2);
 for(const row of code)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(Number.isSafeInteger(target)&&target>=0&&target<code.length);
});

test('Array callable owner must retain actual constructibility independently of its Call body',()=>{
 const owner={$:'primitiveMethodOwner',identity:7n,realm:8n,operation:'Array',constructable:true};
 const directory={$:'collectionCallableDirectory',realms:[8n],exoticKey:{$:'symbol',identity:9n},constructors:[],primitiveMethods:[owner]};
 const compiled=compileJavaScriptCollectionCallDirectory(directory);assert.equal(compiled.error,'');assert.deepEqual(compiled.program.functions,[compileJavaScriptArrayCallFunction(7n,8n)]);
 for(const constructable of [false,undefined])assert.notEqual(compileJavaScriptCollectionCallDirectory({...directory,primitiveMethods:[{...owner,constructable}]}).error,'');
});
