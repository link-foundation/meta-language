// The existing generated fold is the executable algorithm. Its compiler ABI
// and complete body are checked independently, including unused declarations.
import {typeError,unsupported} from './diagnostics.js';
import {sameType} from './types.js';
import {qualifyRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {createRustBorrowedVectorElementPosition} from './rust-borrowed-vectors.js';
import {checkRustSourceExpressionPosition} from './rust-source-expression-positions.js';
import {checkOwnedProgramContainers} from './owned-program-containers.js';
import {acceptRustDataTypeFieldSignatureContainer} from './rust-data-type-field-signature-rules.js';
import {hasRustSourceRegionComposition} from './rust-source-region-composition.js';

const model='Rust fallible iterator compiler interface 1';
const fail=reason=>{throw typeError(reason);};
const exact=(value,keys)=>value&&typeof value==='object'&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));
const sameOwner=(left,right)=>Boolean(left&&right)&&left.kind===right.kind&&left.path.length===right.path.length&&left.path.every((part,index)=>part===right.path[index])&&left.arguments.length===right.arguments.length&&left.arguments.every((part,index)=>sameOwner(part,right.arguments[index]));
const direct=position=>position.wrappers.length===0&&position.lifetimes.length===0&&!position.arrayElementPosition;
const standard=(owner,path,count)=>owner.kind==='library'&&owner.path.join('::')===path&&owner.arguments.length===count;

export function attachRustFallibleIteratorInterface(parser,entry,parameters,item,mapped,result){
 if(parser.origins.collecting||!hasRustSourceRegionComposition(parser))return entry;
 const position=origin=>{
  const actual=qualifyRustStandardSumSourceOrigin(parser,origin);
  if(!actual)throw unsupported('Rust fallible iterator interface','every actual generated formal, borrowed item and Result retains its complete source owner',entry.span);
  return createRustBorrowedVectorElementPosition(actual);
 };
 return {...entry,sourceFallibleIterator:{model,parameters:parameters.map(position),item:position(item),mapped:position(mapped),result:position(result),span:{...entry.span}}};
}

function resultFields(position,resolveData,resolveType){
 const data=position.type.kind==='data'?resolveData(position.type):null;
 if(!direct(position)||!standard(position.owner,'std::result::Result',2)||data?.k!=='data'||data.ctors.length!==2)fail('a fallible iterator retains its actual complete direct standard Result owner');
 const ok=data.ctors.find(row=>row.name==='Ok'),error=data.ctors.find(row=>row.name==='Err');
 if(ok?.fields.length!==1||error?.fields.length!==1)fail('a fallible iterator retains both actual one-payload Result constructors');
 return {ok:resolveType(ok.fields[0].type),error:resolveType(error.fields[0].type)};
}

