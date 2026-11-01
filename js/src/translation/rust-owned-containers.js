// Actual structured source owners, allocation nodes and value loans only.
// The source-generated rules own every admission/trait/transfer decision.
import {unsupported} from './diagnostics.js';
import {readRustNominalOwnershipFacts} from './rust-nominal-ownership.js';
import {readRustCopyClosureValueBorrows} from './rust-copy-closure-lifetimes.js';
import {mergeRustOwnedLoanBindings,projectRustOwnedLoanBindings} from './rust-owned-binding-rules.js';
import {checkRustOwnedOrigin,checkRustOwnedPlaceMove,consumeRustOwnedOrigin,trackRustOwnedBindings} from './rust-owned-bindings.js';
import {readRustStandardSumOwnedFieldProductionError} from './rust-standard-sum-ownership-rules.js';
import {measureRustStandardSumSourceMetadata} from './rust-standard-sum-source-metadata.js';
import {readRustStandardSumSourceMetadataReservation,readRustStandardSumSourceMetadataCountError} from './rust-standard-sum-source-metadata-rules.js';
import {readRustOwnedBoxTypeError,acceptRustOwnedBoxCallResult} from './rust-owned-recursive-datatype-rules.js';
import {acceptRustOwnedBoxTypeNode,readRustOwnedContainerTypeNode,readRustOwnedContainerSumNode,readRustOwnedContainerNodeRole,acceptRustOwnedContainerClone,readRustOwnedContainerLeafRole,readRustOwnedContainerChildModel,readRustOwnedBoxCanonicalOwner,readRustOwnedBoxConstructorError,readRustOwnedBoxMoveError,acceptRustOwnedBoxIntrinsicCopyValue,acceptRustOwnedBoxValueLayers,acceptRustOwnedNominalVectorElement,readRustOwnedContainerRuntimeMetadataCost,readRustOwnedBoxCloneError,acceptRustOwnedBoxLayerClone,acceptRustOwnedContainerValueLoanRole,readRustOwnedBoxPayloadError,readRustOwnedContainerDepthError,acceptRustOwnedContainerClosed} from './rust-owned-recursive-datatype-rules.js';
const empty=()=>({profile:'',cloneImplemented:false,closed:false,node:'unknown',owner:'',inner:null});
const sourceOwner=origin=>origin?.expandedSourceOwner??origin?.sourceOwner;
const copy=value=>structuredClone(value);
const fail=(message,span)=>{throw unsupported('Rust owned container',message,span);};

