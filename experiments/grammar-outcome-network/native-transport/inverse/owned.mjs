// Source-bound owned reconstruction. Native DTO/static-reference reconstruction
// and physical CST ownership are separate obligations, not inferred here.
import assert from 'node:assert/strict';
import {types} from 'node:util';
import {decodeStructuredDataLinks} from '../../../../js/src/structured-data-graph.js';
import {StructuredDataNetwork} from '../../../../js/src/structured-data-encode.js';
import {LinkNetwork} from '../../../../js/src/network.js';
import {Link,LinkId,LinkMetadata,LinkFlags} from '../../../../js/src/primitives.js';
import {wellFormedString} from '../../../../js/src/structured-data-model.js';
import {readOrdinaryInversePlan} from './plan.mjs';
import {outcomeWireEvents,preflightOutcome} from '../wire/events.mjs';
import {acceptOwnedOutcomeTargetWidth,readOwnedOutcomeJsonKind,readOwnedOutcomeJsonFields,acceptOwnedOutcomeContainer} from './rules.mjs';

const absent=Symbol('absent native option'),flattened=Symbol('source-owned flattened variant');
const bytes=value=>Buffer.byteLength(value,'utf8');
const defaults={nodeLimit:100000,edgeLimit:100000,depthLimit:256,byteLimit:16*1024*1024,metadataByteLimit:32*1024*1024};
const prototypes=new Set([Object.prototype,Array.prototype,Map.prototype,StructuredDataNetwork.prototype,LinkNetwork.prototype,
  Link.prototype,LinkId.prototype,LinkMetadata.prototype,LinkFlags.prototype,null]);

/** Refusals retain source-qualified types and occurrence ordinals. */
export class OrdinaryInverseRefusal extends TypeError {
  constructor(code,typeReference,path,message) {
    super(message);this.name='OrdinaryInverseRefusal';this.code=code;this.typeReference=typeReference;
    const route=[];let count=0,size=0;
    for(let item=path;item;item=item.parent){
      if(++count>256||(size+=bytes(String(item.step)))>1024*1024)break;
      route.push(item.step);
    }
    this.route=route.reverse();
  }
}
const refuse=(frame,code,message)=>{throw new OrdinaryInverseRefusal(code,frame.typeReference,frame.path,message);};
const requireValue=(condition,frame,code,message)=>{if(!condition)refuse(frame,code,message);};
const route=(parent,step)=>({parent,step});

/** Inspect descriptors before reading values; proxy traps/getters never run. */
function requirePassiveData(value,maximum,frame) {
  const pending=[value],seen=new Set();let count=0;
  while(pending.length){
    const current=pending.pop();
    requireValue(++count<=maximum,frame,'input-budget','passive input inspection exceeds its resource limit');
    if(current===null||['undefined','string','number','boolean','bigint','symbol'].includes(typeof current))continue;
    requireValue(typeof current==='object'&&!types.isProxy(current),frame,'active-input','functions and proxies are not passive input');
    if(seen.has(current))continue;seen.add(current);
    requireValue(prototypes.has(Object.getPrototypeOf(current)),frame,'active-input','unsupported input prototype');
    if(Object.getPrototypeOf(current)===Map.prototype){
      const size=Object.getOwnPropertyDescriptor(Map.prototype,'size').get.call(current);
      requireValue(size*2+pending.length+count<=maximum,frame,'input-budget','mapping inspection reservation exceeds its resource limit');
      Map.prototype.forEach.call(current,(item,key)=>{pending.push(key,item);});
    }
    if(Array.isArray(current))requireValue(current.length+pending.length+count<=maximum,frame,'input-budget','array inspection reservation exceeds its resource limit');
    for(const key of Reflect.ownKeys(current)){
      const descriptor=Object.getOwnPropertyDescriptor(current,key);
      requireValue(typeof key==='string'&&Object.hasOwn(descriptor,'value'),frame,'active-input','accessors and symbolic fields are not passive input');
      pending.push(descriptor.value);
      requireValue(pending.length+count<=maximum,frame,'input-budget','pending input inspection exceeds its resource limit');
    }
  }
}

