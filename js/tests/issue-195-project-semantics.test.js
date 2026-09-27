// Issue #195 project-aware semantics evidence: for every language of the
// four-language corpus, a real multi-file project (manifest, module tree and
// entry program) is loaded, every semantic construct links the entry program
// to the declarations it names in other project files, missing or broken
// project context is diagnosed without fabricated links, and the valid
// context enables the construct's behavior (imports, macro and notation
// expansion, tactics, attributes, effects).
// The Rust twin is rust/tests/unit/issue_195_project_semantics.rs; the
// projects themselves are in experiments/issue-195-projects.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import { analyzeProgram, SEMANTIC_CONSTRUCTS } from '../src/index.js';
import {
  ISSUE_195_FIXTURE_FILES,
  issue195Slug,
  recordIssue195Observations,
} from './support/issue-195-observations.js';

const corpus = JSON.parse(await readFile(new URL('../../parity/fixtures/four-language-conformance.json', import.meta.url)));

const ASSERTIONS = [
  'projectContextLoaded',
  'languageSpecificStructurePreserved',
  'sourceMappingPreserved',
  'missingContextDiagnosed',
  'validContextEnablesBehavior',
];

function analyze(fixture, sources) {
  const entry = fixture.sources.find(({ path }) => path === fixture.entry).source;
  return analyzeProgram(entry, fixture.language, {
    root: fixture.root,
    entry: fixture.entry,
    sources,
    files: sources.map(({ path }) => path),
    dependencies: fixture.dependencies,
  });
}

function projectEvidence(program, construct, sources) {
  const text = (file, start, end) => sources.find(({ path }) => path === file).source.slice(start, end);
  return program.constructs.find(({ kind }) => kind === construct).evidence
    .filter(({ file }) => file !== undefined)
    .map(({ kind, name, file, start, end }) => ({ kind, name, file, text: text(file, start, end) }));
}

function diagnostics(program) {
  return program.diagnostics.map(({ kind, term, start, end }) => ({ kind, term, text: program.source.slice(start, end) }));
}

test('issue 195 project programs cover every language and construct', () => {
  assert.deepEqual(corpus.projectPrograms.map(({ language }) => language), corpus.languages.map(({ name }) => name));
  for (const fixture of corpus.projectPrograms) {
    assert.deepEqual(Object.keys(fixture.constructs), [...SEMANTIC_CONSTRUCTS]);
    for (const construct of SEMANTIC_CONSTRUCTS) {
      assert.ok(fixture.constructs[construct].length > 0, `${fixture.language} ${construct} has expected project evidence`);
    }
  }
});

for (const fixture of corpus.projectPrograms) {
  const program = analyze(fixture, fixture.sources);
  const entrySource = fixture.sources.find(({ path }) => path === fixture.entry);
  const bare = analyze(fixture, []);
  const alone = analyze(fixture, [entrySource]);
  const broken = fixture.brokenContexts.map((context) => ({
    context,
    program: analyze(fixture, fixture.sources.map((source) =>
      context.sources.find(({ path }) => path === source.path) ?? source)),
  }));

  for (const construct of SEMANTIC_CONSTRUCTS) {
    const requirementId = `I195-SEM-${issue195Slug(fixture.language)}-${construct}`;
    const testName = `issue 195 ${fixture.language} ${construct} is project-aware`;
    test(testName, () => {
      // projectContextLoaded: every module request resolves to a project file,
      // every source file and manifest is read, and nothing is diagnosed.
      assert.deepEqual(diagnostics(program), []);
      assert.deepEqual(
        program.projectModules.map(({ request, module, start, end }) => ({ request, module, text: program.source.slice(start, end) })),
        fixture.modules,
      );
      const files = program.projectFacts.filter(({ kind }) => kind === 'project-file').map(({ name }) => name);
      assert.deepEqual(files, fixture.sources.map(({ path }) => path));
      assert.ok(program.projectFacts.some(({ kind }) => kind === 'project-manifest'), 'the project manifest is read');

      // languageSpecificStructurePreserved: the construct's links carry the
      // language's own symbol identity, roles and declaration kinds.
      const evidence = projectEvidence(program, construct, fixture.sources);
      assert.equal(program.constructs.find(({ kind }) => kind === construct).status, 'represented');
      assert.deepEqual(evidence.map(({ kind, name, file }) => ({ kind, name, file })),
        fixture.constructs[construct].map(({ kind, name, file }) => ({ kind, name, file })));

      // sourceMappingPreserved: every link spans exactly the expected source
      // text, and every declaration range lies within its project file.
      assert.deepEqual(evidence, fixture.constructs[construct]);
      for (const reference of program.projectReferences) {
        const target = fixture.sources.find(({ path }) => path === reference.file);
        assert.ok(target, `${reference.symbol} is declared in a project file`);
        assert.ok(reference.declaration.start < reference.declaration.end && reference.declaration.end <= target.source.length);
        assert.ok(reference.symbol.startsWith(`${reference.file}#`), reference.symbol);
      }

      // missingContextDiagnosed: without the other project files, the module
      // requests are diagnosed and no project link is fabricated; a broken
      // project reports exactly what is wrong.
      for (const missing of [bare, alone]) {
        assert.deepEqual(diagnostics(missing), fixture.missingContext);
        assert.deepEqual(missing.projectModules, []);
        assert.deepEqual(missing.projectReferences, []);
        assert.deepEqual(missing.expansions, []);
      }
      for (const { context, program: brokenProgram } of broken) {
        assert.deepEqual(diagnostics(brokenProgram), context.diagnostics, context.description);
      }

      // validContextEnablesBehavior: the valid project enables the links and
      // expansions that the missing context cannot produce.
      assert.ok(evidence.length > 0);
      assert.deepEqual(
        program.expansions.map(({ name, kind, start, end, expansion, target }) =>
          ({ name, kind, text: program.source.slice(start, end), expansion, target })),
        fixture.expansions,
      );
      assert.deepEqual(bare.constructs.find(({ kind }) => kind === construct).evidence.filter(({ file }) => file !== undefined), []);
      assert.ok(program.projectReferences.length > 0 && program.projectModules.length > 0);

      recordIssue195Observations({
        requirementId,
        suffix: 'positive-and-negative',
        fixtureId: `planned:semantic:${fixture.language}:${construct}`,
        fixtureFile: ISSUE_195_FIXTURE_FILES.fourLanguage,
        assertions: ASSERTIONS,
        testName,
      });
    });
  }
}
