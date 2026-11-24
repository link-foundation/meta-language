// Deterministic regular grammar discovery. The numeric table is an internal
// transport over existing Links data, not an independently meaningful ontology.
// Eight cells per row: states (0,parent,accept,reject,arrivals,finals,red,rank),
// edges (1,source,symbol,target,count,0,0,0), accepted folds (2,to,from,evidence,...).

/** @param {number[]} table @param {number} at @returns {number} */
export function readInferenceCell(table, at) {
  return table[at] ?? 0;
}

/** @param {number[]} table @param {number} at @param {number} value @returns {number[]} */
export function replaceInferenceCell(table, at, value) {
  return table.map((cell, index) => index === at ? value : cell);
}

/** @param {number[]} table @param {number} state @returns {number} */
export function findInferenceRepresentative(table, state) {
  let current = state;
  for (let count = 0; count < table.length; count += 8) {
    const parent = readInferenceCell(table, current + 1);
    if (parent === current) return current;
    current = parent;
  }
  throw new RangeError('grammar inference parent cycle');
}

/** @param {number[]} table @param {number} state @param {number} symbol @returns {number} */
export function findInferenceTransition(table, state, symbol) {
  for (let at = 0; at < table.length; at += 8) {
    if (readInferenceCell(table, at) === 1 && readInferenceCell(table, at + 1) === state
      && readInferenceCell(table, at + 2) === symbol) return at;
  }
  return -1;
}

/** Label/length/symbol sequences retain duplicates as independent observations.
 * @param {number[]} samples @param {number} alphabetSize @param {number} stateLimit
 * @returns {number[]}
 */
export function buildInferencePrefixTree(samples, alphabetSize, stateLimit) {
  let table = [0, 0, 0, 0, 0, 0, 1, 0], position = 0, states = 1;
  while (position < samples.length) {
    const label = readInferenceCell(samples, position), length = readInferenceCell(samples, position + 1);
    if ((label !== 0 && label !== 1) || length < 0 || Math.floor(length) !== length
      || position + 2 + length > samples.length) return [-1];
    position += 2;
    let state = 0;
    if (label === 1) table = replaceInferenceCell(table, 4, readInferenceCell(table, 4) + 1);
    for (let index = 0; index < length; index += 1) {
      const symbol = readInferenceCell(samples, position + index);
      if (symbol < 0 || symbol >= alphabetSize || Math.floor(symbol) !== symbol) return [-2];
      let edge = findInferenceTransition(table, state, symbol);
      if (edge < 0) {
        if (states >= stateLimit) return [-3];
        const next = table.length;
        table = [...table, 0, next, 0, 0, 0, 0, 0, states];
        edge = table.length;
        table = [...table, 1, state, symbol, next, 0, 0, 0, 0];
        states += 1;
      }
      if (label === 1) table = replaceInferenceCell(table, edge + 4, readInferenceCell(table, edge + 4) + 1);
      state = readInferenceCell(table, edge + 3);
      if (label === 1) table = replaceInferenceCell(table, state + 4, readInferenceCell(table, state + 4) + 1);
    }
    table = replaceInferenceCell(table, state + (label === 1 ? 2 : 3), 1);
    if (label === 1) table = replaceInferenceCell(table, state + 5, readInferenceCell(table, state + 5) + 1);
    if (readInferenceCell(table, state + 2) === 1 && readInferenceCell(table, state + 3) === 1) {
      return [-4];
    }
    position += length;
  }
  return table;
}

/** A total deterministic logarithm for the confidence bound. The finite series
 * and binary64 operations are part of this versioned algorithm definition.
 * @param {number} value @returns {number}
 */
export function approximateInferenceLogarithm(value) {
  if (!(value > 0) || !Number.isFinite(value)) throw new RangeError('grammar inference logarithm domain');
  let scaled = value, exponent = 0;
  while (scaled > 2) { scaled /= 2; exponent += 1; }
  while (scaled < 1) { scaled *= 2; exponent -= 1; }
  const ratio = (scaled - 1) / (scaled + 1), square = ratio * ratio;
  let term = ratio, total = 0;
  for (let index = 0; index < 64; index += 1) {
    total += term / (2 * index + 1); term *= square;
  }
  return 2 * total + exponent * 0.6931471805599453;
}

