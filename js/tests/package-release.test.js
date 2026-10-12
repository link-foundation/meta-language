import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { test } from 'node:test';

import { parseCargoManifest } from '../scripts/dependency-inventory.mjs';
import { grammarFile, readGrammarLicense } from '../scripts/grammar-files.mjs';
import { externalGrammarPath } from '../scripts/external-grammar-cache.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { releaseIdentity, releaseRegistryState, verifyPublishedRelease } from '../scripts/release-publication.mjs';

async function readJson(url) {
  return readProjectData(url);
}

test('npm package metadata uses the public unscoped package name', async () => {
  const packageJson = await readJson(new URL('../package.json', import.meta.url));
  const packageLock = await readJson(new URL('../package-lock.json', import.meta.url));
  const rootReadme = await readFile(new URL('../../README.md', import.meta.url), 'utf8');
  const jsReadme = await readFile(new URL('../README.md', import.meta.url), 'utf8');
  const rustReadme = await readFile(new URL('../../rust/README.md', import.meta.url), 'utf8');
  const issue163CaseStudy = await readFile(
    new URL('../../docs/case-studies/issue-163/README.md', import.meta.url),
    'utf8',
  );

  assert.equal(packageJson.name, 'meta-language');
  assert.equal(packageLock.name, 'meta-language');
  assert.equal(packageLock.packages[''].name, 'meta-language');
  assert.equal(
    packageJson.repository.url,
    'git+https://github.com/link-foundation/meta-language.git',
  );

  for (const readmeWithBadge of [rootReadme, jsReadme]) {
    assert.ok(readmeWithBadge.includes('npmjs.com/package/meta-language'));
  }

  for (const readmeWithImport of [rootReadme, jsReadme, rustReadme]) {
    assert.ok(readmeWithImport.includes("from 'meta-language'"));
  }

  for (const publicDoc of [rootReadme, jsReadme, rustReadme, issue163CaseStudy]) {
    assert.equal(publicDoc.includes('@link-foundation/meta-language'), false);
  }
});

test('npm delivery is script-free and carries every locked portable grammar', async () => {
  const packageJson = await readJson(new URL('../package.json', import.meta.url));
  const lock = await readJson(new URL('../src/vendor/grammars/grammar-lock.json', import.meta.url));

  assert.equal(packageJson.scripts.install, undefined);
  assert.equal(packageJson.scripts.prepack, undefined);
  assert.equal(packageJson.dependencies['tree-sitter-rocq'], undefined);
  assert.equal(packageJson.dependencies['@kreuzberg/tree-sitter-language-pack'], undefined);
  assert.equal(packageJson.dependencies['tree-sitter'], undefined);
  assert.equal(packageJson.dependencies['web-tree-sitter'], lock.treeSitterRuntime.javascript);
  assert.equal(packageJson.bundleDependencies, undefined);
  assert.ok(packageJson.files.includes('src'));
  for (const [id, grammar] of Object.entries(lock.grammars)) {
    const file = (name) => externalGrammarPath(grammarFile(grammar, name));
    const wasm = gunzipSync(await readFile(file(`${id}.wasm.gz`)));
    assert.equal(createHash('sha256').update(wasm).digest('hex'), grammar.wasmSha256, id);
    const license = readGrammarLicense(grammarFile(grammar, `${id}.LICENSE`)).toString('utf8');
    assert.ok(license.trim().length > 0, `${id} carries its license`);
  }
});

