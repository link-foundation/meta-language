import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

test('native array value decisions and linked policies are generated from the whole JavaScript source', () => {
  const generator = fileURLToPath(new URL('../scripts/generate-array-value-rules.mjs', import.meta.url));
  const result = spawnSync(process.execPath, [generator, '--check'], { encoding: 'utf8', timeout: 30000, maxBuffer: 1024 * 1024 });
  assert.equal(result.error, undefined);
  assert.equal(result.status, 0, result.stderr || result.stdout);
  assert.match(result.stdout, /shared optional read admission and complete target rendering/u);
});
