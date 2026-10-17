// Closed accesses to the dense primitive columns produced by fact transport.
// The source checks the actual selected column; correlated columns do not
// establish a bounds proof. Missing data aborts instead of inventing a value.

/** @returns {number} */
function refuseRequiredNumberFact() {
  throw new RangeError('missing required Number fact column value');
}

/** @returns {string} */
function refuseRequiredTextFact() {
  throw new RangeError('missing required text fact column value');
}

/** @returns {boolean} */
function refuseRequiredBooleanFact() {
  throw new RangeError('missing required Boolean fact column value');
}

/** @param {number[]} values @param {number} index @returns {number} */
export function readRequiredNumberFact(values, index) {
  if (index < 0 || index % 1 !== 0 || index >= values.length) {
    throw new RangeError('invalid required Number fact column position');
  }
  return values[index] ?? refuseRequiredNumberFact();
}

/** @param {string[]} values @param {number} index @returns {string} */
export function readRequiredTextFact(values, index) {
  if (index < 0 || index % 1 !== 0 || index >= values.length) {
    throw new RangeError('invalid required text fact column position');
  }
  return values[index] ?? refuseRequiredTextFact();
}

/** @param {boolean[]} values @param {number} index @returns {boolean} */
export function readRequiredBooleanFact(values, index) {
  if (index < 0 || index % 1 !== 0 || index >= values.length) {
    throw new RangeError('invalid required Boolean fact column position');
  }
  return values[index] ?? refuseRequiredBooleanFact();
}
