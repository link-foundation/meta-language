// Owned Unicode strings and immutable string inputs over retained scalar calls.
import {acceptRustScalarMathematicalOperation} from './rust-scalar-mathematical-interface-rules.js';
import {readRustStringByteLengthOperation} from './rust-string-byte-length-rules.js';
/** @param {string} model @returns {boolean} */
export function acceptRustTextualSourceInterfaceModel(model){return model==='retained textual Rust function interface 1';}
/** @param {string} kind @param {string} path @param {number} children @returns {string} */
export function readRustTextualSourceType(kind,path,children){
 if(children!==0)return '';
 if(kind==='primitive'&&path==='str')return 'str';
 return kind==='library'&&path==='std::string::String'?'String':'';
}
/** @param {string} operation @param {number} count @param {string} result @returns {boolean} */
export function acceptRustTextualSourceOperation(operation,count,result){return operation===readRustStringByteLengthOperation()?count===1&&result==='fixed':acceptRustScalarMathematicalOperation(operation,count,result);}
/** Scalar typing remains with the core; string ordering needs its own model.
 * @param {string} domain @param {string} operation @returns {boolean} */
export function acceptRustTextualSourceBinary(domain,operation){return domain!=='string'||operation==='eq'||operation==='ne';}
/** Copy an actual standard String or str referent into the core Unicode value.
 * @param {string} value @returns {string} */
export function renderRustTextualSourceInput(value){return '::std::borrow::ToOwned::to_owned(&('+value+'))';}
/** The first immutable str borrow comes from the actual standard String slot.
 * @param {string} value @returns {string} */
export function renderRustTextualSourceSliceArgument(value){return '::std::string::String::as_str(&('+value+'))';}
/** @returns {string} */
export function readRustTextualSourceEncoding(){return 'retained-textual-source-interface';}
/** @returns {string} */
export function readRustTextualSourceStatement(){return 'retained scalar calls compose with owned standard Unicode String endpoints and immutable String/str inputs; complete argument values are materialized once before reborrowing; string byte lengths retain the exact UTF-8 measure and borrowed outputs, pointer/capacity observations and arbitrary ownership remain separate obligations';}
