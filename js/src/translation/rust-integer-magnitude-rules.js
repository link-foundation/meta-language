import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** @param {string} method @param {boolean} ordinary @returns {number} */
export function readRustIntegerMagnitudeArity(method,ordinary){return !ordinary?-1:method==='unsigned_abs'?0:method==='abs_diff'?1:-1;}
/** @param {string} method @param {string} kind @param {string} bits
 * @param {boolean} signed @param {string} owner @param {string} path
 * @param {number} children @param {boolean} proven @param {number} layers
 * @returns {boolean} */
export function acceptRustIntegerMagnitudeReceiver(method,kind,bits,signed,owner,path,children,proven,layers){return readRustIntegerMagnitudeArity(method,true)>=0&&(method!=='unsigned_abs'||signed)&&kind==='fixed'&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
/** @param {string} receiver @returns {string} */
export function readRustIntegerMagnitudeResultOwner(receiver){return receiver==='isize'?'usize':['i8','i16','i32','i64','i128'].includes(receiver)?'u'+receiver.slice(1):['u8','u16','u32','u64','u128','usize'].includes(receiver)?receiver:'';}
/** Signed negative magnitude first adds one, then negates a representable value,
 * casts that nonnegative value, then adds one in the unsigned domain. Absolute
 * difference orders operands in their actual source domain and uses a guarded
 * unsigned modular subtraction; its result always fits the complete uN range.
 * No signed minimum is negated and no wider source/host integer is needed.
 * @param {string} method @returns {string[]} */
export function planRustIntegerMagnitude(method){
 if(method==='unsigned_abs')return ['operand','zero','lt','operand','positive unit','add','neg','unsigned cast','unsigned unit','add','operand','unsigned cast','if'];
 if(method!=='abs_diff')return [];
 const forward=['unsigned operand','unsigned right','lt','unsigned maximum','unsigned right','unsigned operand','sub','sub','unsigned unit','add','unsigned operand','unsigned right','sub','if'];
 const backward=['unsigned right','unsigned operand','lt','unsigned maximum','unsigned operand','unsigned right','sub','sub','unsigned unit','add','unsigned right','unsigned operand','sub','if'];
 return ['operand','right','ge'].concat(forward,backward,['if']);
}
