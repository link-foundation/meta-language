// Prints the first lines of carried items whose frontend diagnostic matches a pattern:
//   node experiments/self-translation-carried-samples.mjs 'default parameter' js/src/a.js ...
import { readFileSync } from 'node:fs';
import { selfTranslate } from '../js/src/self-translation.js';
import { checkProgram } from '../js/src/translation/check.js';
import { parseJavaScript } from '../js/src/translation/javascript.js';
const [pattern, ...files] = process.argv.slice(2);
for (const file of files) {
  const source = readFileSync(file, 'utf8');
  const bytes = Buffer.from(source, 'utf8');
  for (const item of selfTranslate(source, 'JavaScript', 'Rust').items) {
    if (item.status !== 'carried' || item.term === 'comment') continue;
    const text = bytes.subarray(item.start, item.end).toString('utf8');
    let message = item.reason;
    try { checkProgram(parseJavaScript(text)); } catch (error) { message = String(error.message); }
    if (message.includes(pattern)) console.log(`${file}: ${message.split('\n')[0].slice(0, 120)}\n  ${text.split('\n').slice(0, 3).join('\n  ')}`);
  }
}
