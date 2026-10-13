import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';
import { acceptPrecedenceOperandHead } from '../src/translation/frontend-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/grammar-prefix-operand-precedence.lino', import.meta.url));
const parser = compileGrammar(parseGrammarLinks(nativeGrammarText('native-rust')));
const reconstruct = node => node.type === 'node' ? node.children.map(reconstruct).join('') : node.text;

test('suffix precedence requires the actual first child source rule or reduction', () => {
  for (const value of fixture.operandHeadCases) {
    assert.equal(acceptPrecedenceOperandHead(value.sourceRuleMatches, value.sourceReductionMatches), value.accepted);
  }
});

test('named and anonymous reduced operands retain their application suffix', () => {
  const selected = compileGrammar(parseGrammarLinks(fixture.operandGrammar));
  for (const source of fixture.operandSources) {
    const result = selected.parseTree(source);
    assert.equal(result.ok, true, source);
    assert.deepEqual(result.ambiguities, []);
    assert.equal(result.tree.kind, 'assignment');
    assert.equal(result.tree.children[0].kind, 'application');
    assert.equal(reconstruct(result.tree), source);
  }
});

for (const value of fixture.cases) {
  test(`source prefix and operand CST fidelity: ${value.id}`, () => {
    const oracle = oracleSnapshot(value.source, 'Rust');
    assert.equal(oracle.recovers, false);
    assert.deepEqual(oracle.rows, value.rows, 'the independent oracle remains pinned');
    const result = parser.parseTree(value.source);
    assert.equal(result.ok, true, value.source);
    assert.deepEqual(result.ambiguities, []);
    assert.deepEqual(nativeRows(result.tree, value.source, fixture.projection), value.rows);
    assert.equal(reconstruct(result.tree), value.source);
  });
}
