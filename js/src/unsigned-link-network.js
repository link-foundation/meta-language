// Explicit exact-ID profile. Existing Number-profile APIs retain their own
// input domain. Dense bridges are reversible coordinate changes, never new
// global concept identities or semantic-equivalence certificates.
import {LinkId,LinkMetadata,LinkType,ParseConfiguration} from './primitives.js';
import {LinkNetwork} from './network.js';
import {acceptNetworkProjection,acceptNetworkProjectionLink} from './network-projection-rules.js';
import {allocateUnsignedSourceIdentities,readUnsignedSourceAllocationError} from './unsigned-source-document-rules.js';
import {parseLinoStatements,decodeCanonicalStatements,hasMetaSublinks} from './lino-serialization.js';
import {encodeLinkStatement} from './lino-serialization-write.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary,BinaryLinksError} from './link-network-binary.js';
import {parseUnsignedLinkIdentity,renderUnsignedLinkIdentity,compareUnsignedLinkIdentities,
 advanceUnsignedLinkIdentity,encodeUnsignedLinkEnvelope,decodeUnsignedLinkEnvelope,
 readUnsignedLinkEnvelopeError} from './unsigned-link-identity-rules.js';

/** Positive exact u64; JSON uses its decimal spelling, never a rounded Number. */
export class UnsignedLinkId {
 #words;
 constructor(value){
  if(value instanceof UnsignedLinkId)this.#words=value.words();
  else{
   if(value instanceof LinkId)value=value.asU64();
   if(typeof value==='number'&&!Number.isSafeInteger(value))throw new TypeError('an unsigned link Number must be an exact safe integer');
   if(!['string','number','bigint'].includes(typeof value))throw new TypeError('an unsigned link identity needs an exact integer or decimal spelling');
   this.#words=parseUnsignedLinkIdentity(String(value));
  }
  Object.freeze(this);
 }
 static from(value){return value instanceof UnsignedLinkId?(value.words(),value):new UnsignedLinkId(value);}
 static fromU64(value){return UnsignedLinkId.from(value);}
 words(){return [...this.#words];}
 asU64(){return BigInt(this.toString());}
 equals(other){return compareUnsignedLinkIdentities(this.#words,UnsignedLinkId.from(other).words())===0;}
 compare(other){return compareUnsignedLinkIdentities(this.#words,UnsignedLinkId.from(other).words());}
 toString(){return renderUnsignedLinkIdentity(this.#words);}
 toJSON(){return this.toString();}
 valueOf(){return this.asU64();}
}

/** An exact-ID graph link; metadata has the ordinary public representation. */
export class UnsignedLink {
 #id;#references;#metadata;
 constructor(id,references=[],metadata=LinkMetadata.new()){
  this.#id=UnsignedLinkId.from(id);
  this.#references=references.map(UnsignedLinkId.from);
  this.#metadata=metadata instanceof LinkMetadata?metadata:new LinkMetadata(metadata);
 }
 id(){return this.#id;}
 references(){return [...this.#references];}
 metadata(){return this.#metadata;}
 setReferences(references){const next=references.map(UnsignedLinkId.from);this.#references=next;}
 setMetadata(metadata){this.#metadata=metadata instanceof LinkMetadata?metadata:new LinkMetadata(metadata);}
 clone(){return new UnsignedLink(this.#id,this.#references,this.#metadata.clone());}
}

/** Full u64 graph storage. The exhausted allocator is separate from u64::MAX.
 * Text carries graph/registration data; binary additionally keeps the exact
 * allocator and selected-syntax cache, including after deletion/exhaustion. */
export class UnsignedLinkNetwork {
 _links=new Map();_terms=new Map();_conceptSyntax=new Map();_nextIdentity=UnsignedLinkId.from(1);
 static parse(text,language,configuration=ParseConfiguration.default(),options={}){return createUnsignedSourceDocumentLinks(text,language,configuration,options).network;}
 static parseWithRegistry(registry,text,language,configuration=ParseConfiguration.default(),options={}){return rebaseUnsignedLinkNetwork(LinkNetwork.parseWithRegistry(registry,text,language,configuration),options.firstIdentity??1,options).network;}
 reconstructText(options={}){return createDenseUnsignedLinkNetwork(this,options).network.reconstructText();}
 reconstructBytes(options={}){return createDenseUnsignedLinkNetwork(this,options).network.reconstructBytes();}
 projectedLinks(projection='ConcreteSyntax'){
  if(typeof projection!=='string'||!acceptNetworkProjection(projection))throw new TypeError('a network projection must be Lossless, ConcreteSyntax, AbstractSyntax or Semantic');
  return this.links().filter(link=>acceptNetworkProjectionLink(projection,link.metadata().linkType??''));
 }
 projected_links(projection='ConcreteSyntax'){return this.projectedLinks(projection);}
 get nextIdentity(){return this._nextIdentity;}
 len(){return this._links.size;}
 links(){return [...this._links.values()].sort((first,second)=>first.id().compare(second.id()));}
 link(id){return this._links.get(UnsignedLinkId.from(id).asU64());}
 findTerm(term){return this._terms.get(term);}
 insertLink(references=[],metadata=LinkMetadata.new()){return this.insertLinkWithOptionalId(undefined,references,metadata);}
 insertLinkWithOptionalId(preferred,references=[],metadata=LinkMetadata.new()){
  if(preferred===undefined&&this._nextIdentity===null)throw new RangeError('unsigned link allocator exhausted');
  const id=UnsignedLinkId.from(preferred??this._nextIdentity),link=new UnsignedLink(id,references,metadata);
  let next=this._nextIdentity;
  if(next!==null&&id.compare(next)>=0){const words=advanceUnsignedLinkIdentity(id.words());next=words.length===0?null:UnsignedLinkId.from(renderUnsignedLinkIdentity(words));}
  this._links.set(id.asU64(),link);this._nextIdentity=next;return id;
 }
 insertTypedPoint(type,term,definition=undefined){
  const existing=this.findTerm(term);
  if(existing!==undefined&&this.link(existing)){
   if(definition!==undefined)this.link(existing).setMetadata(this.link(existing).metadata().withDefinition(definition));
   return existing;
  }
  if(this._nextIdentity===null)throw new RangeError('unsigned link allocator exhausted');
  const id=this._nextIdentity;
  this.insertLink([id],LinkMetadata.new().withLinkType(type).withTerm(term).withNamed(true).withDefinition(definition));
  this._terms.set(term,id);return id;
 }
 insertPoint(term){return this.insertTypedPoint(LinkType.Concept,term);}
 insertObject(term){return this.insertTypedPoint(LinkType.Object,term);}
 insertField(term){return this.insertTypedPoint(LinkType.Field,term);}
 insertFieldRelation(parent,label,child){
  // Validate operands before allocating a label or changing an existing graph.
  parent=UnsignedLinkId.from(parent);child=UnsignedLinkId.from(child);
  if(this._nextIdentity===null)throw new RangeError('unsigned link allocator exhausted');
  const existing=this.findTerm(label),needsLabel=existing===undefined||!this.link(existing);
  if(needsLabel&&advanceUnsignedLinkIdentity(this._nextIdentity.words()).length===0)throw new RangeError('unsigned link allocator lacks two available identities');
  const field=this.insertTypedPoint(LinkType.Field,label,'A field label names a relation between links.');
  return this.insertLink([parent,field,child],LinkMetadata.new().withLinkType(LinkType.Field));
 }
 setLinkMetadata(id,metadata){const link=this.link(id);if(!link)return false;link.setMetadata(metadata);return true;}
 deleteLink(id){
  const actual=UnsignedLinkId.from(id);this._links.delete(actual.asU64());
  for(const [term,value]of this._terms)if(value.equals(actual))this._terms.delete(term);
 }
 clone(){
  const graph=new UnsignedLinkNetwork();graph._nextIdentity=this._nextIdentity;
  for(const link of this.links())graph._links.set(link.id().asU64(),link.clone());
  graph._terms=new Map(this._terms);graph._conceptSyntax=new Map(this._conceptSyntax);return graph;
 }
 toLino(){
  // Reuse the canonical metadata writer through temporary, explicit local IDs;
  // only the leading ID/reference tokens are supplied by the exact profile.
  return this.links().map(link=>{
   const value=link.metadata(),registered=value.term!==undefined&&this.findTerm(value.term)?.equals(link.id());
   const metadata=encodeLinkStatement(1,[],value,registered).slice(3);
   return '('+link.id()+':'+link.references().map(id=>' '+id).join('')+metadata+'\n';
  }).join('');
 }
 static fromLino(text){
  const graph=new UnsignedLinkNetwork(),statements=parseLinoStatements(text);
  if(hasMetaSublinks(statements)){
   for(const row of decodeCanonicalStatements(statements,value=>UnsignedLinkId.from(value))){
    const id=graph.insertLinkWithOptionalId(row.id,row.references,row.metadata);
    if(row.registered&&row.metadata.term!==undefined)graph._terms.set(row.metadata.term,id);
   }
  }else for(const row of statements){
   if(row.id==null||row.values.some(value=>value.id==null||value.values.length!==0))throw new TypeError('unsigned Links Notation needs identified flat links or canonical metadata');
   graph.insertLinkWithOptionalId(row.id,row.values.map(value=>value.id),LinkMetadata.new().withLinkType(row.values.length?LinkType.Relation:LinkType.Concept));
  }
  return graph;
 }
}

/** A fresh, separate coordinate assignment. Complete inverse IDs are explicit;
 * neither the source graph nor concept identities are silently merged. */
export function rebaseUnsignedLinkNetwork(source,firstIdentity=1,options={}){
 const bounds=limits(options),first=UnsignedLinkId.from(firstIdentity);
 let exact=source;
 if(source instanceof LinkNetwork){
  if(source.len()>bounds.recordLimit)throw new BinaryLinksError('record resource limit exceeded');
  exact=new UnsignedLinkNetwork();
  for(const link of source.links())exact.insertLinkWithOptionalId(link.id(),link.references(),link.metadata().clone());
  exact._terms=new Map([...source._terms].map(([term,id])=>[term,UnsignedLinkId.from(id)]));exact._conceptSyntax=new Map(source._conceptSyntax);
 }
 const view=createDenseUnsignedLinkNetwork(exact,bounds),reason=readUnsignedSourceAllocationError(first.words(),view.directory.length,bounds.recordLimit);
 if(reason!=='')throw new BinaryLinksError(reason);
 const plan=allocateUnsignedSourceIdentities(first.words(),view.directory.length,bounds.recordLimit),identities=[];
 for(let at=3;at<plan.length;at+=2)identities.push(UnsignedLinkId.from(renderUnsignedLinkIdentity([plan[at],plan[at+1]])));
 const nextIdentity=plan[1]===-1?null:UnsignedLinkId.from(renderUnsignedLinkIdentity([plan[1],plan[2]]));
 return {network:restoreDenseUnsignedLinkNetwork(view.network,identities,nextIdentity,bounds),sourceIdentities:view.directory,identities};
}
/** Registered source grammars use the same dispatch before exact allocation. */
export function createUnsignedSourceDocumentLinksWithRegistry(registry,text,language,configuration=ParseConfiguration.default(),options={}){
 return rebaseUnsignedLinkNetwork(LinkNetwork.parseWithRegistry(registry,text,language,configuration),options.firstIdentity??1,options);
}
/** Default ordinary parsing produces a CST; optional projections keep the full
 * exact source network available. All original source configuration is passed. */
export function createUnsignedSourceDocumentLinks(text,language,configuration=ParseConfiguration.default(),options={}){
 return rebaseUnsignedLinkNetwork(LinkNetwork.parse(text,language,configuration),options.firstIdentity??1,options);
}

function limits(options){
 const byteLimit=options.byteLimit??128*1024*1024,recordLimit=options.recordLimit??100000;
 if(!Number.isSafeInteger(byteLimit)||byteLimit<33||!Number.isSafeInteger(recordLimit)||recordLimit<1)throw new BinaryLinksError('invalid unsigned link resource limits');
 return {byteLimit,recordLimit};
}

/** Reversible coordinate view for existing source/CST/concept algorithms.
 * Callers retain the complete directory when carrying results back. It is
 * never a claim that unrelated global concept identities are equivalent. */
export function createDenseUnsignedLinkNetwork(source,options={}){
 if(!(source instanceof UnsignedLinkNetwork))throw new TypeError('expected an unsigned link network');
 const bounds=limits(options),links=source.links();
 if(links.length>bounds.recordLimit)throw new BinaryLinksError('record resource limit exceeded');
 const names=new Map();
 const remember=value=>{
  const id=UnsignedLinkId.from(value);names.set(id.asU64(),id);
  if(names.size>Math.floor((bounds.byteLimit-25)/8))throw new BinaryLinksError('unsigned link directory resource limit exceeded');
 };
 for(const link of links){remember(link.id());for(const id of link.references())remember(id);}
 for(const id of source._terms.values())remember(id);
 const directory=[...names.values()].sort((first,second)=>first.compare(second));
 const indexes=new Map(directory.map((id,index)=>[id.asU64(),index+1]));
 const dense=new LinkNetwork(),local=id=>indexes.get(UnsignedLinkId.from(id).asU64());
 for(const link of links)dense.insertLinkWithOptionalId(local(link.id()),link.references().map(local),link.metadata().clone());
 dense._nextId=directory.length+1;
 dense._terms=new Map([...source._terms].map(([term,id])=>[term,LinkId.from(local(id))]));
 dense._conceptSyntax=new Map(source._conceptSyntax);
 return {network:dense,directory,nextIdentity:source.nextIdentity};
}

/** Restore only coordinates present in the supplied directory. Adding nodes
 * in a dense view requires an explicit new global-ID allocation beforehand. */
export function restoreDenseUnsignedLinkNetwork(dense,directory,nextIdentity,options={}){
 const bounds=limits(options);
 if(!(dense instanceof LinkNetwork)||!Array.isArray(directory)||dense.len()>bounds.recordLimit||directory.length>Math.floor((bounds.byteLimit-25)/8))throw new BinaryLinksError('invalid dense unsigned graph');
 const ids=directory.map(UnsignedLinkId.from);
 for(let index=1;index<ids.length;index+=1)if(ids[index-1].compare(ids[index])>=0)throw new BinaryLinksError('unsigned link directory must be strictly ordered');
 const cursor=nextIdentity===null?null:UnsignedLinkId.from(nextIdentity),used=new Set();
 const global=id=>{const local=LinkId.from(id).asU64();if(local>ids.length)throw new BinaryLinksError('foreign dense unsigned coordinate');used.add(local);return ids[local-1];};
 const graph=new UnsignedLinkNetwork();
 for(const link of dense.links()){
  const id=global(link.id());
  if(cursor!==null&&id.compare(cursor)>=0)throw new BinaryLinksError('unsigned allocator does not follow every stored link');
  graph.insertLinkWithOptionalId(id,link.references().map(global),link.metadata().clone());
 }
 graph._terms=new Map([...dense._terms].map(([term,id])=>[term,global(id)]));
 if(used.size!==ids.length)throw new BinaryLinksError('unused unsigned link directory entry');
 graph._conceptSyntax=new Map(dense._conceptSyntax);graph._nextIdentity=cursor;return graph;
}

/** Complete exact-ID snapshot; its inner metadata codec remains MLDSNP02. */
export function encodeUnsignedLinkNetworkBinary(source,options={}){
 const bounds=limits(options),view=createDenseUnsignedLinkNetwork(source,bounds);
 const body=encodeLinkNetworkBinary(view.network,bounds),identities=view.directory.flatMap(id=>id.words());
 const encoded=encodeUnsignedLinkEnvelope(identities,view.nextIdentity?.words()??[],Array.from(body),bounds.byteLimit);
 if(encoded[0]<0)throw new BinaryLinksError(readUnsignedLinkEnvelopeError(encoded[0]));
 return Uint8Array.from(encoded);
}
export function decodeUnsignedLinkNetworkBinary(bytes,options={}){
 const bounds=limits(options);
 if(!(bytes instanceof Uint8Array)&&!Array.isArray(bytes))throw new BinaryLinksError('expected an unsigned link byte array');
 if(bytes.length>bounds.byteLimit)throw new BinaryLinksError('unsigned link envelope resource limit');
 const fields=decodeUnsignedLinkEnvelope(Array.from(bytes),bounds.byteLimit);
 if(fields[0]<0)throw new BinaryLinksError(readUnsignedLinkEnvelopeError(fields[0]));
 const count=fields[3],directory=[];
 for(let index=0;index<count;index+=1)directory.push(UnsignedLinkId.from(renderUnsignedLinkIdentity(fields.slice(4+index*2,6+index*2))));
 const next=fields[0]===1?null:UnsignedLinkId.from(renderUnsignedLinkIdentity(fields.slice(1,3)));
 const body=Uint8Array.from(fields.slice(4+count*2));
 if(body[7]!==50)throw new BinaryLinksError('the unsigned link envelope requires a complete version-two inner network');
 const dense=decodeLinkNetworkBinary(body,bounds);
 if(dense._nextId!==count+1)throw new BinaryLinksError('noncanonical dense unsigned allocator');
 return restoreDenseUnsignedLinkNetwork(dense,directory,next,bounds);
}
