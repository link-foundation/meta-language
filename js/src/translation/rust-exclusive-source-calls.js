// Actual lexical source places, selected callees and private ordered state
// operations. Nothing here executes an original Rust or host callback body.
import {unsupported,typeError} from './diagnostics.js';
import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustOwnedRecordStateSlots,readRustOwnedRecordStatePlace,readRustOwnedRecordStateLayout,readRustOwnedRecordMutableName,testRustOwnedRecordStatePlaceLoans} from './rust-owned-record-state.js';
import {readRustOwnedRecordStateFieldIdentity} from './rust-owned-record-state-rules.js';
import {testRustOwnedRecordFieldDescendant,readRustOwnedRecordWholeError} from './rust-owned-record-update-rules.js';
import {testRustExclusiveSourcePlaceOverlap} from './rust-exclusive-source-loan-rules.js';
import {readRustSourceValueName} from './rust-source-constants.js';
const fail=(reason,span)=>{throw unsupported('Rust exclusive source call',reason,span);};
const operations={borrow:'rustExclusiveSourceBorrow',call:'rustExclusiveSourceCall',write:'rustExclusiveSourceWholeWrite'};
export function initializeRustExclusiveSourceCalls(parser){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1'&&parser.standardCollectionSourceModel!=='Rust closed standard collections 1')return null;
 if(parser.exclusiveSourceCalls)return parser.exclusiveSourceCalls;
 const arena={borrows:[],calls:[],writes:[],frames:[],receivers:new WeakMap()};parser.exclusiveSourceCalls=arena;
 const guard=(slot,fields,span)=>{for(const frame of arena.frames)for(const borrow of frame.borrows)if(testRustExclusiveSourcePlaceOverlap(slot.lexical,fields,borrow.slot.lexical,borrow.fields))fail('an earlier selected exclusive argument still holds this overlapping source place',span);};
 parser.exclusiveSourceReadGuard=guard;parser.exclusiveSourceWriteGuard=guard;return arena;
}
export function selectRustExclusiveSourcePlace(parser,node,path){
 const origin=parser.origins.origin(node,path),identity=origin.sourcePlaceBindingIdentity??origin.sourceBindingIdentity,slots=readRustOwnedRecordStateSlots(parser);
 let slot=slots.find(row=>row.lexical===origin.sourceLexicalBindingIdentity||identity===row.root||identity&&testRustOwnedRecordFieldDescendant(row.root,identity));if(!slot)return null;
 const layouts=[],fields=[];let current=slot.origin,currentIdentity=slot.root;
 while(identity&&identity!==currentIdentity){if(fields.length>=128)fail('the actual exclusive receiver place exceeds its field projection bound',node.span);const found=readRustOwnedRecordStateLayout(parser,current);if(!found)return null;let index=-1;for(let at=0;at<found.entry.fields.length;at++){const child=readRustOwnedRecordStateFieldIdentity(currentIdentity,[at]);if(child===identity||testRustOwnedRecordFieldDescendant(child,identity)){index=at;currentIdentity=child;break;}}if(index<0)return null;layouts.push({...found,index,origin:current,field:found.entry.fields[index]});fields.push(index);current=found.entry.fields[index];}
 return {slot,layouts,fields,field:current};
}
function acquire(parser,place,parameter,span){
 const arena=initializeRustExclusiveSourceCalls(parser),frame=arena?.frames.at(-1);if(!frame||!place)fail('an exclusive argument needs its actual selected callable and mutable lexical place',span);
 if(place.slot.callable!==frame.callable)fail('exclusive arguments cannot capture a mutable root from another callable',span);
 for(const pending of arena.frames)for(const borrow of pending.borrows)if(testRustExclusiveSourcePlaceOverlap(place.slot.lexical,place.fields,borrow.slot.lexical,borrow.fields))fail('exclusive arguments retain disjoint actual places until the selected Call',span);
 if(!parser.origins.collecting&&parser.origins.sourceBindingStates.get(place.slot.root)!=='available')fail('an exclusive argument needs its still available actual owner',span);
 if(!parser.origins.collecting){for(let depth=1;depth<=place.fields.length;depth++)if((parser.origins.sourceBindingStates.get(readRustOwnedRecordStateFieldIdentity(place.slot.root,place.fields.slice(0,depth)))??'available')!=='available')fail('an exclusive argument needs every actual selected field initialized',span);const root=readRustOwnedRecordStateFieldIdentity(place.slot.root,place.fields),states=[...parser.origins.sourceBindingStates],error=readRustOwnedRecordWholeError(root,states.map(row=>row[0]),states.map(row=>row[1]));if(error)fail(error,span);}
 if(!parser.origins.collecting&&testRustOwnedRecordStatePlaceLoans(parser,place.slot,place.fields))fail('an exclusive argument overlaps an actual surviving source reference loan',span);
 const info={...place,parameter,span},index=arena.borrows.length;if(index>=100000)fail('exclusive source arguments exceed their finite construction budget',span);arena.borrows.push(info);frame.borrows.push(info);
 const node={k:'math',op:operations.borrow,name:String(index),args:[],span},origin={...place.field,wrappers:['reference'],sourceLifetimes:[null],sourceExclusiveMode:'exclusive',sourceBorrowedPlace:true,sourceBindingIdentity:place.slot.root,sourceLexicalBindingIdentity:place.slot.lexical,sourceLoanBindingIdentities:[place.slot.root]};return parser.origins.mark(node,origin);
}
export function readRustExclusiveSourceBorrow(parser,path,token){
 const arena=initializeRustExclusiveSourceCalls(parser),frame=arena?.frames.at(-1),parameter=frame?.current;
 if(!frame||!parameter||parameter.origin.sourceExclusiveMode!=='exclusive')fail('mutable references are admitted only as actual selected exclusive call operands',token);
 const c=parser.cursor;c.expect('mut','exclusive source borrow');const dereference=Boolean(c.eat('*')),name=c.identifier('exclusive source place');
 const source=parser.origins.scope.get(name.value);if(dereference&&source?.sourceExclusiveMode!=='exclusive')fail('a reborrow dereferences its actual exclusive parameter',name);
 const fields=[];while(c.eat('.')){const field=c.peek();if(field.kind==='number'&&/^(0|[1-9][0-9]*)$/.test(field.raw)){fields.push(c.next().raw);}else fields.push(c.identifier('exclusive source field').value);}
 if(c.is('['))fail('array-index reference aliases need their actual indexed place and invalidation protocol',c.peek());
 const span={start:token.start,end:c.peek(-1).end},place=readRustOwnedRecordStatePlace(parser,name.value,fields,span);return acquire(parser,place,parameter.index,span);
}
function argumentsFor(parser,entry,path,offset=0,receiver=null){
 const arena=initializeRustExclusiveSourceCalls(parser),callable=readRustOwnedRecordStateSlots(parser)[0]?.callable??parser.ownedRecordState?.currentCallable??0,frame={entry,callable,borrows:[],current:null},args=[],c=parser.cursor;if(arena.frames.length>=256||entry.parameters.length>256)fail('exclusive source calls exceed their bounded nesting or formal count',c.peek());arena.frames.push(frame);
 try{
  if(receiver){frame.current={index:0,origin:entry.parameters[0]};args.push(acquire(parser,receiver,0,receiver.span));}
  while(!c.is(')')){
   const index=offset+args.length-(receiver?1:0),expected=entry.parameters[index];frame.current={index,origin:expected??{}};let value;
   if(expected?.sourceExclusiveMode==='exclusive'&&!c.is('&')){
    const token=c.peek(),source=c.peek().kind==='identifier'?parser.origins.scope.get(token.value):null;
    if(source?.sourceExclusiveMode!=='exclusive')fail('an owned argument requires explicit &mut; implicit reborrow needs an actual exclusive formal',token);
    c.next();value=acquire(parser,readRustOwnedRecordStatePlace(parser,token.value,[],token),index,token);
   }else value=withRustStandardSumSourceContext(parser,expected,()=>readRustExpectedCopyClosure(parser,path,expected));
   parser.origins.prepareOwnedArgument(value,expected,path);args.push(value);if(!c.eat(','))break;
  }
  return args;
 }finally{arena.frames.pop();}
}
export function readRustExclusiveSourceCallArguments(parser,callee,path){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1')return null;
 if(callee.path.length===1&&parser.origins.scope.has(callee.path[0]))return null;
 const entry=parser.origins.lookupValueOwner(readRustSourceValueName(parser,callee.path,path),path);return entry?.k==='fn'&&entry.parameters.some(row=>row.sourceExclusiveMode==='exclusive')?argumentsFor(parser,entry,path):null;
}
export function captureRustExclusiveSourceReceiver(parser,receiver,method,path){
 const arena=initializeRustExclusiveSourceCalls(parser);if(!arena)return false;
 const origin=parser.origins.origin(receiver,path),owner=origin.type?.kind==='named'?parser.origins.lookupTypeOwner(origin.type.path,origin.path):null,selected=owner?parser.inherentMethods.get(owner.fullName+'.'+method.value):null;
 const vector=parser.exclusiveSourceModel==='Rust exclusive source loans 1'&&method.value==='push'&&origin.type?.kind==='array'&&(origin.expandedSourceOwner??origin.sourceOwner)?.path.join('::')==='std::vec::Vec';
 if(!selected?.exclusive&&!vector)return false;
 const place=selectRustExclusiveSourcePlace(parser,receiver,path);if(!place)fail('an exclusive receiver requires its actual mutable lexical root and field path',receiver.span);
 const callee=selected?['crate',...owner.fullName.split('.'),method.value]:null,entry=callee?parser.origins.lookupValueOwner(callee,path):null;
 const element=vector?{...parser.origins.typeOrigin(origin.type.element,origin.path),sourceOwner:(origin.expandedSourceOwner??origin.sourceOwner).arguments[0]}:null;
 arena.receivers.set(receiver,{place:{...place,span:receiver.span},callee,entry:entry??{parameters:[{...place.field,sourceExclusiveMode:'exclusive'},element]},vector});return true;
}
export function readRustExclusiveSourceMethodArguments(parser,receiver,method,path){
 const record=parser.exclusiveSourceCalls?.receivers.get(receiver);if(!record)return null;
 const values=argumentsFor(parser,record.entry,path,1,record.place);record.borrow=values[0];return values.slice(1);
}
export function readRustExclusiveSourceCall(parser,callee,args,path,span){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1')return null;
 const entry=parser.origins.lookupValueOwner(callee,path);if(entry?.k!=='fn'||!entry.parameters.some(row=>row.sourceExclusiveMode==='exclusive'))return null;
 if(args.length!==entry.parameters.length)throw typeError('exclusive source call requires its actual complete argument list',span);
 const arena=initializeRustExclusiveSourceCalls(parser),borrows=[];for(let index=0;index<args.length;index++)if(entry.parameters[index].sourceExclusiveMode==='exclusive'){
  const argument=args[index],selected=argument.k==='math'&&argument.op===operations.borrow?arena.borrows[Number(argument.name)]:null;if(!selected||selected.parameter!==index)fail('an exclusive formal retains its actual selected source place operand',span);borrows.push({...selected,index});
 }
 parser.origins.call(callee,args,path,span,true);const index=arena.calls.length;if(index>=100000)fail('exclusive source calls exceed their finite construction budget',span);
 arena.calls.push({callee:[...callee],parameters:entry.parameters,result:entry.result,borrows,span});return parser.origins.mark({k:'math',op:operations.call,name:String(index),args:args.map(value=>({value,spread:false})),span},entry.result);
}
export function readRustExclusiveSourceMethod(parser,receiver,method,args,path,span){
 const info=parser.exclusiveSourceCalls?.receivers.get(receiver);if(!info)return null;
 if(!info.borrow)fail('exclusive receiver operands retain their ordered argument frame',span);
 if(!info.vector)return readRustExclusiveSourceCall(parser,info.callee,[info.borrow,...args],path,span);
 if(args.length!==1)throw typeError('Vec push requires exactly its actual element argument',span);
 parser.origins.argument(parser.origins.origin(args[0],path),info.entry.parameters[1],span);
 const prior=parser.origins.mark({k:'math',op:'rustOwnedRecordStateRead',name:String(parser.ownedRecordEffects.reads.length),args:[],span},info.place.slot.origin);parser.ownedRecordEffects.reads.push(info.place.slot);
 const value=parser.origins.mark({k:'array',items:[{value:prior,spread:true},{value:args[0],spread:false}],span},{...info.place.field,wrappers:[],sourceLifetimes:[],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null,sourceBorrowedPlace:false,sourceExclusiveParameter:false,sourceOwnedResultProof:true});return write(parser,info.place,value,path,span);
}
function write(parser,place,value,path,span){
 const arena=initializeRustExclusiveSourceCalls(parser),index=arena.writes.length;if(index>=100000)fail('exclusive whole-place writes exceed their finite construction budget',span);
 if(place.slot.callable!==parser.ownedRecordState?.currentCallable)fail('an exclusive write cannot capture a mutable root from another callable',span);
 if(!parser.origins.collecting&&(parser.origins.sourceBindingStates.get(place.slot.root)!=='available'||testRustOwnedRecordStatePlaceLoans(parser,place.slot,place.fields)))fail('an exclusive write needs its actual available root without surviving overlapping source references',span);
 if(!parser.origins.collecting)parser.origins.argument(parser.origins.origin(value,path),place.field,span);
 arena.writes.push({...place,value,path,span});return parser.origins.mark({k:'math',op:operations.write,name:String(index),args:[{value,spread:false}],span},parser.origins.typeOrigin({kind:'unit'},path));
}
export function readRustExclusiveSourceWholeAssignment(parser,path){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1')return null;const c=parser.cursor,start=c.peek(),star=c.is('*')?1:0;
 if(c.peek(star).kind!=='identifier'||!c.is('=',star+1))return null;
 const source=parser.origins.scope.get(c.peek(star).value);if(star&&source?.sourceExclusiveMode!=='exclusive')return null;
 const place=readRustOwnedRecordStatePlace(parser,c.peek(star).value,[],start);if(!place)return null;
 parser.exclusiveSourceWriteGuard?.(place.slot,[],start);for(let index=0;index<star+2;index++)c.next();
 const value=withRustStandardSumSourceContext(parser,place.field,()=>readRustExpectedCopyClosure(parser,path,place.field));return write(parser,place,value,path,{start:start.start,end:value.span?.end??c.peek(-1).end});
}
export function readRustExclusiveSourceDereference(parser,path,token){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1')return null;
 const c=parser.cursor,name=c.peek(),origin=name.kind==='identifier'?parser.origins.scope.get(name.value):null;
 if(origin?.sourceExclusiveMode!=='exclusive'||c.is('.',1))return null;c.next();return readRustOwnedRecordMutableName(parser,{k:'name',path:[name.value],span:{start:token.start,end:name.end}},path);
}
export function readRustExclusiveSourceNode(parser,node){
 if(node?.k!=='math'||!parser.exclusiveSourceCalls)return null;const index=Number(node.name);if(!Number.isSafeInteger(index)||index<0||String(index)!==node.name)return null;
 if(node.op===operations.call){const info=parser.exclusiveSourceCalls.calls[index];return info&&node.args.length===info.parameters.length?{kind:'call',info}:null;}
 if(node.op===operations.write){const info=parser.exclusiveSourceCalls.writes[index];return info&&node.args.length===1?{kind:'write',info}:null;}
 return null;
}
export function testRustExclusiveSourcePrivateNode(node){return node?.k==='math'&&Object.values(operations).includes(node.op);}
// A supplied complete projected library may use the same actual packet effect
// after independently validating the selected helper declaration and signature.
export function recordRustExclusiveSourcePacketEffect(parser,callee,args,parameters,result,borrows,span){
 const arena=initializeRustExclusiveSourceCalls(parser),entry=parser.origins.lookupValueOwner(callee,[]);
 if(!arena||entry?.k!=='fn'||parameters.length!==args.length||borrows.some(row=>!readRustOwnedRecordStateSlots(parser).some(slot=>slot.lexical===row.slot.lexical&&slot.root===row.slot.root)))fail('a library state packet needs its actual selected callable and live source root slots',span);
 const index=arena.calls.length;if(index>=100000)fail('library state packets exceed their finite source construction budget',span);arena.calls.push({callee:[...callee],parameters,result,borrows,span});return parser.origins.mark({k:'math',op:operations.call,name:String(index),args:args.map(value=>({value,spread:false})),span},result);
}
