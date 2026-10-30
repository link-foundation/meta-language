// Future source-boundary controls only; do not execute before the complete draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-optional-calls.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
test('every corrected original refusal keeps its exact positive input and history',()=>{
 const calls=readProjectData(new URL('../../parity/fixtures/javascript-optional-property-calls.lino',import.meta.url));
 const chains=readProjectData(new URL('../../parity/fixtures/javascript-optional-chains.lino',import.meta.url));
 for(const row of fixture.originalRefusals)assert.ok([...fixture.cases,...calls.cases,...chains.cases].some(value=>value.source===row.source),row.name);
 for(const path of ['js/scripts/generate-javascript-optional-call-fixtures.mjs','parity/fixtures/javascript-optional-calls.lino','js/tests/translation-javascript-optional-calls.test.js'])assert.ok(calls.history.some(value=>value.path===path&&value.source.length>0));
});
