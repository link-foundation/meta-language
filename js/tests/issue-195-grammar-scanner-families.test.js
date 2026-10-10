import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { contentScanner, scannerFamilies } from '../scripts/scanner-families.mjs';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const observeScannerState = (assertions, testName) => recordIssue195Observations({
  requirementId: 'I195-GRAMMAR-SCANNER-STATE-SEMANTICS', suffix: 'behavior',
  fixtureId: 'planned:repository-directive:i195-grammar-scanner-state-semantics',
  fixtureFile: 'parity/fixtures/scanner-state-semantics.json', assertions, testName,
});

test('scanner values normalize single uppercase scalars without expanding names', (context) => {
  const { listing, accept, reject } = JSON.parse(readFileSync(new URL('../../parity/fixtures/scanner-state-semantics.json', import.meta.url), 'utf8'))[0];
  const parser = compileGrammar(parseGrammarLinks(listing));
  for (const input of accept) {
    assert.equal(parser.parseTree(input).ok, true, input);
  }
  for (const input of reject) assert.equal(parser.parseTree(input).ok, false, input);
  observeScannerState(['scannerCaseConversionUsesSingleScalars'], context.name);
});

test('scanner lookahead predicates observe preceding state mutations', (context) => {
  const { listing } = JSON.parse(readFileSync(new URL('../../parity/fixtures/scanner-state-semantics.json', import.meta.url), 'utf8'))[1];
  const parser = compileGrammar(parseGrammarLinks(listing));
  const outcome = parser.parseTree('x');
  assert.equal(outcome.ok, true);
  assert.equal(outcome.tree.children[0].text, 'x');
  assert.equal(parser.parseTree('y').ok, false);
  assert.equal(parser.parseTree('x').ok, true);
  observeScannerState(['scannerPredicatesSeeCurrentState', 'scannerStateFailuresRemainIsolated'], context.name);
});

const fixtureGroups = [
  ['parity/fixtures/scanner-fragments.json', 'I195-GRAMMAR-SCANNER-FRAGMENTS'],
  ['parity/fixtures/scanner-remembered-content.json', 'I195-GRAMMAR-SCANNER-REMEMBERED-CONTENT'],
  ['parity/fixtures/scanner-remembered-delimiters.json', 'I195-GRAMMAR-SCANNER-REMEMBERED-DELIMITERS'],
  ['parity/fixtures/scanner-context-tokens.json', 'I195-GRAMMAR-SCANNER-CONTEXT-TOKENS'],
  ['parity/fixtures/scanner-delimiter-runs.json', 'I195-GRAMMAR-SCANNER-DELIMITER-RUNS'],
  ['parity/fixtures/scanner-families.json', 'I195-GRAMMAR-SCANNER-DELIMITER-FAMILIES'],
  ['parity/fixtures/scanner-counted-delimiters.json', 'I195-GRAMMAR-SCANNER-COUNTED-DELIMITERS'],
];
const fixtures = fixtureGroups.flatMap(([fixtureFile, requirementId]) => JSON.parse(readFileSync(new URL(`../../${fixtureFile}`, import.meta.url), 'utf8')).map((item) => ({ ...item, fixtureFile, requirementId })));

test('external extras keep skipped prefixes outside their named token span', () => {
  const listing = `(grammar (start source))
(extra (ref annotation))
(scanner annotations (tokens annotation) (operations (while (next (class plain (char %20) (char %0A))) (do (skip (class plain (char %20) (char %0A))))) (consume (literal %23)) (while (all (not atEnd) (not (next (literal %0A)))) (do advance)) (emit annotation)))
(rule source normal (repeat0 (literal x)))`;
  const input = 'x \n#note';
  const outcome = compileGrammar(parseGrammarLinks(listing)).parseTree(input);
  assert.equal(outcome.ok, true);
  const annotation = outcome.tree.children.find(({ kind }) => kind === 'annotation');
  assert.equal(annotation.start, 3);
  assert.equal(annotation.end, 8);
  assert.equal(annotation.text, '#note');
  assert.equal(outcome.tree.children.map(({ text }) => text).join(''), input);
});