function readOptions(plan,options) {
  const frame={typeReference:plan.rootReference,path:null};
  requirePassiveData(options,100,frame);
  requireValue(options&&Object.getPrototypeOf(options)===Object.prototype,frame,'options','plain options required');
  const allowed=[...Object.keys(defaults),'usizeBits','rootReference','numberFacts','codec'];
  requireValue(Object.keys(options).every(key=>allowed.includes(key)),frame,'options','unknown inverse option');
  const limits={...defaults,...options};
  for(const key of Object.keys(defaults))requireValue(Number.isSafeInteger(limits[key])&&limits[key]>0,frame,'options','positive explicit resource limits required');
  requireValue(limits.depthLimit<=plan.admission.maximumDepth,frame,'options','depth exceeds the authenticated profile');
  requireValue(acceptOwnedOutcomeTargetWidth(limits.usizeBits)&&plan.admission.targetUsizeWidths.includes(limits.usizeBits),frame,'target-profile','explicit 32 or 64 bit usize profile required');
  requireValue(limits.numberFacts===undefined||(Array.isArray(limits.numberFacts)&&limits.numberFacts.length===0),frame,'number-facts','ordinary profile admits exactly empty number facts');
  requireValue(options.codec===undefined||(options.codec!==null&&typeof options.codec==='object'&&Object.getPrototypeOf(options.codec)===Object.prototype&&Object.keys(options.codec).every(key=>['nodeLimit','depthLimit','byteLimit'].includes(key))),frame,'options','closed codec resource limits required');
  limits.codec={nodeLimit:defaults.nodeLimit,depthLimit:defaults.depthLimit,byteLimit:defaults.byteLimit,...options.codec};
  for(const key of ['nodeLimit','depthLimit','byteLimit'])requireValue(Number.isSafeInteger(limits.codec[key])&&limits.codec[key]>0,frame,'options','positive codec resource limits required');
  requireValue(limits.codec.depthLimit<=plan.admission.maximumDepth,frame,'options','codec depth exceeds the authenticated profile');
  limits.rootReference??=plan.rootReference;
  requireValue(plan.graph.roots.includes(limits.rootReference),frame,'root-type','root must be an actual source declaration');
  return limits;
}

function readInteger(value,kind,limits,frame) {
  requireValue(typeof value==='number'&&Number.isFinite(value)&&Number.isInteger(value)&&!Object.is(value,-0),frame,'integer','finite integral binary64 excluding negative zero required');
  const integer=BigInt(value);
  requireValue(acceptOwnedOutcomeIntegerValue(value,kind==='i64',limits.usizeBits),frame,'integer-range','integer exceeds its source type and target width');
  const buffer=Buffer.alloc(8);buffer.writeDoubleBE(value);
  return {decimal:integer.toString(),bits:buffer.toString('hex')};
}

function readRecordInputs(context,binding,value,frame) {
  requireValue(value!==null&&typeof value==='object'&&!Array.isArray(value),frame,'record','projected record required');
  const keys=Object.keys(value),values=new Map();let cursor=0;
  const take=(key,required)=>{
    if(keys[cursor]===key){cursor++;return value[key];}
    if(required)refuse(frame,'record-members','missing or reordered projected member');
    return absent;
  };
  for(const row of binding.wire){
    if(row.literal){requireValue(Object.is(take(row.key,true),row.value),frame,'literal','variant literal differs from its source rule');continue;}
    if(row.flatten){
      const field=binding.fields.find(item=>item.identity===row.fieldReference);
      const named=context.nodes.get(field.typeReference),leaf=context.bindings.get(named.owner);
      const candidates=leaf.variants.filter(variant=>variant.wire.flatten.every((entry,index)=>
        keys[cursor+index]===entry.key&&(entry.literal?Object.is(value[entry.key],entry.value):
          variant.wire.payloadType.kind==='string'&&typeof value[entry.key]==='string')));
      requireValue(candidates.length===1,{...frame,typeReference:field.typeReference,path:route(frame.path,field.identity)},'flattened-variant','flattened source variants must be disjoint and complete');
      const selected=candidates[0],payloads=[];
      for(const entry of selected.wire.flatten){const item=take(entry.key,true);if(!entry.literal)payloads[entry.payload]=item;}
      values.set(row.fieldReference,{[flattened]:selected.identity,payloads});continue;
    }
    let item=take(row.key,!['some','true'].includes(row.presence));
    if(row.presence==='nullable'&&item===null)item=absent;
    if(row.presence==='true'){
      requireValue(item===absent||item===true,frame,'flag','present true-only flags must be true');
      item=item!==absent;
    }
    values.set(row.fieldReference,item);
  }
  requireValue(cursor===keys.length,frame,'record-members','unknown, trailing or reordered projected members');
  return binding.fields.map(field=>({typeReference:field.typeReference,value:values.get(field.identity),step:field.identity}));
}

