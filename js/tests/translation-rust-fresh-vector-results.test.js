// Deferred ordinary-result controls; complete original products join the matrix.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {acceptRustKnownFreshVectorCall} from '../src/translation/rust-fresh-vector-result-rules.js';
import {assertRustFreshVectorResultCorrection} from './support/rust-fresh-vector-result-corrections.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-fresh-vector-results.lino',import.meta.url));
test('the exact historical default refusal now retains its real vector value',()=>assertRustFreshVectorResultCorrection(fixture.legacy));
for(const correction of fixture.corrections)test('complete original corrected runtime calls: '+correction.id,()=>{const cases=fixture.cases.filter(row=>row.legacyCorrection?.id===correction.id);assert.equal(cases.length,correction.calls.length);assert.deepEqual(cases.map(row=>row.legacyCorrection.call),correction.calls.map((_,index)=>index));assert.ok(cases.every(row=>row.source.startsWith(correction.source)));assertRustFreshVectorResultCorrection(correction);});
for(const row of fixture.decisions)test('known fresh source call: '+JSON.stringify(row),()=>assert.equal(acceptRustKnownFreshVectorCall(row.kind,row.model,row.wrappers,row.checked),row.expected));
