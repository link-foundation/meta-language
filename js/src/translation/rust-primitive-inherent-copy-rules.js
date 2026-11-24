import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
import {readRustPrimitiveAssociatedArity} from './rust-primitive-associated-call-rules.js';
/** Only actual immutable reference chains to standard Copy integer primitives.
 * Ord methods are excluded: reference Self can change their result type.
 * Explicit associated-call arguments never use method receiver adjustment.
 * @param {string} method @param {string} kind @param {string} bits
 * @param {boolean} signed @param {string} owner @param {string} path
 * @param {number} children @param {boolean} proven @param {number} layers
 * @param {boolean} referencesOnly @param {boolean} copyEvidence
 * @returns {boolean} */
export function acceptRustPrimitiveInherentCopy(method,kind,bits,signed,owner,path,children,proven,layers,referencesOnly,copyEvidence){
 return method!=='min'&&method!=='max'&&readRustPrimitiveAssociatedArity(path,method)>0&&kind==='fixed'&&layers>=1&&referencesOnly&&copyEvidence&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,0);
}