export function checkRustFallibleIteratorInterface(facet,entry,language,resolveType=value=>value,resolveData=()=>null){
 checkOwnedProgramContainers({sourceFallibleIterator:facet},{selectedFields:['sourceFallibleIterator'],acceptContainer:acceptRustDataTypeFieldSignatureContainer,fail});
 if(language!=='Rust'||!exact(facet,['model','parameters','item','mapped','result','span'])||facet.model!==model||entry.k!=='fn'||entry.generated!==true||!Array.isArray(facet.parameters)||facet.parameters.length<2||facet.parameters.length>258||entry.params.length!==facet.parameters.length||new Set(entry.params.map(row=>row.name)).size!==entry.params.length||entry.params.some(row=>typeof row.name!=='string'||!row.name)||entry.ret.kind!=='tuple'||entry.ret.elements.length!==facet.parameters.length-1||!exact(facet.span,['start','end'])||!Number.isSafeInteger(facet.span.start)||!Number.isSafeInteger(facet.span.end)||facet.span.start<0||facet.span.end<=facet.span.start||entry.span?.start!==facet.span.start||entry.span?.end!==facet.span.end)fail('a fallible iterator retains its exact generated declaration, original span and complete ordered result/state ABI');
 for(const field of ['sourceSignature','sourceExclusiveSignature','sourceFnMutSpecialization','sourceStateTransform','sourceVectorMutation','sourceFiniteRecordLoopHelper'])if(entry[field]!==undefined)fail('a fallible iterator compiler helper has no competing callable or state interface');
 const position=(value,type)=>checkRustSourceExpressionPosition({model:'Rust borrowed expression source position 1',position:value},type,language,resolveType,resolveData).position;
 const parameters=facet.parameters.map((value,index)=>position(value,entry.params[index].type)),vector=parameters[0],callable=parameters[1],callback=callable.type;
 if(vector.type.kind!=='array'||!standard(vector.owner,'std::vec::Vec',1)||vector.wrappers.length>1||vector.wrappers.some(layer=>layer!=='reference')||vector.arrayElementPosition||!direct(callable)||callback.kind!=='function'||callback.params.length!==parameters.length-1||callback.ret.kind!=='tuple'||callback.ret.elements.length!==parameters.length-1)fail('a fallible iterator retains its actual contiguous readonly Vec and complete callback state function');
 const item=position(facet.item,vector.type.element),mapped=position(facet.mapped,callback.ret.elements[0]),result=position(facet.result,entry.ret.elements[0]);
 const returned=callable.owner.arguments.at(-1);
 if(item.wrappers.length!==1||item.wrappers[0]!=='reference'||item.arrayElementPosition||!sameType(item.type,callback.params[0])||!sameOwner(item.owner,vector.owner.arguments[0])||!sameOwner(item.owner,callable.owner.arguments[0])||returned.kind!=='tuple'||!sameOwner(mapped.owner,returned.arguments[0]))fail('a fallible mapper receives exactly its actual shared standard vector element and returns its actual Result owner');
 const source=resultFields(mapped,resolveData,resolveType),output=resultFields(result,resolveData,resolveType);
 if(output.ok.kind!=='array'||!sameType(output.ok.element,source.ok)||!sameType(output.error,source.error)||!standard(result.owner.arguments[0],'std::vec::Vec',1)||!sameOwner(result.owner.arguments[0].arguments[0],mapped.owner.arguments[0])||!sameOwner(result.owner.arguments[1],mapped.owner.arguments[1]))fail('fallible collect retains its actual ordered success vector and unchanged error owner');
 for(let index=2;index<parameters.length;index++)if(!direct(parameters[index])||!sameType(parameters[index].type,callback.params[index-1])||!sameType(parameters[index].type,callback.ret.elements[index-1])||!sameType(parameters[index].type,entry.ret.elements[index-1])||!sameOwner(parameters[index].owner,callable.owner.arguments[index-1])||!sameOwner(parameters[index].owner,returned.arguments[index-1]))fail('each fallible iterator environment slot keeps its exact direct input, callback writeback and output type and owner');
 return {model,parameters,item,mapped,result,span:{...facet.span}};
}

const variable=(node,name)=>node?.k==='var'&&node.name===name;
function projection(node,name,index){if(node?.k!=='tupleIndex'||node.index!==index||!variable(node.object,name))fail('a fallible iterator retains its actual ordered result/state projection');}
function packet(node,width){if(node?.k!=='tuple'||node.items.length!==width)fail('a fallible iterator retains every actual result/state packet slot');return node.items;}
function result(node,position,tag){if(node?.k!=='ctor'||node.data!==position.type.name||node.ctor!==tag||node.args.length!==1||!sameType(node.type,position.type))fail('a fallible iterator retains its actual selected Result payload construction');return node.args[0];}
function matchProjection(node,binding,index,names){
 // General match compilation evaluates a tuple projection in one local first.
 if(node?.k!=='let'||typeof node.name!=='string'||!node.name||names.has(node.name))fail('a fallible iterator evaluates each matched result once in a distinct actual binder');
 projection(node.value,binding,index);names.add(node.name);const match=node.body;
 if(match?.k!=='match'||!variable(match.scrutinee,node.name)||match.cases.length!==2)fail('a fallible iterator retains both actual Result branches');
 return match;
}
function branches(match,position,names){
 const selected={};
 for(const tag of ['Ok','Err']){
  const row=match.cases.find(arm=>arm.pattern.k==='ctor'&&arm.pattern.data===position.type.name&&arm.pattern.ctor===tag),name=row?.pattern.binds?.[0];
  if(!row||row.pattern.binds.length!==1||typeof name!=='string'||!name||names.has(name))fail('a fallible iterator retains distinct actual Ok/Err payload bindings');
  names.add(name);selected[tag]={name,body:row.body};
 }
 return selected;
}
function writebacks(fields,binding,count){for(let index=0;index<count;index++)projection(fields[index+1],binding,index+1);}

