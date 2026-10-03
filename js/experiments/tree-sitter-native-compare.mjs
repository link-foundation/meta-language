// Converts a tree-sitter grammar.json with tree-sitter-native-convert.mjs,
// parses sample files with the native executor and with the grammar's
// tree-sitter oracle, and compares the two s-expressions.
//   node experiments/tree-sitter-native-compare.mjs ID GRAMMAR.json SAMPLE...
// ID names the oracle build: src/vendor/grammars/ID.wasm.gz or
// oracles/grammars/ID.wasm.gz.
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from 'web-tree-sitter';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { convertTreeSitterGrammar } from './tree-sitter-native-convert.mjs';

const [id, grammarPath, ...samples] = process.argv.slice(2);
const grammar = JSON.parse(readFileSync(grammarPath, 'utf8'));
const { text, report, keywords } = convertTreeSitterGrammar(grammar);
writeFileSync(`/tmp/${id}-native.lino`, text);
console.log(`converted ${Object.keys(grammar.rules).length} rules, ${keywords.length} keywords; approximations ${report.approximations.length}, unsupported ${report.unsupported.length}`);
for (const line of report.unsupported) console.log(`  unsupported: ${line}`);

let started = performance.now();
const compiled = compileGrammar(parseGrammarLinks(text));
console.log(`compiled in ${Math.round(performance.now() - started)} ms`);

await Parser.init({ wasmBinary: gunzipSync(readFileSync(new URL('../src/vendor/web-tree-sitter/web-tree-sitter.wasm.gz', import.meta.url))) });
const wasmPath = [new URL(`../src/vendor/grammars/${id}.wasm.gz`, import.meta.url), new URL(`../oracles/grammars/${id}.wasm.gz`, import.meta.url)]
  .find((url) => existsSync(url));
const parser = new Parser();
parser.setLanguage(await Language.load(gunzipSync(readFileSync(wasmPath))));

const hidden = (kind) => kind === null || kind === undefined || kind.startsWith('_') || kind.startsWith("'");
// Leading trivia leave their node, as native-grammar-parser.js projects them.
const hoist = (node) => {
  if (node.type !== 'node') return [node];
  const children = node.children.flatMap(hoist);
  let first = 0;
  while (first < children.length && children[first].trivia) first += 1;
  return [...children.slice(0, first), { ...node, children: children.slice(first) }];
};
function sexp(node) {
  node = { ...node, children: node.children.flatMap(hoist) };
  const parts = [];
  const visit = (child, out) => {
    if (child.type === 'error') { out.push('(ERROR)'); return; }
    if (child.type === 'missing') { out.push(`(MISSING ${child.kind ?? ''})`); return; }
    const inner = [];
    for (const grandchild of child.children ?? []) visit(grandchild, inner);
    if (hidden(child.kind)) { out.push(...inner); return; }
    const field = child.field ? `${child.field}: ` : '';
    out.push(`${field}(${child.kind}${inner.length ? ` ${inner.join(' ')}` : ''})`);
  };
  visit(node, parts);
  return parts.join(' ');
}

let agreed = 0;
for (const sample of samples) {
  const source = readFileSync(sample, 'utf8');
  const oracle = parser.parse(source).rootNode.toString();
  started = performance.now();
  let native;
  try {
    const { tree } = compiled.parseTree(source, { errorRecovery: true, recovery: 'accept' });
    native = sexp(tree.root ?? tree);
  } catch (error) {
    native = `THROWN ${error.message}`;
  }
  const ms = Math.round(performance.now() - started);
  const same = native === oracle;
  if (same) agreed += 1;
  console.log(`${same ? 'SAME' : 'DIFF'} ${sample} (${source.length} chars, ${ms} ms)`);
  if (!same) {
    let at = 0;
    while (at < oracle.length && oracle[at] === native[at]) at += 1;
    const from = Math.max(0, at - 300);
    console.log(`  oracle: …${oracle.slice(from, at + 300)}`);
    console.log(`  native: …${native.slice(from, at + 300)}`);
  }
}
console.log(`${agreed}/${samples.length} agree`);
