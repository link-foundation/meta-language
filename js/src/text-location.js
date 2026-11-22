import { normalizeQuoteScalar, normalizeYoScalar, selectTextEdit } from './text-location-policy.js';

const graphemes = new Intl.Segmenter('und', { granularity: 'grapheme' });
const normalizationOrder = ['nfkc', 'quotes', 'dashes', 'yo', 'case', 'softHyphen', 'whitespace'];
const wordCharacter = /[\p{L}\p{M}\p{N}_]/u;

function sourceText(input) {
  if (typeof input === 'string') return input;
  return input.reconstructText();
}

function normalizationSet(names) {
  for (const name of names) {
    if (!normalizationOrder.includes(name)) throw new RangeError(`unknown normalization ${name}`);
  }
  return new Set(names);
}

function projection(source, names) {
  const result = [];
  function* scalars() {
    let index = 0;
    for (const segment of source) {
      yield { segment, index };
      index += segment.length;
    }
  }
  for (const { segment, index } of names.has('nfkc') ? graphemes.segment(source) : scalars()) {
    let text = segment;
    if (names.has('nfkc')) text = text.normalize('NFKC');
    if (names.has('quotes')) text = Array.from(text, value => String.fromCodePoint(normalizeQuoteScalar(value.codePointAt(0)))).join('');
    if (names.has('dashes')) text = text.replace(/[\p{Pd}−]/gu, '-');
    if (names.has('yo')) text = Array.from(text, value => String.fromCodePoint(normalizeYoScalar(value.codePointAt(0)))).join('');
    if (names.has('case')) text = Array.from(text, value => value.toLowerCase()).join('');
    if (names.has('softHyphen')) text = text.replace(/\u00ad/gu, '');
    const end = index + segment.length;
    for (const value of text) {
      if (names.has('whitespace') && /\p{White_Space}/u.test(value)) {
        if (result.at(-1)?.text === ' ') result.at(-1).end = end;
        else result.push({ text: ' ', start: index, end });
      } else result.push({ text: value, start: index, end });
    }
  }
  return result;
}

function groupProjection(entries, unit) {
  if (unit === 'scalar') return entries;
  const text = entries.map(entry => entry.text).join('');
  const groups = [];
  if (unit === 'word') {
    let previous = false;
    for (const entry of entries) {
      const word = wordCharacter.test(entry.text);
      if (word && previous) {
        groups.at(-1).text += entry.text;
        groups.at(-1).end = entry.end;
      } else if (word) groups.push({ ...entry });
      previous = word;
    }
    return groups;
  }
  let index = 0;
  for (const { segment } of graphemes.segment(text)) {
    const count = [...segment].length;
    groups.push({ text: segment, start: entries[index].start, end: entries[index + count - 1].end });
    index += count;
  }
  return groups;
}

function mappedText(text, names) {
  return projection(text, names).map(entry => entry.text).join('');
}

function requiredNormalizations(source, fragment, names) {
  const selected = new Set(names);
  for (const name of normalizationOrder) {
    if (!selected.delete(name)) continue;
    if (mappedText(source, selected) !== mappedText(fragment, selected)) selected.add(name);
  }
  return normalizationOrder.filter(name => selected.has(name));
}

function startsAtSourceBoundary(units, start) {
  return start === 0 || units[start - 1].start !== units[start].start;
}

function endsAtSourceBoundary(units, end) {
  return end === units.length || units[end - 1].end !== units[end].end;
}

function locateExact(source, fragment, words, needle, names) {
  const matches = [];
  const prefix = new Uint32Array(needle.length);
  for (let index = 1, cursor = 0; index < needle.length; index += 1) {
    while (cursor && needle[index].text !== needle[cursor].text) cursor = prefix[cursor - 1];
    if (needle[index].text === needle[cursor].text) cursor += 1;
    prefix[index] = cursor;
  }
  let cursor = 0;
  for (const [index, word] of words.entries()) {
    while (cursor && word.text !== needle[cursor].text) cursor = prefix[cursor - 1];
    if (word.text === needle[cursor].text) cursor += 1;
    if (cursor !== needle.length) continue;
    const first = index + 1 - needle.length;
    if (startsAtSourceBoundary(words, first) && endsAtSourceBoundary(words, index + 1)) {
      const start = words[first].start;
      const end = word.end;
      const text = source.slice(start, end);
      matches.push({ start, end, text, normalizations: requiredNormalizations(text, fragment, names), distance: 0, edits: [] });
    }
    cursor = prefix[cursor - 1];
  }
  return matches;
}