function describeValue(context,frame) {
  const source=context.nodes.get(frame.typeReference),value=frame.value,limits=context.limits;
  requireValue(Boolean(source),frame,'source-type','unknown source-qualified type');
  const result={form:source.kind,data:null,children:[]};
  const child=(typeReference,item,step)=>({typeReference,value:item,step});
  if(source.kind==='named')result.children=[child(source.owner,value,source.owner)];
  else if(source.kind==='type_item')result.children=[child(source.aliasTypeReference,value,source.aliasTypeReference)];
  else if(source.kind==='Box'||source.kind==='reference')result.children=[child(source.kind==='Box'?source.arguments[0]:source.references[0],value,0)];
  else if(source.kind==='Option'){
    result.data=value!==absent;
    if(result.data)result.children=[child(source.arguments[0],value,0)];
  }else if(source.kind==='Vec'){
    requireValue(Array.isArray(value),frame,'vector','projected vector required');
    requireValue(value.length<=limits.nodeLimit,frame,'node-budget','vector expansion exceeds node limit');
    result.children=value.map((item,index)=>child(source.arguments[0],item,index));
  }else if(source.kind==='IndexMap'){
    requireValue(value!==null&&typeof value==='object'&&!Array.isArray(value),frame,'mapping','projected ordered mapping required');
    const names=Object.keys(value);requireValue(names.length<=Math.floor(limits.nodeLimit/2),frame,'node-budget','mapping expansion exceeds node limit');
    for(const [index,key]of names.entries())result.children.push(child(source.arguments[0],key,index*2),child(source.arguments[1],value[key],index*2+1));
  }else if(source.kind==='struct_item'){
    result.children=readRecordInputs(context,context.bindings.get(source.identity),value,frame);
  }else if(source.kind==='enum_item'){
    const binding=context.bindings.get(source.identity);let selected;
    if(value&&typeof value==='object'&&Object.hasOwn(value,flattened))selected=binding.variants.find(variant=>variant.identity===value[flattened]);
    else selected=binding.variants.find(variant=>variant.fields&&variant.wire.filter(row=>row.literal).every(row=>
      value!==null&&typeof value==='object'&&Object.hasOwn(value,row.key)&&Object.is(value[row.key],row.value)));
    if(!selected){
      const candidates=binding.variants.filter(variant=>variant.wire.transparent===0&&
        (variant.wire.payloadType.kind==='string'?typeof value==='string':variant.wire.payloadType.kind==='i64'&&typeof value==='number'));
      requireValue(candidates.length===1,frame,'variant','no disjoint source variant matches the projection');selected=candidates[0];
    }
    result.data=selected.identity;
    result.children=selected.fields?readRecordInputs(context,selected,value,frame):
      selected.tuple.map((item,index)=>child(item.typeReference,value&&typeof value==='object'&&Object.hasOwn(value,flattened)?value.payloads[index]:value,item.identity));
    requireValue(!context.plan.admission.refusedVariants.includes(selected.identity),frame,'unsupported-embedding','embedded native values require their own representation contract');
  }else if(source.kind==='String'||source.kind==='str'){
    requireValue(typeof value==='string'&&wellFormedString(value),frame,'string','well-formed UTF-8 string required');result.data=value;
  }else if(source.kind==='bool'){
    requireValue(typeof value==='boolean',frame,'boolean','boolean required');result.data=value;
  }else if(source.kind==='usize'||source.kind==='i64')result.data=readInteger(value,source.kind,limits,frame);
  else if(source.kind==='json'){
    if(context.plan.admission.opaqueJson==='null only')requireValue(value===null,frame,'unsupported-json','non-null opaque JSON requires representation facts');
    else Object.assign(result,describeJsonProjection(value,frame,limits.nodeLimit));
  }else refuse(frame,'source-type','unsupported source-qualified form');
  return result;
}

