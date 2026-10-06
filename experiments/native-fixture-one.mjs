// Builds the native grammar fixture of one NATIVE_GRAMMARS entry and reports
// its counts or the first disagreement, without writing anything.
//   node experiments/native-fixture-one.mjs c
import { NATIVE_GRAMMARS, buildNativeGrammarFixture } from '../js/scripts/generate-native-grammar-fixtures.mjs';

const entry = NATIVE_GRAMMARS.find(({ id }) => id === process.argv[2]);
const started = Date.now();
const fixture = buildNativeGrammarFixture(entry);
console.log(entry.id, Object.fromEntries(Object.entries(fixture).filter(([, value]) => Array.isArray(value)).map(([key, value]) => [key, value.length])), `${Date.now() - started} ms`);
