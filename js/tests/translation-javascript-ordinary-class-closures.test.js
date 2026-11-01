// Actual supplied modules use the existing CST alias/re-export composer.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {readSuppliedSourceLibraryClosure} from '../src/translation/source-library-closure.js';
import {captureJavaScriptOrdinaryClassSourceClosure,checkJavaScriptOrdinaryClassSourceClosure} from '../src/translation/javascript-ordinary-class-closure-source.js';
import {prepareJavaScriptOrdinaryClassSourceClosure} from '../src/translation/javascript-ordinary-class-closure.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {analyzeProgram} from '../src/program-representation.js';
import {javascriptOrdinaryClassClosureOriginals as originals,javascriptOrdinaryClassScopeWorkloads as workloads} from './fixtures/javascript-ordinary-class-closures.mjs';
const options={aliases:true,classes:'closed local class declarations 1',reExports:'resolved named module providers 1'};
function capture(row,modules){return captureJavaScriptOrdinaryClassSourceClosure(readSuppliedSourceLibraryClosure(row.entryPath,modules,options),{entryPath:row.entryPath});}
function run(row,archive){const linked=prepareJavaScriptOrdinaryClassSourceClosure(archive,{realmIdentity:67n}),execution=runJavaScriptSourceTransition(linked.embedding,linked.begin(row.entry,row.arguments.map(linked.embedding.encode)),{steps:65536}).transition;assert.equal(execution.$,'machineCompleted',execution.reason);assert.deepEqual(linked.embedding.materializeData(execution.value),row.expected);return linked;}
for(const row of originals)test(row.name,()=>{const archive=capture(row,row.modules);assert.deepEqual(checkJavaScriptOrdinaryClassSourceClosure(archive),archive);run(row,archive);const forged=structuredClone(archive);forged.owners[0].originalName='forgedName';assert.throws(()=>checkJavaScriptOrdinaryClassSourceClosure(forged));});
for(const row of workloads)test(row.name,async()=>{
 const modules=row.paths.map(path=>({path,source:readFileSync(new URL('../../'+path,import.meta.url),'utf8')})),archive=capture(row,modules),input=modules.find(module=>module.path===row.scopeSource),original=readFileSync(new URL('../../'+row.completeOriginalScopeModule,import.meta.url),'utf8'),program=analyzeProgram(input.source,'JavaScript'),binding=program.bindings.find(binding=>binding.kind==='class'&&binding.name===row.className),syntax=program.sourceMappings.find(node=>node.term==='class_declaration'&&node.start<=binding.declaration.start&&node.end>=binding.declaration.end);
 assert.ok(syntax);assert.ok(original.includes(input.source.slice(syntax.start,syntax.end)),'the entire selected Scope declaration remains exact original code; complete ir.js remains an independent obligation');assert.equal(archive.completeSourceReceipts.length,row.paths.length);run(row,archive);
 // Genuine original ESM bodies execute only as this deferred oracle.
 const namespace=await import(new URL('../../'+row.entryPath,import.meta.url));assert.deepEqual(namespace[row.entry](...row.arguments),row.expected);
});