function createContext(plan,limits) {
  return {plan,limits,nodes:new Map(plan.graph.nodes.map(node=>[node.identity,node])),
    bindings:new Map(plan.bindings.map(binding=>[binding.declarationReference,binding]))};
}

/** JSON contents remain inspectable owned links; native Number tags stay explicit gaps. */
function describeJsonProjection(value,frame,limit) {
  const kind=readOwnedOutcomeJsonKind(value===null,Array.isArray(value),typeof value);
  if(kind==='null')return {data:{kind},children:[]};
  if(kind==='number'){
    requireValue(Number.isFinite(value),frame,'json-number','finite projected JSON number required');
    const buffer=Buffer.alloc(8);buffer.writeDoubleBE(value);
    return {data:{kind:'number',bits:buffer.toString('hex')},children:[]};
  }
  if(kind==='string'){
    requireValue(wellFormedString(value),frame,'string','projected JSON string must be well formed');
    return {data:{kind:'string',value},children:[]};
  }
  if(kind==='boolean')return {data:{kind,value},children:[]};
  requireValue(kind==='array'||kind==='object',frame,'json','unsupported JSON projection');
  const array=kind==='array',keys=array?null:Object.keys(value);
  requireValue((array?value.length:keys.length)<=limit,frame,'node-budget','JSON expansion exceeds node limit');
  if(keys)requireValue(keys.every(wellFormedString),frame,'string','projected JSON keys must be well formed');
  return {data:array?{kind:'array',aliasOf:null}:{kind:'object',keys,aliasOf:null},children:(array?value:keys.map(key=>value[key]))
    .map((item,index)=>({typeReference:frame.typeReference,value:item,step:index}))};
}

/** Count first; construct the flat owned occurrence graph only on full admission. */
function walkProjection(context,value,construct) {
  const {limits}=context,pending=[{typeReference:limits.rootReference,value,path:null,depth:1,parent:null}];
  let occurrences=0,edges=0,scalarBytes=0,metadataBytes=0,maximumDepth=0;
  const nodes=construct?[]:null,borrowRequirements=construct?[]:null,jsonRequirements=construct?[]:null,jsonAliases=construct?[]:null,containers=new Map();
  while(pending.length){
    const frame=pending.pop(),description=describeValue(context,frame),source=context.nodes.get(frame.typeReference);
    requireValue(++occurrences<=limits.nodeLimit,frame,'node-budget','owned occurrence resource limit');
    requireValue(frame.depth<=limits.depthLimit,frame,'depth-budget','owned depth resource limit');
    maximumDepth=Math.max(maximumDepth,frame.depth);
    edges+=description.children.length;
    requireValue(edges<=limits.edgeLimit&&description.children.length+pending.length+occurrences<=limits.nodeLimit,frame,'edge-budget','pending occurrence or edge reservation exceeds its resource limit');
    const container=acceptOwnedOutcomeContainer(source.kind,description.data?.kind??'',Boolean(source.kind==='enum_item'&&context.bindings.get(source.identity).variants.find(item=>item.identity===description.data)?.fields));
    if(container){
      const previous=containers.get(frame.value);
      if(previous){
        requireValue(source.kind==='json'&&previous.json,frame,'container-alias','native owned containers cannot share projected occurrences');
        description.data.aliasOf=previous.node;
        if(construct)jsonAliases.push({node:occurrences,firstNode:previous.node,typeReference:frame.typeReference,firstTypeReference:previous.typeReference});
      }else containers.set(frame.value,{node:occurrences,typeReference:frame.typeReference,json:source.kind==='json'});
    }
    if(source.kind==='enum_item')metadataBytes+=bytes(description.data);
    else if(typeof description.data==='string')scalarBytes+=bytes(description.data);
    else if(description.data!==null)scalarBytes+=bytes(JSON.stringify(description.data));
    metadataBytes+=bytes(frame.typeReference)+bytes(description.form);
    requireValue(scalarBytes<=limits.byteLimit,frame,'byte-budget','owned scalar byte resource limit');
    requireValue(metadataBytes<=limits.metadataByteLimit,frame,'metadata-budget','owned source metadata resource limit');
    let row;
    if(construct){
      row={id:occurrences,typeReference:frame.typeReference,form:description.form,data:description.data,children:[]};nodes.push(row);
      if(frame.parent)frame.parent.children.push(row.id);
      if(source.kind==='reference')borrowRequirements.push({node:row.id,typeReference:row.typeReference});
      if(source.kind==='json'&&description.data?.kind==='number')jsonRequirements.push({node:row.id,typeReference:row.typeReference,bits:description.data.bits,representation:'unresolved native number variant'});
    }
    for(let index=description.children.length-1;index>=0;index--){const child=description.children[index];pending.push({
      typeReference:child.typeReference,value:child.value,path:route(frame.path,child.step),depth:frame.depth+1,parent:row});}
  }
  return construct?{nodes,borrowRequirements,...(context.plan.admission.opaqueJson==='null only'?{}:{jsonRequirements,jsonAliases})}: {occurrences,edges,scalarBytes,metadataBytes,maximumDepth};
}

