import { parsePdfCst } from '../src/pdf-grammar.js';

const source = process.argv[2] ?? '%PDF-1.7\n%%EOF\n';
const text = source.replace(/\\n/g, '\n');
function dump(node, depth = 0) {
  const flags = [node.isError && 'ERROR', node.isMissing && 'MISSING', node.hasError && 'hasError', node.extra && 'extra']
    .filter(Boolean).join(',');
  console.log(`${'  '.repeat(depth)}${node.field ? `${node.field}: ` : ''}${node.named ? node.term : JSON.stringify(node.term)} [${node.start},${node.end}] ${flags} ${node.children.length ? '' : JSON.stringify(text.slice(node.start, node.end))}`);
  for (const child of node.children) dump(child, depth + 1);
}
dump(parsePdfCst(text));
