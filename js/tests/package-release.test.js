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
  assert.doesNotMatch(workflow, /NODE_AUTH_TOKEN/);
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
