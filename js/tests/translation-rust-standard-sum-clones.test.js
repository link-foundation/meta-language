// Deferred controls; complete sources are consumed by the common matrix.
import assert from 'node:assert/strict';
import test from 'node:test';
import * as rules from '../src/translation/rust-owned-standard-sum-method-rules.js';
import {rustStandardSumCloneDecisions as decisions,rustStandardSumCloneOriginals as originals,rustStandardSumCloneRefusals as refusals} from './fixtures/rust-standard-sum-clones.mjs';
import {rustStandardSumIntegrationOriginals,rustStandardSumIntegrationRefusals} from './fixtures/rust-standard-sum-integration.mjs';
for(const [at,row]of decisions.entries())test('actual standard sum Clone context '+at,()=>{assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected);});
test('all ordinary clone inputs retain their complete source and ownership oracles',()=>{
 assert.equal(decisions.length,9);assert.equal(originals.length,5);assert.equal(refusals.length,3);
 for(const row of originals){const actual=rustStandardSumIntegrationOriginals.find(value=>value.name===row.name);assert.ok(actual);assert.equal(actual.source,row.source);assert.equal(actual.stdout,row.stdout);assert.equal(actual.sourceVersionRange,row.sourceVersionRange);}
 for(const row of refusals){const actual=rustStandardSumIntegrationRefusals.find(value=>value.name===row.name);assert.ok(actual);assert.equal(actual.source,row.source);assert.equal(actual.originalCompilerExpectation.outcome,row.originalCompilerOutcome);}
});
