// Deferred source/asset controls. No represented collection operation runs
// during interface construction; do not execute while drafting requirements.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createRustStandardCollectionSourceLibrary} from '../src/translation/rust-standard-collection-library.js';
import {readRustStandardCollectionTestAsset as actualAsset} from './support/rust-standard-collection-library.js';
const entryPath='js/src/translation/rust-standard-collection-value-packet.js';
const handlers={invalid:reason=>({k:'unit'}),resource:(state,reason)=>({k:'unit'})};
test('automatic interfaces retain complete actual source owners and packet transition bodies',()=>{
 const {text,receipt}=actualAsset(),library=createRustStandardCollectionSourceLibrary(text,receipt,handlers);
 assert.equal(library.entryPath,entryPath);assert.equal(library.sourceHandlers,handlers);assert.equal(library.functions.length,3);
 for(const binding of library.functions){assert.ok(library.program.declarations.get(binding.checkedName).body);assert.deepEqual(binding.sourcePath,['crate',...binding.checkedName.split('.')]);}
 const packet=library.program.declarations.get(library.packetType.checkedName);assert.equal(packet.ctors.length,3);assert.deepEqual(Object.keys(library.packetType.constructors),['normal','invalid','resource']);
 assert.throws(()=>createRustStandardCollectionSourceLibrary(text+'\n',receipt,handlers),/bytes differ/);
 assert.throws(()=>createRustStandardCollectionSourceLibrary(text,{...receipt,sourceDeclarationOwners:[]},handlers),/ownership differs/);
 assert.throws(()=>createRustStandardCollectionSourceLibrary(text,receipt,{invalid:handlers.invalid}),/continuations/);
 assert.throws(()=>createRustStandardCollectionSourceLibrary(text,receipt,handlers,{nodeLimit:1}),/node budget/);
 const changed=structuredClone(receipt);changed.sourceClosure.inputs[0].source+='\nexport const forged=1;';assert.throws(()=>createRustStandardCollectionSourceLibrary(text,changed,handlers),/closure differs|source|declaration/);
});
test('receipt declarations and invalid resource selections cannot certify bodies',()=>{
 for(const receipt of [null,{}, {model:'Complete source library asset receipt 1',schemaVersion:1,entryPath,referenceSemantics:false}])assert.throws(()=>createRustStandardCollectionSourceLibrary('',receipt,handlers),TypeError);
 for(const options of [{byteLimit:0},{nodeLimit:NaN},{referenceLimit:-1},{depthLimit:Infinity}])assert.throws(()=>createRustStandardCollectionSourceLibrary('',{},handlers,options),/positive safe integer/);
});
