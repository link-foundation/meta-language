// Byte characters have the actual Rust u8 type, not an arbitrary numeric type.
// https://doc.rust-lang.org/reference/patterns.html#literal-patterns
/** @param {string} token @returns {boolean} */
export function acceptRustSourceBytePatternToken(token){return token==='byteChar';}
/** @param {string} kind @param {number} bits @param {boolean} signed @returns {string} */
export function readRustSourceBytePatternTypeError(kind,bits,signed){return kind==='fixed'&&bits===8&&!signed?'':'a byte character pattern requires an unsigned 8-bit scrutinee';}
/** @param {string} form @returns {boolean} */
export function acceptRustSourceLiteralPatternAlias(form){return form==='strLit'||form==='byteLit';}

/** @param {string} language @param {string} kind @param {boolean} typed
 * @param {string[]} patterns @returns {boolean} */
export function acceptRustSourceByteLiteralInference(language,kind,typed,patterns){
 if(language!=='Rust'||kind!=='num'||typed)return false;
 for(const pattern of patterns)if(pattern==='byteLit')return true;
 return false;
}
