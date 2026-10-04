// Under `(matching longest)` a token the parse asks for is lexed over a
// separator or the token of a silent rule's extra whose text it also matches,
// as tree-sitter shifts a valid token before it reduces the same text to an
// extra (INI's and CSV's line ends). rust/tests/unit/grammar_token_over_extra.rs
// checks the Rust executor against the same fixture.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { compileGrammar, parseNativeGrammar, renderSyntaxTree } from '../src/index.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/grammar-token-over-extra.json', import.meta.url), 'utf8'));

test('a token is lexed over the separator or silent extra its text also matches', () => {
  for (const { name, grammar, parses } of fixture.cases) {
    const parser = compileGrammar(parseNativeGrammar(grammar));
    for (const { input, tree } of parses) {
      const result = parser.parseTree(input);
      assert.equal(result.tree ? renderSyntaxTree(result.tree) : JSON.stringify(result.rejection), tree, `${name}: ${JSON.stringify(input)}`);
    }
  }
});
