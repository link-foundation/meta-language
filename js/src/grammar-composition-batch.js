// Batch authoring over a complete archived grammar inventory. The existing
// decorator engine and global verifier retain their actual responsibilities.
import {DecoratorSet} from './decorators.js';
import {decorateGrammar} from './grammar-decorators.js';
import {createDiscoveredConceptLinks,readDiscoveredConceptLinks} from './discovered-concept-links.js';
import {createMetaLanguageArchive,readMetaLanguageArchive} from './meta-language-archive.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from './structured-data-links.js';
import {createSourceDefinitionProgramLinks} from './source-definition-catalog.js';
import {readGrammarCompositionBatchIssue,readGrammarCompositionBatchFieldOrder,readGrammarCompositionBatchCountIssue,readGrammarCompositionBatchTextIssue,
 readGrammarCompositionBatchEntryName,acceptGrammarCompositionBatchConfiguration} from './grammar-composition-batch-rules.js';

const model='source-qualified grammar composition batch 1';
const fail=message=>{throw new TypeError('grammar composition batch: '+message);};
const definition=()=>createSourceDefinitionProgramLinks('meta-language/source-program/grammar-composition-batch-program');
function fields(value,role){
 const expected=readGrammarCompositionBatchFieldOrder(role).split('|');
 if(!value||typeof value!=='object'||Array.isArray(value)||![Object.prototype,null].includes(Object.getPrototypeOf(value)))fail(role+' needs ordinary data');
 const keys=Reflect.ownKeys(value);if(keys.length!==expected.length||keys.some(key=>!expected.includes(key)))fail(role+' needs exactly its declared fields');
 const result={};for(const field of expected){const descriptor=Object.getOwnPropertyDescriptor(value,field);if(!descriptor||!Object.hasOwn(descriptor,'value')||!descriptor.enumerable)fail(role+' needs enumerable own data fields');result[field]=descriptor.value;}
 return result;
}
function policies(value){
 if(!Array.isArray(value)||Object.getPrototypeOf(value)!==Array.prototype||value.length>4096)fail('policies need a bounded dense array');
 const keys=Reflect.ownKeys(value);if(keys.length!==value.length+1||keys[keys.length-1]!=='length')fail('policies need exactly their indexed data fields');
 const result=[];
 for(let index=0;index<value.length;index+=1){
  const descriptor=Object.getOwnPropertyDescriptor(value,String(index));if(keys[index]!==String(index)||!descriptor||!Object.hasOwn(descriptor,'value')||!descriptor.enumerable)fail('policy indices need own data fields');
  const row=fields(descriptor.value,'policy');for(const field of ['source','rule','composition'])if(typeof row[field]!=='string'||!row[field].isWellFormed())fail('policy fields need Unicode scalar text');result.push(row);
 }
 return result;
}
function apply(sources,selected){
 const countIssue=readGrammarCompositionBatchCountIssue(sources.length,sources.reduce((count,source)=>count+source.grammar.rules.size,0),selected.length);if(countIssue)fail(countIssue);
 let units=0;
 for(const source of sources){units+=source.id.length;const issue=readGrammarCompositionBatchTextIssue(units);if(issue)fail(issue);}
 for(const source of sources)for(const rule of source.grammar.rules.values()){units+=source.id.length+1+rule.name.length;const issue=readGrammarCompositionBatchTextIssue(units);if(issue)fail(issue);}
 for(const row of selected){units+=row.source.length+1+row.rule.length+row.composition.length;const issue=readGrammarCompositionBatchTextIssue(units);if(issue)fail(issue);}
 const owners=[],names=[],kinds=[];
 for(const source of sources)for(const rule of source.grammar.rules.values()){owners.push(source.id);names.push(rule.name);kinds.push(rule.kind);}
 const issue=readGrammarCompositionBatchIssue(sources.map(source=>source.id),owners,names,kinds,selected.map(row=>row.source),selected.map(row=>row.rule),selected.map(row=>row.composition));if(issue)fail(issue);
 const grouped=new Map();for(const [index,row]of selected.entries()){
  const values=grouped.get(row.source)??[];values.push({id:'composition-batch-'+index,level:'grammar-rule',order:0,when:[['name',row.rule]],actions:[{op:'set',field:'composition',value:row.composition}]});grouped.set(row.source,values);
 }
 return sources.map(source=>({...source,grammar:decorateGrammar(source.grammar,new DecoratorSet(grouped.get(source.id)??[]))}));
}
const name=role=>readGrammarCompositionBatchEntryName(role);
function entry(role,kind,value){return{identifier:name(role),kind,network:value.network,root:value.root};}
function select(entries,role,kind){const value=entries.find(value=>value.identifier===name(role));if(!value||value.kind!==kind)fail('missing '+role+' entry');return value;}
function graphKey(value,options){return createMetaLanguageArchive([{...value,identifier:'composition-comparison',kind:'discovered concepts'}],options).network.toLino();}

/** Apply a complete batch without parsing input or changing the original graph.
 * The current concept space is recomputed by the existing verifier. Original
 * proofs remain in their complete predecessor graph, never transferred by name.
 */
export function createGrammarCompositionBatchLinks(network,root,requests,options={}){
 const original=readDiscoveredConceptLinks(network,root),selected=policies(requests);
 const current=createDiscoveredConceptLinks(apply(original.sources,selected));
 const configuration=encodeStructuredDataLinks({schemaVersion:1,model,policies:selected});
 return createMetaLanguageArchive([entry('original','discovered concepts',{network,root}),entry('current','discovered concepts',current),entry('configuration','language configuration',configuration),entry('algorithm','typed program',definition())],options);
}

/** Eager source-free reconstruction checks the actual batch against its entire
 * original context. No parser execution or equivalence declaration supplies it.
 */
export function readGrammarCompositionBatchLinks(network,root,options={}){
 const entries=readMetaLanguageArchive(network,root,options).entries;if(entries.length!==4)fail('the batch requires exactly its four entries');
 const originalEntry=select(entries,'original','discovered concepts'),currentEntry=select(entries,'current','discovered concepts'),configurationEntry=select(entries,'configuration','language configuration'),algorithm=select(entries,'algorithm','typed program');
 const original=readDiscoveredConceptLinks(originalEntry.network,originalEntry.root),current=readDiscoveredConceptLinks(currentEntry.network,currentEntry.root);
 const configuration=fields(decodeStructuredDataLinks(configurationEntry.network,configurationEntry.root),'configuration');if(!acceptGrammarCompositionBatchConfiguration(configuration.schemaVersion,configuration.model))fail('unsupported batch configuration');
 const selected=policies(configuration.policies),expected=createDiscoveredConceptLinks(apply(original.sources,selected));
 if(graphKey(currentEntry,options)!==graphKey(expected,options))fail('the current complete grammar graph differs from its actual composition');
 const expectedDefinition=definition();if(graphKey(algorithm,options)!==graphKey(expectedDefinition,options))fail('the batch decision definition differs from its selected implementation');
 return {current,sources:current.sources,space:current.space,original,policies:selected,entries};
}
