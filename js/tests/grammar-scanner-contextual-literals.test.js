import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { contextualLiteralScanner } from '../scripts/contextual-literal-scanner.mjs';
import { nativeScannerProfile } from '../scripts/generate-native-scanner-profiles.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const read = file => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const readData = file => readProjectData(new URL(`../../${file}`, import.meta.url));
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
const rubyListing = read('parity/grammars/native/ruby.lino');
const rubyProfile = readData('parity/naming/grammar-name-expansions.json').grammars.ruby.scannerFamilies;
function rubyScannerParser(changes = {}) {
  const rubyGrammar = parseGrammarLinks(rubyListing);
  const profiles = rubyProfile.map((entry, index) => index === 0 ? { ...entry, ...changes } : entry);
  const generated = parseGrammarLinks(`(grammar (start unused))\n${nativeScannerProfile(profiles)}(rule unused normal (literal x))\n`);
  rubyGrammar.declarations.scanners = generated.declarations.scanners;
  for (const [name, rule] of generated.rules) {
    if (name !== 'unused') rubyGrammar.rules.set(name, { ...rubyGrammar.rules.get(name), ...rule });
  }
  return compileGrammar(rubyGrammar);
}
const rubyParser = rubyScannerParser();
const rubyProjection = readData('parity/fixtures/native-grammars/ruby-profile.json');

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
  const corpus = readData('parity/fixtures/native-grammars/ruby-profile-corpus.json');
  const source = corpus.cases.find(({ file, title }) => file === 'declarations.txt' && title === 'empty method').source;
  const oracle = oracleSnapshot(source, 'ruby');
  assert.equal(oracle.recovers, false);
  const native = rubyParser.parseTree(source);
  assert.equal(native.ok, true);
  assert.deepEqual(native.ambiguities, []);
  assert.deepEqual(nativeRows(native.tree, source, rubyProjection), oracle.rows);
  assert.equal(leaves(native.tree).map(({ text }) => text).join(''), source);
});

test('shared lexical decisions distinguish adjacent operators from command arguments', () => {
  const parser = rubyScannerParser({ sharedLexicalDecisions: [
    ['binary_star', 'splat_star'], ['binary_star_star', 'hash_splat_star_star'],
  ] });
  for (const source of ['foo-a', '@ivar-1', 'a *b', 'a*b']) {
    const oracle = oracleSnapshot(source, 'ruby');
    assert.equal(oracle.recovers, false, source);
    const native = parser.parseTree(source);
    assert.equal(native.ok, true, source);
    assert.deepEqual(native.ambiguities, [], source);
    assert.deepEqual(nativeRows(native.tree, source, rubyProjection), oracle.rows, source);
    assert.equal(leaves(native.tree).map(({ text }) => text).join(''), source);
  }
  const lexical = [{ token: 'operator', pattern: '\\*' }, { token: 'word', pattern: '[A-Za-z]+' }];
  for (const sharedLexicalDecisions of [1, [['operator']], [['operator', 'operator']], [['operator', 'unknown']], [['operator', 'word'], ['word', 'operator']]]) {
    assert.throws(() => contextualLiteralScanner({ ...descriptor, lexical, sharedLexicalDecisions }), TypeError);
  }
});

test('guarded whitespace delimiters preserve input-ended literals and percent operators', () => {
  const whitespace = ['\r', '\n', ' ', '\t'];
  const parser = rubyScannerParser({ delimiters: [
    ...rubyProfile[0].delimiters.filter(({ opening }) => !whitespace.includes(opening)),
    ...whitespace.map(opening => ({ opening, closing: '', guard: '(not (valid forward_slash))' })),
  ] });
  for (const source of [...whitespace.map(opening => `%${opening}foo`), '"hello" %\n[foo]']) {
    const oracle = oracleSnapshot(source, 'ruby');
    assert.equal(oracle.recovers, false, source);
    const native = parser.parseTree(source);
    assert.equal(native.ok, true, source);
    assert.deepEqual(native.ambiguities, [], source);
    assert.deepEqual(nativeRows(native.tree, source, rubyProjection), oracle.rows, source);
    assert.equal(leaves(native.tree).map(({ text }) => text).join(''), source);
  }
  assert.equal(parser.parseTree('%q(hello').ok, false);
});
