import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  deliveryAssertions,
  supportedPlatformCoverage,
} from '../scripts/issue-195-delivery.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const corpusPath = 'parity/fixtures/issue-195-evidence.json';
const corpusBytes = await readFile(path.join(root, corpusPath));
const corpus = JSON.parse(corpusBytes.toString('utf8'));
const corpusSha256 = sha256(corpusBytes);
const context = { corpus, corpusSha256, version: '9.9.9' };
const expected = { npm: 'a'.repeat(64), crate: 'b'.repeat(64) };
const { delivery } = corpus;

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

/** A consumer report shaped like `run-issue-195-consumer.mjs` output that meets every check. */
function consumerReport(platform, overrides = {}) {
  const observations = {
    parses: delivery.consumerCorpus.map(() => ({ clean: true, reconstructs: true, requiredTermPresent: true })),
    programs: delivery.programs.map(() => ({ emitted: true, bindings: ['main'] })),
    translations: delivery.translations.map(() => ({ decodes: true })),
  };
  return {
    platform,
    corpusSha256,
    agreement: { mismatches: [] },
    npm: {
      version: '9.9.9',
      checksum: {
        sha256: expected.npm,
        expectedSha256: expected.npm,
        installedIntegrity: 'sha512-x',
        expectedIntegrity: 'sha512-x',
      },
      clean: { consumerWasEmpty: true, freshCache: true },
      offline: { networkAttempts: [], guardRejectsNetwork: true },
      publicEntryPoints: { used: delivery.publicEntryPoints.javascript, privatePathRejected: true },
      observations,
    },
    crate: {
      version: '9.9.9',
      checksum: { sha256: expected.crate, expectedSha256: expected.crate },
      clean: { consumerWasEmpty: true, freshTargetDirectory: true },
      offline: {
        builtOffline: true,
        ranWithNetworkDisabledEnvironment: true,
        networkDependencies: [],
        networkApiFiles: [],
      },
      publicEntryPoints: { used: delivery.publicEntryPoints.rust, privatePathRejected: true },
      observations,
    },
    ...overrides,
  };
}

const allPlatforms = () => delivery.supportedPlatforms.map((platform) => consumerReport(platform));

test('supported platforms are observed when every declared platform report passes', () => {
  for (const sides of [['npm'], ['crate'], ['npm', 'crate']]) {
    assert.deepEqual(supportedPlatformCoverage(allPlatforms(), sides, expected, context), {
      observed: true,
      failures: [],
    });
  }
});

test('a missing platform report leaves supported platforms unobserved', () => {
  const reports = allPlatforms().filter(({ platform }) => platform !== 'win32-x64');
  const coverage = supportedPlatformCoverage(reports, ['npm'], expected, context);
  assert.equal(coverage.observed, false);
  assert.deepEqual(coverage.failures, [{ platform: 'win32-x64', reason: 'no consumer report' }]);
});

test('a report of CRLF corpus bytes is rejected with the differing corpus digest', () => {
  // The Windows consumer hashed a CRLF checkout of the corpus before it was pinned to LF.
  const crlf = sha256(Buffer.from(corpusBytes.toString('utf8').replace(/\n/gu, '\r\n')));
  const reports = allPlatforms().map((report) =>
    report.platform === 'win32-x64' ? { ...report, corpusSha256: crlf } : report);
  const coverage = supportedPlatformCoverage(reports, ['crate'], expected, context);
  assert.equal(coverage.observed, false);
  assert.equal(coverage.failures.length, 1);
  assert.equal(coverage.failures[0].platform, 'win32-x64');
  assert.match(coverage.failures[0].reason, new RegExp(`corpus SHA-256 ${crlf} differs`, 'u'));
  assert.deepEqual(
    deliveryAssertions(reports.at(-1), ['crate'], expected, context),
    { cleanEnvironment: false, exactArtifactChecksum: false, publicEntryPoints: false, offlineFirstParse: false },
  );
});

test('one failing assertion on one platform names the platform and the assertion', () => {
  const reports = allPlatforms();
  const darwin = reports.find(({ platform }) => platform === 'darwin-arm64');
  darwin.npm = { ...darwin.npm, offline: { networkAttempts: ['registry.npmjs.org'], guardRejectsNetwork: true } };
  const coverage = supportedPlatformCoverage(reports, ['npm'], expected, context);
  assert.deepEqual(coverage.failures, [{ platform: 'darwin-arm64', reason: 'failed offlineFirstParse' }]);
  // The crate side of the same report is unaffected.
  assert.equal(supportedPlatformCoverage(reports, ['crate'], expected, context).observed, true);
});

test('cross-runtime cells need npm and crate observations to agree on every platform', () => {
  const reports = allPlatforms();
  reports[0].agreement = { mismatches: [{ program: 'sum', npm: '3', crate: '4' }] };
  const coverage = supportedPlatformCoverage(reports, ['npm', 'crate'], expected, context);
  assert.equal(coverage.observed, false);
  assert.match(coverage.failures[0].reason, /^npm and crate observations disagree/u);
  assert.equal(supportedPlatformCoverage(reports, ['npm'], expected, context).observed, true);
});

test('a checksum or version for another candidate is not accepted', () => {
  const reports = allPlatforms();
  reports[1].crate = { ...reports[1].crate, version: '9.9.8' };
  assert.deepEqual(
    supportedPlatformCoverage(reports, ['crate'], expected, context).failures,
    [{ platform: reports[1].platform, reason: 'failed exactArtifactChecksum' }],
  );
  const otherCandidate = { ...expected, npm: 'c'.repeat(64) };
  assert.equal(supportedPlatformCoverage(allPlatforms(), ['npm'], otherCandidate, context).observed, false);
});

test('the hashed evidence corpus is checked out with LF line endings on every platform', () => {
  const attributes = execFileSync('git', ['check-attr', 'text', 'eol', '--', corpusPath], {
    cwd: root,
    encoding: 'utf8',
  });
  assert.match(attributes, /: text: set$/mu);
  assert.match(attributes, /: eol: lf$/mu);
  assert.equal(corpusBytes.includes(Buffer.from('\r\n')), false);
});

test('a semantic translation traces its source by provenance, and only its exact source', () => {
  const provenance = ({ source, sourceLanguage }) => ({
    sourceLanguage,
    sourceSha256: sha256(Buffer.from(source, 'utf8')),
    sourceBytes: Buffer.byteLength(source, 'utf8'),
  });
  const withTranslations = (translations) => {
    const report = consumerReport('linux-x64');
    for (const side of ['npm', 'crate']) {
      report[side] = { ...report[side], observations: { ...report[side].observations, translations } };
    }
    return report;
  };
  const traced = withTranslations(delivery.translations.map((translation) => ({ decodes: false, provenance: provenance(translation) })));
  assert.equal(deliveryAssertions(traced, ['npm', 'crate'], expected, context).offlineFirstParse, true);
  const other = withTranslations(delivery.translations.map((translation) =>
    ({ decodes: false, provenance: provenance({ ...translation, source: `${translation.source} ` }) })));
  assert.equal(deliveryAssertions(other, ['npm', 'crate'], expected, context).offlineFirstParse, false);
});
