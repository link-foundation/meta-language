import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { contextualLiteralScanner } from '../scripts/contextual-literal-scanner.mjs';
import { nativeScannerProfile } from '../scripts/generate-native-scanner-profiles.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const read = file => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const leaves = node => node.type === 'token' ? [node] : node.children.flatMap(leaves);

const descriptor = {
  name: 'literal_context',
  tokens: ['string_start', 'string_content', 'string_end', 'line_break', 'no_line_break', 'short_interpolation', 'operator', 'word'],
  contentToken: 'string_content', endToken: 'string_end', openGateToken: 'string_start',
  lineBreakToken: 'line_break', noLineBreakToken: 'no_line_break',
  shortInterpolationToken: 'short_interpolation', shortInterpolationPattern: '[A-Za-z_]',
  literals: [{ token: 'string_start', prefix: '', delimiter: '"', interpolated: true, words: false }],
};

test('contextual lexical policies retain fallback when a following condition does not match', () => {
  const scanner = contextualLiteralScanner({ ...descriptor, lexical: [
    { token: 'operator', pattern: '\\*', following: '[^\\s]' },
    { token: 'operator', pattern: '\\*' },
  ] });
  const parser = compileGrammar(parseGrammarLinks(`(grammar (start document))\n${scanner}(rule document normal (seq (ref operator) (literal %20) (literal x)))\n`));
  const outcome = parser.parseTree('* x');
  assert.equal(outcome.ok, true);
  assert.deepEqual(leaves(outcome.tree).map(({ text }) => text), ['*', ' ', 'x']);
});

test('contextual line continuations retain their source and separate lexical words', () => {
  const scanner = contextualLiteralScanner({ ...descriptor,
    lineContinuationPattern: '\\\\(?:\\r?\\n|[ \\t\\v\\f])',
    lexical: [{ token: 'word', pattern: '[A-Za-z]+' }],
  });
  const parser = compileGrammar(parseGrammarLinks(`(grammar (start document))\n${scanner}(rule document normal (seq (ref word) (ref word)))\n`));
  for (const source of ['alpha \\\n beta', 'alpha \\\r\n beta']) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, source);
    assert.equal(leaves(outcome.tree).map(({ text }) => text).join(''), source);
    assert.deepEqual(leaves(outcome.tree).filter(({ kind }) => kind === 'word').map(({ text }) => text), ['alpha', 'beta']);
  }
  for (const lineContinuationPattern of ['', 'x*', 1]) {
    assert.throws(() => contextualLiteralScanner({ ...descriptor, lineContinuationPattern }), TypeError);
  }
});

// Exercise generated scanner decisions without rewriting the shared generated
// Ruby files while a grammar author prepares the rest of the batch.
const rubyGrammar = parseGrammarLinks(read('parity/grammars/native/ruby.lino'));
const rubyProfile = JSON.parse(read('parity/naming/grammar-name-expansions.json')).grammars.ruby.scannerFamilies;
const generated = parseGrammarLinks(`(grammar (start unused))\n${nativeScannerProfile(rubyProfile)}(rule unused normal (literal x))\n`);
rubyGrammar.declarations.scanners = generated.declarations.scanners;
for (const [name, rule] of generated.rules) {
  if (name !== 'unused') rubyGrammar.rules.set(name, { ...rubyGrammar.rules.get(name), ...rule });
}
const rubyParser = compileGrammar(rubyGrammar);
const rubyProjection = JSON.parse(read('parity/fixtures/native-grammars/ruby-profile.json'));

test('shared contextual literal behavior preserves nested strings, balanced delimiters and regular expression suffixes', () => {
  for (const source of ['"#{[1, "c #{2} d", 3]}"', '%r(a(b)c)im', '%W(one #{2} three)', 'foo /bar/']) {
    const oracle = oracleSnapshot(source, 'ruby');
    assert.equal(oracle.recovers, false, source);
    const native = rubyParser.parseTree(source);
    assert.equal(native.ok, true, source);
    assert.deepEqual(native.ambiguities, [], source);
    assert.deepEqual(nativeRows(native.tree, source, rubyProjection), oracle.rows, source);
    assert.equal(leaves(native.tree).map(({ text }) => text).join(''), source);
  }
});

test('Ruby scanner dispatch parses the pinned empty method within the unchanged default budget', () => {
  const corpus = JSON.parse(read('parity/fixtures/native-grammars/ruby-profile-corpus.json'));
  const source = corpus.cases.find(({ file, title }) => file === 'declarations.txt' && title === 'empty method').source;
  const oracle = oracleSnapshot(source, 'ruby');
  assert.equal(oracle.recovers, false);
  const native = rubyParser.parseTree(source);
  assert.equal(native.ok, true);
  assert.deepEqual(native.ambiguities, []);
  assert.deepEqual(nativeRows(native.tree, source, rubyProjection), oracle.rows);
  assert.equal(leaves(native.tree).map(({ text }) => text).join(''), source);
});
