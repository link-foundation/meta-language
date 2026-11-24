// Exact primitive Display composition over the existing complete string graph.
import {acceptRustStringPredicateExpression,acceptRustStringPredicateExpressionField} from './rust-string-predicate-interface-rules.js';
/** @param {string} model @returns {boolean} */
export function acceptRustDisplayedStringInterfaceModel(model){return model==='retained displayed Rust string function interface 1';}
/** @param {string} kind @returns {boolean} */
export function acceptRustDisplayedStringExpression(kind){return kind==='toString'||acceptRustStringPredicateExpression(kind);}
/** @param {string} kind @param {string} field @returns {boolean} */
export function acceptRustDisplayedStringExpressionField(kind,field){return kind==='toString'?(field==='k'||field==='type'||field==='span'||field==='arg'||field==='console'):acceptRustStringPredicateExpressionField(kind,field);}
/** The common checker retains the complete fixed-width type and exact value.
 * @param {string} kind @param {string} result @param {boolean} console @returns {boolean} */
export function acceptRustPrimitiveDisplayOperand(kind,result,console){return (kind==='fixed'||kind==='bool')&&result==='string'&&!console;}
/** @returns {string} */
export function readRustDisplayedStringEncoding(){return 'retained-displayed-string-source-interface';}
/** @returns {string} */
export function readRustDisplayedStringStatement(){return 'retained String/str interfaces compose with exact decimal fixed-width integer and lowercase Boolean Display through the existing complete primitive string conversion; original widths, signedness, source owners, lifetimes, captured argument order and checked calls remain; floating-point formatting, custom Display, console representations and format specifiers retain separate obligations';}
