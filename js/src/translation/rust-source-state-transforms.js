// Compiler state functions have their own ABI; they are not invented Rust
// source headers. Original source admission and body checking remain separate.
import {typeError,unsupported} from './diagnostics.js';
import {sameType} from './types.js';
import {assertRustOwnedRecordLoopAdmission} from './rust-owned-record-loops.js';
import {testRustOwnedRecordLoopChunkName} from './rust-owned-record-loop-rules.js';
import {hasRustSourceRegionComposition,retainRustSourceRegionState} from './rust-source-region-composition.js';
import {qualifyRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {createRustBorrowedVectorElementPosition} from './rust-borrowed-vectors.js';
import {checkRustSourceStateTransition} from './rust-source-state-transitions.js';
import {checkRustSourceExpressionPosition} from './rust-source-expression-positions.js';
import {checkOwnedProgramContainers} from './owned-program-containers.js';
import {acceptRustDataTypeFieldSignatureContainer} from './rust-data-type-field-signature-rules.js';
const model='Rust finite loop state transformation 1';
const exact=(value,keys)=>value&&typeof value==='object'&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));
const fail=message=>{throw typeError(message);};
export function attachRustFiniteLoopStateTransform(parser,fn,source,control){
 if(!hasRustSourceRegionComposition(parser)||!retainRustSourceRegionState(parser))return fn;assertRustOwnedRecordLoopAdmission(source);
 const position=origin=>{const actual=qualifyRustStandardSumSourceOrigin(parser,origin);if(!actual)throw unsupported('Rust source state transformation','a generated state function retains each complete actual source position',source.span);return createRustBorrowedVectorElementPosition(actual);};
 const result=control?source.result:parser.origins.typeOrigin({kind:'unit'},source.path);
 return {...fn,sourceStateTransform:{model,slots:source.slots.map(slot=>({binder:{...slot.sourceBinder},position:position(slot.origin)})),control,result:position(result),span:{...source.span}}};
}
export function checkRustFiniteLoopStateTransform(facet,entry,language,resolveType=value=>value,resolveData=()=>null){
 checkOwnedProgramContainers({sourceStateTransform:facet},{selectedFields:['sourceStateTransform'],acceptContainer:acceptRustDataTypeFieldSignatureContainer,fail});
 if(language!=='Rust'||!exact(facet,['model','slots','control','result','span'])||facet.model!==model||entry.k!=='fn'||entry.generated!==true||!testRustOwnedRecordLoopChunkName(entry.name)||!Array.isArray(facet.slots)||facet.slots.length<1||facet.slots.length>512||entry.params.length!==facet.slots.length||entry.ret.kind!=='tuple'||entry.ret.elements.length!==facet.slots.length+1||!exact(facet.span,['start','end'])||!Number.isSafeInteger(facet.span.start)||!Number.isSafeInteger(facet.span.end)||facet.span.start<0||facet.span.end<=facet.span.start||entry.span?.start!==facet.span.start||entry.span?.end!==facet.span.end)fail('a finite state transformation retains its actual generated function, bounded state slots and original loop span');
 if(entry.sourceSignature!==undefined||entry.sourceExclusiveSignature!==undefined||entry.sourceFnMutSpecialization!==undefined)fail('compiler state functions retain their own interface rather than a Rust source signature');
 const identities=new Set(),slots=facet.slots.map((slot,index)=>{if(!exact(slot,['binder','position'])||!sameType(entry.params[index].type,entry.ret.elements[index+1]))fail('a compiler state function retains every ordered input/output state type');const checked=checkRustSourceStateTransition({model:'Rust source state transition 1',operation:'initialize',...slot},entry.params[index].type,language,resolveType,resolveData);if(identities.has(checked.binder.identity))fail('compiler state formals retain distinct actual source binders');identities.add(checked.binder.identity);return {binder:checked.binder,position:checked.position};});
 if(facet.control!==null&&!(facet.control?.kind==='named'&&exact(facet.control,['kind','path'])||facet.control?.kind==='data'&&exact(facet.control,['kind','name'])))fail('a compiler state completion is explicit Unit or its actual nominal type');
 const control=facet.control===null?null:resolveType(facet.control);
 if(control){if(control.kind!=='data'||!sameType(control,entry.ret.elements[0]))fail('a compiler state function retains its actual nominal completion type');}
 else if(entry.ret.elements[0].kind!=='unit')fail('a nonreturning compiler state function retains actual Unit completion');
 if(!facet.result?.type)fail('a compiler state function retains its complete completion payload position');
 const payloadType=control?resolveType(facet.result.type):entry.ret.elements[0],result=checkRustSourceExpressionPosition({model:'Rust borrowed expression source position 1',position:facet.result},payloadType,language,resolveType,resolveData).position;
 if(!control&&(result.wrappers.length||result.lifetimes.length||result.owner.kind!=='unit'))fail('a nonreturning compiler state function has direct Unit source completion');
 if(control){const data=resolveData(control),names=['FunctionResult','StatementContinuation','LoopBreak','LoopContinue'];if(data?.k!=='data'||data.ctors.length!==names.length||data.ctors.some((ctor,index)=>ctor.name!==names[index]||ctor.fields.length!==(index===0?1:0))||!sameType(resolveType(data.ctors[0].fields[0].type),result.type))fail('a compiler state completion retains its actual complete declaration and payload');}
 return {model,slots,control,result,span:{...facet.span}};
}
