// Runs the JavaScript translation pipeline stage by stage and returns every
// stage's result in the JSON shape the Rust port serialises: bigints as
// decimal strings, the checked program's declarations as an array and its
// items as `{ k: 'decl', fullName }` references. A stage that throws records
// its error kind and message, and the later stages are not run.
import { readdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { createHash } from 'node:crypto';

import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { parseLean } from '../src/translation/lean.js';
import { parseRocq } from '../src/translation/rocq.js';
import { parseRust } from '../src/translation/rust.js';

export const EMITTERS = { javascript: emitJavaScript, rust: emitRust, lean: emitLean, rocq: emitRocq };
export const FRONTENDS = { lean: parseLean, v: parseRocq, rs: parseRust, mjs: parseJavaScript, js: parseJavaScript };

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
export const CORPUS_DIRECTORY = join(root, 'parity', 'fixtures', 'translation-corpus');
export const CASES_DIRECTORY = join(root, 'js', 'tests', 'fixtures', 'translation-cases');

const portable = (value) => JSON.parse(JSON.stringify(value, (key, item) => (typeof item === 'bigint' ? item.toString() : item)));

const portableProgram = (program) => {
  const items = (list) => list.map((item) => (item.k === 'module'
    ? { ...item, items: items(item.items) }
    : { k: 'decl', fullName: item.fullName }));
  return portable({ ...program, items: items(program.items), declarations: [...program.declarations.values()] });
};

const failure = (error) => ({ error: { kind: error.kind ?? 'crash', message: error.message } });

/**
 * Every stage's result for a source text in the language its extension names.
 * @param {string} extension
 * @param {string} text
 */
export function runStages(extension, text) {
  const stages = {};
  let surface;
  try {
    surface = FRONTENDS[extension](text);
    stages.parse = { value: portable(surface) };
  } catch (error) {
    stages.parse = failure(error);
    return stages;
  }
  let program;
  try {
    program = checkProgram(surface);
    stages.check = { value: portableProgram(program) };
  } catch (error) {
    stages.check = failure(error);
    return stages;
  }
  stages.emit = {};
  for (const [target, emit] of Object.entries(EMITTERS)) {
    try {
      stages.emit[target] = { value: portable(emit(program)) };
    } catch (error) {
      stages.emit[target] = failure(error);
    }
  }
  return stages;
}

/** JSON with object keys sorted, so equal values have one spelling. */
export function canonicalJson(value) {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(',')}]`;
  if (value && typeof value === 'object') {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalJson(value[key])}`).join(',')}}`;
  }
  return JSON.stringify(value);
}

export const digest = (value) => createHash('sha256').update(canonicalJson(value)).digest('hex');

/**
 * The translation case sets, as `{ set, name, extension, source }` in a stable order.
 */
export async function loadCases() {
  const cases = [];
  for (const file of readdirSync(CASES_DIRECTORY).filter((name) => name.endsWith('.mjs')).sort()) {
    const set = file.replace(/\.mjs$/u, '');
    const { default: entries } = await import(pathToFileURL(join(CASES_DIRECTORY, file)).href);
    for (const [name, [extension, source]] of Object.entries(entries)) cases.push({ set, name, extension, source });
  }
  return cases;
}
