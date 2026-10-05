import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import {
  GRAMMARS_V4_SOURCES,
  bulkLanguageRow,
  bulkLanguages,
  descEntryPoint,
  missingFeatures,
  renderMatrix,
  startingAt,
} from '../scripts/run-grammar-bulk-pipeline.mjs';
import { GRAMMAR_SOURCES } from '../scripts/build-vendored-grammars.mjs';
import { compileGrammar, importAntlr } from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

// The pipeline over all 57 languages runs in CI (the grammar-bulk-pipeline
// job), which publishes the matrix; these tests check what it reads and
// writes on a pinned source that needs no download.
const read = (relative) => JSON.parse(readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8'));
const inventory = read('parity/language-grammar-inventory.json');
const pinned = read('parity/grammars/sources.json').sources;
const v4 = read(GRAMMARS_V4_SOURCES);

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-BULK-PIPELINE',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-bulk-pipeline',
    fixtureFile: 'docs/grammar/bulk-pipeline.md',
    assertions,
    testName,
    runtime: 'tooling',
  });
}

test('every catalog language with a grammar has a pinned tree-sitter source and its grammars-v4 mapping names catalog languages', () => {
  const languages = bulkLanguages();
  assert.deepEqual(
    languages.map(({ name }) => name),
    inventory.languages.filter(({ grammars }) => grammars?.length > 0).map(({ name }) => name),
  );
  assert.ok(languages.length >= 57, `${languages.length} languages`);
  for (const { name, grammars, source } of languages) {
    const id = grammars[0];
    assert.ok(pinned.some(({ language }) => language === id) || GRAMMAR_SOURCES[id], `${name}: no pinned tree-sitter grammar ${id}`);
    assert.equal(typeof source, 'string', `${name}: no sample source`);
  }
  const names = new Set(languages.map(({ name }) => name));
  for (const language of [...Object.keys(v4.languages), ...Object.keys(v4.notes ?? {})]) {
    assert.ok(names.has(language), `grammars-v4 ${language} is no catalog language with a grammar`);
  }
  assert.match(v4.revision, /^[0-9a-f]{40}$/u);
  observe(['everyCatalogLanguageImported'], 'bulk pipeline covers the catalog');
});

test('a bulk row imports, compiles, parses and merges a pinned grammar and lists what is missing', async () => {
  const entry = { name: 'C (offline)', grammars: ['c'], source: 'int main(void) { return 0; }\n' };
  const reports = [];
  const row = await bulkLanguageRow(entry, (report) => reports.push(structuredClone(report)));
  assert.equal(row.treeSitter.stage, 'done', row.treeSitter.error);
  assert.equal(row.treeSitter.compiled, true);
  assert.equal(row.treeSitter.sample, 'accepted', row.treeSitter.rejection);
  assert.ok(row.treeSitter.rules > 100);
  assert.equal(row.grammarsV4, null);
  assert.equal(row.merge.stage, 'done', row.merge.error);
  assert.deepEqual(row.merge.sources, ['tree-sitter']);
  assert.ok(reports.some((report) => report.treeSitter.stage === 'compile'), 'reports after each stage');
  row.missing = missingFeatures(row);
  assert.deepEqual(row.missing, []);

  const rejected = { ...entry, source: 'int main(void) { return 0; \n' };
  const failing = await bulkLanguageRow(rejected);
  assert.equal(failing.treeSitter.sample, 'rejected');
  failing.missing = missingFeatures(failing);
  assert.deepEqual(failing.missing.map(({ stage }) => stage), ['tree-sitter sample']);
  assert.match(failing.missing[0].feature, /^\w+ at \d+:\d+/u);
  observe(['everyCatalogLanguageImported', 'missingFeaturesListed'], 'bulk row on a pinned grammar');
});

test('a grammars-v4 grammar starts at the entry point its desc.xml names', () => {
  const desc = '<desc>\n   <targets>Java</targets>\n   <entry-point>program</entry-point>\n</desc>\n';
  assert.equal(descEntryPoint(desc), 'program');
  assert.equal(descEntryPoint('<desc><targets>Java</targets></desc>'), null);
  // Like TypeScriptParser.g4, the first rule is not the one the tests parse from.
  const imported = importAntlr("grammar G; initializer : '=' NUMBER ; program : initializer? NUMBER EOF ; NUMBER : [0-9]+ ; WS : ' ' -> skip ;");
  assert.equal(imported.startRule().name, 'initializer');
  const grammar = startingAt(imported, descEntryPoint(desc));
  assert.equal(grammar.startRule().name, 'program');
  assert.deepEqual(grammar.ruleNames(), imported.ruleNames());
  assert.ok(compileGrammar(grammar).parseTree('= 1 2').tree);
  assert.equal(compileGrammar(imported).parseTree('= 1 2').tree ?? null, null);
  assert.equal(startingAt(imported, null), imported);
  assert.equal(startingAt(imported, 'missing'), imported);
  observe(['everyCatalogLanguageImported'], 'bulk grammars-v4 entry point');
});

