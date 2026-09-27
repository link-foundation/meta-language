// Writes the draft corpus from issue-195-rename-corpus.mjs into the shared
// four-language fixture as `bindingRenameCorpus`, recording each rename result
// (or rejection) and, for JavaScript, the observation of executing the source.
import { readFileSync, writeFileSync } from 'node:fs';
import { runInNewContext } from 'node:vm';
import { analyzeProgram } from '../js/src/index.js';
import { cases } from './issue-195-rename-corpus.mjs';

const path = new URL('../parity/fixtures/four-language-conformance.json', import.meta.url);
const corpus = JSON.parse(readFileSync(path, 'utf8'));
corpus.bindingRenameCorpus = cases.map(([language, assertions, source, binding, declarationOccurrence, replacement]) => {
  const program = analyzeProgram(source, language);
  const selected = program.bindings.filter(({ name }) => name === binding)[declarationOccurrence];
  let expected = null;
  try {
    expected = program.renameBinding(selected.id, replacement).emit();
  } catch {
    expected = null;
  }
  const entry = { language, assertions, source, binding, declarationOccurrence, replacement, allowed: expected !== null };
  if (expected !== null) entry.expected = expected;
  if (language === 'JavaScript') {
    const context = {};
    runInNewContext(source, context);
    entry.expectedObservation = JSON.parse(JSON.stringify(context.result));
  }
  return entry;
});
// Append (or replace) the section textually so the hand-formatted rest of the
// fixture keeps its layout.
const text = readFileSync(path, 'utf8').replace(/,\n  "bindingRenameCorpus": \[[\s\S]*(?=\n\}\n$)/u, '');
const section = JSON.stringify({ bindingRenameCorpus: corpus.bindingRenameCorpus }, null, 2).slice(2, -2);
writeFileSync(path, text.replace(/\n\}\n$/u, `,\n${section}\n}\n`));
JSON.parse(readFileSync(path, 'utf8'));
console.log(`wrote ${corpus.bindingRenameCorpus.length} binding rename cases`);
