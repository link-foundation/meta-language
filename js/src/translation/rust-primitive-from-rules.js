import {readRustPrimitiveAssociatedOwner} from './rust-primitive-associated-call-rules.js';
import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** @param {string} owner @param {string} method @returns {boolean} */
export function acceptRustPrimitiveFromMethod(owner,method){return method==='from'&&['bool','i8','u8','i16','u16','i32','u32','i64','u64','i128','u128','isize','usize','f64'].includes(owner);}
/** @param {string[]} segments @param {boolean} ordinary @param {boolean} shadowed
 * @param {boolean} absolute @returns {string} */
export function readRustPrimitiveFromOwner(segments,ordinary,shadowed,absolute){
 if(!ordinary||shadowed)return '';
 if(segments.length===2&&!absolute&&segments[0]==='f64'||segments.length===4&&(segments[0]==='std'||segments[0]==='core')&&segments[1]==='primitive'&&segments[2]==='f64')return 'f64';
 return readRustPrimitiveAssociatedOwner(segments,ordinary,shadowed,absolute);
}
/** Unconstrained integer syntax uses Rust's ordinary i32 fallback, never the
 * conversion destination as an invented argument context.
 * @param {string} form @param {boolean} typed @param {string} kind @returns {string} */
export function readRustPrimitiveFromLiteralOwner(form,typed,kind){
 if(form!=='numeral'&&form!=='negative numeral')return '';
 if(kind==='float')return 'f64';
 return !typed?'i32':'';
}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustPrimitiveFromSource(kind,bits,signed,owner,path,children,proven,layers){return kind==='float'&&bits===''&&!signed&&owner==='primitive'&&path==='f64'&&children===0&&proven&&layers===0||acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
/** Exact standard From relations, including portable pointer implementations.
 * Width equality alone does not establish that a trait implementation exists.
 * @param {string} target @param {string} source @returns {string} */
export function planRustPrimitiveFrom(target,source){
 if(!acceptRustPrimitiveFromMethod(target,'from')||!acceptRustPrimitiveFromMethod(source,'from'))return '';
 if(target===source)return 'identity';
 if(source==='bool'&&target!=='bool'&&target!=='f64')return 'boolean integer';
 if(target==='f64'&&['i8','i16','i32','u8','u16','u32'].includes(source))return 'floating exact';
 const admitted=source==='u8'&&['u16','u32','u64','u128','usize','i16','i32','i64','i128','isize'].includes(target)
  ||source==='u16'&&['u32','u64','u128','usize','i32','i64','i128'].includes(target)
  ||source==='u32'&&['u64','u128','i64','i128'].includes(target)
  ||source==='u64'&&['u128','i128'].includes(target)
  ||source==='i8'&&['i16','i32','i64','i128','isize'].includes(target)
  ||source==='i16'&&['i32','i64','i128','isize'].includes(target)
  ||source==='i32'&&['i64','i128'].includes(target)
  ||source==='i64'&&target==='i128';
 return admitted?'integer exact':'';
}
