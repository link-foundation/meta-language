// Parses the draft inventory sources with the raw vendored grammars (no
// adapter) and reports whether positive sources are clean and recovery sources
// contain ERROR or MISSING nodes.
//   node experiments/validate-default-cst-drafts.mjs [Language ...] [--tree]
import { readFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from 'web-tree-sitter';

import { DRAFTS } from './default-cst-source-drafts.mjs';

const args = process.argv.slice(2);
const showTree = args.includes('--tree');
const only = args.filter((arg) => !arg.startsWith('--'));
await Parser.init();
const grammars = new Map();
async function grammar(id) {
  if (!grammars.has(id)) {
    const url = new URL(`../src/vendor/grammars/${id}.wasm.gz`, import.meta.url);
    grammars.set(id, await Language.load(gunzipSync(await readFile(url))));
  }
  return grammars.get(id);
}

function problems(node, out = []) {
  if (node.isError || node.isMissing) out.push(`${node.isMissing ? 'MISSING' : 'ERROR'} ${node.type} ${node.startIndex}-${node.endIndex}`);
  for (const child of node.children) problems(child, out);
  return out;
}

function dump(node, depth = 0, field = null) {
  const label = node.isNamed ? node.type : JSON.stringify(node.type);
  console.log(`${'  '.repeat(depth)}${field ? `${field}: ` : ''}${label}${node.isExtra ? ' [extra]' : ''} ${node.startIndex}-${node.endIndex}`);
  node.children.forEach((child, index) => dump(child, depth + 1, node.fieldNameForChild(index)));
}

for (const [name, draft] of Object.entries(DRAFTS)) {
  if (only.length && !only.includes(name)) continue;
  const parser = new Parser();
  parser.setLanguage(await grammar(draft.grammar));
  for (const kind of ['source', 'recovery']) {
    const tree = parser.parse(draft[kind]);
    const found = problems(tree.rootNode);
    const ok = kind === 'source' ? found.length === 0 : found.length > 0;
    console.log(`${ok ? 'ok  ' : 'FAIL'} ${name} ${kind}: ${found.join('; ') || 'clean'}`);
    if (showTree) dump(tree.rootNode);
    tree.delete();
  }
  parser.delete();
}
