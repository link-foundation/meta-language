import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { buildJsonDecoderBase } from '../src/grammar-importers/json-schema.js';
import { renderGrammarLinks, parseGrammarLinks } from '../src/grammar-links.js';

const source = renderGrammarLinks(buildJsonDecoderBase());
assert.equal(renderGrammarLinks(parseGrammarLinks(source)), source);
const files = [
  '../src/data/json-document/decoder.lino',
  '../../rust/src/data/json-document/decoder.lino',
];
for (const path of files) {
  const file = new URL(path, import.meta.url);
  if (process.argv.includes('--check')) assert.equal(readFileSync(file, 'utf8'), source, path);
  else writeFileSync(file, source);
}
