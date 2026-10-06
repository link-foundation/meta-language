// Cleanup of the regenerable grammar work in .grammar-cache: imported grammar
// corpora, generator and oracle builds and merged-grammar caches are cache
// classes under the same safety rules, and canonical native grammars,
// fixtures, licenses and evidence are never removed
// (requirement I195-CACHE-CLEANUP-GRAMMAR-CACHES).
import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import { CACHE_CLASSES, GRAMMAR_CACHE_KINDS, GRAMMAR_CACHE_ROOT, grammarCacheDirectory } from '../../scripts/lib/cache-classes.mjs';
import { runCleanup } from '../../scripts/lib/cache-cleanup.mjs';
import { git, makeBase, makeRepository, put } from './support/cache-fixtures.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const KIB = 1024;
const REQUIREMENT = 'I195-CACHE-CLEANUP-GRAMMAR-CACHES';

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${REQUIREMENT.toLowerCase()}`,
    fixtureFile: 'scripts/lib/cache-classes.mjs',
    assertions,
    testName,
    runtime: 'tooling',
  });
}

const clean = (fixture, options = {}) => runCleanup({ cwd: fixture.root, tmpRoot: fixture.tmpRoot, docker: false, preflight: false, ...options });
const grammarClass = (report) => report.classes['grammar-caches'];
const cached = (fixture, kind, name, file, bytes) => put(grammarCacheDirectory(fixture.root, kind, name), file, bytes);

/** A fixture repository with one cache of every grammar kind and the canonical files beside them. */
function fixtureWithGrammarCaches(t) {
  const fixture = makeRepository(makeBase(t));
  const { root, env } = fixture;
  const canonical = {
    nativeGrammar: put(root, 'grammars/rust/grammar.lino', '(grammar rust)\n'),
    grammarFixture: put(root, 'parity/fixtures/grammar/rust.json', '{}\n'),
    license: put(root, 'js/src/vendor/tree-sitter-demo/LICENSE', 'MIT\n'),
    pinnedRevision: put(root, `${GRAMMAR_CACHE_ROOT}/corpora/pinned/REVISION`, 'abc123\n'),
  };
  git(root, ['add', 'grammars', 'parity', 'js/src/vendor'], env);
  git(root, ['add', '-f', canonical.pinnedRevision], env);
  git(root, ['commit', '-q', '-m', 'canonical grammar files'], env);
  canonical.evidence = put(root, 'issue-195-artifacts/grammar/merge-report.json', '{}\n');
  const caches = {
    corpus: cached(fixture, 'corpora', 'tree-sitter-rust@abc123', 'grammar.js', 32 * KIB),
    oracle: cached(fixture, 'oracles', 'antlr4-4.13.2', 'antlr.jar', 16 * KIB),
    merged: cached(fixture, 'merged', 'rust', 'merged.lino', 8 * KIB),
  };
  return { ...fixture, canonical, caches, pinned: path.dirname(canonical.pinnedRevision) };
}

test('the grammar caches are one registered class covering corpora, oracle builds and merged grammars', () => {
  const cacheClass = CACHE_CLASSES.find(({ id }) => id === 'grammar-caches');
  assert.ok(cacheClass, 'the grammar-caches class is registered');
  assert.deepEqual(cacheClass.covers, ['grammar-corpora', 'oracle-build', 'merged-grammar-cache']);
  assert.equal(cacheClass.activeSensitive, true, 'a running oracle or merge keeps its cache');
  assert.deepEqual(Object.keys(GRAMMAR_CACHE_KINDS), ['corpora', 'oracles', 'merged']);
  assert.equal(grammarCacheDirectory('/repository', 'merged', 'rust'), path.join('/repository', GRAMMAR_CACHE_ROOT, 'merged', 'rust'));
  for (const name of ['..', '.', 'a/b', 'a\\b', '']) {
    assert.throws(() => grammarCacheDirectory('/repository', 'corpora', name), /single path segment/u, JSON.stringify(name));
  }
  assert.throws(() => grammarCacheDirectory('/repository', 'grammars', 'rust'), /unknown grammar cache kind/u);
  observe(
    ['grammarCorporaClassRegistered', 'oracleBuildsClassRegistered', 'mergedGrammarCachesClassRegistered'],
    'the grammar caches are one registered class covering corpora, oracle builds and merged grammars',
  );
});

test('a full clean removes every grammar cache and keeps canonical grammars, fixtures, licenses, tracked files and evidence', (t) => {
  const fixture = fixtureWithGrammarCaches(t);
  const report = clean(fixture, { mode: 'full' });
  for (const [role, file] of Object.entries(fixture.caches)) assert.ok(!existsSync(file), `${role} ${file} is removed`);
  for (const [role, file] of Object.entries(fixture.canonical)) assert.ok(existsSync(file), `${role} ${file} survives`);
  const entry = grammarClass(report);
  assert.deepEqual(entry.removed.map(({ path: removed }) => removed).sort(), [
    '.grammar-cache/corpora/tree-sitter-rust@abc123',
    '.grammar-cache/merged/rust',
    '.grammar-cache/oracles/antlr4-4.13.2',
  ]);
  assert.ok(entry.removed.every(({ bytes }) => bytes > 0), 'every removal reports its bytes');
  assert.match(entry.skipped.find(({ path: skipped }) => skipped === '.grammar-cache/corpora/pinned').reason, /tracked/u);
  assert.equal(git(fixture.root, ['status', '--porcelain']), '', 'the worktree is unchanged');
  const repeat = clean(fixture, { mode: 'full' });
  assert.deepEqual(grammarClass(repeat).removed, [], 'a second clean removes nothing');
  observe(
    ['grammarCorporaClassRegistered', 'oracleBuildsClassRegistered', 'mergedGrammarCachesClassRegistered', 'canonicalGrammarsPreserved'],
    'a full clean removes every grammar cache and keeps canonical grammars, fixtures, licenses, tracked files and evidence',
  );
});

test('pruning removes merged-grammar caches first and imported corpora last', (t) => {
  const fixture = fixtureWithGrammarCaches(t);
  const generous = clean(fixture, { budgetMb: 64 });
  assert.deepEqual(grammarClass(generous).removed, [], 'grammar caches within the budget are kept');
  const tight = clean(fixture, { budgetMb: (40 * KIB) / (1024 * 1024) });
  assert.deepEqual(grammarClass(tight).removed.map(({ path: removed }) => removed), [
    '.grammar-cache/merged/rust',
    '.grammar-cache/oracles/antlr4-4.13.2',
  ]);
  assert.ok(existsSync(fixture.caches.corpus), 'the corpus, which needs the network to restore, is kept last');
  clean(fixture, { budgetMb: 0 });
  assert.ok(!existsSync(fixture.caches.corpus));
  for (const [role, file] of Object.entries(fixture.canonical)) assert.ok(existsSync(file), `${role} ${file} survives`);
  observe(['canonicalGrammarsPreserved'], 'pruning removes merged-grammar caches first and imported corpora last');
});
