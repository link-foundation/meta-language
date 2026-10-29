// Deferred source/codec/emission/native controls; never execute while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {registerRustCallableProfile} from './helpers/rust-callable-profile.js';
import {readRustCopyClosureParameterForm} from '../src/translation/rust-copy-closure-rules.js';
import {readRustArrayLookupConsumerArity,testRustArrayLookupLazyFallback,readRustArrayLookupLazyFallbackError,planRustArrayLookupConsumer,testRustArrayLookupPredicateConsumer,readRustArrayLookupPredicateError,readRustArrayLookupCallableContextArity} from '../src/translation/rust-array-lookup-rules.js';
registerRustCallableProfile('rust-array-lookup-lazy');
test('lookup map_or retains two eager arguments and its selected conversion branch',()=>{
 assert.equal(readRustArrayLookupConsumerArity('map_or',true),2);
 assert.equal(readRustArrayLookupConsumerArity('map_or',false),-1);
 assert.equal(testRustArrayLookupLazyFallback('map_or'),false);
 assert.equal(testRustArrayLookupPredicateConsumer('map_or'),false);
 assert.deepEqual(planRustArrayLookupConsumer('map_or'),['condition','selected','fallback','if']);
});
test('lazy lookup retains its actual arity, callable ownership and branch plan',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/rust-array-lookup-lazy.lino',import.meta.url));
 assert.equal(readRustArrayLookupConsumerArity('unwrap_or_else',true),1);
 assert.equal(readRustArrayLookupConsumerArity('unwrap_or_else',false),-1);
 assert.equal(testRustArrayLookupLazyFallback('unwrap_or_else'),true);
 for(const consumer of ['unwrap_or','unwrap_or_default','is_some','is_none','unknown'])assert.equal(testRustArrayLookupLazyFallback(consumer),false);
 assert.deepEqual(planRustArrayLookupConsumer('unwrap_or_else'),['condition','selected','fallback','if']);
 for(const row of fixture.lazySignatures)assert.equal(readRustArrayLookupLazyFallbackError(row.kind,row.parameters,row.wrappers,row.knownResult),row.expected);
});
test('lookup predicates retain actual parameter/result owners and absent defaults',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/rust-array-lookup-lazy.lino',import.meta.url));
 for(const consumer of ['is_some_and','is_none_or']){
  assert.equal(readRustArrayLookupConsumerArity(consumer,true),1);
  assert.equal(readRustArrayLookupConsumerArity(consumer,false),-1);
  assert.equal(testRustArrayLookupPredicateConsumer(consumer),true);
  assert.equal(testRustArrayLookupLazyFallback(consumer),false);
  assert.deepEqual(planRustArrayLookupConsumer(consumer),['condition','predicate',consumer==='is_some_and'?'false':'true','if']);
 }
 for(const consumer of ['unwrap_or_else','unwrap_or','unwrap_or_default','is_some','is_none','unknown'])assert.equal(testRustArrayLookupPredicateConsumer(consumer),false);
 for(const row of fixture.predicateSignatures)assert.equal(readRustArrayLookupPredicateError(row.kind,row.parameters,row.wrappers,row.knownParameter,row.result,row.knownResult),row.expected);
});
test('lookup callable inference requires the actual receiver and element evidence',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/rust-array-lookup-lazy.lino',import.meta.url));
 for(const row of fixture.callableContexts)assert.equal(readRustArrayLookupCallableContextArity(row.consumer,row.receiver,row.element),row.expected);
 for(const row of fixture.closureParameterForms)assert.equal(readRustCopyClosureParameterForm(row.role),row.expected);
});