/** @param {number} first @param {number} firstCount @param {number} second
 * @param {number} secondCount @param {number} bound @returns {boolean}
 */
export function acceptInferenceProportion(first, firstCount, second, secondCount, bound) {
  if (firstCount === 0 || secondCount === 0) return true;
  return Math.abs(first / firstCount - second / secondCount)
    <= bound * (1 / Math.sqrt(firstCount) + 1 / Math.sqrt(secondCount));
}

/** Recursive statistical compatibility uses an explicit pair worklist.
 * @param {number[]} table @param {number} first @param {number} second
 * @param {number} bound @param {number} pairLimit @returns {number}
 */
export function acceptInferenceDistributions(table, first, second, bound, pairLimit) {
  let pending = [first, second], seen = [];
  let position = 0;
  while (position < pending.length) {
    if (position / 2 >= pairLimit) return -5;
    const left = findInferenceRepresentative(table, readInferenceCell(pending, position));
    const right = findInferenceRepresentative(table, readInferenceCell(pending, position + 1));
    position += 2;
    const key = Math.min(left, right) * table.length + Math.max(left, right);
    if (left === right || seen.includes(key)) continue;
    seen = [...seen, key];
    const leftCount = readInferenceCell(table, left + 4), rightCount = readInferenceCell(table, right + 4);
    if (!acceptInferenceProportion(readInferenceCell(table, left + 5), leftCount,
      readInferenceCell(table, right + 5), rightCount, bound)) return 0;
    for (let at = 0; at < table.length; at += 8) {
      if (readInferenceCell(table, at) !== 1) continue;
      const owner = readInferenceCell(table, at + 1);
      if (owner !== left && owner !== right) continue;
      const symbol = readInferenceCell(table, at + 2), a = findInferenceTransition(table, left, symbol), b = findInferenceTransition(table, right, symbol);
      if (!acceptInferenceProportion(a < 0 ? 0 : readInferenceCell(table, a + 4), leftCount,
        b < 0 ? 0 : readInferenceCell(table, b + 4), rightCount, bound)) return 0;
      if (a >= 0 && b >= 0) pending = [...pending, readInferenceCell(table, a + 3), readInferenceCell(table, b + 3)];
    }
  }
  return 1;
}

/** Empty return means inconsistent labels. Every accepted recursive fold
 * redirects real transitions and combines real observed frequencies.
 * @param {number[]} original @param {number} target @param {number} source
 * @param {number} pairLimit @returns {number[]}
 */
export function foldInferenceStates(original, target, source, pairLimit) {
  let table = original.slice(), evidence = 0, position = 0;
  let pending = [target, source];
  while (position < pending.length) {
    if (position / 2 >= pairLimit) return [-5];
    const left = findInferenceRepresentative(table, readInferenceCell(pending, position));
    const right = findInferenceRepresentative(table, readInferenceCell(pending, position + 1));
    position += 2;
    if (left === right) continue;
    const acceptLeft = readInferenceCell(table, left + 2), acceptRight = readInferenceCell(table, right + 2);
    const rejectLeft = readInferenceCell(table, left + 3), rejectRight = readInferenceCell(table, right + 3);
    if (acceptLeft === 1 && rejectRight === 1 || rejectLeft === 1 && acceptRight === 1) return [];
    if (acceptLeft === 1 && acceptRight === 1) evidence += 1;
    if (rejectLeft === 1 && rejectRight === 1) evidence += 1;
    table = replaceInferenceCell(table, right + 1, left);
    table = replaceInferenceCell(table, left + 2, Math.max(acceptLeft, acceptRight));
    table = replaceInferenceCell(table, left + 3, Math.max(rejectLeft, rejectRight));
    for (let field = 4; field <= 5; field += 1) {
      table = replaceInferenceCell(table, left + field, readInferenceCell(table, left + field) + readInferenceCell(table, right + field));
    }
    table = replaceInferenceCell(table, left + 6, Math.max(readInferenceCell(table, left + 6), readInferenceCell(table, right + 6)));
    for (let edge = 0; edge < table.length; edge += 8) {
      if (readInferenceCell(table, edge) !== 1 || readInferenceCell(table, edge + 1) !== right) continue;
      const symbol = readInferenceCell(table, edge + 2), other = findInferenceTransition(table, left, symbol);
      if (other < 0) table = replaceInferenceCell(table, edge + 1, left);
      else {
        pending = [...pending, readInferenceCell(table, other + 3), readInferenceCell(table, edge + 3)];
        table = replaceInferenceCell(table, other + 4, readInferenceCell(table, other + 4) + readInferenceCell(table, edge + 4));
        table = replaceInferenceCell(table, edge, -1);
      }
    }
  }
  for (let edge = 0; edge < table.length; edge += 8) {
    if (readInferenceCell(table, edge) === 1) table = replaceInferenceCell(table, edge + 3,
      findInferenceRepresentative(table, readInferenceCell(table, edge + 3)));
  }
  return [...table, 2, target, source, evidence, 0, 0, 0, 0];
}

