// Parses files with a native .lino grammar and prints one s-expression per
// file, the one rust/examples/native_grammar_experiment.rs prints, to compare
// the two runtimes:
//   node experiments/native-lino-sexp.mjs GRAMMAR.lino FILE... > js.txt
//   cargo run --release --example native_grammar_experiment -- GRAMMAR.lino FILE... > rust.txt
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';

const hidden = (kind) => kind === null || kind === undefined || kind.startsWith('_') || kind.startsWith("'");
// Leading trivia leave their node, as native-grammar-parser.js projects them.
const hoist = (node) => {
  if (node.type !== 'node') return [node];
  const children = node.children.flatMap(hoist);
  let first = 0;
  while (first < children.length && children[first].trivia) first += 1;
  return [...children.slice(0, first), { ...node, children: children.slice(first) }];
};
function sexp(root) {
  const parts = [];
  const visit = (child, out) => {
    if (child.type === 'error') { out.push('(ERROR)'); return; }
    if (child.type === 'missing') { out.push(`(MISSING ${child.kind ?? ''})`); return; }
    if (child.type === 'embed') return;
    const inner = [];
    for (const grandchild of child.children ?? []) visit(grandchild, inner);
    if (hidden(child.kind)) { out.push(...inner); return; }
    const field = child.field ? `${child.field}: ` : '';
    out.push(`${field}(${child.kind}${inner.length ? ` ${inner.join(' ')}` : ''})`);
  };
  visit({ ...root, children: root.children.flatMap(hoist) }, parts);
  return parts.join(' ');
}

const [grammarPath, ...files] = process.argv.slice(2);
const compiled = compileGrammar(parseGrammarLinks(readFileSync(grammarPath, 'utf8')));
for (const file of files) {
  const started = performance.now();
  const { tree } = compiled.parseTree(readFileSync(file, 'utf8'), { errorRecovery: true, recovery: 'accept' });
  console.log(tree ? sexp(tree.root ?? tree) : '');
  console.error(`${file}: ${Math.round(performance.now() - started)} ms`);
}
