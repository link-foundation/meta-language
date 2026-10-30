// Rust panic values use the existing intrinsic and output-effect containers.
/** @returns {string} */
export function readRustPanicValueOperation(){return 'raiseRustPanicValue';}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptLinkedRustPanicValue(kind,operation){return kind==='math'&&operation===readRustPanicValueOperation();}
/** @param {string} language @param {number} count @param {boolean} spread
 * @param {string} kind @returns {string} */
export function readRustPanicValueArgumentError(language,count,spread,kind){return language==='Rust'&&count===1&&!spread&&kind==='string'?'':'a Rust panic value needs one checked source String message';}
/** Formal targets carry the original message through the existing output sum.
 * Direct executable targets evaluate the actual message exactly once.
 * @param {string} target @param {string} message @returns {string} */
export function renderRustPanicValueExpression(target,message){return target==='JavaScript'?'((ml_message)=>{throw new Error(ml_message);})('+message+')':target==='Rust'?'panic!("{}", '+message+')':'';}
