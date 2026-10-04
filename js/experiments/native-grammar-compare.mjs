// Compares a draft native grammar, parity/grammars/native/<id>.lino, with its
// tree-sitter oracle on the sources given as JSON string bodies on the
// command line (or one per line on stdin with --stdin):
//   node experiments/native-grammar-compare.mjs <id> <Language> '<projection json>' 'a\n' ...
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const [id, language, projectionText, ...rest] = process.argv.slice(2);
const projection = JSON.parse(projectionText);
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../../parity/grammars/native/${id}.lino`, import.meta.url), 'utf8')));
const inputs = rest[0] === '--stdin'
  ? readFileSync(0, 'utf8').split('\n').filter(Boolean).map((line) => JSON.parse(line))
  : rest.map((text) => JSON.parse(`"${text}"`));
const counts = {};
for (const source of inputs) {
  const oracle = oracleRows(source, language);
  const outcome = parser.parseTree(source);
  const native = outcome.ok ? nativeRows(outcome.tree, source, projection) : null;
  let verdict = oracleRecovers(source, language)
    ? (outcome.ok ? 'DIVERGES (oracle recovers, native accepts)' : 'both reject')
    : (!outcome.ok ? 'NATIVE REJECTS' : JSON.stringify(native) === JSON.stringify(oracle) ? 'match' : 'ROWS DIFFER');
  if (outcome.ok && (outcome.ambiguities?.length ?? 0) > 0) verdict += ' AMBIGUOUS';
  counts[verdict] = (counts[verdict] ?? 0) + 1;
  console.log(verdict.padEnd(44), JSON.stringify(source));
  if (verdict.startsWith('ROWS DIFFER') || verdict.startsWith('NATIVE REJECTS') || verdict.includes('AMBIGUOUS')) {
    console.log('  oracle', JSON.stringify(oracle));
    if (native) console.log('  native', JSON.stringify(native));
    else console.log('  rejection', JSON.stringify(outcome.rejection));
  }
}
console.log(JSON.stringify(counts));
