// Deferred source controls. Complete originals join the common integration
// producer; this file retains every independent shared decision assertion.
import test from 'node:test';
import assert from 'node:assert/strict';
import * as rules from '../src/translation/rust-owned-standard-sum-observer-rules.js';
import {rustOwnedStandardSumObserverDecisions} from './fixtures/rust-owned-standard-sum-observers.mjs';
test('owned standard sum observers preserve nonconsuming constructor and source-state decisions',()=>{
 for(const row of rustOwnedStandardSumObserverDecisions){assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);}
});
