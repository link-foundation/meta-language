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
  // A missing separator and a missing closing bracket are inserted; as in
  // tree-sitter, a MISSING leaf comes before the white space after the token
  // it follows.
  assert.equal(tree('{"a" 1}'), '(document (object "{" (pair key:(string "\\"" (string_content "a") "\\"") (MISSING@4 ":") ~" " value:(number "1")) "}"))');
  assert.equal(tree('[1, 2'), '(document (array "[" (number "1") "," ~" " (number "2") (MISSING@5 "]")))');
  // A stray value is skipped: deleting one byte costs less than inserting a comma.
  assert.equal(tree('[1, 2 3]'), '(document (array "[" (number "1") "," ~" " (number "2") ~" " (ERROR@6..7 "3") "]"))');
  // With no repair point left, the rest of the input is one ERROR leaf.
  assert.equal(tree('[1, 2 3]', { ...RECOVER, maxRepairs: 0 }), '(document (ERROR@0..8 "[1, 2 3]"))');
  // Without the option the parse is rejected with no tree.
  assert.equal(json.parseTree('[1, 2 3]').tree, null);
});

test('a skip never ends at an external scanner token, so a repair stays within the step budget', () => {
  // A string's content is a scanner token that reads to the next quote and
  // fails at the end of the input; a scan for a skip that ended at it would
  // run the scanner from every later offset, quadratic in the rest of the
  // input. As in tree-sitter, whose scanners refuse to run in error recovery,
  // no skip ends at such a token, and the stray no-break space is one ERROR.
  const rust = grammars.find(({ entry }) => entry.id === 'rust').parser;
  const source = `let x;\n\u00a0\n${'pub fn a() {}\n'.repeat(64)}`;
  const outcome = rust.parseTree(source, RECOVER);
  assert.equal(outcome.rejection.reason, 'recovered');
  const rendered = renderSyntaxTree(outcome.tree);
  assert.deepEqual(rendered.match(/ERROR@\d+\.\.\d+|MISSING@\d+/gu), ['ERROR@7..9']);
  assert.equal(rendered.match(/\(function_item /gu).length, 64);
  assert.equal(leafText(outcome.tree, source), source);
});

test('a later error is repaired where it is, not by skipping the input after an earlier one', () => {
  // With the stray comma as the only repair point, the cheapest complete
  // result takes the rest of the input as ERROR there; the result that skips
  // the comma reaches the stray bracket, so its end becomes the next repair
  // point and each stray token is one ERROR, as in tree-sitter.
  const rust = grammars.find(({ entry }) => entry.id === 'rust').parser;
  const source = 'struct S { a: u8,, }\nimpl S { fn f(&self) -> u8 { self.a } } ]\n';
  const repairs = (options) => renderSyntaxTree(rust.parseTree(source, options).tree).match(/ERROR@\d+\.\.\d+|MISSING@\d+/gu);
  assert.deepEqual(repairs(RECOVER), ['ERROR@17..18', 'ERROR@61..62']);
  // When the rounds end first, the complete result stands.
  assert.deepEqual(repairs({ ...RECOVER, maxRepairs: 1 }), ['MISSING@17', 'ERROR@17..62']);
});

test('a repaired result preempts no cheaper one where the scanner scans a token of no width', () => {
  // An object that skips the stray `@9` (cost 2) goes on past `e` to its
  // closing brace; a statement block repaired after a MISSING `}` (more cost)
  // scans an automatic semicolon there, which no longer prunes the cheaper
  // object: as in tree-sitter, the stray token is one ERROR.
  const typescript = grammars.find(({ entry }) => entry.id === 'typescript').parser;
  for (const source of ['x = { c : 0 @9 , e } ;', 'f ( { c : 0 @9 , e } ) ;']) {
    const rendered = renderSyntaxTree(typescript.parseTree(source, RECOVER).tree);
    assert.deepEqual(rendered.match(/ERROR@\d+\.\.\d+|MISSING@\d+/gu), ['ERROR@12..14'], rendered);
    assert.ok(rendered.includes('(shorthand_property_identifier "e") ~" " "}")'), rendered);
  }
});

test('a missing keyword is named by its literal', () => {
  // A keyword token is its literal and a lookahead that no word character
  // follows; as tree-sitter names its keyword token, its MISSING leaf is the
  // literal.
  const rust = grammars.find(({ entry }) => entry.id === 'rust').parser;
  const rendered = renderSyntaxTree(rust.parseTree('oc =c =>=', RECOVER).tree);
  assert.ok(rendered.includes('(return_expression (MISSING@7 "return"))'), rendered);
});

test('a long repetition, repaired near its end, keeps every item in order', () => {
  // A join links its parts instead of copying the children before it, so a
  // repetition of n items costs O(n) and not O(n²); the tree is unchanged.
  const json = grammars.find(({ entry }) => entry.id === 'json').parser;
  const count = 10_000;
  const source = `[${Array.from({ length: count }, (_, index) => String(index % 10)).join(',')} 7]`;
  const outcome = json.parseTree(source, RECOVER);
  assert.equal(outcome.rejection.reason, 'recovered');
  const [array] = outcome.tree.children;
  const numbers = array.children.filter((child) => child.kind === 'number');
  assert.equal(numbers.length, count);
  assert.ok(numbers.every((child, index) => source.slice(child.start, child.end) === String(index % 10)));
  assert.equal(leafText(outcome.tree, source), source);
  assert.equal(array.children.at(-2).type, 'error');
});
