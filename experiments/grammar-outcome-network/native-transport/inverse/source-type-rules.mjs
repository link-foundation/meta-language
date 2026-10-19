// Complete source-type edge admission. Scalars and native representations are
// independent obligations; the structural worklist runs before this operation.
/**
 * @param {number[]} types @param {number[]} selectors
 * @param {number[]} offsets @param {number[]} children
 * @param {number[]} modes @param {number[]} patterns
 * @param {number[]} patternOffsets @param {number[]} patternTypes
 * @param {number[]} variantOwners @returns {number}
 */
export function checkOwnedOutcomeSourceTypes(types, selectors, offsets, children, modes, patterns, patternOffsets, patternTypes, variantOwners) {
  const count = types.length;
  const typeCount = modes.length;
  const patternCount = patternOffsets.length - 1;
  if (count < 1 || selectors.length !== count || offsets.length !== count + 1
    || typeCount < 1 || patterns.length !== typeCount || patternCount < 1
    || variantOwners.length !== patternCount || patternOffsets[0] !== 0
    || patternOffsets[patternCount] !== patternTypes.length) return -1;
  for (let at = 0; at < patternOffsets.length; at += 1) {
    const value = patternOffsets[at] ?? -1;
    if (!Number.isSafeInteger(value) || value < 0 || value > patternTypes.length
      || at > 0 && value < (patternOffsets[at - 1] ?? -1)) return -1;
  }
  for (let at = 0; at < patternTypes.length; at += 1) {
    const value = patternTypes[at] ?? -1;
    if (!Number.isSafeInteger(value) || value < 0 || value >= typeCount) return -1;
  }
  for (let at = 0; at < patternCount; at += 1) {
    const owner = variantOwners[at] ?? -2;
    if (!Number.isSafeInteger(owner) || owner < -1 || owner >= typeCount
      || owner >= 0 && modes[owner] !== 4) return -1;
  }
  for (let at = 0; at < typeCount; at += 1) {
    const mode = modes[at] ?? -2;
    const pattern = patterns[at] ?? -1;
    if (!Number.isSafeInteger(mode) || mode < -1 || mode > 5 || !Number.isSafeInteger(pattern)
      || pattern < 0 || pattern >= patternCount || variantOwners[pattern] !== -1) return -1;
    const size = (patternOffsets[pattern + 1] ?? -1) - (patternOffsets[pattern] ?? -1);
    if ((mode === 1 || mode === 2) && size !== 1 || mode === 3 && size !== 2
      || (mode === -1 || mode === 4 || mode === 5) && size !== 0) return -1;
  }
  if (offsets[0] !== 0 || offsets[count] !== children.length) return -2;
  for (let at = 0; at < offsets.length; at += 1) {
    const value = offsets[at] ?? -1;
    if (!Number.isSafeInteger(value) || value < 0 || value > children.length
      || at > 0 && value < (offsets[at - 1] ?? -1)) return -2;
  }
  for (let at = 0; at < count; at += 1) {
    const type = types[at] ?? -1;
    if (!Number.isSafeInteger(type) || type < 0 || type >= typeCount) return -3;
    const mode = modes[type] ?? -1;
    if (mode < 0) return -3;
    const selector = selectors[at] ?? -2;
    if (!Number.isSafeInteger(selector)) return -4;
    let pattern = patterns[type] ?? -1;
    if (mode === 4) {
      if (selector < 0 || selector >= patternCount || variantOwners[selector] !== type) return -4;
      pattern = selector;
    } else if (mode === 1) {
      if (selector !== 0 && selector !== 1) return -4;
    } else if (mode === 5) {
      if (selector < 0 || selector > 5) return -4;
    } else if (selector !== -1) return -4;
    const start = offsets[at] ?? -1;
    const end = offsets[at + 1] ?? -1;
    const first = patternOffsets[pattern] ?? -1;
    const size = (patternOffsets[pattern + 1] ?? -1) - first;
    if ((mode === 0 || mode === 4) && end - start !== size
      || mode === 1 && end - start !== selector || mode === 3 && (end - start) % 2 !== 0
      || mode === 5 && selector < 4 && end !== start) return -5;
    for (let edge = start; edge < end; edge += 1) {
      const child = children[edge] ?? 0;
      if (!Number.isSafeInteger(child) || child < 1 || child > count) return -6;
      const expected = mode === 5 ? type : patternTypes[first + (mode === 1 || mode === 2 ? 0 : mode === 3 ? (edge - start) % 2 : edge - start)] ?? -1;
      if (types[child - 1] !== expected) return -6;
    }
  }
  return 0;
}
