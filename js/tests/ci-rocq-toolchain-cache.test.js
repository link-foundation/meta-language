import assert from 'node:assert/strict';
import {mkdtempSync, readFileSync, rmSync, writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readRocqToolchainCache, verifyRocqToolchainCache, selectRocqToolchainCache} from '../../scripts/ci-rocq-toolchain-cache.mjs';

function toolchainFixture() {
  const calls = [], identities = new Map([
    ['var prefix', '/workspace/_opam'], ['--version', '2.5.1'], ['exec -- ocamlc -version', '5.4.0'],
    ['exec -- ocamlc -config', 'version: 5.4.0\nc_compiler: gcc\narchitecture: amd64'],
    ['exec -- ocamlc -config-var c_compiler', 'gcc'], ['exec -- gcc --version', 'gcc (Ubuntu) 14.2.0'],
    ['var rocq-core:version', '9.3.0'], ['var rocq-stdlib:version', '9.2.0'], ['var rocq-prover:version', 'meta.1'],
    ['exec -- rocq --version', 'The Rocq Prover, version 9.3.0\ncompiled with OCaml 5.4.0'],
  ]);
  const options = {env: {CI: 'true', GITHUB_ACTIONS: 'true', GITHUB_WORKSPACE: '/workspace'},
    platform: 'linux', architecture: 'x64', files: {
      realpathSync: value => value, lstatSync: () => ({isDirectory: () => true, isSymbolicLink: () => false}),
      readFileSync: filename => {assert.equal(filename, '/etc/os-release'); return 'ID=ubuntu\nVERSION_ID="24.04"\n';},
    }, run: (command, args, limits) => {
      assert.equal(command, 'opam'); assert.equal(limits.timeout, 10000); assert.equal(limits.maxBuffer, 128 * 1024);
      calls.push(args.join(' ')); assert.ok(identities.has(args.join(' ')), 'only bounded identity queries execute');
      return {status: 0, stdout: identities.get(args.join(' ')) + '\n'};
    }};
  return {options, identities, calls};
}

test('Rocq cache identity binds actual compiler platform and owned switch while every package proof executes', () => {
  const fixture = toolchainFixture(), original = readRocqToolchainCache(fixture.options);
  assert.deepEqual(verifyRocqToolchainCache(original.key, fixture.options), {key: original.key, path: '/workspace/_opam'});
  for (const name of ['rocq-core', 'rocq-stdlib', 'rocq-prover']) assert.ok(fixture.calls.includes('var ' + name + ':version'));
  assert.ok(fixture.calls.includes('exec -- rocq --version'));
  for (const name of ['--version', 'exec -- ocamlc -config', 'exec -- gcc --version']) {
    const changed = toolchainFixture(); changed.identities.set(name, changed.identities.get(name) + ' changed');
    assert.notEqual(readRocqToolchainCache(changed.options).key, original.key);
    assert.throws(() => verifyRocqToolchainCache(original.key, changed.options), /exact compiler and platform/u);
  }
  const version = toolchainFixture(); version.identities.set('exec -- ocamlc -version', '5.4.1');
  assert.notEqual(readRocqToolchainCache(version.options).key, original.key);
  const changed = toolchainFixture(); changed.options.architecture = 'arm64';
  assert.notEqual(readRocqToolchainCache(changed.options).key, original.key);
  const operatingSystem = toolchainFixture(); operatingSystem.options.files.readFileSync = () => 'ID=ubuntu\nVERSION_ID="26.04"\n';
  assert.notEqual(readRocqToolchainCache(operatingSystem.options).key, original.key);
});

test('Rocq cache refuses outside paths symlinks stale package versions and failed identity commands', () => {
  const key = readRocqToolchainCache(toolchainFixture().options).key;
  for (const name of ['rocq-core', 'rocq-stdlib', 'rocq-prover']) {
    const changed = toolchainFixture(); changed.identities.set('var ' + name + ':version', 'old');
    assert.throws(() => verifyRocqToolchainCache(key, changed.options), /version is exact/u);
    for (const selected of ['rocq-core', 'rocq-stdlib', 'rocq-prover']) assert.ok(changed.calls.includes('var ' + selected + ':version'));
  }
  const outside = toolchainFixture(); outside.identities.set('var prefix', '/outside/_opam');
  assert.throws(() => readRocqToolchainCache(outside.options), /only the workspace local/u);
  const linked = toolchainFixture(); linked.options.files.lstatSync = () => ({isDirectory: () => true, isSymbolicLink: () => true});
  assert.throws(() => readRocqToolchainCache(linked.options), /owned directory/u);
  const escaped = toolchainFixture(); escaped.options.files.realpathSync = value => value.endsWith('_opam') ? '/outside/_opam' : value;
  assert.throws(() => readRocqToolchainCache(escaped.options), /escape through a symlink/u);
  const local = toolchainFixture(); local.options.env.CI = 'false';
  assert.throws(() => readRocqToolchainCache(local.options), /requires CI/u);
  const compiler = toolchainFixture(); compiler.identities.set('exec -- ocamlc -config-var c_compiler', 'ccache gcc');
  assert.throws(() => readRocqToolchainCache(compiler.options), /directly executable/u);
  const binary = toolchainFixture(); binary.identities.set('exec -- rocq --version', 'The Rocq Prover, version 9.2.0');
  assert.throws(() => verifyRocqToolchainCache(key, binary.options), /reports version/u);
  const failed = toolchainFixture(); failed.options.run = () => ({status: 1, stderr: 'identity failed'});
  assert.throws(() => readRocqToolchainCache(failed.options), /identity failed/u);
  assert.throws(() => verifyRocqToolchainCache('', toolchainFixture().options), /pre-restore/u);
});

