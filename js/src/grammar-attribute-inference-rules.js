// The finite search space is data in the same language as grammar actions.
// Candidates are tested by native grammar execution on the instrumented inputs.

/** @returns {number} */
export function readGrammarAttributeObservationLimit() {
  return 32;
}

/** @returns {number} */
export function readGrammarAttributeObservationByteLimit() {
  return 262144;
}

/**
 * @param {string} field Percent-encoded field, or empty when absent.
 * @param {string} childAttribute Percent-encoded attribute, or empty when absent.
 * @returns {string[]}
 */
export function readGrammarAttributeCandidateValues(field, childAttribute) {
  const matched = '(matched)';
  const candidates = [matched, '(uppercase ' + matched + ')', '(length ' + matched + ')', '(number ' + matched + ')'];
  if (field === '') return candidates;
  const captured = '(fieldText ' + field + ')';
  const projections = [captured, '(uppercase ' + captured + ')', '(length ' + captured + ')', '(number ' + captured + ')'];
  if (childAttribute === '') return candidates.concat(projections);
  return candidates.concat(projections, ['(attribute ' + field + ' ' + childAttribute + ')', '(sumOf ' + field + ' ' + childAttribute + ')']);
}
