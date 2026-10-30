/** @param {string} kind @returns {boolean} */
export function acceptRustScalarArrayIndexElement(kind){return kind==='bool'||kind==='float'||kind==='fixed'||kind==='unit';}
/** Only actual standard array storage and immutable dereference layers enter.
 * Unknown inferred element types need independently retained scalar Copy facts.
 * @param {string} kind @param {string} element @param {string} model
 * @param {boolean} proven @param {boolean} layers @param {boolean} inferredCopy @returns {string} */
export function readRustScalarArrayIndexReceiverError(kind,element,model,proven,layers,inferredCopy){
 return proven&&layers&&(model==='owned vector'||model==='immutable slice')
  &&(kind==='array'&&acceptRustScalarArrayIndexElement(element)||kind===''&&inferredCopy)?'':'the receiver needs a proven standard vector or slice of direct scalar Copy elements';
}
/** Actual usize owners differ from equal-width u64 source types.
 * @param {boolean} literal @param {string} kind @param {number} bits
 * @param {boolean} signed @param {string} owner @param {number} wrappers
 * @param {boolean} proven @returns {string} */
export function readRustScalarArrayIndexOperandError(literal,kind,bits,signed,owner,wrappers,proven){return literal||proven&&kind==='fixed'&&bits===64&&!signed&&owner==='usize'&&wrappers===0?'':'the index needs an unsuffixed integer literal or a proven direct usize source owner';}
