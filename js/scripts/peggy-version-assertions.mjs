// Only assertions about the independent optimizer's actual AST shape are bound.
// Native grammar conversion remains independent of the installed Peggy release.
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { ASSERTION_VERSION_POLICY_FILE, readAssertionVersionPolicy } from './antlr-version-assertions.mjs';
import { selectVersionedAssertion } from './assertion-version-ranges.mjs';

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const hash = bytes => createHash('sha256').update(bytes).digest('hex');

export function selectPeggyOptimizerAssertion({ root = repository, registry = readAssertionVersionPolicy(root), runtimeVersion } = {}) {
  const matches = registry.policy.assertions.filter(policy => policy.id === 'peggy-optimized-character-set-shape');
  if (matches.length !== 1) throw new Error('Peggy optimizer shape requires exactly one registered assertion policy');
  const dependencyManifestPolicy = 'js/package.json';
  const dependencyLockPolicy = 'js/package-lock.json';
  const resolve = createRequire(path.join(root, dependencyManifestPolicy));
  const installedFile = resolve.resolve('peggy/package.json');
  const sourcePolicy = path.relative(root, installedFile).split(path.sep).join('/');
  if (sourcePolicy.startsWith('../') || path.isAbsolute(sourcePolicy)) throw new Error('the active Peggy manifest must belong to this checkout');
  const sourceBytes = readFileSync(installedFile);
  const installed = JSON.parse(sourceBytes.toString('utf8'));
  if (installed.name !== 'peggy') throw new Error('the installed optimizer manifest must identify Peggy');
  const selected = selectVersionedAssertion(matches[0], installed.version);
  if (selected.expected !== `Peggy ${installed.version}`) throw new Error('the installed Peggy version differs from the selected expectation');
  if (runtimeVersion !== undefined && runtimeVersion !== installed.version) throw new Error('the executing Peggy version differs from its installed manifest');
  const manifestBytes = readFileSync(path.join(root, dependencyManifestPolicy));
  const lockBytes = readFileSync(path.join(root, dependencyLockPolicy));
  const manifest = JSON.parse(manifestBytes.toString('utf8'));
  const lock = JSON.parse(lockBytes.toString('utf8'));
  if (manifest.devDependencies?.peggy !== installed.version || lock.packages?.['node_modules/peggy']?.version !== installed.version) {
    throw new Error('the actual Peggy manifest, direct dependency pin and lock version must agree');
  }
  return Object.freeze({
    assertionPolicy: matches[0].id, expectation: selected.id, subject: matches[0].subject,
    activeVersion: installed.version, sourceVersion: selected.expected,
    minimum: selected.minimum, maximum: selected.maximum,
    includeMinimum: selected.includeMinimum, includeMaximum: selected.includeMaximum,
    applicabilityPolicy: ASSERTION_VERSION_POLICY_FILE, applicabilityPolicySha256: registry.sha256,
    sourcePolicy, sourcePolicySha256: hash(sourceBytes),
    dependencyManifestPolicy, dependencyManifestPolicySha256: hash(manifestBytes),
    dependencyLockPolicy, dependencyLockPolicySha256: hash(lockBytes),
  });
}
