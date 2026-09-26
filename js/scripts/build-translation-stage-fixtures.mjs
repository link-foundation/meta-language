#!/usr/bin/env node
// Records what the JavaScript translation pipeline produces at every stage
// for the translation corpus and the translation case sets. A successful
// stage is recorded as the SHA-256 of its canonical JSON (keys sorted), a
// failed one as its error kind and message. The Rust test
// `tests/unit/translation_stages.rs` runs the Rust pipeline on the same
// sources and requires the same digests and errors, so the two runtimes stay
// in parity stage by stage.
// Usage: node scripts/build-translation-stage-fixtures.mjs [--check]
import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

import { CORPUS_DIRECTORY, digest, loadCases, runStages } from './translation-stage-lib.mjs';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const output = join(root, 'parity', 'fixtures', 'translation-stages.json');

const record = (result) => (result.error ? { error: result.error } : { sha256: digest(result.value) });

function program(entry, extension, text) {
  const stages = runStages(extension, text);
  const result = { ...entry, parse: record(stages.parse) };
  if (stages.check) result.check = record(stages.check);
  if (stages.emit) {
    result.emit = Object.fromEntries(Object.entries(stages.emit).map(([target, value]) => [target, record(value)]));
  }
  return result;
}

const programs = [];
for (const file of readdirSync(CORPUS_DIRECTORY).filter((name) => /\.(lean|v|rs|mjs)$/u.test(name)).sort()) {
  const path = join(CORPUS_DIRECTORY, file);
  const extension = file.split('.').pop();
  programs.push(program(
    { name: `corpus/${file}`, extension, path: relative(root, path).split('\\').join('/') },
    extension,
    readFileSync(path, 'utf8'),
  ));
}
for (const { set, name, extension, source } of await loadCases()) {
  programs.push(program({ name: `${set}/${name}`, extension, source }, extension, source));
}

const names = new Set();
for (const { name } of programs) {
  if (names.has(name)) throw new Error(`duplicate translation stage program ${name}`);
  names.add(name);
}

const text = `${JSON.stringify({
  generator: 'js/scripts/build-translation-stage-fixtures.mjs',
  programs,
}, null, 2)}\n`;

const count = (predicate) => programs.filter(predicate).length;
const summary = `${programs.length} programs: ${count((entry) => entry.parse.error)} rejected by a frontend, ${count((entry) => entry.check?.error)} by the checker, ${count((entry) => entry.emit)} emitted`;

if (process.argv.includes('--check')) {
  let committed = '';
  try {
    committed = readFileSync(output, 'utf8');
  } catch {
    // A missing fixture is stale.
  }
  if (committed !== text) {
    console.error('translation stage fixtures are stale; run `node js/scripts/build-translation-stage-fixtures.mjs`');
    process.exit(1);
  }
  console.log(`translation stage fixtures match the JavaScript pipeline for ${summary}`);
} else {
  writeFileSync(output, text);
  console.log(`wrote translation stage fixtures for ${summary}`);
}
