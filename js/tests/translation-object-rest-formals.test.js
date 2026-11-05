// Deferred formal source controls; source authors never import this module.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {tokenize} from '../src/translation/lexer.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from '../src/translation/array-binding-policy.js';
import {planJavaScriptSourceRecordParametersWithRest,planJavaScriptSourceRecordParametersWithDefaults,readJavaScriptRecordFormalDefaultBindingPositions} from '../src/translation/javascript-record-parameter-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/object-rest-formals.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
test('formal rest keeps actual slots names and exclusions beside the unchanged preceding default contract',()=>{
 const tokens=tokenize('{left,...rest},tail)','JavaScript').tokens,kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value),reserved=JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings;
 const plan=planJavaScriptSourceRecordParametersWithRest(kinds,values,0,reserved);
 assert.equal(plan[0],tokens.length-1);assert.equal(plan[1],2);
 assert.deepEqual(readJavaScriptRecordFormalDefaultBindingPositions(plan).map(position=>tokens[position].value),['left','rest','tail']);
 assert.deepEqual(planJavaScriptSourceRecordParametersWithDefaults(kinds,values,0,reserved),[-6,3]);
});
