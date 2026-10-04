// Traces the repair rounds of the native Rust grammar on the corpus cases the
// oracle recovers from: the repair points, the steps each round takes and the
// outcome, under a large step limit, to see which round exhausts the budget.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { corpusCases, grammarSourceOf } from '../scripts/import-native-grammars.mjs';
import { Executor } from '../src/grammar-runtime/executor.js';

const original = Executor.prototype.run;
const rounds = [];
Executor.prototype.run = function run(startRule) {
  const outcome = original.call(this, startRule);
  rounds.push({ points: this.repairPoints ? [...this.repairPoints] : null, steps: this.budget.steps, ok: outcome.ok, farthest: outcome.farthest, elementFarthest: outcome.elementFarthest });
  return outcome;
};
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/rust.lino', import.meta.url), 'utf8')));
const titles = (process.env.TITLES ?? 'Unexpected string literal prefixes,Longer json macro contents').split(',');
for (const { title, source } of corpusCases(grammarSourceOf('native-rust')).filter((c) => titles.includes(c.title))) {
  rounds.length = 0;
  const started = performance.now();
  const outcome = parser.parseTree(source, { errorRecovery: true, stepLimit: Number(process.env.LIMIT ?? 50_000_000) });
  console.log(`${title}: bytes=${Buffer.byteLength(source)} budget=${100_000 + 1000 * Buffer.byteLength(source)} ok=${outcome.ok} reason=${outcome.rejection?.reason} (${Math.round(performance.now() - started)} ms)`);
  for (const round of rounds) console.log('  ', JSON.stringify(round));
}
