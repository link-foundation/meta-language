// Deferred source-free graph, recursive CST and refusal controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {spawnSync} from 'node:child_process';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino,encodeStructuredDataLinks,decodeStructuredDataLinks} from '../src/structured-data-links.js';
import {Grammar} from '../src/grammar.js';
import {createDiscoveredConceptLinks} from '../src/discovered-concept-links.js';
import {createGrammarCompositionBatchLinks,readGrammarCompositionBatchLinks} from '../src/grammar-composition-batch.js';
import {createMetaLanguageArchive,readMetaLanguageArchive} from '../src/meta-language-archive.js';
import {globalConceptSpaceToLinks,createGlobalConceptSpaceParser} from '../src/global-concept-space.js';
import {checkGrammarSelectionMappedTree} from '../src/grammar-runtime.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readGrammarCompositionBatchIssue,readGrammarCompositionBatchCountIssue,readGrammarCompositionBatchTextIssue} from '../src/grammar-composition-batch-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/grammar-composition-batch.lino',import.meta.url));
const sources=()=>fixture.sources.map(source=>({...source,grammar:new Grammar(source.grammar.start,source.grammar.rules.map(rule=>[rule.name,rule]),source.grammar.sourceFormat,source.grammar.declarations)}));
const original=()=>createDiscoveredConceptLinks(sources());
function graph(entry){return createMetaLanguageArchive([{...entry,identifier:'exact-original'}]).network.toLino();}
test('whole inventory retains exact original graphs and recursive full CSTs through both codecs',()=>{
 const before=original(),originalBytes=before.network.toLino(),archive=createGrammarCompositionBatchLinks(before.network,before.root,fixture.policies);
 assert.equal(before.network.toLino(),originalBytes);
 for(const network of [LinkNetwork.fromLino(archive.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(archive.network))]){
  const loaded=readGrammarCompositionBatchLinks(network,archive.root);assert.deepEqual(loaded.sources.map(source=>source.id),fixture.sources.map(source=>source.id));assert.deepEqual(loaded.policies,fixture.policies);
  assert.equal(graph(loaded.entries.find(entry=>entry.identifier==='grammar-composition-original-definitions')),graph({...before,kind:'discovered concepts'}));
  assert.deepEqual(loaded.sources[2].grammar.normalized(),sources()[2].grammar.normalized());
  const outcome=createGlobalConceptSpaceParser(globalConceptSpaceToLinks(loaded.space),'quoted-a').parseTree(fixture.input);assert.equal(outcome.ok,true);assert.equal(outcome.sourcePlanes.length,fixture.expectedPlanes);checkGrammarSelectionMappedTree({tree:outcome.tree,sourcePlanes:outcome.sourcePlanes});
 }
});
for(const row of fixture.refusals)test(row.name,()=>{const before=original();assert.throws(()=>createGrammarCompositionBatchLinks(before.network,before.root,row.policies),error=>error.message==='grammar composition batch: '+row.reason);});
for(const role of ['configuration','algorithm','current'])test('changed '+role+' cannot authenticate the batch',()=>{
 const before=original(),archive=createGrammarCompositionBatchLinks(before.network,before.root,fixture.policies),entries=readMetaLanguageArchive(archive.network,archive.root).entries;
 const selected=entries.find(entry=>entry.identifier==='grammar-composition-'+(role==='configuration'?'batch-configuration':role==='algorithm'?'batch-algorithm':'current-definitions'));
 if(role==='configuration'){const data=decodeStructuredDataLinks(selected.network,selected.root);data.policies[0].composition=data.policies[0].composition.replace('any-grammar','all-grammar');const encoded=encodeStructuredDataLinks(data);selected.network=encoded.network;selected.root=encoded.root;}
 else {const replacement=role==='current'?entries.find(entry=>entry.identifier==='grammar-composition-original-definitions'):entries.find(entry=>entry.kind==='language configuration');selected.network=replacement.network;selected.root=replacement.root;}
 const changed=createMetaLanguageArchive(entries);assert.throws(()=>readGrammarCompositionBatchLinks(changed.network,changed.root));
});
for(const row of fixture.decisions)test('actual batch decision: '+row.name,()=>assert.equal(readGrammarCompositionBatchIssue(row.sources,row.owners,row.names,row.kinds,row.policySources,row.policyRules,row.compositions),row.expected));
test('policy accessors are refused before they execute',()=>{const before=original();let calls=0;const row={rule:'quoted',composition:fixture.policies[0].composition};Object.defineProperty(row,'source',{enumerable:true,get(){calls+=1;return 'quoted-a';}});assert.throws(()=>createGrammarCompositionBatchLinks(before.network,before.root,[row]));assert.equal(calls,0);});
test('the complete cross-runtime producer is a real linked graph',()=>{const produced=spawnSync(process.execPath,['--max-old-space-size=352','../experiments/grammar-composition/batch.mjs'],{cwd:new URL('../',import.meta.url),encoding:'utf8',maxBuffer:16*1024*1024});assert.equal(produced.status,0,produced.stderr);const product=readStructuredDataLino(produced.stdout),network=LinkNetwork.fromLino(product.links);assert.equal(readGrammarCompositionBatchLinks(network,product.root).sources.length,fixture.sources.length);checkGrammarSelectionMappedTree(product.bundle);});

for(const row of fixture.resources)test('batch preflight bounds: '+JSON.stringify(row),()=>{assert.equal(readGrammarCompositionBatchCountIssue(row.sources,row.rules,row.policies),row.countIssue);assert.equal(readGrammarCompositionBatchTextIssue(row.units),row.textIssue);});
