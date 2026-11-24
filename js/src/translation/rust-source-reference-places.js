// An explicit shared reference pattern exposes its actual referent place.
// Reference erasure does not grant owning transfer or a new storage lifetime.
import {typeError,unsupported} from './diagnostics.js';
import {sameType} from './types.js';
import {checkOwnedProgramContainers} from './owned-program-containers.js';
import {acceptRustDataTypeFieldSignatureContainer} from './rust-data-type-field-signature-rules.js';
import {checkRustSourceExpressionPosition} from './rust-source-expression-positions.js';
import {sameRustSourceRecordImagePosition} from './rust-source-field-images.js';
import {readRustRecordPlaceOwner} from './rust-record-place-owner-rules.js';
const model='Rust source shared referent place 1';
const exact=(value,keys)=>value&&typeof value==='object'&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));
const fail=reason=>{throw typeError(reason);};
export function createRustSourceSharedReferentPlace(parser,value,span){const name=parser.freshProductBinding();return {k:'let',name,value,body:{k:'name',path:[name],span},span,sourcePlaceTransform:{model,slots:[],places:[],span:{...span}}};}
export function readRustSourceSharedReferentPlacePlan(facet,node,language,resolveType=value=>value,resolveData=()=>null){
 checkOwnedProgramContainers({sourcePlaceTransform:facet},{selectedFields:['sourcePlaceTransform'],acceptContainer:acceptRustDataTypeFieldSignatureContainer,fail});
 if(language!=='Rust'||!exact(facet,['model','slots','places','span'])||facet.model!==model||!Array.isArray(facet.slots)||facet.slots.length||!Array.isArray(facet.places)||facet.places.length||!exact(facet.span,['start','end'])||!Number.isSafeInteger(facet.span.start)||!Number.isSafeInteger(facet.span.end)||facet.span.start<0||facet.span.end<=facet.span.start||node.span?.start!==facet.span.start||node.span?.end!==facet.span.end||node.k!=='let'||!node.type||!node.value?.type||!node.body?.type||typeof node.name!=='string'||!node.name.length||node.name.length>4096||node.sourceStateTransition||node.sourceLoopFold||node.sourceRecordPlace||node.body.k!=='var'||node.body.name!==node.name||node.body.sourcePosition||node.body.sourcePlaceTransform||node.body.sourceRecordPlace||node.body.sourceStateTransition||node.body.sourceLoopFold||!sameType(node.type,node.value.type)||!sameType(node.type,node.body.type)||!node.value.sourcePosition||!node.sourcePosition)fail('a shared referent place retains its exact one-evaluation identity graph, original span and complete source positions');
 const input=checkRustSourceExpressionPosition(node.value.sourcePosition,node.value.type,language,resolveType,resolveData).position,position=checkRustSourceExpressionPosition(node.sourcePosition,node.type,language,resolveType,resolveData).position;
 if(input.wrappers[0]!=='reference'||!readRustRecordPlaceOwner(input,true)||!readRustRecordPlaceOwner(position,true)||!sameRustSourceRecordImagePosition({...input,wrappers:input.wrappers.slice(1),lifetimes:input.lifetimes.slice(1)},position))fail('a shared referent place removes exactly one actual shared reference without replacing its owner, payload layers or remaining lifetimes');
 return {kind:'shared referent place',facet:{model,slots:[],places:[],span:{...facet.span}},value:node.value,input,position};
}
export function readRustSourceSharedReferentPlaceRegions(visitor,plan,node,env,bindings,depth){
 visitor.scope(plan.input,bindings,node);visitor.scope(plan.position,bindings,node);visitor.closedTarget(plan.input,node);
 const value=visitor.read(visitor.expression(plan.value,env,bindings,plan.input,depth+1),'read',node);
 if(value.role!=='reference')throw unsupported('Rust shared referent place','an explicit shared pattern retains its actual reference operand',node.span);
 const inner=visitor.inner(value.position);if(!sameRustSourceRecordImagePosition(inner,plan.position))throw unsupported('Rust shared referent place','the referent retains its actual runtime source owner and complete remaining position',node.span);
 const places=visitor.union([value.borrowRoots??value.regions]);if(!places.length||places.some(root=>!Number.isSafeInteger(root)||root<0||root>=visitor.states.length))throw unsupported('Rust shared referent place','the referent keeps its actual allocated source loan roots',node.span);
 const role=visitor.value(plan.position).role;if(!role)throw unsupported('Rust shared referent place','the referent retains its actual closed source value domain',node.span);
 return {...value,position:visitor.position(plan.position),role,root:places[0],storageRoot:places[0],storageIdentity:undefined,storageOwner:false,producer:false,borrowedPlace:true,placeRoots:places,placeRegions:visitor.union([value.regions,value.placeRegions??[],places]),inheritedBorrowRoots:visitor.union([value.inheritedBorrowRoots??[],places]),borrowRoots:role==='reference'?value.borrowRoots:undefined,regions:role==='reference'?value.regions:[]};
}
