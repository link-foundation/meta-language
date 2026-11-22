import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

// https://html.spec.whatwg.org/entities.json
const [source, check] = process.argv.slice(2);
const bytes = readFileSync(source);
assert.equal(createHash('sha256').update(bytes).digest('hex'),
  'd741d877ac77c4194c4ad526b5b4a19aef8dfe411ab840a466891cdbb9f362e6');
const rows = Object.entries(JSON.parse(bytes)).map(([name, { codepoints, characters }]) => {
  assert.equal(String.fromCodePoint(...codepoints), characters);
  assert.match(name, /^&[A-Za-z][A-Za-z0-9]+;?$/u);
  return `(namedCharacterReference ${name.slice(1).replace(';', '%3B')} ${codepoints.join(' ')})`;
});
const output = `${rows.join('\n')}\n`;
for (const file of ['../src/data/document-character-references.lino', '../../rust/src/data/document-character-references.lino']) {
  const path = new URL(file, import.meta.url);
  if (check === '--check') assert.equal(readFileSync(path, 'utf8'), output);
  else writeFileSync(path, output);
}
const rules = readFileSync(new URL('../src/data/document-projection-rules.lino', import.meta.url), 'utf8');
const nativeRules = new URL('../../rust/src/data/document-projection-rules.lino', import.meta.url);
if (check === '--check') assert.equal(readFileSync(nativeRules, 'utf8'), rules);
else writeFileSync(nativeRules, rules);
console.log(`${rows.length} named character references`);
