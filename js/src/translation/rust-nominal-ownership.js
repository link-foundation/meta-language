// Actual closed declaration/field transport. Graph decisions live in the
// source-generated policy; value production never comes from a type summary.
import {unsupported} from './diagnostics.js';
import {createRustStandardSumConstructorOrigin} from './rust-standard-sum-ownership.js';
import {projectRustCopyClosureValueField,readRustCopyClosureValueBorrows} from './rust-copy-closure-lifetimes.js';
import {mergeRustOwnedLoanBindings,projectRustOwnedLoanBindings} from './rust-owned-binding-rules.js';
import {consumeRustOwnedOrigin,isRustOwnedBindingValue,checkRustOwnedOrigin,trackRustOwnedBindings} from './rust-owned-bindings.js';
import {planRustNominalPatternTransfers,readRustNominalPatternAlternativeError,readRustNominalCloneError} from './rust-nominal-ownership-rules.js';
import {acceptRustNominalCloneInherentOverride} from './rust-nominal-ownership-rules.js';
import {acceptRustNominalVectorElementLeaf,planRustNominalVectorElementOwners,acceptRustNominalVectorElement as acceptVectorElementFacts} from './rust-nominal-ownership-rules.js';
import {readRustNominalOwnershipVectorTemplateNode,planRustNominalOwnershipVectorNodes,acceptRustNominalVerifiedVectorClone} from './rust-nominal-ownership-rules.js';
import {planRustOwnedRecursiveNominalOwners,acceptRustOwnedBoxTypeNode,planRustOwnedNominalRuntimeFacts} from './rust-owned-recursive-datatype-rules.js';
import {readRustOwnedContainerFacts} from './rust-owned-containers.js';
import {readRustNominalOwnershipGraphError,readRustNominalOwnershipLeaf,readRustNominalOwnershipSumNode,planRustNominalOwnershipOwners,acceptRustNominalOwnershipDeclarationOwner} from './rust-nominal-ownership-rules.js';
const empty=()=>({profile:'',cloneImplemented:false,owner:'',definitionName:'',roles:[]});
function actualDeclarationOwner(origins,origin,source,declared){if(source?.kind==='declaration'&&source.arguments.length===0&&declared?.k==='data'&&origins.lookupTypeOwner(source.path,origin.path)?.fullName===declared.fullName)return {...source,path:declared.fullName.split('.')};return source;}
function graph(origins,rows){
 const owners=[...rows.keys()].sort(),positions=new Map(owners.map((name,index)=>[name,index])),kinds=[],ends=[],edges=[],targets=[],roots=[],elementLeaves=[];let visits=0;
 const node=(kind,children=[],target=-1)=>{const error=readRustNominalOwnershipGraphError(kinds.length+1,edges.length+children.length,0);if(error)throw unsupported('Rust nominal ownership',error);const index=kinds.length;kinds.push(kind);edges.push(...children);ends.push(edges.length);targets.push(target);elementLeaves.push(false);return index;};
 function value(origin,depth){
  const error=readRustNominalOwnershipGraphError(++visits,edges.length,depth);if(error)throw unsupported('Rust nominal ownership',error);
  if(!origin?.type)return node('unknown');const type=origin.type,declared=type.kind==='named'?origins.lookupTypeOwner(type.path,origin.path):null,owner=actualDeclarationOwner(origins,origin,origin.expandedSourceOwner??origin.sourceOwner,declared),stored=declared?.standardSumSourceOrigin,sum=stored?.standardSumSource;
  if(origins.standardCollectionSourceModel==='Rust closed standard collections 1'&&origins.standardCollectionSourceStorage?.(origin)===true)return node('standard collection');
  if(acceptRustOwnedBoxTypeNode(origin.wrappers,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1)){const child={...origin,wrappers:origin.wrappers.slice(1),sourceOwner:owner.arguments[0],expandedSourceOwner:undefined};child.copyEvidence=child.wrappers.length===0&&origins.immutableProduct(origins.typeOrigin(child.type,child.path));return node('box',[value(child,depth+1)]);}
  if(sum&&origin.wrappers.length===0){const kind=readRustNominalOwnershipSumNode(sum.owner,sum.arguments.length,sum.provisional===true,stored.proven===true,origins.nominalOwnershipScopeComplete===true);return node(kind,sum.arguments.map(child=>value(child,depth+1)));}
  const closed=Boolean(owner&&(type.kind!=='named'||declared?.k==='data'&&positions.has(declared.fullName)&&acceptRustNominalOwnershipDeclarationOwner(owner.kind,owner.path.join('::'),owner.arguments.length,declared.fullName.split('.').join('::'))));
  let kind=readRustNominalOwnershipLeaf(type.kind,String(type.bits??''),type.signed===true,origin.wrappers,origin.proven===true,origins.immutableProduct(origin),owner?.kind??'',owner?.path.join('::')??'',owner?.path.length??-1,owner?.arguments.length??-1,origin.arraySourceModel??origin.textSourceModel??'',origin.sourceFunctionPointerHeader===true,type.kind==='function'?type.params.length:-1,origins.nominalOwnershipScopeComplete===true,closed,type.kind==='array'?type.element.kind:'');
  if(kind==='unknown')kind=readRustNominalOwnershipVectorTemplateNode(type.kind,origin.wrappers,origin.proven===true,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,origin.arraySourceModel??'',type.kind==='array'?type.element.kind:'',origins.nominalOwnershipScopeComplete===true);
  if(kind==='named')return node('reference',[],positions.get(declared.fullName)??-1);
  if(kind==='tuple')return node('tuple',(origin.fields??[]).map(child=>value(child,depth+1)));
  if(kind==='vector'||kind==='pending vector'){const element={...origins.typeOrigin(type.element,origin.path),sourceOwner:owner.arguments[0]};return node(kind,[value(element,depth+1)]);}
  const index=node(kind);elementLeaves[index]=acceptRustNominalVectorElementLeaf(type.kind,origin.wrappers);return index;
 }
 for(const name of owners){const row=rows.get(name),constructors=row.constructors.map(name=>origins.lookupValueOwner(['crate',...name.split('.')],[]));roots.push(node('nominal',constructors.map(entry=>node('constructor',(entry?.k==='ctor'?entry.fields:[null]).map(origin=>value(origin,0))))));}
 const elements=planRustNominalVectorElementOwners(owners,owners.map(name=>rows.get(name).copy),owners.map(name=>rows.get(name).derivesClone),kinds,ends,edges,targets,roots,elementLeaves);if(elements.length!==owners.length)throw unsupported('Rust nominal vector element','the actual closed nominal element graph exceeds its finite profile');
 const resolved=planRustNominalOwnershipVectorNodes(kinds,ends,edges,targets,elements);if(resolved.length!==kinds.length)throw unsupported('Rust nominal vector element','the actual vector field edges need their complete finite element witness');
 const plan=planRustNominalOwnershipOwners(owners,owners.map(name=>rows.get(name).copy),owners.map(name=>rows.get(name).derivesClone),resolved,ends,edges,targets,roots);if(plan[0]==='error')throw unsupported('Rust nominal ownership',plan[1]);
 const recursive=planRustOwnedRecursiveNominalOwners(owners,owners.map(name=>rows.get(name).copy),owners.map(name=>rows.get(name).derivesClone),kinds,ends,edges,targets,roots,elementLeaves);if(recursive[0]==='error')throw unsupported('Rust recursive nominal ownership',recursive[1]);
 owners.forEach((name,index)=>{const row=rows.get(name);row.recursiveProfile=recursive[1+5*index];row.recursiveCloneImplemented=recursive[2+5*index]==='Clone';row.vectorElement=elements[index];row.closedOwnedContainer=recursive[3+5*index]==='closed';row.constructorWitnessRank=Number(recursive[4+5*index]);row.constructorWitness=Number(recursive[5+5*index]);const runtime=planRustOwnedNominalRuntimeFacts(origins.ownedContainerSourceEnabled===true,plan[1+2*index],plan[2+2*index]==='Clone',row.recursiveProfile,row.recursiveCloneImplemented,row.closedOwnedContainer,row.constructorWitnessRank,row.constructorWitness);row.profile=runtime[0];row.cloneImplemented=runtime[1]==='Clone';});
}
export function finishRustNominalOwnershipDeclarations(origins){
 if(origins.nominalOwnershipScopeComplete!==true)throw unsupported('Rust nominal ownership','the complete declaration scope must succeed before destructor-free source ownership is selected');
 const rows=new Map();
 for(const [name,derive]of origins.nominalCopyDerivations){
  const entry=origins.lookupTypeOwner(['crate',...name.split('.')],[]);if(entry?.k!=='data'||entry.fullName!==name)continue;
  const constructors=[...origins.valueCatalog].filter(([,entry])=>entry.k==='ctor'&&entry.owner===name).map(([name])=>name).sort();
  rows.set(name,{constructors,copy:origins.nominalCopyOwners.has(name),derivesClone:derive.clone===true,profile:'',cloneImplemented:false,recursiveProfile:'',recursiveCloneImplemented:false,vectorElement:false,closedOwnedContainer:false,constructorWitnessRank:-1,constructorWitness:-1});
 }
 origins.nominalOwnershipDeclarations=rows;graph(origins,rows);
}
export function readRustNominalOwnershipFacts(origins,origin){
 if(origins.nominalOwnershipScopeComplete!==true||!origin?.proven||origin.type?.kind!=='named'||origin.wrappers.length)return empty();
 const owner=origins.lookupTypeOwner(origin.type.path,origin.path),row=owner?.k==='data'?origins.nominalOwnershipDeclarations.get(owner.fullName):null;
 const source=actualDeclarationOwner(origins,origin,origin.expandedSourceOwner??origin.sourceOwner,owner);
 return row&&source&&acceptRustNominalOwnershipDeclarationOwner(source.kind,source.path.join('::'),source.arguments.length,owner.fullName.split('.').join('::'))?{profile:row.profile,cloneImplemented:row.cloneImplemented,closedOwnedContainer:row.closedOwnedContainer,owner:owner.fullName,definitionName:owner.fullName,roles:[]}:empty();
}
export function isRustOwnedNominalValue(origins,origin){return readRustNominalOwnershipFacts(origins,origin).profile==='owned';}
export function acceptRustNominalVectorElement(origins,origin){const facts=readRustNominalOwnershipFacts(origins,origin),row=origins.nominalOwnershipDeclarations.get(facts.owner);return acceptVectorElementFacts(facts.profile,facts.cloneImplemented,row?.vectorElement===true);}
// Authenticate the actual constructor declaration before supplying a complete
// type origin to the existing field-validation/one-transfer production path.
// Unknown profiles keep the prior frontend's explicit interpretation boundary.
export function createRustNominalConstructorOrigin(parser,node,args,path,span=node.span){
 const names=node.k==='app'&&node.fn.k==='name'?node.fn.path:node.k==='name'?node.path:null;
 if(names?.length===1&&parser.origins.scope.has(names[0]))return null;
 const entry=names?parser.origins.lookupValueOwner(names,path):null;if(entry?.k!=='ctor')return null;
 const row=parser.origins.nominalOwnershipDeclarations.get(entry.owner);if(!row?.profile)return null;
 const owner=parser.origins.lookupTypeOwner(['crate',...entry.owner.split('.')],[]);if(owner?.k!=='data'||owner.fullName!==entry.owner)return null;
 const origin={...structuredClone(entry.result),type:{kind:'named',path:['crate',...entry.owner.split('.')]},path:[],wrappers:[],proven:true,copyEvidence:row.copy,sourceOwner:{kind:'declaration',path:entry.owner.split('.'),arguments:[]}};
 return createRustStandardSumConstructorOrigin(parser,node,origin,args,path,span);
}
export function projectRustNominalPatternField(table,parent,field,index,borrowed){
 let result=projectRustCopyClosureValueField(parent,{...structuredClone(field),sourceLexicalBindingIdentity:null,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceOwnedResultProof:false,sourceNominalPatternMove:false,sourceNominalBorrowedField:false},index);
 const direct={...parent,wrappers:[]},facts=readRustNominalOwnershipFacts(table,direct);if(facts.profile!=='owned'){if(borrowed){result.wrappers.unshift('reference');result.sourceLifetimes=[parent.sourceLifetimes?.[0]??null,...(result.sourceLifetimes??[])];result=projectRustCopyClosureValueField(parent,result,index);result.sourceValueBorrowBindings=mergeRustOwnedLoanBindings(readRustCopyClosureValueBorrows(result),readRustCopyClosureValueBorrows(parent));}return result;}
 if(borrowed){result.wrappers.unshift('reference');result.sourceLifetimes=[parent.sourceLifetimes?.[0]??null,...(result.sourceLifetimes??[])];result.sourceNominalBorrowedField=true;result.sourceBorrowedPlace=true;result.sourceBindingIdentity=parent.sourceBindingIdentity??parent.sourcePlaceBindingIdentity??null;result.sourcePlaceBindingIdentity=parent.sourcePlaceBindingIdentity??parent.sourceBindingIdentity??null;result.sourceLoanBindingIdentities=mergeRustOwnedLoanBindings(projectRustOwnedLoanBindings(parent.sourceBindingIdentity??'',parent.sourceLoanBindingIdentities??[]),projectRustOwnedLoanBindings(parent.sourcePlaceBindingIdentity??'',parent.sourcePlaceLoanBindingIdentities??[]));result.sourceValueBorrowBindings=mergeRustOwnedLoanBindings(readRustCopyClosureValueBorrows(result),readRustCopyClosureValueBorrows(parent));}
 else if(isRustOwnedBindingValue(table,field)){result.sourceBindingIdentity=parent.sourceBindingIdentity??null;result.sourceOwnedResultProof=parent.sourceOwnedResultProof===true;result.sourceBorrowedPlace=parent.sourceBorrowedPlace===true;result.sourceNominalPatternMove=true;}
 if(borrowed){result=projectRustCopyClosureValueField(parent,result,index);result.sourceValueBorrowBindings=mergeRustOwnedLoanBindings(readRustCopyClosureValueBorrows(result),readRustCopyClosureValueBorrows(parent));}
 return result;
}
export function prepareRustNominalPatternTransfers(table,maps,span,deferredGuard=false){
 const prepared=new Set();if(table.collecting||maps.length===0)return prepared;
 const rows=maps.map(map=>[...map].sort(([left],[right])=>left<right?-1:left>right?1:0)),plans=rows.map(row=>planRustNominalPatternTransfers(row.map(([,value])=>value.sourceBindingIdentity??''),row.map(([,value])=>value.sourceNominalPatternMove===true),row.map(([,value])=>isRustOwnedBindingValue(table,value)),row.map(([,value])=>value.sourceOwnedResultProof===true),deferredGuard));
 for(const plan of plans)if(plan[0]==='error')throw unsupported('Rust nominal pattern ownership',plan[1],span);
 const names=rows[0].filter(([,value])=>value.sourceNominalPatternMove===true&&isRustOwnedBindingValue(table,value)).map(([name])=>name);
 const same=maps.every(map=>names.every(name=>{const first=maps[0].get(name),value=map.get(name);return value?.sourceNominalPatternMove===true&&isRustOwnedBindingValue(table,value)&&table.same(first,value)&&first.sourceBindingIdentity===value.sourceBindingIdentity&&first.sourceOwnedResultProof===value.sourceOwnedResultProof;})&&[...map.values()].filter(value=>value.sourceNominalPatternMove===true&&isRustOwnedBindingValue(table,value)).length===names.length);
 const alternativeError=readRustNominalPatternAlternativeError(same);if(alternativeError)throw unsupported('Rust nominal pattern ownership',alternativeError,span);
 // Alternatives describe one runtime match; consume each actual parent once.
 for(const index of plans[0].slice(1))consumeRustOwnedOrigin(table,rows[0][Number(index)][1],span);
 for(const map of maps)for(const name of names){const value=map.get(name);map.set(name,{...value,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceOwnedResultProof:true,sourceNominalPatternMove:false});prepared.add(name);}
 return prepared;
}
export function readRustNominalClone(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext()||method.value!=='clone')return null;
 const table=parser.origins,actual=table.origin(receiver,path),facts=readRustNominalOwnershipFacts(table,actual);if(!facts.profile)return null;
 const selected=parser.inherentMethods.get(facts.owner+'.clone');if(acceptRustNominalCloneInherentOverride(facts.owner,selected?.owner??'',selected?.mode??0))return null;
 checkRustOwnedOrigin(table,actual,receiver.span);const error=readRustNominalCloneError(true,facts.profile,readRustSourceCloneImplementation(table,actual),actual.wrappers.length,actual.sourceBorrowedPlace===true,trackRustOwnedBindings(table),table.availableOwnedBinding(actual),actual.sourceOwnedResultProof===true);if(error)throw unsupported('Rust nominal Clone',error,receiver.span);
 const c=parser.cursor;c.expect('(','nominal Clone');const close=c.expect(')','nominal Clone'),span={start:receiver.span?.start??method.start,end:close.end},name=parser.freshProductBinding();
 const local=table.mark({k:'name',path:[name],span},actual),node={k:'let',name,type:actual.type,value:receiver,body:local,span};
 return table.mark(node,{...structuredClone(actual),sourceLexicalBindingIdentity:null,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceBorrowedPlace:false,sourceNominalPatternMove:false,sourceNominalBorrowedField:false,sourceOwnedResultProof:facts.profile==='owned'});
}
export function readRustSourceCloneImplementation(origins,origin,depth=0,active=new Set(),context={visits:0}){
 if(readRustNominalOwnershipGraphError(++context.visits,0,depth)!==''||!origin?.proven)return false;
 const container=readRustOwnedContainerFacts(origins,origin);if(container.node==='box'||container.node==='vector')return container.profile==='owned'&&container.closed&&container.cloneImplemented;
 const owner=origin.type?.kind==='named'?origins.lookupTypeOwner(origin.type.path,origin.path):null,stored=owner?.standardSumSourceOrigin,sum=stored?.standardSumSource;
 if(sum){if(stored.proven!==true||sum.provisional||active.has(owner.fullName))return false;active.add(owner.fullName);try{return sum.arguments.every(origin=>readRustSourceCloneImplementation(origins,origin,depth+1,active,context));}finally{active.delete(owner.fullName);}}
 if(owner)return origins.nominalCopyOwners.has(owner.fullName)||readRustNominalOwnershipFacts(origins,origin).cloneImplemented;
 const type=origin.type,source=origin.expandedSourceOwner??origin.sourceOwner;if(!type||!source)return false;
 if(type.kind==='array'){const element={...origins.typeOrigin(type.element,origin.path),sourceOwner:source.arguments[0]};if(acceptRustNominalVerifiedVectorClone(type.kind,origin.wrappers,origin.proven===true,source.kind,source.path.join('::'),source.arguments.length,origin.arraySourceModel??'',acceptRustNominalVectorElement(origins,element)))return true;}
 const leaf=readRustNominalOwnershipLeaf(type.kind,String(type.bits??''),type.signed===true,origin.wrappers,origin.proven===true,origins.immutableProduct(origin),source.kind,source.path.join('::'),source.path.length,source.arguments.length,origin.arraySourceModel??origin.textSourceModel??'',false,type.kind==='function'?type.params.length:-1,true,true,type.kind==='array'?type.element.kind:'');
 if(leaf==='copy'||leaf==='string')return true;
 if(leaf==='tuple')return Boolean(origin.fields?.every(field=>readRustSourceCloneImplementation(origins,field,depth+1,active,context)));
 if(leaf==='vector')return readRustSourceCloneImplementation(origins,{...origins.typeOrigin(type.element,origin.path),sourceOwner:source.arguments[0]},depth+1,active,context);
 return false;
}
