// Lexical identities are separate from owned-value identities. Reuse the
// existing source lifetime state snapshots/drop transitions, never Copy moves.
import {unsupported} from './diagnostics.js';
import {mergeRustOwnedLoanBindings} from './rust-owned-binding-rules.js';
import {readRustCopyClosureValueBorrowError,readRustCopyClosureReturnedBorrowError} from './rust-copy-closure-rules.js';
import {promoteRustMutableRecordValueLoans} from './rust-mutable-record-loans.js';
export function assignRustCopyClosureLexicalBinding(table,origin){
 if(table.collecting)return origin;
 const identity='lexical '+String(table.nextSourceBindingIdentity++);origin.sourceLexicalBindingIdentity=identity;table.sourceBindingStates.set(identity,'available');return promoteRustMutableRecordValueLoans(table,origin);
}
export function readRustCopyClosureValueBorrows(origin){
 const pending=origin?[origin]:[],seen=new Set();let result=[];
 while(pending.length){const value=pending.pop();if(!value||seen.has(value))continue;seen.add(value);result=mergeRustOwnedLoanBindings(result,value.sourceValueBorrowBindings??[]);for(const fields of value.sourceValueFieldBorrowBindings??[])result=mergeRustOwnedLoanBindings(result,fields);pending.push(...(value.fields??[]));}
 return result;
}
export function validateRustCopyClosureValueBorrows(table,origin,span){
 if(table.collecting)return;const error=readRustCopyClosureValueBorrowError(readRustCopyClosureValueBorrows(origin).map(identity=>table.sourceBindingStates.get(identity)??''));if(error)throw unsupported('Rust Copy closure lifetime',error,span);
}
export function readRustCopyClosureCaptureBorrows(table,captured,explicitMove,span){
 let result=[];for(const origin of captured){result=mergeRustOwnedLoanBindings(result,readRustCopyClosureValueBorrows(origin));if(!explicitMove&&!table.collecting){if(!origin?.sourceLexicalBindingIdentity)throw unsupported('Rust Copy closure lifetime','an implicit capture needs its actual lexical binding identity',span);result=mergeRustOwnedLoanBindings(result,[origin.sourceLexicalBindingIdentity]);}}
 const error=table.collecting?'':readRustCopyClosureValueBorrowError(result.map(identity=>table.sourceBindingStates.get(identity)??''));if(error)throw unsupported('Rust Copy closure lifetime',error,span);return result;
}
export function refuseRustCopyClosureReturnedBorrows(table,origin,span){if(table.collecting)return;const error=readRustCopyClosureReturnedBorrowError(readRustCopyClosureValueBorrows(origin).length);if(error)throw unsupported('Rust Copy closure lifetime',error,span);}
function mayContainCallable(origin){
 if(origin?.wrappers?.some(layer=>layer==='reference'))return true;
 if(origin?.standardSumSource)return origin.standardSumSource.arguments.some(mayContainCallable);
 if(origin?.type?.kind==='tuple'&&origin.fields?.length===origin.type.elements.length)return origin.fields.some(mayContainCallable);
 const pending=[origin?.type],seen=new Set();if(!origin?.type)return true;
 while(pending.length){const type=pending.pop();if(!type||seen.has(type))continue;seen.add(type);if(type.kind==='function'||type.kind==='named')return true;if(type.kind==='tuple')pending.push(...type.elements);else if(type.kind==='array')pending.push(type.element);}
 return false;
}
export function carryRustCopyClosureValueBorrows(result,actual){
 result.sourceValueBorrowBindings=mayContainCallable(result)?readRustCopyClosureValueBorrows(actual):[];
 result.sourceValueFieldBorrowBindings=mayContainCallable(result)?actual?.sourceValueFieldBorrowBindings?.map(values=>[...values])??actual?.fields?.map(readRustCopyClosureValueBorrows)??null:null;return result;
}
export function carryRustCopyClosureValueUnion(result,actuals){
 result.sourceValueBorrowBindings=mayContainCallable(result)?actuals.reduce((all,actual)=>mergeRustOwnedLoanBindings(all,readRustCopyClosureValueBorrows(actual)),readRustCopyClosureValueBorrows(result)):[];
 result.sourceValueFieldBorrowBindings=null;return result;
}
export function projectRustCopyClosureValueField(parent,field,index){
 const values=parent.sourceValueFieldBorrowBindings?.[index]??readRustCopyClosureValueBorrows(parent);field.sourceLexicalBindingIdentity=parent.sourceLexicalBindingIdentity??null;field.sourceValueBorrowBindings=mayContainCallable(field)?[...values]:[];field.sourceValueFieldBorrowBindings=null;return field;
}
