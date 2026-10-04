// Counts what a native parse keeps alive: memo entries by rule, trivia memo
// entries, results per memo entry. Usage: node experiments/issue-195-native-memo-census.mjs json 1000
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';
import { Executor } from '../js/src/grammar-runtime/executor.js';

const grammar = process.argv[2] ?? 'json';
const count = Number(process.argv[3] ?? 1000);
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8')));
const run = Executor.prototype.run;
Executor.prototype.run = function census(startRule) {
  const outcome = run.call(this, startRule);
  const byRule = new Map();
  let results = 0;
  for (const [key, entry] of this.memo) {
    const index = Number(key.split('|')[0]);
    byRule.set(index, (byRule.get(index) ?? 0) + 1);
    results += entry.results?.length ?? 0;
  }
  const names = [...this.program.rules.values()];
  global.gc?.(); console.log("retained MB", (process.memoryUsage().heapUsed / 1e6).toFixed(1));
  console.log("memo", this.memo.size, 'results', results, 'trivia', this.triviaMemo.size, 'steps', this.budget.steps);
  console.log([...byRule].sort((a, b) => b[1] - a[1]).map(([i, n]) => `${names.find((r) => r.index === i)?.nodeKind ?? i}:${n}`).join(' '));
  return outcome;
};
const source = `[${Array.from({ length: count }, (_, i) => `{"k${i}": [${i}, true, null, "s"]}`).join(',\n')}]`;
console.log('bytes', source.length);
const outcome = parser.parseTree(source, { errorRecovery: process.argv[4] !== 'plain' });
console.log(outcome.ok);
