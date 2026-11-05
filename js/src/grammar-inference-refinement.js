// Keep prior model/grammar source graphs as flat archive groups. The complete
// algorithm is shared once, not recursively copied into predecessor envelopes.
import {inferRegularGrammar,readRegularGrammarInferenceLinks} from './grammar-inference.js';
import {createMetaLanguageArchive,readMetaLanguageArchive} from './meta-language-archive.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from './structured-data-links.js';
import {LinkId} from './primitives.js';
import {createSourceDefinitionProgramLinks} from './source-definition-catalog.js';
import {createSurfaceReferenceFacts} from './translation/surface-reference-facts.js';
import {acceptRegularGrammarRefinementCount,acceptRegularGrammarRefinementJournal,readRegularGrammarRefinementEntryName,readRegularGrammarRefinementIdentifier,readRegularGrammarRefinementResourceLimit,validateRegularGrammarRefinementHistory,acceptRegularGrammarRefinementIdentifier} from './grammar-inference-refinement-rules.js';
const fail=message=>{throw new TypeError('grammar inference refinement: '+message);};
const modelIdentifier='regular-discovery-model',grammarIdentifier='regular-discovery-grammar',algorithmIdentifier='regular-discovery-algorithm',journalIdentifier='regular-discovery-refinement-journal';
const refinementIdentifier='regular-discovery-refinement-algorithm';
const refinementDefinition=()=>createSourceDefinitionProgramLinks('meta-language/source-program/grammar-inference-refinement-program');
const limits={nodeLimit:readRegularGrammarRefinementResourceLimit('nodes'),referenceLimit:readRegularGrammarRefinementResourceLimit('references'),byteLimit:readRegularGrammarRefinementResourceLimit('bytes')};
function select(entries,identifier,kind){const entry=entries.find(entry=>entry.identifier===identifier);if(!entry||entry.kind!==kind)fail('missing '+identifier);return entry;}
function model(entry){return decodeStructuredDataLinks(entry.network,entry.root);}
function base(entries,modelEntry,grammarEntry){
 const archive=createMetaLanguageArchive([{...modelEntry,identifier:modelIdentifier},select(entries,algorithmIdentifier,'typed program'),{...grammarEntry,identifier:grammarIdentifier}]);
 return readRegularGrammarInferenceLinks(archive.network,archive.root);
}
function journalOf(entries){
 const entry=entries.find(entry=>entry.identifier===journalIdentifier);
 if(!entry)return {schemaVersion:1,model:'labelled regular grammar refinement 1',steps:[]};
 if(entry.kind!=='grammar inference')fail('invalid journal entry role');
 const journal=model(entry);
 if(!journal||!acceptRegularGrammarRefinementJournal(journal.schemaVersion,journal.model)||!Array.isArray(journal.steps)||!acceptRegularGrammarRefinementCount(journal.steps.length))fail('unsupported refinement journal');
 if(Object.keys(journal).sort().join(',')!=='model,schemaVersion,steps')fail('unknown journal fields');
 return journal;
}
function readCheckpoint(network,root){
 const entries=readMetaLanguageArchive(network,root,limits).entries,journal=journalOf(entries);
 const hasJournal=entries.some(entry=>entry.identifier===journalIdentifier),storedDefinition=entries.find(entry=>entry.identifier===refinementIdentifier);
 if(hasJournal){const bundled=refinementDefinition();if(!storedDefinition||storedDefinition.kind!=='typed program'||!storedDefinition.root.equals(bundled.root)||storedDefinition.network.toLino()!==bundled.network.toLino())fail('the selected refinement definition differs from its versioned implementation');}
 else if(storedDefinition)fail('a refinement definition needs its actual journal');
 const currentModel=select(entries,modelIdentifier,'grammar inference'),currentGrammar=select(entries,grammarIdentifier,'discovered concepts');
 const models=[],history=[];
 const expected=new Set();
 for(let ordinal=0;ordinal<journal.steps.length;ordinal+=1){
  const modelName=readRegularGrammarRefinementEntryName(ordinal,'model'),grammarName=readRegularGrammarRefinementEntryName(ordinal,'grammar');expected.add(modelName);expected.add(grammarName);
  const storedModel=select(entries,modelName,'grammar inference'),storedGrammar=select(entries,grammarName,'discovered concepts');
  history.push([storedModel,storedGrammar]);models.push(model(storedModel));
 }
 if(entries.some(entry=>entry.identifier.startsWith('regular-discovery-history-')&&!expected.has(entry.identifier)))fail('unreferenced history entry');
 models.push(model(currentModel));
 const roots=[...models,...journal.steps],facts=createSurfaceReferenceFacts(roots);
 const identities=roots.map((value,index)=>facts.targets[facts.starts[facts.root]+index]);
 const decision=validateRegularGrammarRefinementHistory(facts.forms,facts.texts,facts.starts,facts.counts,facts.labels,facts.targets,identities.slice(0,models.length),identities.slice(models.length));
 if(decision!==1)fail('invalid refinement history (reason '+decision+')');
 for(const [storedModel,storedGrammar]of history)base(entries,storedModel,storedGrammar);
 const current=base(entries,currentModel,currentGrammar);
 return {entries,journal,current,model:models.at(-1),currentModel,currentGrammar};
}
/** Recheck every stored model, decision table, grammar and observation prefix.
 * Historical graph groups and current graph are retained without source buffers. */
