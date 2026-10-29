// Deferred source/codec/emission/native controls; never execute while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {registerRustCallableProfile} from './helpers/rust-callable-profile.js';
import {readRustArrayLookupConsumerArity,testRustArrayLookupLazyFallback,readRustArrayLookupLazyFallbackError,planRustArrayLookupConsumer} from '../src/translation/rust-array-lookup-rules.js';
registerRustCallableProfile('rust-array-lookup-lazy');
test('lazy lookup retains its actual arity, callable ownership and branch plan',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/rust-array-lookup-lazy.lino',import.meta.url));
 assert.equal(readRustArrayLookupConsumerArity('unwrap_or_else',true),1);
 assert.equal(readRustArrayLookupConsumerArity('unwrap_or_else',false),-1);
 assert.equal(testRustArrayLookupLazyFallback('unwrap_or_else'),true);
 for(const consumer of ['unwrap_or','unwrap_or_default','is_some','is_none','unknown'])assert.equal(testRustArrayLookupLazyFallback(consumer),false);
 assert.deepEqual(planRustArrayLookupConsumer('unwrap_or_else'),['condition','selected','fallback','if']);
 for(const row of fixture.lazySignatures)assert.equal(readRustArrayLookupLazyFallbackError(row.kind,row.parameters,row.wrappers,row.knownResult),row.expected);
});
