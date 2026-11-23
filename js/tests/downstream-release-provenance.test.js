import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { verifyReleaseProvenance } from '../scripts/release-publication.mjs';

const release = {
  name: 'meta-language',
  version: '1.0.0',
  commit: 'a'.repeat(40),
  npmTarball: 'https://registry.npmjs.org/meta-language/-/meta-language-1.0.0.tgz',
  npmGitHead: 'a'.repeat(40),
};

function crateArchive() {
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-release-test-'));
  const member = 'meta-language-1.0.0/.cargo_vcs_info.json';
  try {
    mkdirSync(path.join(directory, 'meta-language-1.0.0'));
    writeFileSync(path.join(directory, member), JSON.stringify({ git: { sha1: release.commit } }));
    return execFileSync('tar', ['-czf', '-', '-C', directory, member]);
  } finally {
    rmSync(directory, { force: true, recursive: true });
  }
}

function registryDownload(archive) {
  return {
    crateChecksum: createHash('sha256').update(archive).digest('hex'),
    fetchImpl: async (url) => {
      assert.equal(url, 'https://static.crates.io/crates/meta-language/meta-language-1.0.0.crate');
      return new Response(archive);
    },
  };
}

test('release preflight verifies the exact crate checksum and commit before npm exists', async () => {
  const { crateChecksum, fetchImpl } = registryDownload(crateArchive());
  const state = { ...release, alreadyPublished: false, crateChecksum, npmTarball: null, npmGitHead: null };
  await verifyReleaseProvenance(state, { fetchImpl });
  await assert.rejects(verifyReleaseProvenance({ ...state, crateChecksum: 'b'.repeat(64) }, { fetchImpl }),
    /crate does not match its registry checksum/u);
  await assert.rejects(verifyReleaseProvenance({ ...state, commit: 'c'.repeat(40) }, { fetchImpl }),
    new RegExp(`crate records commit ${release.commit}, expected ${'c'.repeat(40)}`, 'u'));

  const invalid = registryDownload(Buffer.from('not a tar archive'));
  await assert.rejects(verifyReleaseProvenance({ ...state, crateChecksum: invalid.crateChecksum }, invalid),
    /crate has no readable Git provenance/u);
});

test('a published npm package must identify the tagged commit and official registry artifact', async () => {
  const { crateChecksum, fetchImpl } = registryDownload(crateArchive());
  const state = { ...release, alreadyPublished: true, crateChecksum };
  await verifyReleaseProvenance(state, { fetchImpl });
  for (const npmGitHead of ['c'.repeat(40), null]) {
    await assert.rejects(verifyReleaseProvenance({ ...state, npmGitHead }, { fetchImpl }),
      /published npm meta-language@1\.0\.0 records commit/u);
  }
  await assert.rejects(verifyReleaseProvenance({ ...state, npmTarball: 'https://example.com/package.tgz' }, { fetchImpl }),
    /npm returned an unexpected registry tarball/u);
});
