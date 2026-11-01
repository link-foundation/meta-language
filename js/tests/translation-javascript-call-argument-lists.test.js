// Deferred argument List source controls. No execution on import.
import test from 'node:test';import assert from 'node:assert/strict';
import {openJavaScriptCallArgumentList,appendJavaScriptCallArgument,retainJavaScriptCallArgumentLists,readJavaScriptCallArgumentListsError} from '../src/translation/javascript-call-argument-lists.js';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
const heap={$:'ordinaryHeap',objects:[],symbols:[]},number=value=>({$:'number',value}),constant=value=>({$:'sourceConstant',value});
test('abstract arguments preserve snapshots without allocating language Objects',()=>{
 const first=openJavaScriptCallArgumentList(heap,[],3,1);assert.equal(first.$,'argumentListsUpdated');const appended=appendJavaScriptCallArgument(heap,first.lists,3,1,number(7));assert.equal(appended.$,'argumentListsUpdated');assert.deepEqual(first.lists[0].values,[]);assert.deepEqual(appended.lists[0].values,[number(7)]);assert.deepEqual(heap.objects,[]);assert.equal(openJavaScriptCallArgumentList(heap,appended.lists,3,1).$,'argumentListsInvalid');assert.equal(appendJavaScriptCallArgument(heap,appended.lists,4,1,number(9)).$,'argumentListsInvalid');
});
test('crossed handlers discard their own argument evaluation and preserve outer Lists',()=>{
 const first=openJavaScriptCallArgumentList(heap,[],5,0),nested=openJavaScriptCallArgumentList(heap,first.lists,9,2);assert.equal(nested.$,'argumentListsUpdated');assert.deepEqual(retainJavaScriptCallArgumentLists(nested.lists,1),first.lists);assert.equal(readJavaScriptCallArgumentListsError(heap,nested.lists,1).length>0,true);assert.equal(readJavaScriptCallArgumentListsError(heap,nested.lists,2),'');
});
test('source argument spread uses no extra Array and keeps callee selection before argument effects',()=>{
 const source={$:'sourceSnapshotCall',callee:{$:'sourceBinding',identity:1},receiver:constant({$:'undefined'}),arguments:[constant(number(1)),{$:'sourceArgumentSpread',value:{$:'sourceBinding',identity:2},iteratorKey:constant({$:'symbol',identity:99n}),realm:1n},constant(number(9))]},compiled=compileJavaScriptMutableCallSourceWithCaptures(source,[1,2],[]);assert.equal(compiled.error,'');const code=compiled.instructions,open=code.findIndex(row=>row.$==='argumentListOpen'),invoke=code.findIndex(row=>row.$==='argumentListCall'),protocols=code.filter(row=>row.$==='iteratorProtocol');assert.ok(open>0&&invoke>open);assert.equal(code.some(row=>row.$==='arrayAllocate'),false);assert.deepEqual(protocols.map(row=>row.operation),['acquire','step','close']);assert.equal(code.filter(row=>row.$==='argumentListAppend').length,3);assert.equal(code[invoke].slot,code[open].slot);assert.equal(code[invoke].mode,'call');
});
test('variadic base class allocates receiver only after every argument expression',()=>{
 const compiled=compileJavaScriptMutableCallSourceWithCaptures({$:'sourceVariadicCall',callee:{$:'sourceBinding',identity:1},receiver:{$:'sourceAllocate',prototype:constant({$:'null'})},arguments:[constant(number(7))],mode:'ordinary class',receiverAfterArguments:true},[1],[]);assert.equal(compiled.error,'');const code=compiled.instructions;assert.ok(code.findIndex(row=>row.$==='ordinaryAllocate')>code.findIndex(row=>row.$==='argumentListAppend'));assert.equal(code.find(row=>row.$==='argumentListCall').mode,'ordinary class');
});
