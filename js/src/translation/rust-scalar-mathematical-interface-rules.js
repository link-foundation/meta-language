// An additive interpretation; the original scalar model keeps its exact scope.
import {acceptRustScalarSourceExpression,acceptRustScalarSourceExpressionField} from './rust-scalar-source-interface-rules.js';
import {readRustPrimitiveBitwiseArity} from './rust-primitive-bitwise-rules.js';
/** @param {string} model @returns {boolean} */
export function acceptRustScalarMathematicalInterfaceModel(model){return model==='retained mathematical Rust function interface 1';}
/** @param {string} kind @returns {boolean} */
export function acceptRustScalarMathematicalExpression(kind){return kind==='math'||acceptRustScalarSourceExpression(kind);}
/** @param {string} kind @param {string} field @returns {boolean} */
export function acceptRustScalarMathematicalExpressionField(kind,field){return kind==='math'?(field==='k'||field==='type'||field==='span'||field==='op'||field==='args'):acceptRustScalarSourceExpressionField(kind,field);}
/** The core checker still checks every actual operand and source integer owner.
 * @param {string} operation @param {number} argumentsCount @param {string} result @returns {boolean} */
export function acceptRustScalarMathematicalOperation(operation,argumentsCount,result){
 const arity=readRustPrimitiveBitwiseArity(operation);
 if(arity>0)return argumentsCount===arity&&(result==='bool'||result==='fixed');
 if(operation==='abs'||operation==='floor'||operation==='ceil'||operation==='trunc')return argumentsCount===1&&result==='float';
 return (operation==='isNaN'||operation==='isFinite')&&argumentsCount===1&&result==='bool';
}
/** @returns {string} */
export function readRustScalarMathematicalEncoding(){return 'retained-mathematical-source-interface';}
/** @returns {string} */
export function readRustScalarMathematicalStatement(){return 'retained version-two scalar source headers and checked closed calls compose with primitive Rust bit operations and the admitted binary64 absolute, floor, ceiling, truncation and predicate operations; every operand remains a checked scalar and earlier interface models retain their scope';}
