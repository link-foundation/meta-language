// Prints the checked IR of selected functions of a corpus project.
// Usage: node experiments/translation-ir-dump.mjs <project> <fullName>...
import { readFileSync } from 'node:fs';
import { parseLean } from '../src/translation/lean.js';
import { parseRocq } from '../src/translation/rocq.js';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';

const [source, ...names] = process.argv.slice(2);
const frontends = { lean: parseLean, v: parseRocq, rs: parseRust };
const program = checkProgram(frontends[source.split('.').pop()](readFileSync(source, 'utf8')));
const strip = (key, value) => (key === 'span' ? undefined : value);
for (const name of names) {
  const entry = program.declarations.get(name);
  console.log(name, 'decreasing', entry.decreasing);
  console.log(JSON.stringify(entry.body, strip));
}
