// Future source and independent initialization-order controls. Execute after full drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {tokenize} from '../src/translation/lexer.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from '../src/translation/array-binding-policy.js';
import {planJavaScriptWholeParameterDefaults,acceptJavaScriptWholeParameterInitializationOrder,containsJavaScriptWholeParameterDefault} from '../src/translation/whole-parameter-default-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/whole-parameter-defaults.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
for(const row of fixture.interleavingControls)test('the original source preserves its earlier pattern binding failure',()=>{assert.throws(()=>runInNewContext(row.source,{}, {timeout:1000}),error=>error.name===row.errorClass&&error.message===row.message);assert.throws(()=>runInNewContext(emitJavaScript(checkProgram(parseJavaScript(row.source))).text.replace(/^export /gmu,''),{}, {timeout:1000}),error=>error.name===row.errorClass&&error.message===row.message);});
test('whole default plans retain actual slots and retain the call-site commutation predicate',()=>{
 const plan=source=>{const tokens=tokenize(source,'JavaScript').tokens;return planJavaScriptWholeParameterDefaults(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings,256);};
 const first=plan('first=3,{value}={value:first})');assert.ok(first[0]>0);assert.equal(first[1],2);assert.equal(containsJavaScriptWholeParameterDefault(first),true);assert.equal(acceptJavaScriptWholeParameterInitializationOrder(first),true);
 assert.equal(acceptJavaScriptWholeParameterInitializationOrder(plan('{first},other=3)')),false);
 assert.equal(acceptJavaScriptWholeParameterInitializationOrder(plan('{first},{other}={other:3})')),false);
});
