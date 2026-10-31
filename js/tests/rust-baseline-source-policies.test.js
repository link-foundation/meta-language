// Deferred controls only. Do not import or execute until full drafting ends.
import assert from 'node:assert/strict';
import test from 'node:test';
import * as aliases from '../src/translation/rust-generic-type-alias-rules.js';
import * as sums from '../src/translation/rust-standard-sum-source-rules.js';
import {parameterPlans,substitutionPlans,expansionPlans} from './fixtures/rust-generic-type-alias-draft.mjs';
import {rustStandardSumSourceDecisions} from './fixtures/rust-standard-sum-source-rules.mjs';

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
