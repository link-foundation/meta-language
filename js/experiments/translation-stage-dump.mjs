// Writes the surface AST and the checked IR of a program as JSON, in the
// shape the Rust port deserialises: declarations as an array and program
// items as `{ k: 'decl', fullName }` references. The Rust checker's output is
// compared with these dumps stage by stage. Each target's emitted text and
// contract is written to `emit-<target>.json`, or its failure to
// `emit-<target>.error.json`.
// Usage: node experiments/translation-stage-dump.mjs <source> <outdir>
import { mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { parseLean } from '../src/translation/lean.js';
import { parseRocq } from '../src/translation/rocq.js';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { emitRust } from '../src/translation/emit-rust.js';

const emitters = { javascript: emitJavaScript, rust: emitRust, lean: emitLean, rocq: emitRocq };
const frontends = { lean: parseLean, v: parseRocq, rs: parseRust, mjs: parseJavaScript, js: parseJavaScript };
const [source, outdir] = process.argv.slice(2);
mkdirSync(outdir, { recursive: true });
// Outputs of an earlier run would pass for this run's stages.
for (const name of readdirSync(outdir)) if (name.endsWith('.json')) rmSync(join(outdir, name));

const json = (value) => JSON.stringify(value, (key, item) => (typeof item === 'bigint' ? item.toString() : item), 1);
const portableProgram = (program) => {
  const items = (list) => list.map((item) => (item.k === 'module'
    ? { ...item, items: items(item.items) }
    : { k: 'decl', fullName: item.fullName }));
  return { ...program, items: items(program.items), declarations: [...program.declarations.values()] };
};

const write = (name, value) => writeFileSync(join(outdir, name), `${json(value)}\n`);
const failure = (stage, error) => write('error.json', { stage, kind: error.kind ?? 'crash', message: error.message });
let surface;
try {
  surface = frontends[source.split('.').pop()](readFileSync(source, 'utf8'));
  write('surface.json', surface);
} catch (error) {
  failure('parse', error);
  process.exit(0);
}
let program;
try {
  program = checkProgram(surface);
  write('ir.json', portableProgram(program));
} catch (error) {
  failure('check', error);
  process.exit(0);
}
for (const [target, emit] of Object.entries(emitters)) {
  try {
    write(`emit-${target}.json`, emit(program));
  } catch (error) {
    write(`emit-${target}.error.json`, { stage: 'emit', kind: error.kind ?? 'crash', message: error.message });
  }
}
