#!/usr/bin/env node
// Records what the JavaScript translation pipeline produces at every stage
// for the translation corpus and the translation case sets. A successful
// stage is recorded as the SHA-256 of its canonical JSON (keys sorted), a
// failed one as its error kind and message. The Rust test
// `tests/unit/translation_stages.rs` runs the Rust pipeline on the same
// sources and requires the same digests and errors, so the two runtimes stay
// in parity stage by stage.
// Usage: node scripts/build-translation-stage-fixtures.mjs [--check] [--only name1,name2]
import { readdirSync, readFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

import { CORPUS_DIRECTORY, digest, loadCases, runStages } from './translation-stage-lib.mjs';
import { readProjectData, readProjectFileBytes, serializeProjectData, writeProjectData } from './project-data.mjs';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const output = join(root, 'parity', 'fixtures', 'translation-stages.json');
const onlyIndex = process.argv.indexOf('--only');
const onlyValue = onlyIndex < 0 ? null : process.argv[onlyIndex + 1];
if (onlyIndex >= 0 && (!onlyValue || onlyValue.startsWith('--'))) throw new Error('--only needs exact comma-separated program names');
const selected = onlyValue === null ? null : new Set(onlyValue.split(','));
if (selected?.has('')) throw new Error('--only program names cannot be empty');
const previousPrograms = selected ? readProjectData(output).programs : null;
const previousByName = new Map((previousPrograms ?? []).map(entry => [entry.name, entry]));

const record = (result) => (result.error ? { error: result.error } : { sha256: digest(result.value) });

function program(entry, extension, text) {
  if (selected && !selected.has(entry.name)) {
    const previous = previousByName.get(entry.name);
    if (!previous) throw new Error(`unselected translation program is new: ${entry.name}`);
    const { parse, check, emit, ...descriptor } = previous;
    if (JSON.stringify(descriptor) !== JSON.stringify(entry)) throw new Error(`unselected translation source or metadata changed: ${entry.name}`);
    return previous;
  }
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
if (selected) {
  for (const name of selected) if (!names.has(name)) throw new Error(`unknown selected translation program: ${name}`);
  if (programs.length !== previousPrograms.length || programs.some((entry, index) => entry.name !== previousPrograms[index].name)) {
    throw new Error('a selected refresh must preserve every program and its original order');
  }
}

const fixture = {
  generator: 'js/scripts/build-translation-stage-fixtures.mjs',
  programs,
};
const text = serializeProjectData(fixture, output);

const count = (predicate) => programs.filter(predicate).length;
const summary = `${programs.length} programs: ${count((entry) => entry.parse.error)} rejected by a frontend, ${count((entry) => entry.check?.error)} by the checker, ${count((entry) => entry.emit)} emitted`;
const selection = selected ? `; refreshed only ${[...selected].join(', ')}` : '';

if (process.argv.includes('--check')) {
  let committed = '';
  try {
    committed = readProjectFileBytes(output).toString('utf8');
  } catch {
    // A missing fixture is stale.
  }
  if (committed !== text) {
    console.error('translation stage fixtures are stale; run `node js/scripts/build-translation-stage-fixtures.mjs`');
    if (committed) {
      const previous = readProjectData(output);
      const before = new Map(previous.programs.map((entry) => [entry.name, entry]));
      const changed = programs.filter((entry) => JSON.stringify(before.get(entry.name)) !== JSON.stringify(entry));
      for (const entry of changed.slice(0,10)) {
        const old = before.get(entry.name);
        const stages = ['parse', 'check', 'emit'].filter((stage) => JSON.stringify(old?.[stage]) !== JSON.stringify(entry[stage]));
        console.error(`changed translation fixture: ${entry.name} (${stages.join(', ') || 'source or metadata'})`);
      }
      if (changed.length > 10) console.error(`and ${changed.length - 10} more changed translation fixtures`);
    }
    process.exit(1);
  }
  console.log(`translation stage fixtures match the JavaScript pipeline for ${summary}${selection}`);
} else {
  writeProjectData(output, fixture);
  console.log(`wrote translation stage fixtures for ${summary}${selection}`);
}
