// Parses a valid file repeated 1, 2, 4, ... times with a native grammar and
// reports the parse time and heap of each size, after compiling the grammar
// once, to show whether the executor's cost grows linearly with the input.
//   node --max-old-space-size=2048 experiments/issue-195-memory/native-scaling-probe.mjs native-typescript FILE MAX_COPIES
import { readFileSync } from 'node:fs';
import { parseNative } from '../../js/src/native-grammar-parser.js';

const [id, file, maxCopies = '8'] = process.argv.slice(2);
const unit = readFileSync(file, 'utf8');
parseNative(id, 'x;\n');
for (let copies = 1; copies <= Number(maxCopies); copies *= 2) {
  const source = unit.repeat(copies);
  globalThis.gc?.();
  const before = process.memoryUsage().heapUsed;
  let peak = before;
  const timer = setInterval(() => { peak = Math.max(peak, process.memoryUsage().heapUsed); }, 20);
  const started = performance.now();
  const root = parseNative(id, source);
  clearInterval(timer);
  peak = Math.max(peak, process.memoryUsage().heapUsed);
  console.log(JSON.stringify({ copies, length: source.length, ms: Math.round(performance.now() - started),
    peakHeapMiB: Math.round((peak - before) / 2 ** 20), hasError: root.hasError }));
}
