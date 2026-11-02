// Actual SOURCE binder/referent transport, separate from TYPE-only positions.
// Revisions in the region interpreter are local proof coordinates and are
// never serialized here.
import {typeError,unsupported} from './diagnostics.js';
import {hasRustSourceRegionComposition,retainRustSourceRegionState} from './rust-source-region-composition.js';
import {readRustOwnedRecordStateDeclarations} from './rust-owned-record-state.js';
import {qualifyRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {createRustBorrowedVectorElementPosition} from './rust-borrowed-vectors.js';
import {checkRustSourceSignature} from './rust-source-signatures.js';
const exact=(value,keys)=>value&&typeof value==='object'&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));
function binder(value){return exact(value,['name','identity','start','end'])&&typeof value.name==='string'&&value.name.length>0&&value.name.length<=4096&&typeof value.identity==='string'&&value.identity.length>0&&value.identity.length<=4096&&Number.isSafeInteger(value.start)&&Number.isSafeInteger(value.end)&&value.start>=0&&value.end>value.start;}
export function attachRustSourceStateTransition(parser,node,slot,operation){
 if(!hasRustSourceRegionComposition(parser)||!retainRustSourceRegionState(parser))return node;
 const actual=slot&&readRustOwnedRecordStateDeclarations(parser).get(slot.currentName),tokens=parser.sourceConstantTokens;if(!Array.isArray(tokens)||tokens.length>1000000)throw unsupported('Rust source state transition','state publication needs its bounded original source token inventory',node.span);let first=0,last=tokens.length;while(first<last){const middle=Math.floor((first+last)/2);if(tokens[middle].start<(slot?.sourceBinder?.start??-1))first=middle+1;else last=middle;}const token=tokens[first];
 if(!['initialize','write'].includes(operation)||!binder(slot?.sourceBinder)||actual?.lexical!==slot.lexical||actual?.root!==slot.root||actual?.sourceBinder?.identity!==slot.sourceBinder.identity||token?.kind!=='identifier'||token.value!==slot.sourceBinder.name||token.start!==slot.sourceBinder.start||token.end!==slot.sourceBinder.end)throw unsupported('Rust source state transition','a state transition retains its actual admitted mutable slot and original binder token',node.span);
 const origin=qualifyRustStandardSumSourceOrigin(parser,slot.origin);if(!origin)throw unsupported('Rust source state transition','a state root needs its complete qualified referent origin',node.span);
 return parser.origins.reuseExpressionOrigin(node,{...node,sourceStateTransition:{model:'Rust source state transition 1',operation,binder:{...slot.sourceBinder},position:createRustBorrowedVectorElementPosition(origin)}});
}
export function checkRustSourceStateTransition(facet,type,language,resolveType=value=>value,resolveData=()=>null){
 if(language!=='Rust'||!exact(facet,['model','operation','binder','position'])||facet.model!=='Rust source state transition 1'||!['initialize','write'].includes(facet.operation)||!binder(facet.binder))throw typeError('a source state transition retains its exact Rust model, operation and actual source binder');
 const checked=checkRustSourceSignature({model:'Rust source function signature 2',language:'Rust',parameters:[],result:facet.position,lifetimeBindings:[]},[],type,language,resolveType,resolveData);if(checked.result.wrappers.length||checked.result.lifetimes.length||checked.result.arrayElementPosition)throw typeError('a source state transition publishes its actual direct closed referent');return {...facet,binder:{...facet.binder},position:checked.result};
}
