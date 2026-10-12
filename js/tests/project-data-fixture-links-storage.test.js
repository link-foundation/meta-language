import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync } from 'node:fs';
import test from 'node:test';
import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import { createFixtureSourceLayout, readFixtureSourceBytes, renderFixtureSourceBytes } from '../scripts/fixture-source-bytes.mjs';

const root = new URL('../../', import.meta.url);
const hash = value => createHash('sha256').update(value).digest('hex');

test('fixture Links data preserves every original ordered decoded baseline', () => {
  const baselines = readProjectData(new URL('../../parity/fixtures/structured-data-migration-baselines.lino', import.meta.url));
  assert.ok(baselines.length >= 76);
  assert.equal(new Set(baselines.map(entry => entry.logicalPath)).size, baselines.length);
  for (const { logicalPath, decodedSha256, sourceSha256 } of baselines) {
    const file = new URL(logicalPath, root);
    assert.equal(existsSync(file), false, `${logicalPath} has one authoritative Links representation`);
    assert.ok(resolveProjectDataPath(file).endsWith('.lino'), logicalPath);
    assert.equal(hash(JSON.stringify(readProjectData(file))), decodedSha256, `${logicalPath}: values and ordered occurrences`);
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
