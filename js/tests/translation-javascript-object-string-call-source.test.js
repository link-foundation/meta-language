// Deferred slot/dispatch controls; no execution during source drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptObjectStringFunction,compileJavaScriptArrayStringFunction} from '../src/translation/javascript-object-string-call-source.js';
import {beginJavaScriptObjectObservation} from '../src/translation/javascript-object-integrity.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),absent={$:'undefined'},tag={$:'symbol',identity:20n};
const roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)};
const text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});

test('Object string conversion keeps builtin slot tag separate from observable tag Get',()=>{
 const fn=compileJavaScriptObjectStringFunction(7n,8n,roots,tag),code=fn.instructions;
 const brand=code.findIndex(row=>row.$==='objectObservation'&&row.operation==='Object.builtinTag');const get=code.findIndex(row=>row.$==='ordinaryGet');assert.ok(brand>0&&get>brand);assert.equal(code[get].receiver,1);assert.equal(code[get-1].value,tag);
 assert.equal(compileJavaScriptObjectStringFunction(7n,8n,roots,absent),undefined);
 for(const row of code)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(target>=0&&target<code.length);
});

test('Array string conversion selects current callable join and otherwise actual Object owner',()=>{
 const owner=object(7n,true),fn=compileJavaScriptArrayStringFunction(9n,8n,roots,owner),code=fn.instructions,get=code.findIndex(row=>row.$==='ordinaryGet'),invoke=code.find(row=>row.$==='call');
 assert.equal(code.filter(row=>row.$==='ordinaryGet').length,1);assert.equal(code[get].receiver,1);assert.equal(invoke.receiver,1);assert.deepEqual(invoke.arguments,[]);assert.ok(code.some(row=>row.$==='constant'&&row.value===owner));
 assert.equal(compileJavaScriptArrayStringFunction(9n,8n,roots,object(7n)),undefined);
 for(const row of code)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(target>=0&&target<code.length);
});

test('builtin tags use actual slots rather than prototype resemblance or Symbol and BigInt slots',()=>{
 const row=(identity,fields={})=>({$:'ordinaryObject',identity,prototype:{$:'null'},callable:false,extensible:true,properties:[],...fields});
 const heap={$:'ordinaryHeap',symbols:[20n],objects:[row(1n),row(2n,{stringData:text('abc')}),row(3n,{primitiveData:{$:'number',value:7}}),row(4n,{primitiveData:{$:'boolean',value:true}}),row(5n,{primitiveData:{$:'integer',value:7n}}),row(6n,{primitiveData:tag}),row(7n,{callable:true}),row(8n,{regularExpression:true})]};
 for(const [identity,expected]of [[1n,'Object'],[2n,'String'],[3n,'Number'],[4n,'Boolean'],[5n,'Object'],[6n,'Object'],[7n,'Function'],[8n,'RegExp']]){const result=beginJavaScriptObjectObservation(heap,'Object.builtinTag',object(identity,identity===7n),absent);assert.equal(result.$,'objectIntegrityCompleted',result.reason);assert.deepEqual(result.value,text(expected));}
});

test('catalog requires actual tag key fallback callable and primitive realm roots',()=>{
 const base={$:'collectionCallableDirectory',realms:[8n],exoticKey:tag,constructors:[],primitiveMethods:[]};
 const owner={$:'primitiveMethodOwner',identity:7n,realm:8n,operation:'Object.prototype.toString',wrapperRoots:roots,tagKey:tag};assert.equal(compileJavaScriptCollectionCallDirectory({...base,primitiveMethods:[owner]}).error,'');assert.notEqual(compileJavaScriptCollectionCallDirectory({...base,primitiveMethods:[{...owner,tagKey:undefined}]}).error,'');
 const array={$:'primitiveMethodOwner',identity:9n,realm:8n,operation:'Array.toString',wrapperRoots:roots,objectString:object(7n,true)};assert.equal(compileJavaScriptCollectionCallDirectory({...base,primitiveMethods:[array]}).error,'');assert.notEqual(compileJavaScriptCollectionCallDirectory({...base,primitiveMethods:[{...array,objectString:absent}]}).error,'');
});