/** @param {number[]} table @returns {number[]} */
export function readInferenceBlueStates(table) {
  let next = [];
  for (let edge = 0; edge < table.length; edge += 8) {
    if (readInferenceCell(table, edge) !== 1) continue;
    const source = findInferenceRepresentative(table, readInferenceCell(table, edge + 1));
    const target = findInferenceRepresentative(table, readInferenceCell(table, edge + 3));
    if (readInferenceCell(table, source + 6) === 1 && readInferenceCell(table, target + 6) === 0 && !next.includes(target)) next = [...next, target];
  }
  let result = [];
  for (let state = 0; state < table.length; state += 8) if (next.includes(state)) result = [...result, state];
  return result;
}

/** RPNI (0), EDSM (1), or deterministic ALERGIA-style confidence merging (2).
 * Sample order and first-observed symbol order are explicit tie breakers.
 * @param {number[]} samples @param {number} alphabetSize @param {number} strategy
 * @param {number} alpha @param {number} stateLimit @param {number} pairLimit
 * @param {number} attemptLimit @returns {number[]}
 */
export function inferRegularGrammarTable(samples, alphabetSize, strategy, alpha, stateLimit, pairLimit, attemptLimit) {
  if (strategy !== 0 && strategy !== 1 && strategy !== 2) return [-7];
  if (!(alpha > 0 && alpha < 1)) return [-8];
  let table = buildInferencePrefixTree(samples, alphabetSize, stateLimit), attempts = 0;
  if (readInferenceCell(table, 0) < 0) return table;
  const bound = Math.sqrt(0.5 * approximateInferenceLogarithm(2 / (1 - alpha)));
  while (true) {
    const blue = readInferenceBlueStates(table);
    if (blue.length === 0) return table;
    let best = [], bestEvidence = -1;
    for (let index = 0; index < blue.length; index += 1) {
      if (strategy !== 1 && index > 0) break;
      const source = readInferenceCell(blue, index);
      for (let target = 0; target < table.length; target += 8) {
        if (readInferenceCell(table, target) !== 0 || readInferenceCell(table, target + 6) !== 1
          || findInferenceRepresentative(table, target) !== target) continue;
        attempts += 1;
        if (attempts > attemptLimit) return [-6];
        if (strategy === 2) {
          const compatible = acceptInferenceDistributions(table, target, source, bound, pairLimit);
          if (compatible < 0) return [compatible];
          if (compatible === 0) continue;
        }
        const candidate = foldInferenceStates(table, target, source, pairLimit);
        if (candidate.length === 0) continue;
        if (readInferenceCell(candidate, 0) < 0) return candidate;
        const score = readInferenceCell(candidate, candidate.length - 5);
        if (score > bestEvidence) { best = candidate; bestEvidence = score; }
        if (strategy !== 1) break;
      }
    }
    if (best.length > 0) table = best;
    else table = replaceInferenceCell(table, readInferenceCell(blue, 0) + 6, 1);
  }
}

