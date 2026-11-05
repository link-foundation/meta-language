// A direct field is a place in its actual owner, not storage allocated by the
// temporary pattern binder used to represent projection in the common core.
import {typeError,unsupported} from './diagnostics.js';
import {sameType} from './types.js';
import {hasRustSourceRegionComposition,retainRustSourceRegionState} from './rust-source-region-composition.js';
import {qualifyRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {createRustBorrowedVectorElementPosition} from './rust-borrowed-vectors.js';
import {checkRustSourceExpressionPosition} from './rust-source-expression-positions.js';
import {sameRustSourcePlaceOwner} from './rust-source-place-transforms.js';
import {checkOwnedProgramContainers} from './owned-program-containers.js';
import {acceptRustDataTypeFieldSignatureContainer} from './rust-data-type-field-signature-rules.js';
import {readRustRecordPlaceOwner} from './rust-record-place-owner-rules.js';
import {readRustSourceRecordProjectionOwner,projectRustSourceRecordFieldImage} from './rust-source-field-images.js';
const model='Rust source record place 1',exact=(value,keys)=>value&&typeof value==='object'&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));
const fail=reason=>{throw typeError(reason);};
export function attachRustRecordPlace(parser,node,root,field,index){
 if(parser.origins.collecting||!hasRustSourceRegionComposition(parser)||!retainRustSourceRegionState(parser)||field.arrayElementPosition)return node;
 const position=origin=>{const actual=qualifyRustStandardSumSourceOrigin(parser,origin);if(!actual)throw unsupported('Rust source record place','the actual parent and field retain their complete qualified source positions',node.span);return createRustBorrowedVectorElementPosition(actual);};
 const owner=position(root),selected=position(field);if(!readRustRecordPlaceOwner(owner)||!readRustRecordPlaceOwner(selected,false))return node;
 return parser.origins.reuseExpressionOrigin(node,{...node,sourceRecordPlace:{model,root:owner,field:index,value:selected,span:{...node.span}}});
}
function plain(node){if(!node||node.sourcePosition||node.sourceStateTransition||node.sourceLoopFold||node.sourcePlaceTransform||node.sourceRecordPlace)fail('a source field projection retains its actual generated pattern plumbing');}
export function readRustRecordPlacePlan(facet,node,language,resolveType=value=>value,resolveData=()=>null){
 checkOwnedProgramContainers({sourceRecordPlace:facet},{selectedFields:['sourceRecordPlace'],acceptContainer:acceptRustDataTypeFieldSignatureContainer,fail});
 if(language!=='Rust'||!exact(facet,['model','root','field','value','span'])||facet.model!==model||!Number.isSafeInteger(facet.field)||facet.field<0||Object.is(facet.field,-0)||!exact(facet.span,['start','end'])||!Number.isSafeInteger(facet.span.start)||!Number.isSafeInteger(facet.span.end)||facet.span.start<0||facet.span.end<=facet.span.start||node.span?.start!==facet.span.start||node.span?.end!==facet.span.end||node.sourceStateTransition||node.sourceLoopFold||node.sourcePlaceTransform)fail('a source record place retains its exact model, original field span and actual declaration index');
 let projected=node,base=node.scrutinee;if(node.k==='let'){projected=node.body;plain(projected);if(projected?.scrutinee?.k!=='var'||projected.scrutinee.name!==node.name)fail('a source projection wrapper evaluates its actual owner once');plain(projected.scrutinee);base=node.value;}
 if(projected?.k!=='match'||!Array.isArray(projected.cases)||projected.cases.length!==1||!base?.type)fail('a source record place retains its actual one-constructor projection');
 const position=(value,type)=>checkRustSourceExpressionPosition({model:'Rust borrowed expression source position 1',position:value},type,language,resolveType,resolveData).position,root=position(facet.root,base.type),value=position(facet.value,node.type),owner=readRustRecordPlaceOwner(root),data=owner?.position.type.kind==='data'?resolveData(owner.position.type):null,fields=data?.sourceFieldSignatures?.ctors?.[0]?.fields;
 if(!owner||root.arrayElementPosition||data?.k!=='data'||data.ctors.length!==1||!Array.isArray(fields)||fields.length!==data.ctors[0].fields.length||facet.field>=fields.length)fail('a source field place retains its actual complete closed record owner through shared references and canonical Box layers');
 const selected=fields[facet.field],row=projected.cases[0],pattern=row?.pattern;if(!sameType(selected.type,value.type)||!sameRustSourcePlaceOwner(selected.owner,value.owner)||selected.wrappers.join('/')!==value.wrappers.join('/')||selected.lifetimes.join('/')!==value.lifetimes.join('/')||Boolean(selected.arrayElementPosition)!==Boolean(value.arrayElementPosition)||!readRustRecordPlaceOwner(selected,false)||selected.arrayElementPosition)fail('a source field place retains the actual complete direct or canonical owned Box field position');
 // The common pattern compiler names even wildcard fields. Those unused names
 // grant no field reads or moves: the complete body is only the selected var.
 if(pattern?.k!=='ctor'||pattern.data!==data.fullName||pattern.ctor!==data.ctors[0].name||!Array.isArray(pattern.binds)||pattern.binds.length!==fields.length||typeof pattern.binds[facet.field]!=='string'||pattern.binds.some(name=>name!==null&&(typeof name!=='string'||!name.length))||new Set(pattern.binds.filter(name=>name!==null)).size!==pattern.binds.filter(name=>name!==null).length||row.body?.k!=='var'||row.body.name!==pattern.binds[facet.field]||!sameType(projected.type,value.type))fail('a source field place binds exactly its actual selected declaration field');plain(row.body);
 return {facet:{model,root,field:facet.field,value,span:{...facet.span}},base};
}
export function checkRustRecordPlace(...args){return readRustRecordPlacePlan(...args).facet;}
export function readRustRecordPlaceRegions(visitor,node,env,bindings,depth){
 const {facet,base}=readRustRecordPlacePlan(node.sourceRecordPlace,node,visitor.program.sourceLanguage,value=>value,value=>visitor.program.declarations.get(value.name));visitor.scope(facet.root,bindings,node);visitor.closedTarget(facet.root,node);if(facet.value.wrappers.length)visitor.closedTarget(facet.value,node);else visitor.closedOutput(facet.value,node);const owner=readRustSourceRecordProjectionOwner(visitor,base,env,bindings,facet.root,depth);
 return projectRustSourceRecordFieldImage(visitor,owner,facet,node);
}
