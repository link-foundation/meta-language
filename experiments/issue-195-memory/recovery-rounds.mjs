// Times each error-recovery round of a native parse: wraps the executor's
// `run` to print the round, its seconds and the memo cells kept so far.
//   node --max-old-space-size=2048 experiments/issue-195-memory/recovery-rounds.mjs native-javascript FILE [bytes]
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../../js/src/grammar.js';
import { parseGrammarLinks } from '../../js/src/grammar-links.js';
import { Executor } from '../../js/src/grammar-runtime/executor.js';
import { nativeGrammarText } from '../../js/src/native-grammar-parser.js';

const [id, file, bytes] = process.argv.slice(2);
let source = readFileSync(file, 'utf8');
if (bytes) source = source.slice(0, Number(bytes));
const run = Executor.prototype.run;
let round = 0;
Executor.prototype.run = function (...args) {
  const start = performance.now();
  try {
    const outcome = run.apply(this, args);
    console.log(JSON.stringify({ round: round++, seconds: ((performance.now() - start) / 1000).toFixed(2), repairs: this.repairPoints ? [...this.repairPoints] : null, ok: outcome.ok, farthest: outcome.farthest, elementFarthest: outcome.elementFarthest, cells: this.budget.memory.cells }));
    return outcome;
  } catch (error) {
    console.log(JSON.stringify({ round: round++, threw: error.constructor.name, seconds: ((performance.now() - start) / 1000).toFixed(2), cells: this.budget.memory.cells }));
    throw error;
  }
};
const parser = compileGrammar(parseGrammarLinks(nativeGrammarText(id)));
const outcome = parser.parseTree(source, { errorRecovery: true, recovery: 'accept', memoryLimit: Number(process.env.LIMIT ?? 2_000_000) });
console.log(JSON.stringify({ ok: outcome.ok, rejection: outcome.rejection }));
