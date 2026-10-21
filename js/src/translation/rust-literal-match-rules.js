// Closed standard matches! projection. Patterns outside this literal-only
// domain keep their own obligations; no source pattern is treated as opaque.
// https://doc.rust-lang.org/std/macro.matches.html
// https://doc.rust-lang.org/reference/patterns.html#literal-patterns

/** @param {string} kind @param {string} textModel @param {boolean} proven
 * @param {string} wrappers @param {boolean} shadowed @returns {boolean} */
export function acceptRustLiteralMatchReceiver(kind, textModel, proven, wrappers, shadowed) {
  return proven && !shadowed && kind === 'string' && textModel === 'string slice'
    && wrappers === 'reference';
}

/** @param {string} kind @returns {boolean} */
export function acceptRustLiteralMatchCatalogOwner(kind) {
  return kind === 'data';
}

/** The first cell is the exclusive end, followed by original literal token
 * positions in pattern order. An empty result refuses the complete pattern.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string} close @returns {number[]} */
export function planRustLiteralMatchPattern(kinds, values, start, close) {
  if (kinds.length !== values.length || !Number.isInteger(start) || start < 0
    || start >= kinds.length || (close !== ')' && close !== ']' && close !== '}')) return [];
  let at = start;
  /** @type {number[]} */
  const positions = [];
  if (kinds[at] === 'punct' && values[at] === '|') at += 1;
  while (at < kinds.length) {
    if (kinds[at] !== 'string' || typeof values[at] !== 'string') return [];
    positions.push(at);
    at += 1;
    if (kinds[at] === 'punct' && values[at] === close) return [at + 1, ...positions];
    if (kinds[at] === 'punct' && values[at] === ','
      && kinds[at + 1] === 'punct' && values[at + 1] === close) return [at + 2, ...positions];
    if (kinds[at] !== 'punct' || values[at] !== '|') return [];
    at += 1;
  }
  return [];
}
