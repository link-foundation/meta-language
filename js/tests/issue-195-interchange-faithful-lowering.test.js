// Issue #195 interchange faithful lowering: for the grammars in
// parity/fixtures/grammar-importers.json (lowering section), lowering into
// every less expressive notation writes the recorded executable text, which
// that notation's own importer reads back as the lowered grammar and which
// accepts and rejects the samples like the original when every encoding is
// exact; the recorded reconstruction metadata names every helper rule, its
// construct, its encoding and its original expression, every rename, kind and
// documentation step, and reconstructs the original from the executable; and
// a package missing any step, or an emitter that writes the unlowered grammar,
// is reported as broken instead of dropping the feature silently. The Rust
// twin is rust/tests/unit/issue_195_interchange_faithful_lowering.rs.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  GRAMMAR_LOWERING_FORMATS,
  Grammar,
  GrammarImportError,
  GrammarLoweringError,
  acceptsText,
  checkGrammarLowering,
  droppedGrammarFeatures,
  grammarEmitter,
  grammarImporter,
  lowerGrammar,
  parseGrammarLinks,
  parseLoweringMetadata,
  reconstructGrammar,
  renderGrammarLinks,
  renderLinksExpression,
  renderLoweringMetadata,
} from '../src/index.js';
import { ISSUE_195_FIXTURE_FILES, recordIssue195Observations } from './support/issue-195-observations.js';

const corpus = JSON.parse(await readFile(new URL('../../parity/fixtures/grammar-importers.json', import.meta.url)));

const REQUIREMENT_ID = 'I195-INTERCHANGE-FAITHFUL-LOWERING';

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

// Every lowering target of every fixture grammar, with the parsed original.
function targets() {
  return corpus.lowering.flatMap((entry) => {
    const grammar = parseGrammarLinks(entry.links);
    return entry.targets.map((target) => ({ entry, grammar, target, label: `${entry.id} ${target.format}` }));
  });
}

// The metadata lines of the steps that rewrite an original rule; a helper
// lowered from another helper is restored together with its owner.
function topLevelStepLines(metadata) {
  const { order, steps } = parseLoweringMetadata(metadata);
  const lines = metadata.split('\n');
  return steps.flatMap((step, index) => (step.kind === 'helper' && !order.includes(step.owner) ? [] : [lines[4 + index]]));
}

test('issue 195 grammars lower into an executable grammar in every less expressive notation', () => {
  assert.deepEqual(corpus.lowering.map(({ id }) => id), ['features:lowering', 'lookahead:lowering']);
  for (const entry of corpus.lowering) {
    assert.deepEqual(entry.targets.map(({ format }) => format), GRAMMAR_LOWERING_FORMATS, entry.id);
  }
  for (const { entry, grammar, target, label } of targets()) {
    assert.equal(renderGrammarLinks(grammar), entry.links, label);
    const lowering = lowerGrammar(grammar, target.format);
    assert.equal(lowering.status, target.status, label);
    assert.equal(lowering.executable, target.executable, label);
    assert.equal(lowering.metadata, target.metadata, label);
    assert.deepEqual(lowering.report.lossy, [], label);
    // The target's own importer reads the executable back as the lowered grammar.
    const imported = grammarImporter(target.format)(target.executable);
    assert.deepEqual(imported.ruleNames(), lowering.grammar.ruleNames(), label);
    assert.equal(imported.startRule()?.name, lowering.grammar.startRule().name, label);
    if (target.status === 'exact') {
      for (const text of entry.accepts) assert.ok(acceptsText(imported, text), `${label} accepts ${text}`);
      for (const text of entry.rejects) assert.ok(!acceptsText(imported, text), `${label} rejects ${text}`);
    }
    const report = checkGrammarLowering(grammar, target.format, { accepts: entry.accepts, rejects: entry.rejects });
    assert.deepEqual(report.failures, [], label);
    assert.equal(report.status, target.status, label);
  }
  // Every feature grammar lowering is exact; the lookahead grammar is exact
  // only in pest, which writes lookaheads and ordered choices itself.
  assert.deepEqual(
    targets().filter(({ target }) => target.status === 'exact').map(({ label }) => label),
    [...GRAMMAR_LOWERING_FORMATS.map((format) => `features:lowering ${format}`), 'lookahead:lowering pest'],
  );
  assert.throws(() => lowerGrammar(parseGrammarLinks(corpus.lowering[0].links), 'cobol'), GrammarLoweringError);
  assert.throws(() => lowerGrammar(new Grammar(null, new Map()), 'bnf'), GrammarLoweringError);
  record('loweringExecutable', 'issue 195 grammars lower into an executable grammar in every less expressive notation');
});

