// Prints the checked IR of one function of a corpus project, without spans.
// Usage: node experiments/translation-dump-fn.mjs <project> <Full.name>
import { readFileSync } from 'node:fs';
import { parseLean } from '../src/translation/lean.js';
import { parseRocq } from '../src/translation/rocq.js';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';

const [source, name] = process.argv.slice(2);
const frontends = { lean: parseLean, v: parseRocq, rs: parseRust, mjs: parseJavaScript, js: parseJavaScript };
const program = checkProgram(frontends[source.split('.').pop()](readFileSync(source, 'utf8')));
const entry = program.declarations.get(name);
console.log(JSON.stringify({ params: entry.params, ret: entry.ret, decreasing: entry.decreasing, body: entry.body },
  (key, value) => (key === 'span' ? undefined : value), 1));
