// Run with Node 22 and --max-old-space-size=512. This verifies profiles without planning native observations.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {createOriginalLinkRawEmissionArtifactReader} from '../original-link-complete-table-projection/raw-native-observations.mjs';
import {
  ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,
  ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT,
  readOriginalLinkRawEmissionProducerSources,
  readOriginalLinkRawEmissionProfile,
} from '../original-link-complete-table-projection/raw-emission-profiles.mjs';

const artifactPaths = [ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT, ORIGINAL_LINK_RAW_CURRENT_ARTIFACT];
const rulePath = new URL('../original-link-complete-table-projection/raw-rules.mjs', import.meta.url);
const readArtifactWithReuse = artifactPaths.map(createOriginalLinkRawEmissionArtifactReader);
const repetitions = 5;

function compare(readArtifact) {
  const started = performance.now();
  const [historicalArtifact, currentArtifact] = artifactPaths.map(readArtifact);
  const decoded = performance.now();
  const result = readOriginalLinkRawEmissionProfile('current', {
    historicalArtifact,
    currentArtifact,
    source: readFileSync(rulePath, 'utf8'),
    producerSources: readOriginalLinkRawEmissionProducerSources(),
  });
  return {result, decodeMilliseconds: decoded - started, verifyMilliseconds: performance.now() - decoded};
}

for (let iteration = 1; iteration <= repetitions; iteration++) {
  const fresh = compare(path => readStructuredDataLino(readFileSync(path, 'utf8')));
  const reused = compare((_path, index) => readArtifactWithReuse[index]());
  assert.deepEqual(reused.result, fresh.result);
  console.log(JSON.stringify({iteration,
    freshDecodeMilliseconds: Math.round(fresh.decodeMilliseconds),
    freshVerifyMilliseconds: Math.round(fresh.verifyMilliseconds),
    reusedDecodeMilliseconds: Math.round(reused.decodeMilliseconds),
    reusedVerifyMilliseconds: Math.round(reused.verifyMilliseconds),
  }));
}
