// Pure-core SSA snapshots of actual mutable record places. No runtime cell,
// source Copy upgrade or public uninitialized-value constructor is introduced.
import {unsupported,typeError} from './diagnostics.js';
import {readRustOwnedRecordMutableBindingError,readRustOwnedRecordStateWriteError,testRustOwnedRecordStateLoanOverlap,planRustOwnedRecordStatePacket,planRustOwnedRecordStateFields,readRustOwnedRecordStateFieldIdentity,readRustOwnedRecordStateAssignmentTokenCount,readRustOwnedRecordStateStorageDisposition,readRustOwnedRecordStateStorageError,acceptRustOwnedRecordStateStorageOwner} from './rust-owned-record-state-rules.js';
import {readRustOwnedRecordUpdateVersionError} from './rust-owned-record-update-rules.js';
import {rustOwnedRecordFieldState} from './rust-owned-record-updates.js';
import {readRustNominalOwnershipFacts} from './rust-nominal-ownership.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {readRustStandardSumSourceIdentity,withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {captureRustOwnedStandardSumMethodValue} from './rust-owned-standard-sum-methods.js';
import {readRustCopyClosureValueBorrows} from './rust-copy-closure-lifetimes.js';
import {readRustRecordSourceType} from './rust-records.js';
import {readRustBorrowedVectorMutationError} from './rust-borrowed-vector-rules.js';
import {registerRustMutableRecordLoanRoot,testRustMutableRecordLiveLoan} from './rust-mutable-record-loans.js';
import {acceptRustMutableRecordValueLoans} from './rust-mutable-record-loan-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust owned record state',reason,span);};
const copy=value=>structuredClone(value);
function state(parser){return parser.ownedRecordState??= {slots:new Map(),frames:[],declarations:new Map(),nextFrame:0,nextCallable:1,currentCallable:0,request:null};}
export function installRustOwnedRecordEffectTransport(parser,transport){state(parser).effectTransport=transport;}
export function readRustOwnedRecordStateDeclarations(parser){return state(parser).declarations??=new Map();}
function clonedSlots(slots){return new Map([...slots].map(([key,value])=>[key,{...value}]));}
function activeSlots(parser){const current=state(parser);return [...current.slots.values()].filter(slot=>slot.callable===current.currentCallable).sort((left,right)=>left.sequence-right.sequence);}
export function readRustOwnedRecordStateSlots(parser){return activeSlots(parser);}
export function readRustOwnedRecordStateCallable(parser){return state(parser).currentCallable;}
export function readRustOwnedRecordStateSlot(parser,lexical){return state(parser).slots.get(lexical);}
export function requireRustOwnedRecordStateStorage(parser,origin,span){return requireStorage(parser,origin,span);}
// Called only after the separate exclusive source header/binder has selected
// its actual referent. The source mode is retained independently of this SSA
// carrier; no immutable reference receives mutation authority by resemblance.
export function registerRustExclusiveSourceStateRoot(parser,name,mode,span){
 const current=state(parser),table=parser.origins,actual=table.scope.get(name),frame=current.frames.at(-1);
 if(table.collecting&&actual&&frame&&!actual.sourceLexicalBindingIdentity)actual.sourceLexicalBindingIdentity='exclusive collecting lexical '+String(current.nextFrame++);
 const collection=mode==='owned'&&parser.standardCollectionSourceModel==='Rust closed standard collections 1'&&parser.standardCollectionSourceStorage?.(actual)===true;
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1'&&!collection||!actual||!frame||!actual.sourceLexicalBindingIdentity||!['owned','exclusive'].includes(mode)||mode==='exclusive'&&actual.sourceExclusiveMode!=='exclusive'||mode==='owned'&&actual.wrappers.length!==0)fail('exclusive state roots need their actual admitted source mode and lexical binder',span);
 const origin={...copy(actual),wrappers:[],sourceLifetimes:[],sourceBorrowedPlace:mode==='exclusive',sourceExclusiveParameter:mode==='exclusive'};
 delete origin.sourceExclusiveMode;
 if(!table.collecting)requireStorage(parser,origin,span);
 const root=actual.sourceBindingIdentity??actual.sourceLexicalBindingIdentity;table.sourceBindingStates.set(root,'available');origin.sourceBindingIdentity=root;
 const binding=parser.freshProductBinding(),slot={name,lexical:actual.sourceLexicalBindingIdentity,root,origin,currentName:binding,frame:frame.id,callable:current.currentCallable,sequence:current.nextFrame++,exclusiveMode:mode};
 current.slots.set(slot.lexical,slot);readRustOwnedRecordStateDeclarations(parser).set(binding,slot);return slot;
}
export function readRustOwnedRecordStatePlace(parser,name,fields,span){
 const origin=parser.origins.scope.get(name),slot=sourceSlot(parser,origin,name);if(!slot)return null;
 const layouts=layoutPath(parser,slot.origin,fields,span);return {slot,layouts,field:layouts.at(-1)?.field??slot.origin,fields:layouts.map(row=>row.index)};
}
export function projectRustOwnedRecordStatePlace(parser,layouts,object,span){
 let value=object;for(const {layout,entry,index}of layouts){const name=parser.freshProductBinding(),constructor=['crate',...layout.constructor.split('.')];value={k:'match',scrutinees:[value],rows:[{patterns:[{k:'ctor',path:constructor,args:entry.fields.map((_,at)=>at===index?{k:'bind',name,span}:{k:'wild',span}),span}],body:{k:'name',path:[name],span},span}],span};}return value;
}
export function testRustOwnedRecordStatePlaceLoans(parser,slot,fields){return overlaps(parser,slot,readRustOwnedRecordStateFieldIdentity(slot.root,fields));}
function sourceSlot(parser,origin,name){const current=state(parser);if(origin?.sourceLexicalBindingIdentity)return current.slots.get(origin.sourceLexicalBindingIdentity);const site=origin?.sourceRecordStateCollectingSite;return parser.origins.collecting&&site?[...current.slots.values()].find(slot=>slot.site?.start===site.start&&slot.site?.end===site.end):null;}
function record(parser,origin){const owner=origin.type?.kind==='named'?parser.origins.lookupTypeOwner(origin.type.path,origin.path):null,layout=owner?parser.namedRecords.get(owner.fullName):null,entry=layout?parser.origins.lookupValueOwner(['crate',...layout.constructor.split('.')],[]):null;return layout&&entry?.k==='ctor'&&entry.owner===owner.fullName?{layout,entry}:null;}
export function readRustOwnedRecordStateLayout(parser,origin){return record(parser,origin);}
function requireStorage(parser,root,span){
 const table=parser.origins,pending=[{origin:root,depth:0,active:[]}];let nodes=0;
 while(pending.length){const {origin,depth,active}=pending.pop(),error=readRustOwnedRecordStateStorageError(++nodes,depth);if(error)fail(error,span);
  if(parser.standardCollectionSourceStorage?.(origin)===true)continue;
  const sourceOwner=origin.expandedSourceOwner??origin.sourceOwner,role=readRustStandardSumOwnershipFacts(table,origin).profile,disposition=readRustOwnedRecordStateStorageDisposition(origin.type?.kind??'',origin.wrappers.length,role);if(disposition==='refuse'||!acceptRustOwnedRecordStateStorageOwner(sourceOwner?.kind??'',sourceOwner?.path.join('::')??''))fail('mutable record state requires actual direct storage without callable, reference or Box fields',span);if(disposition==='leaf')continue;
  if(origin.type.kind==='tuple'){if(!origin.fields)fail('tuple storage needs every actual complete child origin',span);for(const child of origin.fields)pending.push({origin:child,depth:depth+1,active});continue;}
  if(origin.type.kind==='array'){const owner=origin.expandedSourceOwner??origin.sourceOwner;if(owner?.kind!=='library'||owner.arguments.length!==1)fail('vector state needs its actual complete source element owner',span);const position=origin.arrayElementPosition;if(position?.wrappers.length)fail('mutable record state requires actual direct storage without callable, reference or Box fields',span);const child={...table.typeOrigin(origin.type.element,origin.path),sourceOwner:copy(position?.owner??owner.arguments[0]),arrayElementPosition:position?.arrayElementPosition?copy(position.arrayElementPosition):null};pending.push({origin:child,depth:depth+1,active});continue;}
  const declared=table.lookupTypeOwner(origin.type.path,origin.path);if(!declared||active.includes(declared.fullName))fail('recursive mutable record storage needs its separate finite state interpretation',span);const next=[...active,declared.fullName],sum=declared.standardSumSourceOrigin?.standardSumSource;
  if(sum){if(declared.standardSumSourceOrigin.proven!==true||sum.provisional)fail('sum state storage needs actual final source admission',span);for(const child of sum.arguments)pending.push({origin:child,depth:depth+1,active:next});continue;}
  const row=table.nominalOwnershipDeclarations.get(declared.fullName);if(!row?.profile)fail('nominal state storage needs its actual destructor-free declaration inventory',span);
  for(const name of row.constructors){const entry=table.lookupValueOwner(['crate',...name.split('.')],[]);if(entry?.k!=='ctor'||entry.owner!==declared.fullName)fail('state fields need their actual complete constructor inventory',span);for(const child of entry.fields)pending.push({origin:child,depth:depth+1,active:next});}
 }
}
export function beginRustOwnedRecordStateBlock(parser,functionBody=false){
 const current=state(parser),frame={id:current.nextFrame++,before:clonedSlots(current.slots),callableBefore:current.currentCallable,request:current.request};current.request=null;if(functionBody)current.currentCallable=current.nextCallable++;current.frames.push(frame);return frame;
}
export function endRustOwnedRecordStateBlock(parser,frame){
 const current=state(parser);if(current.frames.at(-1)!==frame)throw new TypeError('record state scope frames must leave in source order');
 if(frame.request)frame.request.output=frame.request.slots.map(slot=>({...current.slots.get(slot.lexical)}));
 current.frames.pop();current.slots=frame.before;current.currentCallable=frame.callableBefore;
}
export function beginRustOwnedRecordStateCallable(parser){const current=state(parser),before=current.currentCallable;current.currentCallable=current.nextCallable++;return before;}
export function endRustOwnedRecordStateCallable(parser,before){state(parser).currentCallable=before;}
export function registerRustOwnedRecordMutableBinding(parser,name,span){
 if(parser.exclusiveSourceModel==='Rust exclusive source loans 1')return registerRustExclusiveSourceStateRoot(parser,name,'owned',span).currentName;
 if(parser.standardCollectionSourceModel==='Rust closed standard collections 1'&&parser.standardCollectionSourceStorage?.(parser.origins.scope.get(name))===true)return registerRustExclusiveSourceStateRoot(parser,name,'owned',span).currentName;
 const current=state(parser),table=parser.origins,origin=table.scope.get(name),profile=origin?readRustNominalOwnershipFacts(table,origin).profile:'';
 if(table.collecting){
  const frame=current.frames.at(-1);if(!origin||!record(parser,origin)||origin.wrappers.length)throw unsupported('mutable binding','mutation is outside the portable core',span);if(!frame)fail('the private mutable header needs its actual complete record layout',span);
  table.standardSumSourceNeedsFinalAdmission=true;const lexical='private collecting record slot '+String(current.nextFrame++),binding=parser.freshProductBinding();
  origin.sourceRecordStateCollectingSite={start:span.start,end:span.end};const slot={name,lexical,root:'',site:origin.sourceRecordStateCollectingSite,origin:copy(origin),currentName:binding,frame:frame.id,callable:current.currentCallable,sequence:current.nextFrame++,provisional:true};current.slots.set(lexical,slot);readRustOwnedRecordStateDeclarations(parser).set(binding,slot);return binding;
 }
 if(profile!=='owned')throw unsupported('mutable binding','mutation is outside the portable core',span);
 const error=readRustOwnedRecordMutableBindingError(parser.ordinaryCompletionContext(),true,profile,Boolean(origin?.sourceBindingIdentity),Boolean(origin?.sourceLexicalBindingIdentity),origin?.wrappers.length??-1);if(error)fail(error,span);
 const frame=current.frames.at(-1);if(!frame)fail('a mutable record needs its actual source block state frame',span);const version=readRustOwnedRecordUpdateVersionError(parser.sourceVersionMinor);if(version)fail(version,span);requireStorage(parser,origin,span);if(!readRustStandardSumSourceIdentity(parser,origin))fail('mutable storage needs its actual complete source identity',span);
 registerRustMutableRecordLoanRoot(table,origin,span);
 const binding=parser.freshProductBinding(),slot={name,lexical:origin.sourceLexicalBindingIdentity,root:origin.sourceBindingIdentity,origin:copy(origin),currentName:binding,frame:frame.id,callable:current.currentCallable,sequence:current.nextFrame++};current.slots.set(origin.sourceLexicalBindingIdentity,slot);readRustOwnedRecordStateDeclarations(parser).set(binding,slot);return binding;
}
export function readRustOwnedRecordMutableName(parser,node,path){
 if(node.k!=='name'||node.path.length!==1)return node;const origin=parser.origins.scope.get(node.path[0]),current=state(parser),slot=sourceSlot(parser,origin,node.path[0]);if(!slot)return origin?parser.origins.mark(node,origin):node;
 if(!parser.origins.collecting&&slot.callable!==current.currentCallable)fail('a captured mutable record needs its separate callable environment state interpretation',node.span);
 parser.exclusiveSourceReadGuard?.(slot,[],node.span);
 return current.effectTransport?current.effectTransport.read(slot,node):parser.origins.mark({...node,path:[slot.currentName]},slot.origin);
}
function packet(parser,node,slots,span){
 const plan=planRustOwnedRecordStatePacket(slots.length);if(plan.length!==slots.length+1)fail('the complete record state result exceeds its carrier budget',span);
 const table=parser.origins,valueOrigin=table.origin(node,parser.modulePath),origins=[valueOrigin,...slots.map(slot=>slot.origin)],items=plan.map(index=>index===0?node:table.mark({k:'name',path:[slots[index-1].currentName]},slots[index-1].origin));
 if(!table.collecting&&origins.some(origin=>!origin.type||!origin.proven))fail('a state result needs every actual complete value and root type',span);
 const result=table.mark({k:'tuple',items,span},{...table.unknownOrigin(parser.modulePath),type:origins.every(origin=>origin.type)?{kind:'tuple',elements:origins.map(origin=>origin.type)}:null,fields:origins,proven:!table.collecting&&origins.every(origin=>origin.proven),copyEvidence:false});return {node:result,origin:valueOrigin};
}
function appendPacket(parser,node,slots,span){
 if(node.k==='let'){const result=appendPacket(parser,node.body,slots,span);return {...result,node:{...node,body:result.node}};}
 if(node.k==='print'){const result=appendPacket(parser,node.body,slots,span);return {...result,node:{...node,body:result.node}};}
 return packet(parser,node,slots,span);
}
function stateBlock(parser,path,slots){
 const current=state(parser),previous=current.request,request={slots,output:null};current.request=request;let node;
 try{node=parser.block(path);}finally{current.request=previous;}
 if(!request.output)fail('record state block hooks must return their actual surviving slot snapshots',node.span);
 const valueOrigin=parser.origins.origin(node,path),wrapped=appendPacket(parser,node,request.output,node.span);return {node:wrapped.node,origin:valueOrigin,slots:request.output};
}
function extract(parser,path,node,origin,slots,span){
 const table=parser.origins,packetName=parser.freshProductBinding(),bindings=[{name:packetName,value:node,span}],reference={k:'name',path:[packetName]};
 const plan=planRustOwnedRecordStatePacket(slots.length);for(let index=1;index<plan.length;index+=1){const slot=slots[index-1],name=parser.freshProductBinding(),value=table.mark({k:'tupleIndex',object:reference,index:plan[index]},slot.origin);bindings.push({name,value,span});state(parser).slots.set(slot.lexical,{...slot,currentName:name});}
 const value=table.mark({k:'tupleIndex',object:reference,index:0,block:true,span},origin);return {bindings,value};
}
function readStateControl(parser,path){
 const current=state(parser),slots=activeSlots(parser),c=parser.cursor;if(!slots.length)return null;
 if(c.is('{')){const read=stateBlock(parser,path,slots);return extract(parser,path,read.node,read.origin,read.slots,read.node.span);}
 if(!c.is('if')||c.is('let',1))return null;
 const start=c.next(),condition=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path,{noStruct:true})),before=parser.origins.snapshotOwnedBindings(),aliases=clonedSlots(current.slots),then=stateBlock(parser,path,slots),thenStates=parser.origins.snapshotOwnedBindings();
 parser.origins.restoreOwnedBindings(before);current.slots=clonedSlots(aliases);
 let otherwise;
 if(c.eat('else')){if(!c.is('{'))fail('record state else-if needs its explicit nested block continuation',c.peek());otherwise=stateBlock(parser,path,slots);}
 else{const unit={k:'unit',span:{start:start.start,end:start.end}},value=packet(parser,unit,slots,unit.span);otherwise={node:value.node,origin:parser.origins.origin(unit,path),slots};}
 const elseStates=parser.origins.snapshotOwnedBindings();parser.origins.joinOwnedBindings(thenStates,elseStates,!then.origin.diverging,!otherwise.origin.diverging);current.slots=aliases;
 const span={start:start.start,end:c.peek(-1).end},origin=parser.origins.checkedJoin?parser.origins.checkedJoin([then.origin,otherwise.origin],span):parser.origins.join([then.origin,otherwise.origin]);
 const body={k:'if',cond:condition,then:then.node,else:otherwise.node,span};return extract(parser,path,body,origin,slots,span);
}
function layoutPath(parser,root,fields,span){const result=[];let origin=root;for(const name of fields){const found=record(parser,origin);if(!found)fail('an assigned place needs every actual nominal record field owner',span);const index=found.layout.names.indexOf(name);if(index<0)throw typeError('a Rust record has no field '+name,span);result.push({...found,index,origin,field:found.entry.fields[index]});origin=found.entry.fields[index];}return result;}
function reconstruct(parser,path,layouts,depth,object,replacement,span){
 if(depth===layouts.length)return replacement;
 const {layout,entry,index}=layouts[depth],plan=planRustOwnedRecordStateFields(entry.fields.length,index);if(plan.length!==entry.fields.length)fail('field reconstruction needs its actual complete declaration-order plan',span);
 const names=entry.fields.map(()=>parser.freshProductBinding()),value=depth+1===layouts.length?replacement:reconstruct(parser,path,layouts,depth+1,{k:'name',path:[names[index]]},replacement,span),args=plan.map(position=>position<0?value:{k:'name',path:[names[position]]});
 const constructor=['crate',...layout.constructor.split('.')];return {k:'match',scrutinees:[object],rows:[{patterns:[{k:'ctor',path:constructor,args:names.map((name,position)=>position===index&&depth+1===layouts.length?{k:'wild',span}:{k:'bind',name,span}),span}],body:args.length?{k:'app',fn:{k:'name',path:constructor,span},args,span}:{k:'name',path:constructor,span},span}],span};
}
function overlaps(parser,slot,target){
 if(testRustMutableRecordLiveLoan(parser.origins,slot.root,target))return true;
 for(const origin of parser.origins.scope.values()){
  const valueLoans=readRustCopyClosureValueBorrows(origin),loans=[origin.sourceBindingIdentity,origin.sourcePlaceBindingIdentity,...(origin.sourceLoanBindingIdentities??[]),...(origin.sourcePlaceLoanBindingIdentities??[]),...valueLoans].filter(Boolean),lexical=valueLoans.includes(slot.lexical);
  if(!acceptRustMutableRecordValueLoans(origin.wrappers?.includes('reference')===true,origin.sourceBorrowedPlace===true,parser.origins.immutableProduct(origin),valueLoans.length))continue;
  if(testRustOwnedRecordStateLoanOverlap(slot.root,target,loans,lexical))return true;
 }return false;
}
export function readRustOwnedRecordStateAssignment(parser,path){
 const c=parser.cursor,tokens=c.tokens.slice(c.index,c.index+1025),forms=tokens.map(token=>['(',')','.','=','}'].includes(token.value)?token.value:token.kind==='identifier'?'name':token.kind==='number'&&/^(0|[1-9][0-9]*)$/u.test(token.raw)?'number':'other'),count=readRustOwnedRecordStateAssignmentTokenCount(forms);if(count<0)return null;
 const used=tokens.slice(0,count),rootToken=used.find(token=>token.value!=='('),source=parser.origins.scope.get(rootToken?.value);if(!source||!record(parser,source))return null;
 const mutation=readRustBorrowedVectorMutationError(parser.origins.borrowedVectorSourceEnabled===true);if(mutation)throw unsupported('Rust borrowed vector source',mutation,{start:used[0].start,end:used.at(-1).end});
 const fields=[];for(let at=0;at<used.length;at+=1)if(used[at].value==='.')fields.push(used[++at].kind==='number'?used[at].raw:used[at].value);
 const span={start:used[0].start,end:used.at(-1).end},layouts=layoutPath(parser,source,fields,span),field=layouts.at(-1).field,current=state(parser),slot=sourceSlot(parser,source,rootToken.value),frame=current.frames.at(-1);
 for(let at=0;at<=count;at+=1)c.next();
 const temporaryFrame=parser.origins.ownedTemporaryFrame(),value=withRustStandardSumSourceContext(parser,field,()=>readRustExpectedCopyClosure(parser,path,field));parser.origins.argument(parser.origins.origin(value,path),field,value.span);
 let actual=parser.origins.origin(value,path);if(!parser.origins.collecting&&readRustStandardSumOwnershipFacts(parser.origins,field).profile==='owned'){actual=captureRustOwnedStandardSumMethodValue(parser,value,path);parser.origins.consumeOwnedBinding(actual,value.span);}
 const identity=parser.origins.collecting?null:readRustStandardSumSourceIdentity(parser,actual),expected=parser.origins.collecting?null:readRustStandardSumSourceIdentity(parser,field),same=Boolean(identity&&identity===expected),rootState=slot?parser.origins.sourceBindingStates.get(slot.root)??'':'',indices=layouts.map(item=>item.index),target=slot?readRustOwnedRecordStateFieldIdentity(slot.root,indices):'';
 let parentState=rootState;for(let index=1;index<indices.length;index+=1){const parent=readRustOwnedRecordStateFieldIdentity(slot?.root??'',indices.slice(0,index));if(rustOwnedRecordFieldState(parser.origins.sourceBindingStates,parent)!=='available')parentState='moved';}
 if(slot)parser.exclusiveSourceWriteGuard?.(slot,indices,span);
 const allowed=Boolean(slot&&frame&&(slot.frame===frame.id||frame.request?.slots.some(outer=>outer.lexical===slot.lexical)||current.effectTransport)),error=parser.origins.collecting?'':readRustOwnedRecordStateWriteError(Boolean(slot),slot?.callable===current.currentCallable,allowed,parentState,target?rustOwnedRecordFieldState(parser.origins.sourceBindingStates,target):'',same,slot?overlaps(parser,slot,target):false);if(error)fail(error,span);if(!slot)fail('field assignment needs its actual source mutable declaration',span);
 if(current.effectTransport){if(!parser.origins.collecting)parser.origins.sourceBindingStates.set(target,'available');const result=current.effectTransport.write({slot,layouts,value,field,target,path,span:{start:span.start,end:value.span?.end??span.end}});parser.origins.finishOwnedExpression(result,path,temporaryFrame,false);return {bindings:[],value:result};}
 const rhs=parser.freshProductBinding(),next=parser.freshProductBinding(),reference={k:'name',path:[rhs]},object={k:'name',path:[slot.currentName]},snapshot=reconstruct(parser,path,layouts,0,object,reference,span),bindings=[{name:rhs,type:readRustRecordSourceType(parser,field.type,field.path),value,span:value.span},{name:next,type:readRustRecordSourceType(parser,slot.origin.type,slot.origin.path),value:snapshot,span}];
 if(!parser.origins.collecting)parser.origins.sourceBindingStates.set(target,'available');current.slots.set(slot.lexical,{...slot,currentName:next});
 const unit={k:'unit',span:{start:span.start,end:value.span?.end??span.end}};parser.origins.finishOwnedExpression(unit,path,temporaryFrame,false);return {bindings,value:unit};
}
export function readRustOwnedRecordStateStatement(parser,path){if(state(parser).effectTransport)return null;return readRustOwnedRecordStateAssignment(parser,path)??readStateControl(parser,path);}
export function reconstructRustOwnedRecordState(parser,path,layouts,object,replacement,span){return reconstruct(parser,path,layouts,0,object,replacement,span);}
