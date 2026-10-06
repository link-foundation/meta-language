// Narrows the Lean generative edit/6 case the oracle parses cleanly and the native grammar rejects.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { oracleRecovers } from '../scripts/native-grammar-rows.mjs';
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/lean.lino', import.meta.url), 'utf8')));
for (const source of process.argv.slice(2).map((s) => JSON.parse(`"${s}"`))) {
  const outcome = parser.parseTree(source);
  console.log(JSON.stringify(source), 'native', outcome.ok, outcome.rejection?.offset ?? outcome.rejection?.message ?? '', 'oracle-recovers', oracleRecovers(source, 'Lean'));
}
