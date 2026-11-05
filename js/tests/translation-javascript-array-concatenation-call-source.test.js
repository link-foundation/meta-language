// Deferred compiler/catalog controls. No source behavior executes on import.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptArrayConcatenationFunction} from '../src/translation/javascript-array-concatenation-call-source.js';
import {readJavaScriptCallCodeRegisters} from '../src/translation/javascript-call-code.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
const object=(identity,callable=false)=>({$:'object',identity,callable});
const roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)};
const spreadableKey={$:'symbol',identity:12n},species={$:'arraySpeciesDirectory',realm:8n,arrayConstructor:object(9n,true),arrayPrototype:object(10n),speciesKey:{$:'symbol',identity:11n},constructors:[{$:'arrayConstructorFact',identity:9n,realm:8n,constructable:true,intrinsicArray:true,intrinsicKind:'Array'}]};
test('concatenation keeps boxed receiver and spreadability Symbol as actual operands',()=>{
 const fn=compileJavaScriptArrayConcatenationFunction(7n,8n,roots,species,spreadableKey),code=fn.instructions,selected=code.find(row=>row.$==='arrayConcatenation');assert.deepEqual(selected,{$:'arrayConcatenation',destination:7,input:1,spreadableKey:6,species});assert.deepEqual(readJavaScriptCallCodeRegisters(selected),[7,1,6]);assert.equal(code[0].operation,'require object coercible');assert.equal(code.some(row=>row.destination===0),false);assert.ok(code.some(row=>row.$==='constant'&&row.destination===6&&row.value===spreadableKey));
 assert.equal(compileJavaScriptArrayConcatenationFunction(7n,8n,roots,species,undefined),undefined);assert.equal(compileJavaScriptArrayConcatenationFunction(7n,8n,undefined,species,spreadableKey),undefined);
});
test('concatenation protected owner requires actual roots species and well-known Symbol',()=>{
 const owner={$:'primitiveMethodOwner',identity:7n,realm:8n,operation:'Array.concat',wrapperRoots:roots,species,spreadableKey},directory={$:'collectionCallableDirectory',realms:[8n],exoticKey:{$:'symbol',identity:13n},constructors:[],primitiveMethods:[owner]};assert.equal(compileJavaScriptCollectionCallDirectory(directory).error,'');
 for(const changed of [{wrapperRoots:undefined},{species:undefined},{spreadableKey:undefined},{spreadableKey:{$:'text',units:[]}}])assert.notEqual(compileJavaScriptCollectionCallDirectory({...directory,primitiveMethods:[{...owner,...changed}]}).error,'');
});
