// Issue #195 structured transformation evidence: for every language of the
// shared four-language corpus, each public structured operation runs on a
// program constructed from fragments and reloaded from a snapshot that never
// held the original source buffer. Every intermediate program's derived
// metadata (CST, spans, source mappings, diagnostics) must equal a fresh parse
// of its emitted text, the edit sequence must reach the intended structure,
// and a follow-up edit must restore the constructed program.
// The Rust twin is rust/tests/unit/issue_195_structured_transformations.rs.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  constructProgram,
  constructProgramFromFragments,
  ProgramRepresentation,
} from '../src/index.js';
import {
  ISSUE_195_FIXTURE_FILES,
  issue195Slug,
  recordIssue195Observations,
} from './support/issue-195-observations.js';

const corpus = JSON.parse(await readFile(new URL('../../parity/fixtures/four-language-conformance.json', import.meta.url)));

const ASSERTIONS = [
  'publicApiUsed',
  'editSequence',
  'serializeReload',
  'constructionWithoutOriginalSource',
  'treeIntegrity',
  'spansUpdated',
  'sourceMappingsUpdated',
  'diagnosticsUpdated',
  'originalBufferDiscarded',
  'emittedSourceReparses',
  'intendedStructureObserved',
];

const DECLARED = /^(first|second|third|primary)$/iu;

// Declaration names in source order, as the CST reports them.
function declaredNames(program) {
  return program.querySyntax('identifier')
    .map(({ start, end }) => program.source.slice(start, end))
    .filter((name) => DECLARED.test(name))
    .map((name) => name.toLowerCase());
}

function nameRange(program, name) {
  const range = program.querySyntax('identifier')
    .find(({ start, end }) => program.source.slice(start, end).toLowerCase() === name);
  assert.ok(range, `${program.language} declares ${name}`);
  return range;
}

// The derived metadata of an edited program must be exactly what a fresh parse
// of its emitted text derives, and every mapping must slice the new source.
function assertConsistent(program, expected, label) {
  const emitted = program.emit();
  assert.equal(emitted, expected, `${label} emits`);
  assert.equal(program.source, emitted, `${label} source`);
  assert.equal(program.network.verifyFullMatch().isClean(), true, `${label} tree integrity`);
  const root = program.sourceMappings.reduce((widest, mapping) =>
    mapping.end - mapping.start > widest.end - widest.start ? mapping : widest);
  assert.deepEqual([root.start, root.end], [0, emitted.length], `${label} root covers source`);
  for (const mapping of program.sourceMappings) {
    assert.ok(mapping.start >= 0 && mapping.start <= mapping.end && mapping.end <= emitted.length, label);
    const text = emitted.slice(mapping.start, mapping.end);
    assert.equal(mapping.byteStart, Buffer.byteLength(emitted.slice(0, mapping.start)), `${label} byteStart`);
    assert.equal(mapping.byteEnd - mapping.byteStart, Buffer.byteLength(text), `${label} byte span`);
  }
  const reparsed = constructProgram(emitted, program.language, program.project);
  assert.deepEqual(program.sourceMappings, reparsed.sourceMappings, `${label} source mappings`);
  assert.deepEqual(program.diagnostics, reparsed.diagnostics, `${label} diagnostics`);
  assert.deepEqual(program.normalized(), reparsed.normalized(), `${label} reparses`);
}

// Each operation: the edit under test, its intended result, and the follow-up
// edit that restores the constructed program.
function operations(fixture) {
  const { first, second, inserted, source } = fixture;
  const firstRange = { start: 0, end: first.length };
  const secondRange = { start: first.length, end: source.length };
  const replacement = first.replace(/first|FIRST/u, 'primary');
  return {
    construct: {
      apply: (program) => ProgramRepresentation.fromSnapshot(
        constructProgramFromFragments([second, first], program.language, program.project).serializeSnapshot(),
      ),
      expected: second + first,
      names: ['second', 'first'],
      restore: (edited) => edited.move({ start: 0, end: second.length }, edited.source.length),
    },
    query: {
      apply: (program) => program.replace(nameRange(program, 'first'), 'primary'),
      expected: replacement + second,
      names: ['primary', 'second'],
      restore: (edited) => edited.replace(nameRange(edited, 'primary'), fixture.language === 'Rust' ? 'FIRST' : 'first'),
    },
    insert: {
      apply: (program) => program.insert(source.length, inserted),
      expected: source + inserted,
      names: ['first', 'second', 'third'],
      restore: (edited) => edited.delete({ start: source.length, end: source.length + inserted.length }),
    },
    replace: {
      apply: (program) => program.replace(firstRange, replacement),
      expected: replacement + second,
      names: ['primary', 'second'],
      restore: (edited) => edited.replace({ start: 0, end: replacement.length }, first),
    },
    delete: {
      apply: (program) => program.delete(secondRange),
      expected: first,
      names: ['first'],
      restore: (edited) => edited.insert(first.length, second),
    },
    clone: {
      apply: (program) => program.clone(firstRange, source.length),
      expected: source + first,
      names: ['first', 'second', 'first'],
      restore: (edited) => edited.delete({ start: source.length, end: source.length + first.length }),
    },
    move: {
      apply: (program) => program.move(secondRange, 0),
      expected: second + first,
      names: ['second', 'first'],
      restore: (edited) => edited.move({ start: second.length, end: source.length }, 0),
    },
  };
}

