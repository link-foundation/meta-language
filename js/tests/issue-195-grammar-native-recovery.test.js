// Requirement I195-GRAMMAR-NATIVE-RECOVERY: automatic error recovery in the
// native executor. With `errorRecovery` a parse the grammar rejects is
// repaired into a tree of ERROR and MISSING leaves, with no recovery rules in
// the grammar. Every rejection of parity/fixtures/native-grammars/*.json
// records its `recovered` tree; rust/tests/unit/issue_195_grammar_native_recovery.rs
// checks the Rust executor against the same trees.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { NATIVE_GRAMMARS, fixturePath } from '../scripts/generate-native-grammar-fixtures.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');
const grammars = NATIVE_GRAMMARS.map((entry) => ({
  entry,
  fixture: JSON.parse(read(fixturePath(entry))),
  parser: compileGrammar(parseGrammarLinks(read(entry.grammar))),
}));
const RECOVER = { errorRecovery: true };

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-NATIVE-RECOVERY',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-native-recovery',
    fixtureFile: 'parity/fixtures/native-grammars/json.json',
    assertions,
    testName,
  });
}

// The leaves in source order; a MISSING leaf covers no bytes.
function leafText(tree, source) {
  const bytes = Buffer.from(source, 'utf8');
  const parts = [];
  const visit = (node) => {
    if (node.type === 'node') node.children.forEach(visit);
    else if (node.type === 'embed') visit(node.root);
    else if (node.type === 'missing') assert.equal(node.start, node.end);
    else parts.push(bytes.subarray(node.start, node.end));
  };
  visit(tree);
  return Buffer.concat(parts).toString('utf8');
}

test('every native grammar repairs each fixture rejection into its recorded tree', (context) => {
  let repaired = 0;
  for (const { entry, fixture, parser } of grammars) {
    assert.ok(fixture.rejections.length > 0, entry.id);
    for (const { source, recovered } of fixture.rejections) {
      const label = `${entry.id} ${JSON.stringify(source)}`;
      const plain = parser.parseTree(source);
      assert.equal(plain.ok, false, label);
      assert.equal(plain.tree, null, label);
      const outcome = parser.parseTree(source, RECOVER);
      assert.equal(outcome.ok, false, label);
      assert.equal(outcome.rejection.reason, 'recovered', label);
      assert.equal(renderSyntaxTree(outcome.tree), recovered, label);
      assert.equal(leafText(outcome.tree, source), source, label);
      const accepted = parser.parseTree(source, { ...RECOVER, recovery: 'accept' });
      assert.equal(accepted.ok, true, label);
      assert.equal(renderSyntaxTree(accepted.tree), recovered, label);
      repaired += 1;
    }
  }
  assert.ok(repaired >= 200, `${repaired} repaired rejections`);
  observe(['nativeRecoveryTreesMatchFixtures', 'nativeRecoveryTreesLossless', 'nativeRecoveryReportedAsRecovered'], context.name);
});

test('automatic recovery leaves the trees of accepted input unchanged', (context) => {
  for (const { entry, fixture, parser } of grammars) {
    for (const { source } of [...fixture.matches, ...fixture.divergences]) {
      const plain = parser.parseTree(source);
      const outcome = parser.parseTree(source, RECOVER);
      assert.equal(outcome.ok, true, `${entry.id} ${JSON.stringify(source)}`);
      assert.equal(renderSyntaxTree(outcome.tree), renderSyntaxTree(plain.tree), `${entry.id} ${JSON.stringify(source)}`);
    }
  }
  observe(['nativeRecoveryKeepsAcceptedTrees'], context.name);
});

test('a repair inserts a MISSING leaf or skips input as an ERROR leaf, whichever costs less', () => {
  const json = grammars.find(({ entry }) => entry.id === 'json').parser;
  const tree = (source, options = RECOVER) => renderSyntaxTree(json.parseTree(source, options).tree);
  // A missing separator and a missing closing bracket are inserted.
  assert.equal(tree('{"a" 1}'), '(document (object "{" (pair key:(string "\\"" (string_content "a") "\\"") ~" " (MISSING@5 ":") value:(number "1")) "}"))');
  assert.equal(tree('[1, 2'), '(document (array "[" (number "1") "," ~" " (number "2") (MISSING@5 "]")))');
  // A stray value is skipped: deleting one byte costs less than inserting a comma.
  assert.equal(tree('[1, 2 3]'), '(document (array "[" (number "1") "," ~" " (number "2") ~" " (ERROR@6..7 "3") "]"))');
  // With no repair point left, the rest of the input is one ERROR leaf.
  assert.equal(tree('[1, 2 3]', { ...RECOVER, maxRepairs: 0 }), '(document (ERROR@0..8 "[1, 2 3]"))');
  // Without the option the parse is rejected with no tree.
  assert.equal(json.parseTree('[1, 2 3]').tree, null);
});
