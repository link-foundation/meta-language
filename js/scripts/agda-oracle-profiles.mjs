// Test-oracle profiles retain historical data and require current exact trees.
// No parser, native compiler or WebAssembly module is initialized here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { join } from 'node:path';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';
import { readAssertionVersionPolicy } from './antlr-version-assertions.mjs';
import { selectVersionedAssertion } from './assertion-version-ranges.mjs';

const repository = fileURLToPath(new URL('../../', import.meta.url));
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
export const AGDA_HISTORICAL_FIXTURE_FILE = 'parity/fixtures/native-grammars/agda.lino';
export const AGDA_CURRENT_ORACLE_PROFILE_FILE = 'parity/fixtures/native-grammars/agda-current-oracle-profile.lino';
export const AGDA_HISTORICAL_FIXTURE = Object.freeze({
  file: AGDA_HISTORICAL_FIXTURE_FILE, bytes: 216411,
  sha256: 'b3ff7b43cdf4ba617ac2b19f47cb927c404eab48b38e0c6152a82e5379a1b18f',
});

/** Select a singleton from actual locked versions and authenticated source bytes. */
export function selectAgdaOracleAssertion({ root = repository, registry = readAssertionVersionPolicy(root) } = {}) {
  const policies = registry.policy.assertions.filter(policy => policy.id === 'agda-corrected-layout-oracle');
  assert.equal(policies.length, 1, 'one registered Agda oracle assertion policy is required');
  const manifestBytes = readFileSync(join(root, 'rust/vendor/tree-sitter-agda-layout/source-manifest.lino'));
  const manifest = readStructuredDataLino(manifestBytes.toString('utf8'));
  const active = /^tree-sitter-agda (\d+\.\d+\.\d+)$/u.exec(manifest.crate);
  assert.ok(active, 'the actual oracle source identifies its crate version');
  const selected = selectVersionedAssertion(policies[0], active[1]);
  const patchBytes = readFileSync(join(root, 'rust/vendor/tree-sitter-agda-layout/layout-state.patch'));
  const actual = { profile: 'complete-layout-state', crate: manifest.crate, revision: manifest.revision,
    treeSitter: manifest.treeSitterCli, parserSha256: manifest.parserSha256,
    originalScannerSha256: manifest.originalScannerSha256, scannerSha256: manifest.scannerSha256,
    patchSha256: digest(patchBytes), sourceManifestSha256: digest(manifestBytes) };
  assert.deepEqual(actual, selected.expected, 'the active complete scanner profile matches its registered exact source pins');
  const lock = readFileSync(join(root, 'rust/Cargo.lock'), 'utf8');
  for (const [name, version] of [['tree-sitter-agda', active[1]], ['tree-sitter', actual.treeSitter]]) {
    const records = lock.split('[[package]]').filter(record => record.includes(`\nname = "${name}"\n`));
    assert.equal(records.length, 1, 'the actual Cargo oracle/runtime lock identity is unique');
    assert.ok(records[0].includes(`\nversion = "${version}"\n`), 'the actual Cargo version matches the selected profile');
  }
  const runtime = JSON.parse(readFileSync(join(root, 'js/node_modules/web-tree-sitter/package.json'), 'utf8'));
  assert.equal(runtime.name, 'web-tree-sitter');
  assert.equal(runtime.version, actual.treeSitter, 'the executing JavaScript oracle runtime matches the pinned Tree-sitter version');
  return { assertionPolicy: policies[0].id, expectation: selected.id, activeVersion: active[1],
    minimum: selected.minimum, maximum: selected.maximum,
    includeMinimum: selected.includeMinimum, includeMaximum: selected.includeMaximum,
    applicabilityPolicySha256: registry.sha256, source: actual };
}

/** Historical recovery assertions concern this exact sandboxed released WASM. */
export function selectAgdaReleasedOracleAssertion({ root = repository, registry = readAssertionVersionPolicy(root) } = {}) {
  const policies = registry.policy.assertions.filter(policy => policy.id === 'agda-released-wasm-oracle');
  assert.equal(policies.length, 1, 'one historical Agda WASM assertion policy is required');
  const lock = readStructuredDataLino(readFileSync(join(root, 'js/src/vendor/grammars/grammar-lock.lino'), 'utf8'));
  const grammar = lock.grammars.agda;
  const selected = selectVersionedAssertion(policies[0], grammar.version);
  assert.deepEqual({ profile: 'released-wasm', crate: grammar.crate, version: grammar.version,
    treeSitter: lock.treeSitterRuntime.javascript, parserSha256: grammar.parserSha256,
    wasmSha256: grammar.wasmSha256 }, selected.expected, 'the historical oracle remains its exact released artifact');
  return selected;
}

/** This is an authored current expectation, never an observation of execution. */
export function buildAgdaCurrentOracleFixture(originalBytes, profile) {
  assert.equal(originalBytes.length, AGDA_HISTORICAL_FIXTURE.bytes, 'the complete historical fixture byte count is retained');
  assert.equal(digest(originalBytes), AGDA_HISTORICAL_FIXTURE.sha256, 'the complete historical fixture remains immutable');
  const original = readStructuredDataLino(originalBytes.toString('utf8'));
  assert.equal(original.divergences.length, 4, 'every original authored layout source is retained');
  const current = structuredClone(original);
  current.matches.push(...current.divergences.map(({ source, rows }) => ({ source, rows })));
  current.divergences = [];
  current.oracleProfile = { ...structuredClone(profile), historicalFixture: { ...AGDA_HISTORICAL_FIXTURE },
    expectationBasis: 'Authored current obligations: the complete corrected scanner must accept every retained valid source cleanly and match its entire accepted native row golden. Native and WebAssembly execution are CI checks, not prior observations.',
    historicalDivergences: structuredClone(original.divergences) };
  const inverse = structuredClone(current);
  inverse.matches.splice(original.matches.length);
  inverse.divergences = inverse.oracleProfile.historicalDivergences;
  delete inverse.oracleProfile;
  assert.deepEqual(inverse, original, 'the profile projection restores every historical field and row');
  assert.deepEqual(Buffer.from(structuredDataToLino(inverse)), originalBytes, 'the exact historical bytes are reconstructed');
  return current;
}

export function readAgdaCurrentOracleFixture(root = repository) {
  return buildAgdaCurrentOracleFixture(readFileSync(join(root, AGDA_HISTORICAL_FIXTURE_FILE)),
    selectAgdaOracleAssertion({ root }));
}
