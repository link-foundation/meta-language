// Counts the executor runs of a native parse and the steps, memo entries and
// repair points of each, by wrapping Executor.prototype.run.
//   node experiments/lean-parse-rounds.mjs [LANGUAGE] ID
import { readFileSync } from 'node:fs';
import { Executor } from '../js/src/grammar-runtime/executor.js';
import { parseNative } from '../js/src/native-grammar-parser.js';

const [language = 'Lean', id = 'property/8', key = 'source'] = process.argv.slice(2);
const dir = new URL('../parity/fixtures/issue-195-generative/', import.meta.url);
const manifest = JSON.parse(readFileSync(new URL('manifest.json', dir), 'utf8'));
const fixture = JSON.parse(readFileSync(new URL(manifest.languages[language].file, dir), 'utf8'));
const source = process.argv[5] ?? fixture.cases.find((entry) => entry.id === id)[key];
const run = Executor.prototype.run;
const runs = [];
Executor.prototype.run = function (start) {
  const started = performance.now();
  const before = this.budget.steps;
  const outcome = run.call(this, start);
  runs.push({ nested: this.begin !== 0 || this.end !== this.bytes.length, ms: Math.round(performance.now() - started), steps: this.budget.steps - before, memo: this.memo.size, points: this.repairPoints ? [...this.repairPoints].join(',') : '-', ok: outcome.ok, stale: this.expectations.stale });
  return outcome;
};
const started = performance.now();
parseNative(`native-${language.toLowerCase()}`, source);
console.log(`${Math.round(performance.now() - started)} ms, ${runs.length} runs, ${source.length} bytes`);
for (const entry of runs) console.log(JSON.stringify(entry));
