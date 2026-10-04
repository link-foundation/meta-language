// Runs the native Rust grammar on the corpus cases the oracle recovers from,
// with and without error recovery, and prints the outcome and the time.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { corpusCases, grammarSourceOf } from '../scripts/import-native-grammars.mjs';

const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/rust.lino', import.meta.url), 'utf8')));
const wanted = (process.env.FILE ?? 'error.txt').split(',');
for (const { file, title, source } of corpusCases(grammarSourceOf('native-rust')).filter(({ file }) => wanted.includes(file))) {
  for (const options of [{}, { errorRecovery: true }]) {
    const started = performance.now();
    const outcome = parser.parseTree(source, options);
    console.log(`${file}: ${title} ${JSON.stringify(options)} ok=${outcome.ok} reason=${outcome.rejection?.reason} message=${outcome.rejection?.message ?? ''} (${Math.round(performance.now() - started)} ms)`);
  }
  console.log(JSON.stringify(source));
}