/** Reconstruct an owned model from the existing closed typed Links wire graph. */
function readProjection(authority,network,root,options) {
  const plan=readOrdinaryInversePlan(authority),limits=readOptions(plan,options),frame={typeReference:limits.rootReference,path:null};
  requirePassiveData(network,limits.codec.nodeLimit*64,frame);requirePassiveData(root,16,frame);
  requireValue([StructuredDataNetwork.prototype,LinkNetwork.prototype].includes(Object.getPrototypeOf(network)),frame,'network','an existing structured data or canonical link network is required');
  const value=decodeStructuredDataLinks(network,root,limits.codec);
  return {context:createContext(plan,limits),value};
}

/** Admission/counting never creates an owned output graph. */
export function preflightOrdinaryOutcome(authority,network,root,options={}) {
  const {context,value}=readProjection(authority,network,root,options);
  return walkProjection(context,value,false);
}

export function reconstructOrdinaryOutcome(authority,network,root,options={}) {
  const {context,value}=readProjection(authority,network,root,options),{plan,limits}=context;
  const cost=walkProjection(context,value,false),owned=walkProjection(context,value,true);
  assert.equal(owned.nodes.length,cost.occurrences,'construction follows successful complete counting');
  const model={schemaVersion:1,profile:plan.profile,planSha256:authority.sha256,root:1,
    targetProfile:{usizeBits:limits.usizeBits},...owned};
  const projected=projectOrdinaryOutcome(authority,model,options);
  assert.deepEqual(projected.events,projectionEvents(value,limits.codec),'complete ordered forward/inverse/forward projection law');
  return model;
}

