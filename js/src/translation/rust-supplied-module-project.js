import {isDeepStrictEqual} from 'node:util';
// Strengthen the existing supplied-project composer with actual CST ownership;
// this module adds no competing filesystem resolver or semantic parser.
import {composeRustSourceProject,createRustSourceProjectProgramLinks,readRustSourceProjectConcreteGraphs} from '../rust-source-project.js';
import {readProgramConcreteSyntaxFacts} from '../program-representation.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from '../structured-data-links.js';
import {tokenize} from './lexer.js';
const fail=reason=>{throw new TypeError('Rust supplied module CST: '+reason);};
const model='Rust supplied concrete module ownership 1',identifier=path=>'Rust supplied source '+path;
function originalFacts(original){
 const syntax=readProgramConcreteSyntaxFacts(original.network,original.source),byId=new Map(syntax.map(row=>[row.linkId,row])),fields=new Map();
 for(const edge of original.network.fieldRelations()){let owned=fields.get(edge.parent.asU64());if(owned===undefined){owned=new Map();fields.set(edge.parent.asU64(),owned);}const children=owned.get(edge.label)??[];children.push(edge.child.asU64());owned.set(edge.label,children);}
 return {syntax,tokens:tokenize(original.source,'Rust').tokens,field(parent,label){const children=fields.get(parent.linkId)?.get(label)??[];if(children.length>1)fail('a module field must have one actual CST owner');return children.length===0?undefined:byId.get(children[0]);}};
}
function outputOffsets(source,mappings){
 const wanted=new Set(mappings.flatMap(row=>[row.outputStart,row.outputEnd])),offsets=new Map([[0,0]]);let bytes=0,units=0;
 for(const character of source){bytes+=Buffer.byteLength(character);units+=character.length;if(wanted.has(bytes))offsets.set(bytes,units);}
 if([...wanted].some(value=>!offsets.has(value)))fail('composition output coordinates split an actual UTF8 scalar');return offsets;
}
function moduleWitnesses(composition,originals){
 const facts=new Map(originals.map(original=>[original.path,originalFacts(original)])),modules=[],namespaces=new Set();
 for(const actual of composition.modules){
  const source=facts.get(actual.path);if(!source)fail('composition refers to a foreign concrete source');
  const candidates=source.syntax.filter(node=>node.term==='module_item'&&node.start<=actual.startUnit&&node.end===actual.endUnit);if(candidates.length!==1)fail('composition has no unique actual module CST declaration');
  const declaration=candidates[0],name=source.field(declaration,'name'),body=source.field(declaration,'body'),token=name&&source.tokens.find(token=>token.kind==='identifier'&&token.start===name.start&&token.end===name.end),namespace=actual.module.split('::');
  if(!token||namespace.at(-1)!==token.value||namespace[0]!=='crate')fail('composed namespace differs from the actual module name field');
  if(namespaces.has(actual.module))fail('duplicate actual module namespace: '+actual.module);namespaces.add(actual.module);
  if((body!==undefined)!==(actual.path===actual.provider))fail('module provider disagrees with its actual inline body field');
  modules.push({namespace,parent:namespace.slice(0,-1),name:token.value,source:identifier(actual.path),sourceLink:declaration.linkId,declaration:{start:declaration.start,end:declaration.end},nameSpan:{start:name.start,end:name.end},kind:body?'inline':'external',...(body?{}:{target:identifier(actual.provider)})});
 }
 return modules;
}
/** Preserve default full CSTs and source-free archived reload while verifying
 * each composed module against the real generated grammar name/body fields.
 */
