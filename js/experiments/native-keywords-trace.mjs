// Imports one grammar of parity/grammars/sources.json with the keyword
// exclusion trace on, printing why each keyword candidate is excluded.
//   GRAMMAR=native-java node experiments/native-keywords-trace.mjs
import { readFileSync } from 'node:fs';

import { importSource } from '../scripts/import-native-grammars.mjs';

process.env.NATIVE_KEYWORDS_TRACE ||= '1';
const read = (path) => JSON.parse(readFileSync(new URL(`../../${path}`, import.meta.url), 'utf8'));
const { sources } = read('parity/grammars/sources.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
const entry = sources.find((item) => item.native === process.env.GRAMMAR);
importSource(entry, words, naming.grammars[entry.language]);
