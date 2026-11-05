// Source-only controls; no execution during the bulk draft phase.
import test from 'node:test';
import assert from 'node:assert/strict';
import * as rules from '../src/translation/javascript-source-text-rules.js';
import * as semantics from '../src/translation/javascript-semantic-values.js';
import * as primitives from '../src/translation/javascript-call-primitives.js';
import * as boolean from '../src/translation/javascript-descriptor-conversion.js';
import {readJavaScriptCallProgramError,beginJavaScriptCallMachine,runJavaScriptCallMachine} from '../src/translation/javascript-call-machine.js';
import {createJavaScriptPropertyDescriptor} from '../src/translation/javascript-property-descriptors.js';
import {planJavaScriptOrdinaryGet,readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryOwnKeys} from '../src/translation/javascript-ordinary-heap.js';
import {structuredDataToLino,readStructuredDataLino,encodeStructuredDataLinks,decodeStructuredDataLinks} from '../src/structured-data-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as codecs from '../../experiments/javascript-properties/error-class-source-codecs.mjs';
import {javascriptSourceTextProfileDecisions as profile,javascriptSourceTextSemanticDecisions as values,javascriptSourceTextBooleanDecisions as booleans,javascriptSourceTextPrimitiveDecisions as operations,javascriptSourceTextLargeControls as large} from './fixtures/javascript-source-text.mjs';
for(const [name,module,rows]of [['profile',rules,profile],['semantic',semantics,values],['truth',boolean,booleans],['primitive',primitives,operations]])test('source text '+name+' exact decisions',()=>{for(const row of rows)assert.deepEqual(module[row.operation](...row.arguments),row.expected,row.operation);});
test('source text exact UTF16 codecs and Links preserve unpaired surrogates',()=>{
 const value=semantics.constructJavaScriptSourceText('\ud800x\udfff'),wire=codecs.encodeJavaScriptSemanticValue(value),packet=encodeStructuredDataLinks(wire);
 for(const restored of [wire,readStructuredDataLino(structuredDataToLino(wire)),decodeStructuredDataLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),packet.root)])assert.deepEqual(codecs.decodeJavaScriptSemanticValue(restored),value);
 for(const text of [null,[],{},0]){assert.notEqual(semantics.readJavaScriptSemanticValueError({$:'sourceText',profile:'inspectable UTF16 source text 1',text}),'');assert.throws(()=>codecs.decodeJavaScriptSemanticValue({$:'sourceText',profile:'inspectable UTF16 source text 1',text}));}
});
test('compact keys participate in actual ordinary lookup and property ordering',()=>{
 const descriptor=createJavaScriptPropertyDescriptor('data',true,true);descriptor.writable=true;descriptor.value=semantics.constructJavaScriptSourceText('value');
 const key=semantics.constructJavaScriptSourceText('1'),heap={$:'ordinaryHeap',symbols:[],objects:[{$:'ordinaryObject',identity:1n,prototype:{$:'null'},extensible:true,callable:false,properties:[{$:'ordinaryProperty',key,descriptor}]}]};
 assert.equal(readJavaScriptOrdinaryHeapError(heap),'');assert.deepEqual(planJavaScriptOrdinaryGet(heap,1n,{$:'text',units:[49]},{$:'object',identity:1n,callable:false}).value,descriptor.value);assert.deepEqual(readJavaScriptOrdinaryOwnKeys(heap,1n).keys,[key]);
});
test('compact strings remain semantic operands across portable call frames',()=>{
 const value=semantics.constructJavaScriptSourceText('\ud800text'),program={$:'callProgram',realms:[1n],exoticKey:{$:'symbol',identity:3n},functions:[{$:'callFunction',identity:1n,realm:1n,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:1,registers:2,captures:[],instructions:[{$:'argument',destination:1,index:0},{$:'return',source:1}]}]},heap={$:'ordinaryHeap',symbols:[3n],objects:[{$:'ordinaryObject',identity:1n,prototype:{$:'null'},extensible:true,callable:true,properties:[]}]};
 assert.equal(readJavaScriptCallProgramError(program,heap),'');const begin=beginJavaScriptCallMachine(program,heap,1n,{$:'undefined'},[value]);assert.equal(begin.$,'machineContinue');const completed=runJavaScriptCallMachine(program,begin.machine,32,[]).transition;assert.equal(completed.$,'machineCompleted');assert.deepEqual(completed.value,value);
});
test('retained old bounds and actual large compact resource controls',()=>{
 assert.equal(process.env.CI,'true','large compact text controls require the CI resource profile');
 const old=large[0],oldValue={$:'text',units:Array.from({length:old.count},()=>old.unit)};assert.equal(semantics.readJavaScriptSemanticValueError(oldValue),old.expectedOldError);assert.equal(semantics.readJavaScriptSemanticValueError(semantics.constructJavaScriptSourceText(String.fromCharCode(old.unit).repeat(old.count))),old.expectedSuccessorError);
 const row=large[1],value=semantics.constructJavaScriptSourceText(row.text.repeat(row.count));assert.equal(primitives.executeJavaScriptCallPrimitive('read text length',[value]).value.value,row.expectedLength);assert.equal(semantics.readJavaScriptSemanticTextString(primitives.executeJavaScriptCallPrimitive('read semantic type',[value]).value),row.expectedType);assert.equal(primitives.executeJavaScriptCallPrimitive('convert primitive text',[value]).$,row.expectedConversion);assert.equal(primitives.executeJavaScriptCallPrimitive('slice text',[value,{$:'number',value:0},{$:'number',value:1}]).$,row.expectedUnsupportedSlice);
 const maximum=large[2];assert.equal(semantics.readJavaScriptSemanticValueError(semantics.constructJavaScriptSourceText(maximum.text.repeat(maximum.count))),maximum.expectedError);assert.throws(()=>semantics.constructJavaScriptSourceText(maximum.text.repeat(maximum.overflow)),error=>error.message===maximum.expectedOverflowError);
});
