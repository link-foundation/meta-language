// Deferred original-source, interval, refusal and complete source controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from '../src/translation/array-binding-policy.js';
import {planJavaScriptSourceRecordParameters,planJavaScriptSourceRecordParametersWithDefaults,readJavaScriptRecordFormalDefaultBindingPositions} from '../src/translation/javascript-record-parameter-rules.js';
import {partitionJavaScriptFormalInitializerBindingPositions,findJavaScriptPatternDefaultExpressionEnd} from '../src/translation/array-formal-parameter-rules.js';
import {readJavaScriptRecordFormalDefaultOperands} from '../src/translation/record-formal-defaults.js';
const fixture=readProjectData(new URL('../../parity/fixtures/record-formal-defaults.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
test('required-only record plans retain their original default refusal',()=>{
 const tokens=tokenize('{value=3})','JavaScript').tokens,kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value),reserved=JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings;
 assert.deepEqual(planJavaScriptSourceRecordParameters(kinds,values,0,reserved),[-2,2]);
 const plan=planJavaScriptSourceRecordParametersWithDefaults(kinds,values,0,reserved);
 assert.equal(plan[0],tokens.length-1);assert.deepEqual(plan.slice(-4),[1,1,3,4]);
});
test('record initializer intervals and bound names retain their exact source roles',()=>{
 const tokens=tokenize('first,{value:second=first,third=second},last)','JavaScript').tokens;
 const plan=planJavaScriptSourceRecordParametersWithDefaults(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings),positions=readJavaScriptRecordFormalDefaultBindingPositions(plan);
 assert.deepEqual(positions.map(position=>tokens[position].value),['first','second','third','last']);
 const selected=partitionJavaScriptFormalInitializerBindingPositions(positions,positions[2]);assert.equal(selected[0],2);assert.deepEqual(selected.slice(1,3).map(position=>tokens[position].value),['first','second']);
 for(const source of ['Math.max(3,4)}','(3,4)}','"} , )"}','`record${3}`}']){const tokens=tokenize(source,'JavaScript').tokens,end=findJavaScriptPatternDefaultExpressionEnd(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,'}');assert.equal(tokens[end].kind,'punct');assert.equal(tokens[end].value,'}');}
});
test('source-only default operands retain actual source nodes and reject malformed arity and spread',()=>{
 const owner={k:'name',path:['owner']},fallback={k:'num',value:'3'},args=[{value:owner,spread:false},{value:{k:'str',value:'value'},spread:false},{value:fallback,spread:false}];
 const selected=readJavaScriptRecordFormalDefaultOperands({args});assert.equal(selected.value.object,owner);assert.equal(selected.fallback,fallback);assert.equal(selected.value.k,'field');
 for(const invalid of [args.slice(0,2),[...args,{value:fallback,spread:false}],[{...args[0],spread:true},...args.slice(1)],[args[0],{value:owner,spread:false},args[2]]])assert.throws(()=>readJavaScriptRecordFormalDefaultOperands({args:invalid}),error=>error.kind==='type');
 assert.throws(()=>readJavaScriptRecordFormalDefaultOperands({args},'Rust'),error=>error.kind==='type');
});
for(const row of fixture.correctedInputs)test('retained former closure default refusal has independent original and emitted value controls',()=>{
 const run=source=>runInNewContext(source.replace(/^export /gmu,'')+'\ncompute();',{}, {timeout:1000});
 assert.equal(String(run(row.source)),row.expected);
 assert.equal(String(run(emitJavaScript(checkProgram(parseJavaScript(row.source))).text)),row.expected);
});