/** Inspect all row shapes and source edges before constructing native controls. */
function inspectOwnedModel(context,model,sha256) {
  const frame={typeReference:context.limits.rootReference,path:null},limits=context.limits;
  requirePassiveData(model,limits.nodeLimit*32,frame);
  const exact=(value,keys)=>value&&typeof value==='object'&&!Array.isArray(value)&&
    Reflect.ownKeys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));
  const recursiveJson=context.plan.admission.opaqueJson!=='null only';
  requireValue(exact(model,['schemaVersion','profile','planSha256','root','targetProfile','nodes','borrowRequirements',...(recursiveJson?['jsonRequirements','jsonAliases']:[])]),frame,'model','closed owned model required');
  requireValue(model.schemaVersion===1&&model.profile===context.plan.profile&&model.planSha256===sha256&&model.root===1,frame,'model','owned model authority or schema differs');
  requireValue(exact(model.targetProfile,['usizeBits'])&&model.targetProfile.usizeBits===limits.usizeBits,frame,'target-profile','owned target profile differs');
  requireValue(Array.isArray(model.nodes)&&model.nodes.length>0&&model.nodes.length<=limits.nodeLimit,frame,'node-budget','bounded nonempty node inventory required');
  requireValue(Array.isArray(model.borrowRequirements),frame,'borrow-manifest','borrow manifest required');
  const pending=[{id:1,typeReference:limits.rootReference,path:null,depth:1}],borrowRequirements=[],jsonRequirements=[],jsonAliases=[];
  let visited=0,edges=0,scalarBytes=0,metadataBytes=0;
  while(pending.length){
    const current=pending.pop(),row=model.nodes[current.id-1],source=context.nodes.get(current.typeReference);
    requireValue(current.id===++visited&&Boolean(source)&&exact(row,['id','typeReference','form','data','children']),current,'model-row','owned graph must be closed, single-parent preorder');
    requireValue(row.id===current.id&&row.typeReference===current.typeReference&&row.form===source.kind&&Array.isArray(row.children),current,'model-row','source type, form or child inventory differs');
    requireValue(current.depth<=limits.depthLimit,current,'depth-budget','owned model depth resource limit');
    let expected=[];
    if(source.kind==='named')expected=[source.owner];
    else if(source.kind==='type_item')expected=[source.aliasTypeReference];
    else if(source.kind==='Box')expected=[source.arguments[0]];
    else if(source.kind==='reference'){expected=[source.references[0]];borrowRequirements.push({node:row.id,typeReference:row.typeReference});}
    else if(source.kind==='Option'){
      requireValue(typeof row.data==='boolean',current,'option','explicit option presence required');expected=row.data?[source.arguments[0]]:[];
    }else if(source.kind==='Vec')expected=row.children.map(()=>source.arguments[0]);
    else if(source.kind==='IndexMap'){
      requireValue(row.children.length%2===0,current,'mapping','ordered mapping needs paired children');expected=row.children.map((_,index)=>source.arguments[index%2]);
    }else if(source.kind==='struct_item')expected=context.bindings.get(source.identity).fields.map(item=>item.typeReference);
    else if(source.kind==='enum_item'){
      const selected=context.bindings.get(source.identity).variants.find(item=>item.identity===row.data);
      requireValue(Boolean(selected)&&!context.plan.admission.refusedVariants.includes(row.data),current,'variant','unsupported or changed source variant');
      expected=(selected.fields??selected.tuple).map(item=>item.typeReference);
    }else if(source.kind==='String'||source.kind==='str')requireValue(typeof row.data==='string'&&wellFormedString(row.data),current,'string','owned UTF-8 string required');
    else if(source.kind==='bool')requireValue(typeof row.data==='boolean',current,'boolean','owned boolean required');
    else if(source.kind==='json'){
      if(!recursiveJson)requireValue(row.data===null,current,'unsupported-json','only null opaque JSON is admitted');
      else {
        const data=row.data;
        requireValue(data!==null&&typeof data==='object'&&typeof data.kind==='string',current,'json','owned JSON descriptor required');
        const keys=readOwnedOutcomeJsonFields(data.kind);
        requireValue(exact(data,keys),current,'json','closed owned JSON descriptor required');
        if(data.kind==='array'||data.kind==='object'){
          requireValue(data.aliasOf===null||Number.isSafeInteger(data.aliasOf)&&data.aliasOf>0&&data.aliasOf<row.id,current,'json-alias','JSON alias must name an earlier owned occurrence');
          if(data.aliasOf!==null)jsonAliases.push({node:row.id,firstNode:data.aliasOf,typeReference:row.typeReference,firstTypeReference:model.nodes[data.aliasOf-1].typeReference});
        }
        if(data.kind==='array')expected=row.children.map(()=>source.identity);
        else if(data.kind==='object'){
          requireValue(Array.isArray(data.keys)&&data.keys.length===row.children.length&&data.keys.every(key=>typeof key==='string'&&wellFormedString(key))&&new Set(data.keys).size===data.keys.length,current,'json','ordered unique JSON object keys required');
          expected=row.children.map(()=>source.identity);
        }else if(data.kind==='number'){
          requireValue(typeof data.bits==='string'&&/^[0-9a-f]{16}$/u.test(data.bits)&&Number.isFinite(Buffer.from(data.bits,'hex').readDoubleBE()),current,'json-number','finite exact JSON binary64 bits required');
          jsonRequirements.push({node:row.id,typeReference:row.typeReference,bits:data.bits,representation:'unresolved native number variant'});
        }else if(data.kind==='string')requireValue(typeof data.value==='string'&&wellFormedString(data.value),current,'string','owned JSON string required');
        else if(data.kind==='boolean')requireValue(typeof data.value==='boolean',current,'boolean','owned JSON boolean required');
        else requireValue(data.kind==='null',current,'json','unknown JSON descriptor');
      }
    }
    else if(source.kind==='usize'||source.kind==='i64'){
      requireValue(exact(row.data,['decimal','bits'])&&typeof row.data.decimal==='string'&&row.data.decimal.length<=21&&/^(?:0|-?[1-9][0-9]*)$/u.test(row.data.decimal)&&typeof row.data.bits==='string'&&/^[0-9a-f]{16}$/u.test(row.data.bits),current,'integer','closed canonical integer representation required');
      const integer=BigInt(row.data.decimal),represented=Number(integer),checked=readInteger(represented,source.kind,limits,current);
      requireValue(checked.decimal===row.data.decimal&&checked.bits===row.data.bits,current,'integer','integer spelling, range or binary64 bits differ');
    }else refuse(current,'source-type','unsupported owned source type');
    if(['named','type_item','Box','reference','Vec','IndexMap','struct_item'].includes(source.kind))requireValue(row.data===null,current,'model-row','structural nodes must carry null data');
    requireValue(row.children.length===expected.length,current,'model-row','complete source child count required');
    edges+=expected.length;metadataBytes+=bytes(row.typeReference)+bytes(row.form);
    if(source.kind==='enum_item')metadataBytes+=bytes(row.data);
    else scalarBytes+=row.data===null?0:typeof row.data==='string'?bytes(row.data):bytes(JSON.stringify(row.data));
    requireValue(edges<=limits.edgeLimit&&scalarBytes<=limits.byteLimit&&metadataBytes<=limits.metadataByteLimit,current,'model-budget','owned model exceeds its resource limits');
    for(let index=expected.length-1;index>=0;index--){const id=row.children[index];
      requireValue(Number.isSafeInteger(id)&&id>row.id&&id<=model.nodes.length,current,'model-edge','invalid or cyclic owned child');
      pending.push({id,typeReference:expected[index],path:route(current.path,index),depth:current.depth+1});
    }
  }
  requireValue(visited===model.nodes.length,frame,'model-row','unreachable owned rows');
  requireValue(JSON.stringify(model.borrowRequirements)===JSON.stringify(borrowRequirements),frame,'borrow-manifest','exact unresolved static-borrow manifest required');
  if(recursiveJson){
    requireValue(JSON.stringify(model.jsonRequirements)===JSON.stringify(jsonRequirements),frame,'json-manifest','exact unresolved native JSON number manifest required');
    requireValue(JSON.stringify(model.jsonAliases)===JSON.stringify(jsonAliases),frame,'json-alias','exact projected JSON alias manifest required');
    const classes=new Map(),classOf=new Map(),ends=new Map();
    for(const row of [...model.nodes].reverse()){
      ends.set(row.id,row.children.length?ends.get(row.children.at(-1)):row.id);
      if(row.form!=='json')continue;
      const {aliasOf,...data}=row.data;
      const signature=JSON.stringify([data,row.children.map(id=>classOf.get(id))]);
      if(!classes.has(signature))classes.set(signature,classes.size+1);
      classOf.set(row.id,classes.get(signature));
    }
    for(const alias of jsonAliases){
      const row=model.nodes[alias.node-1],first=model.nodes[alias.firstNode-1];
      requireValue(first.form==='json'&&first.data.kind===row.data.kind&&first.data.aliasOf===null&&ends.get(first.id)<row.id&&classOf.get(row.id)===classOf.get(first.id),frame,'json-alias','JSON alias requires a complete equal earlier subtree outside its ancestors');
      for(const [index,id] of row.children.entries()){
        const child=model.nodes[id-1],original=model.nodes[first.children[index]-1];
        if(['array','object'].includes(child.data.kind))requireValue(child.data.aliasOf===(original.data.aliasOf??original.id),frame,'json-alias','nested JSON alias identities must agree at every child');
      }
    }
  }
}

