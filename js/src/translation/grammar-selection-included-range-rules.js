// Closed source-view decisions. Native implementations are generated from
// this complete module; host adapters supply actual source coordinates only.

/** @param {string} mapping @returns {boolean} */
export function acceptGrammarSelectionIncludedRangeMapping(mapping) {
  return mapping === 'included-ranges';
}

/** Zero preserves the existing JSON/identity dispatcher. One selects this
 * common syntactic view; two refuses mixed or lexical mapping contexts.
 * @param {string[]} mappings @param {boolean} inToken @returns {number}
 */
export function readGrammarSelectionIncludedRangeContext(mappings, inToken) {
  let found = false;
  for (let index = 0; index < mappings.length; index += 1) {
    if (acceptGrammarSelectionIncludedRangeMapping(mappings[index] ?? 'identity')) found = true;
  }
  if (!found) return 0;
  if (inToken) return 2;
  for (let index = 0; index < mappings.length; index += 1) {
    if (!acceptGrammarSelectionIncludedRangeMapping(mappings[index] ?? 'identity')) return 2;
  }
  return 1;
}

/** @param {string} kind @param {string} mapping @returns {boolean} */
export function acceptGrammarSelectionIncludedRangeCandidate(kind, mapping) {
  return kind === 'grammar-range-candidate' && acceptGrammarSelectionIncludedRangeMapping(mapping);
}

/** Coordinates are relative UTF-8 byte offsets, never UTF-16 positions.
 * @param {string} kind @param {number} start @param {number} end @returns {boolean}
 */
export function acceptGrammarSelectionIncludedRangeSourceRange(kind, start, end) {
  return kind === 'source-range' && Number.isInteger(start) && Number.isInteger(end)
    && start >= 0 && start < end && end <= 4294967295;
}

/** @param {number[]} first @param {number[]} next @returns {boolean} */
export function acceptGrammarSelectionIncludedRangeAgreement(first, next) {
  if (first.length !== next.length) return false;
  for (let index = 0; index < first.length; index += 1) if (first[index] !== next[index]) return false;
  return true;
}

/** Validate before reserving work for untrusted range columns. Nonempty
 * disjoint integral ranges cannot outnumber the physical source bytes.
 * @param {number} bytes @param {number} cells @param {boolean} steps @returns {number}
 */
export function readGrammarSelectionIncludedRangeReservation(bytes, cells, steps) {
  if (!Number.isInteger(bytes) || bytes <= 0 || bytes > 4294967295
      || !Number.isInteger(cells) || cells <= 0 || cells % 2 !== 0 || cells > 2 * bytes) return -1;
  return (steps ? 64 : 32) * bytes + 64;
}

/** Flat relative [start,end] pairs become complete ordered correspondence
 * columns. Excluded gaps have zero output width; the last row owns the end
 * point. A proper byte contraction prevents fresh-plane non-consuming cycles.
 * @param {number[]} ranges @param {number} length @returns {number[]}
 */
export function readGrammarSelectionIncludedRangePlan(ranges, length) {
  if (readGrammarSelectionIncludedRangeReservation(length, ranges.length, false) < 0) return [];
  const plan = [0, 0, 0, 0];
  let cursor = 0, output = 0;
  for (let index = 0; index < ranges.length; index += 2) {
    const start = ranges[index] ?? -1;
    const end = ranges[index + 1] ?? -1;
    if (!acceptGrammarSelectionIncludedRangeSourceRange('source-range', start, end)
        || start < cursor || end > length) return [];
    if (cursor < start) plan.push(cursor, start, output, output);
    plan.push(start, end, output, output + end - start);
    output += end - start;
    cursor = end;
  }
  if (output >= length) return [];
  if (cursor < length) plan.push(cursor, length, output, output);
  plan.push(length, length, output, output);
  return plan;
}

/** Stream one gap, scalar copy or point from the validated complete plan.
 * Width is measured by the existing decoder on actual parent bytes. Returning
 * a fixed-size part lets native callers borrow the complete unchanged plan.
 * [0,nextRow,nextPosition,nextOutput,inputStart,inputEnd,outputStart,outputEnd]
 * emits one row; [1] ends; [] refuses inconsistent transport facts.
 * @param {number[]} plan @param {number} row @param {number} position
 * @param {number} output @param {number} width @returns {number[]}
 */
export function readGrammarSelectionIncludedRangePart(plan, row, position, output, width) {
  if (plan.length < 12 || plan.length % 4 !== 0 || !Number.isInteger(row)
      || row < 0 || row % 4 !== 0 || row > plan.length) return [];
  if (row === plan.length) return [1];
  const start = plan[row] ?? -1;
  const end = plan[row + 1] ?? -1;
  const outputStart = plan[row + 2] ?? -1;
  const outputEnd = plan[row + 3] ?? -1;
  const following = row + 4;
  const followingPosition = plan[following] ?? end;
  if (outputStart === outputEnd) {
    if (position !== start || output !== outputStart) return [];
    return [0, following, followingPosition, output, start, end, output, output];
  }
  if (!Number.isInteger(position) || position < start || position >= end
      || output !== outputStart + position - start || !Number.isInteger(width)
      || width < 1 || width > 4 || position + width > end) return [];
  const next = position + width;
  const nextOutput = output + width;
  return [0, next === end ? following : row, next === end ? followingPosition : next,
    nextOutput, position, next, output, nextOutput];
}

/** The parent source has already passed actual UTF-8 validation. Its interior
 * boundaries are precisely non-continuation bytes; its end is a boundary too.
 * @param {number} byte @param {boolean} atEnd @returns {boolean}
 */
export function acceptGrammarSelectionIncludedRangeByteBoundary(byte, atEnd) {
  return atEnd || Number.isInteger(byte) && byte >= 0 && byte <= 255
    && (byte < 128 || byte >= 192);
}

/** Keep every legacy ordered JSON field unchanged. Only this explicit
 * operation adds range data to its source derivation.
 * @param {string} role @returns {string}
 */
export function readGrammarSelectionIncludedRangeFieldOrder(role) {
  if (role === 'derivation') return 'operation|input|start|end|ranges';
  if (role === 'mapping') return 'kind|operation|input|output|ranges|intervals';
  if (role === 'candidate') return 'kind|name|mapping|ranges';
  if (role === 'range') return 'kind|start|end';
  return '';
}

/** @param {string} operation @param {number} input @param {number} output
 * @param {number} start @param {number} end @param {number} inputLength @returns {boolean}
 */
export function acceptGrammarSelectionIncludedRangeDerivation(operation, input, output, start, end, inputLength) {
  return acceptGrammarSelectionIncludedRangeMapping(operation) && Number.isInteger(input)
    && input >= 0 && input < output && Number.isInteger(start) && Number.isInteger(end)
    && start >= 0 && start < end && end <= inputLength;
}

/** @param {string} mapping @param {boolean} physicalChildren @param {boolean} sourceMapping @returns {boolean} */
export function acceptGrammarSelectionIncludedRangeProfile(mapping, physicalChildren, sourceMapping) {
  return acceptGrammarSelectionIncludedRangeMapping(mapping) && physicalChildren && sourceMapping;
}

/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptGrammarSelectionIncludedRangeDescriptor(kind, operation) {
  return kind === 'source-mapping' && acceptGrammarSelectionIncludedRangeMapping(operation);
}
