// Deferred actual source contracts, immutable predecessors and closed calls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {acceptRustFreshVectorResultDeclaration,acceptRustFreshVectorResultCall} from '../src/translation/rust-fresh-vector-call-result-rules.js';
import {assertRustFreshVectorCallResultCorrection} from './support/rust-fresh-vector-call-result-corrections.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-fresh-vector-call-results.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.declarations)test('actual closed result declaration: '+JSON.stringify(row),()=>assert.equal(acceptRustFreshVectorResultDeclaration(row.ordinary,row.model,row.wrappers,row.element,row.copy),row.expected));
for(const row of fixture.calls)test('actual closed result call: '+JSON.stringify(row),()=>assert.equal(acceptRustFreshVectorResultCall(row.kind,row.model,row.wrappers,row.constructed,row.declared,row.called),row.expected));
for(const row of fixture.corrections)test('the exact original call tail now has fresh result value controls',()=>assertRustFreshVectorCallResultCorrection(row.original));
test('the existing explicit constructor-only result profile keeps its refusal',()=>{
 const previous=readProjectData(new URL('../../parity/fixtures/rust-array-results.lino',import.meta.url));
 const source=fixture.corrections[0].original.source;
 assert.throws(()=>checkProgram(parseRust(source,{arrayResultModel:previous.arrayResultModel})),error=>error.kind==='unsupported'&&error.reason.startsWith('fresh Rust array result:'));
});
