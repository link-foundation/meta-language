// Reuse the complete graph archive for source-qualified default concrete trees.
import {createHash} from 'node:crypto';
import {LinkNetwork} from './network.js';
import {LinkMetadata,LinkType} from './primitives.js';
import {canonicalLanguageName} from './language-catalog.js';
import {createMetaLanguageArchive,readMetaLanguageArchive} from './meta-language-archive.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from './structured-data-links.js';
import {compareProjectTranslationIdentifiers} from './project-translation-rules.js';
import {readSourceProjectArchiveVocabulary as vocabulary,acceptSourceProjectPath,acceptSourceProjectSyntaxProjection,selectSourceProjectSyntaxRootPositions,readSourceProjectDirectoryError,readSourceProjectExtensionError} from './source-project-archive-rules.js';
const fail=message=>{throw new TypeError('source project archive: '+message);};
const digest=source=>createHash('sha256').update(source,'utf8').digest('hex');
function bound(options,name,fallback){const value=options[name]??fallback;if(!Number.isSafeInteger(value)||value<1)fail('invalid '+name);return value;}
function limits(options){if(!options||typeof options!=='object'||Array.isArray(options))fail('expected a resource options record');return {nodeLimit:bound(options,'nodeLimit',1000000),referenceLimit:bound(options,'referenceLimit',4000000),byteLimit:bound(options,'byteLimit',128*1024*1024)};}

/** Create complete default CST snapshots for a supplied catalog-language
 * inventory. Recovery information is archived, never relabeled as success.
 * Selecting an AST view retains its complete concrete graph and provenance.
 */
export function createSourceProjectArchive(inputs,options={}){
 const selected=limits(options),projection=options.projection??'ConcreteSyntax';
 if(!acceptSourceProjectSyntaxProjection(projection))fail('unknown source syntax projection');
 if(!Array.isArray(inputs)||inputs.length===0||inputs.length>bound(options,'fileLimit',10000))fail('expected a bounded nonempty source inventory');
 const records=Array.from(inputs,row=>{if(!row||typeof row.path!=='string'||!acceptSourceProjectPath(row.path)||typeof row.source!=='string'||!row.source.isWellFormed())fail('invalid original source input');const language=canonicalLanguageName(row.language);if(!language)fail('unknown source language');return {path:row.path,source:row.source,language};}).sort((a,b)=>compareProjectTranslationIdentifiers(a.path,b.path));
 if(new Set(records.map(row=>row.path)).size!==records.length)fail('duplicate source project path');
 if(records.reduce((sum,row)=>sum+Buffer.byteLength(row.source),0)>selected.byteLimit)fail('source byte limit exceeded');
 const entries=[],files=[];
 for(const row of records){
  const network=LinkNetwork.parse(row.source,row.language),links=network.links(),targets=[],firsts=[],counts=[];
  for(const link of links){firsts.push(targets.length);const references=link.references();counts.push(references.length);for(const id of references)targets.push(id.asU64());}
  const positions=selectSourceProjectSyntaxRootPositions(links.map(link=>link.metadata().linkType===LinkType.Syntax?'syntax':'other'),links.map(link=>link.id().asU64()),firsts,counts,targets);
  if(positions.length===0)fail('the selected parser did not produce a concrete syntax tree: '+row.path);
  if(network.reconstructText()!==row.source)fail('concrete syntax lost original source bytes: '+row.path);
  if(options.requireClean===true&&!network.verifyFullMatch().isClean())fail('unclean original concrete syntax: '+row.path);
  const root=network.insertLink(positions.map(position=>links[position].id()),LinkMetadata.new().withLinkType(LinkType.Semantic).withLanguage(row.language).withTerm(vocabulary('root')).withDefinition(row.path));
  const identifier=vocabulary('sourcePrefix')+row.path;
  entries.push({identifier,kind:'concrete syntax',network,root});files.push({bytes:Buffer.byteLength(row.source),identifier,language:row.language,path:row.path,sha256:digest(row.source)});
 }
 const directory=encodeStructuredDataLinks({files,model:vocabulary('model'),projection,schemaVersion:1},selected);
 entries.unshift({identifier:vocabulary('directory'),kind:'language configuration',network:LinkNetwork.fromLino(directory.network.toLino()),root:directory.root});
 return createMetaLanguageArchive(entries,selected);
}

/** Eager source-free reconstruction of all concrete graphs and their complete
 * term, selected syntax and allocator state through the common archive codec.
 * No parser or current grammar catalog participates in reload.
 */
