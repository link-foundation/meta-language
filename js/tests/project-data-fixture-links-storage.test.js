import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import { createFixtureSourceLayout, readFixtureSourceBytes, renderFixtureSourceBytes } from '../scripts/fixture-source-bytes.mjs';
import { readStructuredDataLino } from '../src/structured-data-links.js';
import { normalizeMigrationProviderReceipts } from '../scripts/validate-structured-data-migration.mjs';

const root = new URL('../../', import.meta.url);
const hash = value => createHash('sha256').update(value).digest('hex');

test('fixture Links data preserves every original ordered decoded baseline', () => {
  const baselines = readProjectData(new URL('../../parity/fixtures/structured-data-migration-baselines.lino', import.meta.url));
  assert.ok(baselines.length >= 76);
  assert.equal(new Set(baselines.map(entry => entry.logicalPath)).size, baselines.length);
  const revisions = readProjectData(new URL('../../parity/fixtures/structured-data-migration-revisions.lino', import.meta.url));
  const contract = readProjectData(new URL('../../parity/fixtures/project-data-tooling-contract.lino', import.meta.url));
  for (const { logicalPath, decodedSha256, sourceSha256 } of baselines) {
    const file = new URL(logicalPath, root);
    assert.equal(existsSync(file), false, `${logicalPath} has one authoritative Links representation`);
    assert.ok(resolveProjectDataPath(file).endsWith('.lino'), logicalPath);
    let protectedValue = readProjectData(file);
    const refresh = contract.currentFreshnessChecks.find(check => check.logicalPaths.includes(logicalPath))?.providerReceiptRefresh;
    if (refresh) {
      const revision = revisions.find(entry => entry.logicalPath === logicalPath)?.migrationRevision;
      assert.match(revision, /^[0-9a-f]{40}$/u);
      const historical = readStructuredDataLino(execFileSync('git', ['show', `${revision}:${logicalPath.replace(/\.json$/u, '.lino')}`],
        { cwd: fileURLToPath(root), encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 }));
      assert.equal(hash(JSON.stringify(historical)), decodedSha256, `${logicalPath}: frozen original conversion`);
      for (const { language, provider } of refresh) {
        const catalog = readProjectData(new URL('../../js/src/data/language-catalog.lino', import.meta.url));
        const grammar = catalog.languages.find(entry => entry.name === language).grammars.find(entry => entry.id === provider);
        assert.equal(protectedValue.languages[language].grammars[provider].parserSha256, grammar.parserSha256);
        for (const runtime of ['js', 'rust']) assert.equal(hash(readFileSync(new URL(`../../${runtime}/src/data/${catalog.nativeGrammars[provider].file}`, import.meta.url))), grammar.parserSha256);
      }
      protectedValue = normalizeMigrationProviderReceipts(historical, protectedValue, refresh);
    }
    assert.equal(hash(JSON.stringify(protectedValue)), decodedSha256, `${logicalPath}: values and ordered occurrences`);
    if (sourceSha256) assert.equal(hash(readFixtureSourceBytes(file)), sourceSha256, `${logicalPath}: unchanged provenance bytes`);
  }
});

test('fixture provenance formatting retains Unicode spellings and whitespace as separate records', () => {
  const source = '{\n  "source": "\\ufeffhéllo\\u0301", "number": 1e2, "array": [ true, null ]\n}\n';
  const value = JSON.parse(source);
  const layout = createFixtureSourceLayout(source);
  assert.ok(layout.whitespace.length > 0);
  assert.ok(layout.spellings.length > 0);
  assert.equal(renderFixtureSourceBytes(value, layout).toString('utf8'), source);
  assert.throws(() => renderFixtureSourceBytes({ ...value, array: [] }, layout), /token count/);
  assert.throws(() => renderFixtureSourceBytes(value, { ...layout, whitespace: [[0, 'source code']] }), /whitespace/);
  assert.throws(() => renderFixtureSourceBytes(value, { ...layout, spellings: [[3, '"a different source"']] }), /spelling/);
});
