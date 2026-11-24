// Decisions over scalar values and regular expression terms are shared by both runtimes.

/** @param {number} first @param {number} last @param {number} low @param {number} high @returns {[number, number]} */
export function intersectScalarIntervals(first, last, low, high) {
  let start = first;
  let end = last;
  if (low > start) start = low;
  if (high < end) end = high;
  return [start, end];
}

/** @param {number} first @param {number} last @returns {[number, number, number, number]} */
export function surrogatePairBoundaries(first, last) {
  const firstLow = (first - 65536) % 1024;
  const lastLow = (last - 65536) % 1024;
  return [
    55296 + (first - 65536 - firstLow) / 1024, 56320 + firstLow,
    55296 + (last - 65536 - lastLow) / 1024, 56320 + lastLow,
  ];
}

/** @param {number} lead @param {number} trail @returns {number} */
export function scalarFromSurrogatePair(lead, trail) {
  return 65536 + (lead - 55296) * 1024 + (trail - 56320);
}

/** @param {boolean} nullable @param {number} minimum @returns {number} */
export function consumingPatternLowerBound(nullable, minimum) {
  if (nullable) return 0;
  return minimum;
}

/** @param {number} minimum @param {number} maximum @returns {boolean} */
export function boundedPatternRepetition(minimum, maximum) {
  return minimum <= 1024 && (maximum < 0 || (maximum <= 1024 && minimum <= maximum));
}

/** @returns {[number, number, number]} */
export function stringIntersectionLimits() {
  return [12288, 4096, 3000000];
}
