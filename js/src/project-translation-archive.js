// Actual source/data/graph transport. Admission rules are shared source;
// complete CST storage and source-free reconstruction reuse the common archive.
import {createHash} from 'node:crypto';
import {draftProjectTranslation,projectTranslationDraftToLinks} from './project-translation.js';
import {createSourceProjectArchive,readSourceProjectArchive,extendSourceProjectArchive} from './source-project-archive.js';
import {createMetaLanguageArchive,readMetaLanguageArchive} from './meta-language-archive.js';
import {selfTranslationLanguage} from './self-translation.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from './structured-data-links.js';
import {readProjectTranslationArchiveVocabulary as vocabulary,readProjectTranslationArchiveSourceLanguage,readProjectTranslationArchiveHeaderError,readProjectTranslationArchiveSourceError,readProjectTranslationArchiveProductError,readProjectTranslationArchiveBindingError,readProjectTranslationArchiveConfigurationError,readProjectTranslationArchiveImportError} from './project-translation-archive-rules.js';
const fail=message=>{throw new TypeError('project translation archive: '+message);};
const digest=source=>createHash('sha256').update(source,'utf8').digest('hex');
const limits=options=>({nodeLimit:options.nodeLimit??1000000,referenceLimit:options.referenceLimit??4000000,byteLimit:options.byteLimit??128*1024*1024});
function packProjectTranslationArchive(inputs,options,draft,sources){
 const packet=projectTranslationDraftToLinks(draft,options);
 const inventory=new Map(draft.inventory.map(file=>[file.identifier,file])),originals=new Map(inputs.map(file=>[file.identifier,file]));
 const configuration=encodeStructuredDataLinks({schemaVersion:1,options,modules:draft.inventory.map(file=>({identifier:file.identifier,language:file.language,imports:file.imports,sourceIssues:file.sourceIssues,options:originals.get(file.identifier)?.options??{}}))},options);
 sources??=createSourceProjectArchive(inputs.map(row=>({path:row.identifier,source:row.source,language:readProjectTranslationArchiveSourceLanguage(inventory.get(row.identifier)?.language??'',row.identifier)})),options);
 return createMetaLanguageArchive([
  {identifier:vocabulary('configuration'),kind:'language configuration',network:configuration.network,root:configuration.root},
  {identifier:vocabulary('draft'),kind:'language configuration',network:packet.network,root:packet.root},
  {identifier:vocabulary('sources'),kind:'concrete syntax',network:sources.network,root:sources.root}
 ],limits(options));
}
export function createProjectTranslationArchive(inputs,options={}){return packProjectTranslationArchive(inputs,options,draftProjectTranslation(inputs,options));}

/** No source files, parser, compiler or current grammar catalog are consulted.
 * This validates preservation and source membership, not product semantics.
 */
