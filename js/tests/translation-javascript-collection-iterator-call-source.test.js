// Deferred source controls. No represented operation runs during drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileJavaScriptCollectionIteratorCallDirectory} from '../src/translation/javascript-collection-iterator-call-source.js';
import {orderJavaScriptCallCodeFunctions} from '../src/translation/javascript-call-code.js';
import {readJavaScriptCallProgramError} from '../src/translation/javascript-call-machine.js';
const object=identity=>({$:'object',identity,callable:false});
const owner=(identity,operation)=>({$:'collectionIteratorMethodOwner',identity,realm:1n,operation,iteratorPrototype:object(2n),arrayPrototype:object(3n),objectPrototype:object(4n)});
const directory=methods=>({$:'collectionIteratorCallableDirectory',realms:[1n],exoticKey:{$:'symbol',identity:5n},methods});
test('actual Set keys aliases one values owner and directories sort exact identities',()=>{
 const compiled=compileJavaScriptCollectionIteratorCallDirectory(directory([owner(20n,'Set.keys'),owner(9n,'MapIterator.next'),owner(20n,'Set.values'),owner(11n,'Iterator.self')]));
 assert.equal(compiled.error,'');assert.deepEqual(compiled.program.functions.map(row=>row.identity),[9n,11n,20n]);
 assert.equal(compiled.program.functions[2].instructions[1].iterationKind,'values');
 assert.equal(compiled.program.functions[0].instructions[2].$,'collectionIteratorResult');
 assert.equal(compiled.program.functions[1].receiverMode,'strict');
 const heap={$:'ordinaryHeap',symbols:[5n],objects:[2n,3n,4n,9n,11n,20n].map(identity=>({$:'ordinaryObject',identity,prototype:{$:'null'},extensible:true,callable:[9n,11n,20n].includes(identity),properties:[]}))};
 assert.equal(readJavaScriptCallProgramError(compiled.program,heap),'');
});
test('a source spelling cannot assign one real identity incompatible iterator semantics',()=>{
 for(const methods of [[owner(9n,'Map.keys'),owner(9n,'Map.values')],[owner(9n,'MapIterator.next'),owner(9n,'SetIterator.next')],[owner(9n,'Set.keys'),{...owner(9n,'Set.values'),iteratorPrototype:object(99n)}]])assert.notEqual(compileJavaScriptCollectionIteratorCallDirectory(directory(methods)).error,'');
});
test('next aliases retain the actual allocation prototypes and source realm',()=>{
 for(const replacement of [{arrayPrototype:object(99n)},{objectPrototype:object(99n)},{realm:2n}])assert.notEqual(compileJavaScriptCollectionIteratorCallDirectory(directory([owner(9n,'MapIterator.next'),{...owner(9n,'MapIterator.next'),...replacement}])).error,'');
 assert.notEqual(compileJavaScriptCollectionIteratorCallDirectory(directory([owner(9n,'Map.clear')])).error,'');
});

test('callable ordering is stable across uneven runs and compares full unsigned identities',()=>{
 const method=(identity,realm)=>({$:'callFunction',identity,realm,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[]});
 const low=method(9007199254740992n,1n), high=method(18446744073709551615n,1n);
 const first=method(9007199254740993n,1n), second=method(first.identity,2n);
 const smallest=method(1n,1n), beforeHigh=method(high.identity-1n,1n), another=method(6n,1n);
 const original=[high,first,low,second,beforeHigh,smallest,another];
 const input=Object.freeze(original.slice());
 const ordered=orderJavaScriptCallCodeFunctions(input);
 assert.deepEqual(ordered,[smallest,another,low,first,second,beforeHigh,high]);
 assert.equal(ordered[3],first);
 assert.equal(ordered[4],second);
 assert.deepEqual(input,original);
});
