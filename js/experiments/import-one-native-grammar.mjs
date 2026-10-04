// Imports one pinned source of parity/grammars/sources.json (by native id)
// without the others, prints its import report and, with WRITE=1, writes
// its native grammar to parity/grammars/native/<language>.lino, so a new
// language can be checked without loading every grammar.
//   node experiments/import-one-native-grammar.mjs native-typescript
import { readFileSync, writeFileSync } from 'node:fs';

import { grammarSourceOf, importSource, NAME_EXPANSIONS, NATIVE_DIRECTORY } from '../scripts/import-native-grammars.mjs';

const root = new URL('../../', import.meta.url);
const entry = grammarSourceOf(process.argv[2]);
const naming = JSON.parse(readFileSync(new URL(NAME_EXPANSIONS, root), 'utf8'));
const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
const result = importSource(entry, words, naming.grammars[entry.language]);
console.log(JSON.stringify({ rules: result.rules.length, report: result.imported.report }, null, 1).slice(0, 6000));
if (process.env.WRITE) writeFileSync(new URL(`${NATIVE_DIRECTORY}/${entry.language}.lino`, root), result.text);
