// Readonly ownership guards are real Boolean expressions. Their existing
// outer places retain ownership/revision state while the guard is evaluated.
import {typeError,unsupported} from './diagnostics.js';
import {checkOwnedProgramContainers} from './owned-program-containers.js';
import {acceptRustDataTypeFieldSignatureContainer} from './rust-data-type-field-signature-rules.js';
import {checkRustSourceExpressionPosition} from './rust-source-expression-positions.js';
import {readRustRecordPlacePlan} from './rust-record-place-regions.js';
import {sameType} from './types.js';
const model='Rust source readonly match guard 1',exact=(value,keys)=>value&&typeof value==='object'&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));
const fail=reason=>{throw typeError(reason);};
export function attachRustSourceReadonlyMatchGuard(parser,node){return parser.origins.reuseExpressionOrigin(node,{...node,sourcePlaceTransform:{model,slots:[],places:[],span:{...node.span}}});}
export function readRustSourceReadonlyMatchGuardPlan(facet,node,language,resolveType=value=>value,resolveData=()=>null){
 checkOwnedProgramContainers({sourcePlaceTransform:facet},{selectedFields:['sourcePlaceTransform'],acceptContainer:acceptRustDataTypeFieldSignatureContainer,fail});
 if(language!=='Rust'||!exact(facet,['model','slots','places','span'])||facet.model!==model||!Array.isArray(facet.slots)||facet.slots.length||!Array.isArray(facet.places)||facet.places.length||!exact(facet.span,['start','end'])||!Number.isSafeInteger(facet.span.start)||!Number.isSafeInteger(facet.span.end)||facet.span.start<0||facet.span.end<=facet.span.start||node.type?.kind!=='bool'||node.span?.start!==facet.span.start||node.span?.end!==facet.span.end||node.sourceStateTransition||node.sourceLoopFold||node.sourceRecordPlace)fail('a readonly source match guard retains its exact Boolean graph, original span and empty external state interface');
 if(node.sourcePosition){const p=checkRustSourceExpressionPosition(node.sourcePosition,node.type,language,resolveType,resolveData).position;if(p.type.kind!=='bool'||p.wrappers.length||p.lifetimes.length||p.arrayElementPosition||p.owner.kind!=='primitive'||p.owner.path.join('::')!=='bool')fail('a readonly source guard retains its actual direct Boolean result position');}
 const expression={...node};delete expression.sourcePlaceTransform;return {kind:'readonly match guard',facet:{model,slots:[],places:[],span:{...facet.span}},expression};
}
export function readRustSourceReadonlyMatchGuardRegions(visitor,plan,node,env,bindings,depth){
 const before=visitor.states.slice(),storages=new Map(visitor.storages);visitor.branches(2*before.length);
 const position={type:{kind:'bool'},owner:{kind:'primitive',path:['bool'],arguments:[]},wrappers:[],lifetimes:[]},value=visitor.read(visitor.expression(plan.expression,env,bindings,position,depth+1),'read',node);
 if(before.some((state,index)=>visitor.states[index]!==state)||storages.size!==visitor.storages.size||[...storages].some(([id,storage])=>visitor.storages.get(id)?.root!==storage.root))throw unsupported('Rust readonly match guard','a guard retains every pre-existing actual ownership and storage revision; effectful guards require their explicit successor protocol',node.span);
 return visitor.compatible(value,position,node);
}
const placeModel='Rust source readonly pattern place 1';
export function createRustSourceReadonlyPatternPlace(parser,value,span){const name=parser.freshProductBinding();return {k:'let',name,value,body:{k:'name',path:[name],span},span,sourcePlaceTransform:{model:placeModel,slots:[],places:[],span:{...span}}};}
export function readRustSourceReadonlyPatternPlacePlan(facet,node,language,resolveType=value=>value,resolveData=()=>null){
 checkOwnedProgramContainers({sourcePlaceTransform:facet},{selectedFields:['sourcePlaceTransform'],acceptContainer:acceptRustDataTypeFieldSignatureContainer,fail});
 if(language!=='Rust'||!exact(facet,['model','slots','places','span'])||facet.model!==placeModel||!Array.isArray(facet.slots)||facet.slots.length||!Array.isArray(facet.places)||facet.places.length||!exact(facet.span,['start','end'])||!Number.isSafeInteger(facet.span.start)||!Number.isSafeInteger(facet.span.end)||facet.span.start<0||facet.span.end<=facet.span.start||node.span?.start!==facet.span.start||node.span?.end!==facet.span.end||node.k!=='let'||!node.type||!node.value?.type||!node.body?.type||typeof node.name!=='string'||!node.name.length||node.name.length>4096||node.sourceStateTransition||node.sourceLoopFold||node.sourceRecordPlace||node.body?.k!=='var'||node.body.name!==node.name||node.body.sourcePosition||node.body.sourcePlaceTransform||node.body.sourceRecordPlace||node.body.sourceStateTransition||node.body.sourceLoopFold||!sameType(node.type,node.value?.type)||!sameType(node.type,node.body.type))fail('a readonly pattern place retains its actual one-evaluation projection or proven alias and exact result binding');
 if(node.value.sourceRecordPlace)readRustRecordPlacePlan(node.value.sourceRecordPlace,node.value,language,resolveType,resolveData);else if(node.value.k!=='var'||node.value.sourceStateTransition||node.value.sourcePlaceTransform||node.value.sourceLoopFold)fail('a readonly pattern alias retains its actual existing lexical place');
 const position=node.sourcePosition?checkRustSourceExpressionPosition(node.sourcePosition,node.type,language,resolveType,resolveData).position:null;
 return {kind:'readonly pattern place',facet:{model:placeModel,slots:[],places:[],span:{...facet.span}},value:node.value,position};
}
export function readRustSourceReadonlyPatternPlaceRegions(visitor,plan,node,env,bindings,depth){
 const value=visitor.read(visitor.expression(plan.value,env,bindings,null,depth+1),'read',node);if(!['owned','string','box'].includes(value.role)||value.regions.length)throw unsupported('Rust readonly pattern place','a guard view retains its actual closed owning source place',node.span);
 visitor.closedOutput(value.position,node);if(plan.position){visitor.scope(plan.position,bindings,node);visitor.compatible(value,plan.position,node);}
 const places=value.placeRoots??[value.storageRoot??value.root];if(!places.length||places.some(root=>!Number.isSafeInteger(root)||root<0||root>=visitor.states.length))throw unsupported('Rust readonly pattern place','a view retains its actual allocated lexical or selected source storage',node.span);
 let proof=value.readonlyPatternProof;if(!value.borrowedPlace||!visitor.readonlyPatternPlaces.has(proof)){proof={};visitor.readonlyPatternPlaces.add(proof);visitor.loans++;visitor.budget();}
 return {...value,borrowedPlace:true,readonlyPatternBinding:true,readonlyPatternProof:proof,placeRoots:[...places],placeRegions:visitor.union([value.placeRegions??[],places]),inheritedBorrowRoots:visitor.union([value.inheritedBorrowRoots??[],places])};
}
