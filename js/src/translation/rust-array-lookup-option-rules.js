// Stored owned results extend scalar .copied() only. Existing complete
// immediate-consumer pipelines keep their original source interpretation.
import {readRustArrayLookupArity,readRustArrayLookupConsumerArity} from './rust-array-lookup-rules.js';
/** @param {string} method @param {boolean} ordinary @param {boolean} copied
 * @param {string} nextConsumer @returns {string} */
export function readRustArrayLookupOptionDisposition(method,ordinary,copied,nextConsumer){
 if(readRustArrayLookupArity(method,ordinary)<0||!copied)return '';
 return readRustArrayLookupConsumerArity(nextConsumer,true)>=0?'existing consumer':'owned scalar option';
}
/** @param {string} method @returns {string[]} */
export function readRustArrayLookupOptionCaptureRoles(method){
 return method==='get'?['receiver','index']:method==='first'||method==='last'?['receiver']:[];
}
/** @returns {string} */
export function readRustArrayLookupOptionOwner(){return 'std::option::Option';}
/** @param {boolean} present @returns {string} */
export function readRustArrayLookupOptionVariant(present){return present?'Some':'None';}
