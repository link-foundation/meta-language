// Deferred actual constructor, language-boundary and trace controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {format} from 'node:util';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram,coerce} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
const fixture=readProjectData(new URL('../../parity/fixtures/absence-conversions.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.traces)test(row.name,()=>{const observe=source=>{const lines=[];let error=null;try{runInNewContext(source,{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});}catch(thrown){error={name:thrown.name,message:thrown.message};}return {lines,error};};const expected={lines:row.lines,error:row.error};assert.deepEqual(observe(row.source),expected);assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(row.source))).text),expected);});
test('an absence policy cannot erase a possible source nullish alternative',()=>{
 const source={k:'var',name:'value',type:{kind:'option',element:{kind:'unit'},absence:'null'}},target={kind:'option',element:{kind:'float'},absence:'null'};
 assert.throws(()=>coerce(source,target,'JavaScript'),error=>error.kind==='type');
 for(const language of ['Rust','Lean','Rocq'])assert.throws(()=>coerce(source,{...target,absence:'both'},language),error=>error.kind==='type');
 const result=coerce(source,{...target,absence:'both'},'JavaScript');assert.equal(result.k,'let');assert.equal(result.body.k,'if');assert.equal(result.body.then.absence,'null');assert.equal(result.body.else.absence,'undefined');
});
