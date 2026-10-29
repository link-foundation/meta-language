// Deferred source/alias/codec/target controls. Never execute while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {mkdirSync,mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {join,relative} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {readSourceLibraryModuleFacts,readSourceLibraryClosure,readSourceLibraryDeclarationOwnership} from '../scripts/read-source-library-closure.mjs';
import {projectSourceLibraryEntryPoints} from '../scripts/source-library-entry-points.mjs';
import {readSourceLibraryDeclarationBindingKind} from '../src/translation/source-library-binding-rules.js';
import {acceptSourceLibraryFunctionExport} from '../src/translation/source-library-export-rules.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const root=fileURLToPath(new URL('../../',import.meta.url));
const fixture=readProjectData(new URL('../../parity/fixtures/source-library-constant-functions.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.node);
for(const [index,row]of fixture.policies.entries())test('actual source binding policy '+index,()=>{
 assert.equal(readSourceLibraryDeclarationBindingKind(row.keyword,row.kind,row.name,row.callable),row.binding);
 assert.equal(acceptSourceLibraryFunctionExport(row.keyword,row.kind,row.name,row.callable),row.exported);
});
for(const row of fixture.facts)test(row.name,()=>{
 const read=()=>readSourceLibraryModuleFacts(row.source,'actual-source.js',true);
 if(row.error){assert.throws(read,TypeError);return;}
 const facts=read();assert.deepEqual(facts.exports,row.exports);
 assert.deepEqual(facts.declarations,row.bindings.map(binding=>binding.name));
 assert.deepEqual(facts.declarationBindings??facts.declarations.map(name=>({name,kind:'function'})),row.bindings);
});
for(const row of fixture.modules)test(row.name,async context=>{
 const parent=join(root,'issue-195-results/work/source-library-constant-functions');mkdirSync(parent,{recursive:true});
 const directory=mkdtempSync(join(parent,'case-'));context.after(()=>rmSync(directory,{recursive:true,force:true}));
 for(const file of row.files){assert.match(file.path,/^[a-z][a-z-]*\.js$/u);writeFileSync(join(directory,file.path),file.source);}
 const entry=relative(root,join(directory,row.entry)).replaceAll('\\','/');
 const original=await import(pathToFileURL(join(directory,row.entry)).href);
 assert.equal(original.run(...row.arguments),row.expected);
 const read=()=>readSourceLibraryClosure(entry,{aliases:row.aliases});
 if(row.error){assert.throws(read,error=>error instanceof TypeError&&error.message.includes(row.error));return;}
 const closure=read(),checked=checkProgram(parseJavaScript(closure.source,{referenceSemantics:false}));
 const owners=readSourceLibraryDeclarationOwnership(closure,checked);assert.equal(owners.length,checked.declarations.size);
 const selected=projectSourceLibraryEntryPoints(checked,closure,entry).program;
 const packet=encodeTypedProgramLinks(selected,{provenance:{sourceModel:fixture.model,sourceClosure:closure,sourceDeclarationOwners:owners}}),products=[selected,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}))];
 for(const product of products){
  const emitted=emitJavaScript(product).text.replace(/^export /gmu,'');
  assert.equal(runInNewContext(emitted+'\nrun(...input)',{input:row.arguments},{timeout:1000}),row.expected);
  for(const emit of [emitRust,emitLean,emitRocq])assert.ok(emit(product).definitions.length>0);
 }
 for(const input of closure.inputs){assert.equal(input.source,row.files.find(file=>input.path.endsWith('/'+file.path)).source);assert.ok(input.sourceSha256.length===64);}
});
