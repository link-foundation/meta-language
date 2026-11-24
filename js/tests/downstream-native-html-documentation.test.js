import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'js/README.md';
const packageDirectory = new URL('../', import.meta.url);

test('the native HTML README example runs through the public package entry point', (context) => {
  const readme = readFileSync(new URL('README.md', packageDirectory), 'utf8');
  const section = readme.split('### Parse HTML with source positions\n')[1]?.split('\n## ')[0];
  assert.ok(section);
  const example = /\x60{3}js\n([^]*?)\n\x60{3}/u.exec(section)?.[1];
  assert.ok(example);
  assert.match(example, /from 'meta-language';/u);
  const output = execFileSync(process.execPath, ['--input-type=module', '--eval', example], {
    cwd: packageDirectory,
    encoding: 'utf8',
  });
  assert.equal(output.trim().replaceAll('\r\n', '\n'),
    `article [ 'lang="fr"' ] [ 'été', '<em>🙂</em>' ]\n[ 19, 24 ] [ 19, 22 ]`);
  recordIssue195Observations({
    requirementId: 'I195-NPM-NATIVE-HTML-DOCUMENTATION',
    suffix: 'example', fixtureId: 'npm:html-readme', fixtureFile,
    assertions: ['publicEntryPoint', 'tagAttributesChildrenTextAndSpans'], testName: context.name,
  });
});

test('the documented Node.js minimum matches the npm engine declaration', (context) => {
  const readme = readFileSync(new URL('README.md', packageDirectory), 'utf8');
  const manifest = JSON.parse(readFileSync(new URL('package.json', packageDirectory), 'utf8'));
  assert.equal(manifest.engines.node, '>=22');
  assert.match(readme, /The package requires Node\.js 22 or later\./u);
  assert.match(readme, /shields\.io\/badge\/node-%3E%3D22-blue\.svg/u);
  recordIssue195Observations({
    requirementId: 'I195-NPM-NATIVE-HTML-NODE-MINIMUM',
    suffix: 'documentation', fixtureId: 'npm:html-readme', fixtureFile,
    assertions: ['declaredEngineMatchesDocumentation'], testName: context.name,
  });
});