export function readRefinedRegularGrammarInferenceLinks(network,root){const value=readCheckpoint(network,root);return {...value.current,network,root:LinkId.from(root)};}
/** Append labelled counterexamples under the same selected learner/limits.
 * Multiplicities remain significant. Failure leaves the original graph intact.
 * This is deterministic relearning from the retained corpus, not a semantic
 * proof of an unknown language or an incremental complexity guarantee. */
export function refineRegularGrammarInference(network,root,positives=[],negatives=[],{identifier}={}){
 const previous=readCheckpoint(network,root),ordinal=previous.journal.steps.length;
 if(!acceptRegularGrammarRefinementCount(ordinal+1))fail('refinement history limit exceeded');
 if(!Array.isArray(positives)||!Array.isArray(negatives))fail('additions must be labelled example arrays');
 const nextIdentifier=identifier??readRegularGrammarRefinementIdentifier(previous.model.identifier,ordinal+1);
 if(typeof nextIdentifier!=='string'||!acceptRegularGrammarRefinementIdentifier(nextIdentifier,[previous.model.identifier,...previous.journal.steps.map(step=>step.identifier)]))fail('a new source-qualified refinement identity is required');
 const next=inferRegularGrammar([...previous.model.positives,...positives],[...previous.model.negatives,...negatives],{...previous.model.options,identifier:nextIdentifier,language:previous.model.language,edition:previous.model.edition});
 const nextEntries=readMetaLanguageArchive(next.network,next.root).entries;
 const journal={schemaVersion:1,model:'labelled regular grammar refinement 1',steps:[...previous.journal.steps,{identifier:previous.model.identifier,nextIdentifier,positives,negatives}]};
 const data=encodeStructuredDataLinks(journal);
 const reserved=new Set([modelIdentifier,grammarIdentifier,algorithmIdentifier,journalIdentifier,refinementIdentifier]);
 const entries=[...nextEntries,...previous.entries.filter(entry=>!reserved.has(entry.identifier)),
  {...previous.currentModel,identifier:readRegularGrammarRefinementEntryName(ordinal,'model')},
  {...previous.currentGrammar,identifier:readRegularGrammarRefinementEntryName(ordinal,'grammar')},
  {identifier:journalIdentifier,kind:'grammar inference',...data},
  previous.entries.find(entry=>entry.identifier===refinementIdentifier)??{identifier:refinementIdentifier,kind:'typed program',...refinementDefinition()}];
 const archive=createMetaLanguageArchive(entries,limits);
 return {...next,...archive};
}