for (const fixture of corpus.transformationPrograms) {
  const phantom = corpus.phantomImportCases.find(({ language }) => language === fixture.language);
  for (const [operation, { apply, expected, names, restore }] of Object.entries(operations(fixture))) {
    const testName = `issue 195 ${fixture.language} structured ${operation} runs on a reloaded program and reparses`;
    test(testName, () => {
      assert.ok(phantom, `${fixture.language} has a phantom import case`);
      const project = { root: `/workspace/${fixture.language}`, files: ['main'], dependencies: [] };

      // Construct from fragments, persist, and drop every reference to the
      // constructed program: the reload is the only input from here on.
      let constructed = constructProgramFromFragments([fixture.first, fixture.second], fixture.language, project);
      const serialized = constructed.serializeSnapshot();
      constructed = undefined;
      assert.equal(Object.hasOwn(JSON.parse(serialized), 'source'), false, 'snapshot holds no source buffer');
      assert.equal(serialized.includes(JSON.stringify(fixture.source)), false, 'snapshot holds no joined source');
      const program = ProgramRepresentation.fromSnapshot(serialized);
      assertConsistent(program, fixture.source, `${fixture.language} reload`);
      assert.deepEqual(declaredNames(program), ['first', 'second']);

      const edited = apply(program);
      assert.notEqual(edited, program, `${operation} returns a new program`);
      assert.equal(program.emit(), fixture.source, `${operation} leaves its input intact`);
      assertConsistent(edited, expected, `${fixture.language} ${operation}`);
      assert.deepEqual(declaredNames(edited), names, `${fixture.language} ${operation} structure`);

      // Spans follow the text: a prefix shifts every declaration name by its
      // length, and the unresolved import surfaces as a project diagnostic
      // that disappears again with the import.
      const withImport = edited.insert(0, phantom.source);
      assertConsistent(withImport, phantom.source + expected, `${fixture.language} ${operation} import`);
      const shifted = withImport.querySyntax('identifier')
        .filter(({ start, end }) => DECLARED.test(withImport.source.slice(start, end)));
      const original = edited.querySyntax('identifier')
        .filter(({ start, end }) => DECLARED.test(edited.source.slice(start, end)));
      assert.deepEqual(
        shifted,
        original.map(({ start, end }) => ({ start: start + phantom.source.length, end: end + phantom.source.length })),
        `${fixture.language} ${operation} spans shift`,
      );
      assert.equal(edited.diagnostics.some(({ kind }) => kind === 'missing-project-context'), false);
      const missing = withImport.diagnostics.filter(({ kind }) => kind === 'missing-project-context');
      assert.equal(missing.length, 1, `${fixture.language} ${operation} missing dependency`);
      assert.equal(missing[0].term, phantom.dependency);
      assert.ok(missing[0].start >= 0 && missing[0].end <= phantom.source.length);
      const withoutImport = withImport.delete({ start: 0, end: phantom.source.length });
      assertConsistent(withoutImport, expected, `${fixture.language} ${operation} import removed`);
      assert.deepEqual(withoutImport.diagnostics, edited.diagnostics);

      // The follow-up edit restores the constructed program exactly.
      const restored = restore(edited);
      assertConsistent(restored, fixture.source, `${fixture.language} ${operation} restored`);
      assert.deepEqual(restored.normalized(), program.normalized(), `${fixture.language} ${operation} round trip`);
      assert.deepEqual(declaredNames(restored), ['first', 'second']);

      recordIssue195Observations({
        requirementId: `I195-XFORM-${issue195Slug(fixture.language)}-${operation}`,
        suffix: 'positive',
        fixtureId: `planned:transformation:${fixture.language}:${operation}`,
        fixtureFile: ISSUE_195_FIXTURE_FILES.fourLanguage,
        assertions: ASSERTIONS,
        testName,
      });
    });
  }
}
