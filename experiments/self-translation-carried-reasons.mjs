// Tallies the frontend diagnostics that make self-translation carry items of
// the given js/src modules:
//   node experiments/self-translation-carried-reasons.mjs js/src/access.js ...
import { readFileSync } from 'node:fs';
import { selfTranslate } from '../js/src/self-translation.js';
import { checkProgram } from '../js/src/translation/check.js';
import { parseJavaScript } from '../js/src/translation/javascript.js';
const tally = new Map();
for (const file of process.argv.slice(2)) {
  const source = readFileSync(file, 'utf8');
  const bytes = Buffer.from(source, 'utf8');
  const { items } = selfTranslate(source, 'JavaScript', 'Rust');
  // A carried item is checked with the comments directly before it, as
  // self-translation groups them.
  let groupStart = null;
  for (const item of items) {
    if (item.status === 'comment' || item.term === 'comment') { groupStart ??= item.start; continue; }
    const start = groupStart ?? item.start;
    groupStart = null;
    if (item.status !== 'carried') continue;
    const text = bytes.subarray(start, item.end).toString('utf8');
    let message = item.reason;
    try { checkProgram(parseJavaScript(text)); } catch (error) { message = `${error.kind}: ${String(error.message).split('\n')[0].replace(/at \d+:\d+.*/, '').slice(0, 110)}`; }
    tally.set(message, (tally.get(message) ?? 0) + 1);
  }
}
for (const [message, count] of [...tally].sort((a, b) => b[1] - a[1])) console.log(count, message);
