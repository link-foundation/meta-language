// Deferred value/effect, original-domain and complete source controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {format} from 'node:util';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {planJavaScriptMissingArgumentInference} from '../src/translation/missing-argument-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/missing-arguments.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function observe(source){const lines=[];let error=null;try{runInNewContext(source,{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});}catch(thrown){error={name:thrown.name,message:thrown.message};}return {lines,error};}
for(const row of fixture.traces)test(row.name,()=>{const expected={lines:row.lines,error:row.error};assert.deepEqual(observe(row.source),expected);const program=checkProgram(parseJavaScript(row.source));assert.deepEqual(observe(emitJavaScript(program).text),expected);const packet=encodeTypedProgramLinks(program);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(observe(emitTypedProgramLinks(network,packet.root,'JavaScript').code),expected);});
test('only actual inferred owners receive implicit undefined',()=>{assert.deepEqual(planJavaScriptMissingArgumentInference([true,true,false],[false,true,false],0),[-1,-2,-2]);assert.deepEqual(planJavaScriptMissingArgumentInference([false],[false],0),[]);assert.deepEqual(planJavaScriptMissingArgumentInference([true],[true],0),[]);assert.deepEqual(planJavaScriptMissingArgumentInference([true],[false],1),[]);});
test('a declared required Number keeps its original arity requirement',()=>{assert.throws(()=>checkProgram(parseJavaScript('/** @param {number} value @returns {string} */function read(value){return String(value);}export function compute(){return read();}')),error=>error.kind==='type');});
test('complete missing-input definition survives codecs and all emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/missing-argument-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
