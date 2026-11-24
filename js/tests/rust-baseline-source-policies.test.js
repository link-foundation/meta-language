// Deferred controls only. Do not import or execute until full drafting ends.
import assert from 'node:assert/strict';
import test from 'node:test';
import * as aliases from '../src/translation/rust-generic-type-alias-rules.js';
import * as sums from '../src/translation/rust-standard-sum-source-rules.js';
import {parameterPlans,substitutionPlans,expansionPlans} from './fixtures/rust-generic-type-alias-draft.mjs';
import {rustStandardSumSourceDecisions} from './fixtures/rust-standard-sum-source-rules.mjs';
import * as methods from '../src/translation/rust-standard-sum-method-rules.js';
import * as panic from '../src/translation/rust-panic-value-rules.js';
import * as pointers from '../src/translation/rust-function-pointer-rules.js';
import {readRustArrayLookupOptionDisposition} from '../src/translation/rust-array-lookup-option-rules.js';
import {rustStandardSumMethodDecisions} from './fixtures/rust-standard-sum-methods.mjs';
import {rustStandardOptionPanicMethodDecisions} from './fixtures/rust-standard-option-panic-methods.mjs';
import {rustArrayLookupOptionLegacyConsumers} from './fixtures/rust-array-lookup-options.mjs';

function selectVersionProfile(){
 const [major,minor]=process.versions.node.split('.').map(Number);
 assert.ok(major===24||major===22&&minor>=15,'uncovered policy-control Node '+process.versions.node);
}
test('generic alias actual-token parameter plans retain independent literal expectations',()=>{
 selectVersionProfile();
 for(const row of parameterPlans)assert.deepEqual(aliases.readRustGenericTypeAliasParameterPlan(row.kinds,row.values,row.raws,row.position),row.expected);
});
test('generic alias substitutions use resolved binding identities and exact argument arity',()=>{
 selectVersionProfile();
 for(const row of substitutionPlans)assert.deepEqual(aliases.readRustGenericTypeAliasSubstitutionPlan(row.parameters,row.argumentsProven,row.occurrences),row.expected);
});
test('generic alias recursion retains actual declaration ownership',()=>{
 selectVersionProfile();
 for(const row of expansionPlans)assert.equal(aliases.readRustGenericTypeAliasExpansionError(row.declaration,row.declarations,row.expansions),row.expected);
});
test('standard sum source decisions retain every independent literal control',()=>{
 selectVersionProfile();
 for(const row of rustStandardSumSourceDecisions){
  assert.ok(Object.hasOwn(sums,row.operation),'registered actual policy operation '+row.operation);
  assert.equal(typeof sums[row.operation],'function');
  assert.deepEqual(sums[row.operation](...row.arguments),row.expected,row.operation);
 }
});
test('standard sum methods and panic controls retain all literal decisions',()=>{
 selectVersionProfile();const operations={...methods,...panic};
 for(const row of [...rustStandardSumMethodDecisions,...rustStandardOptionPanicMethodDecisions]){
  assert.ok(Object.hasOwn(operations,row.operation),'actual shared operation '+row.operation);
  assert.deepEqual(operations[row.operation](...row.arguments),row.expected,row.operation);
 }
});
test('header collection never changes the final function-pointer domain checker',()=>{
 selectVersionProfile();
 assert.equal(pointers.readRustFunctionPointerHeaderTypeError(true,[false],false,true),'');
 assert.equal(pointers.readRustFunctionPointerHeaderTypeError(true,[false],false,false),'function pointer parameters require direct immutable Copy source types');
 assert.equal(pointers.readRustFunctionPointerTypeError(true,[false],false),'function pointer parameters require direct immutable Copy source types');
 for(const [kind,layers,header,expected]of [['function',0,true,true],['function',1,true,false],['function',0,false,false],['fixed',0,true,false]])assert.equal(pointers.acceptRustFunctionPointerHeaderCopy(kind,layers,header),expected);
});
test('every original copied lookup consumer retains the old source route',()=>{
 selectVersionProfile();
 for(const method of ['get','first','last']){
  for(const next of rustArrayLookupOptionLegacyConsumers)assert.equal(readRustArrayLookupOptionDisposition(method,true,true,next),'existing consumer');
  for(const next of ['','map','and_then'])assert.equal(readRustArrayLookupOptionDisposition(method,true,true,next),'owned scalar option');
  assert.equal(readRustArrayLookupOptionDisposition(method,true,false,''),'');
 }
});
