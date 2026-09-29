// Prints a JavaScript program's translation into each target, or its diagnostic.
// Usage: node experiments/translate-show.mjs <program.mjs> [Target...]
import { readFileSync } from 'node:fs';
import { translateProgram } from '../js/src/program-translation.js';

const [file, ...targets] = process.argv.slice(2);
const source = readFileSync(file, 'utf8');
for (const target of targets.length ? targets : ['Rust', 'Lean', 'Rocq']) {
  const result = translateProgram(source, 'JavaScript', target);
  console.log(`===== ${target}`);
  console.log(result.diagnostic ? `${result.diagnostic.kind}: ${result.diagnostic.message}` : result.code);
}
