// Protected Object Call and directory controls; execute only after full draft.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptObjectCallFunction} from '../src/translation/javascript-object-call-source.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
const object=identity=>({$:'object',identity,callable:false});
const roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)};
test('Object Call retains all actual primitive roots and real object identity passthrough',()=>{
 const fn=compileJavaScriptObjectCallFunction(7n,8n,roots);assert.ok(fn);assert.equal(fn.instructions[1].operation,'test object value');assert.equal(fn.instructions.at(-1).source,1);assert.equal(fn.instructions.filter(row=>row.$==='primitiveWrapper').length,1);
 for(const row of fn.instructions)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(target>=0&&target<fn.instructions.length);
 for(const field of ['objectPrototype','stringPrototype','numberPrototype','booleanPrototype','integerPrototype','symbolPrototype'])assert.equal(compileJavaScriptObjectCallFunction(7n,8n,{...roots,[field]:{$:'null'}}),undefined);
});
test('Object Call compilation retains actual constructibility separately from its wrapper Call implementation',()=>{
 const owner={$:'primitiveMethodOwner',identity:7n,realm:8n,operation:'Object',constructable:true,wrapperRoots:roots},directory={$:'collectionCallableDirectory',realms:[8n],exoticKey:{$:'symbol',identity:9n},constructors:[],primitiveMethods:[owner]};
 assert.equal(compileJavaScriptCollectionCallDirectory(directory).error,'');assert.notEqual(compileJavaScriptCollectionCallDirectory({...directory,primitiveMethods:[{...owner,constructable:false}]}).error,'');assert.notEqual(compileJavaScriptCollectionCallDirectory({...directory,primitiveMethods:[{...owner,wrapperRoots:undefined}]}).error,'');
});
