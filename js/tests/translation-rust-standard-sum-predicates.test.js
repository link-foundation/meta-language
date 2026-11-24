// Deferred literal policy controls; actual source rows run through the common
// frontend/CST/typed/text/binary/four-target integration matrix when enabled.
import test from 'node:test';
import assert from 'node:assert/strict';
import * as rules from '../src/translation/rust-standard-sum-predicate-rules.js';
import {rustStandardSumPredicateDecisions} from './fixtures/rust-standard-sum-predicates.mjs';
test('all predicate and multi-callback source decisions retain independent literal oracles',()=>{
 for(const row of rustStandardSumPredicateDecisions){assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);}
});
