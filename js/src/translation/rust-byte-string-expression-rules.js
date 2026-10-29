// Canonical lexical byte payload to its actual immutable static array value.
// Rust Reference expr.literal.byte-string.type: &'static [u8; N].
/** Packet [1,...bytes], [-1] suffix, [-2] invalid lexical transport.
 * @param {string} payload @param {string} suffix @returns {number[]} */
export function readRustByteStringExpressionBytes(payload,suffix){
 if(suffix!=='')return [-1];if(payload.length%2!==0)return [-2];
 const bytes=[1];for(let at=0;at<payload.length;at+=2){const high='0123456789abcdef'.indexOf(payload.charAt(at)),low='0123456789abcdef'.indexOf(payload.charAt(at+1));if(high<0||low<0)return [-2];bytes.push(high*16+low);}return bytes;
}
/** @param {number} code @returns {string} */
export function readRustByteStringExpressionError(code){return code===-1?'a Rust byte string expression cannot have an identifier suffix':'invalid canonical Rust byte string payload';}
/** @param {string} actual @param {string} expected @param {boolean} borrowed @returns {boolean} */
export function acceptRustStaticByteArrayCoercion(actual,expected,borrowed){return actual==='static byte array'&&expected==='immutable slice'&&borrowed;}
/** Keep source array lengths distinct before an explicit slice view.
 * @param {string} model @param {string} leftKind @param {string} leftPath
 * @param {string} rightKind @param {string} rightPath @returns {boolean} */
export function acceptRustStaticByteArrayOwnerEquality(model,leftKind,leftPath,rightKind,rightPath){return model!=='static byte array'||leftKind==='array'&&rightKind==='array'&&leftPath===rightPath;}
/** @param {string} method @param {number} count @param {string} model
 * @param {boolean} proven @param {number} layers @param {boolean} references
 * @returns {boolean} */
export function acceptRustStaticByteArraySliceView(method,count,model,proven,layers,references){return method==='as_slice'&&count===0&&model==='static byte array'&&proven&&layers>0&&references;}
/** Standard immutable element reads have the same array/slice bounds meaning.
 * The source owner and exact fixed length remain unchanged on the operand.
 * @param {string} model @returns {string} */
export function readRustStaticByteArrayReadModel(model){return model==='static byte array'?'immutable slice':model;}
