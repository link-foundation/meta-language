// Report the changed fields in one regenerated consuming snapshot before its
// normal conservation check. The hook leaves the checked files on disk intact.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { registerHooks } from 'node:module';
import { isDeepStrictEqual } from 'node:util';

const producer = new URL('./raw-consuming-emission.mjs', import.meta.url).href;
const report = Symbol.for('meta-language raw consuming snapshot comparison');
const checkpoint = '  verifyOriginalLinkRawConsumingConservation(snapshotArtifact);';

function summarize(value) {
  if (typeof value !== 'string' || value.length <= 200) return value;
  return { bytes: Buffer.byteLength(value), sha256: createHash('sha256').update(value).digest('hex') };
}

function reportDifferences(previous, current, path = []) {
  if (Object.is(previous, current)) return;
  if (previous && current && typeof previous === 'object' && typeof current === 'object'
    && Array.isArray(previous) === Array.isArray(current)) {
    const previousKeys = Object.keys(previous);
    const currentKeys = Object.keys(current);
    const keys = new Set([...previousKeys, ...currentKeys]);
    if (keys.size === previousKeys.length && keys.size === currentKeys.length
      && !isDeepStrictEqual(previousKeys, currentKeys)) {
      console.log(JSON.stringify({ path, previousKeys, currentKeys }));
    }
    for (const key of keys) reportDifferences(previous[key], current[key], [...path, key]);
    return;
  }
  console.log(JSON.stringify({ path, previous: summarize(previous), current: summarize(current) }));
}

globalThis[report] = reportDifferences;
const hooks = registerHooks({
  load(url, context, nextLoad) {
    const loaded = nextLoad(url, context);
    if (url !== producer) return loaded;
    const source = Buffer.from(loaded.source).toString('utf8');
    assert.equal(source.split(checkpoint).length, 2);
    const comparison = '  globalThis[Symbol.for("meta-language raw consuming snapshot comparison")](readProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT), { ...snapshotArtifact, archive: currentArtifact.archive });';
    return { ...loaded, source: source.replace(checkpoint, `${comparison}\n${checkpoint}`) };
  },
});

try {
  const { generateOriginalLinkRawConsumingEmission } = await import(producer);
  await generateOriginalLinkRawConsumingEmission(true);
} finally {
  hooks.deregister();
  delete globalThis[report];
}
