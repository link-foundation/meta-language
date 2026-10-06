// Parses sample files with a native .lino grammar and compares its projected
// rows (native-grammar-rows.mjs) with the rows of the language's tree-sitter
// oracle, the comparison the native grammar fixtures make.
//   node experiments/native-lino-rows-compare.mjs LANGUAGE GRAMMAR.lino SAMPLE...
// Options come from the environment: EXTRAS=comment HIDDEN= ANONYMOUS=.
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRows } from '../scripts/native-grammar-rows.mjs';

const [language, grammarPath, ...samples] = process.argv.slice(2);
const list = (name) => (process.env[name] ? process.env[name].split(',') : []);
const text = readFileSync(grammarPath, 'utf8');
let started = performance.now();
const compiled = compileGrammar(parseGrammarLinks(text));
console.log(`compiled in ${Math.round(performance.now() - started)} ms`);
const options = { hidden: list('HIDDEN'), anonymous: list('ANONYMOUS'), extras: list('EXTRAS'), oracleKinds: nativeOracleKinds(text) };

let agreed = 0;
for (const sample of samples) {
  const source = readFileSync(sample, 'utf8');
  const oracle = oracleRows(source, language);
  started = performance.now();
  let native;
  try {
    const { tree } = compiled.parseTree(source, { errorRecovery: true, recovery: 'accept' });
    native = nativeRows(tree.root ?? tree, source, options);
  } catch (error) {
    native = [[`THROWN ${error.message}`]];
  }
  const ms = Math.round(performance.now() - started);
  const at = oracle.findIndex((row, index) => JSON.stringify(row) !== JSON.stringify(native[index]));
  const same = at === -1 && oracle.length === native.length;
  if (same) agreed += 1;
  console.log(`${same ? 'SAME' : 'DIFF'} ${sample} (${source.length} chars, ${oracle.length} rows, ${ms} ms)`);
  if (!same) {
    const first = at === -1 ? oracle.length : at;
    for (let index = Math.max(0, first - 2); index < first + 4; index += 1) {
      console.log(`  ${index} oracle ${JSON.stringify(oracle[index])}  native ${JSON.stringify(native[index])}`);
    }
  }
}
console.log(`${agreed}/${samples.length} agree`);
