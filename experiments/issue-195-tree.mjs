// Usage: node experiments/issue-195-tree.mjs <Language> <source-file | -e source>
// Prints the containment tree the project-aware analysis builds from a
// program's source mappings, with each node's exact text.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../js/src/index.js';
import { analyzeProgram } from '../js/src/program-representation.js';
import { syntaxTree } from '../js/src/program-project-tree.js';

const [language, flag, value] = process.argv.slice(2);
const source = flag === '-e' ? value.replaceAll('\\n', '\n') : readFileSync(flag, 'utf8');
const mappings = language === 'JSON'
  ? LinkNetwork.parse(source, 'JSON').links().filter((l) => l.metadata().span && l.metadata().linkType === 'Syntax')
    .map((l) => ({ term: l.metadata().term, start: l.metadata().span.byteRange.start, end: l.metadata().span.byteRange.end }))
  : analyzeProgram(source, language).sourceMappings;
const tree = syntaxTree(mappings, source);
const print = (node, depth) => {
  console.log(`${'  '.repeat(depth)}${node.term} [${node.start},${node.end}] ${JSON.stringify(source.slice(node.start, node.end)).slice(0, 60)}`);
  node.children.forEach((child) => print(child, depth + 1));
};
tree.roots.forEach((root) => print(root, 0));
