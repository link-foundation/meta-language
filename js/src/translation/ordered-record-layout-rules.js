// Complete target syntax over actual rendered payload/order operands. Order
// admission and enumeration algorithms remain in the shared source runtime.
import {renderRecordValue} from './record-emission-rules.js';
/** Ordered record storage includes its canonical Number position vector.
 * @param {string} kind @param {boolean} ordered @returns {boolean} */
export function acceptOrderedRecordNumberStorage(kind,ordered){return kind==='record'&&ordered;}
/** @param {string} target @param {string} payload @param {string} order @returns {string} */
export function renderOrderedRecordType(target,payload,order){
 if(target==='Rust')return '('+payload+', '+order+')';
 if(target==='Lean')return '('+payload+' × '+order+')';
 return target==='Rocq'?'(('+payload+' * '+order+')%type)':'';
}
/** @param {string} target @param {string} object @returns {string} */
export function renderOrderedRecordPayload(target,object){
 if(target==='JavaScript')return object;
 if(target==='Rust')return '('+object+').0';
 if(target==='Lean')return '('+object+').1';
 return target==='Rocq'?'(fst '+object+')':'';
}
/** @param {string} target @param {string} object @returns {string} */
export function renderOrderedRecordOrder(target,object){
 if(target==='Rust')return '('+object+').1.clone()';
 if(target==='Lean')return '('+object+').2';
 return target==='Rocq'?'(snd '+object+')':'';
}
/** The supplied order is the result of the full source-defined constructor,
 * never an unchecked permutation witness. Actual values precede order effects.
 * @param {string} target @param {string[]} labels @param {string[]} values @param {string} order @returns {string} */
export function renderOrderedRecordValue(target,labels,values,order){
 const payload=renderRecordValue(target,labels,values);
 if(target==='Rust'||target==='Lean'||target==='Rocq')return '('+payload+', '+order+')';
 if(target!=='JavaScript')return '';
 return '((ml_fields, ml_order) => { const ml_object = {}; const ml_names = ['+labels.join(', ')+']; for (const ml_field of ml_order) { Object.defineProperty(ml_object, ml_names[ml_field], { value: ml_fields[ml_field], writable: true, enumerable: true, configurable: true }); } return ml_object; })(['+values.join(', ')+'], '+order+')';
}
/** @param {string} target @param {string} object @param {string[]} labels @returns {string} */
export function renderOrderedRecordObservedOrder(target,object,labels){
 if(target==='JavaScript')return '((ml_object) => { const ml_names = ['+labels.join(', ')+']; return Object.keys(ml_object).map(ml_name => ml_names.indexOf(ml_name)); })('+object+')';
 return renderOrderedRecordOrder(target,object);
}
/** @param {string} operation @returns {string} */
export function readRecordPropertyOrderKernelEntry(operation){
 if(operation==='createJavaScriptRecordPropertyOrder')return 'create';
 if(operation==='mergeJavaScriptRecordPropertyOrder')return 'merge';
 if(operation==='rebaseJavaScriptRecordPropertyOrder')return 'rebase';
 if(operation==='readJavaScriptRecordPropertyKeys')return 'keys';
 return operation==='acceptJavaScriptRecordPropertyOrder'?'valid':'';
}
/** @param {string} target @param {string} operation @param {string[]} args @returns {string} */
export function renderRecordPropertyOrderKernelCall(target,operation,args){
 const entry=readRecordPropertyOrderKernelEntry(operation);
 if(!entry)return '';
 if(target==='JavaScript')return 'ml_record_property_order.'+entry+'('+args.join(', ')+')';
 if(target==='Lean')return '(MLRecordPropertyOrder.'+entry+' '+args.join(' ')+')';
 if(target==='Rocq')return '(MLRecordPropertyOrder.'+entry+' '+args.join(' ')+')';
 if(target!=='Rust')return '';
 const name=entry==='create'?'create_java_script_record_property_order':entry==='merge'?'merge_java_script_record_property_order':entry==='rebase'?'rebase_java_script_record_property_order':entry==='keys'?'read_java_script_record_property_keys':'accept_java_script_record_property_order';
 return 'crate::ml_record_property_order::'+name+'('+args.map(value=>'&('+value+')').join(', ')+')';
}