/** Project only independently validated owned nodes, never a retained payload. */
export function projectOrdinaryOutcome(authority,model,options={}) {
  const plan=readOrdinaryInversePlan(authority),limits=readOptions(plan,options),context=createContext(plan,limits);
  inspectOwnedModel(context,model,authority.sha256);
  const values=new Map();
  for(let index=model.nodes.length-1;index>=0;index--){
    const row=model.nodes[index],source=context.nodes.get(row.typeReference),children=row.children.map(id=>values.get(id));let value;
    if(['named','type_item','Box','reference'].includes(source.kind))value=children[0];
    else if(source.kind==='Option')value=row.data?children[0]:absent;
    else if(source.kind==='Vec')value=children;
    else if(source.kind==='IndexMap'){
      value=new Map();for(let item=0;item<children.length;item+=2){
        requireValue(!value.has(children[item]),{typeReference:row.typeReference,path:null},'mapping','duplicate owned map key');value.set(children[item],children[item+1]);
      }
    }else if(source.kind==='struct_item'||source.kind==='enum_item'){
      const binding=context.bindings.get(source.identity),selected=source.kind==='enum_item'?binding.variants.find(item=>item.identity===row.data):binding;
      value=source.kind==='enum_item'?{$variant:selected.nativeName}:{};
      if(selected.fields)selected.fields.forEach((field,item)=>{if(children[item]!==absent)Object.defineProperty(value,field.nativeName,{value:children[item],enumerable:true,writable:true,configurable:true});});
      else {assert.equal(children.length,1);value.payload=children[0];}
    }else if(source.kind==='json'&&plan.admission.opaqueJson!=='null only'){
      if(row.data.kind==='number')value=Buffer.from(row.data.bits,'hex').readDoubleBE();
      else if(row.data.kind==='array')value=children;
      else if(row.data.kind==='object'){
        const target={};row.data.keys.forEach((key,item)=>Object.defineProperty(target,key,{value:children[item],enumerable:true,configurable:true,writable:true}));
        // Only independently checked owned keys enter this internal ordered view.
        value=new Proxy(target,{ownKeys:()=>[...row.data.keys]});
      }else value=row.data.kind==='null'?null:row.data.value;
    }else if(source.kind==='usize'||source.kind==='i64')value=BigInt(row.data.decimal);
    else value=row.data;
    values.set(row.id,value);
  }
  const replacements=new Map((model.jsonAliases??[]).map(row=>[values.get(row.node),values.get(row.firstNode)]));
  const substitute=value=>replacements.get(value)??value;
  for(const [id,value] of values){
    if(value===null||typeof value!=='object')continue;
    if(value instanceof Map)for(const [key,item] of value)value.set(key,substitute(item));
    else for(const key of Object.keys(value))value[key]=substitute(value[key]);
    values.set(id,substitute(value));
  }
  const native=values.get(1),declaration=context.nodes.get(limits.rootReference),type={kind:'owned',name:declaration.name};
  preflightOutcome(plan.wirePlan,native,limits.codec,type);
  return {native,events:[...outcomeWireEvents(plan.wirePlan,native,type,{maximumFrames:limits.codec.nodeLimit})]};
}

