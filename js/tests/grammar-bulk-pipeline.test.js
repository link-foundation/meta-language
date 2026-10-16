import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import test from 'node:test';

import {
  GRAMMARS_V4_SOURCES,
  bulkLanguageRow,
  bulkLanguages,
  descEntryPoint,
  missingFeatures,
  renderMatrix,
  startingAt,
  additionalGrammarTexts,
  importAdditionalGrammar,
  compareMergeDecisionReports,
  createPipelineGlobalConceptSpace,
} from '../scripts/run-grammar-bulk-pipeline.mjs';
import { GRAMMAR_SOURCES } from '../scripts/build-vendored-grammars.mjs';
import { compileGrammar, importAntlr, renderGrammarLinks, deserializeGrammar } from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

// The pipeline over all 57 languages runs in CI (the grammar-bulk-pipeline
// job), which publishes the matrix; these tests check what it reads and
// writes on a pinned source that needs no download.
const read = (relative) => readProjectData(new URL(`../../${relative}`, import.meta.url));
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
    for (const id of grammars) assert.ok(pinned.some(({ language }) => language === id) || GRAMMAR_SOURCES[id], `${name}: no pinned tree-sitter grammar ${id}`);
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

test('the additional official Groovy source is pinned, licensed and read without a download', () => {
  const [source] = additionalGrammarTexts('Groovy');
  assert.equal(source.id, 'apache-groovy');
  assert.equal(source.format, 'antlr');
  assert.equal(source.license, 'Apache-2.0');
  assert.match(source.revision, /^[0-9a-f]{40}$/u);
  assert.equal(source.files.length, 2);
  assert.ok(source.files.every(({ sha256, bytes }) => /^[0-9a-f]{64}$/u.test(sha256) && bytes > 0));
  assert.match(source.text, /parser grammar GroovyParser/u);
  assert.match(source.text, /lexer grammar GroovyLexer/u);
  assert.ok(source.corpora.length > 0);
});

test('additional grammar formats become native executable rules and retain import failures', async () => {
  const grammar = await importAdditionalGrammar({ id: 'official', format: 'bnf', entryPoint: 'document', text: '<document> ::= "official"\n' });
  assert.ok(compileGrammar(grammar).parseTree('official').tree);
  await assert.rejects(importAdditionalGrammar({ id: 'unknown', format: 'unavailable', text: '' }), /unsupported additional grammar format unavailable/u);
  await assert.rejects(importAdditionalGrammar({ id: 'official', format: 'bnf', entryPoint: 'absent', text: '<document> ::= "official"\n' }), /no entry point absent/u);
  const row = { treeSitter: { stage: 'done' }, additionalSources: [{ id: 'official', stage: 'import', error: 'untranslated action', undefinedRules: ['TOKEN'] }] };
  assert.deepEqual(missingFeatures(row), [
    { stage: 'additional official import', feature: 'undefined rules: TOKEN' },
    { stage: 'additional official import', feature: 'untranslated action' },
  ]);
  const failing = await bulkLanguageRow({ name: 'Multiline option control', grammars: ['control'], source: 'x' }, () => {}, {
    async loadTreeSitterSource() {
      return { origin: 'synthetic plain-string control', text: JSON.stringify({ name: 'control', rules: { document: { type: 'STRING', value: 'x' } } }) };
    },
    async loadGrammarsV4() {
      return { files: ['control/Control.g4'], entryPoint: 'start', text: "grammar Control; options {\n  superClass = UnresolvedBase;\r\n} start: A EOF; A: 'x';" };
    },
    loadAdditionalSources() { return []; },
  });
  assert.equal(failing.treeSitter.compiled, true);
  assert.equal(failing.treeSitter.sample, 'accepted');
  assert.equal(failing.grammarsV4.stage, 'compile');
  assert.notEqual(failing.grammarsV4.compiled, true);
  assert.match(failing.grammarsV4.error, /unresolved option:/u);
  assert.ok(failing.grammarsV4.error.includes('\\n  superClass = UnresolvedBase;\\r\\n'));
  assert.ok(failing.grammarsV4.error.length <= 300);
  assert.doesNotMatch(failing.grammarsV4.error, /[\r\n]/u);
  failing.missing = missingFeatures(failing);
  assert.ok(failing.missing.some(({ stage, feature }) => stage === 'grammars-v4 compile' && feature === failing.grammarsV4.error));
  const markdown = renderMatrix({
    grammarsV4Revision: v4.revision, grammarsV4Notes: {}, languages: [failing],
    summary: { treeSitterImported: 1, treeSitterCompiled: 1, treeSitterAccepted: 1, rowsMatch: 0, grammarsV4Imported: 1, grammarsV4Mapped: 1, grammarsV4Compiled: 0, grammarsV4Accepted: 0, merged: Number(failing.merge.stage === 'done'), mergedFromSeveral: Number(failing.merge.sources.length > 1), sharingRules: Number(failing.merge.sharedRules > 0), sharedRules: failing.merge.sharedRules ?? 0 },
  });
  assert.ok(markdown.split('\n').includes(`- grammars-v4 compile: ${failing.grammarsV4.error}`));
  assert.ok(markdown.includes('\\n  superClass = UnresolvedBase;\\r\\n'));
  assert.doesNotMatch(markdown, /\n  superClass = UnresolvedBase;/u);
});