export function readRustOwnedContainerFacts(table,origin){
 let visits=0;const active=new Set();
 function walk(value,wrappers,owner,depth){
  if(!table.ownedContainerSourceEnabled||!table.nominalOwnershipScopeComplete||!value?.proven||!value.type||!owner||++visits>65536||depth>256)return empty();
  const type=value.type,nominal=type.kind==='named'?readRustNominalOwnershipFacts(table,{...value,wrappers:[],sourceOwner:owner,expandedSourceOwner:undefined}):empty();
  const model=value.arraySourceModel??value.textSourceModel??readRustOwnedContainerChildModel(type.kind,owner.kind,owner.path.join('::'),owner.arguments.length),leaf=readRustOwnedContainerLeafRole(type.kind,String(type.bits??''),type.signed===true,value.proven,owner.kind,owner.path.join('::'),owner.arguments.length,model);
  const tupleChildren=type.kind==='tuple'&&value.fields?.length===type.elements.length&&owner.kind==='tuple'&&owner.arguments.length===value.fields.length?value.fields.map((child,index)=>walk(child,child.wrappers,owner.arguments[index],depth+1)):null;
  let node=readRustOwnedContainerTypeNode(type.kind,wrappers,value.proven,owner.kind,owner.path.join('::'),owner.arguments.length,model,leaf==='copy'?'copy':leaf==='owned'?'string':tupleChildren?'tuple':'unknown',leaf==='copy'||Boolean(tupleChildren?.every(child=>child.profile==='copy'))||table.immutableProduct(value),nominal.profile,table.nominalOwnershipScopeComplete);
  if(node==='box'){const inner=walk(value,wrappers.slice(1),owner.arguments[0],depth+1),profile=readRustOwnedContainerNodeRole(node,[inner.profile],'');return {profile,cloneImplemented:profile!==''&&acceptRustOwnedContainerClone(node,[inner.cloneImplemented],false),closed:profile!==''&&inner.closed,node,owner:owner.path.join('::'),inner};}
  if(node==='reference')return {profile:readRustOwnedContainerNodeRole(node,[],''),cloneImplemented:acceptRustOwnedContainerClone(node,[],false),closed:acceptRustOwnedContainerClosed(node,[],false),node,owner:owner.path.join('::'),inner:null};
  const declared=type.kind==='named'?table.lookupTypeOwner(type.path,value.path):null,stored=declared?.standardSumSourceOrigin,sum=stored?.standardSumSource;
  if(sum&&wrappers.length===0){node=readRustOwnedContainerSumNode(sum.owner,sum.arguments.length,stored.proven===true,sum.provisional===true,wrappers.length);if(node==='unknown'||active.has(declared.fullName))return empty();active.add(declared.fullName);const children=sum.arguments.map(child=>walk(child,child.wrappers,sourceOwner(child),depth+1));active.delete(declared.fullName);const profile=readRustOwnedContainerNodeRole(node,children.map(child=>child.profile),'');return {profile,cloneImplemented:profile!==''&&acceptRustOwnedContainerClone(node,children.map(child=>child.cloneImplemented),false),closed:profile!==''&&children.every(child=>child.closed),node,owner:sum.owner,inner:null};}
  if(node==='named')return {profile:nominal.profile,cloneImplemented:nominal.cloneImplemented,closed:nominal.closedOwnedContainer===true,node,owner:nominal.owner,inner:null};
  if(node==='vector'){const element={type:type.element,path:value.path,wrappers:[],proven:value.proven,sourceOwner:owner.arguments[0]},inner=walk(element,[],owner.arguments[0],depth+1);if(!inner.closed)return empty();const profile=readRustOwnedContainerNodeRole(node,[inner.profile],'');return {profile,cloneImplemented:profile!==''&&acceptRustOwnedContainerClone(node,[inner.cloneImplemented],false),closed:profile!=='',node,owner:owner.path.join('::'),inner};}
  if(node==='tuple'){if(!tupleChildren)return empty();const profile=readRustOwnedContainerNodeRole(node,tupleChildren.map(child=>child.profile),'');return {profile,cloneImplemented:profile!==''&&tupleChildren.every(child=>child.cloneImplemented),closed:profile!==''&&tupleChildren.every(child=>child.closed),node,owner:'',inner:null};}
  const profile=readRustOwnedContainerNodeRole(node,[],'');return {profile,cloneImplemented:profile!==''&&acceptRustOwnedContainerClone(node,[],false),closed:profile!=='',node,owner:owner.path.join('::'),inner:null};
 }
 return walk(origin,origin?.wrappers??[],sourceOwner(origin),0);
}
export function isRustOwnedBoxValue(table,origin){if(table.ownedContainerSourceEnabled!==true)return false;const facts=readRustOwnedContainerFacts(table,origin);return facts.node==='box'&&facts.profile==='owned'||acceptRustOwnedBoxValueLayers(origin?.wrappers??[],(origin?.sourceBoxLayers??[]).map(layer=>layer.owner));}
export function acceptRustOwnedNominalVectorElementValue(table,origin){const facts=readRustOwnedContainerFacts(table,origin);return origin?.type?.kind==='named'&&acceptRustOwnedNominalVectorElement(facts.profile,facts.closed);}
export function checkRustOwnedBoxSourceType(parser,origin,span){const table=parser.origins;if(!table.ownedContainerSourceEnabled)return;const depth=readRustOwnedContainerDepthError(origin.wrappers.length,0);if(depth)fail(depth,span);const facts=readRustOwnedContainerFacts(table,origin),error=readRustOwnedBoxTypeError(origin.wrappers[0]==='box',parser.headerDiscovery===true,table.nominalOwnershipScopeComplete===true,facts.profile,facts.closed);if(error)fail(error,span);}
export function checkRustOwnedContainerSourceDepth(parser,depth,span){if(!parser.origins.ownedContainerSourceEnabled)return;const error=readRustOwnedContainerDepthError(depth,0);if(error)fail(error,span);}
export function markRustOwnedBoxCallResult(table,node,path,origin){
 const facts=readRustOwnedContainerFacts(table,origin),names=node.k==='app'&&node.fn.k==='name'?node.fn.path:node.k==='name'?node.path:[],entry=table.lookupValueOwner(names,path),result=entry?.k==='fn'?entry.result:null,actual=result?readRustOwnedContainerFacts(table,result):empty();
 if(!acceptRustOwnedBoxCallResult(facts.node==='box'&&facts.profile==='owned'&&facts.closed,actual.node==='box'&&actual.profile==='owned'&&actual.closed,Boolean(result)&&JSON.stringify(sourceOwner(origin))===JSON.stringify(sourceOwner(result)),Boolean(result)&&table.same(origin,result),Boolean(node.span&&table.freshCalls.has(`${node.span.start}:${node.span.end}`))))return origin;
 return {...detached(origin),sourceOwnedResultProof:true};
}