test('the tree-sitter oracles of the native languages are development files only', async () => {
  const packageJson = await readJson(new URL('../package.json', import.meta.url));
  const lock = await readJson(new URL('../src/vendor/grammars/grammar-lock.json', import.meta.url));
  const manifest = await readFile(new URL('../../rust/Cargo.toml', import.meta.url), 'utf8');
  const build = await readFile(new URL('../../rust/build.rs', import.meta.url), 'utf8');
  const { dependencies } = parseCargoManifest(manifest);
  const include = manifest.match(/^include = \[([^\]]*)\]/mu)[1];
  const oracles = Object.values(lock.grammars).filter((grammar) => grammar.oracle);
  assert.deepEqual(
    oracles.map(({ id }) => id),
    ['agda', 'c', 'cmake', 'cpp', 'css', 'csv', 'dart', 'diff', 'dtd', 'erlang', 'go', 'graphql', 'groovy', 'hcl', 'html', 'ini', 'java', 'javascript', 'json', 'json5', 'lean', 'lua', 'make', 'nix', 'odin', 'pascal', 'powershell', 'proto', 'python', 'r', 'racket', 'regex', 'rocq', 'rust', 'scheme', 'solidity', 'sql', 'toml', 'tsx', 'typescript', 'vb', 'xml', 'zig'],
  );
  assert.ok(packageJson.files.every((entry) => !entry.startsWith('oracles')));
  for (const grammar of oracles) {
    assert.ok(grammarFile(grammar, '').startsWith('js/oracles/'), grammar.id);
    await assert.rejects(
      readFile(new URL(`../src/vendor/grammars/${grammar.id}.wasm.gz`, import.meta.url)),
      grammar.id,
    );
    if (grammar.crate) {
      // The Rust suites still load the oracle crate, as a development dependency.
      const declared = dependencies.filter(({ name }) => name === grammar.crate);
      assert.deepEqual(declared.map(({ kind }) => kind), ['development'], grammar.crate);
    } else {
      // A vendored oracle parser is neither compiled by production nor published.
      assert.ok(!include.includes(grammar.vendored.replace(/^rust\//u, '')), grammar.vendored);
      assert.ok(!build.includes(`"${grammar.id}"`), grammar.id);
    }
  }
});

test('npm lockfile carries no native or network-downloaded grammar packages', async () => {
  const packageLock = await readJson(new URL('../package-lock.json', import.meta.url));

  for (const packagePath of Object.keys(packageLock.packages)) {
    assert.equal(packagePath.includes('tree-sitter-language-pack'), false, packagePath);
    assert.notEqual(packagePath, 'node_modules/tree-sitter', packagePath);
  }
});

test('Rust delivery closes each decompressed vendored parser before compiling it', async () => {
  const buildScript = await readFile(new URL('../../rust/oracles/native-source-oracles/build.rs', import.meta.url), 'utf8');

  assert.match(buildScript, /fn decompress_parser\([^]*?\n}/);
  assert.match(
    buildScript,
    /decompress_parser\(&compressed, &parser\);\s+let mut compiler = cc::Build::new\(\)/,
  );
});

test('JavaScript workflow publishes to npm with trusted publishing provenance', async () => {
  const workflow = await readFile(
    new URL('../../.github/workflows/js.yml', import.meta.url),
    'utf8',
  );

  assert.match(workflow, /release:\s*\n\s+types:\s+\[published\]/);
  assert.match(workflow, /id-token:\s+write/);
  assert.match(workflow, /registry-url:\s+['"]https:\/\/registry\.npmjs\.org['"]/);
  assert.match(workflow, /working-directory:\s+js/);
  assert.match(workflow, /npm publish --provenance/);
  assert.match(workflow, /node scripts\/release-publication\.mjs --preflight/);
  assert.match(workflow, /node scripts\/release-publication\.mjs --published/);
  assert.doesNotMatch(workflow, /npm view[^\n]*>\/dev\/null 2>&1/);
  // OIDC trusted publishing is the steady-state mechanism, so the only
  // NODE_AUTH_TOKEN in the workflow is the optional NPM_TOKEN bootstrap
  // fallback, and the placeholder credential setup-node writes must be
  // stripped before publishing (issue #191, actions/setup-node#1551).
  assert.match(workflow, /node scripts\/prepare-npm-auth\.mjs/);
  assert.equal(workflow.match(/^\s*NODE_AUTH_TOKEN:/gm).length, 1);
  assert.match(workflow, /NODE_AUTH_TOKEN:\s+\$\{\{\s*secrets\.NPM_TOKEN\s*\}\}/);
  assert.match(workflow, /permissions:\s*\n\s+contents:\s+read/);
  // The workflow-level group cancels superseded pull request runs only; every
  // other event gets its own group, so a release run is never cancelled. The
  // literal js- prefix keeps the group apart from ci.yml, whose github.workflow
  // a called workflow sees.
  const header = workflow.split('\njobs:\n')[0];
  assert.equal(header.match(/\nconcurrency:\n/g).length, 1);
  assert.ok(
    header.includes(
      "concurrency:\n  group: js-${{ github.event_name == 'pull_request' && github.ref || github.run_id }}\n  cancel-in-progress: true\n",
    ),
  );

  const publishJob = workflow.slice(workflow.indexOf('  publish:\n'));
  assert.match(publishJob, /group:\s+release-\$\{\{ github\.repository \}\}-main-write/);
  assert.match(publishJob, /cancel-in-progress:\s+false/);
  assert.doesNotMatch(publishJob, /queue:\s+/);
  assert.match(publishJob, /REQUESTED_VERSION:\s+\$\{\{ github\.event\.inputs\.release_version \}\}/);
  assert.doesNotMatch(publishJob, /run:[^\n]*\$\{\{\s*github\.event\.inputs\.release_version\s*\}\}/);
});

const releaseFixture = () => ({
  packageJson: { name: 'meta-language', version: '1.0.0' },
  packageLock: { name: 'meta-language', version: '1.0.0', packages: { '': { name: 'meta-language', version: '1.0.0' } } },
  cargoManifest: '[package]\nname = "meta-language"\nversion = "1.0.0"\n\n[dependencies]\nsomething = "9.0.0"\n',
  env: { GITHUB_EVENT_NAME: 'workflow_dispatch', GITHUB_REF: 'refs/tags/v1.0.0', GITHUB_SHA: 'a'.repeat(40), REQUESTED_VERSION: '1.0.0' },
  commit: 'a'.repeat(40),
});

test('publication refuses a branch, differing crate or lock version and a different workflow commit', () => {
  assert.deepEqual(releaseIdentity(releaseFixture()), {
    name: 'meta-language', version: '1.0.0', tag: 'v1.0.0', commit: 'a'.repeat(40),
  });
  for (const mutate of [
    (fixture) => { fixture.env.GITHUB_REF = 'refs/heads/main'; },
    (fixture) => { fixture.cargoManifest = fixture.cargoManifest.replace('version = "1.0.0"', 'version = "1.0.1"'); },
    (fixture) => { fixture.packageLock.version = '1.0.1'; },
    (fixture) => { fixture.packageLock.packages[''].version = '1.0.1'; },
    (fixture) => { fixture.commit = 'b'.repeat(40); },
    (fixture) => { fixture.env.REQUESTED_VERSION = '1.0.1'; },
    (fixture) => { fixture.env.GITHUB_EVENT_NAME = 'pull_request'; },
    (fixture) => { fixture.env.RELEASE_TAG = 'v1.0.1'; },
  ]) {
    const fixture = releaseFixture();
    mutate(fixture);
    assert.throws(() => releaseIdentity(fixture));
  }
  const release = releaseFixture();
  release.env.GITHUB_EVENT_NAME = 'release';
  assert.throws(() => releaseIdentity(release), /release event/u);
  release.env.RELEASE_TAG = 'v1.0.0';
  assert.equal(releaseIdentity(release).tag, 'v1.0.0');
});

const registryFixture = (npmStatus = 200) => async (url) => {
  const crate = String(url).startsWith('https://crates.io/');
  return new Response(JSON.stringify(crate
    ? { version: { num: '1.0.0', yanked: false, checksum: 'b'.repeat(64) } }
    : { name: 'meta-language', version: '1.0.0', dist: { integrity: 'sha512-fixture', tarball: 'https://registry.npmjs.org/meta-language/-/meta-language-1.0.0.tgz' } }),
  { status: crate ? 200 : npmStatus });
};

test('registry preflight treats only npm 404 as absent and refuses outages or invalid crate metadata', async () => {
  const identity = releaseIdentity(releaseFixture());
  assert.equal((await releaseRegistryState(identity, { fetchImpl: registryFixture(404) })).alreadyPublished, false);
  assert.equal((await releaseRegistryState(identity, { fetchImpl: registryFixture() })).alreadyPublished, true);
  for (const status of [401, 403, 429, 500, 503]) {
    await assert.rejects(releaseRegistryState(identity, { fetchImpl: registryFixture(status) }), new RegExp(`HTTP ${status}`, 'u'));
  }
  await assert.rejects(releaseRegistryState(identity, { fetchImpl: async () => { throw new Error('network unavailable'); } }), /network unavailable/u);
  for (const version of [
    { num: '0.46.0', yanked: false, checksum: 'b'.repeat(64) },
    { num: '1.0.0', yanked: true, checksum: 'b'.repeat(64) },
    { num: '1.0.0', yanked: false },
  ]) {
    await assert.rejects(releaseRegistryState(identity, { fetchImpl: async () => new Response(JSON.stringify({ version })) }), /non-yanked/u);
  }
  await assert.rejects(releaseRegistryState(identity, { fetchImpl: async () => new Response('not JSON') }), /invalid registry metadata/u);
  for (const value of ['null', '[]', '"version absent"']) {
    await assert.rejects(releaseRegistryState(identity, { fetchImpl: async () => new Response(value) }), /invalid registry metadata/u);
  }
});

test('post-publication verification waits for registry propagation and fails if npm stays absent', async () => {
  const identity = releaseIdentity(releaseFixture());
  let lookups = 0;
  let delays = 0;
  const fetchImpl = async (url) => registryFixture(String(url).startsWith('https://registry.npmjs.org/') && ++lookups === 1 ? 404 : 200)(url);
  assert.equal((await verifyPublishedRelease(identity, { fetchImpl, delay: async () => { delays += 1; } })).alreadyPublished, true);
  assert.equal(delays, 1);
  assert.equal(lookups, 2);
  await assert.rejects(verifyPublishedRelease(identity, {
    fetchImpl: registryFixture(404), attempts: 2, delay: async () => {},
  }), /after 2 registry checks/u);
});

test('Rust release pipeline delegates npm publishing to the canonical JavaScript workflow', async () => {
  const rustWorkflow = await readFile(
    new URL('../../.github/workflows/rust.yml', import.meta.url),
    'utf8',
  );
  const releaseScript = await readFile(
    new URL('../../rust/scripts/version-and-commit.rs', import.meta.url),
    'utf8',
  );
  const releaseCheck = await readFile(
    new URL('../../rust/scripts/check-release-needed.rs', import.meta.url),
    'utf8',
  );

  assert.match(releaseScript, /npm/);
  assert.match(releaseScript, /version/);
  assert.match(releaseScript, /package-lock\.json/);
  assert.doesNotMatch(releaseCheck, /npm_published|npm_required|registry\.npmjs\.org/);
  assert.doesNotMatch(rustWorkflow, /npm publish|npm view|NODE_AUTH_TOKEN/);
  assert.doesNotMatch(rustWorkflow, /Publish JavaScript package to npm/);

  for (const jobName of ['auto-release', 'manual-release']) {
    const start = rustWorkflow.indexOf(`  ${jobName}:\n`);
    assert.notEqual(start, -1);
    const remainder = rustWorkflow.slice(start);
    const nextJob = remainder.slice(1).search(/\n  [a-z][a-z0-9-]*:\n/);
    const job = nextJob === -1 ? remainder : remainder.slice(0, nextJob + 1);
    const createRelease = job.indexOf('- name: Create GitHub Release');
    const dispatchPublisher = job.indexOf('- name: Dispatch JavaScript publisher');

    assert.match(job, /actions:\s+write/);
    assert.match(job, /gh workflow run js\.yml --ref "v\$RELEASE_VERSION"/);
    assert.match(job, /release_version="\$RELEASE_VERSION"/);
    assert.ok(createRelease >= 0 && dispatchPublisher > createRelease);
  }
});

test('issue 195 acceptance workflow produces exact-checkpoint evidence with pinned toolchains', async () => {
  const workflow = await readFile(
    new URL('../../.github/workflows/ci.yml', import.meta.url),
    'utf8',
  );

  assert.match(workflow, /node-version:\s*24/);
  assert.match(workflow, /dtolnay\/rust-toolchain@1\.99\.0/);
  assert.match(workflow, /ocaml\/setup-ocaml@v3/);
  assert.match(
    workflow,
    /opam repository add rocq-released https:\/\/rocq-prover\.org\/opam\/released/,
  );
  assert.match(workflow, /opam install[^\n]*rocq-core=9\.3\.0[^\n]*rocq-stdlib=9\.2\.0/);
  assert.match(workflow, /opam var bin >> "\$GITHUB_PATH"/);
  assert.doesNotMatch(workflow, /leanprover\/lean-action/);
  assert.match(workflow, /elan-x86_64-unknown-linux-gnu\.tar\.gz/);
  assert.match(workflow, /42b94d4244e8353142c456ec0e4ca6528fd898a6c604d4059f494e706e431f63/);
  assert.match(workflow, /leanprover\/lean4:v4\.34\.1/);
  assert.match(workflow, /node js\/scripts\/run-issue-195-evidence\.mjs/);
  assert.match(workflow, /--checkpoint "\$ACCEPTANCE_CHECKPOINT"/);
  assert.match(workflow, /--commit "\$GITHUB_SHA"/);
  assert.match(workflow, /npm ci --ignore-scripts/);
});
