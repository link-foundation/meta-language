// All behavior is inside deferred test callbacks; authoring does not execute it.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptStringConversionFunction,constructJavaScriptSymbolDescriptiveText} from '../src/translation/javascript-string-conversion-call-source.js';
import {beginJavaScriptCallMachine,stepJavaScriptCallMachine,resumeJavaScriptCallSymbolDescription} from '../src/translation/javascript-call-machine.js';
import {beginJavaScriptObjectObservation} from '../src/translation/javascript-object-integrity.js';
const text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))});
test('Symbol descriptive strings retain absent empty and exact UTF16 descriptions',()=>{
 for(const [description,expected] of [[{$:'undefined'},'Symbol()'],[text(''),'Symbol()'],[text('Symbol.iterator'),'Symbol(Symbol.iterator)'],[{$:'text',units:[55296]},'Symbol('+String.fromCharCode(55296)+')']])assert.deepEqual(constructJavaScriptSymbolDescriptiveText(description).value,text(expected));
 assert.equal(constructJavaScriptSymbolDescriptiveText({$:'null'}).$,'primitiveInvalid');assert.equal(constructJavaScriptSymbolDescriptiveText({$:'sourceText',profile:'inspectable UTF16 source text 1',text:'x'.repeat(1048569)}).$,'primitiveResourceLimit');
});
test('Symbol metadata suspension authenticates the actual instruction Symbol and optional description',()=>{
 const heap={$:'ordinaryHeap',objects:[],symbols:[9n]},fn={$:'callFunction',identity:1n,realm:2n,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:1,registers:3,captures:[],instructions:[{$:'symbolDescription',destination:2,input:1},{$:'return',source:2}]},program={$:'callProgram',functions:[fn],realms:[2n],exoticKey:{$:'symbol',identity:9n}};
 let result=beginJavaScriptCallMachine(program,heap,1n,{$:'undefined'},[{$:'symbol',identity:9n}]);assert.equal(result.$,'machineContinue');result=stepJavaScriptCallMachine(program,result.machine);assert.equal(result.$,'machineContinue');const waiting=result.machine,request=stepJavaScriptCallMachine(program,waiting);assert.equal(request.$,'machineNeedSymbolDescription');assert.deepEqual(request.symbol,{$:'symbol',identity:9n});
 for(const description of [{$:'undefined'},text('')]){const resumed=resumeJavaScriptCallSymbolDescription(program,waiting,description);assert.equal(resumed.$,'machineContinue');const completed=stepJavaScriptCallMachine(program,resumed.machine);assert.equal(completed.$,'machineCompleted');assert.deepEqual(completed.value,description);}
 assert.equal(resumeJavaScriptCallSymbolDescription(program,waiting,{$:'number',value:7}).$,'machineInvalid');
 const forged={...waiting,frames:waiting.frames.map(frame=>({...frame,pending:{...frame.pending,symbol:{$:'symbol',identity:8n}}}))};assert.equal(resumeJavaScriptCallSymbolDescription(program,forged,text('forged')).$,'machineInvalid');
});
test('String and Symbol Call bodies retain complete branches and independent metadata operations',()=>{
 for(const operation of ['String','Symbol.toString','Symbol.valueOf','Symbol.toPrimitive','Symbol.description']){const fn=compileJavaScriptStringConversionFunction(1n,2n,operation);assert.ok(fn);assert.equal(fn.instructions.some(row=>row.$==='functionName'),false);for(const row of fn.instructions)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(target>=0&&target<fn.instructions.length);}
});
test('Symbol value extraction requires its own actual Symbol data slot',()=>{
 const heap={$:'ordinaryHeap',symbols:[9n],objects:[{$:'ordinaryObject',identity:1n,prototype:{$:'null'},extensible:true,callable:false,properties:[],primitiveData:{$:'symbol',identity:9n}},{$:'ordinaryObject',identity:2n,prototype:{$:'object',identity:1n,callable:false},extensible:true,callable:false,properties:[]}]};
 const result=beginJavaScriptObjectObservation(heap,'Symbol.valueOf',{$:'object',identity:1n,callable:false},{$:'undefined'});assert.equal(result.$,'objectIntegrityCompleted');assert.deepEqual(result.value,{$:'symbol',identity:9n});
 assert.equal(beginJavaScriptObjectObservation(heap,'Symbol.valueOf',{$:'object',identity:2n,callable:false},{$:'undefined'}).$,'objectIntegrityError');
});
