#!/usr/bin/env node
// Compile bounded public consumers with an explicitly supplied existing compiler.
// This checker never downloads a compiler or installs consumer dependencies.
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import {publicDeclarationConsumers as createPublicDeclarationConsumers} from './public-declaration-consumers.mjs';
import {selectVersionedAssertion} from './assertion-version-ranges.mjs';
import {readAssertionVersionPolicy} from './antlr-version-assertions.mjs';

const publicModule = fileURLToPath(new URL('../src/index.js', import.meta.url));

export function publicDeclarationConsumers(moduleSpecifier = publicModule) {
  return createPublicDeclarationConsumers(moduleSpecifier);
}

export function selectPublicDeclarationDiagnostics(consumer, compilerVersion, registry = readAssertionVersionPolicy()) {
  if (consumer.assertionPolicy === undefined) return consumer.expectedDiagnostics;
  const matches = registry.policy.assertions.filter(policy => policy.id === consumer.assertionPolicy);
  if (matches.length !== 1) throw new Error(`${consumer.name}: exactly one registered assertion policy is required for ${consumer.assertionPolicy}`);
  const selected = selectVersionedAssertion(matches[0], compilerVersion);
  if (!Array.isArray(selected.expected) || selected.expected.some(code => typeof code !== 'string' || !/^TS[0-9]+$/u.test(code))) {
    throw new Error(`${consumer.name}: the selected compiler expectation must contain explicit TypeScript diagnostic codes`);
  }
  return selected.expected;
}

export function checkPublicDeclarationContracts(compiler) {
  if (typeof compiler !== 'string' || !compiler.trim()) {
    throw new TypeError('Pass --compiler PATH to an existing TypeScript executable; this checker never installs dependencies.');
  }
  const version = spawnSync(compiler, ['--version'], { encoding: 'utf8', timeout: 20_000, maxBuffer: 256 * 1024 });
  if (version.error) throw new Error(`Cannot execute the required TypeScript compiler ${compiler}: ${version.error.message}`);
  const compilerVersion = /^Version (\S+)/mu.exec(version.stdout ?? '')?.[1];
  if (version.status !== 0 || !compilerVersion) {
    throw new Error(`The required compiler did not report its TypeScript version: ${version.stdout ?? ''}${version.stderr ?? ''}`);
  }
  const registry = readAssertionVersionPolicy();
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-public-declarations-'));
  try {
    return publicDeclarationConsumers().map(consumer => {
      const {name, source} = consumer;
      const expectedDiagnostics = selectPublicDeclarationDiagnostics(consumer, compilerVersion, registry);
      const file = join(directory, `${name}.mts`);
      writeFileSync(file, source);
      const result = spawnSync(compiler, ['--noEmit', '--strict', '--pretty', 'false', '--target', 'ES2022',
        '--module', 'NodeNext', '--moduleResolution', 'NodeNext', file],
      { encoding: 'utf8', timeout: 20_000, maxBuffer: 256 * 1024 });
      if (result.error) throw new Error(`Cannot execute the required TypeScript compiler ${compiler}: ${result.error.message}`);
      const diagnostics = `${result.stdout ?? ''}${result.stderr ?? ''}`;
      const codes = [...diagnostics.matchAll(/error (TS\d+):/gu)].map((match) => match[1]);
      // TypeScript 5 exits 2 for diagnostics; the pinned TypeScript 7 exits 1.
      // Both must still produce exactly the declared rejection diagnostics.
      const expectedExit = expectedDiagnostics.length ? [1, 2].includes(result.status) : result.status === 0;
      if (!expectedExit
        || JSON.stringify(codes) !== JSON.stringify(expectedDiagnostics)) {
        throw new Error(`${name}: compiler exit ${result.status}; expected diagnostics ${expectedDiagnostics.join(', ') || 'none'}\n${diagnostics}`);
      }
      return { name, compilerVersion, diagnostics: codes };
    });
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const args = process.argv.slice(2);
    if (args.length !== 2 || args[0] !== '--compiler') throw new TypeError('Usage: node js/scripts/check-public-declaration-contracts.mjs --compiler PATH');
    const results = checkPublicDeclarationContracts(args[1]);
    console.log(`public declaration contracts: ${results.length} real consumer checks passed (TypeScript ${results[0].compilerVersion})`);
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
