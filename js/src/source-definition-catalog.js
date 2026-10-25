// The catalog loads one existing complete source graph at a time. It owns no
// executable substitute bodies and authorizes no semantic deduplication.
import {openSync,fstatSync,readSync,closeSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readRuntimeProjectData} from './project-data-loader.js';
import {LinkNetwork} from './network.js';
import {LinkId} from './primitives.js';
import {decodeTypedProgramLinks} from './typed-program-links.js';
import {recheckLinkedTypedProgram} from './typed-program-checked-projection.js';
import {encodeFormalFunctionDefinition} from './formal-function-links.js';
import {acceptSourceDefinitionCatalogHeader,validateSourceDefinitionCatalogEntries,selectSourceDefinitionCatalogEntry,acceptSourceDefinitionCatalogLimits,readSourceDefinitionCatalogIdentity} from './source-definition-catalog-rules.js';
const fail=message=>{throw new TypeError('source definition catalog: '+message);};
function readAsset(entry,byteLimit){
 const descriptor=openSync(new URL('./data/'+entry.asset,import.meta.url),'r');
 try{
  const stat=fstatSync(descriptor);
  if(!stat.isFile()||stat.size!==entry.bytes||stat.size>byteLimit)fail('program asset identity changed');
  // One extra byte detects growth without reading an unbounded replacement.
  const bytes=Buffer.alloc(entry.bytes+1);let offset=0;
  while(offset<bytes.length){const count=readSync(descriptor,bytes,offset,bytes.length-offset,null);if(!count)break;offset+=count;}
  if(offset!==entry.bytes)fail('program asset identity changed');
  return bytes.subarray(0,offset);
 }finally{closeSync(descriptor);}
}
/** Fresh ordinary metadata; editing it never edits the shipped catalog. */
export function sourceDefinitionCatalog(){
 const value=readRuntimeProjectData(new URL('./data/source-definition-catalog.lino',import.meta.url));
 if(!value||!Array.isArray(value.entries)||!acceptSourceDefinitionCatalogHeader(value.schemaVersion,value.model,value.entries.length))fail('unsupported catalog header');
 const entries=value.entries;
 for(const entry of entries)if(!entry||typeof entry.identifier!=='string'||typeof entry.asset!=='string'||typeof entry.sha256!=='string'||typeof entry.bytes!=='number'||typeof entry.root!=='string')fail('invalid catalog entry');
 const code=validateSourceDefinitionCatalogEntries(entries.map(entry=>entry.identifier),entries.map(entry=>entry.asset),entries.map(entry=>entry.sha256),entries.map(entry=>entry.bytes),entries.map(entry=>entry.root));
 if(code!==1)fail('invalid catalog entries (reason '+code+')');
 return entries.map(entry=>({...entry,definitionIdentity:readSourceDefinitionCatalogIdentity(entry.identifier,entry.sha256)}));
}
/** Unknown qualified identities have no implicit name or language fallback. */
export function sourceDefinitionEntry(identifier){
 if(typeof identifier!=='string')fail('identifier must be text');
 const entries=sourceDefinitionCatalog(),index=selectSourceDefinitionCatalogEntry(entries.map(entry=>entry.identifier),identifier);
 return index<0?undefined:entries[index];
}
/** Load, authenticate and statically recheck one complete source definition.
 * Limits govern this selected program, never an eager catalog-wide load. */
export function createSourceDefinitionProgramLinks(identifier,{byteLimit=16*1024*1024,nodeLimit=100000,depthLimit=256}={}){
 if(!acceptSourceDefinitionCatalogLimits(byteLimit,nodeLimit,depthLimit))fail('invalid resource limit');
 const entry=sourceDefinitionEntry(identifier);if(!entry)fail('unknown qualified program identity');
 if(entry.bytes>byteLimit)fail('program byte limit exceeded');
 const bytes=readAsset(entry,byteLimit);
 if(bytes.length!==entry.bytes||createHash('sha256').update(bytes).digest('hex')!==entry.sha256)fail('program asset identity changed');
 const network=LinkNetwork.fromLino(bytes.toString('utf8'));
 if(network.links().length>nodeLimit)fail('program node limit exceeded');
 const roots=network.links().filter(link=>link.metadata().language==='meta-language typed program'&&link.metadata().term==='typed program'&&link.metadata().definition==='1');
 const root=LinkId.from(entry.root);
 if(roots.length!==1||!roots[0].id().equals(root))fail('program root identity changed');
 recheckLinkedTypedProgram(decodeTypedProgramLinks(network,root,{nodeLimit,depthLimit}),{wholeProgram:true});
 return {network,root};
}
/** Select the actual owned function and complete recursive context. Existing
 * formal-model refusals remain binding; catalog presence is not a proof. */
export function createSourceDefinitionFormalFunctionLinks(identifier,declaration,options={}){
 return encodeFormalFunctionDefinition(createSourceDefinitionProgramLinks(identifier,options),declaration,{provenance:{sourceDefinition:sourceDefinitionEntry(identifier)}});
}