export function readSourceProjectArchive(network,root,options={}){
 const selected=limits(options),archive=readMetaLanguageArchive(network,root,selected),directories=archive.entries.filter(entry=>entry.identifier===vocabulary('directory'));
 if(directories.length!==1||directories[0].kind!=='language configuration')fail('missing source project directory');
 const directory=decodeStructuredDataLinks(directories[0].network,directories[0].root,selected);
 if(!directory||typeof directory!=='object'||Object.keys(directory).sort().join('|')!=='files|model|projection|schemaVersion'||typeof directory.model!=='string'||typeof directory.schemaVersion!=='number'||typeof directory.projection!=='string'||!Array.isArray(directory.files)||directory.files.some(file=>!file||typeof file!=='object'||Object.keys(file).sort().join('|')!=='bytes|identifier|language|path|sha256'||['path','language','identifier','sha256'].some(key=>typeof file[key]!=='string')||typeof file.bytes!=='number'))fail('invalid source project directory');
 const files=directory.files,error=readSourceProjectDirectoryError(directory.model,directory.schemaVersion,directory.projection,files.map(file=>file.path),files.map(file=>file.language),files.map(file=>file.identifier),files.map(file=>file.bytes),files.map(file=>file.sha256));if(error)fail(error);
 const projection=options.projection??directory.projection;if(!acceptSourceProjectSyntaxProjection(projection))fail('unknown source syntax projection');
 if(archive.entries.length!==files.length+1)fail('foreign source project entry');
 const entries=new Map(archive.entries.map(entry=>[entry.identifier,entry]));
 const sources=files.map(file=>{const entry=entries.get(file.identifier),source=entry?.network.reconstructText(),chosen=entry?.network.link(entry.root);
  if(entry?.kind!=='concrete syntax'||chosen?.metadata().term!==vocabulary('root')||chosen.metadata().language!==file.language||chosen.metadata().definition!==file.path||source===undefined||Buffer.byteLength(source)!==file.bytes||digest(source)!==file.sha256)fail('changed source concrete graph or directory');
  const roots=chosen.references();if(roots.length===0||roots.some(id=>entry.network.link(id)?.metadata().linkType!==LinkType.Syntax))fail('invalid original source concrete roots');
  const links=entry.network.links(),targets=[],firsts=[],counts=[];for(const link of links){firsts.push(targets.length);const children=link.references();counts.push(children.length);for(const id of children)targets.push(id.asU64());}
  const expected=selectSourceProjectSyntaxRootPositions(links.map(link=>link.metadata().linkType===LinkType.Syntax?'syntax':'other'),links.map(link=>link.id().asU64()),firsts,counts,targets).map(position=>links[position].id().asU64());
  if(roots.length!==expected.length||roots.some((id,index)=>id.asU64()!==expected[index]))fail('changed original source concrete roots');
  return {...file,network:entry.network,root:entry.root,source,projection,view:entry.network.projectedLinks(projection)};
 });
 return {directory,sources};
}

/** Append new source identities while reusing all preceding complete graphs.
 * Old sources never pass through a current parser during extension.
 */
export function extendSourceProjectArchive(network,root,additions,options={}){
 const selected=limits(options),previous=readSourceProjectArchive(network,root,options);
 if(!Array.isArray(additions)||additions.some(row=>!row||typeof row.path!=='string'))fail('invalid original source input');
 const error=readSourceProjectExtensionError(previous.sources.map(file=>file.path),additions.map(file=>file.path));if(error)fail(error);
 if(previous.sources.length+additions.length>bound(options,'fileLimit',10000))fail('expected a bounded nonempty source inventory');
 const added=createSourceProjectArchive(additions,options),incoming=readSourceProjectArchive(added.network,added.root,options),sources=[...previous.sources,...incoming.sources].sort((a,b)=>compareProjectTranslationIdentifiers(a.path,b.path));
 if(sources.reduce((count,file)=>count+file.bytes,0)>selected.byteLimit)fail('source byte limit exceeded');
 const files=sources.map(file=>({bytes:file.bytes,identifier:file.identifier,language:file.language,path:file.path,sha256:file.sha256})),projection=options.projection??previous.directory.projection;
 const directory=encodeStructuredDataLinks({files,model:vocabulary('model'),projection,schemaVersion:1},selected);
 return createMetaLanguageArchive([{identifier:vocabulary('directory'),kind:'language configuration',network:LinkNetwork.fromLino(directory.network.toLino()),root:directory.root},...sources.map(file=>({identifier:file.identifier,kind:'concrete syntax',network:file.network,root:file.root}))],selected);
}
