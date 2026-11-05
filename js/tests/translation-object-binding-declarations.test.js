// Deferred actual source, initialization order and retained-input controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from '../src/translation/array-binding-policy.js';
import {planJavaScriptObjectPatternBindings,readJavaScriptObjectPatternBindingPositions,readJavaScriptPatternInitializerScopeBoundary,readJavaScriptPatternInitializerSubject} from '../src/translation/javascript-record-parameter-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/object-binding-declarations.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
test('shared pattern plan retains properties, actual local names and every initializer interval',()=>{
 const tokens=tokenize('{left:first=3,right=first+1}','JavaScript').tokens,kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value),reserved=JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings;
 const previous=[],before=structuredClone(previous),plan=planJavaScriptObjectPatternBindings(kinds,values,0,reserved,previous,2);
 assert.equal(plan[0],tokens.length-1);assert.equal(plan[1],2);assert.deepEqual(previous,before);
 assert.deepEqual(readJavaScriptObjectPatternBindingPositions(plan).map(position=>tokens[position].value),['first','right']);
 for(const profile of [0,1])assert.deepEqual(planJavaScriptObjectPatternBindings(kinds,values,0,reserved,[],profile),[-2,4]);
 assert.deepEqual(planJavaScriptObjectPatternBindings(kinds,values,0,reserved,['first'],2),[-8,3]);
});
test('declaration initializers keep a lexical boundary while all formal contexts retain theirs',()=>{
 assert.equal(readJavaScriptPatternInitializerScopeBoundary('ordinary object binding declaration'),0);
 assert.equal(readJavaScriptPatternInitializerSubject('ordinary object binding declaration'),'binding');
 assert.equal(readJavaScriptPatternInitializerScopeBoundary('ordinary named function declaration'),2);
 assert.equal(readJavaScriptPatternInitializerScopeBoundary('ordinary arrow function closure'),1);
 assert.equal(readJavaScriptPatternInitializerSubject('ordinary named function declaration'),'parameter');
});
for(const row of fixture.correctedInputs)test('preserved former mutable object refusal retains independent original and emitted observations',()=>{
 parseJavaScript(row.source);const source=row.source+row.invocation;
 const run=source=>{let output='';runInNewContext(source.replace(/^export /gmu,''),{console:{log:value=>{output+=String(value)+'\n';}}},{timeout:1000});return output;};
 assert.equal(run(source),row.expected);assert.equal(run(emitJavaScript(checkProgram(parseJavaScript(source))).text),row.expected);
});
