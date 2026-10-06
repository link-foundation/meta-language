// Counts what each executor memo table holds after a native parse of FILE
// repeated 1, 2, 4, ... times: entries per table and the results the rule
// memo keeps, to find which table grows with the input.
//   node --max-old-space-size=2048 experiments/issue-195-memory/memo-census-probe.mjs native-typescript FILE MAX_COPIES
import { readFileSync } from 'node:fs';
import { Executor } from '../../js/src/grammar-runtime/executor.js';
import { parseNative } from '../../js/src/native-grammar-parser.js';

const [id, file, maxCopies = '4'] = process.argv.slice(2);
const unit = readFileSync(file, 'utf8');
const executors = [];
const run = Executor.prototype.run;
Executor.prototype.run = function (startRule) { executors.push(this); return run.call(this, startRule); };
parseNative(id, 'x;\n');
const tables = ['memo', 'triviaMemo', 'operandMemo', 'shiftMemo', 'edgeMemo', 'belowMemo', 'scannerMemo', 'embedMemo', 'repairMemo'];
for (let copies = 1; copies <= Number(maxCopies); copies *= 2) {
  executors.length = 0;
  const source = unit.repeat(copies);
  const root = parseNative(id, source);
  const census = { copies, length: source.length, runs: executors.length, hasError: root.hasError };
  for (const executor of executors.slice(-1)) {
    for (const table of tables) census[table] = executor[table].size;
    let results = 0, children = 0;
    for (const entry of executor.memo.values()) {
      for (const result of entry.results ?? []) { results += 1; children += result.children.length; }
    }
    Object.assign(census, { memoResults: results, memoChildren: children, steps: executor.budget.steps });
  }
  console.log(JSON.stringify(census));
}