test('wrapped scanner tokens retain skipped trivia and read consumed text past a mark', () => {
  const cases = JSON.parse(readFileSync(new URL('../../parity/fixtures/scanner-token-spans.json', import.meta.url), 'utf8'));
  for (const fixture of cases) {
    const parser = compileGrammar(parseGrammarLinks(fixture.listing));
    for (const input of fixture.accept) {
      const outcome = parser.parseTree(input);
      assert.equal(outcome.ok, true, `${fixture.name}: ${input}`);
      assert.equal(renderSyntaxTree(outcome.tree), fixture.trees[input]);
      const leaf = outcome.tree.children.find(({ kind }) => kind === 'label');
      const start = input === 'é😀' ? 0 : 2;
      assert.equal(leaf.start, start);
      assert.equal(leaf.end, start + 2);
      assert.equal(outcome.tree.end, Buffer.byteLength(input));
    }
    for (const input of fixture.reject) assert.equal(parser.parseTree(input).ok, false, input);
  }
});

test('template context descriptors reject conflicting tokens, empty labels and malformed escapes', () => {
  const descriptor = { family: 'template-context', name: 'templates', quotedStartToken: 'quoted_start', quotedEndToken: 'quoted_end', contentToken: 'content', interpolationStartToken: 'interpolation_start', interpolationEndToken: 'interpolation_end', directiveStartToken: 'directive_start', directiveEndToken: 'directive_end', delimiterToken: 'label', labelPattern: '[a-z]+' };
  for (const change of [{ delimiterToken: 'content' }, { labelPattern: '' }, { labelPattern: 'a*' }, { name: ') fail (' }, { escapeCharacters: ['ab'] }, { hexadecimalEscapes: [{ prefix: 'u', digits: 0 }] }, { hexadecimalEscapes: [{ prefix: 'n', digits: 4 }] }]) assert.throws(() => scannerFamilies([{ ...descriptor, ...change }]), TypeError);
});
for (const fixture of fixtures) {
  test(`scanner family ${fixture.name} preserves text and rejects truncated delimiters`, () => {
    assert.equal(scannerFamilies(fixture.scanners), fixture.generated);
    const parser = compileGrammar(parseGrammarLinks(fixture.listing));
    for (const input of fixture.accept) {
      const outcome = parser.parseTree(input);
      assert.equal(outcome.ok, true, JSON.stringify({ input, rejection: outcome.rejection }));
      assert.equal(outcome.tree.start, 0);
      assert.equal(outcome.tree.end, Buffer.byteLength(input));
      const texts = (node) => node.type === 'token' ? node.text : node.children.map(texts).join('');
      assert.equal(texts(outcome.tree), input);
      assert.equal(renderSyntaxTree(outcome.tree), fixture.trees[input]);
    }
    for (const input of fixture.reject) assert.equal(parser.parseTree(input).ok, false, input);
    recordIssue195Observations({
      requirementId: fixture.requirementId, suffix: 'behavior',
      fixtureId: `planned:repository-directive:${fixture.requirementId.toLowerCase()}`,
      fixtureFile: fixture.fixtureFile, assertions: [fixture.assertion],
      testName: `scanner family ${fixture.name} preserves text and rejects truncated delimiters`,
    });
  });
}

test('scanner families satisfy tree-sitter external declarations through executable grammar data', () => {
  const fixture = fixtures.find((item) => item.name === 'nested comments');
  const grammar = {
    name: 'comments', externals: [{ type: 'SYMBOL', name: 'comment' }],
    rules: { source: { type: 'REPEAT1', content: { type: 'SYMBOL', name: 'comment' } } },
  };
  const imported = importTreeSitterNative(grammar, { scanners: scannerFamilies(fixture.scanners), immediate: ['comment'] });
  assert.equal(imported.scanners.length, 1);
  assert.deepEqual(imported.report.unsupported, []);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
  assert.equal(parser.parseTree('/*a/*b*/c*/').ok, true);
  assert.equal(parser.parseTree('/*a/*b*/').ok, false);
});

test('scanner family descriptors reject invalid and ambiguous definitions', () => {
  const descriptor = { family: 'delimited', name: 'comments', token: 'comment', opening: '/*', closing: '*/' };
  for (const field of ['name', 'token']) assert.throws(() => scannerFamilies([{ ...descriptor, [field]: ') fail (' }]), TypeError);
  assert.throws(() => scannerFamilies([{ ...descriptor, opening: '' }]), TypeError);
  assert.throws(() => scannerFamilies([{ ...descriptor, opening: '"', closing: '"', nested: true }]), TypeError);
  assert.throws(() => scannerFamilies([descriptor, descriptor]), TypeError);
  assert.throws(() => scannerFamilies([{ ...descriptor, family: 'host callback' }]), TypeError);
  assert.throws(() => scannerFamilies([{ family: 'content', name: 'strings', token: 'string', closeToken: 'string', closing: '"' }]), TypeError);
  const split = { family: 'split-counted-delimiter', name: 'blocks', startToken: 'start', contentToken: 'content', endToken: 'end', prefix: '[', marker: '=', opening: '[', closing: ']', suffix: ']' };
  for (const countModulo of [0, 1, 2.5, Infinity, '256']) assert.throws(() => scannerFamilies([{ ...split, countModulo }]), TypeError);
  assert.throws(() => scannerFamilies([{ ...split, endToken: 'start' }]), TypeError);
  assert.throws(() => scannerFamilies([{ ...split, contentStops: [''] }]), TypeError);
});