export function captureRustSuppliedModuleProject(inputs,{crateRoot,library=false,parseOptions={},fileLimit=1024,moduleLimit=4096,depthLimit=64,byteLimit=16777216,nodeLimit=1000000,referenceLimit=4000000}={}){
 const options={...parseOptions,library,fileLimit,moduleLimit,expansionLimit:moduleLimit,depthLimit,byteLimit,outputUnitLimit:byteLimit,nodeLimit,referenceLimit,retainCompilation:true};
 const packet=createRustSourceProjectProgramLinks(inputs,crateRoot,options),originals=readRustSourceProjectConcreteGraphs(packet.network,packet.root,options),modules=moduleWitnesses(packet.composition,originals);
 const offsets=outputOffsets(packet.composition.source,packet.composition.mappings),segments=packet.composition.mappings.map(row=>({generatedStart:offsets.get(row.outputStart),generatedEnd:offsets.get(row.outputEnd),generatedByteStart:row.outputStart,generatedByteEnd:row.outputEnd,source:identifier(row.path),start:row.startUnit,end:row.endUnit,role:row.kind==='source'?'original':row.kind}));
 const graph={model,crateRoot,library,entryNormalization:packet.entryNormalization,files:packet.composition.files.map(file=>({path:file.path,identifier:identifier(file.path),bytes:file.sourceBytes,sha256:file.sourceSha256,selected:!packet.composition.unusedSources.includes(file.path)})),modules,segments};
 const encoded=encodeStructuredDataLinks(graph,{nodeLimit,referenceLimit,byteLimit}),links=encoded.network.links(),existing=packet.network.links();if(existing.length+links.length+2>nodeLimit)fail('module ownership graph exceeds its node budget');
 if([...existing,...links].reduce((sum,link)=>sum+link.references().length,0)+3>referenceLimit)fail('module ownership graph exceeds its reference budget');
 const ids=new Map();for(const link of links)ids.set(link.id().asU64(),packet.network.insertLink([],link.metadata().clone()));for(const link of links)packet.network.link(ids.get(link.id().asU64())).setReferences(link.references().map(reference=>{const target=ids.get(reference.asU64());if(!target)fail('dangling module ownership reference');return target;}));
 packet.network.insertFieldRelation(packet.root,'Rust module concrete ownership',ids.get(encoded.root.asU64()));
 const archive=packet.network.fieldRelations().filter(edge=>edge.parent.asU64()===packet.root.asU64()&&edge.label==='source project concrete archive');if(archive.length!==1)fail('complete concrete archive must have its original unique owner');
 return {...packet,model,moduleGraph:graph,archive:{network:packet.network,root:archive[0].child}};
}


/** Recheck ownership after text/binary Links reload without original files or
 * a grammar invocation. Lexical module rules are reused; no source body runs. */
export function readRustSuppliedModuleOwnership(network,root,{fileLimit=1024,moduleLimit=4096,depthLimit=64,byteLimit=16777216,nodeLimit=1000000,referenceLimit=4000000}={}){
 const options={fileLimit,moduleLimit,depthLimit,byteLimit,nodeLimit,referenceLimit,expansionLimit:moduleLimit,outputUnitLimit:byteLimit};
 const fields=network.fieldRelations().filter(edge=>edge.parent.asU64()===root.asU64()&&edge.label==='Rust module concrete ownership');if(fields.length!==1)fail('module ownership directory must retain its actual unique root');
 const graph=decodeStructuredDataLinks(network,fields[0].child,options);if(graph?.model!==model||typeof graph.crateRoot!=='string'||typeof graph.library!=='boolean')fail('unknown module ownership model or source entry');
 const originals=readRustSourceProjectConcreteGraphs(network,root,options),composition=composeRustSourceProject(originals.map(row=>({path:row.path,source:row.source})),graph.crateRoot,options),modules=moduleWitnesses(composition,originals),offsets=outputOffsets(composition.source,composition.mappings);
 const original=originals.find(row=>row.path===graph.crateRoot);if(!original)fail('missing original crate root');const facts=originalFacts(original),hasMain=facts.syntax.some(node=>{if(node.term!=='function_item')return false;const name=facts.field(node,'name'),token=name&&facts.tokens.find(token=>token.start===name.start&&token.end===name.end);return token?.value==='main'&&!facts.syntax.some(parent=>['module_item','function_item','implementation_item','trait_item'].includes(parent.term)&&parent.start<node.start&&parent.end>=node.end);});
 const expected={model,crateRoot:graph.crateRoot,library:graph.library,entryNormalization:graph.library&&!hasMain?'empty frontend entry for a source library':'none',files:composition.files.map(file=>({path:file.path,identifier:identifier(file.path),bytes:file.sourceBytes,sha256:file.sourceSha256,selected:!composition.unusedSources.includes(file.path)})),modules,segments:composition.mappings.map(row=>({generatedStart:offsets.get(row.outputStart),generatedEnd:offsets.get(row.outputEnd),generatedByteStart:row.outputStart,generatedByteEnd:row.outputEnd,source:identifier(row.path),start:row.startUnit,end:row.endUnit,role:row.kind==='source'?'original':row.kind}))};
 if(!isDeepStrictEqual(graph,expected))fail('module ownership directory differs from its actual current concrete sources');return graph;
}
