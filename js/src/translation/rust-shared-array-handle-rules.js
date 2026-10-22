// Finite source admission. Native semantics are generated from this entire module.
import {acceptRustSourceArrayElement} from './rust-array-rules.js';
import {acceptReferenceIdentityComparison} from './reference-identity-rules.js';
/** @param {string} model @returns {boolean} */
export function acceptRustSharedArrayHandleSourceModel(model) {
  return model === '' || model === 'standard Rust shared array identity handles 1';
}
/** @param {number} units @returns {boolean} */
export function acceptRustSharedArrayHandleSourceSize(units) {
  return units >= 0 && units <= 65536 && units % 1 === 0;
}
/** @param {string} role @param {string} owner @param {boolean} shadowed @returns {boolean} */
export function acceptRustSharedArrayHandleLayer(role, owner, shadowed) {
  return !shadowed && (role === 'handle' && owner === 'std::rc::Rc'
    || role === 'cell' && owner === 'std::cell::RefCell');
}
/** @param {string} kind @param {boolean} indirected @returns {boolean} */
export function acceptRustSharedArrayHandlePayload(kind, indirected) {
  return acceptRustSourceArrayElement(kind, indirected);
}
/** @param {string} actual @param {string} expected @param {number} actualLayers @param {number} expectedLayers @returns {boolean} */
export function acceptRustSharedArrayHandleTransfer(actual, expected, actualLayers, expectedLayers) {
  return actual === 'standard shared array handle' && expected === actual && actualLayers === 0 && expectedLayers === 0;
}
/** @param {string} operation @param {number} arity
 * @param {boolean} owner @param {boolean} leftHandle @param {boolean} rightHandle
 * @param {boolean} borrowed @param {boolean} sameDomain @param {boolean} freshLiteral
 * @returns {string} */
export function readRustSharedArrayHandleSourceForm(operation, arity, owner, leftHandle, rightHandle, borrowed, sameDomain, freshLiteral) {
  if (!owner) return '';
  if (operation === 'allocate' && arity === 1 && freshLiteral) return 'allocation';
  if (operation === 'clone' && arity === 0 && leftHandle && !borrowed) return 'alias';
  if (operation === 'pointer equality' && arity === 2 && borrowed
    && acceptReferenceIdentityComparison('eq', 'array', 'array', leftHandle, rightHandle, sameDomain)) return 'identity equality';
  return '';
}
/** Rust ordinary equality and Display observe payloads; this profile has only
 * explicit pointer identity, not a substitute payload operation.
 * @param {boolean} leftHandle @param {boolean} rightHandle @returns {boolean} */
export function acceptRustSharedArrayHandleValueObservation(leftHandle, rightHandle) {
  return !leftHandle && !rightHandle;
}
/** The explicit canonical domain owns identity semantics after source text is
 * gone. Plain Rust binary syntax never selects this Surface operation.
 * @param {string} language @param {string} operation @param {string} kind
 * @param {boolean} reference @returns {string} */
export function readRustSharedArrayHandleComparisonProjection(language, operation, kind, reference) {
  if (language === 'Rust' && kind === 'array' && reference && (operation === 'eq' || operation === 'ne')) return 'referenceIdentity';
  return '';
}
