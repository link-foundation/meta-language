#!/usr/bin/env node
// Checks candidate rejections of a native grammar: the oracle recovers from
// each, the native grammar rejects it and its automatic recovery repairs it.
//   node experiments/native-rejection-candidates.mjs C ../parity/grammars/native/c.lino 'int x' '{'
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { oracleRecovers } from '../scripts/native-grammar-rows.mjs';

const [language, grammar, ...sources] = process.argv.slice(2);
const parser = compileGrammar(parseGrammarLinks(readFileSync(grammar, 'utf8')));
for (const source of sources) {
  const started = performance.now();
  const recovers = oracleRecovers(source, language);
  const rejects = !parser.parseTree(source).ok;
  const repaired = parser.parseTree(source, { errorRecovery: true });
  const ms = Math.round(performance.now() - started);
  const verdict = recovers && rejects && repaired.rejection?.reason === 'recovered' ? 'OK ' : 'BAD';
  console.log(`${verdict} ${JSON.stringify(source)} oracle-recovers=${recovers} native-rejects=${rejects} repair=${repaired.rejection?.reason ?? 'none'} ${ms} ms`);
}
