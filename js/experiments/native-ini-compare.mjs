// Compares parity/grammars/native/ini.lino with the tree-sitter-ini oracle on
// the sources given as JSON string bodies on the command line.
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { hasRecovery, nativeRows, oracleRows } from '../scripts/native-grammar-rows.mjs';

const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/ini.lino', import.meta.url), 'utf8')));
const projection = { hidden: ['blank_space'], anonymous: ['newline', 'comment_marker'], extras: ['comment'] };
for (const source of process.argv.slice(2).map((text) => JSON.parse(`"${text}"`))) {
  const oracle = oracleRows(source, 'INI');
  const outcome = parser.parseTree(source);
  const native = outcome.ok ? nativeRows(outcome.tree, source, projection) : null;
  const verdict = hasRecovery(oracle)
    ? (outcome.ok ? 'DIVERGES (oracle recovers, native accepts)' : 'both reject')
    : (!outcome.ok ? 'NATIVE REJECTS' : JSON.stringify(native) === JSON.stringify(oracle) ? 'match' : 'ROWS DIFFER');
  const ambiguous = outcome.ok && (outcome.ambiguities?.length ?? 0) > 0 ? ` AMBIGUOUS ${JSON.stringify(outcome.ambiguities)}` : "";
  console.log(verdict.padEnd(44), JSON.stringify(source), ambiguous);
  if (verdict === 'ROWS DIFFER') {
    console.log('  oracle', JSON.stringify(oracle));
    console.log('  native', JSON.stringify(native));
  }
}
