// Runs the reverse conversion check on the commented sources and on every
// shared importer case, printing the status and failures.
import { readFileSync } from 'node:fs';
import { grammarImporter, grammarEmitter } from '../js/src/index.js';
import { checkGrammarReverseConversion } from '../js/src/grammar-reverse.js';
import { SOURCES } from './issue-195-reverse-conversion-sources.mjs';

const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url), 'utf8'));
const inputs = [
  ...Object.entries(SOURCES).map(([format, source]) => ({ id: `${format}:commented`, format, source, accepts: [], rejects: [] })),
  ...corpus.cases,
];
for (const { id, format, source, accepts, rejects } of inputs) {
  try {
    const report = checkGrammarReverseConversion(source, {
      importGrammar: grammarImporter(format), emitGrammar: grammarEmitter(format), accepts, rejects,
    });
    console.log(id, report.status, JSON.stringify(report.failures));
  } catch (error) {
    console.log(id, 'ERROR', error.message);
  }
}
