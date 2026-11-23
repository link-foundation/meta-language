import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const [path, option] = process.argv.slice(2);
assert.ok(path && (!option || option === '--check'), 'Provide Unicode 17.0.0 UnicodeData.txt');
const bytes = readFileSync(path);
const digest = createHash('sha256').update(bytes).digest('hex');
assert.equal(digest, '2e1efc1dcb59c575eedf5ccae60f95229f706ee6d031835247d843c11d96470c');

const ranges = { word: [], dash: [], connector: [] };
let start;
for (const record of bytes.toString('utf8').trim().split('\n')) {
  const [value, name, category] = record.split(';', 3);
  const scalar = Number.parseInt(value, 16);
  if (name.endsWith(', First>')) {
    assert.equal(start, undefined);
    start = { scalar, category };
    continue;
  }
  const first = start?.scalar ?? scalar;
  if (start) assert.ok(name.endsWith(', Last>') && category === start.category);
  start = undefined;

  const word = 'LMN'.includes(category[0]) || scalar === 0x005f;
  const dash = category === 'Pd' || scalar === 0x2212;
  const connector = category === 'Pc';
  if (!word && !dash && !connector) continue;
  const target = word ? ranges.word : dash ? ranges.dash : ranges.connector;
  const previous = target.at(-1);
  if (previous && previous.last + 1 === first) previous.last = scalar;
  else target.push({ first, last: scalar });
}
assert.equal(start, undefined);
const classified = Object.entries(ranges).flatMap(([name, values]) => values.map(range => ({ name, ...range })));
classified.sort((left, right) => left.first - right.first);
const rows = classified.map(({ name, first, last }) => `(${name} ${first.toString(16)} ${last.toString(16)})`);
const links = `(unicode-character-classes 17.0.0 ${digest})\n${rows.join('\n')}\n`;
for (const location of ['../src/data/text-location-character-classes.lino', '../../rust/src/data/text-location-character-classes.lino']) {
  const output = new URL(location, import.meta.url);
  if (option) assert.equal(readFileSync(output, 'utf8'), links, output.pathname);
  else writeFileSync(output, links);
}
