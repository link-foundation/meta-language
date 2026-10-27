import {registerRustCallableProfile} from './helpers/rust-callable-profile.js';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readRustScalarValueType} from '../src/translation/rust-scalar-source-interface-rules.js';
import * as rules from '../src/translation/rust-callable-owner-rules.js';
// Deferred actual source/codec/product controls. Never execute while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-callable-owner.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
test('every recursive callable owner contract join equality and resource premise stays explicit',()=>{
 for(const row of fixture.contractErrors)assert.equal(rules.readRustCallableOwnerError(row.actual,row.expected),row.expectedError);
 for(const row of fixture.equalities)assert.equal(rules.acceptRustCallableOwnerDescriptorsEqual(row.left,row.right),row.expected);
 for(const row of fixture.joins)assert.equal(rules.readRustCallableOwnerJoinError(row.rows),row.expectedError);
 for(const row of fixture.limits)assert.equal(rules.readRustCallableOwnerProjectionLimit(row.name),row.expected);
 for(const row of fixture.oversized){const descriptor=Array(row.size).fill('');assert.equal(rules.readRustCallableOwnerError(descriptor,[]),row.expectedError);assert.equal(rules.readRustCallableOwnerJoinError(descriptor),row.expectedError);assert.equal(rules.acceptRustCallableOwnerDescriptorsEqual(descriptor,descriptor),false);}
});
test('the corrected emitter scope retains the complete initial draft and every original source/type input',()=>{
 assert.equal(readRustScalarValueType('function',0,false),'');
 for(const correction of fixture.assertionCorrections){const path=new URL('../../'+correction.retainedDraft,import.meta.url),bytes=readFileSync(path),prior=readProjectData(path);assert.equal(bytes.length,correction.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),correction.sha256);assert.ok(prior.cases.length<=fixture.cases.length);for(let at=0;at<prior.cases.length;at+=1){const before=prior.cases[at],after=fixture.cases[at];for(const field of ['name','module','source','expected','resultType'])assert.deepEqual(after[field],before[field]);for(const check of before.checks)assert.ok(after.checks.includes(check));}}
});
registerRustCallableProfile('rust-callable-owner');
