// Authoring optimization only. Every lookup compares every complete input byte.
import assert from 'node:assert/strict';

/** Retain one successful result per explicit profile; failures never enter it. */
export function createOriginalLinkRawProfileCache() {
  const profiles = new Map();
  return (profile, inputs, verify) => {
    assert.equal(typeof profile, 'string');
    assert.equal(typeof verify, 'function');
    const entries = Object.entries(inputs);
    assert.ok(entries.length > 0 && entries.every(([, bytes]) => Buffer.isBuffer(bytes)));
    const previous = profiles.get(profile);
    if (previous && previous.entries.length === entries.length && entries.every(([file, bytes], index) =>
      previous.entries[index][0] === file && previous.entries[index][1].equals(bytes))) {
      return structuredClone(previous.result);
    }
    // Own the inputs before invoking the verifier, which may mutate its buffers.
    const snapshot = entries.map(([file, bytes]) => [file, Buffer.from(bytes)]);
    const result = verify();
    const actual = Object.entries(inputs);
    assert.ok(actual.length === snapshot.length && actual.every(([file, bytes], index) =>
      file === snapshot[index][0] && Buffer.isBuffer(bytes) && snapshot[index][1].equals(bytes)), 'profile inputs changed during verification');
    profiles.set(profile, {entries: snapshot, result: structuredClone(result)});
    return result;
  };
}
