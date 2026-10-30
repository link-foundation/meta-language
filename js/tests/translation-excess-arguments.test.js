// Deferred actual source controls. No drafting author executes this module.
import assert from 'node:assert/strict';
import test from 'node:test';
import {format} from 'node:util';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
const fixture=readProjectData(new URL('../../parity/fixtures/excess-arguments.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.traces)test(row.name,()=>{const observe=source=>{const lines=[];let error=null;try{runInNewContext(source.replace(/^export /gmu,''),{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});}catch(thrown){error={name:thrown.name,message:thrown.message};}return {lines,error};};const expected={lines:row.lines,error:row.error};assert.deepEqual(observe(row.source),expected);assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(row.source))).text),expected);});
test('Rust keeps its exact declared arity',()=>{const surface=parseRust('fn read(value:i64)->i64{value} fn f()->i64{read(3,4)}');assert.throws(()=>checkProgram(surface),error=>error.kind==='type'&&/expects 1 arguments but got 2/u.test(error.reason));});
