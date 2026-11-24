// Deferred profile/refusal checks; actual source/product/codec cases use the
// common source matrix and native producer after drafting is complete.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {readJavaScriptRecordClosureContext,acceptJavaScriptRecordClosureParameterProfile} from '../src/translation/record-closure-parameter-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/record-closure-parameters.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
test('closure contexts retain actual roles and no named-declaration alias',()=>{
 for(const [method,expression,context]of [[false,false,'ordinary arrow function closure'],[false,true,'ordinary function expression closure'],[true,true,'ordinary object method closure']]){
  assert.equal(readJavaScriptRecordClosureContext(method,expression),context);
  assert.equal(acceptJavaScriptRecordClosureParameterProfile(context,false,0),true);
  assert.equal(acceptJavaScriptRecordClosureParameterProfile(context,true,0),false);
  assert.equal(acceptJavaScriptRecordClosureParameterProfile(context,false,1),false);
 }
 assert.equal(acceptJavaScriptRecordClosureParameterProfile('ordinary named function declaration',false,0),false);
});
