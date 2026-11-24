// Future source decisions; the shared source matrix covers full products.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../src/project-data-files.js';
import {readRustFreshVectorArgumentForm,acceptRustFreshOwnedVectorArgument} from '../src/translation/rust-fresh-vector-argument-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-fresh-vector-arguments.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
test('every declared constructor decision retains its independent expected form',()=>{
 for(const row of fixture.forms)assert.equal(readRustFreshVectorArgumentForm(row.kind,row.diverging,row.model,row.wrappers,row.constructor),row.expected,JSON.stringify(row));
});
test('every declared scalar consumption decision retains its actual parameter contract',()=>{
 for(const row of fixture.admissions)assert.equal(acceptRustFreshOwnedVectorArgument(row.ordinary,row.model,row.wrappers,row.element,row.fresh),row.expected,JSON.stringify(row));
});
