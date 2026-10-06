// Prints the named nodes of the built-in LiNo CST with their fields.
import { parseLinoCst } from '../js/src/lino-grammar.js';
function show(node, text, depth = 0, field = '') {
  if (!node.named && node.term !== 'ERROR') return;
  if (node.term === 'comment') return;
  console.log(`${'  '.repeat(depth)}${field ? `${field}: ` : ''}${node.term} ${JSON.stringify(text.slice(node.start, node.end))}`);
  for (const child of node.children ?? []) show(child, text, depth + 1, child.field ?? '');
}
for (const src of process.argv.slice(2).map((s) => JSON.parse(`"${s}"`))) {
  const tree = parseLinoCst(src);
  if (process.env.RAW) console.log(Object.keys(tree), Object.keys(tree.children[0] ?? {}));
  show(tree, src);
}
