// Deferred source/VM integration controls, never executed during drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptArraySliceFunction} from '../src/translation/javascript-array-slice-call-source.js';
import {readJavaScriptCallCodeRegisters} from '../src/translation/javascript-call-code.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
const object=(identity,callable=false)=>({$:'object',identity,callable});
const roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)};
const species={$:'arraySpeciesDirectory',realm:8n,arrayConstructor:object(9n,true),arrayPrototype:object(10n),speciesKey:{$:'symbol',identity:11n},constructors:[{$:'arrayConstructorFact',identity:9n,realm:8n,constructable:true,intrinsicArray:true,intrinsicKind:'Array'}]};
test('slice boxes into an explicit operand and retains actual species authority',()=>{
 const fn=compileJavaScriptArraySliceFunction(7n,8n,roots,species),selected=fn.instructions.find(row=>row.$==='arraySlice');assert.deepEqual(selected,{$:'arraySlice',destination:6,input:1,species});assert.deepEqual(readJavaScriptCallCodeRegisters(selected),[6,1]);assert.equal(fn.instructions[0].operation,'require object coercible');assert.equal(fn.instructions.some(row=>row.destination===0),false);
 assert.equal(compileJavaScriptArraySliceFunction(7n,8n,undefined,species),undefined);assert.equal(compileJavaScriptArraySliceFunction(7n,8n,roots,undefined),undefined);assert.equal(compileJavaScriptArraySliceFunction(7n,8n,roots,{...species,realm:12n}),undefined);
});
test('slice protected catalog compilation preserves mandatory roots and species',()=>{
 const owner={$:'primitiveMethodOwner',identity:7n,realm:8n,operation:'Array.slice',wrapperRoots:roots,species};const directory={$:'collectionCallableDirectory',realms:[8n],exoticKey:{$:'symbol',identity:12n},constructors:[],primitiveMethods:[owner]};assert.equal(compileJavaScriptCollectionCallDirectory(directory).error,'');
 for(const changed of [{wrapperRoots:undefined},{species:undefined},{species:{...species,realm:12n}}])assert.notEqual(compileJavaScriptCollectionCallDirectory({...directory,primitiveMethods:[{...owner,...changed}]}).error,'');
});
