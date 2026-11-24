// Actual literal suffixes constrain matching; equal representations do not
// establish equal Rust source types (notably usize/u64 and isize/i64).
import {acceptRustSourceSignaturePrimitive} from './rust-source-signature-rules.js';
/** @param {string} suffix @param {boolean} floating @returns {string} */
export function readRustSourceNumericPatternOwner(suffix,floating){return suffix!==''?suffix:floating?'f64':'';}
/** @param {string} suffix @param {boolean} floating @returns {string} */
export function readRustSourceNumericPatternLexicalError(suffix,floating){return floating&&suffix!==''&&suffix!=='f64'&&suffix!=='f32'?'a floating literal pattern requires a floating suffix':'';}
/** @param {string} actual @param {string} expected @param {boolean} typed
 * @param {boolean} collecting @returns {string} */
export function readRustSourceNumericPatternOwnerError(actual,expected,typed,collecting){
 return collecting||actual===expected||actual===''&&!typed?'':'a suffixed pattern requires the actual matching primitive source owner';
}
/** @param {string} owner @param {string} kind @param {string} bits
 * @param {boolean} signed @returns {string} */
export function readRustSourceNumericPatternTypeError(owner,kind,bits,signed){
 return (kind==='fixed'||kind==='float')&&acceptRustSourceSignaturePrimitive(kind,owner,bits,signed)?'':'a numeric pattern type must match its actual primitive suffix';
}
/** @param {string} owner @param {boolean} negative @returns {string} */
export function readRustSourceNumericPatternNegationError(owner,negative){
 return negative&&['u8','u16','u32','u64','u128','usize'].includes(owner)?'unsigned numeric patterns cannot use negation':'';
}
/** @param {string} language @param {string} kind @param {boolean} typed
 * @param {string[]} owners @returns {string} */
export function readRustSourceNumericPatternInference(language,kind,typed,owners){
 if(language!=='Rust'||kind!=='num'||typed)return '';
 let selected='';
 for(const owner of owners){if(owner==='')continue;if(selected!==''&&selected!==owner)return '';selected=owner;}
 return selected==='f64'?'':selected;
}
/** @param {string} language @param {string} kind @param {boolean} typed
 * @param {string[]} owners @returns {string} */
export function readRustSourceNumericPatternInferenceError(language,kind,typed,owners){
 if(language==='Rust'&&kind==='num'&&!typed)for(const owner of owners)if(owner==='f64')return 'an unsuffixed integer scrutinee cannot acquire a floating pattern type';
 return '';
}
/** @param {string} language @param {string} kind @returns {string} */
export function readRustSourceIntegerPatternContextError(language,kind){return language==='Rust'&&kind==='float'?'an integer literal pattern requires an integer scrutinee':'';}
/** @param {string} form @returns {boolean} */
export function acceptRustSourceNumericPatternAlias(form){return form==='typedNumLit';}
