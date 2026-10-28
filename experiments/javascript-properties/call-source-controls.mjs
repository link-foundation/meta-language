// Deferred actual frontend -> shared compiler -> executor and original source.
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {lowerCheckedJavaScriptCallModule} from '../../js/src/translation/javascript-call-lowering.js';
import {beginJavaScriptCallMachine,stepJavaScriptCallMachine} from '../../js/src/translation/javascript-call-machine.js';
import {encodeJavaScriptCallSource,encodeJavaScriptCallSourceCompilation} from './call-source-codecs.mjs';
import {decodeJavaScriptSemanticValue,encodeJavaScriptSemanticValue,encodeJavaScriptCallProgram,encodeJavaScriptCallMachineTransition,encodeJavaScriptOrdinaryHeap} from './call-machine-codecs.mjs';
import {assertVersions} from './controls.mjs';
export {assertVersions};
export const callSourceFixture=readProjectData(new URL('../../parity/fixtures/javascript-call-source.lino',import.meta.url));
function registry(row){return {functions:row.functions.map((name,index)=>({name,identity:10n+BigInt(index),realm:1000n})),realms:[1000n],exoticKey:{$:'symbol',identity:18446744073709551615n}};}
function materialize(value){switch(value.$){case 'undefined':return undefined;case 'null':return null;case 'boolean':case 'number':case 'integer':return value.value;case 'text':return String.fromCharCode(...value.units);default:assert.fail('primitive source controls require independent primitive inputs');}}
function observePrimitive(value){if(value===undefined)return {$:'undefined'};if(value===null)return {$:'null'};switch(typeof value){case 'boolean':return {$:'boolean',value};case 'number':return encodeJavaScriptSemanticValue({$:'number',value});case 'bigint':return {$:'integer',value:String(value)};case 'string':return {$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))};default:assert.fail('original source control produced an unrepresented primitive');}}
export function observeCallSource(row){
 const actualRegistry=registry(row),checked=checkProgram(parseJavaScript(row.source)),lowered=lowerCheckedJavaScriptCallModule(checked,actualRegistry);
 const heap={$:'ordinaryHeap',objects:actualRegistry.functions.map(fn=>({$:'ordinaryObject',identity:fn.identity,prototype:{$:'null'},extensible:true,callable:true,properties:[]})),symbols:[actualRegistry.exoticKey.identity]};
 const arguments_=row.arguments.map(decodeJavaScriptSemanticValue),entry=actualRegistry.functions.find(fn=>fn.name===row.entry).identity;
 let current=beginJavaScriptCallMachine(lowered.program,heap,entry,{$:'undefined'},arguments_);const trace=[encodeJavaScriptCallMachineTransition(current)];
 for(let steps=0;current.$==='machineContinue';steps+=1){assert.ok(steps<4096,'every actual frontend invocation must reach its full terminal observation');current=stepJavaScriptCallMachine(lowered.program,current.machine);trace.push(encodeJavaScriptCallMachineTransition(current));}
 assert.equal(current.$,'machineCompleted');assert.deepEqual(encodeJavaScriptSemanticValue(current.value),row.expected);assert.deepEqual(encodeJavaScriptOrdinaryHeap(current.heap),encodeJavaScriptOrdinaryHeap(heap));
 const original=runInNewContext('(function(){"use strict";'+row.source.replaceAll('export function ','function ')+';return '+row.entry+';})()');
 assert.deepEqual(observePrimitive(Reflect.apply(original,undefined,arguments_.map(materialize))),row.expected,'independent original source behavior');
 return {...row,entry:String(entry),program:encodeJavaScriptCallProgram(lowered.program),heap:encodeJavaScriptOrdinaryHeap(heap),trace,
  sourceUnits:lowered.sourceUnits.map(unit=>({...unit,source:encodeJavaScriptCallSource(unit.source),compilation:encodeJavaScriptCallSourceCompilation(unit.compilation)}))};
}
export function observeCallSourceRefusal(row){const checked=checkProgram(parseJavaScript(row.source));assert.throws(()=>lowerCheckedJavaScriptCallModule(checked,registry(row)),/registered call source lowering/);}
