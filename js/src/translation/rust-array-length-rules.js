/** Keep the existing explicit Number path and every nonordinary source profile.
 * @param {string} method @param {boolean} binary64 @param {boolean} ordinary
 * @returns {boolean} */
export function acceptRustExactArrayLength(method,binary64,ordinary){return method==='len'&&!binary64&&ordinary;}
/** Only actual bounded array counts have this exact source codomain.
 * @param {string} expression @param {string} result @param {number} bits
 * @param {boolean} signed @param {string} operand @returns {boolean} */
export function acceptRustExactArrayLengthResult(expression,result,bits,signed,operand){return expression==='length'&&result==='fixed'&&bits===64&&!signed&&operand==='array';}
/** @param {string} target @param {string} operand @returns {string} */
export function renderRustExactArrayLength(target,operand){
 if(target==='Rust')return '(('+operand+').len() as u64)';
 if(target==='JavaScript')return 'BigInt(('+operand+').length)';
 if(target==='Lean')return '('+operand+').size';
 return target==='Rocq'?'(N.of_nat (List.length '+operand+'))':'';
}
