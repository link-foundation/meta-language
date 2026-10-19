import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import test from 'node:test';
import {downloadExternalGrammarAssets} from '../scripts/external-grammar-cache.mjs';

function createMaterializationWorkspace(context, {nested = false, outside = false} = {}) {
  const repository = mkdtempSync(join(tmpdir(), 'meta-language-materialization-'));
  const external = outside ? mkdtempSync(join(tmpdir(), 'meta-language-materialization-cache-')) : null;
  context.after(() => {
    rmSync(repository, {recursive: true, force: true});
    if (external) rmSync(external, {recursive: true, force: true});
  });
  execFileSync('git', ['init', '--quiet'], {cwd: repository});
  const root = nested ? join(repository, 'project') : repository;
  mkdirSync(root, {recursive: true});
  const bytes = Buffer.from('pinned grammar λ\n');
  const path = 'vendor/source.txt';
  const options = {root, cacheRoot: external ?? join(root, '.external-grammars'),
    manifest: {schemaVersion: 1, assets: {[path]: {group: 'sources', bytes: bytes.length,
      sha256: createHash('sha256').update(bytes).digest('hex'), encoding: 'identity',
      downloads: [{url: 'https://assets.example.test/immutable-source', transform: 'identity'}]}}}};
  const prefix = nested ? '/project' : '';
  writeFileSync(join(repository, '.gitignore'), `${prefix}/.external-grammars/\n`);
  return {repository, options, path, bytes, prefix};
}

test('materialized downloads require ignored untracked destinations before acquisition for every cache location', async context => {
  for (const nested of [false, true]) for (const outside of [false, true]) {
    const {repository, options, path, bytes, prefix} = createMaterializationWorkspace(context, {nested, outside});
    let requests = 0;
    const acquire = {...options, all: true, materialize: true,
      fetchImpl: async () => { requests++; return new Response(bytes); }};
    for (const materialize of [true, 1, 'materialize']) {
      await assert.rejects(downloadExternalGrammarAssets({...acquire, materialize}), /materialized input must be explicitly ignored by Git/u);
    }
    assert.equal(requests, 0);
    assert.equal(existsSync(join(options.cacheRoot, path)), false);
    assert.equal(existsSync(join(options.root, path)), false);
    const ignore = `${prefix}/.external-grammars/\n${prefix}/vendor/\n`;
    writeFileSync(join(repository, '.gitignore'), ignore);
    assert.deepEqual(await downloadExternalGrammarAssets(acquire), [{path, reused: false}]);
    assert.equal(requests, 1);
    assert.deepEqual(readFileSync(join(options.root, path)), bytes);
    assert.equal(readFileSync(join(repository, '.gitignore'), 'utf8'), ignore);
  }
});

test('ignored tracked materialized inputs refuse before cache replay or any destination modification', async context => {
  for (const nested of [false, true]) for (const outside of [false, true]) {
    const {repository, options, path, bytes, prefix} = createMaterializationWorkspace(context, {nested, outside});
    writeFileSync(join(repository, '.gitignore'), `${prefix}/.external-grammars/\n${prefix}/vendor/\n`);
    mkdirSync(join(options.root, 'vendor'), {recursive: true});
    const target = join(options.root, path);
    writeFileSync(target, bytes);
    execFileSync('git', ['add', '--force', '--', `${nested ? 'project/' : ''}${path}`], {cwd: repository});
    let requests = 0;
    const acquire = {...options, all: true, materialize: true,
      fetchImpl: async () => { requests++; return new Response(bytes); }};
    const index = execFileSync('git', ['ls-files', '--stage', '-z'], {cwd: repository});
    await assert.rejects(downloadExternalGrammarAssets(acquire), /materialized input must be untracked/u);
    assert.equal(requests, 0);
    assert.equal(existsSync(join(options.cacheRoot, path)), false);
    assert.deepEqual(readFileSync(target), bytes);
    assert.deepEqual(execFileSync('git', ['ls-files', '--stage', '-z'], {cwd: repository}), index);
    mkdirSync(join(options.cacheRoot, 'vendor'), {recursive: true});
    writeFileSync(join(options.cacheRoot, path), bytes);
    writeFileSync(target, 'tracked edited source');
    await assert.rejects(downloadExternalGrammarAssets(acquire), /materialized input must be untracked/u);
    assert.equal(requests, 0);
    assert.equal(readFileSync(target, 'utf8'), 'tracked edited source');
    assert.deepEqual(readFileSync(join(options.cacheRoot, path)), bytes);
    assert.deepEqual(execFileSync('git', ['ls-files', '--stage', '-z'], {cwd: repository}), index);
  }
});

test('all selected materialized paths are admitted before an otherwise valid first download starts', async context => {
  const {repository, options, path, bytes} = createMaterializationWorkspace(context);
  options.manifest.assets['unignored/source.txt'] = structuredClone(options.manifest.assets[path]);
  writeFileSync(join(repository, '.gitignore'), '/.external-grammars/\n/vendor/\n');
  let requests = 0;
  await assert.rejects(downloadExternalGrammarAssets({...options, all: true, jobs: 1, materialize: true,
    fetchImpl: async () => { requests++; return new Response(bytes); }}), /materialized input must be explicitly ignored by Git: unignored\/source.txt/u);
  assert.equal(requests, 0);
  assert.equal(existsSync(join(options.cacheRoot, path)), false);
  assert.equal(existsSync(join(options.root, path)), false);
  // Cache-only acquisition remains available without a materialized source.
  assert.deepEqual(await downloadExternalGrammarAssets({...options, assets: [path],
    fetchImpl: async () => { requests++; return new Response(bytes); }}), [{path, reused: false}]);
  assert.equal(requests, 1);
  assert.deepEqual(readFileSync(join(options.cacheRoot, path)), bytes);
  assert.equal(existsSync(join(options.root, path)), false);
  writeFileSync(join(repository, '.gitignore'), '/.external-grammars/\n');
  await assert.rejects(downloadExternalGrammarAssets({...options, assets: [path], materialize: true,
    fetchImpl: () => { throw new Error('cached bytes do not authorize an unignored destination'); }}), /materialized input must be explicitly ignored by Git/u);
  assert.deepEqual(readFileSync(join(options.cacheRoot, path)), bytes);
  assert.equal(existsSync(join(options.root, path)), false);
});
