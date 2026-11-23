import { readFileSync } from 'node:fs';

const controls = new Set(['Control', 'CR', 'LF']);
const hangulFollowers = new Map([
  ['L', new Set(['L', 'V', 'LV', 'LVT'])],
  ['LV', new Set(['V', 'T'])],
  ['V', new Set(['V', 'T'])],
  ['LVT', new Set(['T'])],
  ['T', new Set(['T'])],
]);
let properties;

function readGraphemeProperties() {
  const tables = { break: [], conjunct: [], pictograph: [] };
  const source = readFileSync(new URL('./data/text-location-grapheme-properties.lino', import.meta.url), 'utf8');
  for (const line of source.trim().split('\n').slice(1)) {
    const [table, first, last, value] = line.slice(1, -1).split(' ');
    tables[table].push({ first: Number.parseInt(first, 16), last: Number.parseInt(last, 16), value });
  }
  return tables;
}

function rangeValue(ranges, codePoint) {
  let first = 0;
  let end = ranges.length;
  while (first < end) {
    const middle = first + Math.floor((end - first) / 2);
    const range = ranges[middle];
    if (codePoint < range.first) end = middle;
    else if (codePoint > range.last) first = middle + 1;
    else return range.value;
  }
  return undefined;
}

// Unicode 17.0.0 default extended grapheme cluster boundaries, UAX #29 §3.1.1.
function breaksBefore(previous, current, { conjunct, pictographJoiner, regionalIndicators }) {
  const { category, indic, pictograph } = current;
  if (previous === 'CR' && category === 'LF') return false;
  if (controls.has(previous) || controls.has(category)) return true;
  if (hangulFollowers.get(previous)?.has(category)) return false;
  if (category === 'Extend' || category === 'ZWJ' || category === 'SpacingMark' || previous === 'Prepend') return false;
  if (indic === 'Consonant' && conjunct === 'linked') return false;
  if (pictograph && pictographJoiner) return false;
  return category !== 'Regional_Indicator' || regionalIndicators % 2 === 0;
}

/** Yields extended grapheme clusters and their original UTF-16 offsets. */
export function* segmentGraphemes(source) {
  const tables = properties ??= readGraphemeProperties();
  const context = { conjunct: undefined, pictographJoiner: false, regionalIndicators: 0 };
  let pictographWithExtenders = false;
  let previous;
  let start = 0;
  let index = 0;
  for (const scalar of source) {
    const codePoint = scalar.codePointAt(0);
    const category = rangeValue(tables.break, codePoint) ?? 'Other';
    const indic = rangeValue(tables.conjunct, codePoint);
    const pictograph = rangeValue(tables.pictograph, codePoint) !== undefined;
    if (index > 0 && breaksBefore(previous, { category, indic, pictograph }, context)) {
      yield { segment: source.slice(start, index), index: start };
      start = index;
    }

    if (indic === 'Consonant') context.conjunct = 'consonant';
    else if (indic === 'Linker' && context.conjunct) context.conjunct = 'linked';
    else if (indic !== 'Extend') context.conjunct = undefined;
    context.pictographJoiner = category === 'ZWJ' && pictographWithExtenders;
    pictographWithExtenders = pictograph || (category === 'Extend' && pictographWithExtenders);
    context.regionalIndicators = category === 'Regional_Indicator' ? context.regionalIndicators + 1 : 0;
    previous = category;
    index += scalar.length;
  }
  if (start < index) yield { segment: source.slice(start), index: start };
}
