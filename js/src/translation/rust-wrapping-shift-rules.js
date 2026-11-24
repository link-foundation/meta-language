import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** @param {string} method @returns {boolean} */
export function acceptRustWrappingShiftMethod(method){return ['wrapping_shl','wrapping_shr','rotate_left','rotate_right'].includes(method);}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustWrappingShiftReceiver(kind,bits,signed,owner,path,children,proven,layers){return kind==='fixed'&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
/** Closed existing source operations; hosts only allocate their actual ASTs.
 * @param {string} term @returns {string} */
export function readRustWrappingShiftOperation(term){return term==='shift left'?'shiftRustPrimitiveBitsLeft':term==='shift right'?'shiftRustPrimitiveBitsRight':term==='bits or'?'disjoinRustPrimitiveBits':'';}
/** Standard counts retain u32 ownership. Both modulo counts are below Self's
 * width, including zero. Rotations use unsigned bit patterns and restore Self.
 * Actual input and count are captured before this pure source plan.
 * @param {string} method @returns {string[]} */
export function planRustWrappingShift(method){
 if(!acceptRustWrappingShiftMethod(method))return [];
 const count=['count','width','rem'];
 if(method==='wrapping_shl'||method==='wrapping_shr')return ['operand'].concat(count,[method==='wrapping_shl'?'shift left':'shift right']);
 const inverse=['width'].concat(count,['sub','width','rem']);
 return ['unsigned operand'].concat(count,[method==='rotate_left'?'shift left':'shift right','unsigned operand'],inverse,[method==='rotate_left'?'shift right':'shift left','bits or','source cast']);
}