export function reserveRustOwnedContainerCopies(table,values){return reserve(table,values);}
function reserve(table,values){
 const cost=measureRustStandardSumSourceMetadata(values),active=new Set();let borrowed=false;
 const event=(role,units,depth)=>{const add=readRustOwnedContainerRuntimeMetadataCost(role,units);if(add.length!==2)fail('the Box value metadata has an unknown projection role');cost[0]+=add[0];cost[1]+=add[1];const error=readRustStandardSumSourceMetadataCountError(cost[0],cost[1],depth);if(error)fail(error);};
 const strings=(role,items,depth)=>{event(role,0,depth);if(acceptRustOwnedContainerValueLoanRole(role)&&items?.length)borrowed=true;for(const value of items??[])event('binding',value.length,depth+1);};
 const fields=(role,items,depth)=>{event(role,0,depth);for(const values of items??[]){if(acceptRustOwnedContainerValueLoanRole(role)&&values.length)borrowed=true;strings('list',values,depth+1);}};
 function walk(value,depth){if(!value)return;if(active.has(value))fail('cyclic Box source value metadata needs a finite projection');event('origin',0,depth);active.add(value);
  strings('value loans',value.sourceValueBorrowBindings,depth+1);fields('value field loans',value.sourceValueFieldBorrowBindings,depth+1);strings('loans',value.sourceLoanBindingIdentities,depth+1);strings('place loans',value.sourcePlaceLoanBindingIdentities,depth+1);strings('temporary loans',value.sourceExtendingTemporaryBindings,depth+1);
  event('list',0,depth+1);for(const lifetime of value.sourceLifetimes??[])event('binding',lifetime?.length??0,depth+2);
  event('list',0,depth+1);for(const layer of value.sourceBoxLayers??[]){event('layer',0,depth+2);event('owner',layer.owner.length,depth+3);event('intrinsic copy',1,depth+3);strings('payload loans',layer.payloadBorrowBindings,depth+3);fields('payload field loans',layer.payloadFieldBorrowBindings,depth+3);}
  for(const child of [...(value.fields??[]),...(value.callableParameters??[]),...(value.standardSumSource?.arguments??[])])walk(child,depth+1);if(value.callableResult)walk(value.callableResult,depth+1);active.delete(value);
 }
 for(const value of values)walk(value,0);
 const next=readRustStandardSumSourceMetadataReservation(table.ownedContainerMaterializedNodes??0,table.ownedContainerMaterializedUnits??0,cost[0],cost[1]);if(next.length!==2)fail('the complete Box value metadata exceeds its materialization budget');table.ownedContainerMaterializedNodes=next[0];table.ownedContainerMaterializedUnits=next[1];
 return borrowed;
}
function payloadBorrows(value){let result=readRustCopyClosureValueBorrows(value);if(value.wrappers.some(layer=>layer==='reference'))result=mergeRustOwnedLoanBindings(result,mergeRustOwnedLoanBindings(projectRustOwnedLoanBindings(value.sourceBindingIdentity??'',value.sourceLoanBindingIdentities??[]),projectRustOwnedLoanBindings(value.sourcePlaceBindingIdentity??'',value.sourcePlaceLoanBindingIdentities??[])));return result;}
function detached(origin){return {...origin,sourceLexicalBindingIdentity:null,sourceRecordStateCollectingSite:null,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceBorrowedPlace:false,sourceStandardSumPatternMove:false,sourceStandardSumBorrowedField:false,sourceNominalPatternMove:false,sourceNominalBorrowedField:false};}
export function createRustOwnedBoxConstructor(parser,argument,ownerPath,path,span){
 const table=parser.origins;if(!table.ownedContainerSourceEnabled)return null;const owner=readRustOwnedBoxCanonicalOwner(ownerPath.join('::')),actual=table.origin(argument,path),facts=readRustOwnedContainerFacts(table,actual),intrinsic=acceptRustOwnedBoxIntrinsicCopyValue(argument.k,table.immutableProduct(actual)),role=facts.profile||(isRustOwnedBoxValue(table,actual)?'owned':intrinsic?'copy':'');
 const depthError=readRustOwnedContainerDepthError(actual.wrappers.length+1,(actual.sourceBoxLayers?.length??0)+1);if(depthError)fail(depthError,span);
 const error=readRustOwnedBoxConstructorError(owner,role,1);if(error)fail(error,span);checkRustOwnedOrigin(table,actual,argument.span);checkRustOwnedPlaceMove(table,actual,argument.span);
 const transferred=table.ownedArgumentTransfers.has(argument),producer=actual.sourceOwnedResultProof===true||actual.freshArrayValue===true||table.freshVectorConstructors.has(`${argument.span?.start}:${argument.span?.end}`);
 if(!table.collecting){const error=readRustStandardSumOwnedFieldProductionError(role==='owned',transferred,table.availableOwnedBinding(actual),producer);if(error)fail(error,argument.span);}
 const hasBorrows=reserve(table,[actual]),payloadError=readRustOwnedBoxPayloadError(facts.closed,intrinsic,hasBorrows,isRustOwnedBoxValue(table,actual));if(payloadError)fail(payloadError,argument.span);
 const borrowed=facts.closed?[]:payloadBorrows(actual),layer={owner,intrinsicCopy:role==='copy',payloadBorrowBindings:borrowed,payloadFieldBorrowBindings:facts.closed?null:copy(actual.sourceValueFieldBorrowBindings??null)};
 if(role==='owned'&&!transferred)consumeRustOwnedOrigin(table,actual,argument.span);
 const result=detached(copy(actual));result.wrappers.unshift('box');result.copyEvidence=false;result.sourceOwnedResultProof=true;result.sourceValueBorrowBindings=borrowed;if(facts.closed)result.sourceValueFieldBorrowBindings=null;result.sourceBoxLayers=[layer,...copy(actual.sourceBoxLayers??[])];
 const childOwner=sourceOwner(actual);result.sourceOwner=childOwner?{kind:'library',path:owner.split('::'),arguments:[copy(childOwner)]}:null;delete result.expandedSourceOwner;
 return table.mark({...argument,span},result);
}
function peel(table,origin){const result=copy(origin),owner=sourceOwner(origin);result.wrappers.shift();if(acceptRustOwnedBoxTypeNode(origin.wrappers,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1)){result.sourceOwner=copy(owner.arguments[0]);delete result.expandedSourceOwner;}const layer=origin.sourceBoxLayers?.[0];result.sourceBoxLayers=copy((origin.sourceBoxLayers??[]).slice(1));if(layer){result.sourceValueBorrowBindings=copy(layer.payloadBorrowBindings);result.sourceValueFieldBorrowBindings=copy(layer.payloadFieldBorrowBindings);}result.copyEvidence=result.wrappers.length===0&&(result.type?table.immutableProduct(table.typeOrigin(result.type,result.path)):layer?.intrinsicCopy===true);return result;}
export function finishRustOwnedBoxDereference(table,node,path,span){
 const actual=table.origin(node,path);if(!table.ownedContainerSourceEnabled||actual.wrappers[0]!=='box')return null;const facts=readRustOwnedContainerFacts(table,actual),layer=actual.sourceBoxLayers?.[0];if(facts.node!=='box'&&!isRustOwnedBoxValue(table,actual))return null;
 checkRustOwnedOrigin(table,actual,span);reserve(table,[actual]);const child=peel(table,actual),copied=facts.inner?.profile==='copy'||layer?.intrinsicCopy===true;
 if(!copied){checkRustOwnedPlaceMove(table,actual,span);if(!table.collecting){const error=readRustOwnedBoxMoveError(trackRustOwnedBindings(table),table.availableOwnedBinding(actual),actual.sourceOwnedResultProof===true);if(error)fail(error,span);}consumeRustOwnedOrigin(table,actual,span);const value=detached(child);value.sourceOwnedResultProof=true;return table.mark(node,value);}
 const value=detached(child);value.sourceOwnedResultProof=false;value.sourcePlaceBindingIdentity=actual.sourceBindingIdentity??actual.sourcePlaceBindingIdentity??null;value.sourcePlaceLoanBindingIdentities=mergeRustOwnedLoanBindings(projectRustOwnedLoanBindings(actual.sourceBindingIdentity??'',actual.sourceLoanBindingIdentities??[]),projectRustOwnedLoanBindings(actual.sourcePlaceBindingIdentity??'',actual.sourcePlaceLoanBindingIdentities??[]));return table.mark(node,value);
}
export function readRustOwnedBoxRecordParent(table,origin){
 if(!table.ownedContainerSourceEnabled)return origin;const owner=sourceOwner(origin);if(!origin.wrappers.includes('box')||!owner)return origin;
 reserve(table,[origin]);let result=copy(origin),borrowed=result.sourceBorrowedPlace===true,references=[];
 for(let at=0;at<256&&result.wrappers.length;at+=1){if(result.wrappers[0]==='reference'){borrowed=true;references.push('reference');result.wrappers.shift();continue;}const owner=sourceOwner(result);if(!acceptRustOwnedBoxTypeNode(result.wrappers,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1))break;result=peel(table,result);}
 result.wrappers=[...references,...result.wrappers];result.sourceBorrowedPlace=borrowed;return result;
}
export function readRustOwnedBoxRecordOwner(table,origin){
 if(!table.ownedContainerSourceEnabled||origin?.type?.kind!=='named')return null;let source=sourceOwner(origin),boxed=false,borrowed=origin.sourceBorrowedPlace===true;
 for(const layer of origin.wrappers){if(layer==='reference'){borrowed=true;continue;}if(!acceptRustOwnedBoxTypeNode([layer],source?.kind??'',source?.path.join('::')??'',source?.arguments.length??-1))return null;source=source.arguments[0];boxed=true;}
 if(!boxed)return null;const facts=readRustNominalOwnershipFacts(table,{...origin,wrappers:[],sourceOwner:source,expandedSourceOwner:undefined});return facts.profile?{owner:facts.owner,copy:facts.profile==='copy',borrowed}:null;
}
export function readRustOwnedBoxClone(parser,receiver,method,path){
 const table=parser.origins;if(!table.ownedContainerSourceEnabled||method.value!=='clone')return null;const actual=table.origin(receiver,path);let wrappers=[...actual.wrappers];while(wrappers[0]==='reference')wrappers.shift();const direct={...actual,wrappers},facts=readRustOwnedContainerFacts(table,direct),layers=actual.sourceBoxLayers??[],generic=acceptRustOwnedBoxValueLayers(wrappers,layers.map(layer=>layer.owner));if(facts.node!=='box'&&!generic)return null;
 checkRustOwnedOrigin(table,actual,receiver.span);const readonly=actual.wrappers.length!==wrappers.length||actual.sourceBorrowedPlace===true,clone=facts.cloneImplemented||acceptRustOwnedBoxLayerClone(wrappers,layers.map(layer=>layer.owner),layers.map(layer=>layer.intrinsicCopy)),closed=facts.closed||generic,error=readRustOwnedBoxCloneError(true,clone,readonly,closed,table.availableOwnedBinding(direct),actual.sourceOwnedResultProof===true);if(error)fail(error,receiver.span);
 const c=parser.cursor;c.expect('(','Box Clone');const close=c.expect(')','Box Clone'),span={start:receiver.span?.start??method.start,end:close.end},name=parser.freshProductBinding();reserve(table,[actual]);const local=table.mark({k:'name',path:[name],span},actual),node={k:'let',name,type:actual.type,value:receiver,body:local,span},result=detached(copy(direct));result.copyEvidence=false;result.sourceOwnedResultProof=true;
 if(layers.length){result.sourceValueBorrowBindings=copy(layers[0].payloadBorrowBindings);result.sourceValueFieldBorrowBindings=copy(layers[0].payloadFieldBorrowBindings);}else if(readonly&&closed){result.sourceValueBorrowBindings=[];result.sourceValueFieldBorrowBindings=null;}
 return table.mark(node,result);
}
