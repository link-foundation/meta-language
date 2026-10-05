// Prints the canonical CST lines of the public tree of a source, as the
// conformance and generative suites compare them with the oracle's.
//   node experiments/native-cst-lines.mjs LANGUAGE 'source'   (JSON-escaped with -j)
import { LinkNetwork } from '../src/index.js';
import { documentGrammarRoots, renderCstLines } from '../tests/support/cst-lines.js';

const [language, ...rest] = process.argv.slice(2);
const source = rest[0] === '-j' ? JSON.parse(rest[1]) : rest[0];
const network = LinkNetwork.parse(source, language);
console.log(renderCstLines(documentGrammarRoots(network, language), language).text);
console.log('clean:', network.verifyFullMatch().isClean());