test('missing features name the stage and feature of every failure and the matrix lists them', () => {
  const failing = {
    language: 'Example',
    grammars: ['example'],
    treeSitter: { stage: 'compile', rules: 3, unsupported: ['external scanner token heredoc'], error: 'unknown rule x' },
    grammarsV4: { files: ['example/Example.g4'], stage: 'import', undefinedRules: ['A', 'B'], timedOut: true },
    merge: { sources: ['tree-sitter'], stage: 'merge', error: 'conflicting | rules' },
  };
  failing.missing = missingFeatures(failing);
  assert.deepEqual(failing.missing, [
    { stage: 'tree-sitter import', feature: 'external scanner token heredoc' },
    { stage: 'tree-sitter compile', feature: 'unknown rule x' },
    { stage: 'grammars-v4 import', feature: 'undefined rules: A B' },
    { stage: 'grammars-v4 import', feature: 'timed out' },
    { stage: 'merge', feature: 'conflicting | rules' },
  ]);
  const passing = {
    language: 'Fine',
    grammars: ['fine'],
    treeSitter: { stage: 'done', rules: 9, unsupported: [], compiled: true, sample: 'accepted', parseMs: 2, rowsMatch: true },
    grammarsV4: null,
    merge: { stage: 'done', rules: 9, sharedRules: 0 },
    missing: [],
  };
  const summary = { treeSitterImported: 2, treeSitterCompiled: 1, treeSitterAccepted: 1, rowsMatch: 1, grammarsV4Imported: 0, grammarsV4Mapped: 1, grammarsV4Compiled: 0, grammarsV4Accepted: 0, merged: 1, mergedFromSeveral: 0, sharingRules: 0, sharedRules: 0 };
  const markdown = renderMatrix({ grammarsV4Revision: v4.revision, summary, grammarsV4Notes: { Fine: 'no grammar' }, languages: [failing, passing] });
  assert.match(markdown, /^# Bulk grammar pipeline matrix\n/u);
  assert.match(markdown, /\| Example \| 3 \| 1 \| fails in compile \|/u);
  assert.match(markdown, /\| Fine \| 9 \| 0 \| yes \| accepted \| 2 \| yes \| — \|/u);
  assert.match(markdown, /### Example\n\n- tree-sitter import: external scanner token heredoc\n/u);
  assert.match(markdown, /- merge: conflicting \\\| rules\n/u);
  assert.doesNotMatch(markdown, /### Fine/u);
  assert.match(markdown, /## grammars-v4 notes\n\n- Fine: no grammar\n/u);
  assert.match(markdown, /1 languages merge, and 0 of the 0 merged from two or more sources share rules \(0 shared rules in all\)\./u);
  observe(['matrixPublished', 'missingFeaturesListed'], 'bulk matrix rendering');
});

// I195-MERGE-REAL-RECONCILIATION: a merge of two sources that shares no rule
// only concatenates them, and the matrix lists that as a missing feature; one
// source alone has nothing to share.
test('a merge of several sources that shares no rule is listed as a concatenation', () => {
  const row = (merge) => ({
    language: 'Example',
    grammars: ['example'],
    treeSitter: { stage: 'done', rules: 4, unsupported: [], compiled: true, sample: 'accepted', rowsMatch: true },
    grammarsV4: { files: ['example/Example.g4'], stage: 'done', rules: 5, compiled: true, sample: 'accepted' },
    merge: { stage: 'done', sources: ['tree-sitter', 'grammars-v4'], ...merge },
  });
  assert.deepEqual(missingFeatures(row({ rules: 9, sharedRules: 0 })), [
    { stage: 'merge', feature: 'the merge only concatenates its sources, sharing no rule (tree-sitter, grammars-v4: 9 rules)' },
  ]);
  assert.deepEqual(missingFeatures(row({ rules: 7, sharedRules: 2 })), []);
  assert.deepEqual(missingFeatures(row({ sources: ['tree-sitter'], rules: 4, sharedRules: 0 })), []);
  observe(['missingFeaturesListed'], 'bulk matrix lists a concatenating merge');
});
