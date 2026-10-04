// Regenerates the native grammar fixtures of the named grammars only:
// node experiments/regenerate-native-fixtures.mjs rust lean
import { writeFileSync } from 'node:fs';
import { NATIVE_GRAMMARS, buildNativeGrammarFixture, fixturePath, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';

for (const id of process.argv.slice(2)) {
  const entry = NATIVE_GRAMMARS.find((candidate) => candidate.id === id);
  const started = Date.now();
  writeFileSync(new URL(`../../${fixturePath(entry)}`, import.meta.url), renderFixture(buildNativeGrammarFixture(entry)));
  console.log(`wrote ${fixturePath(entry)} in ${Date.now() - started} ms`);
}
