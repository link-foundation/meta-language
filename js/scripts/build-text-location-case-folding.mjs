import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const check = process.argv.includes('--check');
const [source] = process.argv.slice(2).filter(argument => argument !== '--check');
assert.ok(source, 'Provide the Unicode 17.0.0 CaseFolding.txt path');

// https://www.unicode.org/Public/17.0.0/ucd/CaseFolding.txt
const data = readFileSync(source);
const digest = createHash('sha256').update(data).digest('hex');
assert.equal(digest, 'ff8d8fefbf123574205085d6714c36149eb946d717a0c585c27f0f4ef58c4183');

const mappings = [];
for (const line of data.toString('utf8').split('\n')) {
  const [code, status, mapping] = line.split('#', 1)[0].split(';').map(field => field.trim());
  // Unicode full default case folding uses common and full mappings.
  if (status === 'C' || status === 'F') mappings.push(`(${code} ${mapping})`);
}
const links = `(unicode-case-folding 17.0.0 ${digest})\n${mappings.join('\n')}\n`;

for (const location of [
  '../src/data/text-location-case-folding.lino',
  '../../rust/src/data/text-location-case-folding.lino',
]) {
  const path = new URL(location, import.meta.url);
  if (check) assert.equal(readFileSync(path, 'utf8'), links, path.pathname);
  else writeFileSync(path, links);
}
