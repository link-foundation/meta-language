import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

async function readJson(url) {
  return JSON.parse(await readFile(url, 'utf8'));
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

test('npm delivery is script-free and carries the portable Rocq grammar', async () => {
  const packageJson = await readJson(new URL('../package.json', import.meta.url));
  const rocqGrammar = await readFile(
    new URL('../src/vendor/tree-sitter-rocq.wasm', import.meta.url),
  );

  assert.equal(packageJson.scripts.install, undefined);
  assert.equal(packageJson.scripts.prepack, undefined);
  assert.equal(packageJson.dependencies['tree-sitter-rocq'], undefined);
  assert.equal(packageJson.dependencies['web-tree-sitter'], '0.25.10');
  assert.equal(packageJson.bundleDependencies, undefined);
  assert.ok(rocqGrammar.byteLength > 0);
});

test('npm lockfile records every optional native language-pack package', async () => {
  const packageLock = await readJson(new URL('../package-lock.json', import.meta.url));
  const languagePack =
    packageLock.packages['node_modules/@kreuzberg/tree-sitter-language-pack'];

  for (const packageName of Object.keys(languagePack.optionalDependencies)) {
    const topLevelPath = `node_modules/${packageName}`;
    const nestedPath =
      `node_modules/@kreuzberg/tree-sitter-language-pack/node_modules/${packageName}`;
    assert.ok(
      packageLock.packages[topLevelPath] || packageLock.packages[nestedPath],
      `package-lock.json is missing ${packageName}`,
    );
  }
});

test('Rust delivery closes the decompressed Rocq parser before compiling it', async () => {
  const buildScript = await readFile(new URL('../../rust/build.rs', import.meta.url), 'utf8');

  assert.match(buildScript, /fn decompress_rocq_parser\([^]*?\n}/);
  assert.match(
    buildScript,
    /decompress_rocq_parser\(&compressed, &parser\);\s+let mut compiler = cc::Build::new\(\)/,
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
  assert.match(workflow, /npm view "meta-language@\$PACKAGE_VERSION" version >\/dev\/null 2>&1/);
  // OIDC trusted publishing is the steady-state mechanism, so the only
  // NODE_AUTH_TOKEN in the workflow is the optional NPM_TOKEN bootstrap
  // fallback, and the placeholder credential setup-node writes must be
  // stripped before publishing (issue #191, actions/setup-node#1551).
  assert.match(workflow, /node scripts\/prepare-npm-auth\.mjs/);
  assert.equal(workflow.match(/^\s*NODE_AUTH_TOKEN:/gm).length, 1);
  assert.match(workflow, /NODE_AUTH_TOKEN:\s+\$\{\{\s*secrets\.NPM_TOKEN\s*\}\}/);
  assert.match(workflow, /permissions:\s*\n\s+contents:\s+read/);
  assert.doesNotMatch(workflow.split('\njobs:\n')[0], /\nconcurrency:\n/);

  const publishJob = workflow.slice(workflow.indexOf('  publish:\n'));
  assert.match(publishJob, /group:\s+release-\$\{\{ github\.repository \}\}-main-write/);
  assert.match(publishJob, /cancel-in-progress:\s+false/);
  assert.doesNotMatch(publishJob, /queue:\s+/);
  assert.match(publishJob, /REQUESTED_VERSION:\s+\$\{\{ github\.event\.inputs\.release_version \}\}/);
  assert.doesNotMatch(publishJob, /run:[^\n]*\$\{\{\s*github\.event\.inputs\.release_version\s*\}\}/);
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
    assert.match(job, /gh workflow run js\.yml --ref main/);
    assert.match(job, /release_version="\$RELEASE_VERSION"/);
    assert.ok(createRelease >= 0 && dispatchPublisher > createRelease);
  }
});

test('issue 195 acceptance workflow produces exact-checkpoint evidence with pinned toolchains', async () => {
  const workflow = await readFile(
    new URL('../../.github/workflows/issue-195-acceptance.yml', import.meta.url),
    'utf8',
  );

  assert.match(workflow, /node-version:\s*24/);
  assert.match(workflow, /dtolnay\/rust-toolchain@1\.98\.1/);
  assert.match(workflow, /ocaml\/setup-ocaml@v3/);
  assert.match(
    workflow,
    /opam repository add rocq-released https:\/\/rocq-prover\.org\/opam\/released/,
  );
  assert.match(workflow, /opam install[^\n]*rocq-core=9\.2\.0[^\n]*rocq-stdlib=9\.2\.0/);
  assert.doesNotMatch(workflow, /leanprover\/lean-action/);
  assert.match(workflow, /elan-x86_64-unknown-linux-gnu\.tar\.gz/);
  assert.match(workflow, /42b94d4244e8353142c456ec0e4ca6528fd898a6c604d4059f494e706e431f63/);
  assert.match(workflow, /leanprover\/lean4:v4\.33\.1/);
  assert.match(workflow, /node js\/scripts\/run-issue-195-evidence\.mjs/);
  assert.match(workflow, /--checkpoint "\$ACCEPTANCE_CHECKPOINT"/);
  assert.match(workflow, /--commit "\$GITHUB_SHA"/);
  assert.match(workflow, /npm ci --ignore-scripts/);
});
