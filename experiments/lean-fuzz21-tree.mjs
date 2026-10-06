// Prints the public CST of the Lean generative case fuzz/21 (`:f f : Foo where\n  bar :=⟩`).
import { LinkNetwork } from '../js/src/index.js';
import { documentGrammarRoots, renderCstLines } from '../js/tests/support/cst-lines.js';
const source = ':f f : Foo where\n  bar :=⟩';
const network = LinkNetwork.parse(source, 'Lean');
console.log(renderCstLines(documentGrammarRoots(network, 'Lean'), 'Lean').text);
