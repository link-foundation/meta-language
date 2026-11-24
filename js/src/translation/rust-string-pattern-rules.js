// String literal patterns reuse the common literal-pattern algebra. Source
// ownership remains explicit; a core String value alone proves no Rust borrow.
// https://doc.rust-lang.org/reference/patterns.html#literal-patterns
/** @param {string} token @returns {boolean} */
export function acceptRustSourceStringPatternToken(token){return token==='string';}
/** @param {string} kind @param {string} model @param {boolean} proven
 * @param {number} layers @param {boolean} references @param {string} ownerKind
 * @param {string} ownerPath @param {number} argumentsCount
 * @param {boolean} collecting @returns {string} */
export function readRustSourceStringPatternError(kind,model,proven,layers,references,ownerKind,ownerPath,argumentsCount,collecting){
 if(collecting&&!proven)return '';
 return proven&&kind==='string'&&model==='string slice'&&Number.isInteger(layers)&&layers>0&&references&&ownerKind==='primitive'&&ownerPath==='str'&&argumentsCount===0?'':'a literal pattern requires its proven immutable borrowed str source owner';
}
