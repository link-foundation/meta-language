// Deferred source controls; no drafting author imports this file.
import assert from 'node:assert/strict';
import test from 'node:test';
import {format} from 'node:util';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {planOptionalCallableArguments,planOptionalCallableParameterBridge} from '../src/translation/optional-callable-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/optional-callables.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
test('only actual optional argument domains acquire implicit undefined',()=>{assert.deepEqual(planOptionalCallableArguments('JavaScript',['float','option'],['','undefined'],1),[2,0,-1]);assert.deepEqual(planOptionalCallableArguments('JavaScript',['option'],['both'],0),[1,-1]);for(const language of ['Rust','Lean','Rocq'])assert.deepEqual(planOptionalCallableArguments(language,['option'],['undefined'],0),[]);assert.deepEqual(planOptionalCallableArguments('JavaScript',['option'],['null'],0),[]);assert.deepEqual(planOptionalCallableArguments('JavaScript',['float'],[''],0),[]);});
test('optional callback bridges retain every exact or explicit present parameter',()=>{assert.deepEqual(planOptionalCallableParameterBridge('JavaScript',3,[false,true],[true,false]),[2,1,0]);assert.deepEqual(planOptionalCallableParameterBridge('JavaScript',1,[false],[false]),[]);assert.deepEqual(planOptionalCallableParameterBridge('JavaScript',2,[true,true],[false,false]),[]);});
for(const row of fixture.traces)test(row.name,()=>{const observe=source=>{const lines=[];let error=null;try{runInNewContext(source.replace(/^export /gmu,''),{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});}catch(thrown){error={name:thrown.name,message:thrown.message};}return {lines,error};};const expected={lines:row.lines,error:row.error};assert.deepEqual(observe(row.source),expected);assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(row.source))).text),expected);});
