// Deferred controls; do not execute during bulk source drafting. Complete
// originals also join the shared native/frontend/codec/four-emitter matrix.
import assert from 'node:assert/strict';
import test from 'node:test';
import * as rules from '../src/translation/rust-owned-standard-sum-method-rules.js';
import {rustStandardDefaultDecisions,rustStandardDefaultOriginals,rustStandardDefaultRefusals} from './fixtures/rust-standard-default-values.mjs';
import {rustStandardSumIntegrationOriginals,rustStandardSumIntegrationRefusals} from './fixtures/rust-standard-sum-integration.mjs';
for(const [at,row]of rustStandardDefaultDecisions.entries())test('actual standard default policy '+at,()=>{
 assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected);
});
test('every complete default original and refusal stays attached to the shared actual matrix',()=>{
 assert.equal(rustStandardDefaultDecisions.length,26);assert.equal(rustStandardDefaultOriginals.length,8);assert.equal(rustStandardDefaultRefusals.length,5);
 for(const row of rustStandardDefaultOriginals){const actual=rustStandardSumIntegrationOriginals.find(value=>value.name===row.name);assert.ok(actual);assert.equal(actual.source,row.source);assert.equal(actual.stdout,row.stdout);assert.equal(actual.sourceVersionRange,row.sourceVersionRange);}
 for(const row of rustStandardDefaultRefusals){const actual=rustStandardSumIntegrationRefusals.find(value=>value.name===row.name);assert.ok(actual);assert.equal(actual.source,row.source);assert.equal(actual.originalCompilerExpectation.outcome,row.originalCompilerOutcome);}
});