test('the shipped Rust string scanner is generated by the common content family', () => {
  const generated = contentScanner({ name: 'strings', token: 'string_content', closeToken: 'string_close', closing: '"', stops: ['\\'], allowEnd: false });
  const listing = readFileSync(new URL('../../parity/grammars/scanners/rust.lino', import.meta.url), 'utf8');
  assert.equal(listing.split('\n')[0] + '\n', generated);
});

test('delimiter-run and line-boundary descriptors reject invalid token and count definitions', () => {
  const run = { family: 'delimiter-run', name: 'quotes', contentToken: 'content', endToken: 'end', delimiter: '"', count: 3 };
  for (const count of [0, 1, 2.5, Infinity, '3']) assert.throws(() => scannerFamilies([{ ...run, count }]), TypeError);
  for (const change of [{ endToken: 'content' }, { delimiter: '' }, { name: ') fail (' }]) assert.throws(() => scannerFamilies([{ ...run, ...change }]), TypeError);
  assert.throws(() => scannerFamilies([{ family: 'line-boundary', name: 'lines', token: ') fail (' }]), TypeError);
});

test('context and counted line scanner descriptors reject malformed policies', () => {
  const boundary = { family: 'lookahead-boundary', name: 'boundary', token: 'boundary', before: ['}'] };
  for (const change of [{ before: [] }, { allowEnd: 1 }, { token: ') fail (' }]) assert.throws(() => scannerFamilies([{ ...boundary, ...change }]), TypeError);
  const context = { family: 'context-token', name: 'colon', token: 'colon', opening: ':', target: '{', stops: [';'] };
  for (const change of [{ stops: [] }, { comments: 1 }, { immediateCharacters: ['ab'] }, { immediateRanges: [['z', 'a']] }]) assert.throws(() => scannerFamilies([{ ...context, ...change }]), TypeError);
  const counted = { family: 'line-counted-delimiter', name: 'quotes', token: 'quotes' };
  for (const change of [{ minimum: 1 }, { minimum: 2.5 }, { countModulo: 2 }, { optionalPrefixCharacters: ['ab'], prefix: '~' }]) assert.throws(() => scannerFamilies([{ ...counted, ...change }]), TypeError);
});

test('remembered delimiter descriptors reject nullable tags and conflicting token names', () => {
  const descriptor = { family: 'remembered-delimiter', name: 'tags', startToken: 'opening', contentToken: 'body', endToken: 'closing', tagPattern: '\\$[^$\\s]*\\$' };
  for (const change of [{ tagPattern: '' }, { tagPattern: 'a*' }, { tagPattern: '(a|)' }, { endToken: 'opening' }, { startToken: ') fail (' }]) {
    assert.throws(() => scannerFamilies([{ ...descriptor, ...change }]));
  }
});

test('remembered literal and content descriptors validate their state and boundary policies', () => {
  const whole = { family: 'remembered-literal', name: 'tags', token: 'literal', startToken: 'opening', contentToken: 'body', endToken: 'closing', tagPattern: '\\$[^$\\s]*\\$' };
  for (const change of [{ token: 'opening' }, { excludedLabels: ') fail (' }, { tagPattern: 'a*' }]) assert.throws(() => scannerFamilies([{ ...whole, ...change }]), TypeError);
  const content = { family: 'remembered-content', name: 'tags', delimiterToken: 'marker', contentToken: 'body', delimiterPattern: '[a-z]{1,4}', closingPrefix: ')', closingSuffix: '"' };
  for (const change of [{ contentToken: 'marker' }, { delimiterPattern: 'a*' }, { closingPrefix: '' }, { closingSuffix: '' }, { allowEmpty: 1 }, { allowEnd: 'yes' }]) assert.throws(() => scannerFamilies([{ ...content, ...change }]), TypeError);
});
