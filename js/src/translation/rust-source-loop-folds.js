// Source loop interfaces retain types/binders, never proof epochs or runtime IDs.
import {typeError,unsupported} from './diagnostics.js';
import {sameType} from './types.js';
import {hasRustSourceRegionComposition,retainRustSourceRegionState} from './rust-source-region-composition.js';
import {createRustBorrowedVectorElementPosition} from './rust-borrowed-vectors.js';
import {qualifyRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {checkRustSourceExpressionPosition} from './rust-source-expression-positions.js';
import {checkRustSourceStateTransition} from './rust-source-state-transitions.js';
import {testRustStandardCollectionProjectedGraphEquality as equal} from './rust-standard-collection-call-adapter.js';
import {checkOwnedProgramContainers} from './owned-program-containers.js';
import {acceptRustDataTypeFieldSignatureContainer} from './rust-data-type-field-signature-rules.js';
const model='Rust owned vector source fold 1';
const exact=(value,keys)=>value&&typeof value==='object'&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));
const fail=message=>{throw typeError(message);};
export function attachRustOwnedVectorSourceFold(parser,node,source,slots,control){
 if(!hasRustSourceRegionComposition(parser)||!retainRustSourceRegionState(parser))return node;
 if(!source.view.owned)throw unsupported('Rust source loop fold','shared collection loops require their actual collection region interface',source.span);
 const position=origin=>{const actual=qualifyRustStandardSumSourceOrigin(parser,origin);if(!actual)throw unsupported('Rust source loop fold','each source fold position needs its actual complete owner',source.span);return createRustBorrowedVectorElementPosition(actual);};
 return {...node,sourceLoopFold:{model,input:position(source.view.actual),item:position(source.view.item),result:position(source.result),control,slots:slots.map(slot=>({binder:{...slot.sourceBinder},position:position(slot.origin)})),span:{...source.span}}};
}
export function checkRustOwnedVectorSourceFold(facet,node,language,resolveType=value=>value,resolveData=()=>null){
 checkOwnedProgramContainers({sourceLoopFold:facet},{selectedFields:['sourceLoopFold'],acceptContainer:acceptRustDataTypeFieldSignatureContainer,fail});
 if(language!=='Rust'||!exact(facet,['model','input','item','result','control','slots','span'])||facet.model!==model||!Array.isArray(facet.slots)||facet.slots.length>512||!exact(facet.span,['start','end'])||!Number.isSafeInteger(facet.span.start)||!Number.isSafeInteger(facet.span.end)||facet.span.start<0||facet.span.end<=facet.span.start)fail('a source fold needs its exact bounded Rust interface and original source span');
 if(!(facet.control?.kind==='named'&&exact(facet.control,['kind','path'])||facet.control?.kind==='data'&&exact(facet.control,['kind','name'])))fail('a source fold control retains only its actual nominal type identity');
 if(!facet.result?.type||node.span?.start!==facet.span.start||node.span?.end!==facet.span.end)fail('a source fold retains its actual enclosing result and complete source span');
 if(node.k!=='method'||node.op!=='reduce'||node.args?.length!==2||node.args[0].k!=='lambda'||node.args[0].params.length!==2||node.type?.kind!=='tuple'||node.type.elements.length!==facet.slots.length+2||node.type.elements[0].kind!=='bool'||node.object?.type?.kind!=='array')fail('a source fold interface belongs to its actual two-parameter initialized reduce packet');
 const checkedPosition=(value,type)=>checkRustSourceExpressionPosition({model:'Rust borrowed expression source position 1',position:value},type,language,resolveType,resolveData).position;
 const input=checkedPosition(facet.input,node.object.type),item=checkedPosition(facet.item,node.object.type.element),resultType=resolveType(facet.result.type),result=checkedPosition(facet.result,resultType),control=resolveType(facet.control),declaration=resolveData(control),callback=node.args[0];
 if(input.wrappers.length||input.lifetimes.length||input.arrayElementPosition||input.owner.kind!=='library'||input.owner.path.join('::')!=='std::vec::Vec'||input.owner.arguments.length!==1||item.wrappers.length||item.lifetimes.length||item.arrayElementPosition||!equal(input.owner.arguments[0],item.owner))fail('an owned source fold retains its actual direct standard Vec and closed item positions');
 if(control.kind!=='data'||!sameType(control,node.type.elements[1])||!sameType(callback.params[0].type,node.type)||!sameType(callback.params[1].type,item.type)||callback.type?.kind!=='function'||!sameType(callback.type.ret,node.type)||!sameType(node.args[1].type,node.type))fail('a source fold retains every actual accumulator, item and result type');
 const body=callback.body,initial=node.args[1],accumulator=callback.params[0].name;
 if(body?.k!=='if'||body.cond?.k!=='tupleIndex'||body.cond.index!==0||body.cond.object?.k!=='var'||body.cond.object.name!==accumulator||body.then?.k!=='var'||body.then.name!==accumulator||initial.k!=='tuple'||initial.items[0]?.k!=='lit'||initial.items[0].value!==false||initial.items[1]?.k!=='ctor'||initial.items[1].data!==control.name||initial.items[1].ctor!=='StatementContinuation')fail('a source fold starts active and passes through its exact accumulator after completion');
 const variants=['FunctionResult','StatementContinuation','LoopBreak','LoopContinue'];
 if(declaration?.k!=='data'||declaration.ctors.length!==variants.length||declaration.ctors.some((ctor,index)=>ctor.name!==variants[index]||ctor.fields.length!==(index===0?1:0))||!sameType(resolveType(declaration.ctors[0].fields[0].type),result.type))fail('a source fold retains its actual complete compiler control layout and result type');
 const identities=new Set(),slots=facet.slots.map((slot,index)=>{if(!exact(slot,['binder','position']))fail('a source fold slot retains its exact source binder and position');const checked=checkRustSourceStateTransition({model:'Rust source state transition 1',operation:'initialize',...slot},node.type.elements[index+2],language,resolveType,resolveData);if(identities.has(checked.binder.identity))fail('source fold slots retain distinct actual binders');identities.add(checked.binder.identity);return {binder:checked.binder,position:checked.position};});
 return {model,input,item,result,control,slots,span:{...facet.span}};
}
