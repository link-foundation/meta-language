// Deferred independent controls. The shared full-source matrix runs the
// corresponding original compiler, frontend, codecs and four emitters later.
import assert from 'node:assert/strict';
import test from 'node:test';
import * as rules from '../src/translation/rust-standard-sum-source-rules.js';
import {rustStandardSumOwnedPropagationDecisions as decisions,rustStandardSumOwnedPropagationOriginals as originals,rustStandardSumOwnedPropagationRefusals as refusals,rustStandardSumOwnedPropagationPreviousBoundaries as history} from './fixtures/rust-standard-sum-owned-propagation.mjs';
import {rustStandardSumIntegrationOriginals,rustStandardSumIntegrationRefusals} from './fixtures/rust-standard-sum-integration.mjs';
for(const [at,row]of decisions.entries())test('owned residual context '+at,()=>{assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected);});
test('every owned residual source and compiler contract joins the complete shared matrix',()=>{
 assert.equal(decisions.length,15);assert.equal(originals.length,7);assert.equal(refusals.length,4);assert.equal(history.length,1);
 assert.equal(history[0].previousCollection,'rustStandardSumOwnedPropagationRefusals');assert.equal(history[0].previousIndex,3);assert.equal(history[0].originalCompilerOutcome,'accept');assert.equal(originals.filter(row=>row.source===history[0].row.source).length,1);assert.equal(refusals.filter(row=>row.source===history[0].row.source).length,0);
 for(const row of originals){const actual=rustStandardSumIntegrationOriginals.find(value=>value.name===row.name);assert.ok(actual);assert.equal(actual.source,row.source);assert.equal(actual.stdout,row.stdout);assert.equal(actual.sourceVersionRange,row.sourceVersionRange);}
 for(const row of refusals){const actual=rustStandardSumIntegrationRefusals.find(value=>value.name===row.name);assert.ok(actual);assert.equal(actual.source,row.source);assert.equal(actual.originalCompilerExpectation.outcome,row.originalCompilerOutcome);}
});
