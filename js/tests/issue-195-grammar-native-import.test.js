// The import pipeline of the native merged grammars,
// js/scripts/import-native-grammars.mjs: how it reads the pinned upstream
// test corpora and how js/src/grammar-importers/tree-sitter-native.js turns
// a pinned tree-sitter grammar into a native Links Notation grammar.
import assert from 'node:assert/strict';
import { test } from 'node:test';

import { corpusFileCases } from '../scripts/import-native-grammars.mjs';

const rule = (character, length = 80) => character.repeat(length);

test('a corpus case ends at its longest line of dashes, as in tree-sitter test', () => {
  const corpus = [
    rule('='),
    'Cargo script frontmatter',
    rule('='),
    '',
    '---cargo',
    '[dependencies]',
    '---',
    '',
    'fn main() {}',
    '',
    rule('-'),
    '',
    '(source_file (frontmatter) (function_item))',
    '',
    rule('='),
    'Plain',
    rule('='),
    'x',
    '---',
    '',
    '(source_file)',
    '',
  ].join('\n');
  assert.deepEqual(corpusFileCases(corpus), [
    { title: 'Cargo script frontmatter', source: '\n---cargo\n[dependencies]\n---\n\nfn main() {}\n' },
    { title: 'Plain', source: 'x' },
  ]);
});

test('of equally long lines of dashes, the last divides, and a case without one is all source', () => {
  const corpus = [rule('='), 'Twice', rule('='), 'a', '---', 'b', '---', '(x)', rule('='), 'Open', rule('='), 'c', ''].join('\n');
  assert.deepEqual(corpusFileCases(corpus), [
    { title: 'Twice', source: 'a\n---\nb' },
    { title: 'Open', source: 'c\n' },
  ]);
});