test('decision reports explain every lost or changed shared correspondence', () => {
  const decision = (members, basis, definition = 'normal:lit("x")') => ({ kind: 'reconciled', name: 'value', members, basis, definition });
  const prior = { languages: [{ language: 'Example', treeSitter: { sha256: 'old' }, merge: {
    sharedRules: 2, decisions: [decision(['a:value', 'b:value'], 'name-correspondence'), decision(['a:word', 'b:word'], 'name-correspondence')],
  } }] };
  const next = { languages: [{ language: 'Example', treeSitter: { sha256: 'new' }, merge: {
    sharedRules: 1, decisions: [decision(['a:value', 'b:value'], 'structural-correspondence-up-to-lexical-roles'),
      { kind: 'kept-unique', name: 'word', members: ['a:word'], concept: 'unknown-value' },
      { kind: 'kept-unique', name: 'other-word', members: ['b:word'], concept: 'null-pointer' }],
  } }] };
  const [report] = compareMergeDecisionReports(prior, next);
  assert.equal(report.baselineAvailable, true);
  assert.equal(report.delta, -1);
  assert.deepEqual(report.sourceHashes, { previous: ['old', null], current: ['new', null] });
  assert.deepEqual(report.changes.map(({ kind }) => kind), ['changed-correspondence', 'removed-correspondence']);
  assert.match(report.changes[1].reason, /explicit semantic concepts/u);
  assert.equal(report.changes[1].currentMembers.length, 2);
  assert.equal(compareMergeDecisionReports(null, next)[0].baselineAvailable, false);
});

test('an additional lexer retains tokens and skipped trivia when linked with its Yacc parser', async () => {
  const grammar = await importAdditionalGrammar({
    id: 'parser-with-lexer', format: 'yacc', entryPoint: 'document',
    text: '%token NAME\n%start document\n%%\ndocument: NAME ;\n',
    lexerText: '%option nodefault\n%%\n[a-z]+ { return NAME; }\n[ \\t\\n]+ ;\n%%\n',
  });
  assert.deepEqual(grammar.undefinedNonterminals(), []);
  assert.ok(grammar.declarations.extras.length > 0);
  const outcome = compileGrammar(grammar).parseTree('name \n');
  assert.equal(outcome.ok, true);
});

test('the pipeline retains all serialized source origins and forwards only explicit formal premises', () => {
  const fixture = read('parity/fixtures/global-concept-space.json');
  const list = fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammarLinks: renderGrammarLinks(deserializeGrammar(grammar)) }));
  const options = { records: fixture.records, equivalences: fixture.equivalences };
  const grounded = createPipelineGlobalConceptSpace(list, { ...options, formalDefinitions: fixture.formalDefinitions });
  assert.deepEqual(grounded, fixture.expectedSpace);
  const unknown = createPipelineGlobalConceptSpace(list, options);
  assert.equal(unknown.proofs.length, 0);
  assert.deepEqual(unknown.formalDefinitions, {});
  assert.equal(unknown.concepts.length, 6);
  assert.equal(Object.keys(unknown.mappings).length, 6);
  assert.equal(unknown.failures.length, 2);
});
