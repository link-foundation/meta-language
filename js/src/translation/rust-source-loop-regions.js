// Symbolic source interpretation of the actual checked fold, including its
// zero-iteration path. These local values/epochs are never serialized.
import {unsupported} from './diagnostics.js';
import {sameType} from './types.js';
import {checkRustOwnedVectorSourceFold} from './rust-source-loop-folds.js';
import {divergeRustSourceRegions,isRustSourceRegionDivergence} from './rust-source-region-divergence.js';
import * as rules from './rust-borrowed-source-region-rules.js';
const fail=(reason,node)=>{throw unsupported('Rust source loop regions',reason,node?.span);};
const packetRole='source loop packet',controlRole='source loop control';
export function readRustSourceLoopTransport(visitor,value,mode,node){
 if(![packetRole,controlRole].includes(value.role))return null;
 if(value.role===packetRole){if(!Array.isArray(value.fields)||value.fields.length!==value.loop.slots.length+2||value.fields.length>514)fail('a loop packet retains its complete direct state inventory',node);for(const field of value.fields)visitor.read(field,'read',node);}
 if(mode==='move'||mode==='transport'){if(value.root===null&&!value.producer)fail('a loop carrier needs its actual available producer',node);if(value.root!==null)visitor.states[value.root]=rules.consumeRustBorrowedSourceRegion(visitor.states[value.root]);return {...value,root:null,producer:true};}
 return value;
}
function matchingFrame(visitor,node,control=false){return (visitor.loopFrames??[]).findLast(frame=>sameType(control?frame.facet.control:frame.type,node.type));}
function sameLoop(left,right){return sameType(left.control,right.control)&&sameType(left.result.type,right.result.type)&&left.slots.length===right.slots.length&&left.slots.every((slot,index)=>slot.binder.identity===right.slots[index].binder.identity);}
export function readRustSourceLoopField(visitor,value,index,node){
 if(value.role!==packetRole)return null;
 if(!Number.isSafeInteger(index)||index<0||index>=value.fields.length||!sameType(value.packetType.elements[index],node.type))fail('a loop field retains its actual complete packet and slot type',node);
 const field=value.fields[index],state=index>=2,held=visitor.read(field,state?'transport':'move',node);
 return {...held,root:null,producer:true,storageRoot:state?field.storageRoot:undefined,storageIdentity:state?field.storageIdentity:undefined,storageOwner:false,packetState:state};
}
export function joinRustSourceLoopValues(visitor,values,node){
 const first=values[0];if(![packetRole,controlRole].includes(first.role))return null;
 if(values.some(value=>value.role!==first.role||!sameLoop(value.loop,first.loop)))fail('loop alternatives retain their actual common control and source slot inventory',node);
 if(first.role===packetRole){if(values.some(value=>!sameType(value.packetType,first.packetType)||value.fields.length!==first.fields.length))fail('loop alternatives retain every actual accumulator type',node);const fields=first.fields.map((_,index)=>visitor.joinValues(values.map(value=>value.fields[index]),node));return {...first,root:null,producer:true,fields,regions:visitor.union(fields.map(field=>field.regions))};}
 const variants=new Map();for(const value of values)for(const variant of value.variants){const prior=variants.get(variant.name);if(prior&&Boolean(prior.payload)!==Boolean(variant.payload))fail('one actual compiler control variant retains its payload arity',node);variants.set(variant.name,prior?.payload?{name:variant.name,payload:visitor.joinValues([prior.payload,variant.payload],node)}:variant);}
 return {...first,root:null,producer:true,variants:[...variants.values()],regions:visitor.union([...variants.values()].map(variant=>variant.payload?.regions??[]))};
}
function assertSlots(visitor,value,facet,node){
 if(value.role!==packetRole||!sameLoop(value.loop,facet)||value.fields.length!==facet.slots.length+2)fail('a loop result publishes its exact actual source state packet',node);
 for(let index=0;index<facet.slots.length;index++){const slot=facet.slots[index],field=value.fields[index+2],storage=visitor.storages.get(slot.binder.identity);if(field.storageIdentity!==slot.binder.identity||!storage||storage.root!==field.storageRoot||field.regions.length)fail('every loop state slot publishes its current closed source binder revision',node);visitor.compatible(visitor.read(field,'read',node),slot.position,node);}
}
export function readRustSourceLoopTuple(visitor,node,env,bindings,depth){
 const frame=matchingFrame(visitor,node);if(!frame||node.k!=='tuple')return null;
 if(node.sourcePosition||node.sourceStateTransition||node.items.length!==frame.facet.slots.length+2)fail('a compiler loop packet cannot replace an ordinary source qualifier',node);
 const fields=[];for(let index=0;index<node.items.length;index++){const actual=visitor.expression(node.items[index],env,bindings,null,depth+1),state=index>=2;
  if(index===0&&(actual.role!=='copy'||actual.position?.type.kind!=='bool'))fail('the loop done flag is its actual direct Boolean',node.items[index]);
  if(index===1&&(actual.role!==controlRole||!sameLoop(actual.loop,frame.facet)))fail('the loop control value belongs to its actual compiler declaration',node.items[index]);
  if(state){const slot=frame.facet.slots[index-2],storage=visitor.storages.get(slot.binder.identity);if(actual.storageIdentity!==slot.binder.identity||!storage||storage.root!==actual.storageRoot||actual.regions.length)fail('loop packet state retains its actual current closed binder',node.items[index]);visitor.compatible(actual,slot.position,node.items[index]);}
  const held=visitor.read(actual,state?'transport':'move',node.items[index]);fields.push({...held,root:visitor.root(),producer:true,storageRoot:state?actual.storageRoot:undefined,storageIdentity:state?actual.storageIdentity:undefined,storageOwner:false,packetState:state});
 }
 if(node.items[0].k!=='lit'||typeof node.items[0].value!=='boolean'||node.items[0].value===false&&fields[1].variants.some(variant=>variant.name!=='StatementContinuation'))fail('every continuing loop packet retains its actual continuation control invariant',node);
 return {...visitor.value(null,visitor.root('',true),visitor.union(fields.map(field=>field.regions)),true),role:packetRole,loop:frame.facet,packetType:node.type,fields};
}
export function readRustSourceLoopConstructor(visitor,node,env,bindings,depth){
 const frame=matchingFrame(visitor,node,true);if(!frame||node.k!=='ctor')return null;
 const declaration=visitor.program.declarations.get(node.data),ctor=declaration?.ctors.find(ctor=>ctor.name===node.ctor),returned=node.ctor==='FunctionResult';
 if(node.data!==frame.facet.control.name||!ctor||node.args.length!==(returned?1:0))fail('a source loop constructor retains its actual compiler control identity',node);
 const payload=returned?visitor.read(visitor.compatible(visitor.expression(node.args[0],env,bindings,frame.facet.result,depth+1),frame.facet.result,node.args[0]),'move',node.args[0]):null;
 return {...visitor.value(null,visitor.root('',true),payload?.regions??[],true),role:controlRole,loop:frame.facet,variants:[{name:node.ctor,payload}]};
}
export function readRustSourceLoopMatch(visitor,node,scrutinee,env,bindings,expected,depth){
 if(scrutinee.role!==controlRole)return null;scrutinee=visitor.read(scrutinee,'move',node.scrutinee);
 const count=visitor.states.length,base=visitor.states.slice(),storageBase=new Map(visitor.storages),outcomes=[],storageOutcomes=[],results=[];
 for(const arm of node.cases){if(arm.pattern.k!=='ctor'||arm.pattern.data!==scrutinee.loop.control.name)fail('loop control matching retains actual complete nominal constructor patterns',node);const variant=scrutinee.variants.find(value=>value.name===arm.pattern.ctor);if(!variant)continue;
  if(arm.pattern.binds.length!==(variant.payload?1:0))fail('a loop return pattern retains its actual payload binding',node);
  for(let index=0;index<count;index++)visitor.states[index]=base[index];visitor.storages=new Map(storageBase);const local=new Map(env),ids=[],start=visitor.temporaries.length;
  if(variant.payload&&arm.pattern.binds[0]!==null)ids.push(visitor.bind(local,arm.pattern.binds[0],{...variant.payload,root:null,producer:true},node.scrutinee));
  let result;visitor.branches(2*count);try{result=visitor.read(visitor.expression(arm.body,local,bindings,expected,depth+1),'move',arm.body);for(const id of ids)visitor.states[id]=rules.expireRustBorrowedSourceRegion(visitor.states[id]);visitor.finishExpression(result,start,arm.body);}catch(error){if(isRustSourceRegionDivergence(error))continue;throw error;}results.push(result);outcomes.push(visitor.states.slice(0,count));storageOutcomes.push(new Map(visitor.storages));
 }
 if(!results.length){if(!node.cases.some(arm=>scrutinee.variants.some(variant=>variant.name===arm.pattern.ctor)))fail('a loop control match retains at least one actual reachable alternative',node);divergeRustSourceRegions();}
 for(let index=0;index<count;index++){let state=outcomes[0][index];for(const outcome of outcomes.slice(1))state=rules.mergeRustBorrowedSourceRegion(state,outcome[index]);visitor.states[index]=state;}
 visitor.mergeStorages(storageBase,storageOutcomes,node,results);return visitor.joinValues(results,node);
}
function freeBindings(visitor,node,parameters){
 const names=new Set(),pending=[{node,bound:new Set(parameters)}];
 while(pending.length){const {node:value,bound}=pending.pop();if(!value||typeof value!=='object')continue;visitor.visits++;visitor.budget();if(value.k==='var'){if(!bound.has(value.name))names.add(value.name);continue;}
  if(value.k==='let'){pending.push({node:value.value,bound},{node:value.body,bound:new Set([...bound,value.name])});continue;}
  if(value.k==='lambda'){pending.push({node:value.body,bound:new Set([...bound,...value.params.map(parameter=>parameter.name)])});continue;}
  if(value.k==='match'){pending.push({node:value.scrutinee,bound});for(const arm of value.cases){const inner=new Set(bound);if(arm.pattern.name)inner.add(arm.pattern.name);for(const name of arm.pattern.binds??[])if(name!==null)inner.add(name);pending.push({node:arm.body,bound:inner});}continue;}
  for(const[key,child]of Object.entries(value))if(!['type','span'].includes(key)&&!key.startsWith('source')&&child&&typeof child==='object'){if(Array.isArray(child))for(const item of child)pending.push({node:item,bound});else pending.push({node:child,bound});}
 }
 return names;
}
function captureRoots(value){
 const result=new Set(),seen=new Set(),pending=[value];while(pending.length){const current=pending.pop();if(seen.has(current))continue;seen.add(current);if(seen.size>100000)fail('actual loop capture regions exceed their finite inventory');for(const root of [current.root,current.storageRoot,...current.regions])if(root!=null)result.add(root);pending.push(...(current.fields??[]));for(const variant of current.variants??[])if(variant.payload)pending.push(variant.payload);}return result;
}
export function readRustOwnedVectorFoldRegions(visitor,node,env,bindings,depth){
 const facet=checkRustOwnedVectorSourceFold(node.sourceLoopFold,node,visitor.program.sourceLanguage,value=>value,value=>visitor.program.declarations.get(value.name));if(!visitor.composed)fail('a source loop needs the actual closed-call region composition',node);
 for(const position of [facet.input,facet.item,...facet.slots.map(slot=>slot.position)]){visitor.scope(position,bindings,node);visitor.closedOutput(position,node);}visitor.scope(facet.result,bindings,node);
 const input=visitor.compatible(visitor.expression(node.object,env,bindings,facet.input,depth+1),facet.input,node.object);visitor.read(input,'move',node.object);
 const frame={facet,type:node.type},frames=visitor.loopFrames??=[];if(frames.length>=128)fail('nested source loop interfaces exceed their finite region depth',node);frames.push(frame);
 try{
  const initial=visitor.expression(node.args[1],env,bindings,null,depth+1);assertSlots(visitor,initial,facet,node.args[1]);const callback=node.args[0],count=visitor.states.length,base=visitor.states.slice(),storageBase=new Map(visitor.storages),local=new Map(env),ids=[],captures=freeBindings(visitor,callback.body,callback.params.map(parameter=>parameter.name));
  ids.push(visitor.bind(local,callback.params[0].name,{...initial,root:null,producer:true},node.args[1]));ids.push(visitor.bind(local,callback.params[1].name,visitor.value(visitor.position(facet.item),null,[],true),callback));
  let result;try{result=visitor.read(visitor.expression(callback.body,local,bindings,null,depth+1),'move',callback.body);}catch(error){if(!isRustSourceRegionDivergence(error))throw error;for(let index=0;index<count;index++)visitor.states[index]=base[index];visitor.storages=new Map(storageBase);for(const id of ids)visitor.states[id]=rules.expireRustBorrowedSourceRegion(visitor.states[id]);visitor.read(initial,'read',node.args[1]);return initial;}assertSlots(visitor,result,facet,callback.body);
  for(const[id,storage]of storageBase)if(!facet.slots.some(slot=>slot.binder.identity===id)&&visitor.storages.get(id)?.root!==storage.root)fail('noncarried loop state retains its unchanged actual source revision',node);
  if(visitor.storages.size!==storageBase.size)fail('iteration-local storage cannot escape a source loop body',node);
  for(const name of captures){const value=env.get(name);if(!value)fail('loop captures retain their actual enclosing lexical binders',node);for(const root of captureRoots(value))if(root<count&&visitor.states[root]!==base[root])fail('every repeated loop capture retains ownership and live loan epochs on all paths',node);visitor.read(value,'read',node);}
  for(const id of ids)visitor.states[id]=rules.expireRustBorrowedSourceRegion(visitor.states[id]);visitor.read(result,'read',callback.body);
  const step=visitor.states.slice(0,count),storageStep=new Map(visitor.storages),results=[initial,result];visitor.branches(2*count);for(let index=0;index<count;index++)visitor.states[index]=rules.mergeRustBorrowedSourceRegion(base[index],step[index]);visitor.mergeStorages(storageBase,[storageBase,storageStep],node,results);const joined=visitor.joinValues(results,node);assertSlots(visitor,joined,facet,node);return joined;
 }finally{if(frames.at(-1)!==frame)fail('source loop region scopes close in their actual nesting order',node);frames.pop();}
}
