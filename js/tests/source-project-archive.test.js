// Deferred complete source graph state, original bytes and snapshot extension.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../src/project-data-files.js';
import {createSourceProjectArchive,readSourceProjectArchive,extendSourceProjectArchive} from '../src/source-project-archive.js';
import {readMetaLanguageArchive,createMetaLanguageArchive} from '../src/meta-language-archive.js';
import {canonicalLanguageName} from '../src/language-catalog.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {selectSourceProjectSyntaxRootPositions} from '../src/source-project-archive-rules.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
const fixture=readProjectData(new URL('../../parity/fixtures/source-project-archive.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const originals=inputs=>[...inputs].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0).map(file=>({path:file.path,language:canonicalLanguageName(file.language),source:file.source}));
for(const row of fixture.cases)test('complete source snapshot: '+row.name,()=>{
 const archive=createSourceProjectArchive(row.inputs,row.options),baseline=readSourceProjectArchive(archive.network,archive.root,row.options);
 assert.deepEqual(baseline.sources.map(({path,language,source})=>({path,language,source})),originals(row.inputs));
 for(const file of baseline.sources){assert.equal(file.network.reconstructText(),file.source);assert.equal(file.projection,row.options.projection??'ConcreteSyntax');assert.deepEqual(file.view,file.network.projectedLinks(file.projection));if(row.unclean)assert.equal(file.network.verifyFullMatch().isClean(),false);}
 for(const network of [LinkNetwork.fromLino(archive.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(archive.network))]){
  const replay=readSourceProjectArchive(network,archive.root,row.options);assert.deepEqual(replay.directory,baseline.directory);
  for(const [index,file]of replay.sources.entries()){const prior=baseline.sources[index];assert.equal(file.source,prior.source);assert.equal(file.root.asU64(),prior.root.asU64());assert.equal(file.network.toLino(),prior.network.toLino());assert.equal(file.network._nextId,prior.network._nextId);assert.deepEqual([...file.network._terms],[...prior.network._terms]);assert.deepEqual([...file.network._conceptSyntax],[...prior.network._conceptSyntax]);}
 }
 const abstract=readSourceProjectArchive(archive.network,archive.root,{...row.options,projection:'AbstractSyntax'});for(const [index,file]of abstract.sources.entries()){assert.equal(file.network.toLino(),baseline.sources[index].network.toLino());assert.equal(file.projection,'AbstractSyntax');}
});
for(const row of fixture.extensions)test('complete source extension: '+row.name,()=>{
 let archive=createSourceProjectArchive(row.initial,row.options);
 // Independent selected syntax and allocator state must survive append even
 // when neither state can be reconstructed by reparsing the original text.
 const entries=readMetaLanguageArchive(archive.network,archive.root).entries;
 const source=entries.find(entry=>entry.kind==='concrete syntax');source.network._conceptSyntax.set('snapshot marker\0JavaScript','preserved selected syntax');source.network._nextId+=11;
 archive=createMetaLanguageArchive(entries);
 const before=readSourceProjectArchive(archive.network,archive.root,row.options),extended=extendSourceProjectArchive(archive.network,archive.root,row.added,row.options),after=readSourceProjectArchive(extended.network,extended.root,row.options);
 assert.deepEqual(after.sources.map(({path,language,source})=>({path,language,source})),originals([...row.initial,...row.added]));
 for(const prior of before.sources){const actual=after.sources.find(file=>file.path===prior.path);assert.equal(actual.network.toLino(),prior.network.toLino());assert.equal(actual.network._nextId,prior.network._nextId);assert.deepEqual([...actual.network._conceptSyntax],[...prior.network._conceptSyntax]);}
 assert.throws(()=>extendSourceProjectArchive(archive.network,archive.root,row.initial,row.options),/cannot replace an existing source/);
 assert.equal(readSourceProjectArchive(archive.network,archive.root,row.options).sources.length,row.initial.length);
});
for(const row of fixture.refusals)test('explicit source snapshot refusal: '+row.name,()=>assert.throws(()=>createSourceProjectArchive(row.inputs,row.options),error=>error.message.includes(row.message)));
for(const [index,row]of fixture.rootCases.entries())test('complete original root selection '+index,()=>assert.deepEqual(selectSourceProjectSyntaxRootPositions(row.kinds,row.identifiers,row.firsts,row.counts,row.targets),row.expected));
test('source archive definitions remain editable through both codecs and every target',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/source-project-archive-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
