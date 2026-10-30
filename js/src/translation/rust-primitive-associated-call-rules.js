import {readRustPrimitiveOwnerPath} from './rust-primitive-owner-path-rules.js';
import {readRustIntegerMagnitudeArity} from './rust-integer-magnitude-rules.js';
import {acceptRustWrappingShiftMethod} from './rust-wrapping-shift-rules.js';
import {acceptRustIntegerBitMethod} from './rust-integer-bit-rules.js';
import {readRustCheckedIntegerArity} from './rust-checked-integer-rules.js';
import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** Resolve only actual primitive paths; lexical resolution stays in the host.
 * @param {string[]} segments @param {boolean} ordinary
 * @param {boolean} shadowed @param {boolean} absolute @returns {string} */
export function readRustPrimitiveAssociatedOwner(segments,ordinary,shadowed,absolute){return readRustPrimitiveOwnerPath(segments,ordinary,shadowed,absolute);}

/** Full call arity includes the explicit receiver. The existing method rule
 * remains responsible for its exact source interpretation.
 * @param {string} owner @param {string} method @returns {number} */
export function readRustPrimitiveAssociatedArity(owner,method){
 if(!['bool','i8','u8','i16','u16','i32','u32','i64','u64','i128','u128','isize','usize'].includes(owner))return -1;
 if(method==='min'||method==='max')return 2;
 if(owner==='bool')return -1;
 const magnitude=readRustIntegerMagnitudeArity(method,true);
 if(magnitude>=0&&(method!=='unsigned_abs'||owner.startsWith('i')))return magnitude+1;
 if(acceptRustWrappingShiftMethod(method))return 2;
 if(acceptRustIntegerBitMethod(method))return 1;
 const checked=readRustCheckedIntegerArity(method,true);
 if(checked>=0&&(method!=='checked_abs'||owner.startsWith('i')))return checked+1;
 if(['saturating_add','saturating_sub','saturating_mul','wrapping_add','wrapping_sub'].includes(method))return 2;
 if(method==='wrapping_neg')return 1;
 if(!owner.startsWith('i'))return -1;
 return ['is_negative','is_positive','signum','saturating_neg','saturating_abs','wrapping_abs'].includes(method)?1:-1;
}
/** @param {string} expected @param {string} form @param {boolean} typed
 * @returns {boolean} */
export function acceptRustPrimitiveAssociatedLiteral(expected,form,typed){return expected!==''&&expected!=='bool'&&!typed&&(form==='numeral'||form==='negative numeral');}
/** @param {string} expected @param {string} kind @param {string} bits
 * @param {boolean} signed @param {string} owner @param {string} path
 * @param {number} children @param {boolean} proven @param {number} layers
 * @returns {boolean} */
export function acceptRustPrimitiveAssociatedReceiver(expected,kind,bits,signed,owner,path,children,proven,layers){return expected===path&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
