// Deferred routing controls; end-to-end originals feed the complete standard
// sum integration matrix. None executes during source authoring.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readRustArrayLookupOption} from '../src/translation/rust-array-lookup-options.js';
import {readRustArrayLookupOptionDisposition,readRustArrayLookupOptionCaptureRoles,readRustArrayLookupOptionOwner,readRustArrayLookupOptionVariant} from '../src/translation/rust-array-lookup-option-rules.js';
import {rustArrayLookupOptionLegacyConsumers as consumers} from './fixtures/rust-array-lookup-options.mjs';

test('every old copied immediate consumer retains its route without source or cursor mutation',()=>{
 for(const method of ['get','first','last'])for(const consumer of consumers){
  assert.equal(readRustArrayLookupOptionDisposition(method,true,true,consumer),'existing consumer');
  const values=['.','copied','(',')','.',consumer,'('],tokens=values.map(value=>({value,kind:['copied',consumer].includes(value)?'identifier':'punct'}));
  const cursor={index:0,is:(value,offset=0)=>tokens[offset]?.value===value,peek:(offset=0)=>tokens[offset]??{value:'',kind:'eof'}};
  const parser={cursor,ordinaryCompletionContext:()=>true,get origins(){throw new Error('old immediate consumers must not read new source facts');}};
  assert.equal(readRustArrayLookupOption(parser,{},{value:method},[],{start:0,end:1},[]),null);
  assert.equal(cursor.index,0,'old parser still owns every suffix token');
 }
});
test('stored scalar result source decisions retain copied-only ownership and actual capture order',()=>{
 for(const method of ['get','first','last'])for(const following of ['','map','and_then'])assert.equal(readRustArrayLookupOptionDisposition(method,true,true,following),'owned scalar option');
 assert.equal(readRustArrayLookupOptionDisposition('get',false,true,''),'');
 assert.equal(readRustArrayLookupOptionDisposition('get',true,false,''),'');
 assert.equal(readRustArrayLookupOptionDisposition('unknown',true,true,''),'');
 assert.deepEqual(readRustArrayLookupOptionCaptureRoles('get'),['receiver','index']);
 assert.deepEqual(readRustArrayLookupOptionCaptureRoles('first'),['receiver']);
 assert.deepEqual(readRustArrayLookupOptionCaptureRoles('last'),['receiver']);
 assert.deepEqual(readRustArrayLookupOptionCaptureRoles('unknown'),[]);
 assert.equal(readRustArrayLookupOptionOwner(),'std::option::Option');
 assert.equal(readRustArrayLookupOptionVariant(true),'Some');assert.equal(readRustArrayLookupOptionVariant(false),'None');
});
