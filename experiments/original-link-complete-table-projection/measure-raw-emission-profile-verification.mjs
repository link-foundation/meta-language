import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { compileFunction } from 'node:vm';
import { readStructuredDataLino } from '../../js/src/structured-data-links.js';
import {
  ORIGINAL_LINK_RAW_CURRENT_ARTIFACT, ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT,
  readOriginalLinkRawEmissionProducerSources, readOriginalLinkRawEmissionProfile,
} from './raw-emission-profiles.mjs';

const path = 'experiments/original-link-complete-table-projection/raw-native-observations.mjs';
const oracleUrl = new URL('./raw-native-observations.mjs', import.meta.url);
const rulesUrl = new URL('./raw-rules.mjs', import.meta.url);
const predecessor = 'ec9f4efb0e9ef936114e09823981352f6db13700';

function measureProfile(source, repeats) {
  let changedRules = false;
  let checks = 0;
  const dependencies = {
    assert, oracleUrl, readStructuredDataLino, ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,
    ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT, readOriginalLinkRawEmissionProducerSources,
    readFileSync: (file, encoding) => {
      const bytes = readFileSync(file, encoding);
      if (!changedRules || file.href !== rulesUrl.href) return bytes;
      return encoding ? bytes + 'modified' : Buffer.concat([bytes, Buffer.from('modified')]);
    },
    readOriginalLinkRawEmissionProfile: (...args) => {
      checks++;
      return readOriginalLinkRawEmissionProfile(...args);
    },
    sha256: value => createHash('sha256').update(value).digest('hex'),
  };
  const start = source.indexOf('function readOriginalLinkRawSelectedEmissionProfile(');
  const end = source.indexOf('\n/** Complete source replay', start);
  assert.ok(start > 0 && end > start);
  const verifier = compileFunction('let selectedEmissionSnapshot;\n' + source.slice(start, end)
    .replaceAll('import.meta.url', 'oracleUrl') + '\nreturn readOriginalLinkRawSelectedEmissionProfile;', Object.keys(dependencies))
    (...Object.values(dependencies));
  const started = performance.now();
  const profile = verifier('current');
  for (let index = 1; index < repeats; index++) assert.deepEqual(verifier('current'), profile);
  const milliseconds = Math.round(performance.now() - started);
  const repeatedChecks = checks;
  changedRules = true;
  assert.throws(() => verifier('current'), assert.AssertionError);
  assert.equal(checks, repeatedChecks + 1);
  changedRules = false;
  assert.deepEqual(verifier('current'), profile);
  return { profile, milliseconds, checks: repeatedChecks };
}

const before = measureProfile(execFileSync('git', ['show', `${predecessor}:${path}`], { encoding: 'utf8' }), 3);
const after = measureProfile(readFileSync(oracleUrl, 'utf8'), 3);
assert.deepEqual(after.profile, before.profile);
assert.equal(before.checks, 3);
assert.equal(after.checks, 1);
console.log(JSON.stringify({ beforeMilliseconds: before.milliseconds, afterMilliseconds: after.milliseconds, reads: 3 }));
