// Imports a two-token tree-sitter grammar whose external string content is
// scanned by a native scanner, prints the native text and parses a string.
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const grammar = {
  name: 'quoted',
  rules: {
    source: { type: 'REPEAT', content: { type: 'SYMBOL', name: 'string' } },
    string: { type: 'SEQ', members: [{ type: 'STRING', value: '"' }, { type: 'SYMBOL', name: 'string_content' }, { type: 'SYMBOL', name: '_close' }] },
  },
  extras: [{ type: 'PATTERN', value: '\\s' }],
  externals: [{ type: 'SYMBOL', name: 'string_content' }, { type: 'SYMBOL', name: '_close' }],
};
const scanners = '(scanner quotes (tokens string_content _close) (operations (if (valid string_content) (then (while (not (next (literal %22))) (do advance)) (emit string_content))) (if (valid _close) (then (consume (literal %22)) (emit _close))) fail))\n';
const imported = importTreeSitterNative(grammar, { scanners, immediate: ['string_content', '_close'] });
const text = renderTreeSitterNative(imported);
console.log(text);
const outcome = compileGrammar(parseGrammarLinks(text)).parseTree('"a b" "c"');
console.log(outcome.ok, JSON.stringify(outcome.tree, (key, value) => (key === 'attributes' ? undefined : value)).slice(0, 600));
