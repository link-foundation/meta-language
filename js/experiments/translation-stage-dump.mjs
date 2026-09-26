// Writes the surface AST and the checked IR of a program as JSON, in the
// shape the Rust port deserialises: declarations as an array and program
// items as `{ k: 'decl', fullName }` references. The Rust checker's output is
// compared with these dumps stage by stage. Each target's emitted text and
// contract is written to `emit-<target>.json`, or its failure to
// `emit-<target>.error.json`.
// Usage: node experiments/translation-stage-dump.mjs <source> <outdir>
import { mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { runStages } from '../scripts/translation-stage-lib.mjs';

const [source, outdir] = process.argv.slice(2);
mkdirSync(outdir, { recursive: true });
// Outputs of an earlier run would pass for this run's stages.
for (const name of readdirSync(outdir)) if (name.endsWith('.json')) rmSync(join(outdir, name));

const write = (name, value) => writeFileSync(join(outdir, name), `${JSON.stringify(value, null, 1)}\n`);
const stages = runStages(source.split('.').pop(), readFileSync(source, 'utf8'));
for (const [stage, file] of [['parse', 'surface.json'], ['check', 'ir.json']]) {
  if (!stages[stage]) break;
  if (stages[stage].error) write('error.json', { stage, ...stages[stage].error });
  else write(file, stages[stage].value);
}
for (const [target, result] of Object.entries(stages.emit ?? {})) {
  if (result.error) write(`emit-${target}.error.json`, { stage: 'emit', ...result.error });
  else write(`emit-${target}.json`, result.value);
}
