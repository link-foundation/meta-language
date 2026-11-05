// Additive retained interfaces over the existing complete scalar string values.
import {acceptRustScalarMathematicalExpression,acceptRustScalarMathematicalExpressionField} from './rust-scalar-mathematical-interface-rules.js';
/** @param {string} model @returns {boolean} */
export function acceptRustStringPredicateInterfaceModel(model){return model==='retained string predicate Rust function interface 1';}
/** @param {string} kind @returns {boolean} */
export function acceptRustStringPredicateExpression(kind){return kind==='stringTest'||acceptRustScalarMathematicalExpression(kind);}
/** @param {string} kind @param {string} field @returns {boolean} */
export function acceptRustStringPredicateExpressionField(kind,field){return kind==='stringTest'?(field==='k'||field==='type'||field==='span'||field==='op'||field==='string'||field==='search'):acceptRustScalarMathematicalExpressionField(kind,field);}
/** Both complete arguments remain Unicode scalar strings; no host coercion.
 * @param {string} operation @param {string} result @param {string} object @param {string} search @returns {boolean} */
export function acceptRustStringPredicateOperands(operation,result,object,search){return result==='bool'&&object==='string'&&search==='string'&&(operation==='startsWith'||operation==='endsWith'||operation==='includes');}
/** @returns {string} */
export function readRustStringPredicateEncoding(){return 'retained-string-predicate-source-interface';}
/** @returns {string} */
export function readRustStringPredicateStatement(){return 'retained owned standard String and immutable String/str source interfaces compose with exact prefix, suffix and substring predicates over complete Unicode scalar strings; original input owners, lifetimes, argument order, checked calls and nonescaping results remain explicit; character patterns, dynamic predicates and general string transformations require separate interpretations';}