test('issue 195 lowering metadata explicitly reconstructs the original grammar', () => {
  for (const { entry, grammar, target, label } of targets()) {
    const metadata = parseLoweringMetadata(target.metadata);
    assert.equal(renderLoweringMetadata(metadata), target.metadata, label);
    assert.equal(metadata.format, target.format, label);
    assert.equal(metadata.status, target.status, label);
    assert.equal(metadata.source, 'peg', label);
    assert.equal(metadata.start, grammar.startRule().name, label);
    assert.deepEqual(metadata.order, grammar.ruleNames(), label);
    // Every helper rule of the executable is a step naming its owner, its
    // construct, its encoding and the original expression it replaces.
    const executable = grammarImporter(target.format)(target.executable);
    const helpers = metadata.steps.filter((step) => step.kind === 'helper');
    const executableNames = new Set(executable.ruleNames());
    const originalNames = new Set(metadata.order.map((name) => metadata.steps
      .find((step) => step.kind === 'rename' && step.rule === name)?.value ?? name));
    assert.deepEqual(
      [...executableNames].filter((name) => !originalNames.has(name)),
      helpers.map(({ helper }) => helper),
      label,
    );
    for (const step of helpers) {
      assert.ok(step.construct.length > 0 && step.note.length > 0, `${label} ${step.helper}`);
      assert.ok(['exact', 'approximate'].includes(step.encoding), `${label} ${step.helper}`);
      assert.ok(target.metadata.includes(` ${renderLinksExpression(step.original)})\n`), `${label} ${step.helper}`);
    }
    assert.equal(target.status === 'approximate', helpers.some(({ encoding }) => encoding === 'approximate'), label);
    // The executable alone loses the features the steps carry; the package
    // (executable plus metadata) reconstructs every one of them.
    if (metadata.steps.length > 0) assert.notDeepEqual(droppedGrammarFeatures(grammar, executable), [], label);
    const reconstructed = reconstructGrammar(target.executable, target.metadata);
    assert.equal(renderGrammarLinks(reconstructed), target.reconstructed, label);
    assert.deepEqual(droppedGrammarFeatures(grammar, reconstructed), [], label);
    assert.deepEqual(droppedGrammarFeatures(grammar, parseGrammarLinks(target.reconstructed)), [], label);
    if (target.status === 'exact') {
      for (const text of entry.accepts) assert.ok(acceptsText(reconstructed, text), `${label} accepts ${text}`);
      for (const text of entry.rejects) assert.ok(!acceptsText(reconstructed, text), `${label} rejects ${text}`);
    }
  }
  // The metadata states every approximation: the BNF executable of the
  // lookahead grammar accepts x, which the original and its reconstruction reject.
  const lookahead = corpus.lowering[1];
  const bnf = lookahead.targets.find(({ format }) => format === 'bnf');
  assert.ok(acceptsText(grammarImporter('bnf')(bnf.executable), 'x'));
  assert.ok(!acceptsText(reconstructGrammar(bnf.executable, bnf.metadata), 'x'));
  assert.deepEqual(
    parseLoweringMetadata(bnf.metadata).steps.filter(({ encoding }) => encoding === 'approximate').map(({ construct }) => construct),
    ['orderedChoice', 'not', 'any'],
  );
  const { metadata } = bnf;
  assert.throws(() => parseLoweringMetadata(metadata.replace('(lowering bnf approximate)', '(lowering bnf maybe)')), GrammarImportError);
  assert.throws(() => parseLoweringMetadata(metadata.replace('(source peg)', '(source cobol)')), GrammarImportError);
  assert.throws(() => parseLoweringMetadata(metadata.replace(' repeat0 exact ', ' repeat0 lossless ')), GrammarImportError);
  assert.throws(() => parseLoweringMetadata(metadata.replace('(lowering bnf', '(lowering cobol')), GrammarLoweringError);
  assert.throws(() => parseLoweringMetadata(`${metadata}(kind list fancy)\n`), GrammarImportError);
  assert.throws(() => parseLoweringMetadata(`${metadata}(swap list item)\n`), GrammarImportError);
  assert.throws(() => parseLoweringMetadata('(lowering bnf exact)\n(source)\n'), GrammarImportError);
  record('reconstructionMetadataExplicit', 'issue 195 lowering metadata explicitly reconstructs the original grammar');
});

test('issue 195 lowering never drops a grammar feature silently', () => {
  let removed = 0;
  for (const { entry, grammar, target, label } of targets()) {
    // A package missing any step that rewrites an original rule is broken.
    for (const line of topLevelStepLines(target.metadata)) {
      const report = checkGrammarLowering(grammar, target.format, {
        accepts: entry.accepts,
        rejects: entry.rejects,
        editMetadata: (metadata) => metadata.split('\n').filter((other) => other !== line).join('\n'),
      });
      assert.equal(report.status, 'broken', `${label} without ${line}`);
      assert.ok(report.failures.length > 0, `${label} without ${line}`);
      assert.ok(report.failures.every(({ kind }) => kind === 'feature-dropped'), `${label} without ${line}`);
      removed += 1;
    }
  }
  assert.equal(removed, 80);
  // The detail names the dropped feature.
  const [features] = corpus.lowering;
  const grammar = parseGrammarLinks(features.links);
  const without = (pattern) => (metadata) => metadata.split('\n').filter((line) => !line.startsWith(pattern)).join('\n');
  const details = (format, pattern) => checkGrammarLowering(grammar, format, { editMetadata: without(pattern) })
    .failures.map(({ detail }) => detail);
  assert.deepEqual(details('gbnf', '(kind sep'), ['rule sep lost its kind token', 'rule sep changed its definition']);
  assert.deepEqual(details('bnf', '(doc message'), ['rule message changed its documentation']);
  assert.deepEqual(details('gbnf', '(helper lowered2 shout'), ['rule shout changed its definition']);
  assert.deepEqual(details('gbnf', '(rename message root'), ['the executable grammar has no rule message']);
  // An emitter that writes the unlowered grammar reports what the target
  // cannot carry instead of a faithful lowering.
  for (const format of GRAMMAR_LOWERING_FORMATS) {
    const report = checkGrammarLowering(grammar, format, {
      accepts: features.accepts, rejects: features.rejects, emitGrammar: () => grammarEmitter(format)(grammar),
    });
    assert.equal(report.status, 'broken', format);
    const kinds = new Set(report.failures.map(({ kind }) => kind));
    assert.ok(kinds.has('lossy-emission') && kinds.has('not-executable'), format);
  }
  assert.deepEqual(droppedGrammarFeatures(grammar, new Grammar(null, new Map())), [
    'rules [message, greeting, sep, name, letter, digit, shout] became []', 'the start rule changed',
  ]);
  record('noSilentlyDroppedFeature', 'issue 195 lowering never drops a grammar feature silently');
});