export function checkRustFallibleIteratorBody(entry,facet){
 const pending=[entry.body],seen=new Set();while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;seen.add(node);if(seen.size>100000)fail('the complete fallible iterator body exceeds its inspection budget');for(const[key,child]of Object.entries(node)){if(key.startsWith('source'))fail('fallible iterator plumbing carries no competing source qualifier or stored proof certificate');if(!['type','span','domain','from','to'].includes(key)&&child&&typeof child==='object'){if(Array.isArray(child))pending.push(...child);else pending.push(child);}}}
 const fold=entry.body,input=entry.params[0].name,mapper=entry.params[1].name,width=entry.ret.elements.length,count=entry.params.length-2;
 if(fold.k!=='method'||fold.op!=='reduce'||!variable(fold.object,input)||fold.args.length!==2||!sameType(fold.type,entry.ret))fail('a fallible iterator retains its actual initialized ordered input fold');
 const callback=fold.args[0];if(callback?.k!=='lambda'||callback.params.length!==2||callback.captures!==undefined&&(!Array.isArray(callback.captures)||callback.captures.length!==1||callback.captures[0]!==mapper)||callback.type?.kind!=='function'||!sameType(callback.type.ret,entry.ret)||!sameType(callback.params[0].type,entry.ret)||!sameType(callback.params[1].type,facet.item.type))fail('a fallible fold retains its actual accumulator/item lambda and sole mapper capture');
 const accumulator=callback.params[0].name,item=callback.params[1].name,names=new Set(entry.params.map(row=>row.name));for(const name of [accumulator,item]){if(typeof name!=='string'||!name||names.has(name))fail('a fallible fold retains distinct actual formal and item binders');names.add(name);}
 const initial=packet(fold.args[1],width),empty=result(initial[0],facet.result,'Ok');if(empty.k!=='array'||empty.items.length||empty.type.kind!=='array'||!sameType(fold.args[1].type,entry.ret))fail('fallible collection starts with its actual empty success vector and complete typed state packet');
 for(let index=0;index<count;index++)if(!variable(initial[index+1],entry.params[index+2].name))fail('a fallible iterator seeds every actual environment slot in order');
 const outer=branches(matchProjection(callback.body,accumulator,0,names),facet.result,names);
 if(!variable(outer.Err.body,accumulator))fail('an existing Err keeps the complete accumulator and never invokes the callback again');
 const called=outer.Ok.body;if(called?.k!=='let'||typeof called.name!=='string'||!called.name||names.has(called.name)||called.value?.k!=='apply'||!variable(called.value.fn,mapper)||called.value.args.length!==count+1||!variable(called.value.args[0],item)||!sameType(called.value.type,entry.params[1].type.ret))fail('each successful input invokes exactly the actual mapper once with its current item/state packet');names.add(called.name);
 for(let index=0;index<count;index++)projection(called.value.args[index+1],accumulator,index+1);
 const inner=branches(matchProjection(called.body,called.name,0,names),facet.mapped,names),failed=packet(inner.Err.body,width),succeeded=packet(inner.Ok.body,width);
 if(!variable(result(failed[0],facet.result,'Err'),inner.Err.name))fail('a new Err propagates its actual unchanged error payload');writebacks(failed,called.name,count);
 const append=result(succeeded[0],facet.result,'Ok');if(append.k!=='append'||!sameType(append.type,empty.type)||!variable(append.left,outer.Ok.name)||append.right?.k!=='array'||!sameType(append.right.type,empty.type)||append.right.items.length!==1||!variable(append.right.items[0],inner.Ok.name))fail('successful collection appends exactly the actual mapped item after its complete preceding success prefix');writebacks(succeeded,called.name,count);
 return entry;
}
