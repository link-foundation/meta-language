#!/usr/bin/env node
// CI toolchain cache identity only. Tests and observations never enter this cache.
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import * as filesystem from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';

const selectedPackages = [['rocq-core', '9.3.0'], ['rocq-stdlib', '9.2.0'], ['rocq-prover', 'meta.1']];

/** Query actual installed identities; accept only this job's owned local switch. */
export function readRocqToolchainCache({env = process.env, platform = process.platform,
  architecture = process.arch, run = spawnSync, files = filesystem} = {}) {
  assert.equal(env.CI, 'true', 'Rocq toolchain caching requires CI');
  assert.equal(env.GITHUB_ACTIONS, 'true', 'Rocq toolchain caching requires GitHub Actions');
  assert.equal(platform, 'linux', 'the current Rocq cache layout belongs to Linux jobs');
  assert.ok(env.GITHUB_WORKSPACE && !/[\r\n\0]/u.test(env.GITHUB_WORKSPACE), 'a cache has one workspace path');
  const workspace = files.realpathSync(env.GITHUB_WORKSPACE), expected = path.join(workspace, '_opam');
  const execute = (command, args) => {
    const result = run(command, args, {env, encoding: 'utf8', timeout: 10000, maxBuffer: 128 * 1024});
    if (result.error) throw result.error;
    assert.equal(result.status, 0, command + ' ' + args.join(' ') + ': ' + (result.stderr ?? ''));
    const output = (result.stdout ?? '').trim();
    assert.ok(output && !output.includes('\0'), 'a toolchain identity query returns actual text');
    return output;
  };
  const prefix = execute('opam', ['var', 'prefix']);
  assert.equal(prefix, expected, 'only the workspace local _opam switch may be cached');
  const stat = files.lstatSync(prefix);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), 'a cached switch is an owned directory');
  assert.equal(files.realpathSync(prefix), expected, 'a cached switch cannot escape through a symlink');
  const opamVersion = execute('opam', ['--version']);
  const compilerVersion = execute('opam', ['exec', '--', 'ocamlc', '-version']);
  assert.match(compilerVersion, /^5\.4\.[0-9]+(?:\+[^\s]+)?$/u, 'the resolved compiler belongs to the requested 5.4 series');
  const compilerConfiguration = execute('opam', ['exec', '--', 'ocamlc', '-config']);
  const cCompiler = execute('opam', ['exec', '--', 'ocamlc', '-config-var', 'c_compiler']);
  assert.match(cCompiler, /^[A-Za-z0-9_./+-]+$/u, 'the current layout reports one directly executable C compiler');
  const cCompilerVersion = execute('opam', ['exec', '--', cCompiler, '--version']);
  const operatingSystem = files.readFileSync('/etc/os-release', 'utf8');
  assert.ok(operatingSystem.trim(), 'the actual operating system identity is present');
  const identity = ['rocq-local-switch-v1', platform, architecture, operatingSystem, workspace, prefix,
    opamVersion, compilerVersion, compilerConfiguration, cCompiler, cCompilerVersion, selectedPackages];
  const key = 'rocq-local-switch-v1-' + createHash('sha256').update(JSON.stringify(identity)).digest('hex');
  return {key, path: prefix, execute};
}

/** Verify restored compiler identity and all selected installed package versions. */
export function verifyRocqToolchainCache(expectedKey, options) {
  assert.match(expectedKey ?? '', /^rocq-local-switch-v1-[0-9a-f]{64}$/u, 'the pre-restore toolchain key is required');
  const current = readRocqToolchainCache(options);
  assert.equal(current.key, expectedKey, 'a restored toolchain must retain the exact compiler and platform identity');
  const installedPackages = selectedPackages.map(([name, version]) =>
    [name, version, current.execute('opam', ['var', name + ':version'])]);
  for (const [name, expected, actual] of installedPackages) {
    assert.equal(actual, expected, 'the installed ' + name + ' version is exact');
  }
  const version = current.execute('opam', ['exec', '--', 'rocq', '--version']);
  assert.ok(version.split(/\r?\n/u).some(line => line.includes('Rocq') && /(?:^|[^0-9])9\.3\.0(?:[^0-9.]|$)/u.test(line)),
    'the selected Rocq executable reports version 9.3.0');
  return {key: current.key, path: current.path};
}

/** Unsupported cold-cache contexts retain the original uncached workflow. */
export function selectRocqToolchainCache({mode = 'key', enabled = true, cacheHit = false, expectedKey, ...options} = {}) {
  if (!enabled && !cacheHit) return {enabled: false};
  try {
    const current = mode === 'verify' ? verifyRocqToolchainCache(expectedKey, options) : readRocqToolchainCache(options);
    return {enabled: true, key: current.key, path: current.path};
  } catch (error) {
    if (mode === 'verify' && cacheHit) throw error;
    return {enabled: false, reason: error.message};
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    assert.equal(process.argv.length, 3, 'select --key or --verify');
    assert.ok(['--key', '--verify'].includes(process.argv[2]), 'select --key or --verify');
    const current = selectRocqToolchainCache({mode: process.argv[2].slice(2),
      enabled: process.argv[2] === '--key' || process.env.ROCQ_TOOLCHAIN_CACHE_ENABLED === 'true',
      cacheHit: process.env.ROCQ_TOOLCHAIN_CACHE_HIT === 'true', expectedKey: process.env.ROCQ_TOOLCHAIN_CACHE_KEY});
    if (!current.enabled) console.warn('Rocq cache disabled: ' + (current.reason ?? 'identity was not established'));
    if (process.env.GITHUB_OUTPUT) {
      try {filesystem.appendFileSync(process.env.GITHUB_OUTPUT, current.enabled
        ? 'key=' + current.key + '\npath=' + current.path + '\nenabled=true\n' : 'enabled=false\n');}
      catch (error) {console.warn('Rocq cache outputs unavailable: ' + error.message);}
    }
  } catch (error) {
    if (process.env.ROCQ_TOOLCHAIN_CACHE_HIT === 'true') {
      console.error(error.message);
      process.exitCode = 1;
    } else console.warn('Rocq cache disabled: ' + error.message);
  }
}
