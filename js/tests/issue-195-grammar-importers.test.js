// Issue #195 grammar importer evidence: every shared importer case is imported
// through the public API, its declared rules match the runtime-neutral
// rendering recorded in parity/fixtures/grammar-importers.json, the generated
// parser module runs the corpus, the same-format emitter reproduces the
// recorded text, and both re-import and serialization preserve the grammar.
// The Rust twin is rust/tests/unit/issue_195_grammar_importers.rs.
import assert from 'node:assert/strict';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { test } from 'node:test';

import {
  deserializeGrammar,
  emitAbnf,
  emitBnf,
  emitEbnf,
  emitJavascriptParser,
  emitPest,
  emitTreeSitterJson,
  importAbnf,
  importBnf,
  importEbnf,
  importPest,
  importTreeSitterJson,
  parseWithGrammar,
  serializeGrammar,
} from '../src/index.js';
import {
  ISSUE_195_FIXTURE_FILES,
  issue195Slug,
  recordIssue195Observations,
} from './support/issue-195-observations.js';
import { renderGrammarRule } from './support/render-grammar-expression.js';

const corpus = JSON.parse(await readFile(new URL('../../parity/fixtures/grammar-importers.json', import.meta.url)));

// The requirement catalog names each importer; the fixture keys it by format.
const formats = {
  abnf: { importer: 'ABNF', importGrammar: importAbnf, emitGrammar: emitAbnf },
  bnf: { importer: 'BNF', importGrammar: importBnf, emitGrammar: emitBnf },
  ebnf: { importer: 'EBNF', importGrammar: importEbnf, emitGrammar: emitEbnf },
  pest: { importer: 'pest', importGrammar: importPest, emitGrammar: emitPest },
  'tree-sitter-json': {
    importer: 'tree-sitter grammar JSON',
    importGrammar: importTreeSitterJson,
    emitGrammar: emitTreeSitterJson,
  },
};

const ASSERTIONS = [
  'ordinaryPublicImportApi',
  'grammarExpressionsPreserved',
  'generatedParserExecuted',
  'generatedEmitterExecuted',
  'independentCorpusParses',
  'roundTripPreservesGrammar',
];

const packageEntryUrl = new URL('../src/index.js', import.meta.url).href;

// Loads the emitted parser module as a file so it runs exactly as a consumer
// would load it, with its `meta-language` import (the native grammar
// executor) pinned to this checkout's package entry point.
async function loadGeneratedParser(grammar, directory, name) {
  const code = emitJavascriptParser(grammar).replace("from 'meta-language'", `from ${JSON.stringify(packageEntryUrl)}`);
  const file = path.join(directory, `${name}.mjs`);
  await writeFile(file, code);
  return import(pathToFileURL(file).href);
}

function declaredRules(grammar, fixture) {
  return Object.fromEntries(fixture.rules.map((name) => [name, renderGrammarRule(grammar.rule(name))]));
}

for (const [format, { importer, importGrammar, emitGrammar }] of Object.entries(formats)) {
  const fixtures = corpus.cases.filter((fixture) => fixture.format === format);
  test(`issue 195 ${importer} importer cases import, generate, emit, and round-trip`, async () => {
    assert.ok(fixtures.length >= 2, `${format} has message and construct cases`);
    const directory = await mkdtemp(path.join(tmpdir(), 'meta-language-issue-195-import-'));
    try {
      for (const fixture of fixtures) {
        const grammar = importGrammar(fixture.source);
        assert.equal(grammar.startRule()?.name, fixture.start, fixture.id);
        assert.deepEqual(grammar.undefinedNonterminals(), [], fixture.id);
        assert.deepEqual(declaredRules(grammar, fixture), fixture.expressions, fixture.id);

        const generated = await loadGeneratedParser(grammar, directory, fixture.id.replace(/\W/g, '-'));
        for (const source of fixture.accepts) {
          assert.doesNotThrow(() => generated.parse(source), `${fixture.id} accepts ${JSON.stringify(source)}`);
          assert.doesNotThrow(() => parseWithGrammar(grammar, source), fixture.id);
        }
        for (const source of fixture.rejects) {
          assert.throws(() => generated.parse(source), `${fixture.id} rejects ${JSON.stringify(source)}`);
          assert.throws(() => parseWithGrammar(grammar, source), fixture.id);
        }

        const emitted = emitGrammar(grammar);
        assert.equal(emitted.source, fixture.emitted.source, fixture.id);
        assert.deepEqual(emitted.report.lossy, fixture.emitted.lossy, fixture.id);

        // Emission is idempotent from the first re-import on: ABNF appends
        // the referenced core rules once, every other format is a fixpoint.
        const reimported = importGrammar(emitted.source);
        assert.equal(reimported.startRule()?.name, fixture.start, fixture.id);
        assert.deepEqual(declaredRules(reimported, fixture), fixture.expressions, fixture.id);
        const reemitted = emitGrammar(reimported).source;
        assert.equal(emitGrammar(importGrammar(reemitted)).source, reemitted, fixture.id);
        if (format !== 'abnf') assert.equal(reemitted, emitted.source, fixture.id);
        for (const source of fixture.accepts) {
          assert.doesNotThrow(() => parseWithGrammar(reimported, source), `${fixture.id} re-import`);
        }
        for (const source of fixture.rejects) {
          assert.throws(() => parseWithGrammar(reimported, source), `${fixture.id} re-import`);
        }

        const restored = deserializeGrammar(serializeGrammar(grammar));
        assert.deepEqual(restored.normalized(), grammar.normalized(), fixture.id);
      }
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
    recordIssue195Observations({
      requirementId: `I195-IMPORT-${issue195Slug(importer)}`,
      suffix: 'positive',
      fixtureId: `planned:grammar-importer:${importer}`,
      fixtureFile: ISSUE_195_FIXTURE_FILES.grammarImporters,
      assertions: ASSERTIONS,
      testName: `issue 195 ${importer} importer cases import, generate, emit, and round-trip`,
    });
  });
}
