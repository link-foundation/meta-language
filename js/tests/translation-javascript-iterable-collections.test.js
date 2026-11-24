// Deferred controls: authoring these declarations executes no represented code.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptIterableCollectionConstructor,readJavaScriptCollectionConstructorKind} from '../src/translation/javascript-collection-call-source.js';
import {executeJavaScriptCallPrimitive} from '../src/translation/javascript-call-primitives.js';
const object=identity=>({$:'object',identity,callable:false});
function owner(kind){return {$:'collectionConstructorOwner',kind,identity:1n,realm:2n,prototype:object(3n),adder:{...object(4n),callable:true},iteratorKey:{$:'symbol',identity:5n},iteratorMethod:{...object(6n),callable:true},iteratorPrototype:object(7n),nextMethod:{...object(8n),callable:true},methods:[]};}
for(const kind of ['Map','Set'])test(kind+' selects one actual adder before acquisition and shares close ownership',()=>{
 const fn=compileJavaScriptIterableCollectionConstructor(owner(kind)),code=fn.instructions;
 assert.equal(readJavaScriptCollectionConstructorKind(fn),kind);assert.equal(code.some(row=>row.$==='collectionCopy'),false);
 const acquire=code.findIndex(row=>row.$==='iteratorProtocol'&&row.operation==='acquire'),protect=code.findIndex(row=>row.$==='pushFinally'),check=code.findIndex(row=>row.$==='primitive'&&row.operation==='require collection adder');
 assert.ok(check<acquire);assert.ok(acquire<protect);assert.equal(code[protect].target,code.findIndex(row=>row.$==='iteratorProtocol'&&row.operation==='close'));
 const call=code.find(row=>row.$==='call');assert.equal(call.callee,5);assert.equal(call.receiver,3);assert.deepEqual(call.arguments,kind==='Map'?[10,11]:[9]);
 const end=code.length-1;assert.deepEqual(code[end],{$:'return',source:3});assert.equal(code[5].yes,end);assert.equal(code[8].yes,end);assert.equal(code[5].no,6);assert.equal(code[8].no,9);
 assert.deepEqual(code.filter(row=>row.$==='iteratorProtocol').map(row=>row.operation),['acquire','step','close']);
 if(kind==='Map'){const entry=code.findIndex(row=>row.$==='primitive'&&row.operation==='require Map entry object');const reads=code.slice(entry).filter(row=>row.$==='ordinaryGet');assert.deepEqual(reads.map(row=>row.destination),[10,11]);assert.equal(reads.every(row=>row.object===9&&row.receiver===9),true);}
});
test('collection guards classify callable adders and Object entries without invoking them',()=>{
 for(const value of [{$:'undefined'},{$:'null'},{$:'number',value:7},object(1n)])assert.equal(executeJavaScriptCallPrimitive('require collection adder',[value]).$,'primitiveError');
 const callable={...object(1n),callable:true};assert.deepEqual(executeJavaScriptCallPrimitive('require collection adder',[callable]),{$:'primitiveValue',value:callable});
 for(const value of [object(1n),callable])assert.deepEqual(executeJavaScriptCallPrimitive('require Map entry object',[value]),{$:'primitiveValue',value});
 for(const value of [{$:'undefined'},{$:'null'},{$:'text',units:[97,98]},{$:'number',value:7}])assert.equal(executeJavaScriptCallPrimitive('require Map entry object',[value]).errorClass,'TypeError');
});
