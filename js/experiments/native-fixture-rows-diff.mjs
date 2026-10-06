// Prints the rows where a native grammar of the fixture generator and its
// oracle disagree on a source, for one fixture entry by id: the cases of the
// entry whose text holds FILTER, or the JSON sources given.
//   FILTER=optimize_0times node experiments/native-fixture-rows-diff.mjs rocq
import { readFileSync } from 'node:fs';
import path from 'node:path';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { NATIVE_GRAMMARS } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRows } from '../scripts/native-grammar-rows.mjs';

const [id, ...given] = process.argv.slice(2);
const entry = NATIVE_GRAMMARS.find((candidate) => candidate.id === id);
const parser = compileGrammar(parseGrammarLinks(readFileSync(path.join(import.meta.dirname, '../..', entry.grammar), 'utf8')));
const sources = given.length > 0 ? given.map((text) => JSON.parse(text)) : entry.matches.filter((source) => source.includes(process.env.FILTER ?? ''));
for (const source of sources) {
  const oracle = oracleRows(source, entry.language);
  const outcome = parser.parseTree(source);
  const native = outcome.ok ? nativeRows(outcome.tree, source, entry) : [];
  console.log(`== ${JSON.stringify(source).slice(0, 120)} ${outcome.ok ? '' : 'native rejects'}`);
  for (let index = 0; index < Math.max(oracle.length, native.length); index += 1) {
    const [o, n] = [JSON.stringify(oracle[index]), JSON.stringify(native[index])];
    if (o !== n) console.log(`! ${index} ${o}   ${n}`);
  }
}
