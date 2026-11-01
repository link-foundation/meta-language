// Deferred controls. Complete originals join the actual frontend/target suite.
import test from 'node:test';
import assert from 'node:assert/strict';
import * as rules from '../src/translation/rust-standard-sum-guard-rules.js';
import {rustStandardSumGuardAlternativeDecisions} from './fixtures/rust-standard-sum-guard-alternatives.mjs';
import {rustStandardSumGuardOriginals,rustStandardSumGuardRefusals,rustStandardSumGuardPreviousBoundaries} from './fixtures/rust-standard-sum-guards.mjs';
test('expanded alternatives retain the complete former boundary and source coordinate',()=>{
 const previous=rustStandardSumGuardPreviousBoundaries[0];assert.equal(previous.previousCollection,'rustStandardSumGuardRefusals');assert.equal(previous.previousIndex,5);
 assert.deepEqual(previous.previousDiagnosticKinds,['syntax','type','unsupported']);assert.equal(previous.row.reasonIncludes,'alternatives');
 const current=rustStandardSumGuardOriginals.filter(row=>row.name===previous.row.name);assert.equal(current.length,1);assert.equal(current[0].source,previous.row.source);
 assert.equal(current[0].originalCompilerExpectation.outcome,'accept');assert.equal(current[0].stdout,'');
 assert.ok(!rustStandardSumGuardRefusals.some(row=>row.source===previous.row.source));
});
test('guarded owned alternatives require actual owner and root agreement before body transfers',()=>{
 for(const row of rustStandardSumGuardAlternativeDecisions){assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);}
});
