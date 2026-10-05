// Compares the repair sites of each rejection a native grammar fixture records,
// at a git revision and in the working tree, with the tree-sitter oracle's.
//   node experiments/fixture-rejection-sites.mjs REVISION GRAMMAR...
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { Parser } from '../js/node_modules/web-tree-sitter/web-tree-sitter.js';
import { languageEntry } from '../js/src/index.js';
import { loadGrammarLanguage } from '../js/src/grammar-tiering.js';
import { grammarFile } from '../js/scripts/grammar-files.mjs';

const [revision, ...grammars] = process.argv.slice(2);
await Parser.init();
const lock = JSON.parse(readFileSync(new URL('../js/src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));
const sitesOf = (text) => [...text.matchAll(/(ERROR)@(\d+)\.\.(\d+)|(MISSING)@(\d+)/g)]
  .map((m) => (m[1] ? `E${m[2]}..${m[3]}` : `M${m[5]}`)).join(' ');
const totals = { before: 0, after: 0, count: 0 };
for (const grammar of grammars) {
  const path = `parity/fixtures/native-grammars/${grammar}.json`;
  const now = JSON.parse(readFileSync(path, 'utf8'));
  const then = JSON.parse(execFileSync('git', ['show', `${revision}:${path}`], { encoding: 'utf8' }));
  const entry = languageEntry(now.language);
  const id = (entry.oracleGrammars ?? entry.grammars).map((g) => g.id ?? g).find((name) => lock.grammars[name]);
  const parser = new Parser();
  parser.setLanguage(loadGrammarLanguage(gunzipSync(readFileSync(new URL(`../${grammarFile(lock.grammars[id], `${id}.wasm.gz`)}`, import.meta.url)))));
  const previous = new Map(then.rejections.map((r) => [r.source, r.recovered]));
  for (const { source, recovered } of now.rejections) {
    const tree = parser.parse(source);
    const sites = [];
    const walk = (node) => {
      if (node.isMissing) sites.push(`M${Buffer.byteLength(source.slice(0, node.startIndex))}`);
      else if (node.isError) sites.push(`E${node.startIndex}..${node.endIndex}`);
      node.children.forEach(walk);
    };
    walk(tree.rootNode);
    tree.delete();
    const oracle = sites.join(' ');
    const before = sitesOf(previous.get(source) ?? '');
    const after = sitesOf(recovered);
    totals.count += 1;
    if (before === oracle) totals.before += 1;
    if (after === oracle) totals.after += 1;
    if (before !== after) console.log(JSON.stringify({ grammar, source: source.slice(0, 60), oracle, before, after }));
  }
}
console.log(JSON.stringify(totals));
