import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const check = process.argv.includes('--check');
const paths = process.argv.slice(2).filter(argument => argument !== '--check');
assert.equal(paths.length, 4, 'Provide Unicode 17.0.0 GraphemeBreakProperty.txt, DerivedCoreProperties.txt, emoji-data.txt and GraphemeBreakTest.txt');

// https://www.unicode.org/Public/17.0.0/ucd/
const digests = [
  'd6b51d1d2ae5c33b451b7ed994b48f1f4dc62b2272a5831e7fd418514a6bae89',
  '24c7fed1195c482faaefd5c1e7eb821c5ee1fb6de07ecdbaa64b56a99da22c08',
  '2cb2bb9455cda83e8481541ecf5b6dfda66a3bb89efa3fa7c5297eccf607b72b',
  'e2d134d2c52919bace503ebb6a551c1855fe1a1faec18478c78fff254a1793ec',
];
const [breakProperties, coreProperties, emojiProperties, conformance] = paths.map((path, index) => {
  const bytes = readFileSync(path);
  assert.equal(createHash('sha256').update(bytes).digest('hex'), digests[index], path);
  return bytes.toString('utf8').split('\n').map(line => line.split('#', 1)[0].trim()).filter(Boolean);
});

function propertyRanges(lines, propertyName) {
  const ranges = [];
  for (const line of lines) {
    const [codes, property, value] = line.split(';').map(field => field.trim());
    if (propertyName && property !== propertyName) continue;
    const [start, end = start] = codes.split('..').map(code => Number.parseInt(code, 16));
    ranges.push({ start, end, value: value ?? property });
  }
  ranges.sort((left, right) => left.start - right.start);
  const joined = [];
  for (const range of ranges) {
    const previous = joined.at(-1);
    assert.ok(!previous || previous.end < range.start, 'Unicode property ranges overlap');
    if (previous && previous.end + 1 === range.start && previous.value === range.value) previous.end = range.end;
    else joined.push(range);
  }
  return joined;
}

const tables = [
  ['break', propertyRanges(breakProperties)],
  ['conjunct', propertyRanges(coreProperties, 'InCB')],
  ['pictograph', propertyRanges(emojiProperties, 'Extended_Pictographic')],
];
const propertyLinks = tables.flatMap(([table, ranges]) => ranges.map(({ start, end, value }) =>
  `(${table} ${start.toString(16)} ${end.toString(16)} ${value})`));
const conformanceLinks = conformance.map(line => {
  const groups = line.split('÷').map(group => group.replaceAll('×', ' ').trim().split(/\s+/u).join(' ')).filter(Boolean);
  return `(case ${groups.map(group => `(${group})`).join(' ')})`;
});

const outputs = [
  ['../src/data/text-location-grapheme-properties.lino', `(unicode-grapheme-properties 17.0.0 ${digests.slice(0, 3).join(' ')})\n${propertyLinks.join('\n')}\n`],
  ['../../parity/fixtures/text-location-grapheme-conformance.lino', `(unicode-grapheme-conformance 17.0.0 ${digests[3]})\n${conformanceLinks.join('\n')}\n`],
];
for (const [location, links] of outputs) {
  const path = new URL(location, import.meta.url);
  if (check) assert.equal(readFileSync(path, 'utf8'), links, path.pathname);
  else writeFileSync(path, links);
}
