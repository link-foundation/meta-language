// Supplied source inventories only: no disk traversal, downloads or subprocess.
import {createHash} from 'node:crypto';
import {tokenize} from './translation/lexer.js';
import {parseRust} from './translation/rust.js';
import {checkProgram} from './translation/check.js';
import {encodeTypedProgramLinks} from './typed-program-links.js';
import {LinkNetwork} from './network.js';
import {createSourceProjectArchive,readSourceProjectArchive} from './source-project-archive.js';
import {compareProjectTranslationIdentifiers} from './project-translation-rules.js';
import {expandRustProjectSourceEvents} from './rust-project-source-rules.js';

const fail=message=>{throw new TypeError('Rust source project: '+message);};
const hash=source=>createHash('sha256').update(source,'utf8').digest('hex');
function limit(options,name,fallback){if(!options||typeof options!=='object'||Array.isArray(options))fail('expected a resource options record');const value=options[name]??fallback;if(!Number.isSafeInteger(value)||value<1)fail('invalid '+name);return value;}
const unitsToBytes=(source,position)=>Buffer.byteLength(source.slice(0,position),'utf8');

/** Inline only actual module declarations, retaining every original file and
 * every changed boundary. Coordinates explicitly distinguish UTF16 and UTF8.
 * Options bound files, input bytes, tokens, depth, expansions and output units.
 */
export function composeRustSourceProject(inputs,entry,options={}){
 if(!options||typeof options!=='object'||Array.isArray(options))fail('expected a resource options record');
 if(!Array.isArray(inputs)||inputs.length===0||inputs.length>limit(options,'fileLimit',10000))fail('expected a bounded nonempty source inventory');
 const files=inputs.map(row=>{if(!row||typeof row.path!=='string'||typeof row.source!=='string'||!row.source.isWellFormed())fail('expected original UTF8 Rust source files');return {path:row.path,source:row.source,sourceBytes:Buffer.byteLength(row.source),sourceSha256:hash(row.source)};})
  .sort((a,b)=>compareProjectTranslationIdentifiers(a.path,b.path));
 if(files.reduce((total,row)=>total+row.sourceBytes,0)>limit(options,'byteLimit',128*1024*1024))fail('input byte limit exceeded');
 const lexed=[];let tokenCount=0;const tokenLimit=limit(options,'tokenLimit',2000000);
 for(const file of files){const tokens=tokenize(file.source,'Rust').tokens;tokenCount+=tokens.length;if(tokenCount>tokenLimit)fail('token limit exceeded');lexed.push(tokens);}
 const events=expandRustProjectSourceEvents(files.map(row=>row.path),files.map(row=>row.source),
  lexed.map(tokens=>tokens.map(token=>token.kind==='identifier'&&token.value==='mod'&&token.raw==='mod'?'keyword':token.kind)),
  lexed.map(tokens=>tokens.map(token=>token.value)),lexed.map(tokens=>tokens.map(token=>token.start)),lexed.map(tokens=>tokens.map(token=>token.end)),
  entry,limit(options,'depthLimit',128),limit(options,'expansionLimit',4096),limit(options,'outputUnitLimit',128*1024*1024));
 const byPath=new Map(files.map(row=>[row.path,row])),parts=[],mappings=[],modules=[],used=new Set();let outputBytes=0;
 for(let index=0;index<events.length;index+=7){
  const [kind,path,first,last,fragment,module,provider]=events.slice(index,index+7),file=byPath.get(path);
  const startUnit=Number(first),endUnit=Number(last),start=unitsToBytes(file.source,startUnit),end=unitsToBytes(file.source,endUnit);
  if(kind==='file'){used.add(path);continue;}
  if(kind==='module'){modules.push({path,start,end,startUnit,endUnit,module,provider});continue;}
  const outputEnd=outputBytes+Buffer.byteLength(fragment);
  mappings.push({kind,path,start,end,startUnit,endUnit,outputStart:outputBytes,outputEnd,module,provider});parts.push(fragment);outputBytes=outputEnd;
 }
 return {schemaVersion:1,model:'complete Rust source module composition 1',entry,source:parts.join(''),files,modules,mappings,
  unusedSources:files.filter(row=>!used.has(row.path)).map(row=>row.path)};
}

/** The ordinary frontend checks the complete composed crate. Library mode
 * records its empty entry normalization, used only when no main exists.
 */
