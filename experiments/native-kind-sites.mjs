// Prints where a native parse of each file yields the given named kinds (and
// whether it erred): which corpus sources leak a kind into a downstream
// inventory.
//   node experiments/native-kind-sites.mjs LANGUAGE KIND[,KIND...] FILE...
import { readFileSync } from 'node:fs';
import { parseNative } from '../js/src/native-grammar-parser.js';

const [language, kindList, ...files] = process.argv.slice(2);
const wanted = new Set(kindList.split(','));
for (const file of files) {
  const source = readFileSync(file, 'utf8');
  const started = Date.now();
  const root = parseNative(`native-${language}`, source);
  const sites = [];
  const errors = [];
  const walk = (node) => {
    if (node.named && wanted.has(node.term)) sites.push([node.term, node.start, node.end, source.slice(Math.max(0, node.start - 30), node.end + 30)]);
    if (node.isError || node.isMissing) errors.push([node.term, node.start, node.end, source.slice(Math.max(0, node.start - 40), (node.end ?? node.start) + 40)]);
    node.children.forEach(({ node: child }) => walk(child));
  };
  walk(root);
  console.log(JSON.stringify({ file, bytes: source.length, ms: Date.now() - started, error: root.hasError, sites: sites.slice(0, 5), errors: errors.slice(0, 5) }));
}
