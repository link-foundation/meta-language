import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { verifyPublishedNativeHtml, verifyReleaseProvenance } from '../scripts/release-publication.mjs';

const release = {
  name: 'meta-language',
  version: '1.0.0',
  commit: 'a'.repeat(40),
  npmTarball: 'https://registry.npmjs.org/meta-language/-/meta-language-1.0.0.tgz',
  npmGitHead: 'a'.repeat(40),
};

function crateArchive(git = { sha1: release.commit }) {
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-release-test-'));
  const member = 'meta-language-1.0.0/.cargo_vcs_info.json';
  try {
    mkdirSync(path.join(directory, 'meta-language-1.0.0'));
    writeFileSync(path.join(directory, member), JSON.stringify({ git }));
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
    new RegExp(`crate records commit ${release.commit}, expected ${'c'.repeat(40)}\\. Use a new version for both packages to release this commit\\.`, 'u'));

  const dirty = registryDownload(crateArchive({ sha1: release.commit, dirty: true }));
  await assert.rejects(verifyReleaseProvenance({ ...state, crateChecksum: dirty.crateChecksum }, dirty),
    /crate does not record a clean Git checkout/u);

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

test('the published HTML smoke installs an exact registry version and verifies the lockfile', async () => {
  const { name, version, npmTarball } = release;
  const npmIntegrity = 'sha512-registry-integrity';
  const state = { ...release, npmIntegrity };
  const installed = { version, resolved: npmTarball, integrity: npmIntegrity };
  const mismatches = [{ version: '1.0.1' }, { resolved: 'https://example.com/package.tgz' }, { integrity: 'sha512-different-integrity' }];

  for (const mismatch of [null, ...mismatches]) {
    let directory;
    let smokeRuns = 0;
    const verification = verifyPublishedNativeHtml(state, {
      run(command, args, options) {
        if (command === 'npm') {
          directory = options.cwd;
          assert.deepEqual(args, [
            'install', '--ignore-scripts', '--no-audit', '--no-fund', '--cache', path.join(directory, 'npm-cache'),
            '--registry', 'https://registry.npmjs.org', `${name}@${version}`,
          ]);
          assert.deepEqual(JSON.parse(readFileSync(path.join(directory, 'package.json'), 'utf8')), { private: true });
          writeFileSync(path.join(directory, 'package-lock.json'), JSON.stringify({
            packages: { [`node_modules/${name}`]: { ...installed, ...mismatch } },
          }));
          return;
        }
        smokeRuns++;
        assert.equal(command, process.execPath);
        assert.equal(options.cwd, directory);
        assert.deepEqual(options.env, {});
        assert.equal(args[0], '--import');
        assert.equal(path.basename(args[1]), 'deny-consumer-network.mjs');
        assert.deepEqual(args.slice(2), ['--test', path.join(directory, 'native-html.test.mjs')]);
        assert.equal(existsSync(args[3]), true);
      },
    });
    if (mismatch) {
      await assert.rejects(verification, /did not install the registry artifact/u);
    } else {
      await verification;
    }
    assert.equal(smokeRuns, mismatch ? 0 : 1);
    assert.equal(existsSync(directory), false);
  }
});
