import { writeFileSync } from 'node:fs';
import { compileGrammar, parseNativeGrammar, renderSyntaxTree } from '../js/src/index.js';
const line = 'rule doc = normal repeat0(ref(line))\nrule line = normal seq(token(repeat1(class(range("a", "z")))), token(literal("\\n")))\n';
const grammars = {
  'silent extra': `start doc\nmatching longest\nextra ref(blank)\nextra class(char(" "))\n${line}rule blank = silent token(literal("\\n"))\n`,
  separator: `start doc\nmatching longest\nextra class(char(" "), char("\\n"))\n${line}`,
};
const inputs = ['ab\n', 'ab\n\ncd \n', '\nab\n'];
const cases = Object.entries(grammars).map(([name, grammar]) => ({
  name, grammar,
  parses: inputs.map((input) => ({ input, tree: renderSyntaxTree(compileGrammar(parseNativeGrammar(grammar)).parseTree(input).tree) })),
}));
writeFileSync('parity/fixtures/grammar-token-over-extra.json', `${JSON.stringify({
  description: 'Under (matching longest) a token the parse asks for is lexed over the separator or the token of a silent rule\'s extra its text also matches, as tree-sitter shifts a valid token before it reduces the same text to an extra: each line ends with its own "\\n" token, where "\\n" is also an extra. js/tests/grammar-token-over-extra.test.js and rust/tests/unit/grammar_token_over_extra.rs parse every input to its tree.',
  cases,
}, null, 2)}\n`);
