// Prints the public CST lines of a source as the default parser of a language
// gives them: node experiments/native-recovery-tree.mjs Rust 'source'
import { LinkNetwork } from '../src/index.js';
import { documentGrammarRoots, renderCstLines } from '../tests/support/cst-lines.js';

const [language, source] = process.argv.slice(2);
const network = LinkNetwork.parse(source, language);
console.log(renderCstLines(documentGrammarRoots(network, language), language).text);
