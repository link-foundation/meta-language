// Run with Node 22 and --max-old-space-size=640. This verifies profiles without planning native observations.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { readStructuredDataLino, structuredDataToLino } from '../../js/src/structured-data-links.js';
import { createOriginalLinkRawEmissionArtifactReader } from '../original-link-complete-table-projection/raw-native-observations.mjs';
import { verifyOriginalLinkRawEmissionProducerApplicability } from '../original-link-complete-table-projection/raw-emission-producer-applicability.mjs';
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

const historicalArtifact = readArtifactWithReuse[0]();
for (let iteration = 1; iteration <= repetitions; iteration++) {
  const started = performance.now();
  structuredDataToLino(historicalArtifact);
  const serialized = performance.now();
  verifyOriginalLinkRawEmissionProducerApplicability(readOriginalLinkRawEmissionProducerSources());
  console.log(JSON.stringify({
    iteration,
    historicalSerializationMilliseconds: Math.round(serialized - started),
    producerApplicabilityMilliseconds: Math.round(performance.now() - serialized),
  }));
}

for (const file of ['array-relative-element-history.lino', 'ci-39389240046-historical-inputs.lino']) {
  const path = new URL('../../parity/self-translation/' + file, import.meta.url);
  const started = performance.now();
  const ordinary = readProjectData(path);
  const decoded = performance.now();
  const bytes = readFileSync(path);
  const canonical = readBootstrapStructuredDataLino(bytes.toString('utf8'));
  const finished = performance.now();
  assert.deepEqual(canonical, ordinary);
  console.log(JSON.stringify({
    file,
    bytes: bytes.length,
    originalSources: canonical.sources.length,
    generalDecodeMilliseconds: Math.round(decoded - started),
    canonicalDecodeMilliseconds: Math.round(finished - decoded),
  }));
}
