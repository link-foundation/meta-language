// Deferred complete actual-source and exact field-plan controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {selectJavaScriptObjectRestFields} from '../src/translation/object-rest-rules.js';
import {copyJavaScriptRecordPropertyOrder} from '../src/translation/javascript-record-order-rules.js';
import {planJavaScriptObjectPatternBindings,readJavaScriptObjectPatternBindingPositions} from '../src/translation/javascript-record-parameter-rules.js';
import {tokenize} from '../src/translation/lexer.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from '../src/translation/array-binding-policy.js';
const fixture=readProjectData(new URL('../../parity/fixtures/object-rest-bindings.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
test('rest shares actual binding positions while every historical required and default plan remains exact',()=>{
 const tokens=tokenize('{left:first=3,...rest}','JavaScript').tokens,kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value),reserved=JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings;
 const plan=planJavaScriptObjectPatternBindings(kinds,values,0,reserved,[],3);
 assert.equal(plan[0],tokens.length-1);assert.equal(plan[1],2);
 assert.deepEqual(plan.slice(-4),[7,8,-2,-2]);
 assert.deepEqual(readJavaScriptObjectPatternBindingPositions(plan).map(position=>tokens[position].value),['first','rest']);
 assert.deepEqual(planJavaScriptObjectPatternBindings(kinds,values,0,reserved,[],2),[-6,7]);
 for(const profile of [0,1])assert.deepEqual(planJavaScriptObjectPatternBindings(kinds,values,0,reserved,[],profile),[-2,4]);
});
test('field exclusion and runtime order projection retain independent canonical coordinates',()=>{
 const source=['alpha','beta','selected','zeta'].map(units),target=['alpha','beta','zeta'].map(units);
 assert.deepEqual(selectJavaScriptObjectRestFields(source,['selected','selected','missing'].map(units)),[0,1,3]);
 assert.deepEqual(copyJavaScriptRecordPropertyOrder(target,source,[3,2,0,1]),[2,0,1]);
 assert.deepEqual(copyJavaScriptRecordPropertyOrder([],source,[3,2,0,1]),[]);
 assert.throws(()=>copyJavaScriptRecordPropertyOrder(['foreign'].map(units),source,[3,2,0,1]),/source subset/u);
 assert.throws(()=>copyJavaScriptRecordPropertyOrder(target,source,[3,2,0,0]),/source field order/u);
});
