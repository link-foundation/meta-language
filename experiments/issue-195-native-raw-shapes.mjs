// Prints the named-node shape of a native grammar's raw tree (rule names),
// to compare constructs across native grammars.
//   node experiments/issue-195-native-raw-shapes.mjs json '{"a": [1, true]}'
import { compileGrammar } from '../js/src/grammar.js';
import { parseGrammarLinks } from '../js/src/grammar-links.js';
import { nativeGrammarText } from '../js/src/native-grammar-parser.js';

const [id, source] = process.argv.slice(2);
const { tree } = compileGrammar(parseGrammarLinks(nativeGrammarText(id))).parseTree(source);
const shape = (node) => (node.type === 'node' ? [node.kind, ...node.children.map(shape).filter(Boolean)] : node.kind ? `${node.kind}${node.trivia ? '~' : ''}` : null);
console.log(JSON.stringify(shape(tree)));
