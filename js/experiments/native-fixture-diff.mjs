// Writes the freshly built native grammar fixture of one language to a file,
// to diff against the committed one without rewriting it.
//   node experiments/native-fixture-diff.mjs LANGUAGE OUT
import { writeFileSync } from 'node:fs';
import { NATIVE_GRAMMARS, buildNativeGrammarFixture, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';

const [id, out] = process.argv.slice(2);
writeFileSync(out, renderFixture(buildNativeGrammarFixture(NATIVE_GRAMMARS.find((entry) => entry.id === id))));
