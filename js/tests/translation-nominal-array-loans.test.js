// Deferred loan-policy, original source and retained source controls only.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../scripts/project-data.mjs';
import {acceptNominalArrayProjectionLoan} from '../src/translation/nominal-array-loan-rules.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
const fixture=readProjectData(new URL('../../parity/fixtures/nominal-array-loans.lino',import.meta.url));
for(const row of fixture.cases)test(row.name,()=>assert.equal(acceptNominalArrayProjectionLoan(row.result,row.reference,row.receiver,row.constructors,row.cases,row.owner,row.fields,row.bindings,row.slot,row.matches,row.direct),row.expected));
test('the original single-constructor source keeps a direct read loan in native output',()=>{
 const source=readProjectData(new URL('../../parity/fixtures/javascript-data-fields.lino',import.meta.url)).cases.find(row=>row.name==='single-constructor array reads borrow the actual immutable nominal field').source;
 const output=emitRust(checkProgram(parseJavaScript(source))),entry=output.mappings.find(row=>row.kind==='function'&&row.source==='readPacket').target;
 const body=output.definitions.find(text=>text.includes('pub fn '+entry+'('));assert.ok(body);assert.ok(body.includes('.as_slice()'));
 const parameters=body.slice(body.indexOf('pub fn '+entry+'(')+('pub fn '+entry+'(').length),receiver=parameters.slice(0,parameters.indexOf(':')).trim().replace(/^mut\s+/u,'');
 assert.ok(receiver.length>0);assert.ok(body.includes('match &'+receiver+' {'),'the loan borrows its actual target parameter without a cloned temporary');
});
test('all preceding emitter sources and complete heap recipe retain their exact bytes',()=>{
 assert.equal(fixture.predecessors.length,5);
 for(const row of fixture.predecessors){const file=new URL('../../'+row.retained,import.meta.url),bytes=row.raw?readFileSync(file):Buffer.from(readProjectData(file).source);assert.equal(bytes.length,row.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),row.sha256);}
});
