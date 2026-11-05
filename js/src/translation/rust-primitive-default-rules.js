import {readRustPrimitiveFromOwner} from './rust-primitive-from-rules.js';
/** @param {string[]} segments @param {boolean} ordinary @param {boolean} shadowed
 * @param {boolean} absolute @returns {string} */
export function readRustPrimitiveDefaultOwner(segments,ordinary,shadowed,absolute){return segments[segments.length-1]==='default'?readRustPrimitiveFromOwner(segments,ordinary,shadowed,absolute):'';}
/** Every admitted primitive has its actual standard Default implementation.
 * Positive floating zero is distinct from negative zero and NaN.
 * @param {string} owner @returns {string} */
export function readRustPrimitiveDefaultTerm(owner){return owner==='bool'?'false':owner==='f64'?'positive floating zero':['i8','u8','i16','u16','i32','u32','i64','u64','i128','u128','isize','usize'].includes(owner)?'integer zero':'';}
/** @param {string} path @param {boolean} ordinary @param {boolean} shadowed @returns {boolean} */
export function acceptRustPrimitiveDefaultImport(path,ordinary,shadowed){return ordinary&&!shadowed&&(path==='std::default::Default'||path==='core::default::Default');}
