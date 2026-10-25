// Future original/emitted values and real evaluation-order controls; not executed in drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {format} from 'node:util';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
const fixture=readProjectData(new URL('../../parity/fixtures/absent-record-properties.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.cases)test(row.name,()=>{const run=source=>String(runInNewContext(source.replace(/^export /gmu,'')+'\ncompute();',{}, {timeout:1000}));assert.equal(run(row.source),row.value);assert.equal(String(runInNewContext(emitJavaScript(checkProgram(parseJavaScript(row.source))).text,{}, {timeout:1000})),row.value);});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
for(const row of fixture.traces)test(row.name,()=>{const observe=source=>{const lines=[];let error=null;try{runInNewContext(source.replace(/^export /gmu,''),{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});}catch(thrown){error={name:thrown.name,message:thrown.message};}return {lines,error};};const expected={lines:row.lines,error:row.error};assert.deepEqual(observe(row.source),expected);assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(row.source))).text),expected);});
test('a Rust nominal value still refuses a missing field',()=>{const surface=parseRust('struct Object { value:i64 } fn f()->i64{let object=Object{value:3};object.missing}');assert.throws(()=>checkProgram(surface),error=>error.kind==='unsupported'&&/field/u.test(error.reason));});
