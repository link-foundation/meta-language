// All controls are deferred until the complete requirements draft is finished.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {inferRegularGrammar,acceptInferredGrammarSymbols} from '../src/grammar-inference.js';
import {refineRegularGrammarInference,readRefinedRegularGrammarInferenceLinks} from '../src/grammar-inference-refinement.js';
import {readMetaLanguageArchive,createMetaLanguageArchive,metaLanguageArchiveToText,metaLanguageArchiveToBinary,readMetaLanguageArchiveText,readMetaLanguageArchiveBinary} from '../src/meta-language-archive.js';
import {decodeStructuredDataLinks,encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {createDynamicMetaLanguage} from '../src/dynamic-meta-language.js';
import {acceptRegularGrammarRefinementCount,acceptRegularGrammarRefinementJournal,acceptRegularGrammarRefinementIdentifier} from '../src/grammar-inference-refinement-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/regular-grammar-refinement.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
test('journal profile retains finite history, actual versions and distinct source identities',()=>{
 for(const count of [0,1,8])assert.equal(acceptRegularGrammarRefinementCount(count),true);
 for(const count of [-1,0.5,9,NaN,Infinity])assert.equal(acceptRegularGrammarRefinementCount(count),false);
 assert.equal(acceptRegularGrammarRefinementJournal(1,fixture.model),true);assert.equal(acceptRegularGrammarRefinementJournal(2,fixture.model),false);
 assert.equal(acceptRegularGrammarRefinementIdentifier('new',['old']),true);assert.equal(acceptRegularGrammarRefinementIdentifier('old',['old']),false);
});
for(const row of fixture.cases)test(row.name,()=>{
 const initial=inferRegularGrammar(row.positives,row.negatives,row.options),snapshots=[initial];let current=initial,positives=[...row.positives],negatives=[...row.negatives];
 for(const [ordinal,step]of row.steps.entries()){
  const originalText=current.network.toLino();
  const resumed=refineRegularGrammarInference(current.network,current.root,step.positives,step.negatives);
  const direct=refineRegularGrammarInference(current.network,current.root,step.positives,step.negatives);
  assert.equal(direct.network.toLino(),resumed.network.toLino());assert.equal(current.network.toLino(),originalText);
  positives.push(...step.positives);negatives.push(...step.negatives);
  for(const loaded of [readMetaLanguageArchiveText(metaLanguageArchiveToText(resumed,fixture.archiveLimits),fixture.archiveLimits),readMetaLanguageArchiveBinary(metaLanguageArchiveToBinary(resumed,fixture.archiveLimits),fixture.archiveLimits)]){
   const model=readRefinedRegularGrammarInferenceLinks(loaded.network,loaded.root);assert.deepEqual(model.table,resumed.table);assert.deepEqual(model.grammar.normalized(),resumed.grammar.normalized());
   for(const input of positives)assert.equal(acceptInferredGrammarSymbols(model,input),true);
   for(const input of negatives)assert.equal(acceptInferredGrammarSymbols(model,input),false);
   assert.equal(loaded.entries.filter(entry=>entry.identifier==='regular-discovery-algorithm').length,1);
   assert.equal(loaded.entries.filter(entry=>entry.identifier==='regular-discovery-refinement-algorithm').length,1);
   for(let index=0;index<=ordinal;index+=1){const entry=loaded.entries.find(entry=>entry.identifier==='regular-discovery-history-'+index+'-model');const old=decodeStructuredDataLinks(entry.network,entry.root);assert.equal(old.identifier,snapshots[index].source.id);}
  }
  snapshots.push(resumed);current=resumed;
 }
 const language=createDynamicMetaLanguage([initial.source,current.source]);
 for(const [model,example]of [[initial,row.positives[0]],[current,positives.at(-1)]]){const parsed=language.parse(model.source.id,example.join(''));assert.equal(parsed.projection,'concrete syntax');assert.ok(parsed.tree);assert.ok(parsed.archive);}
 const before=current.network.toLino();
 assert.throws(()=>refineRegularGrammarInference(current.network,current.root,[],[],{identifier:initial.source.id}),/new source-qualified/u);
 assert.throws(()=>refineRegularGrammarInference(current.network,current.root,[],[row.positives[0]]),/contradictory/u);
 assert.equal(current.network.toLino(),before);
 const entries=readMetaLanguageArchive(current.network,current.root,fixture.archiveLimits).entries;
 const journalEntry=entries.find(entry=>entry.identifier==='regular-discovery-refinement-journal');
 const journal=decodeStructuredDataLinks(journalEntry.network,journalEntry.root);journal.steps[0].positives=[];
 const data=encodeStructuredDataLinks(journal),changed=createMetaLanguageArchive(entries.map(entry=>entry===journalEntry?{...entry,...data}:entry),fixture.archiveLimits);
 assert.throws(()=>readRefinedRegularGrammarInferenceLinks(changed.network,changed.root),/invalid refinement history/u);
 const missing=createMetaLanguageArchive(entries.filter(entry=>entry.identifier!=='regular-discovery-history-0-grammar'),fixture.archiveLimits);
 assert.throws(()=>readRefinedRegularGrammarInferenceLinks(missing.network,missing.root),/missing regular-discovery-history/u);
 const missingDefinition=createMetaLanguageArchive(entries.filter(entry=>entry.identifier!=='regular-discovery-refinement-algorithm'),fixture.archiveLimits);
 assert.throws(()=>readRefinedRegularGrammarInferenceLinks(missingDefinition.network,missingDefinition.root),/selected refinement definition/u);
});
