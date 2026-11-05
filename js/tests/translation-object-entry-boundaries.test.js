// Deferred direct archive-admission and existing pair-domain refusal controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {checkProgram} from '../src/translation/check.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {validateLinkedTypedProgram} from '../src/typed-program-links-validation.js';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
test('own entry pair equality keeps its explicit identity obligation',()=>{
 assert.throws(()=>checkProgram(parseJavaScript('export function compute(){return Object.entries({z:1}).map(pair=>pair===pair).join("|");}')),error=>error.kind==='unsupported'&&error.reason==='comparison of tuples: tuple value equality and JavaScript reference identity need an explicit operational comparison model; compare positional elements');
});
test('an edited linked entry operation cannot invent its result or argument types',()=>{
 const original=checkProgram(parseJavaScript('export function compute(){return Object.entries({z:1,a:2}).map(([key,value])=>key+String(value)).join("|");}'));
 encodeTypedProgramLinks(original);
 function operation(program){const seen=new Set(),pending=[...program.declarations.values()];while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;seen.add(value);if(value.k==='math'&&value.op==='pairJavaScriptObjectEntryValues')return value;for(const child of Object.values(value))Array.isArray(child)?pending.push(...child):pending.push(child);}assert.fail('the actual entry operation must remain in the pre-emission checked graph');}
 for(const edit of [node=>{node.type={kind:'array',element:{kind:'float'}};},node=>{node.args[0].type={kind:'array',element:{kind:'float'}};},node=>{node.args[1].type={...node.args[1].type,reference:true};},node=>{node.args.pop();}]){const copy=structuredClone(original);edit(operation(copy));assert.throws(()=>validateLinkedTypedProgram(copy));}
});
