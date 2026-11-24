/** Complete shared source terms, never represented byte evaluation.
 * @param {string} method @returns {string[]} */
export function planRustByteAscii(method){
 const ranges=method==='is_ascii'?[0,127]:method==='is_ascii_alphabetic'?[65,90,97,122]:method==='is_ascii_alphanumeric'?[48,57,65,90,97,122]:method==='is_ascii_control'?[0,31,127,127]:method==='is_ascii_digit'?[48,57]:method==='is_ascii_graphic'?[33,126]:method==='is_ascii_hexdigit'?[48,57,65,70,97,102]:method==='is_ascii_lowercase'?[97,122]:method==='is_ascii_punctuation'?[33,47,58,64,91,96,123,126]:method==='is_ascii_uppercase'?[65,90]:method==='is_ascii_whitespace'?[9,10,12,13,32,32]:[];
 let terms=[];for(let at=0;at<ranges.length;at+=2){terms=terms.concat(['operand',String(ranges[at]??0),'ge','operand',String(ranges[at+1]??0),'le','and']);if(at>0)terms=terms.concat(['or']);}return terms;
}
/** @param {string} method @param {boolean} ordinary @returns {number} */
export function readRustByteAsciiArity(method,ordinary){return ordinary&&planRustByteAscii(method).length>0?0:-1;}
/** Only standard u8 uses these byte predicates. Its actual immutable receiver
 * dereferences may be read without moving another source domain.
 * @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} ownerKind @param {string} owner @param {number} children
 * @param {boolean} proven @param {boolean} immutable @returns {string} */
export function readRustByteAsciiReceiverError(kind,bits,signed,ownerKind,owner,children,proven,immutable){return kind==='fixed'&&bits==='8'&&!signed&&ownerKind==='primitive'&&owner==='u8'&&children===0&&proven&&immutable?'':'a byte predicate needs an actual standard u8 receiver with immutable source layers';}
