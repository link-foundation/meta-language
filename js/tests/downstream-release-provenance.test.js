import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import { verifyTaggedRegistryRelease } from '../scripts/release-publication.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const identity = { name: 'meta-language', version: '1.0.0', tag: 'v1.0.0', commit: 'a'.repeat(40) };
const cargoProvenance = { git: { sha1: identity.commit }, path_in_vcs: 'rust' };
const crateArchiveName = 'meta-language-1.0.0';

function crateArchive(provenance = JSON.stringify(cargoProvenance)) {
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-release-'));
  try {
    mkdirSync(join(directory, crateArchiveName));
    if (provenance !== null) writeFileSync(join(directory, crateArchiveName, '.cargo_vcs_info.json'), provenance);
    return execFileSync('tar', ['-czf', '-', '-C', directory, crateArchiveName]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

let matchingArchive;
const registryFixture = (npmStatus = 200, {
  archive = (matchingArchive ??= crateArchive()),
  checksum = createHash('sha256').update(archive).digest('hex'),
  archiveStatus = 200,
  npmOverrides = {},
} = {}) => async (url) => {
  switch (url) {
    case 'https://crates.io/api/v1/crates/meta-language/1.0.0':
      return Response.json({ version: { num: '1.0.0', yanked: false, checksum } });
    case `https://static.crates.io/crates/meta-language/${crateArchiveName}.crate`:
      return new Response(archive, { status: archiveStatus });
    case 'https://registry.npmjs.org/meta-language/1.0.0':
      return Response.json({
        name: 'meta-language', version: '1.0.0', gitHead: cargoProvenance.git.sha1,
        dist: { integrity: 'sha512-fixture', tarball: 'https://registry.npmjs.org/meta-language/-/meta-language-1.0.0.tgz' },
        ...npmOverrides,
      }, { status: npmStatus });
    default:
      throw new Error(`unexpected registry request ${url}`);
  }
};

test('tagged publication verification waits for registry propagation and fails if npm stays absent', async () => {
  let lookups = 0;
  let delays = 0;
  let downloads = 0;
  const fetchImpl = async (url) => {
    if (url.endsWith('.crate')) downloads += 1;
    return registryFixture(url.startsWith('https://registry.npmjs.org/') && ++lookups === 1 ? 404 : 200)(url);
  };
  const published = await verifyTaggedRegistryRelease(identity, {
    fetchImpl, waitForPublication: true, delay: async () => { delays += 1; },
  });
  assert.equal(published.alreadyPublished, true);
  assert.equal(delays, 1);
  assert.equal(lookups, 2);
  assert.equal(downloads, 1);
  await assert.rejects(verifyTaggedRegistryRelease(identity, {
    waitForPublication: true, fetchImpl: registryFixture(404), attempts: 2, delay: async () => {},
  }), /after 2 registry checks/u);
});

test('publication verifies the crate archive checksum and binds both registries to the release commit', async (context) => {
  const archive = crateArchive(JSON.stringify({ ...cargoProvenance, git: { ...cargoProvenance.git, dirty: false } }));
  const checksum = createHash('sha256').update(archive).digest('hex');
  assert.deepEqual(await verifyTaggedRegistryRelease(identity, { fetchImpl: registryFixture(200, { archive }) }), {
    ...identity, crateChecksum: checksum, alreadyPublished: true, npmIntegrity: 'sha512-fixture',
  });
  assert.deepEqual(await verifyTaggedRegistryRelease(identity, { fetchImpl: registryFixture(404, { archive }) }), {
    ...identity, crateChecksum: checksum, alreadyPublished: false, npmIntegrity: null,
  });

  for (const [options, diagnostic] of [
    [{ archiveStatus: 404 }, /download failed \(HTTP 404\)/u],
    [{ checksum: 'b'.repeat(64) }, /archive checksum mismatch/u],
    [{ archive: crateArchive(null) }, /lacks readable Cargo Git provenance/u],
    [{ archive: crateArchive('invalid JSON') }, /lacks readable Cargo Git provenance/u],
    [{ archive: crateArchive(JSON.stringify({ ...cargoProvenance, git: { sha1: 'b'.repeat(40) } })) }, /release commit/u],
    [{ archive: crateArchive(JSON.stringify({ ...cargoProvenance, git: { ...cargoProvenance.git, dirty: true } })) }, /release commit/u],
    [{ archive: crateArchive(JSON.stringify({ ...cargoProvenance, path_in_vcs: 'elsewhere' })) }, /release commit/u],
  ]) {
    const fixture = registryFixture(404, options);
    await assert.rejects(verifyTaggedRegistryRelease(identity, { fetchImpl: async (url) => {
      assert.ok(!url.startsWith('https://registry.npmjs.org/'), 'crate rejection must precede npm lookup');
      return fixture(url);
    } }), diagnostic);
  }

  for (const gitHead of [undefined, 'b'.repeat(40)]) {
    const fetchImpl = registryFixture(200, { npmOverrides: { gitHead } });
    await assert.rejects(verifyTaggedRegistryRelease(identity, { fetchImpl }), /npm must report release commit/u);
    await assert.rejects(verifyTaggedRegistryRelease(identity, { fetchImpl, waitForPublication: true }), /npm must report release commit/u);
  }

  recordIssue195Observations({
    requirementId: 'I195-I230-PAIRED-RELEASE-PROVENANCE', suffix: 'behavior',
    fixtureId: 'i195-i230-paired-release-provenance', fixtureFile: 'js/tests/downstream-release-provenance.test.js',
    assertions: ['crateArchiveMatchesRegistryChecksum', 'crateArchiveIdentifiesCleanReleaseCommit', 'npmMetadataIdentifiesReleaseCommit'],
    testName: context.name,
  });
});
