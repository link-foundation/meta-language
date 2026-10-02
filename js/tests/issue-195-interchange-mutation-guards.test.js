// Issue #195 interchange mutation guards: every shared importer case is round
// tripped through checkGrammarRoundTrip, which changes the grammar before
// export and requires the change in the exported text and the re-imported
// grammar; every malformed source of parity/fixtures/grammar-importers.json is
// rejected; and two importer/emitter pairs that a plain round trip cannot tell
// from correct ones (mutually reversing sequences, re-emitting a stale source)
// are reported as broken. The Rust twin is
// rust/tests/unit/issue_195_interchange_mutation_guards.rs.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  Grammar,
  GrammarImportError,
  GRAMMAR_ROUND_TRIP_MARKER,
  checkGrammarRoundTrip,
  emitAbnf,
  emitBnf,
  emitEbnf,
  emitPest,
  emitTreeSitterJson,
  importAbnf,
  importBnf,
  importEbnf,
  importPest,
  importTreeSitterJson,
  parseWithGrammar,
} from '../src/index.js';
import { ISSUE_195_FIXTURE_FILES, recordIssue195Observations } from './support/issue-195-observations.js';
import { renderGrammarRule } from './support/render-grammar-expression.js';

const corpus = JSON.parse(await readFile(new URL('../../parity/fixtures/grammar-importers.json', import.meta.url)));

const pairs = {
  abnf: { importGrammar: importAbnf, emitGrammar: emitAbnf },
  bnf: { importGrammar: importBnf, emitGrammar: emitBnf },
  ebnf: { importGrammar: importEbnf, emitGrammar: emitEbnf },
  pest: { importGrammar: importPest, emitGrammar: emitPest },
  'tree-sitter-json': { importGrammar: importTreeSitterJson, emitGrammar: emitTreeSitterJson },
};

const REQUIREMENT_ID = 'I195-INTERCHANGE-MUTATION-GUARDS';

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

function accepts(grammar, text) {
  try {
    parseWithGrammar(grammar, text);
    return true;
  } catch {
    return false;
  }
}

// A plain round trip, as in issue-195-grammar-importers.test.js: import,
// export, re-import and compare the declared rules.
function plainRoundTripAgrees(fixture, { importGrammar, emitGrammar }) {
  const imported = importGrammar(fixture.source);
  const reimported = importGrammar(emitGrammar(imported).source);
  return fixture.rules.every((name) => renderGrammarRule(reimported.rule(name)) === renderGrammarRule(imported.rule(name)));
}

function reverseSequences(grammar) {
  const reverse = (expression) => {
    const copy = { ...expression };
    if (Array.isArray(expression.items) && expression.kind !== 'charClass') {
      copy.items = expression.items.map(reverse);
      if (expression.kind === 'seq') copy.items.reverse();
    }
    if (expression.item) copy.item = reverse(expression.item);
    return copy;
  };
  const rules = new Map([...grammar.rules.values()].map((rule) => [rule.name, { kind: rule.kind, expression: reverse(rule.expression) }]));
  return new Grammar(grammar.start, rules, grammar.sourceFormat);
}

test('issue 195 mutated grammars survive export and re-import in every shared format', () => {
  assert.equal(corpus.cases.length, 10);
  for (const fixture of corpus.cases) {
    const report = checkGrammarRoundTrip(fixture.source, {
      ...pairs[fixture.format],
      accepts: fixture.accepts,
      rejects: fixture.rejects,
    });
    assert.deepEqual(report.failures, [], fixture.id);
    assert.equal(report.status, 'preserved', fixture.id);
    // The recorded emission of the unmutated grammar is an independent
    // reference: the mutated export must differ from it and carry the marker.
    assert.notEqual(report.exported, fixture.emitted.source, fixture.id);
    assert.ok(report.exported.includes(GRAMMAR_ROUND_TRIP_MARKER), fixture.id);
    assert.ok(!accepts(pairs[fixture.format].importGrammar(fixture.source), GRAMMAR_ROUND_TRIP_MARKER), fixture.id);
    assert.ok(accepts(report.reimported, GRAMMAR_ROUND_TRIP_MARKER), fixture.id);
    assert.equal(report.reimported.startRule()?.name, fixture.start, fixture.id);
  }
  record('mutationVisibleAfterRoundTrip', 'issue 195 mutated grammars survive export and re-import in every shared format');
});

test('issue 195 malformed grammars and negative samples are rejected', () => {
  const formats = new Set(corpus.malformed.map(({ format }) => format));
  assert.deepEqual([...formats].sort(), Object.keys(pairs).sort());
  for (const { format, reason, source } of corpus.malformed) {
    assert.throws(
      () => checkGrammarRoundTrip(source, pairs[format]),
      GrammarImportError,
      `${format} rejects a source with ${reason}`,
    );
  }
  for (const fixture of corpus.cases) {
    const { importGrammar } = pairs[fixture.format];
    assert.ok(fixture.rejects.length > 0, fixture.id);
    const { reimported } = checkGrammarRoundTrip(fixture.source, pairs[fixture.format]);
    for (const text of fixture.rejects) {
      assert.ok(!accepts(importGrammar(fixture.source), text), `${fixture.id} rejects ${JSON.stringify(text)}`);
      assert.ok(!accepts(reimported, text), `${fixture.id} re-import rejects ${JSON.stringify(text)}`);
    }
  }
  record('malformedGrammarsRejected', 'issue 195 malformed grammars and negative samples are rejected');
});

test('issue 195 mutually wrong importer and exporter pairs are detected', () => {
  for (const fixture of corpus.cases) {
    const { importGrammar, emitGrammar } = pairs[fixture.format];
    const samples = { accepts: fixture.accepts, rejects: fixture.rejects };

    // Both halves reverse every sequence, so each undoes the other's error.
    const reversing = {
      importGrammar: (source) => reverseSequences(importGrammar(source)),
      emitGrammar: (grammar) => emitGrammar(reverseSequences(grammar)),
    };
    assert.ok(plainRoundTripAgrees(fixture, reversing), `${fixture.id} fools a plain round trip`);
    const reversed = checkGrammarRoundTrip(fixture.source, { ...reversing, ...samples });
    assert.equal(reversed.status, 'broken', fixture.id);
    assert.ok(reversed.failures.some(({ kind, stage }) => kind === 'sample-rejected' && stage === 'imported'), fixture.id);

    // The exporter prints the source it was first given, whatever it receives.
    const stale = { importGrammar, emitGrammar: () => ({ source: fixture.source, report: { lossy: [] } }) };
    assert.ok(plainRoundTripAgrees(fixture, stale), `${fixture.id} fools a plain round trip`);
    const unchanged = checkGrammarRoundTrip(fixture.source, { ...stale, ...samples });
    assert.equal(unchanged.status, 'broken', fixture.id);
    const kinds = new Set(unchanged.failures.map(({ kind }) => kind));
    assert.ok(kinds.has('mutation-not-exported'), fixture.id);
    assert.ok(kinds.has('mutation-not-visible'), fixture.id);
    assert.ok(kinds.has('rules-changed'), fixture.id);
  }
  record('mutuallyWrongPairDetected', 'issue 195 mutually wrong importer and exporter pairs are detected');
});
