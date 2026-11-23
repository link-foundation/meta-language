import { readFileSync } from 'node:fs';
import { segmentGraphemes } from './text-location-graphemes.js';

const hangulSyllable = 0xac00;
const hangulLeadingJamo = 0x1100;
const hangulVowelJamo = 0x1161;
const hangulTrailingJamo = 0x11a7;
const leadingCount = 19;
const vowelCount = 21;
const trailingCount = 28;
const syllablesPerLeading = vowelCount * trailingCount;
const syllableCount = leadingCount * syllablesPerLeading;
let normalization;

function readNormalization() {
  const classes = new Map();
  const decompositions = new Map();
  const compositions = new Map();
  const source = readFileSync(new URL('./data/text-location-normalization.lino', import.meta.url), 'utf8');
  for (const line of source.trim().split('\n').slice(1)) {
    const [table, ...fields] = line.slice(1, -1).split(' ');
    if (table === 'class') classes.set(Number.parseInt(fields[0], 16), Number(fields[1]));
    else {
      const [first, ...rest] = fields.map(field => Number.parseInt(field, 16));
      if (table === 'decompose') decompositions.set(first, rest);
      else compositions.set(compositionKey(first, rest[0]), rest[1]);
    }
  }
  return { classes, decompositions, compositions };
}

function compositionKey(first, second) {
  return first * 0x110000 + second;
}

function decompose(scalar, decompositions) {
  const syllable = scalar - hangulSyllable;
  if (syllable < 0 || syllable >= syllableCount) return decompositions.get(scalar) ?? [scalar];
  const leading = hangulLeadingJamo + Math.floor(syllable / syllablesPerLeading);
  const vowel = hangulVowelJamo + Math.floor((syllable % syllablesPerLeading) / trailingCount);
  const trailing = syllable % trailingCount;
  return trailing ? [leading, vowel, hangulTrailingJamo + trailing] : [leading, vowel];
}

function compose(first, second, compositions) {
  const leading = first - hangulLeadingJamo;
  const vowel = second - hangulVowelJamo;
  if (leading >= 0 && leading < leadingCount && vowel >= 0 && vowel < vowelCount) {
    return hangulSyllable + leading * syllablesPerLeading + vowel * trailingCount;
  }
  const syllable = first - hangulSyllable;
  const trailing = second - hangulTrailingJamo;
  if (syllable >= 0 && syllable < syllableCount && syllable % trailingCount === 0 && trailing > 0 && trailing < trailingCount) {
    return first + trailing;
  }
  return compositions.get(compositionKey(first, second));
}

// UAX #15 compatibility decomposition, canonical ordering and composition.
function normalizedScalars(segments) {
  const { classes, decompositions, compositions } = normalization ??= readNormalization();
  const ordered = [];
  let combining = [];
  function emitCombining() {
    combining.sort((left, right) => left.combiningClass - right.combiningClass);
    for (const scalar of combining) ordered.push(scalar);
    combining = [];
  }
  for (const { segment, index } of segments) {
    const end = index + segment.length;
    for (const scalar of segment) {
      for (const value of decompose(scalar.codePointAt(0), decompositions)) {
        const combiningClass = classes.get(value) ?? 0;
        const entry = { value, combiningClass, index, end };
        if (combiningClass) combining.push(entry);
        else {
          if (combining.length) emitCombining();
          ordered.push(entry);
        }
      }
    }
  }
  if (combining.length) emitCombining();

  let starter;
  let lastClass = 0;
  let length = 0;
  for (const entry of ordered) {
    if (starter && (lastClass === 0 || lastClass < entry.combiningClass)) {
      const composite = compose(starter.value, entry.value, compositions);
      if (composite !== undefined) {
        starter.value = composite;
        starter.index = Math.min(starter.index, entry.index);
        starter.end = Math.max(starter.end, entry.end);
        continue;
      }
    }
    lastClass = entry.combiningClass;
    if (!lastClass) starter = entry;
    ordered[length++] = entry;
  }
  ordered.length = length;
  return ordered;
}

/** Yields NFKC text and the smallest ordered original UTF-16 ranges that cover it. */
export function* normalizeCompatibility(source) {
  const input = segmentGraphemes(source);
  if (!/[^\u0000-\u007f]/u.test(source)) {
    for (const { segment, index } of input) yield { text: segment, index, end: index + segment.length };
    return;
  }
  const scalars = normalizedScalars(input);
  const ranges = [];
  for (const [first, scalar] of scalars.entries()) {
    let range = { first, index: scalar.index, end: scalar.end };
    // Expansions, reordering and compositions can make source ranges overlap.
    while (ranges.length && ranges.at(-1).end > range.index) {
      const previous = ranges.pop();
      range = { first: previous.first, index: Math.min(previous.index, range.index), end: Math.max(previous.end, range.end) };
    }
    ranges.push(range);
  }
  for (const [index, range] of ranges.entries()) {
    const last = ranges[index + 1]?.first ?? scalars.length;
    let text = '';
    for (let i = range.first; i < last; i++) text += String.fromCodePoint(scalars[i].value);
    yield { text, index: range.index, end: range.end };
  }
}
