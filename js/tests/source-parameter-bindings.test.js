// Deferred controls only; no authoring script imports this module or its oracle.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {analyzeProgram,ProgramRepresentation} from '../src/program-representation.js';
import {acceptJavaScriptSourceParameterIdentifierTerm,acceptJavaScriptSourceBindingRootTerm,planJavaScriptSourceParameterBindings} from '../src/translation/source-parameter-binding-rules.js';
import {observeSourceParameterProgram} from '../../experiments/source-parameter-bindings/oracle.mjs';
const fixture=readProjectData(new URL('../../parity/fixtures/source-parameter-bindings.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.node);
for(const row of fixture.terms)test('actual formal identifier term '+row.term,()=>{assert.equal(acceptJavaScriptSourceParameterIdentifierTerm(row.term),row.expected);assert.equal(acceptJavaScriptSourceBindingRootTerm(row.term),row.root);});
for(const row of fixture.plans)test(row.name,()=>assert.deepEqual(planJavaScriptSourceParameterBindings(row.kinds,row.values,row.candidates,row.defaultFirsts,row.defaultLasts,row.computedFirsts,row.computedLasts),row.expected));
for(const row of fixture.cases)test(row.name,()=>{
 const program=analyzeProgram(row.source,'JavaScript');
 assert.equal(program.network.reconstructText(),row.source);
 assert.deepEqual(program.bindings.map(binding=>[binding.name,binding.kind,binding.references.length]),row.bindings);
 assert.deepEqual(program.unresolvedReferences.map(reference=>reference.name),row.unresolved);
 assert.equal(observeSourceParameterProgram(row,row.source).value,row.value);
 const binding=program.bindings.find(binding=>binding.name===row.rename.name&&binding.kind===(row.rename.kind??'parameter'));assert.ok(binding);
 const renamed=program.renameBinding(binding.id,row.rename.replacement);
 assert.equal(renamed.source,row.rename.source);
 assert.equal(observeSourceParameterProgram(row,renamed.source).value,row.value);
 const restored=ProgramRepresentation.fromSnapshot(renamed.serializeSnapshot());
 assert.equal(restored.source,renamed.source);assert.deepEqual(restored.bindings,renamed.bindings);
});
