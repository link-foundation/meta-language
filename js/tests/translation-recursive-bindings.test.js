// Deferred source controls; drafting authors never import this module.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from '../src/translation/array-binding-policy.js';
import {planJavaScriptRecursiveBindings,readJavaScriptRecursiveObjectBindingPositions,readJavaScriptRecursiveArrayElementIndex} from '../src/translation/recursive-binding-rules.js';
import {planJavaScriptRecursiveObjectBindings} from '../src/translation/recursive-object-binding-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/recursive-bindings.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
test('mixed immediate owners retain indices independently of their descendants',()=>{
 const tokens=tokenize('[{left:[first,,second]},,third,...rest]','JavaScript').tokens,kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value);
 const plan=planJavaScriptRecursiveBindings(kinds,values,0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings,[],256);
 assert.equal(plan[0],tokens.length-1);
 assert.deepEqual(readJavaScriptRecursiveObjectBindingPositions(plan).map(at=>values[at]),['first','second','third','rest']);
 const positions=[];for(let row=1;row<plan[1];row+=1)if([3,4].includes(plan[2+row*8]))positions.push([values[plan[2+row*8+2]],readJavaScriptRecursiveArrayElementIndex(plan,row)]);
 assert.deepEqual(positions,[['{',0],['first',0],['second',2],['third',2],['...',3]]);
});
test('the original object-only interpretation still refuses the same mixed input',()=>{
 const tokens=tokenize('{value:[nested]}','JavaScript').tokens;
 assert.equal(planJavaScriptRecursiveObjectBindings(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings,[],256)[0],-5);
});
