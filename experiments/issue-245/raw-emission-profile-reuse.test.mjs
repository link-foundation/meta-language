import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { readOriginalLinkRawSelectedEmissionProfile } from '../original-link-complete-table-projection/raw-native-observations.mjs';

test('raw profile proof reuse rereads every input and rejects modified producer bytes', () => {
  const paths = [];
  let changedPath;
  const read = (file) => {
    paths.push(file.pathname);
    const bytes = readFileSync(file);
    if (file.pathname === changedPath) bytes[0] ^= 1;
    return bytes;
  };
  const original = readOriginalLinkRawSelectedEmissionProfile('current', read);
  assert.equal(original.profile, 'current');
  const originalPaths = paths.splice(0);
  assert.ok(originalPaths.some(path => path.endsWith('/raw-current-native-definitions.lino')));
  assert.ok(originalPaths.some(path => path.endsWith('/raw-native-definitions.lino')));
  assert.strictEqual(readOriginalLinkRawSelectedEmissionProfile('current', read), original);
  assert.deepEqual(paths.splice(0), originalPaths);

  changedPath = originalPaths.find(path => path.endsWith('/raw-rules.mjs'));
  assert.ok(changedPath);
  assert.throws(() => readOriginalLinkRawSelectedEmissionProfile('current', read), /entire original raw source changed/u);
  assert.deepEqual(paths.splice(0), originalPaths);
  changedPath = undefined;
  assert.strictEqual(readOriginalLinkRawSelectedEmissionProfile('current', read), original);
  assert.deepEqual(paths, originalPaths);
});