function align(needle, words, start, budget) {
  const length = Math.min(words.length - start, needle.length + budget);
  const rows = [new Map()];
  for (let column = 0; column <= Math.min(length, budget); column += 1) rows[0].set(column, [column, 'insert']);
  const outside = budget + 1;
  for (let row = 1; row <= needle.length; row += 1) {
    const previous = rows[row - 1];
    const cells = new Map();
    rows.push(cells);
    for (let column = Math.max(0, row - budget); column <= Math.min(length, row + budget); column += 1) {
      const equal = column > 0 && (needle[row - 1].text === words[start + column - 1].text);
      const diagonal = column > 0 ? (previous.get(column - 1)?.[0] ?? outside) + Number(!equal) : outside;
      const deletion = (previous.get(column)?.[0] ?? outside) + 1;
      const insertion = (cells.get(column - 1)?.[0] ?? outside) + 1;
      const selected = selectTextEdit(diagonal, deletion, insertion);
      const distance = [diagonal, deletion, insertion][selected];
      if (distance <= budget) cells.set(column, [distance, [equal ? 'equal' : 'replace', 'delete', 'insert'][selected]]);
    }
    if (cells.size === 0) return null;
  }
  let best = null;
  for (const [column, [distance]] of rows.at(-1)) {
    if (column === 0 || !endsAtSourceBoundary(words, start + column)) continue;
    if (best === null || distance < best.distance) best = { column, distance };
  }
  if (best === null) return null;
  const edits = [];
  let column = best.column;
  let row = needle.length;
  while (row || column) {
    const [, kind] = rows[row].get(column);
    const from = kind === 'insert' ? null : needle[row - 1];
    const to = kind === 'delete' ? null : words[start + column - 1];
    if (kind !== 'equal') edits.push({ kind, fragment: from && { ...from }, source: to && { ...to } });
    if (kind !== 'insert') row -= 1;
    if (kind !== 'delete') column -= 1;
  }
  return { ...best, edits: edits.reverse() };
}

/** Locates a fragment in a string or a lossless network. Offsets are UTF-16. */
export function locate(input, fragment, { normalize = [], maxEdits = 0, unit = 'grapheme' } = {}) {
  if (!Number.isSafeInteger(maxEdits) || maxEdits < 0) throw new RangeError('maxEdits must be a nonnegative integer');
  if (!['grapheme', 'word'].includes(unit)) throw new RangeError(`unknown edit unit ${unit}`);
  const names = normalizationSet(normalize);
  const source = sourceText(input);
  const units = maxEdits ? unit : 'scalar';
  const words = groupProjection(projection(source, names), units);
  const needle = groupProjection(projection(fragment, names), units);
  if (!needle.length) return [];
  if (!maxEdits) return locateExact(source, fragment, words, needle, names);
  const matches = [];
  const budget = Math.min(maxEdits, Math.max(needle.length, words.length));
  for (let index = 0; index < words.length; index += 1) {
    if (!startsAtSourceBoundary(words, index)) continue;
    const match = align(needle, words, index, budget);
    if (!match) continue;
    const start = words[index].start;
    const end = words[index + match.column - 1].end;
    const text = source.slice(start, end);
    const normalizations = match.distance === 0 ? requiredNormalizations(text, fragment, names) : [];
    matches.push({ start, end, text, normalizations, distance: match.distance, edits: match.edits });
  }
  return matches;
}

/** Reports every pair of shared word n-grams with original UTF-16 ranges. */
export function wordNgramOverlap(left, right, { size = 3, normalize = [] } = {}) {
  if (!Number.isSafeInteger(size) || size < 1) throw new RangeError('n-gram size must be a positive integer');
  const names = normalizationSet(normalize);
  function* ngrams(source) {
    const words = groupProjection(projection(source, names), 'word');
    for (let index = 0; index + size <= words.length; index += 1) {
      const items = words.slice(index, index + size);
      const span = { start: items[0].start, end: items.at(-1).end };
      yield { words: items.map(item => item.text), span };
    }
  }
  const rightSpans = new Map();
  for (const { words, span } of ngrams(sourceText(right))) {
    const key = JSON.stringify(words);
    if (!rightSpans.has(key)) rightSpans.set(key, []);
    rightSpans.get(key).push(span);
  }
  const matches = [];
  for (const { words, span } of ngrams(sourceText(left))) {
    for (const other of rightSpans.get(JSON.stringify(words)) ?? []) matches.push({ words, left: span, right: other });
  }
  return matches;
}
