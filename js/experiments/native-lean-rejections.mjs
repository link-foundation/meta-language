// For each case of the pinned tree-sitter-lean corpus the oracle recovers
// from, whether the native Lean grammar rejects it by default and repairs it
// with error recovery: the fixture's rejections must be both.
//   node experiments/native-lean-rejections.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { corpusCases, grammarSourceOf } from '../scripts/import-native-grammars.mjs';
import { oracleRecovers } from '../scripts/native-grammar-rows.mjs';

const extra = process.argv.slice(2);
const text = readFileSync(new URL('../../parity/grammars/native/lean.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const cases = [
  ...corpusCases(grammarSourceOf('native-lean')).filter(({ source }) => oracleRecovers(source, 'Lean')),
  ...extra.map((source) => ({ file: 'argv', title: source, source })),
];
for (const { file, title, source } of cases) {
  const started = performance.now();
  const oracle = oracleRecovers(source, 'Lean') ? 'oracle-recovers' : 'oracle-accepts';
  const plain = compiled.parseTree(source).ok ? 'ACCEPT' : 'reject';
  const repaired = compiled.parseTree(source, { errorRecovery: true }).rejection?.reason ?? 'none';
  console.log(`${plain} ${repaired} ${oracle} ${file} :: ${title} (${Math.round(performance.now() - started)} ms)`);
}
