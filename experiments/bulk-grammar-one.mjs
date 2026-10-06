// Experiment: import one locked tree-sitter grammar.json natively, execute it on
// the inventory's positive source and compare with the oracle rows.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { importTreeSitterNative, renderTreeSitterNative } from '../js/src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';
import { nativeRows } from '../js/scripts/native-grammar-rows.mjs';
import { cargoLockVersions, grammarSource } from '../js/scripts/build-vendored-grammars.mjs';

const [language, id] = process.argv.slice(2);
const root = new URL('..', import.meta.url).pathname;
const inventory = JSON.parse(readFileSync(join(root, 'parity/language-grammar-inventory.json'), 'utf8'));
const expected = JSON.parse(readFileSync(join(root, 'parity/fixtures/default-cst-expected.json'), 'utf8'));
const entry = inventory.languages.find((l) => l.name === language);
const source = await grammarSource(id, await cargoLockVersions());
const grammarJson = JSON.parse(readFileSync(join(source.crateDir, source.dir ?? '.', 'src/grammar.json'), 'utf8'));
let t = performance.now();
const imported = importTreeSitterNative(grammarJson, { wordRule: 'word_characters' });
console.log('import ms', Math.round(performance.now() - t), 'rules', imported.rules.length, 'unsupported', imported.report.unsupported.length, imported.report.unsupported.slice(0, 5));
const text = renderTreeSitterNative(imported);
console.log(text.split('\n')[0]);
t = performance.now();
const grammar = parseGrammarLinks(text);
const compiled = compileGrammar(grammar);
console.log('compile ms', Math.round(performance.now() - t));
t = performance.now();
const result = compiled.parseTree(entry.source);
console.log('rejection', JSON.stringify(result.rejection)?.slice(0,400));
console.log('parse ms', Math.round(performance.now() - t), 'tree', Boolean(result.tree), 'diagnostics', (result.diagnostics ?? []).length);
if (result.tree) {
  const rows = nativeRows(result.tree, entry.source, { extras: (grammarJson.extras ?? []).filter((x) => x.type === 'SYMBOL').map((x) => x.name) });
  const oracle = expected.languages[language].positive;
  console.log('rows', rows.length, 'oracle', oracle.length, 'match', JSON.stringify(rows) === JSON.stringify(oracle));
}
