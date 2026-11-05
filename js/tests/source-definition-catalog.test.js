// Deferred until every requirement is drafted. Never import this from authors.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readdirSync,readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {sourceDefinitionCatalog,sourceDefinitionEntry,createSourceDefinitionProgramLinks,createSourceDefinitionFormalFunctionLinks} from '../src/source-definition-catalog.js';
import {runSourceDefinitionCommand} from '../src/source-definition-cli.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {validateSourceDefinitionCatalogEntries,acceptSourceDefinitionCatalogHeader,acceptSourceDefinitionCatalogLimits} from '../src/source-definition-catalog-rules.js';
import {decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {readFormalConceptDefinition} from '../src/formal-concept-definition.js';
import {readMetaLanguageArchiveText,readMetaLanguageArchiveBinary} from '../src/meta-language-archive.js';
const fixture=readProjectData(new URL('../../parity/fixtures/source-definition-catalog.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.cases)test(row.name,()=>assert.equal(validateSourceDefinitionCatalogEntries(row.identifiers,row.assets,row.digests,row.bytes,row.roots),row.result));
test('catalog covers every actual complete source program with exact paired bytes',()=>{
 const entries=sourceDefinitionCatalog(),assets=readdirSync(new URL('../src/data/',import.meta.url)).filter(name=>/^[a-z][a-z0-9-]*-program\.lino$/u.test(name)).sort();
 assert.deepEqual(entries.map(entry=>entry.asset),assets);
 for(const entry of entries){const bytes=readFileSync(new URL('../src/data/'+entry.asset,import.meta.url));assert.equal(bytes.length,entry.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),entry.sha256);assert.deepEqual(bytes,readFileSync(new URL('../../rust/src/data/'+entry.asset,import.meta.url)));assert.equal(entry.definitionIdentity,entry.identifier+'@sha256:'+entry.sha256);}
 entries[0].asset='edited';assert.notEqual(sourceDefinitionCatalog()[0].asset,'edited');
});
test('unknown selectors and explicit resource limits do not load a substitute',()=>{
 assert.equal(sourceDefinitionEntry('array-classification-program'),undefined);
 assert.throws(()=>createSourceDefinitionProgramLinks('missing'),/unknown qualified program identity/u);
 for(const options of [{byteLimit:1},{nodeLimit:1},{byteLimit:0},{depthLimit:0},{nodeLimit:Infinity}])assert.throws(()=>createSourceDefinitionProgramLinks(fixture.selectedIdentifier,options),/limit/u);
 assert.equal(acceptSourceDefinitionCatalogHeader(1,fixture.model,0),false);
 assert.equal(acceptSourceDefinitionCatalogLimits(1,1,1),true);
 assert.equal(acceptSourceDefinitionCatalogLimits(NaN,1,1),false);
});
test('actual owned source definition is fresh, complete and selectable under the existing model',()=>{
 const first=createSourceDefinitionProgramLinks(fixture.selectedIdentifier),second=createSourceDefinitionProgramLinks(fixture.selectedIdentifier);
 assert.notEqual(first.network,second.network);assert.equal(first.root.asU64(),Number(sourceDefinitionEntry(fixture.selectedIdentifier).root));
 assert.equal(decodeTypedProgramLinks(first.network,first.root).declarations.has(fixture.selectedFunction),true);
 const definition=createSourceDefinitionFormalFunctionLinks(fixture.selectedIdentifier,fixture.selectedFunction);assert.ok(readFormalConceptDefinition(definition));
 assert.throws(()=>createSourceDefinitionFormalFunctionLinks(fixture.selectedIdentifier,'not a declaration'),/owned function declaration/u);
});
test('program and function commands retain complete archives in both codecs',()=>{
 for(const command of ['program','function'])for(const binary of [false,true]){
  const args=[command,fixture.selectedIdentifier,...(command==='function'?[fixture.selectedFunction]:[]),...(binary?['--binary']:[])];
  const output=runSourceDefinitionCommand(args);assert.equal(output.exitCode,0,output.stderr);
  const archive=binary?readMetaLanguageArchiveBinary(output.stdout):readMetaLanguageArchiveText(output.stdout);
  assert.equal(archive.entries.length,1);const entry=archive.entries[0];
  assert.equal(entry.kind,command==='function'?'formal concept definition':'typed program');
  if(command==='function')assert.ok(readFormalConceptDefinition(entry));else assert.ok(decodeTypedProgramLinks(entry.network,entry.root).declarations.has(fixture.selectedFunction));
 }
 for(const args of [[],['list','--binary'],['program'],['program',fixture.selectedIdentifier,'extra'],['list','--binary','--binary']])assert.equal(runSourceDefinitionCommand(args).exitCode,2);
});
