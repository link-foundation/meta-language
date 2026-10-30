// Future context/coverage controls; never executed by source authors.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {readJavaScriptRecordConstantContext,acceptJavaScriptRecordCallableParameterProfile} from '../src/translation/record-callable-parameter-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/record-callable-parameters.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
test('ordinary callable contexts stay distinct from the named and nested closure profiles',()=>{
 assert.equal(readJavaScriptRecordConstantContext(false),'ordinary constant arrow function');
 assert.equal(readJavaScriptRecordConstantContext(true),'ordinary constant function expression');
 for(const context of ['ordinary constant arrow function','ordinary constant function expression','ordinary namespace object method']){
  assert.equal(acceptJavaScriptRecordCallableParameterProfile(context,false,0),true);
  assert.equal(acceptJavaScriptRecordCallableParameterProfile(context,true,0),false);
  assert.equal(acceptJavaScriptRecordCallableParameterProfile(context,false,1),false);
 }
 for(const context of ['ordinary named function declaration','ordinary arrow function closure','unknown'])assert.equal(acceptJavaScriptRecordCallableParameterProfile(context,false,0),false);
});
