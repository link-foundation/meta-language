// Probe: every distinct (target, kind, discharge, check) shape of the
// obligations the public translator emits for the four-language corpus.
import { readFileSync } from 'node:fs';
import { translateProgram } from '../js/src/program-translation.js';

const corpusFile = new URL('../parity/fixtures/four-language-conformance.json', import.meta.url);
const corpus = JSON.parse(readFileSync(corpusFile, 'utf8')).translationCorpus;
const directory = new URL(`../${corpus.directory}/`, import.meta.url);
const shapes = new Map();
for (const [source, entry] of Object.entries(corpus.sources)) {
  const text = readFileSync(new URL(entry.file, directory), 'utf8');
  for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    if (target === source) continue;
    const { semantics } = translateProgram(text, source, target);
    for (const { kind, discharge, check } of semantics.obligations) {
      const key = `${source} -> ${target}: ${kind} ${discharge} ${check}`;
      shapes.set(key, (shapes.get(key) ?? 0) + 1);
    }
  }
}
for (const [key, count] of [...shapes].sort()) console.log(`${key} x${count}`);