/** @param {number} code @returns {string} */
export function readInferenceFailureReason(code) {
  if (code === -1) return 'invalid labelled grammar inference sample';
  if (code === -2) return 'invalid grammar inference symbol';
  if (code === -3) return 'grammar inference state limit exceeded';
  if (code === -4) return 'contradictory positive and negative grammar inference samples';
  if (code === -5) return 'grammar inference pair limit exceeded';
  if (code === -6) return 'grammar inference merge attempt limit exceeded';
  if (code === -7) return 'unknown regular grammar inference strategy';
  if (code === -8) return 'grammar inference confidence must be between zero and one';
  return 'invalid grammar inference result';
}

/** @param {number[]} table @param {number[]} symbols @returns {boolean} */
export function acceptInferredSymbolSequence(table, symbols) {
  let state = findInferenceRepresentative(table, 0);
  for (const symbol of symbols) {
    const edge = findInferenceTransition(table, state, symbol);
    if (edge < 0) return false;
    state = findInferenceRepresentative(table, readInferenceCell(table, edge + 3));
  }
  return readInferenceCell(table, state + 2) === 1 && readInferenceCell(table, state + 3) === 0;
}

/** Check actual model rows before following caller-supplied graph references.
 * @param {number[]} table @param {number} alphabetSize @returns {boolean}
 */
export function acceptInferenceTableShape(table, alphabetSize) {
  if (table.length < 8 || table.length > 524288 || table.length % 8 !== 0 || readInferenceCell(table, 0) !== 0) return false;
  for (const value of table) if (!Number.isSafeInteger(value) || value < -1) return false;
  for (let at = 0; at < table.length; at += 8) {
    const kind = readInferenceCell(table, at);
    if (kind !== -1 && kind !== 0 && kind !== 1 && kind !== 2) return false;
    if (kind === 0) {
      const parent = readInferenceCell(table, at + 1);
      if (parent < 0 || parent >= table.length || parent % 8 !== 0 || readInferenceCell(table, parent) !== 0) return false;
      const accept = readInferenceCell(table, at + 2), reject = readInferenceCell(table, at + 3), red = readInferenceCell(table, at + 6);
      if ((accept !== 0 && accept !== 1) || (reject !== 0 && reject !== 1) || accept + reject > 1 || (red !== 0 && red !== 1)) return false;
      if (readInferenceCell(table, at + 4) < 0 || readInferenceCell(table, at + 5) < 0 || readInferenceCell(table, at + 7) < 0) return false;
      let current = at, complete = false;
      for (let count = 0; count < table.length; count += 8) {
        const next = readInferenceCell(table, current + 1);
        if (next < 0 || next >= table.length || next % 8 !== 0 || readInferenceCell(table, next) !== 0) return false;
        if (next === current) { complete = true; break; }
        current = next;
      }
      if (!complete) return false;
    }
    if (kind === 1 || kind === 2) {
      const source = readInferenceCell(table, at + 1), target = readInferenceCell(table, at + (kind === 1 ? 3 : 2));
      if (source < 0 || target < 0 || source >= table.length || target >= table.length || source % 8 !== 0 || target % 8 !== 0
        || readInferenceCell(table, source) !== 0 || readInferenceCell(table, target) !== 0) return false;
      if (kind === 1) {
        const symbol = readInferenceCell(table, at + 2);
        if (symbol < 0 || symbol >= alphabetSize || readInferenceCell(table, at + 4) < 0
          || readInferenceCell(table, source + 1) !== source || readInferenceCell(table, target + 1) !== target) return false;
        for (let other = 0; other < at; other += 8) if (readInferenceCell(table, other) === 1
          && readInferenceCell(table, other + 1) === source && readInferenceCell(table, other + 2) === symbol) return false;
      } else if (readInferenceCell(table, at + 3) < 0) return false;
    }
  }
  return true;
}