export function readProjectTranslationArchive(network,root,options={}){
 const archive=readMetaLanguageArchive(network,root,limits(options));
 if(archive.entries.length!==3)fail('missing or foreign project translation entry');
 const configurationEntry=archive.entries.find(entry=>entry.identifier===vocabulary('configuration'));
 const draftEntry=archive.entries.find(entry=>entry.identifier===vocabulary('draft')),sourceEntry=archive.entries.find(entry=>entry.identifier===vocabulary('sources'));
 if(configurationEntry?.kind!=='language configuration'||draftEntry?.kind!=='language configuration'||sourceEntry?.kind!=='concrete syntax')fail('missing or foreign project translation entry');
 const configuration=decodeStructuredDataLinks(configurationEntry.network,configurationEntry.root,options);
 const draft=decodeStructuredDataLinks(draftEntry.network,draftEntry.root,options),concrete=readSourceProjectArchive(sourceEntry.network,sourceEntry.root,options);
 if(!draft||typeof draft!=='object'||!Array.isArray(draft.inventory)||!Array.isArray(draft.modules)||typeof draft.schemaVersion!=='number'||typeof draft.interpretation!=='string')fail('invalid project translation draft');
 const error=readProjectTranslationArchiveHeaderError(draft.schemaVersion,draft.interpretation,draft.inventory.length,draft.modules.length,concrete.sources.length);if(error)fail(error);
 if(!configuration||typeof configuration!=='object'||!Array.isArray(configuration.modules)||typeof configuration.schemaVersion!=='number'||!configuration.options||typeof configuration.options!=='object'||Array.isArray(configuration.options)||typeof configuration.options.targetLanguage!=='string'||typeof draft.targetLanguage!=='string')fail('invalid project translation configuration');
 const configurationError=readProjectTranslationArchiveConfigurationError(configuration.schemaVersion,configuration.modules.length,draft.inventory.length,draft.targetLanguage,selfTranslationLanguage(configuration.options.targetLanguage)??'');if(configurationError)fail(configurationError);
 const identifiers=new Set(concrete.sources.map(source=>source.path));
 for(let index=0;index<draft.inventory.length;index+=1){const file=draft.inventory[index],module=draft.modules[index],source=concrete.sources[index],settings=configuration.modules[index];
  if(!settings||settings.identifier!==file?.identifier||settings.language!==file?.language||!settings.options||typeof settings.options!=='object'||Array.isArray(settings.options)||!Array.isArray(settings.imports)||!Array.isArray(settings.sourceIssues)||module?.targetLanguage!==draft.targetLanguage)fail('project translation configuration changed');
  if(!file||!module||typeof file.identifier!=='string'||typeof file.language!=='string'||typeof file.sourceSha256!=='string'||typeof file.sourceBytes!=='number'||typeof module.identifier!=='string'||typeof module.sourceLanguage!=='string'||typeof module.sourceSha256!=='string'||typeof module.sourceBytes!=='number'||!Array.isArray(module.gaps)||!Array.isArray(file.sourceIssues)||!Array.isArray(module.sourceIssues)||!Array.isArray(file.imports)||!Array.isArray(module.imports))fail('invalid project source directory');
  for(const [identifier,language,bytes,sha]of [[file.identifier,file.language,file.sourceBytes,file.sourceSha256],[module.identifier,module.sourceLanguage,module.sourceBytes,module.sourceSha256]]){const error=readProjectTranslationArchiveSourceError(identifier,language,bytes,sha,source.path,source.language,source.bytes,source.sha256);if(error)fail(error);}
  const gapIssues=module.gaps.filter(gap=>gap?.diagnostic?.name==='SourceInventoryGap').map(gap=>gap.diagnostic.message);
  if([...file.sourceIssues,...module.sourceIssues,...gapIssues,...settings.sourceIssues].some(issue=>typeof issue!=='string'||!issue.length))fail('invalid project source directory');
  for(const imports of [file.imports,module.imports,settings.imports])if(imports.some(row=>!row||typeof row!=='object'||Object.keys(row).length!==2||typeof row.specifier!=='string'||typeof row.identifier!=='string'))fail('invalid project source directory');
  const importError=readProjectTranslationArchiveImportError(file.imports.map(row=>row.specifier),file.imports.map(row=>identifiers.has(row.identifier)));if(importError)fail(importError);
  const bindingError=readProjectTranslationArchiveBindingError(file.sourceIssues,module.sourceIssues,gapIssues,file.imports.map(row=>row.specifier),file.imports.map(row=>row.identifier),module.imports.map(row=>row.specifier),module.imports.map(row=>row.identifier));if(bindingError)fail(bindingError);
  const settingsError=readProjectTranslationArchiveBindingError(file.sourceIssues,settings.sourceIssues,gapIssues,file.imports.map(row=>row.specifier),file.imports.map(row=>row.identifier),settings.imports.map(row=>row.specifier),settings.imports.map(row=>row.identifier));if(settingsError)fail(settingsError);
  const code=module.draftCode,emitted=module.emittedCode;
  if(code!==null&&typeof code!=='string'||emitted!==null&&typeof emitted!=='string')fail('invalid project product');
  const error=readProjectTranslationArchiveProductError(code!==null,emitted!==null,module.gaps.length,emitted===code,code===null?module.emittedSha256===null:typeof module.emittedSha256==='string'&&digest(code)===module.emittedSha256);if(error)fail(error);
 }
 return {draft,configuration,directory:concrete.directory,sources:concrete.sources,sourceArchive:{network:sourceEntry.network,root:sourceEntry.root}};
}

/** Rebuild portable translator inputs from complete stored trees and settings.
 * This performs no source parse, translation, compilation or product execution.
 */
function restoredProjectTranslationInputs(restored){
 const sources=new Map(restored.sources.map(source=>[source.path,source.source]));
 return {inputs:restored.configuration.modules.map(file=>({identifier:file.identifier,language:file.language,source:sources.get(file.identifier),imports:Object.fromEntries(file.imports.map(row=>[row.specifier,row.identifier])),sourceIssues:[...file.sourceIssues],options:structuredClone(file.options)})),options:structuredClone(restored.configuration.options)};
}

export function restoreProjectTranslationArchiveInputs(network,root,options={}){return restoredProjectTranslationInputs(readProjectTranslationArchive(network,root,options));}

/** Persistent append. A fresh translation may parse sources; existing stored
 * concrete graphs retain their exact original identities and grammar state.
 */
export function extendProjectTranslationArchive(network,root,additions,options={}){
 if(!options||typeof options!=='object'||Array.isArray(options)||!Array.isArray(additions)||!additions.length)fail('expected nonempty added source inputs and resource options');
 const previous=readProjectTranslationArchive(network,root,options),restored=restoredProjectTranslationInputs(previous),selected={...restored.options,...options},inputs=[...restored.inputs,...additions];
 const draft=draftProjectTranslation(inputs,selected),inventory=new Map(draft.inventory.map(file=>[file.identifier,file]));
 const sources=extendSourceProjectArchive(previous.sourceArchive.network,previous.sourceArchive.root,additions.map(row=>({path:row.identifier,source:row.source,language:readProjectTranslationArchiveSourceLanguage(inventory.get(row.identifier)?.language??'',row.identifier)})),selected);
 return packProjectTranslationArchive(inputs,selected,draft,sources);
}
