// Refreshes only the grammar versions and digests that
// parity/fixtures/native-default-cst-expected.json records from the language
// catalog, after a change to native grammar text that leaves every tree
// unchanged (the trees are checked by the default CST test). The full
// regeneration is js/scripts/generate-native-grammar-fixtures.mjs.
import { readFileSync, writeFileSync } from 'node:fs';

import { languageEntry } from '../js/src/index.js';
import { renderFixture } from '../js/scripts/generate-native-grammar-fixtures.mjs';

const file = new URL('../parity/fixtures/native-default-cst-expected.json', import.meta.url);
const fixture = JSON.parse(readFileSync(file, 'utf8'));
const versions = (grammars) => Object.fromEntries(grammars.map(({ id, version, parserSha256 }) => [id, { version, parserSha256 }]));
for (const [name, language] of Object.entries(fixture.languages)) {
  const catalog = languageEntry(name);
  language.grammars = versions(catalog.grammars);
  language.oracleGrammars = versions(catalog.oracleGrammars);
}
writeFileSync(file, renderFixture(fixture));
