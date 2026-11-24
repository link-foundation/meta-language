// Qualified evaluation extension; the existing pure-query decisions remain intact.
/** Zero refuses an unknown type, one retains a pure constant query, two
 * evaluates a represented operand, and three evaluates JavaScript undefined.
 * @param {boolean} javaScript @param {boolean} known @param {boolean} pureSource
 * @param {boolean} pureValue @param {boolean} undefinedValue @returns {number} */
export function readTypeQueryEvaluationPlan(javaScript, known, pureSource, pureValue, undefinedValue) {
  if (javaScript && undefinedValue) return 3;
  if (!known) return 0;
  if (pureSource && pureValue) return 1;
  return javaScript ? 2 : 0;
}
