// Actual checked field/order operands into the shared source runtime/layout.
import {FLOAT,array} from './types.js';
import {typeError} from './diagnostics.js';
import {renderOrderedRecordValue} from './ordered-record-layout-rules.js';
export function constructRecordPropertyNameRows(entries){
 return {k:'array',items:entries.map(entry=>({k:'array',items:Array.from({length:entry.name.length},(_,index)=>({k:'lit',value:String(entry.name.charCodeAt(index)),type:FLOAT})),type:array(FLOAT)})),type:array(array(FLOAT))};
}
export function emitOrderedRecordValue(emitter,node,target,depth){
 if(Boolean(node.type.ordered)!==Boolean(node.order))throw typeError('an ordered record needs its actual own-field order and matching value type',node.span);
 if(!node.order)return null;
 const emit=value=>target==='Lean'?emitter.expr(value,depth):emitter.expr(value);
 const values=node.entries.map(entry=>emit(entry.value));
 return renderOrderedRecordValue(target,node.entries.map(entry=>JSON.stringify(entry.name)),values,emit(node.order));
}
