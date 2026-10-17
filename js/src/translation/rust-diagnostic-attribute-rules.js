// Generated from the shared diagnostic attribute token policy. Do not edit.
/** @param {number} value @returns {boolean} */
export function acceptRustDiagnosticAttributeOffset(value) {
  return Number.isInteger(value) && value >= 0 && value <= 9007199254740991;
}

/** Actual token start includes #. Result is the six policy-defined token positions; [-1] refuses.
 * @param {string[]} kinds @param {string[]} values @param {number} start @returns {number[]} */
export function planRustDiagnosticAttribute(kinds, values, start) {
  if (kinds.length !== values.length || !acceptRustDiagnosticAttributeOffset(start) || kinds.length - start < 13) return [-1];
  if ((kinds[start] ?? "") !== "punct" || (values[start] ?? "") !== "#") return [-1];
  if ((kinds[start + 1] ?? "") !== "punct" || (values[start + 1] ?? "") !== "[") return [-1];
  if ((kinds[start + 2] ?? "") !== "identifier" || (values[start + 2] ?? "") !== "expect") return [-1];
  if ((kinds[start + 3] ?? "") !== "punct" || (values[start + 3] ?? "") !== "(") return [-1];
  if ((kinds[start + 4] ?? "") !== "identifier" || (values[start + 4] ?? "") !== "clippy") return [-1];
  if ((kinds[start + 5] ?? "") !== "punct" || (values[start + 5] ?? "") !== "::") return [-1];
  if ((kinds[start + 6] ?? "") !== "identifier" || (values[start + 6] ?? "") !== "fn_params_excessive_bools") return [-1];
  if ((kinds[start + 7] ?? "") !== "punct" || (values[start + 7] ?? "") !== ",") return [-1];
  if ((kinds[start + 8] ?? "") !== "identifier" || (values[start + 8] ?? "") !== "reason") return [-1];
  if ((kinds[start + 9] ?? "") !== "punct" || (values[start + 9] ?? "") !== "=") return [-1];
  if ((kinds[start + 10] ?? "") !== "string" || typeof values[start + 10] !== "string") return [-1];
  let close = start + 11;
  if ((kinds[close] ?? "") === "punct" && (values[close] ?? "") === ",") close += 1;
  if (kinds.length - close < 2) return [-1];
  if ((kinds[close] ?? "") !== "punct" || (values[close] ?? "") !== ")") return [-1];
  if ((kinds[close + 1] ?? "") !== "punct" || (values[close + 1] ?? "") !== "]") return [-1];
  return [close + 2, start + 2, start + 4, start + 6, start + 8, start + 10];
}

/** Metadata-only recognition of the unchanged four-token legacy spelling.
 * @param {string[]} kinds @param {string[]} values @param {number} start @returns {number} */
export function readRustMustUseAttributeEnd(kinds, values, start) {
  if (kinds.length !== values.length || !acceptRustDiagnosticAttributeOffset(start) || kinds.length - start < 4) return -1;
  if ((kinds[start] ?? "") !== "punct" || (values[start] ?? "") !== "#") return -1;
  if ((kinds[start + 1] ?? "") !== "punct" || (values[start + 1] ?? "") !== "[") return -1;
  if ((kinds[start + 2] ?? "") !== "identifier" || (values[start + 2] ?? "") !== "must_use") return -1;
  if ((kinds[start + 3] ?? "") !== "punct" || (values[start + 3] ?? "") !== "]") return -1;
  return start + 4;
}

/** Complete pre-const token inventory, excluding EOF. The result has four policy-defined cells.
 * @param {string[]} kinds @param {string[]} values @returns {number[]} */
export function planRustDiagnosticAttributePrefix(kinds, values) {
  if (kinds.length !== values.length) return [-1];
  let index = 0;
  let attributes = 0;
  let diagnostics = 0;
  while (index < kinds.length) {
    if (index === kinds.length - 1 && (kinds[index] ?? "") === "identifier" && (values[index] ?? "") === "pub") {
      return diagnostics > 0 ? [kinds.length, attributes, diagnostics, index] : [-1];
    }
    const legacyEnd = readRustMustUseAttributeEnd(kinds, values, index);
    if (legacyEnd >= 0) {
      index = legacyEnd;
    } else {
      const plan = planRustDiagnosticAttribute(kinds, values, index);
      const next = plan[0] ?? -1;
      if (next <= index) return [-1];
      index = next;
      diagnostics += 1;
    }
    attributes += 1;
  }
  return diagnostics > 0 ? [index, attributes, diagnostics, -1] : [-1];
}