/** Independent ordered projection events, with no access to owned model rows. */
function projectionEvents(value,limits) {
  const pending=[{kind:'value',value,depth:1}],events=[];
  while(pending.length){
    const frame=pending.pop();assert.ok(events.length<limits.nodeLimit*4,'projection event resource limit');
    if(frame.kind!=='value'){events.push(frame);continue;}
    const item=frame.value,depth=frame.depth;
    if(item===null){events.push({kind:'null',depth});continue;}
    if(typeof item!=='object'){events.push({kind:typeof item,value:item,depth,...(frame.key?{key:true}:{})});continue;}
    const array=Array.isArray(item);events.push({kind:array?'array':'object',depth});
    pending.push({kind:array?'endArray':'endObject',depth});
    if(array){for(let index=item.length-1;index>=0;index--)pending.push({kind:'value',value:item[index],depth:depth+1});}
    else for(const key of Object.keys(item).reverse())pending.push(
      {kind:'endPair',depth:depth+1},{kind:'value',value:item[key],depth:depth+2},
      {kind:'value',value:key,depth:depth+2,key:true},{kind:'pair',depth:depth+1});
    assert.ok(pending.length<=limits.nodeLimit,'projection frame resource limit');
  }
  events.push({kind:'root',depth:0});return events;
}

import {acceptOwnedOutcomeIntegerValue} from './integer-rules.mjs';
