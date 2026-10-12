import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import test from 'node:test';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { renderSyntaxTree } from '../src/grammar-runtime/syntax-tree.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtures = readProjectData(new URL('../../parity/fixtures/scanner-structured-contexts.json', import.meta.url));
const leaves = (node) => node.type === 'token' ? [node] : node.children.flatMap(leaves);

const families = [
  ['here-document', 'HERE-DOCUMENTS', 'scannerHereDocumentsMatchFixtures'],
  ['interpolated-quoted', 'INTERPOLATED-QUOTES', 'scannerInterpolationContextsMatchFixtures'],
  ['offside-layout', 'OFFSIDE-LAYOUT', 'scannerOffsideLayoutMatchesFixtures'],
  ['fenced-block', 'FENCED-BLOCKS', 'scannerFencedBlocksMatchFixtures'],
];

for (const [family, suffix, assertion] of families) {
  const testName = `${family} scanners preserve concrete source and reject every malformed fixture boundary`;
  test(testName, () => {
    const cases = fixtures.filter((fixture) => fixture.scanners[0].family === family);
    assert.ok(cases.length > 0);
    for (const fixture of cases) {
      assert.equal(scannerFamilies(fixture.scanners), fixture.generated);
      const parser = compileGrammar(parseGrammarLinks(fixture.listing));
      // Failed speculation must leave no remembered label or quote context in
      // the following parse, even when a parser instance is reused.
      for (const input of fixture.reject) assert.equal(parser.parseTree(input).ok, false, input);
      for (const input of fixture.accept) {
        const outcome = parser.parseTree(input);
        assert.equal(outcome.ok, true, `${fixture.name}: ${JSON.stringify(input)}`);
        assert.equal(outcome.tree.start, 0);
        assert.equal(outcome.tree.end, Buffer.byteLength(input));
        assert.equal(leaves(outcome.tree).map(({ text }) => text).join(''), input);
        assert.equal(renderSyntaxTree(outcome.tree), fixture.trees[input]);
        let offset = 0;
        for (const token of leaves(outcome.tree)) {
          assert.equal(token.start, offset);
          offset += Buffer.byteLength(token.text);
          assert.equal(token.end, offset);
        }
      }
    }
    const requirementId = `I195-GRAMMAR-SCANNER-${suffix}`;
    recordIssue195Observations({
      requirementId,
      suffix: 'behavior',
      fixtureId: 'scanner-structured-contexts',
      fixtureFile: 'parity/fixtures/scanner-structured-contexts.json',
      assertions: [assertion],
      testName,
    });
  });
}

test('here-document closing labels are whole line labels and bodies are FIFO', () => {
  const fixture = fixtures.find(({ name }) => name === 'FIFO here-document bodies');
  const parser = compileGrammar(parseGrammarLinks(fixture.listing));
  const tokens = leaves(parser.parseTree(fixture.accept[0]).tree);
  assert.deepEqual(tokens.filter(({ kind }) => kind === 'end').map(({ text }) => text), ['A', 'B']);
  assert.deepEqual(tokens.filter(({ kind }) => kind === 'content').map(({ text }) => text), ['a\n', 'b\n']);
});

test('nested quotes preserve independent interpolation and brace contexts', () => {
  const fixture = fixtures.find(({ name }) => name === 'Ruby nested quoted interpolation');
  const parser = compileGrammar(parseGrammarLinks(fixture.listing));
  const tokens = leaves(parser.parseTree('"#{ {name: "hi"} }"').tree);
  assert.deepEqual(tokens.filter(({ kind }) => ['start', 'end', 'interpolation_start', 'interpolation_end', 'brace_open', 'brace_close'].includes(kind)).map(({ kind }) => kind), ['start', 'interpolation_start', 'brace_open', 'start', 'end', 'brace_close', 'interpolation_end', 'end']);
  assert.equal(tokens.filter(({ kind }) => kind === 'content').map(({ text }) => text).join(''), 'hi');
});

test('a shorter Markdown fence remains body text and a longer matching fence closes', () => {
  const fixture = fixtures.find(({ name }) => name === 'Markdown fenced code block boundaries');
  const parser = compileGrammar(parseGrammarLinks(fixture.listing));
  for (const input of ['````\n```\ntext\n`````\n', '~~~\n```\n~~~\n']) {
    const outcome = parser.parseTree(input);
    assert.equal(outcome.ok, true);
    assert.deepEqual(leaves(outcome.tree).map(({ text }) => text), [input]);
  }
});

test('structured scanner families reject malformed parameters and conflicting tokens', () => {
  for (const fixture of fixtures) {
    const descriptor = fixture.scanners[0];
    assert.throws(() => scannerFamilies([{ ...descriptor, name: ') fail (' }]), TypeError);
    assert.throws(() => scannerFamilies([descriptor, descriptor]), TypeError);
  }
  const here = fixtures[0].scanners[0];
  for (const change of [{ startToken: 'content' }, { labelPattern: 'a*' }, { labelPattern: '' }, { indentation: 'x' }, { queue: 1 }, { quotes: ['"', '"'] }, { quotes: [''] }, { allowEmpty: null }]) assert.throws(() => scannerFamilies([{ ...here, ...change }]), TypeError);
  const quoted = fixtures.find(({ name }) => name === 'Ruby nested quoted interpolation').scanners[0];
  for (const change of [{ braceOpenToken: 'content' }, { escape: '' }, { quotes: [] }, { quotes: [{ opening: '"', closing: '"', interpolation: '', multiline: false }] }, { quotes: [{ opening: '"', closing: '"', interpolation: null, multiline: 1 }] }]) assert.throws(() => scannerFamilies([{ ...quoted, ...change }]), TypeError);
  const fenced = fixtures.find(({ name }) => name === 'Markdown fenced code block boundaries').scanners[0];
  for (const change of [{ minimum: 0 }, { maximumIndentation: -1 }, { markers: ['ab'] }, { markers: ['~', '~'] }, { allowEnd: 1 }]) assert.throws(() => scannerFamilies([{ ...fenced, ...change }]), TypeError);
});
