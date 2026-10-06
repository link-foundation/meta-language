import { compileGrammar, parseGrammarLinks, renderSyntaxTree, renderNativeGrammar } from '../js/src/index.js';
const silent = `(grammar (start doc) (matching longest))
(extra (ref blank))
(extra (class plain (char %20)))
(rule doc normal (repeat0 (ref line)))
(rule line normal (seq (token (repeat1 (class plain (range a z)))) (token (literal %0A))))
(rule blank silent (token (literal %0A)))
`;
const separator = `(grammar (start doc) (matching longest))
(extra (class plain (char %20) (char %0A)))
(rule doc normal (repeat0 (ref line)))
(rule line normal (seq (token (repeat1 (class plain (range a z)))) (token (literal %0A))))
`;
for (const [name, text] of [['silent', silent], ['separator', separator]]) {
  const g = parseGrammarLinks(text);
  console.log(name, JSON.stringify(renderNativeGrammar(g)));
  for (const input of ['ab\n', 'ab\n\ncd \n', '\nab\n']) {
    const r = compileGrammar(g).parseTree(input);
    console.log(JSON.stringify(input), r.tree ? renderSyntaxTree(r.tree) : JSON.stringify(r.rejection));
  }
}
