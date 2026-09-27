// Issue #195 binding rename evidence: for every language of the shared
// four-language corpus, `renameBinding` follows one symbol through shadowing,
// nested scopes, qualified names, Unicode identifiers, and macro or proof
// binders, rewrites exactly that symbol's occurrences, leaves comments,
// strings, and every other name's resolution unchanged, and rejects renames
// that would capture another name.
// The Rust twin is rust/tests/unit/issue_195_binding_rename_corpus.rs.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { analyzeProgram } from '../src/index.js';
import {
  ISSUE_195_FIXTURE_FILES,
  issue195Slug,
  recordIssue195Observations,
} from './support/issue-195-observations.js';

const corpus = JSON.parse(await readFile(new URL('../../parity/fixtures/four-language-conformance.json', import.meta.url)));

const ASSERTIONS = [
  'symbolIdentity',
  'shadowing',
  'nestedScopes',
  'qualifiedNames',
  'unicodeIdentifiers',
  'macroOrProofBinders',
  'captureAvoidance',
  'commentsStringsAndLiteralsUnaffected',
];

// Every name's resolution: what each binding is, how often it is used, and
// which names stay unresolved.
function resolution(program) {
  return {
    bindings: program.bindings.map(({ name, kind, references }) => `${name}:${kind}:${references.length}`).sort(),
    unresolved: program.unresolvedReferences.map(({ name }) => name).sort(),
  };
}

function comments(program) {
  return program.sourceMappings
    .filter(({ term }) => /comment/u.test(term))
    .map(({ start, end }) => program.source.slice(start, end));
}

function observe(source) {
  const context = {};
  runInNewContext(source, context);
  return JSON.parse(JSON.stringify(context.result));
}

for (const language of corpus.languages.map(({ name }) => name)) {
  const cases = corpus.bindingRenameCorpus.filter((fixture) => fixture.language === language);
  const testName = `issue 195 ${language} binding rename follows symbol identity and rejects capture`;
  test(testName, () => {
    assert.deepEqual(
      [...new Set(cases.flatMap(({ assertions }) => assertions))].sort(),
      [...ASSERTIONS].sort(),
      `${language} corpus covers every rename assertion`,
    );
    assert.ok(cases.some(({ allowed }) => allowed) && cases.some(({ allowed }) => !allowed));

    for (const fixture of cases) {
      const label = `${language} ${fixture.assertions.join('+')} ${fixture.binding}#${fixture.declarationOccurrence}`;
      const program = analyzeProgram(fixture.source, language);
      assert.equal(program.network.verifyFullMatch().isClean(), true, `${label} parses`);
      const binding = program.bindings.filter(({ name }) => name === fixture.binding)[fixture.declarationOccurrence];
      assert.ok(binding, `${label} selects a binding`);
      if (fixture.expectedObservation !== undefined) {
        assert.deepEqual(observe(fixture.source), fixture.expectedObservation, `${label} original observation`);
      }

      if (!fixture.allowed) {
        assert.throws(() => program.renameBinding(binding.id, fixture.replacement), /capture|conflict/u, label);
        assert.equal(program.emit(), fixture.source, `${label} leaves the program intact`);
        continue;
      }

      const renamed = program.renameBinding(binding.id, fixture.replacement);
      assert.equal(renamed.emit(), fixture.expected, label);
      assert.equal(renamed.network.verifyFullMatch().isClean(), true, `${label} reparses`);
      // The renamed symbol keeps every occurrence; every other symbol and
      // every unresolved name resolves as before.
      const key = ({ name, kind, references }) => `${name}:${kind}:${references.length}`;
      const expected = resolution(program);
      expected.bindings.splice(expected.bindings.indexOf(key(binding)), 1);
      expected.bindings = [...expected.bindings, key({ ...binding, name: fixture.replacement })].sort();
      assert.deepEqual(resolution(renamed), expected, `${label} resolution`);
      assert.deepEqual(comments(renamed), comments(program), `${label} comments`);
      if (fixture.expectedObservation !== undefined) {
        assert.deepEqual(observe(renamed.emit()), fixture.expectedObservation, `${label} renamed observation`);
      }
    }

    recordIssue195Observations({
      requirementId: `I195-RENAME-${issue195Slug(language)}`,
      suffix: 'positive-and-negative',
      fixtureId: `planned:binding-rename:${language}`,
      fixtureFile: ISSUE_195_FIXTURE_FILES.fourLanguage,
      assertions: ASSERTIONS,
      testName,
    });
  });
}
