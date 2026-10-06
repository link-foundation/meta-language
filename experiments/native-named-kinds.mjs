// Prints the named kinds of a native parse of each source argument, with the
// text each first covers: a probe for kinds that leak into downstream
// inventories (formal-ai's grammar_projection_corpus_ratchet).
//   node experiments/native-named-kinds.mjs LANGUAGE SOURCE...
import { parseNative } from '../js/src/native-grammar-parser.js';

const [language, ...sources] = process.argv.slice(2);
for (const source of sources) {
  const kinds = new Map();
  const walk = (node) => {
    if (node.named && !kinds.has(node.term)) kinds.set(node.term, source.slice(node.start ?? node.span?.start ?? 0, (node.end ?? node.span?.end ?? 0)).slice(0, 40));
    node.children.forEach(({ node: child }) => walk(child));
  };
  const root = parseNative(`native-${language}`, source);
  walk(root);
  console.log(JSON.stringify(source), root.hasError ? 'ERROR' : 'ok', JSON.stringify([...kinds]));
}
