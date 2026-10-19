import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
const worker = fileURLToPath(new URL('../scripts/isolated-grammar-oracle.mjs', import.meta.url));

test('isolated oracle snapshots retain pinned rows and recovery diagnostics', () => {
  for (const source of ['{"grove":7}', '{"grove":']) {
    const expected = oracleSnapshot(source, 'JSON');
    for (let repeat = 0; repeat < 2; repeat += 1) {
      const result = spawnSync(process.execPath, [worker], {
        input: JSON.stringify({ source, language: 'JSON' }), encoding: 'utf8',
        env: { ...process.env, META_LANGUAGE_ORACLE_WORKER: '1' }, maxBuffer: 1 << 20,
      });
      assert.equal(result.status, 0, result.error?.message ?? result.stderr);
      assert.deepEqual(JSON.parse(result.stdout), expected);
    }
  }
});
