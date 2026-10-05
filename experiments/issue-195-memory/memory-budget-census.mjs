// Reports the memo cells (the unit of `memoryLimit`) a native parse takes,
// across its runs and repair rounds, with its time and peak heap, for each
// FILE: the evidence the default memory budget is sized against.
//   node --max-old-space-size=3072 experiments/issue-195-memory/memory-budget-census.mjs native-rust FILE...
import { readFileSync } from 'node:fs';
import { Executor } from '../../js/src/grammar-runtime/executor.js';
import { parseNative } from '../../js/src/native-grammar-parser.js';

const [id, ...files] = process.argv.slice(2);
let memory = null;
const run = Executor.prototype.run;
Executor.prototype.run = function (startRule) { memory = this.budget.memory; return run.call(this, startRule); };
// A parse runs synchronously, so the heap is sampled as cells are taken.
let peak = 0;
const retain = Executor.prototype.retain;
Executor.prototype.retain = function (count) {
  if ((this.budget.memory.cells & 0xfff) + count > 0xfff) peak = Math.max(peak, process.memoryUsage().heapUsed);
  return retain.call(this, count);
};
parseNative(id, '\n');
for (const file of files) {
  const source = readFileSync(file, 'utf8');
  const started = performance.now();
  const base = process.memoryUsage().heapUsed;
  peak = base;
  const root = parseNative(id, source);
  const bytes = Buffer.byteLength(source);
  console.log(JSON.stringify({ file: file.split('/').slice(-2).join('/'), bytes, cells: memory.cells, cellsPerByte: +(memory.cells / bytes).toFixed(1),
    ms: Math.round(performance.now() - started), peakHeapMiB: Math.round(peak / 2 ** 20), bytesPerCell: memory.cells ? Math.round((peak - base) / memory.cells) : 0, hasError: root.hasError, budgetHit: memory.cells > memory.limit }));
}
