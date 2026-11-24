// Future complete source controls. Never imported by source authors.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {tokenize} from '../src/translation/lexer.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from '../src/translation/array-binding-policy.js';
import {planJavaScriptRecursiveObjectBindings,readJavaScriptRecursiveObjectBindingPositions,readJavaScriptRecursiveObjectExcludedPositions,readJavaScriptRecursiveObjectInitializerBinding,readJavaScriptObjectBindingDepthLimit} from '../src/translation/recursive-object-binding-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/recursive-object-bindings.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
const planOf=(source,limit=readJavaScriptObjectBindingDepthLimit())=>{const tokens=tokenize(source,'JavaScript').tokens;return {tokens,plan:planJavaScriptRecursiveObjectBindings(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings,[],limit)};};
test('actual recursive owners preserve each rest exclusion and the first descendant initialization boundary',()=>{
 const {tokens,plan}=planOf('{left:{value=3,...inner},right,...outer}');
 assert.equal(plan[0],tokens.length-1);
 assert.deepEqual(readJavaScriptRecursiveObjectBindingPositions(plan).map(at=>tokens[at].value),['value','inner','right','outer']);
 assert.deepEqual(readJavaScriptRecursiveObjectExcludedPositions(plan,0).map(at=>tokens[at].value),['left','right']);
 assert.deepEqual(readJavaScriptRecursiveObjectExcludedPositions(plan,1).map(at=>tokens[at].value),['value']);
 assert.equal(tokens[readJavaScriptRecursiveObjectInitializerBinding(plan,1)].value,'value');
});
test('shared recursion depth is explicit and malformed or duplicate inputs do not become partial plans',()=>{
 assert.ok(planOf('{left:{value}}',2).plan[0]>0);
 assert.equal(planOf('{left:{value}}',1).plan[0],-9);
 assert.equal(planOf('{left:{value},right:{value}}').plan[0],-8);
 for(const limit of [0,-1,NaN,Infinity,0.5])assert.equal(planOf('{value}',limit).plan[0],-1);
});
test('the retained primitive receiver keeps its original Node value and explicit unresolved field protocol',()=>{
 const row=fixture.retainedPrimitiveInput;
 assert.equal(runInNewContext(row.source.replace(/^export /gmu,'')+'\ncompute();',{}, {timeout:1000}),row.expected);
 assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind==='unsupported'&&error.reason===row.currentRefusal);
});
