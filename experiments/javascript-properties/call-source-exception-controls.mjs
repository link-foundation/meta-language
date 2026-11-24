// Deferred compilation, original execution and complete generated-machine trace.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {compileJavaScriptCallSource} from '../../js/src/translation/javascript-call-source.js';
import {decodeJavaScriptCallSource,encodeJavaScriptCallSource,encodeJavaScriptCallSourceCompilation} from './call-source-codecs.mjs';
import {observeCallMachine} from './call-machine-controls.mjs';
export const sourceExceptionFixture=readProjectData(new URL('../../parity/fixtures/javascript-call-source-exceptions.lino',import.meta.url));
export function observeCallSourceException(row){
 const source=decodeJavaScriptCallSource(row.expression);assert.deepEqual(encodeJavaScriptCallSource(source),row.expression);
 const compiled=compileJavaScriptCallSource(source,row.parameters);assert.equal(compiled.error,'');
 const undef={$:'undefined'},nil={$:'null'},entity=(identity,callable)=>({$:'ordinaryObject',identity,prototype:nil,extensible:true,callable,properties:[]});
 const program={$:'callProgram',realms:['1000'],exoticKey:{$:'symbol',identity:'18446744073709551615'},functions:[{$:'callFunction',identity:'10',realm:'1000',receiverMode:'strict',lexicalReceiver:undef,parameters:row.parameters.length,registers:compiled.registers,captures:[],instructions:encodeJavaScriptCallSourceCompilation(compiled).instructions},...row.otherFunctions]};
 const heap={$:'ordinaryHeap',objects:[entity('1',false),entity('10',true),entity('11',true)],symbols:['18446744073709551615']};
 const actual=observeCallMachine({...row,program,heap,entry:'10'});
 return {...actual,sourceUnits:[{name:row.name,source:row.expression,parameters:row.parameters,compilation:encodeJavaScriptCallSourceCompilation(compiled)}]};
}
export function observeCallSourceExceptionRefusal(row){
 const source=decodeJavaScriptCallSource(row.expression);assert.deepEqual(encodeJavaScriptCallSource(source),row.expression);
 const compiled=compileJavaScriptCallSource(source,row.parameters);
 assert.equal(compiled.error,row.reason);assert.deepEqual(compiled.instructions,[]);assert.equal(compiled.registers,0);
 return {...row,compilation:encodeJavaScriptCallSourceCompilation(compiled)};
}
