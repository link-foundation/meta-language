// Deferred shared policy controls; originals are common frontend/target inputs.
import test from 'node:test';
import assert from 'node:assert/strict';
import * as rules from '../src/translation/rust-standard-sum-guard-rules.js';
import * as bindingRules from '../src/translation/rust-owned-binding-rules.js';
import {rustStandardSumGuardDecisions,rustStandardSumGuardRegistrationDecisions} from './fixtures/rust-standard-sum-guards.mjs';
test('owned sum guards separate readonly preview from actual body transfers',()=>{
 for(const row of rustStandardSumGuardDecisions){assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);}
 for(const row of rustStandardSumGuardRegistrationDecisions){assert.equal(typeof bindingRules[row.operation],'function');assert.deepEqual(bindingRules[row.operation](...row.arguments),row.expected,row.operation);}
});
