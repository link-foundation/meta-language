// Parses a language's inventory source and recovery source with a native
// grammar and compares the rows with the tree-sitter oracle, with timings.
//   node experiments/native-inventory-source.mjs LANGUAGE GRAMMAR.lino
// Options from the environment as in native-corpus-compare.mjs.
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { renderSyntaxTree } from '../src/index.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { hasRecovery, nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const [language, grammarPath] = process.argv.slice(2);
const list = (name) => (process.env[name] ? process.env[name].split(',') : []);
const text = readFileSync(grammarPath, 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: list('HIDDEN'), anonymous: list('ANONYMOUS'), extras: list('EXTRAS'), oracleKinds: nativeOracleKinds(text) };
const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const entry = inventory.languages.find(({ name }) => name === language);
let started = performance.now();
const positive = compiled.parseTree(entry.source);
console.log('positive', positive.ok, `${Math.round(performance.now() - started)} ms`);
if (positive.ok) {
  const same = JSON.stringify(nativeRows(positive.tree, entry.source, options)) === JSON.stringify(oracleRows(entry.source, language));
  console.log('rows equal the oracle:', same);
}
console.log('oracle recovers from the recovery source:', oracleRecovers(entry.recoverySource, language));
started = performance.now();
const repaired = compiled.parseTree(entry.recoverySource, { errorRecovery: true, recovery: 'accept' });
console.log('recovery', repaired.ok, repaired.rejection?.reason, `${Math.round(performance.now() - started)} ms`);
if (repaired.tree) {
  console.log(renderSyntaxTree(repaired.tree));
  console.log('has ERROR or MISSING:', hasRecovery(nativeRows(repaired.tree, entry.recoverySource, options)));
}
