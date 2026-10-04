// Requirement I195-PARITY-FEATURE-COMPLETENESS: parity/language-features.json
// lists every public export of js/src/index.js with the public Rust item that
// carries the same feature, and parity/fixtures/export-parity/cases.lino holds
// the observable output both runtimes must produce for those features. The
// Rust counterparts are rust/tests/unit/javascript_export_parity.rs (generated)
// and rust/tests/unit/export_parity_corpus.rs.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { Parser } from 'links-notation';

import * as javascript from '../src/index.js';
import { exportParity } from '../scripts/generate-export-parity.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const CASES_FILE = 'parity/fixtures/export-parity/cases.lino';
const repositoryFile = (relative) => new URL(`../../${relative}`, import.meta.url);
const features = JSON.parse(readFileSync(repositoryFile('parity/language-features.json'), 'utf8'));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-PARITY-FEATURE-COMPLETENESS',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-parity-feature-completeness',
    fixtureFile: 'docs/vision.md',
    assertions,
    testName,
  });
}

// A link read as `[id, ...values]`, a reference as its id.
const canonical = (link) => ((link.values && link.values.length) || link.id === null
  ? [link.id, ...(link.values ?? []).map(canonical)]
  : link.id);

/** The corpus as `{ name, input, expected }` cases of strings. */
export function exportParityCases(text = readFileSync(repositoryFile(CASES_FILE), 'utf8')) {
  return new Parser().parse(text).map((link) => {
    const [, name, input, expected] = canonical(link);
    const values = (part, label) => {
      assert.ok(Array.isArray(part) && part[1] === label, `${name} has an (${label} ...) part`);
      return part.slice(2);
    };
    return { name, input: values(input, 'input'), expected: values(expected, 'expected') };
  });
}

const optional = (value) => (value === 'none' ? null : value);
const grammarOf = (source) => javascript.importEbnf(source);
const ruleExpression = (grammar, rule) => grammar.rules.get(rule).expression;
const allExpressions = (grammar) => [...grammar.rules.values()].map((rule) => rule.expression);
const display = (expression) => javascript.displayGrammarExpression(expression);
const attempt = (run) => {
  try {
    return run();
  } catch {
    return 'error';
  }
};
const parsed = (grammar, text) => javascript.renderSyntaxTree(javascript.parseWithGrammar(grammar, text));

/** The observable output of one corpus case, as strings. */
const RUNNERS = Object.freeze({
  sniffLanguage: ([text]) => [javascript.sniffLanguage(text) ?? 'none'],
  GRAMMAR_DIAGNOSTIC_KINDS: () => [...javascript.GRAMMAR_DIAGNOSTIC_KINDS],
  displayGrammarExpression: ([source, rule]) => [display(ruleExpression(grammarOf(source), rule))],
  sequence: ([source]) => [display(javascript.sequence(allExpressions(grammarOf(source))))],
  choice: ([source, ordered]) => [display(javascript.choice(allExpressions(grammarOf(source)), ordered === 'true'))],
  canonicalRepeat: ([source, rule, min, max]) => {
    const item = ruleExpression(grammarOf(source), rule);
    const bound = optional(max);
    try {
      return [display(javascript.canonicalRepeat(item, Number(min), bound === null ? null : Number(bound)))];
    } catch (error) {
      return [`error: ${error.message}`];
    }
  },
  canonicalRuleDefinition: ([source, rule]) => [javascript.canonicalRuleDefinition(grammarOf(source).rules.get(rule))],
  acceptsText: ([source, text]) => [String(javascript.acceptsText(grammarOf(source), text))],
  parseWithGrammar: ([source, text]) => [attempt(() => parsed(grammarOf(source), text))],
  compileGrammar: ([source, text]) => [attempt(() =>
    javascript.renderSyntaxTree(javascript.compileGrammar(grammarOf(source)).parse(text)))],
  createGrammarParser: ([source, text]) => [attempt(() =>
    javascript.renderSyntaxTree(javascript.createGrammarParser(grammarOf(source)).parse(text)))],
  renderSyntaxTree: ([source, text]) => [attempt(() => parsed(grammarOf(source), text))],
  validateGrammar: ([source]) => javascript.validateGrammar(grammarOf(source)).map((diagnostic) => diagnostic.kind),
  carryRuleDocs: ([source, rule, doc]) => {
    const documented = grammarOf(source);
    documented.rules.set(rule, Object.freeze({ ...documented.rules.get(rule), doc }));
    const carried = javascript.carryRuleDocs(grammarOf(source), documented);
    return [carried.rules.get(rule).doc ?? 'none'];
  },
  detectEmbeddedRegions: ([text, language]) => javascript.detectEmbeddedRegions(text, language).map((region) => {
    const range = region.span().byteRange;
    return `${region.language()} ${range.start}..${range.end}`;
  }),
  sourceMeanings: ([source, name]) => javascript.sourceMeanings({ source, name }),
  idKey: ([value]) => [String(javascript.idKey(Number(value)))],
  accessModeLabel: ([mode]) => [javascript.accessModeLabel(mode)],
  accessModeIsMutable: ([mode]) => [String(javascript.accessModeIsMutable(mode))],
  accessModeIsReadOnly: ([mode]) => [String(javascript.accessModeIsReadOnly(mode))],
});

test('export parity: every public export is listed with a public Rust item', async () => {
  const { exportNames, entries, missing, features: generated } = await exportParity();
  const listed = features.javascriptExports;
  assert.deepEqual(Object.keys(listed).sort(), Object.keys(entries).sort(), 'the listed exports are the resolved exports');
  assert.equal(
    readFileSync(repositoryFile('parity/language-features.json'), 'utf8'),
    generated,
    'parity/language-features.json is current: run node js/scripts/generate-export-parity.mjs',
  );
  assert.deepEqual(missing, [], 'every JavaScript export resolves to a public Rust item');
  assert.deepEqual(Object.keys(listed).sort(), [...exportNames].sort(), 'every public export is listed');
  for (const [name, entry] of Object.entries(listed)) {
    assert.match(entry.rust, /^[A-Za-z_][\w:]*$/u, `${name} names a Rust path`);
    assert.ok(['function', 'method', 'type', 'constant'].includes(entry.kind), `${name} has a kind`);
  }
  observe(['everyPublicExportListed', 'rustHasEveryFeature'], 'export parity: every public export is listed with a public Rust item');
});

test('export parity: the shared corpus gives the expected output in JavaScript', () => {
  const cases = exportParityCases();
  assert.ok(cases.length > 0, 'the corpus has cases');
  for (const { name, input, expected } of cases) {
    assert.ok(features.javascriptExports[name], `${name} is a listed export`);
    assert.ok(RUNNERS[name], `${name} has a corpus runner`);
    assert.deepEqual(RUNNERS[name](input), expected, `${name}(${input.join(', ')})`);
  }
  observe(['observableOutputEqual'], 'export parity: the shared corpus gives the expected output in JavaScript');
});
