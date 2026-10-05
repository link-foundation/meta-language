// Prints the recovered native JavaScript tree of SOURCE (argv[2]), one node a line.
import { compileGrammar } from '../../js/src/grammar.js';
import { parseGrammarLinks } from '../../js/src/grammar-links.js';
import { nativeGrammarText } from '../../js/src/native-grammar-parser.js';

const id = process.argv[3] ?? 'native-javascript';
const parser = compileGrammar(parseGrammarLinks(nativeGrammarText(id)));
const source = process.argv[2];
const outcome = parser.parseTree(source, { errorRecovery: true, recovery: 'accept' });
const walk = (node, depth) => {
  console.log(`${'  '.repeat(depth)}${node.type}:${node.kind ?? ''}@${node.start}-${node.end} ${JSON.stringify(source.slice(node.start, node.end)).slice(0, 40)}`);
  for (const child of node.children ?? []) walk(child, depth + 1);
};
walk(outcome.tree, 0);
