// Behavioral tooling checks. Git inspection uses argument arrays; staging and
// destructive cleanup happen only in a caller-owned temporary repository.
import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { cleanMaterializedExternalGrammarAssets, downloadExternalGrammarAssets,
  externalGrammarPath } from './external-grammar-cache.mjs';
import { readStructuredDataLino } from '../src/structured-data-links.js';

const git = (root, args, input) => execFileSync('git', args,
  { cwd: root, encoding: 'utf8', ...(input === undefined ? {} : { input }), maxBuffer: 32 * 1024 * 1024 });
const names = value => value.split('\0').filter(Boolean);
const safePath = file => {
  assert.equal(typeof file, 'string');
  assert.ok(file && !/^[\\/]|[\\\0]/u.test(file)
    && file.split('/').every(part => part && part !== '.' && part !== '..'), `safe repository path: ${file}`);
  return file;
};
const put = (root, file, bytes) => {
  safePath(file);
  mkdirSync(dirname(join(root, file)), { recursive: true });
  writeFileSync(join(root, file), bytes);
};

/** Check every registered asset and every prepared native grammar against Git. */
export function auditExternalGrammarStorage(root, { manifest, inventory, provenanceFiles, cacheDirectory }) {
  assert.equal(manifest.schemaVersion, 1);
  assert.match(cacheDirectory, /^\.[a-z][a-z-]*$/u);
  const assetPaths = Object.keys(manifest.assets);
  assetPaths.forEach(safePath);
  assert.ok(assetPaths.length > 0, 'the external asset inventory is not empty');
  const tracked = new Set(names(git(root, ['ls-files', '-z'])));
  for (const file of assetPaths) assert.equal(tracked.has(file), false, `upstream input is untracked: ${file}`);
  const ignoredPaths = [...assetPaths, ...assetPaths.map(file => `${cacheDirectory}/${file}`)];
  const ignored = new Set(names(git(root, ['check-ignore', '--no-index', '-z', '--stdin'], ignoredPaths.join('\0') + '\0')));
  for (const file of ignoredPaths) assert.ok(ignored.has(file), `download path is explicitly ignored: ${file}`);
  const snapshotFiles = [...new Set([
    ...Object.values(inventory.nativeGrammars).map(entry => entry.grammar),
    ...[...tracked].filter(file => /^(?:parity\/grammars\/native\/|(?:js|rust)\/src\/data\/native-grammars\/).*\.lino$/u.test(file)),
  ])];
  assert.ok(snapshotFiles.length > 0, 'the prepared native grammar inventory is not empty');
  const retainedFiles = [...new Set([...snapshotFiles, ...provenanceFiles,
    ...[...tracked].filter(file => /(?:^|[/.])(?:LICENSE|NOTICE|PINS)(?:[./]|$)/iu.test(file)),
  ])];
  retainedFiles.forEach(safePath);
  for (const file of retainedFiles) {
    assert.ok(tracked.has(file), `prepared grammar/provenance remains tracked: ${file}`);
    assert.ok(readFileSync(join(root, file)).length > 0, `prepared grammar/provenance is not empty: ${file}`);
  }
  return { assetPaths, ignoredPaths, snapshotFiles, retainedFiles };
}

/** Exercise ordinary Git staging on the exact registered path inventory. */
export function verifyOrdinaryDownloadStaging(root, scratch, audit) {
  assert.notEqual(resolve(root), resolve(scratch));
  assert.equal(existsSync(join(scratch, '.git')), false, 'only a fresh scratch directory can be staged');
  mkdirSync(scratch, { recursive: true });
  git(scratch, ['init', '--quiet']);
  // Existing forced-tracked metadata stays tracked beneath ignored vendor paths.
  // Contents here are path probes; actual snapshot bytes are checked separately.
  for (const file of audit.retainedFiles) put(scratch, file, 'tracked path probe\n');
  git(scratch, ['add', '--force', '--', ...audit.retainedFiles]);
  copyFileSync(join(root, '.gitignore'), join(scratch, '.gitignore'));
  for (const file of audit.ignoredPaths) put(scratch, file, 'untracked download path probe\n');
  git(scratch, ['add', '--', '.']);
  const staged = new Set(names(git(scratch, ['ls-files', '-z'])));
  for (const file of audit.ignoredPaths) assert.equal(staged.has(file), false, `ordinary staging excludes download: ${file}`);
  for (const file of audit.retainedFiles) assert.ok(staged.has(file), `ordinary staging preserves tracked snapshot/provenance: ${file}`);
  return { excludedPaths: audit.ignoredPaths.length, retainedFiles: audit.retainedFiles.length };
}

/** Replay materialization and verified cleanup of all already-provisioned pins. */
export async function verifyTemporaryGrammarLifeCycle(root, scratch, { manifest, cacheDirectory }) {
  assert.notEqual(resolve(root), resolve(scratch));
  assert.equal(existsSync(join(scratch, '.git')), false, 'lifecycle verification requires fresh scratch');
  mkdirSync(scratch, { recursive: true });
  git(scratch, ['init', '--quiet']);
  copyFileSync(join(root, '.gitignore'), join(scratch, '.gitignore'));
  const cacheRoot = join(scratch, cacheDirectory);
  const paths = Object.keys(manifest.assets);
  // The existing CI provision step obtains these inputs. Reusing its verified
  // bytes avoids downloading the entire inventory a second time.
  for (const file of paths) {
    const original = externalGrammarPath(file, { root, manifest });
    const target = join(cacheRoot, file);
    mkdirSync(dirname(target), { recursive: true });
    copyFileSync(original, target);
  }
  const options = { root: scratch, manifest, cacheRoot };
  const downloads = await downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
    fetchImpl: () => { throw new Error('the provisioned inventory must not be downloaded again'); } });
  assert.deepEqual(downloads.map(entry => entry.path), paths);
  assert.ok(downloads.every(entry => entry.reused), 'every pinned byte was reused and reverified');
  for (const file of paths) assert.equal(externalGrammarPath(file, options), join(scratch, file));
  assert.deepEqual(cleanMaterializedExternalGrammarAssets(options), paths);
  for (const file of paths) assert.equal(existsSync(join(scratch, file)), false, `temporary materialization cleaned: ${file}`);
  rmSync(cacheRoot, { recursive: true, force: true });
  for (const file of paths) assert.equal(existsSync(join(cacheRoot, file)), false, `temporary cache cleaned: ${file}`);
  return { verifiedAssets: downloads.length, materializedAssets: downloads.length, cleanedAssets: paths.length };
}

/** Execute the public native grammar algorithms with file/network denial. */
export function verifyPreparedNativeSnapshots(root, contractFile, { grammarIds = [] } = {}) {
  const guard = fileURLToPath(new URL('../tests/support/deny-external-grammar-assets.js', import.meta.url));
  const script = fileURLToPath(new URL('./check-prepared-native-snapshots.mjs', import.meta.url));
  const child = spawnSync(process.execPath, ['--import', guard, script, contractFile,
    ...grammarIds.flatMap(id => ['--grammar', id])], {
    cwd: root, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024,
    env: { ...process.env, META_LANGUAGE_TOOLING_ROOT: resolve(root) },
  });
  assert.equal(child.status, 0, child.stderr || child.error?.message || `native snapshot process exited ${child.status}`);
  return readStructuredDataLino(child.stdout);
}
