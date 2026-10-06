// Parses one file with a native grammar and reports time, heap and the
// outcome, for finding the input whose parse grows without bound.
//   node --max-old-space-size=2048 experiments/issue-195-memory/native-parse-probe.mjs typescript FILE [BYTES]
import { readFileSync } from 'node:fs';
import { parseNative } from '../../js/src/native-grammar-parser.js';

const [id, file, bytes] = process.argv.slice(2);
let source = readFileSync(file, 'utf8');
if (bytes) source = source.slice(0, Number(bytes));
const started = performance.now();
let peak = 0;
const timer = setInterval(() => { peak = Math.max(peak, process.memoryUsage().rss); }, 50);
const root = parseNative(id, source);
clearInterval(timer);
peak = Math.max(peak, process.memoryUsage().rss);
console.log(JSON.stringify({
  id, file, length: source.length, ms: Math.round(performance.now() - started),
  rssMiB: Math.round(peak / 2 ** 20), heapMiB: Math.round(process.memoryUsage().heapUsed / 2 ** 20),
  term: root.term, hasError: root.hasError, children: root.children.length,
}));
