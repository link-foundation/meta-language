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

/** @param {string} name @param {boolean} ordinary @returns {boolean} */
export function acceptRustPanicFormattingMacro(name,ordinary){return ordinary&&(name==='panic'||name==='unreachable'||name==='unimplemented'||name==='todo');}
/** Braces are raw literal facts; the shared formatter owns their grammar.
 * @param {string} name @param {boolean} ordinary @param {string} kind
 * @param {number} count @param {boolean} braces @returns {boolean} */
export function acceptRustPanicFormattingValue(name,ordinary,kind,count,braces){return acceptRustPanicFormattingMacro(name,ordinary)&&count>0&&(count>1||kind==='str'&&braces);}
/** @param {string} name @returns {string} */
export function readRustPanicFormattingPrefix(name){return name==='unreachable'?'internal error: entered unreachable code: ':name==='unimplemented'?'not implemented: ':name==='todo'?'not yet implemented: ':'';}
/** @param {string} kind @param {string} operation @returns {string} */
export function readRustPanicCompletionKind(kind,operation){return acceptLinkedRustPanicValue(kind,operation)?'abort':kind;}

/** Shared actual str argument contract for Option/Result expect.
 * @param {string} consumer @param {string} kind @param {string} storage
 * @param {number} layers @param {boolean} known @param {boolean} references
 * @returns {string} */
export function readRustPanicMessageSourceError(consumer,kind,storage,layers,known,references){return consumer!=='expect'||kind==='string'&&(storage==='string slice'||storage==='owned string')&&layers>=1&&known&&references?'':'expect needs its actual immutable standard str message source';}

/** @param {string} consumer @returns {boolean} */
export function testRustOptionPanicConsumer(consumer){return consumer==='unwrap'||consumer==='expect';}
/** Option expect uses the actual message without formatting or Debug suffixes.
 * @param {string} consumer @param {string} message @returns {string} */
export function readRustOptionPanicMessage(consumer,message){return consumer==='unwrap'?'called `Option::unwrap()` on a `None` value':consumer==='expect'?message:'';}