test('unsupported Rocq cache contexts disable only the optimization while actual restored mismatches refuse', () => {
  const original = toolchainFixture(), key = readRocqToolchainCache(original.options).key;
  for (const prepare of [
    fixture => fixture.identities.set('var prefix', '/outside/_opam'),
    fixture => fixture.identities.set('exec -- ocamlc -config-var c_compiler', 'ccache gcc -O2'),
    fixture => {fixture.options.run = () => ({error: new Error('identity unavailable')});},
  ]) {
    const fixture = toolchainFixture(); prepare(fixture);
    const disabled = selectRocqToolchainCache(fixture.options);
    assert.equal(disabled.enabled, false); assert.equal(Object.hasOwn(disabled, 'path'), false);
    assert.equal(Object.hasOwn(disabled, 'key'), false);
  }
  const disabled = toolchainFixture();
  assert.deepEqual(selectRocqToolchainCache({...disabled.options, mode: 'verify', enabled: false}), {enabled: false});
  assert.equal(disabled.calls.length, 0, 'an unsupported context adds no later proof failure to the original install path');
  for (const fault of ['exec -- ocamlc -config', 'var rocq-stdlib:version']) {
    const changed = toolchainFixture(); changed.identities.set(fault, 'different');
    assert.equal(selectRocqToolchainCache({...changed.options, mode: 'verify', expectedKey: key}).enabled, false);
    assert.throws(() => selectRocqToolchainCache({...changed.options, mode: 'verify', expectedKey: key, cacheHit: true}),
      /exact compiler and platform|version is exact/u);
  }
  assert.equal(selectRocqToolchainCache({...original.options, mode: 'verify', expectedKey: key, cacheHit: true}).enabled, true);
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-rocq-cache-'));
  try {
    const output = path.join(directory, 'outputs'), script = fileURLToPath(new URL('../../scripts/ci-rocq-toolchain-cache.mjs', import.meta.url));
    const env = {...process.env, CI: 'false', GITHUB_ACTIONS: 'true', GITHUB_OUTPUT: output,
      ROCQ_TOOLCHAIN_CACHE_ENABLED: 'false', ROCQ_TOOLCHAIN_CACHE_HIT: 'false', ROCQ_TOOLCHAIN_CACHE_KEY: key};
    for (const mode of ['--key', '--verify']) {
      writeFileSync(output, '');
      const result = spawnSync(process.execPath, [script, mode], {env, encoding: 'utf8', timeout: 10000});
      assert.equal(result.status, 0, result.stderr);
      assert.equal(readFileSync(output, 'utf8'), 'enabled=false\n', 'unsupported contexts expose no restorable path or key');
    }
    const refused = spawnSync(process.execPath, [script, '--verify'], {encoding: 'utf8', timeout: 10000,
      env: {...env, ROCQ_TOOLCHAIN_CACHE_ENABLED: 'true', ROCQ_TOOLCHAIN_CACHE_HIT: 'true'}});
    assert.equal(refused.status, 1, 'a reported cache hit cannot turn identity refusal into an uncached success');
  } finally {rmSync(directory, {recursive: true, force: true});}
});

test('Rocq cache workflow keeps every install and proof before an exact local-switch save', () => {
  const workflow = readFileSync(new URL('../../.github/workflows/ci.yml', import.meta.url), 'utf8');
  const blocks = workflow.split('      - name: Identify the installed Rocq compiler cache').slice(1);
  assert.equal(blocks.length, 4);
  for (const block of blocks) {
    const selected = block.slice(0, block.indexOf('      - name: Install JavaScript dependencies'));
    assert.match(selected, /uses: actions\/cache\/restore@v6/u);
    assert.match(selected, /if: \$\{\{ (?:matrix\.target == 'rocq' && )?steps\.rocq-toolchain-cache\.outputs\.enabled == 'true' \}\}/u);
    assert.match(selected, /opam repository add rocq-released https:\/\/rocq-prover.org\/opam\/released\n          opam install --yes 'rocq-core=9\.3\.0' 'rocq-stdlib=9\.2\.0' 'rocq-prover=meta\.1'\n          opam var bin >> "\$GITHUB_PATH"\n          opam exec -- rocq --version\n          node scripts\/ci-rocq-toolchain-cache\.mjs --verify/u);
    assert.match(selected, /steps\.rocq-toolchain-install\.outcome == 'success'/u);
    assert.match(selected, /steps\.rocq-toolchain-install\.outputs\.enabled == 'true'/u);
    assert.ok(selected.indexOf('--verify') < selected.indexOf('uses: actions/cache/save@v6'));
    assert.doesNotMatch(selected, /restore-keys:|continue-on-error:|issue-195-results|issue-195-artifacts/u);
    assert.equal((selected.match(/path: \$\{\{ steps\.rocq-toolchain-cache\.outputs\.path \}\}/gu) ?? []).length, 2);
    assert.equal((selected.match(/key: \$\{\{ steps\.rocq-toolchain-cache\.outputs\.key \}\}/gu) ?? []).length, 2);
  }
});
