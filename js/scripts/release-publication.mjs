#!/usr/bin/env node
// Publication checks run only in js.yml's standalone release/tag dispatch.
// A registry outage is a failure, never evidence that a version is unpublished.
import { execFileSync } from 'node:child_process';
import { appendFileSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

export function releaseIdentity({ packageJson, packageLock, cargoManifest, env, commit }) {
  const section = /^\[package\]\s*\n([^]*?)(?=^\[|$(?![^]))/mu.exec(cargoManifest)?.[1] ?? '';
  const cargoName = /^name\s*=\s*"([^"]+)"/mu.exec(section)?.[1];
  const cargoVersion = /^version\s*=\s*"([^"]+)"/mu.exec(section)?.[1];
  const { name, version } = packageJson;
  if (name !== 'meta-language' || cargoName !== name || packageLock.name !== name || packageLock.packages?.['']?.name !== name) {
    throw new Error('the npm package, lockfile and crate must all name meta-language');
  }
  if (!/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/u.test(version) ||
      [cargoVersion, packageLock.version, packageLock.packages?.['']?.version].some((value) => value !== version)) {
    throw new Error('the npm package, both lockfile versions and crate must have the same release version');
  }
  const tag = `v${version}`;
  if (!['release', 'workflow_dispatch'].includes(env.GITHUB_EVENT_NAME) || env.GITHUB_REF !== `refs/tags/${tag}`) {
    throw new Error(`publication must run from the matching release tag refs/tags/${tag}, not ${env.GITHUB_REF ?? 'an unspecified ref'}`);
  }
  if ((env.GITHUB_EVENT_NAME === 'release' && env.RELEASE_TAG !== tag) ||
      (env.RELEASE_TAG && env.RELEASE_TAG !== tag) ||
      (env.REQUESTED_VERSION && env.REQUESTED_VERSION !== version)) {
    throw new Error(`the release event and requested version must match ${tag}`);
  }
  if (!/^[a-f0-9]{40}$/u.test(commit) || commit !== env.GITHUB_SHA) {
    throw new Error('the checked out commit must match the workflow release commit');
  }
  return { name, version, tag, commit };
}

async function registryJson(url, { fetchImpl, allowAbsent = false }) {
  const response = await fetchImpl(url, { headers: { Accept: 'application/json' }, signal: AbortSignal.timeout(15_000) });
  if (allowAbsent && response.status === 404) return null;
  if (!response.ok) throw new Error(`${url} lookup failed (HTTP ${response.status}); publication was not attempted`);
  try {
    const data = await response.json();
    if (!data || typeof data !== 'object' || Array.isArray(data)) throw new Error('invalid metadata object');
    return data;
  } catch {
    throw new Error(`${url} returned invalid registry metadata`);
  }
}

export async function releaseRegistryState(identity, { fetchImpl = fetch } = {}) {
  const { name, version } = identity;
  // The crate must already exist: rust.yml publishes and waits for it before
  // dispatching the npm publisher on the same release tag.
  const crate = await registryJson(`https://crates.io/api/v1/crates/${name}/${version}`, { fetchImpl });
  if (crate.version?.num !== version || crate.version?.yanked !== false || !/^[a-f0-9]{64}$/u.test(crate.version?.checksum ?? '')) {
    throw new Error(`crates.io must serve the non-yanked ${name}@${version} with its checksum before npm publication`);
  }
  const npm = await registryJson(`https://registry.npmjs.org/${name}/${version}`, { fetchImpl, allowAbsent: true });
  if (npm && (npm.name !== name || npm.version !== version || !npm.dist?.integrity || !npm.dist?.tarball)) {
    throw new Error(`npm returned inconsistent artifact metadata for ${name}@${version}`);
  }
  return { ...identity, crateChecksum: crate.version.checksum, alreadyPublished: npm !== null, npmIntegrity: npm?.dist.integrity ?? null };
}

export async function verifyPublishedRelease(identity, {
  fetchImpl = fetch, attempts = 12, delay = () => new Promise((resolve) => setTimeout(resolve, 5_000)),
} = {}) {
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const state = await releaseRegistryState(identity, { fetchImpl });
    if (state.alreadyPublished) return state;
    if (attempt + 1 < attempts) await delay();
  }
  throw new Error(`npm does not serve ${identity.name}@${identity.version} after ${attempts} registry checks`);
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const identity = releaseIdentity({
      packageJson: JSON.parse(readFileSync(path.join(root, 'js/package.json'), 'utf8')),
      packageLock: JSON.parse(readFileSync(path.join(root, 'js/package-lock.json'), 'utf8')),
      cargoManifest: readFileSync(path.join(root, 'rust/Cargo.toml'), 'utf8'),
      env: process.env,
      commit: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim(),
    });
    const mode = process.argv[2];
    if (!['--identity', '--preflight', '--published'].includes(mode)) throw new Error('usage: release-publication.mjs --identity | --preflight | --published');
    const state = mode === '--identity' ? identity : mode === '--preflight'
      ? await releaseRegistryState(identity) : await verifyPublishedRelease(identity);
    if (process.env.GITHUB_OUTPUT) {
      appendFileSync(process.env.GITHUB_OUTPUT, `name=${identity.name}\nversion=${identity.version}\n`);
      if (state.alreadyPublished !== undefined) appendFileSync(process.env.GITHUB_OUTPUT, `already_published=${state.alreadyPublished}\n`);
    }
    console.log(JSON.stringify(state, null, 2));
  } catch (error) {
    console.error(`::error::${error.message}`);
    process.exitCode = 1;
  }
}
