// Shows how a grammar's declared `settling` decides two parses of one text.
import { compileGrammar, parseNativeGrammar, renderSyntaxTree } from '../js/src/index.js';

const body = `rule s = normal choice(ref(a), ref(b))
rule a = normal literal("x")
rule b = normal dynamicPrecedence(1, literal("x"))
`;
for (const settling of [null, 'dynamic ambiguity', 'ambiguity', 'first', 'dynamic first']) {
  const listing = `start s\n${settling ? `settling ${settling}\n` : ''}${body}`;
  const result = compileGrammar(parseNativeGrammar(listing)).parseTree('x');
  console.log(String(settling).padEnd(18), result.tree ? renderSyntaxTree(result.tree) : '-', JSON.stringify(result.ambiguities));
}

// Under `(matching longest)` the `tokens` step prefers the tokens a lexer
// would lex: a literal over a token rule's pattern (a keyword over a name).
const lexed = `rule s = normal choice(ref(keyword), ref(name))
rule keyword = normal literal("if")
rule name = token repeat1(range("a", "z"))
`;
for (const settling of [null, 'tokens precedence dynamic ambiguity', 'precedence dynamic ambiguity', 'dynamic first']) {
  const listing = `start s\nmatching longest\n${settling ? `settling ${settling}\n` : ''}${lexed}`;
  const result = compileGrammar(parseNativeGrammar(listing)).parseTree('if');
  console.log(String(settling).padEnd(38), result.tree ? renderSyntaxTree(result.tree) : '-', JSON.stringify(result.ambiguities));
}
