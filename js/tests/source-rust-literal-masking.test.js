// Deferred controls; source authors parse this module without importing it.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {analyzeProgram,ProgramRepresentation} from '../src/program-representation.js';
const fixture=readProjectData(new URL('../../parity/fixtures/source-rust-literal-masking.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.node);
for(const row of fixture.cases)test(row.name,()=>{
 const program=analyzeProgram(row.source,'Rust'),expected=fixture.binding;
 assert.equal(program.network.reconstructText(),row.source);
 const binding=program.bindings.find(binding=>binding.name===expected.name&&binding.kind===expected.kind);
 assert.ok(binding);assert.equal(binding.references.length,expected.references);
 for(const name of row.excluded){assert.ok(!program.bindings.some(binding=>binding.name===name));assert.ok(!program.unresolvedReferences.some(reference=>reference.name===name));}
 const renamed=program.renameBinding(binding.id,expected.replacement);
 assert.equal(renamed.source,row.renamed);assert.equal(renamed.network.reconstructText(),row.renamed);
 const restored=ProgramRepresentation.fromSnapshot(renamed.serializeSnapshot());
 assert.equal(restored.source,row.renamed);assert.deepEqual(restored.bindings,renamed.bindings);
});
