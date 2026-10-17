// Portable default-CST attachment policy. Parser evaluation budgets remain
// owned by each registered parser; these limits bound the adapter traversal.

/** @param {string} kind @returns {number} */
export function readEmbeddedRegionDefaultLimit(kind) {
  if (kind === 'depth') return 128;
  if (kind === 'count') return 4096;
  if (kind === 'bytes') return 16777216;
  return -1;
}

/** @param {number} value @returns {boolean} */
export function acceptEmbeddedRegionLimit(value) {
  return value >= 0 && value <= 9007199254740991 && value % 1 === 0;
}

/** @param {number} parentStart @param {number} parentEnd
 * @param {number} start @param {number} end @returns {boolean} */
export function acceptEmbeddedRegionBounds(parentStart, parentEnd, start, end) {
  return acceptEmbeddedRegionLimit(parentStart) && acceptEmbeddedRegionLimit(parentEnd)
    && acceptEmbeddedRegionLimit(start) && acceptEmbeddedRegionLimit(end)
    && parentStart <= start && start <= end && end <= parentEnd;
}

/** Find the actual matching ancestor, including the current host. A malformed
 * column or non-decreasing parent reference refuses with -2, not a false miss.
 * Language identifiers are host-interned canonical catalog names.
 * @param {number[]} parents @param {number[]} languages
 * @param {number[]} starts @param {number[]} ends
 * @param {number} parent @param {number} language
 * @param {number} start @param {number} end @returns {number} */
export function readEmbeddedRegionAncestor(parents, languages, starts, ends, parent, language, start, end) {
  if (parents.length !== languages.length || starts.length !== parents.length || ends.length !== parents.length) return -2;
  if (!acceptEmbeddedRegionLimit(language) || !acceptEmbeddedRegionBounds(0, 9007199254740991, start, end)) return -2;
  let position = parent;
  while (position >= 0) {
    if (!acceptEmbeddedRegionLimit(position) || position >= parents.length) return -2;
    const previous = parents[position] ?? NaN;
    const selectedLanguage = languages[position] ?? NaN;
    const selectedStart = starts[position] ?? NaN;
    const selectedEnd = ends[position] ?? NaN;
    if (previous !== -1 && (!acceptEmbeddedRegionLimit(previous) || previous >= position)) return -2;
    if (!acceptEmbeddedRegionLimit(selectedLanguage) || !acceptEmbeddedRegionBounds(0, 9007199254740991, selectedStart, selectedEnd)) return -2;
    if (selectedLanguage === language && selectedStart === start && selectedEnd === end) return position;
    position = previous;
  }
  return position === -1 ? -1 : -2;
}

/** The host supplies physical, absolute byte bounds and current counters.
 * Reusing the current whole host preserves the pre-existing self-region
 * behavior; returning to an earlier identical host is a non-consuming cycle.
 * @param {number} parentStart @param {number} parentEnd
 * @param {number} parentLanguage @param {number} language
 * @param {number} start @param {number} end @param {number} ancestor
 * @param {number} depth @param {number} count @param {number} bytes
 * @param {number} depthLimit @param {number} countLimit @param {number} byteLimit
 * @param {boolean} rootSpellingReuse
 * @returns {string} */
export function readEmbeddedRegionDisposition(parentStart, parentEnd, parentLanguage, language,
  start, end, ancestor, depth, count, bytes, depthLimit, countLimit, byteLimit, rootSpellingReuse) {
  if (!acceptEmbeddedRegionBounds(parentStart, parentEnd, start, end)) return 'invalidEmbeddedRegionBounds';
  if (ancestor === -2) return 'invalidEmbeddedRegionAncestry';
  const sameHost = parentLanguage === language && start === parentStart && end === parentEnd;
  if (sameHost && depth !== 1) return 'reuseAttached';
  if (sameHost && rootSpellingReuse) return 'reuse';
  // The original root adapter parsed a canonical whole-buffer region once
  // when the requested alias had another spelling. Preserve that exact view.
  const legacyRootAlias = sameHost && depth === 1 && !rootSpellingReuse;
  if (ancestor >= 0 && !legacyRootAlias) return 'nonConsumingEmbeddingCycle';
  if (!acceptEmbeddedRegionLimit(depth) || !acceptEmbeddedRegionLimit(count) || !acceptEmbeddedRegionLimit(bytes)
    || !acceptEmbeddedRegionLimit(depthLimit) || !acceptEmbeddedRegionLimit(countLimit) || !acceptEmbeddedRegionLimit(byteLimit)) return 'invalidEmbeddedRegionLimits';
  if (depth > depthLimit) return 'embeddedRegionDepthLimit';
  if (count >= countLimit) return 'embeddedRegionCountLimit';
  if (bytes > byteLimit || end - start > byteLimit - bytes) return 'embeddedRegionByteLimit';
  return 'parse';
}

/** @param {number} current @param {number} start @param {number} end
 * @param {boolean} byteCharge @returns {number} */
export function readEmbeddedRegionCharge(current, start, end, byteCharge) {
  return current + (byteCharge ? end - start : 1);
}

/** @param {string} reason @param {number} depth @param {number} count
 * @param {number} bytes @param {number} start @param {number} end
 * @returns {number} */
export function readEmbeddedRegionDiagnosticObserved(reason, depth, count, bytes, start, end) {
  if (reason === 'embeddedRegionDepthLimit') return depth;
  if (reason === 'embeddedRegionCountLimit') {
    const observed = readEmbeddedRegionCharge(count, start, end, false);
    return acceptEmbeddedRegionLimit(observed) ? observed : -1;
  }
  if (reason === 'embeddedRegionByteLimit') {
    const observed = readEmbeddedRegionCharge(bytes, start, end, true);
    return acceptEmbeddedRegionLimit(observed) ? observed : -1;
  }
  return -1;
}

/** @param {string} reason @param {number} depthLimit
 * @param {number} countLimit @param {number} byteLimit @returns {number} */
export function readEmbeddedRegionDiagnosticLimit(reason, depthLimit, countLimit, byteLimit) {
  if (reason === 'embeddedRegionDepthLimit') return depthLimit;
  if (reason === 'embeddedRegionCountLimit') return countLimit;
  if (reason === 'embeddedRegionByteLimit') return byteLimit;
  return -1;
}

/** Physical source is rendered once when nested CSTs cover the same bytes.
 * Unspanned constructed tokens remain the host's existing ordered fragments.
 * @param {number} start @param {number} coveredUntil @returns {boolean} */
export function acceptEmbeddedRegionSourceCoverage(start, coveredUntil) {
  return start >= coveredUntil;
}
