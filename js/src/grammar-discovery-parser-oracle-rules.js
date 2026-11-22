/** @param {string} resource @returns {number} */
export function readGrammarDiscoveryParserOracleLimit(resource) {
  if (resource === 'queries') return 256;
  if (resource === 'symbols' || resource === 'nesting') return 128;
  if (resource === 'steps') return 50000;
  if (resource === 'memo') return 4096;
  if (resource === 'memory') return 8192;
  return 0;
}

/** @param {boolean} accepted @param {string} rejection @returns {boolean} */
export function acceptGrammarDiscoveryParserOracleAnswer(accepted, rejection) {
  return accepted || rejection === 'syntax';
}

/** @param {number} expected @param {boolean} accepted @returns {boolean} */
export function acceptGrammarDiscoveryParserOracleLabel(expected, accepted) {
  return expected === -1 || expected === (accepted ? 1 : 0);
}

/** @param {boolean} declared @param {boolean} accepted @returns {number} */
export function readGrammarDiscoveryParserOracleTrainingClass(declared, accepted) {
  if (declared) return 0;
  return accepted ? 1 : -1;
}
