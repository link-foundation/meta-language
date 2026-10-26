// Deferred source facts, actual rules, complete products and checked bounds.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {readRustScalarArrayIndexReceiverError,readRustScalarArrayIndexOperandError} from '../src/translation/rust-array-index-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-array-indices.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.receivers)test('actual scalar array receiver decision: '+JSON.stringify(row),()=>assert.equal(readRustScalarArrayIndexReceiverError(row.kind,row.element,row.model,row.proven,row.layers,row.inferredCopy),row.expected));
for(const row of fixture.operands)test('actual usize operand decision: '+JSON.stringify(row),()=>assert.equal(readRustScalarArrayIndexOperandError(row.literal,row.kind,row.bits,row.signed,row.owner,row.wrappers,row.proven),row.expected));
for(const row of fixture.corrections??[])test('the exact earlier arithmetic input retains positive assertions: '+row.name,()=>{
 assert.deepEqual(row.previousExpectation,{kind:'unsupported',reason:'Rust scalar array index: the index needs an unsuffixed integer literal or a proven direct usize source owner'});
 const program=checkProgram(parseRust(row.source)),javascript=emitJavaScript(program),target=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='bad').target;
 const operation=runInNewContext(javascript.text.replace(/^export /gmu,'')+'\n'+target+';');
 assert.equal(String(operation(...row.arguments.map(values=>values.map(BigInt)))),row.value);
 assert.ok(fixture.cases.some(control=>control.source.startsWith(row.source)),'the same complete source joins original native products');
});
for(const row of fixture.bounds)test(row.name,()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
 const program=checkProgram(parseRust(row.source)),javascript=emitJavaScript(program),target=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='read').target;
 const operation=runInNewContext(javascript.text.replace(/^export /gmu,'')+'\n'+target+';');
 assert.throws(()=>operation([7n],BigInt(row.index)),error=>error.name==='RangeError'&&error.message==='array index '+row.index+' out of range');
});
