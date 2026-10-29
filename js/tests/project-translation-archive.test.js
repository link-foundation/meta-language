// Deferred public controls; the author syntax-parses this file without import.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {createProjectTranslationArchive,readProjectTranslationArchive} from '../src/project-translation-archive.js';
import {runCommandLine} from '../src/cli.js';
import {LinkNetwork} from '../src/network.js';
import {LinkType} from '../src/primitives.js';
import {encodeStructuredDataLinks,structuredDataToLino} from '../src/structured-data-links.js';
import {createMetaLanguageArchive,readMetaLanguageArchive,readMetaLanguageArchiveText,readMetaLanguageArchiveBinary} from '../src/meta-language-archive.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/project-translation-archive-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/project-translation-archive.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.node);
for(const row of fixture.grammarProfiles)assert.equal(createHash('sha256').update(readFileSync(new URL('../../'+row.path,import.meta.url))).digest('hex'),row.sha256,'uncovered native grammar revision '+row.path);
for(const row of fixture.vocabulary)test('archive vocabulary '+row.role,()=>assert.equal(rules.readProjectTranslationArchiveVocabulary(row.role),row.expected));
for(const [index,row]of fixture.languages.entries())test('archive grammar profile '+index,()=>assert.equal(rules.readProjectTranslationArchiveSourceLanguage(row.language,row.path),row.expected));
for(const [index,row]of fixture.headers.entries())test('archive header '+index,()=>assert.equal(rules.readProjectTranslationArchiveHeaderError(row.version,row.interpretation,row.inventory,row.modules,row.sources),row.expected));
for(const [index,row]of fixture.sources.entries())test('archive source membership '+index,()=>assert.equal(rules.readProjectTranslationArchiveSourceError(row.identifier,row.language,row.bytes,row.digest,row.path,row.actualLanguage,row.actualBytes,row.actualDigest),row.expected));
for(const [index,row]of fixture.products.entries())test('archive product admission '+index,()=>assert.equal(rules.readProjectTranslationArchiveProductError(row.draft,row.executable,row.gaps,row.identical,row.digest),row.expected));
for(const [index,row]of fixture.bindings.entries())test('archive ordered bindings '+index,()=>assert.equal(rules.readProjectTranslationArchiveBindingError(row.issues,row.actualIssues,row.gapIssues,row.specifiers,row.providers,row.actualSpecifiers,row.actualProviders),row.expected));
for(const [index,row]of fixture.commands.entries())test('archive arguments '+index,()=>assert.deepEqual(rules.planProjectTranslationArchiveCommand(row.args),row.expected));
for(const [index,row]of fixture.configurations.entries())test('archive configuration '+index,()=>assert.equal(rules.readProjectTranslationArchiveConfigurationError(row.version,row.count,row.inventory,row.target,row.selected),row.expected));
test('whole project archives reload every complete original from both codecs without a parser',()=>{
 const packet=createProjectTranslationArchive(fixture.inputs,{targetLanguage:'JavaScript'}),networks=[packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];let reference,configuration;const sourceBytes=new Map();
 for(const network of networks){let restored;const parse=LinkNetwork.parse;try{LinkNetwork.parse=()=>{throw new Error('source-free reconstruction must not reparse');};restored=readProjectTranslationArchive(network,packet.root);}finally{LinkNetwork.parse=parse;}
  assert.equal(restored.sources.length,fixture.inputs.length);if(reference)assert.deepEqual(restored.draft,reference);reference=restored.draft;if(configuration)assert.deepEqual(restored.configuration,configuration);configuration=restored.configuration;assert.equal(configuration.options.targetLanguage,'JavaScript');assert.deepEqual(configuration.modules.find(row=>row.identifier==='src/b.js').options.moduleDirectory,['src']);
  for(const row of fixture.inputs){const source=restored.sources.find(file=>file.path===row.identifier);const bytes=encodeLinkNetworkBinary(source.network);if(sourceBytes.has(row.identifier))assert.deepEqual(bytes,sourceBytes.get(row.identifier));else sourceBytes.set(row.identifier,bytes);assert.equal(source.source,row.source);assert.equal(source.network.reconstructText(),row.source);assert.equal(source.projection,'ConcreteSyntax');assert.equal(source.language,row.identifier.endsWith('.tsx')?'TSX':row.language);assert.ok(source.view.length>0);assert.ok(source.network.links().some(link=>link.metadata().linkType===LinkType.Syntax));}
  const unresolved=restored.draft.modules.find(module=>module.identifier==='src/unresolved.js');assert.equal(unresolved.emittedCode,null);assert.deepEqual(unresolved.sourceIssues,fixture.inputs.at(-1).sourceIssues);
 }
 const abstract=readProjectTranslationArchive(packet.network,packet.root,{projection:'AbstractSyntax'});for(const source of abstract.sources){assert.equal(source.projection,'AbstractSyntax');assert.equal(source.network.reconstructText(),fixture.inputs.find(row=>row.identifier===source.path).source);}
});
test('source membership and retained failure tampering refuse instead of blessing products',()=>{
 const packet=createProjectTranslationArchive(fixture.inputs,{targetLanguage:'JavaScript'}),view=readProjectTranslationArchive(packet.network,packet.root),entries=readMetaLanguageArchive(packet.network,packet.root).entries;
 const replace=draft=>{const data=encodeStructuredDataLinks(draft);return createMetaLanguageArchive(entries.map(entry=>entry.identifier===rules.readProjectTranslationArchiveVocabulary('draft')?{...entry,network:data.network,root:data.root}:entry));};
 for(const change of [draft=>{draft.inventory[0].sourceSha256='0'.repeat(64);},draft=>{draft.modules[0].draftCode='changed product';},draft=>{draft.modules.find(module=>module.identifier==='src/unresolved.js').gaps=[];}]){const draft=structuredClone(view.draft);change(draft);const forged=replace(draft);assert.throws(()=>readProjectTranslationArchive(forged.network,forged.root),/changed|cannot be executable/u);}
 const missing=createMetaLanguageArchive(entries.slice(0,1));assert.throws(()=>readProjectTranslationArchive(missing.network,missing.root),/missing or foreign/u);
 assert.throws(()=>readProjectTranslationArchive(packet.network,packet.root,{nodeLimit:1}),/limit/u);
});
test('public project CLI retains complete concrete graphs in text and binary',()=>{
 const input=structuredDataToLino({inputs:fixture.inputs});for(const binary of [false,true]){const result=runCommandLine(['project-translation','inputs.lino','--to','JavaScript',...(binary?['--binary']:[])],{readFile:path=>{assert.equal(path,'inputs.lino');return input;}});assert.equal(result.exitCode,0,result.stderr);const packet=binary?readMetaLanguageArchiveBinary(result.stdout):readMetaLanguageArchiveText(result.stdout);assert.equal(readProjectTranslationArchive(packet.network,packet.root).sources.length,fixture.inputs.length);}
 assert.equal(runCommandLine(['project-translation','--help']).exitCode,0);assert.equal(runCommandLine(['project-translation']).exitCode,2);
});

test('configuration target changes retain an explicit refusal',()=>{
 const packet=createProjectTranslationArchive(fixture.inputs,{targetLanguage:'JavaScript'}),view=readProjectTranslationArchive(packet.network,packet.root),entries=readMetaLanguageArchive(packet.network,packet.root).entries;
 const changed=structuredClone(view.configuration);changed.options.targetLanguage='Rust';const replacement=encodeStructuredDataLinks(changed),forged=createMetaLanguageArchive(entries.map(entry=>entry.identifier===rules.readProjectTranslationArchiveVocabulary('configuration')?{...entry,network:replacement.network,root:replacement.root}:entry));
 assert.throws(()=>readProjectTranslationArchive(forged.network,forged.root),/configuration changed/u);
});
