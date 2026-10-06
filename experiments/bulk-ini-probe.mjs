// Prints the native listing of a tree-sitter grammar the bulk pipeline imports
// and where it rejects a sample: node experiments/bulk-ini-probe.mjs ini "text"
import { treeSitterGrammarJson } from '../js/scripts/run-grammar-bulk-pipeline.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../js/src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';

const [id, source] = process.argv.slice(2);
const { text } = await treeSitterGrammarJson(id);
const json = JSON.parse(text);
console.log('extras', JSON.stringify(json.extras), 'externals', JSON.stringify(json.externals));
const imported = importTreeSitterNative(json, { wordRule: 'word_characters' });
const listing = renderTreeSitterNative(imported);
console.log(listing);
const result = compileGrammar(parseGrammarLinks(listing)).parseTree(source.replace(/\\n/gu, '\n'));
console.log(result.tree ? 'accepted' : JSON.stringify(result.rejection));
if (result.tree && process.argv[4]) {
  const { readFileSync } = await import('node:fs');
  const { nativeRows } = await import('../js/scripts/native-grammar-rows.mjs');
  const extras = (json.extras ?? []).filter(({ type }) => type === 'SYMBOL').map(({ name }) => name);
  const oracle = JSON.parse(readFileSync('parity/fixtures/default-cst-expected.json', 'utf8')).languages[process.argv[4]].positive;
  const rows = nativeRows(result.tree, source.replace(/\\n/gu, '\n'), { extras, anonymous: ['unnamed_token'] });
  const width = Math.max(rows.length, oracle.length);
  for (let i = 0; i < width; i += 1) {
    const [a, b] = [JSON.stringify(rows[i]), JSON.stringify(oracle[i])];
    console.log(a === b ? '  ' : '!!', a, a === b ? '' : `| oracle ${b}`);
  }
}
