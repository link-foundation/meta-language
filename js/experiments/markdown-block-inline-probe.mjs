// Prints the tree-sitter-markdown block tree so the children of `inline`
// and `pipe_table_cell` nodes (which the inline grammar must skip) are visible.
import { readFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from 'web-tree-sitter';

await Parser.init();
const load = async (id) => Language.load(gunzipSync(await readFile(new URL(`../src/vendor/grammars/${id}.wasm.gz`, import.meta.url))));
const block = await load('markdown');
const source = process.argv[2] ?? '# Title *x*\n\n> Quote with `code`\n> and [link](http://a.b) text.\n\n| a | b |\n|---|---|\n| *c* | d |\n\nText.\n';
const parser = new Parser();
parser.setLanguage(block);
const tree = parser.parse(source);
function show(node, depth) {
  console.log(`${'  '.repeat(depth)}${node.isNamed ? node.type : JSON.stringify(node.type)} ${JSON.stringify(node.text)}`);
  for (const child of node.children) show(child, depth + 1);
}
show(tree.rootNode, 0);
