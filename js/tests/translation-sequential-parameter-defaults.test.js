// Deferred source, trace and shared plan controls. Never run by source authors.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {format} from 'node:util';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {planSequentialParameterInitialization,requireSequentialParameterInitialization} from '../src/translation/sequential-parameter-default-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/sequential-parameter-defaults.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const observe=source=>{const lines=[];let error=null;try{runInNewContext(source.replace(/^export /gmu,''),{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});}catch(thrown){error={name:thrown.name,message:thrown.message};}return {lines,error};};
for(const row of fixture.retainedInputs)test('complete retained default input '+row.source,()=>{const source=row.source+row.invocation,expected={lines:row.expected.trimEnd().split('\n'),error:null};assert.deepEqual(observe(source),expected);assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(source))).text),expected);});
for(const row of fixture.traces)test(row.name,()=>{const expected={lines:row.lines,error:row.error};assert.deepEqual(observe(row.source),expected);assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(row.source))).text),expected);});
test('the shared initialization plan evaluates actual arguments before parameters',()=>{assert.deepEqual(planSequentialParameterInitialization(3,2),[0,0,0,1,1,0,1,1,1,2]);assert.equal(requireSequentialParameterInitialization([true,true],false),false);assert.equal(requireSequentialParameterInitialization([true,false],false),true);assert.equal(requireSequentialParameterInitialization([false,true],true),true);});
