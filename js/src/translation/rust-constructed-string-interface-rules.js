// An independent composition over the checked complete scalar string graph.
import {acceptRustTextualSourceBinary} from './rust-textual-source-interface-rules.js';
/** @param {string} model @returns {boolean} */
export function acceptRustConstructedStringInterfaceModel(model){return model==='retained constructed Rust string function interface 1';}
/** @param {string} domain @param {string} operation @returns {boolean} */
export function acceptRustConstructedStringBinary(domain,operation){return domain==='string'&&operation==='concat'||acceptRustTextualSourceBinary(domain,operation);}
/** @param {string} domain @param {string} result @param {string} left @param {string} right @returns {boolean} */
export function acceptRustConstructedStringConcatenation(domain,result,left,right){return (domain==='string'||domain==='implicit string')&&result==='string'&&left==='string'&&right==='string';}
/** @returns {string} */
export function readRustConstructedStringEncoding(){return 'retained-constructed-string-source-interface';}
/** @returns {string} */
export function readRustConstructedStringStatement(){return 'retained owned String and immutable String/str interfaces compose with existing complete scalar string concatenation, textual formatting, prefix/suffix/substring predicates and checked scalar calls; actual ordered captures and source owners remain explicit; primitive Display conversions, general formatting traits, borrowed outputs and allocation identity retain separate obligations';}
