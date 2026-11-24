// Deferred controls; complete source rows join the common actual integration
// runner after drafting. No source program runs during this authoring phase.
import test from 'node:test';
import assert from 'node:assert/strict';
import * as rules from '../src/translation/rust-standard-sum-combinator-rules.js';
import {rustStandardSumCombinatorDecisions} from './fixtures/rust-standard-sum-combinators.mjs';
test('standard sum combinators retain every independent eager/lazy constructor decision',()=>{
 for(const row of rustStandardSumCombinatorDecisions){assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);}
});
