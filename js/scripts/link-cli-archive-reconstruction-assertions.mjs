import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { ASSERTION_VERSION_POLICY_FILE, readAssertionVersionPolicy } from './antlr-version-assertions.mjs';
import { selectVersionedAssertion } from './assertion-version-ranges.mjs';

const repository = fileURLToPath(new URL('../../', import.meta.url));
const digest = bytes => createHash('sha256').update(bytes).digest('hex');

export const linkCliReconstruction = Object.freeze({
  requirementId: 'I195-DYNAMIC-LINK-CLI-RECONSTRUCTION',
  suffix: 'public-archive-store-replay',
  fixtureFile: 'parity/fixtures/dynamic-language-command.lino',
  assertions: Object.freeze(['nativeLinkCliPersistsCompleteArchiveAcrossProcesses']),
  policyId: 'link-cli-native-store-replay',
  linkCliVersion: '3.1.1',
  notationVersion: '0.31.8',
});

export function linkCliArchiveReconstructionFixture(runtime) {
  return `dynamic-language-command-link-cli-${runtime}`;
}

export function selectLinkCliArchiveReconstructionAssertion({ root = repository,
  registry = readAssertionVersionPolicy(root), linkCliVersion = linkCliReconstruction.linkCliVersion,
} = {}) {
  const policy = registry.policy.assertions.find(item => item.id === linkCliReconstruction.policyId);
  if (!policy) throw new Error('the Link CLI replay assertion requires a registered version policy');
  const selected = selectVersionedAssertion(policy, linkCliVersion);
  const dependencyManifestPolicy = 'js/package.json';
  const dependencyLockPolicy = 'js/package-lock.json';
  const manifestBytes = readFileSync(path.join(root, dependencyManifestPolicy));
  const lockBytes = readFileSync(path.join(root, dependencyLockPolicy));
  const require = createRequire(path.join(root, dependencyManifestPolicy));
  const installedFile = require.resolve('link-cli-links-notation/package.json');
  const sourcePolicy = path.relative(root, installedFile).split(path.sep).join('/');
  const sourceBytes = readFileSync(installedFile);
  const installed = JSON.parse(sourceBytes);
  const manifest = JSON.parse(manifestBytes);
  const lock = JSON.parse(lockBytes).packages['node_modules/link-cli-links-notation'];
  const provider = 'links-notation';
  const version = selected.expected.linksNotation;
  if (installed.name !== provider || installed.version !== version
    || manifest.dependencies?.['link-cli-links-notation'] !== `npm:${provider}@${version}`
    || lock?.name !== provider || lock.version !== version) {
    throw new Error(`uncovered Links Notation store provider release ${installed.name} ${installed.version}`);
  }
  return Object.freeze({
    assertionPolicy: policy.id, expectation: selected.id, subject: policy.subject,
    activeVersion: linkCliVersion, sourceVersion: selected.expected.linkCli,
    minimum: selected.minimum, maximum: selected.maximum,
    includeMinimum: selected.includeMinimum, includeMaximum: selected.includeMaximum,
    applicabilityPolicy: ASSERTION_VERSION_POLICY_FILE, applicabilityPolicySha256: registry.sha256,
    sourcePolicy, sourcePolicySha256: digest(sourceBytes),
    dependencyManifestPolicy, dependencyManifestPolicySha256: digest(manifestBytes),
    dependencyLockPolicy, dependencyLockPolicySha256: digest(lockBytes),
  });
}
