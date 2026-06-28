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
  assert.match(workflow, /npm view meta-language@\$\{\{\s*steps\.package\.outputs\.version\s*\}\} version/);
});
