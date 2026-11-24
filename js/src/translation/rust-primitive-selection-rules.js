import {acceptRustSourceSignaturePrimitive} from './rust-source-signature-rules.js';
/** @param {string} path @param {boolean} ordinary @param {boolean} shadowed
 * @returns {string} */
export function readRustPrimitiveSelectionCallRole(path,ordinary,shadowed){
 return ordinary&&!shadowed?(path==='std::cmp::min'?'min':path==='std::cmp::max'?'max':''):'';
}
/** @param {string} method @param {boolean} ordinary @returns {string} */
export function readRustPrimitiveSelectionComparison(method,ordinary){return ordinary?(method==='min'?'le':method==='max'?'gt':''):'';}
/** The ordinary Boolean order is false before true; use Boolean operations
 * rather than adding an ordering interpretation to the common Boolean domain.
 * @param {string} method @param {string} kind @returns {string} */
export function readRustPrimitiveSelectionCondition(method,kind){
 if(kind==='bool')return method==='min'?'not':method==='max'?'identity':'';
 return kind==='fixed'?readRustPrimitiveSelectionComparison(method,true):'';
}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers){return (kind==='fixed'||kind==='bool')&&owner==='primitive'&&children===0&&proven&&layers===0&&acceptRustSourceSignaturePrimitive(kind,path,bits,signed);}
/** Untyped integer syntax gets the actual primitive receiver's context. All
 * other expressions require their complete independent matching source owner.
 * @param {string} expected @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} actual @param {string} actualBits
 * @param {boolean} actualSigned @param {string} actualOwner
 * @param {number} layers @param {boolean} proven @param {boolean} known
 * @param {boolean} literal @returns {boolean} */
export function acceptRustPrimitiveSelectionArgument(expected,bits,signed,owner,actual,actualBits,actualSigned,actualOwner,layers,proven,known,literal){
 if(layers!==0)return false;
 if(literal&&expected==='fixed'&&actual===''&&actualOwner==='')return true;
 return known&&proven&&expected===actual&&bits===actualBits&&signed===actualSigned&&owner===actualOwner;
}
