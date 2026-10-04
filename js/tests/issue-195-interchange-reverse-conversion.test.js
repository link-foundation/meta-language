// Issue #195 interchange reverse conversion: for one commented, irregularly
// formatted source per format in parity/fixtures/grammar-importers.json
// (reverse section), source grammar -> native links -> exported grammar ->
// native links keeps every rule, kind, definition and documentation while the
// emitter only sees the grammar decoded from the links; the re-imported
// grammar accepts and rejects independent samples like the source; and the
// lossless mode reconstructs the source byte for byte from the grammar links
// and the layout links alone, keeping every untouched definition and comment
// when a rule is changed or renamed. Pairs that drop a rule, drop
// documentation or swap a sequence are reported as different. The Rust twin is
// rust/tests/unit/issue_195_interchange_reverse_conversion.rs.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  GRAMMAR_LOSSLESS_FORMATS,
  Grammar,
  GrammarImportError,
  acceptsText,
  canonicalRuleDefinition,
  checkGrammarReverseConversion,
  emitGrammarLossless,
  grammarEmitter,
  grammarImporter,
  importGrammarLossless,
  mutateGrammarStartRule,
  parseGrammarLayoutLinks,
  parseGrammarLinks,
  renameGrammarRule,
  renderGrammarLayoutLinks,
  renderGrammarLinks,
} from '../src/index.js';
import { ISSUE_195_FIXTURE_FILES, recordIssue195Observations } from './support/issue-195-observations.js';

const corpus = JSON.parse(await readFile(new URL('../../parity/fixtures/grammar-importers.json', import.meta.url)));

const REQUIREMENT_ID = 'I195-INTERCHANGE-REVERSE-CONVERSION';

