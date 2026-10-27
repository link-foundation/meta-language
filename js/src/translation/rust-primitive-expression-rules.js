import {acceptRustPrimitiveFromSource,acceptRustPrimitiveFromMethod} from './rust-primitive-from-rules.js';
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustPrimitiveArithmeticOrigin(kind,bits,signed,owner,path,children,proven,layers){return (kind==='fixed'||kind==='float')&&acceptRustPrimitiveFromMethod(path,'from')&&acceptRustPrimitiveFromSource(kind,bits,signed,owner,path,children,proven,layers);}
/** Only literal syntax receives the known peer's operator context.
 * @param {string} owner @param {boolean} signed @param {string} literal
 * @returns {boolean} */
export function acceptRustPrimitiveArithmeticLiteral(owner,signed,literal){
 if(owner==='f64')return literal==='floating numeral'||literal==='negative floating numeral';
 return literal==='integer numeral'||signed&&literal==='negative integer numeral';
}
/** Return the actual operand that establishes the result primitive identity.
 * @param {string} operation @param {boolean} leftKnown @param {boolean} rightKnown
 * @param {string} leftOwner @param {string} rightOwner @param {boolean} leftSigned
 * @param {boolean} rightSigned @param {string} leftLiteral @param {string} rightLiteral
 * @returns {number} */
export function selectRustPrimitiveArithmeticOrigin(operation,leftKnown,rightKnown,leftOwner,rightOwner,leftSigned,rightSigned,leftLiteral,rightLiteral){
 if(operation==='neg')return leftKnown&&(leftSigned||leftOwner==='f64')?0:-1;
 if(!['add','sub','mul','div','rem'].includes(operation))return -1;
 if(leftKnown&&rightKnown)return leftOwner===rightOwner?0:-1;
 if(leftKnown&&acceptRustPrimitiveArithmeticLiteral(leftOwner,leftSigned,rightLiteral))return 0;
 return rightKnown&&acceptRustPrimitiveArithmeticLiteral(rightOwner,rightSigned,leftLiteral)?1:-1;
}
/** @param {string} form @param {string} operation @param {boolean} booleanOperand
 * @returns {boolean} */
export function acceptRustPrimitiveBooleanResult(form,operation,booleanOperand){return form==='comparison'||form==='boolean-operation'||form==='primitive-operation'&&operation==='not'&&booleanOperand;}