export function parseRustSourceProject(inputs,entry,options={}){
 const composition=composeRustSourceProject(inputs,entry,options);let source=composition.source,entryNormalization='none',surface;
 try{surface=parseRust(source);}catch(error){
  if(options.library===true&&error.kind==='syntax'&&error.reason==='a Rust program needs fn main'){
   source+='\nfn main(){}\n';entryNormalization='empty frontend entry for a source library';surface=parseRust(source);
  }else throw error;
 }
 return {composition,surface,entryNormalization};
}

/** Checked editable semantic Links plus every complete original concrete graph.
 * Concrete islands retain their source identifiers; equal labels do not merge
 * concepts. Resource failure returns no partial archive or executable product.
 */
export function createRustSourceProjectProgramLinks(inputs,entry,options={}){
 const {composition,surface,entryNormalization}=parseRustSourceProject(inputs,entry,options),program=checkProgram(surface);
 const packet=encodeTypedProgramLinks(program,{provenance:{sourceProject:composition,entryNormalization,userProvenance:options.provenance??null}}),nodeLimit=limit(options,'nodeLimit',1000000);
 const archive=createSourceProjectArchive(composition.files.map(file=>({path:file.path,source:file.source,language:'Rust'})),{...options,requireClean:true,projection:'ConcreteSyntax'}),links=archive.network.links();
 if(packet.network.links().length+links.length+2>nodeLimit)fail('concrete graph node limit exceeded');
 const ids=new Map();for(const link of links)ids.set(link.id().asU64(),packet.network.insertLink([],link.metadata().clone()));
 for(const link of links)packet.network.link(ids.get(link.id().asU64())).setReferences(link.references().map(reference=>{const target=ids.get(reference.asU64());if(!target)fail('dangling original concrete reference');return target;}));
 packet.network.insertFieldRelation(packet.root,'source project concrete archive',ids.get(archive.root.asU64()));
 return packet;
}

/** Reload the actual concrete graphs and original source bytes from either
 * graph codec, without original files or a grammar-engine invocation.
 */
export function readRustSourceProjectConcreteGraphs(network,root,options={}){
 if(!network.link(root)||network.links().length>limit(options,'nodeLimit',1000000))fail('invalid source graph root or node limit');
 const fields=network.fieldRelations(),seen=new Set(),results=[],current=fields.filter(field=>field.parent.asU64()===root.asU64()&&field.label==='source project concrete archive');
 if(current.length){if(current.length!==1||fields.some(field=>field.parent.asU64()===root.asU64()&&field.label==='source concrete graph'))fail('ambiguous source concrete archive');
  return readSourceProjectArchive(network,current[0].child,{...options,projection:'ConcreteSyntax'}).sources.map(file=>{if(file.language!=='Rust'||!file.network.verifyFullMatch().isClean())fail('changed original concrete source');return {path:file.path,source:file.source,network:file.network};});}
 // The complete preceding Links layout remains readable under its own roles.
 for(const field of fields.filter(field=>field.parent.asU64()===root.asU64()&&field.label==='source concrete graph')){
  const inventory=network.link(field.child),path=inventory?.metadata().definition;
  if(inventory?.metadata().term!=='complete original source concrete graph'||inventory.metadata().language!=='Rust source project'||typeof path!=='string'||seen.has(path))fail('invalid source concrete inventory');seen.add(path);
  const members=inventory.references(),ids=new Set(members.map(id=>id.asU64()));if(ids.size!==members.length)fail('duplicate source concrete member');
  const concrete=new LinkNetwork();for(const id of members){const link=network.link(id);if(!link||link.references().some(target=>!ids.has(target.asU64())))fail('foreign source concrete reference');concrete.insertLinkWithOptionalId(id,link.references(),link.metadata().clone());}
  const owned=fields.filter(field=>field.parent.asU64()===inventory.id().asU64()),registered=owned.filter(field=>field.label==='registered concrete points'),digests=owned.filter(field=>field.label==='original source digest');
  if(registered.length!==1||digests.length!==1)fail('missing source graph registrations or digest');
  const points=network.link(registered[0].child)?.references();if(!points||new Set(points.map(id=>id.asU64())).size!==points.length)fail('invalid source graph registrations');
  for(const id of points){const term=concrete.link(id)?.metadata().term;if(typeof term!=='string'||concrete._terms.has(term))fail('foreign or ambiguous source graph registration');concrete._terms.set(term,id);}
  const source=concrete.reconstructText(),digest=network.link(digests[0].child)?.metadata().definition;
  if(hash(source)!==digest||!concrete.verifyFullMatch().isClean())fail('changed original concrete source');
  results.push({path,source,network:concrete});
 }
 if(results.length===0)fail('missing source concrete inventories');return results;
}