function record(assertion, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT_ID,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${REQUIREMENT_ID.toLowerCase()}`,
    fixtureFile: ISSUE_195_FIXTURE_FILES.grammarImporters,
    assertions: [assertion],
    testName,
  });
}

function pair(format) {
  return { importGrammar: grammarImporter(format), emitGrammar: grammarEmitter(format) };
}

function definitions(grammar) {
  return grammar.ruleNames().map((name) => `${name}=${canonicalRuleDefinition(grammar.rule(name))}`);
}

// Rebuilds `grammar` with `change` applied to its rules, keeping the start
// rule and source format.
function rebuild(grammar, change) {
  const rules = new Map();
  for (const rule of change([...grammar.rules.values()])) rules.set(rule.name, rule);
  return new Grammar(grammar.start, rules, grammar.sourceFormat);
}

test('issue 195 grammars export from their native links without the source in every format', () => {
  assert.deepEqual(corpus.reverse.map(({ format }) => format), GRAMMAR_LOSSLESS_FORMATS);
  for (const fixture of corpus.reverse) {
    const { importGrammar, emitGrammar } = pair(fixture.format);
    const report = checkGrammarReverseConversion(fixture.source, {
      importGrammar, emitGrammar, accepts: fixture.accepts, rejects: fixture.rejects,
    });
    assert.deepEqual(report.failures, [], fixture.id);
    assert.equal(report.status, 'equivalent', fixture.id);
    assert.equal(report.links, fixture.links, fixture.id);
    assert.equal(report.exported, fixture.exported, fixture.id);
    // The links alone, with the source out of reach, give the same export.
    const decoded = parseGrammarLinks(fixture.links);
    assert.equal(renderGrammarLinks(decoded), fixture.links, fixture.id);
    assert.equal(emitGrammar(decoded).source, fixture.exported, fixture.id);
    assert.notEqual(fixture.exported, fixture.source, fixture.id);
    // The commented sources carry documentation in the formats that read it.
    if (['antlr', 'gbnf', 'lark'].includes(fixture.format)) assert.match(fixture.links, /\(doc /u, fixture.id);
  }
  // EBNF comments are skipped outside string literals only.
  assert.throws(() => grammarImporter('ebnf')('a = "x" ; (* open'), GrammarImportError);
  assert.ok(acceptsText(grammarImporter('ebnf')('a = "(*" ;'), '(*'));
  assert.throws(() => parseGrammarLinks('(grammar (start missing))\n(rule word normal any)\n'), GrammarImportError);
  assert.throws(() => parseGrammarLinks('(grammar)\n(rule word normal (bogus))\n'), GrammarImportError);
  record('exportWithoutOriginalSource', 'issue 195 grammars export from their native links without the source in every format');
});

test('issue 195 re-imported exports are structurally and semantically equivalent to the source', () => {
  for (const fixture of corpus.reverse) {
    const { importGrammar, emitGrammar } = pair(fixture.format);
    const imported = importGrammar(fixture.source);
    const reimported = importGrammar(fixture.exported);
    assert.equal(renderGrammarLinks(reimported), fixture.reimportedLinks, fixture.id);
    assert.deepEqual(definitions(reimported), definitions(imported), fixture.id);
    assert.equal(reimported.startRule()?.name, imported.startRule()?.name, fixture.id);
    for (const text of fixture.accepts) assert.ok(acceptsText(reimported, text), `${fixture.id} accepts ${text}`);
    for (const text of fixture.rejects) assert.ok(!acceptsText(reimported, text), `${fixture.id} rejects ${text}`);

    // An emitter that swaps every top-level sequence or drops the rule
    // documentation (a rebuilt grammar keeps no docs) is reported as different.
    const documented = /\(doc /u.test(fixture.links);
    const broken = {
      'rules-changed': (grammar) => emitGrammar(rebuild(grammar, (rules) => rules.map((rule) => (
        rule.expression.kind === 'seq' ? { ...rule, expression: { ...rule.expression, items: [...rule.expression.items].reverse() } } : rule
      )))),
      'doc-changed': (grammar) => emitGrammar(rebuild(grammar, (rules) => rules)),
    };
    for (const [kind, emitBroken] of Object.entries(broken)) {
      const report = checkGrammarReverseConversion(fixture.source, {
        importGrammar, emitGrammar: emitBroken, accepts: fixture.accepts, rejects: fixture.rejects,
      });
      if (kind === 'doc-changed' && !documented) {
        assert.equal(report.status, 'equivalent', `${fixture.id} ${kind}`);
        continue;
      }
      assert.equal(report.status, 'different', `${fixture.id} ${kind}`);
      assert.ok(report.failures.some((failure) => failure.kind === kind), `${fixture.id} ${kind}`);
    }
  }
  record('reimportEquivalent', 'issue 195 re-imported exports are structurally and semantically equivalent to the source');
});

test('issue 195 lossless mode reconstructs every source exactly from links', () => {
  for (const fixture of corpus.reverse) {
    const importGrammar = grammarImporter(fixture.format);
    const { grammar, layout } = importGrammarLossless(fixture.source, fixture.format);
    assert.equal(renderGrammarLinks(grammar), fixture.links, fixture.id);
    assert.equal(renderGrammarLayoutLinks(layout), fixture.layout, fixture.id);
    const decodedLayout = parseGrammarLayoutLinks(fixture.layout);
    assert.deepEqual(decodedLayout, layout, fixture.id);
    const decoded = parseGrammarLinks(fixture.links);
    const exact = emitGrammarLossless(decoded, decodedLayout);
    assert.equal(exact.source, fixture.source, fixture.id);
    assert.deepEqual(exact.report.lossy, [], fixture.id);

    for (const [edited, expected] of [
      [mutateGrammarStartRule(decoded), fixture.mutated],
      [renameGrammarRule(decoded, fixture.renamed.from, fixture.renamed.to).grammar, fixture.renamed],
    ]) {
      assert.equal(renderGrammarLinks(edited), expected.links, fixture.id);
      const emitted = emitGrammarLossless(edited, decodedLayout);
      assert.equal(emitted.source, expected.source, fixture.id);
      assert.deepEqual(emitted.report.lossy, expected.lossy, fixture.id);
      assert.equal(renderGrammarLinks(importGrammar(emitted.source)), expected.links, fixture.id);
      // Every definition the edit left alone keeps its original text.
      const lines = renderGrammarLinks(edited).split('\n');
      const untouched = layout.members.filter((member) => lines.includes(member.fingerprint));
      assert.ok(untouched.length > 0 && untouched.length < layout.members.length, fixture.id);
      for (const member of untouched) assert.ok(emitted.source.includes(member.text), `${fixture.id} keeps ${member.name}`);
    }
  }
  assert.throws(() => importGrammarLossless('x', 'native'), TypeError);
  assert.throws(() => parseGrammarLayoutLinks('(layout native %)\n'), GrammarImportError);
  record('losslessModeExact', 'issue 195 lossless mode reconstructs every source exactly from links');
});
