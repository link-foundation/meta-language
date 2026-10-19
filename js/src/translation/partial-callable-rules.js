// A contextual parameter list is not a checked function result type.
// The existing match checker retains that context and selects the actual
// checked branch signature when its expected return has not been supplied.

/** @param {string} kind @param {boolean} returnPresent @returns {boolean} */
export function requirePartialCallableResultInference(kind, returnPresent) {
  return kind === 'function' && !returnPresent;
}
