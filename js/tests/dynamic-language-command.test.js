// Deferred until the complete requirements code draft is in place.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {runDynamicLanguageCommand} from '../src/dynamic-language-cli.js';
import {DynamicMetaLanguage} from '../src/dynamic-meta-language.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {readRefinedRegularGrammarInferenceLinks} from '../src/grammar-inference-refinement.js';
import {readStructuralGrammarInferenceLinks} from '../src/grammar-structural-inference.js';
import {createMetaLanguageArchive,metaLanguageArchiveToBinary,metaLanguageArchiveToLinkCliStore,
 readMetaLanguageArchiveText,readMetaLanguageArchiveBinary,readMetaLanguageArchiveLinkCliStore} from '../src/meta-language-archive.js';
import {decodeStructuredDataLinks,readStructuredDataLino} from '../src/structured-data-links.js';
import {LinkNetwork} from '../src/network.js';
import {recordIssue195Observations} from './support/issue-195-observations.js';
const fixture=readProjectData(new URL('../../parity/fixtures/dynamic-language-command.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const encoder=new TextEncoder(),decoder=new TextDecoder('utf-8',{fatal:true});
function bytes(tree){
 if(tree.type==='missing')return [];
 if(tree.type==='token'||tree.type==='error')return tree.text===null?[...tree.hex.matchAll(/../gu)].map(([value])=>Number.parseInt(value,16)):[...encoder.encode(tree.text)];
 return (tree.children??[tree.root]).flatMap(bytes);
}
function concreteData(entry){
 const network=new LinkNetwork(),seen=new Set(),pending=[entry.root];
 for(let index=0;index<pending.length;index+=1){const id=pending[index];if(seen.has(id.asU64()))continue;seen.add(id.asU64());
  const link=entry.network.link(id);network.insertLinkWithOptionalId(id,link.references(),link.metadata().clone());pending.push(...link.references());}
 return decodeStructuredDataLinks(network,entry.root);
}
test('one shared command sequence retains concrete trees, recursive definitions and old contexts',context=>{
 const files=new Map(fixture.files.map(file=>[file.path,encoder.encode(file.text)]));
 const host={readFile:path=>decoder.decode(files.get(path)??assert.fail('missing fixture file '+path)),readBytes:path=>files.get(path)??assert.fail('missing fixture file '+path)};
 for(const step of fixture.steps){
  const checkpointBytes=step.archive&&step.args[0]==='install'?host.readBytes(step.args[1]):null;
  const output=runDynamicLanguageCommand(step.args,host);
  assert.equal(output.exitCode,step.exit,step.name+' '+output.stderr);
  if(step.error)assert.ok(output.stderr.includes(step.error),step.name);
  if(step.rejection)assert.ok(readStructuredDataLino(output.stderr).rejection,step.name);
  if(step.save)files.set(step.save,typeof output.stdout==='string'?encoder.encode(output.stdout):output.stdout);
  for(const path of step.remove??[])assert.ok(files.delete(path),step.name);
  if(step.inventory)assert.deepEqual(readStructuredDataLino(output.stdout).map(row=>row.identifier),step.inventory,step.name);
  if(step.grammar)assert.deepEqual(parseGrammarLinks(output.stdout).normalized(),parseGrammarLinks(host.readFile(step.grammar)).normalized(),step.name);
  if(step.archive){
   const archive=step.args.includes('--link-cli-store')?readMetaLanguageArchiveLinkCliStore(output.stdout)
    :step.binary?readMetaLanguageArchiveBinary(output.stdout):readMetaLanguageArchiveText(output.stdout);
   const session=new DynamicMetaLanguage(archive);
   if (checkpointBytes) {
    const original = step.args.includes('--input-link-cli-store') ? readMetaLanguageArchiveLinkCliStore(checkpointBytes)
      : step.args.includes('--input-binary') ? readMetaLanguageArchiveBinary(checkpointBytes)
      : readMetaLanguageArchiveText(decoder.decode(checkpointBytes));
    const saved = session.entries.filter(entry => entry.kind === 'grammar inference');
    assert.ok(saved.some(entry => entry.root.equals(original.root)
      && entry.network.toLino() === original.network.toLino()), step.name);
   }
   if (step.merge) {
    const report = session.grammarMerge(step.merge.result);
    assert.deepEqual(report.sources, step.merge.sources, step.name);
    const aliases = session.sources.filter(source => step.merge.sources.includes(source.id))
      .flatMap(source => [...source.grammar.rules.keys()].map(rule => `${source.id}:${rule}`));
    assert.deepEqual(report.sourceProductions.map(production => production.alias).sort(), aliases.sort(), step.name);
    assert.equal(session.entries.find(entry => entry.identifier === step.merge.checkpoint)?.kind, 'grammar inference', step.name);
   }
   if (step.oracle) {
    const checkpoint = session.entries.find(entry => entry.identifier === 'grammar inference:oracle-refined');
    assert.deepEqual(readRefinedRegularGrammarInferenceLinks(checkpoint.network, checkpoint.root).oracleObservations, step.oracle, step.name);
   }
   if (step.structuralOracle) {
    const checkpoint = session.entries.find(entry => entry.identifier === 'grammar inference:recursive-oracle-refined');
    assert.deepEqual(readStructuralGrammarInferenceLinks(checkpoint.network, checkpoint.root).oracleObservations, step.structuralOracle, step.name);
   }
   if(step.concrete){
    const entry=session.entries.find(entry=>entry.identifier===step.concrete);
    assert.equal(entry.kind,'concrete syntax',step.name);
    const concrete=concreteData(entry);
    assert.equal(concrete.projection,'concrete syntax',step.name);
    assert.deepEqual(bytes(concrete.outcome.tree),[...encoder.encode(step.text)],step.name);
   }
   if(step.abstract)assert.equal(session.entries.find(entry=>entry.identifier===step.abstract)?.kind,'abstract syntax',step.name);
  }
 }
 recordIssue195Observations({requirementId:'I195-GRAMMAR-INFERENCE-NATIVE-PARSER-ORACLE',suffix:'behavior',
  fixtureId:'grammar-discovery-parser-oracle',fixtureFile:'parity/fixtures/dynamic-language-command.lino',
  assertions:['nativeOracleChecksDeclaredAndIndependentWords','oracleObservationsReplaySourceFreeAfterRefinement','installedCandidateExecutesIndependentOracleCases'],
  testName:context.name});
 recordIssue195Observations({requirementId:'I195-GRAMMAR-INFERENCE-RECURSIVE-PARSER-ORACLE',suffix:'behavior',
  fixtureId:'grammar-discovery-recursive-parser-oracle',fixtureFile:'parity/fixtures/dynamic-language-command.lino',
  assertions:['recursiveOracleLabelsAndAnswersReplayed','recursiveOracleCheckpointSurvivesSourceFreeRefinement','installedRecursiveGrammarParsesDeeperAndRejectsNegative'],
  testName:context.name});
});
test('source-free reload needs no grammar source callback',()=>{
 const files=new Map(fixture.files.map(file=>[file.path,encoder.encode(file.text)]));
 const host={readFile:path=>decoder.decode(files.get(path)??assert.fail('unexpected source read '+path)),readBytes:path=>files.get(path)??assert.fail('unexpected source read '+path)};
 const prepare=runDynamicLanguageCommand(fixture.steps[0].args,host);
 assert.equal(prepare.exitCode,0,prepare.stderr);
 files.set('prepared.lino',encoder.encode(prepare.stdout));files.delete('first.lino');files.delete('configuration.lino');
 const parsed=runDynamicLanguageCommand(fixture.steps[3].args,host);
 assert.equal(parsed.exitCode,0,parsed.stderr);
 assert.ok(new DynamicMetaLanguage(readMetaLanguageArchiveText(parsed.stdout)).entries.some(entry=>entry.identifier==='original document'));
});

test('the native link-cli envelope retains ordered cycles and disconnected links',()=>{
 const network=new LinkNetwork();
 const root=network.insertLink(),target=network.insertLink(),detached=network.insertLink();
 network.link(root).setReferences([target,root,target]);
 network.link(target).setReferences([root]);
 network._terms.set('registered root',root);
 network._conceptSyntax.set('example\0Example language','π');
 network._nextId=81;
 const archive=createMetaLanguageArchive([
  {identifier:'cyclic graph',kind:'ontology',network,root},
  {identifier:'selected target',kind:'ontology',network,root:target},
 ]);
 archive.network._terms.set('outer root',archive.root);
 archive.network._conceptSyntax.set('archive\0Example language','λ');
 archive.network._nextId+=9;
 const packet=metaLanguageArchiveToLinkCliStore(archive),restored=readMetaLanguageArchiveLinkCliStore(packet);
 assert.deepEqual(metaLanguageArchiveToBinary(restored),metaLanguageArchiveToBinary(archive));
 assert.equal(restored.root.asU64(),archive.root.asU64());
 assert.equal(restored.entries[0].network,restored.entries[1].network);
 assert.deepEqual(restored.entries[0].network.link(root).references().map(id=>id.asU64()),[2,1,2]);
 assert.equal(restored.entries[0].network.link(detached).references().length,0);
 assert.equal(restored.entries[0].network.insertLink().asU64(),81);
 assert.throws(()=>readMetaLanguageArchiveLinkCliStore(packet.subarray(0,packet.length-1)));
 assert.throws(()=>readMetaLanguageArchiveLinkCliStore(packet,{byteLimit:packet.length-1}),/oversized link-cli store/u);
});
