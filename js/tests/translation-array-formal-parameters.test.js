// Deferred source profile/plan controls. Source authoring never imports this.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {tokenize} from '../src/translation/lexer.js';
import {findJavaScriptArrayFormalPattern,planJavaScriptArrayFormalParameters,acceptJavaScriptArrayFormalContext} from '../src/translation/array-formal-parameter-rules.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from '../src/translation/array-binding-policy.js';
import {planJavaScriptArrayFormalParametersWithDefaults,readJavaScriptArrayFormalDefaultBindingPositions,findJavaScriptFormalDefaultExpressionEnd} from '../src/translation/array-formal-parameter-rules.js';
import {runInNewContext} from 'node:vm';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
const fixture=readProjectData(new URL('../../parity/fixtures/array-formal-parameters.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
test('actual formal context and default initializer boundaries remain explicit',()=>{
 for(const context of ['ordinary named function declaration','ordinary arrow function closure','ordinary function expression closure','ordinary object method closure','ordinary constant arrow function','ordinary constant function expression','ordinary namespace object method']){
  assert.equal(acceptJavaScriptArrayFormalContext(context,false,0),true);
  assert.equal(acceptJavaScriptArrayFormalContext(context,true,0),false);
  assert.equal(acceptJavaScriptArrayFormalContext(context,false,1),false);
 }
 assert.equal(acceptJavaScriptArrayFormalContext('unknown',false,0),false);
 for(const source of ['value=[1,2])','callback=()=>[1,2])','value="[first]")']){const tokens=tokenize(source,'JavaScript').tokens;assert.equal(findJavaScriptArrayFormalPattern(tokens.map(token=>token.kind),tokens.map(token=>token.value),0),-1);}
 const opaque=tokenize('value=`opaque`,[first])','JavaScript').tokens;
 const selected=findJavaScriptArrayFormalPattern(opaque.map(token=>token.kind),opaque.map(token=>token.value),0);
 assert.equal(opaque[selected].value,'[');assert.equal(opaque[selected].start,15);
 const tokens=tokenize('[first,,...rest],scale)','JavaScript').tokens;
 const plan=planJavaScriptArrayFormalParameters(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings);
 assert.ok(plan[0]>0);assert.equal(plan[1],2);assert.equal(plan[2],2);
});
test('separate default plan preserves the required-only guard and exact intervals',()=>{
 const tokens=tokenize('[first=3])','JavaScript').tokens,kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value);
 assert.deepEqual(planJavaScriptArrayFormalParameters(kinds,values,0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings),[-3,2]);
 const plan=planJavaScriptArrayFormalParametersWithDefaults(kinds,values,0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings);
 assert.deepEqual(plan.slice(-2),[3,4]);assert.deepEqual(readJavaScriptArrayFormalDefaultBindingPositions(plan),[1]);
 for(const source of ['Math.max(3,4)]','(3,4)]','"] , )"]','({key:[1,2]}).key]']){
  const tokens=tokenize(source,'JavaScript').tokens;const end=findJavaScriptFormalDefaultExpressionEnd(tokens.map(token=>token.kind),tokens.map(token=>token.value),0);assert.equal(tokens[end].value,']');assert.ok(end>0);
 }
});
for(const row of fixture.correctedInputs)test('preserved former default refusal now retains its complete original value',()=>{
 const run=source=>{let output='';runInNewContext(source,{console:{log:value=>{output+=String(value)+'\n';}}},{timeout:1000});return output;};
 assert.equal(run(row.source),row.expected);
 const emitted=emitJavaScript(checkProgram(parseJavaScript(row.source)));
 assert.equal(run(emitted.text.replace(/^export /gmu,'')),row.expected);
});
