import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** Resolve only actual primitive paths; lexical resolution stays in the host.
 * @param {string[]} segments @param {boolean} ordinary
 * @param {boolean} shadowed @param {boolean} absolute @returns {string} */
export function readRustPrimitiveAssociatedOwner(segments,ordinary,shadowed,absolute){
 if(!ordinary||shadowed)return '';
 let owner='';
 if(segments.length===2&&!absolute)owner=segments[0]??'';
 else if(segments.length===4&&(segments[0]==='std'||segments[0]==='core')&&segments[1]==='primitive')owner=segments[2]??'';
 return ['bool','i8','u8','i16','u16','i32','u32','i64','u64','i128','u128','isize','usize'].includes(owner)?owner:'';
}
/** Full call arity includes the explicit receiver. The existing method rule
 * remains responsible for its exact source interpretation.
 * @param {string} owner @param {string} method @returns {number} */
export function readRustPrimitiveAssociatedArity(owner,method){
 if(!['bool','i8','u8','i16','u16','i32','u32','i64','u64','i128','u128','isize','usize'].includes(owner))return -1;
 if(method==='min'||method==='max')return 2;
 if(owner==='bool')return -1;
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
