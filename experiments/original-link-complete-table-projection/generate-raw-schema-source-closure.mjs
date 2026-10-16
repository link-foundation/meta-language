// This generator records only a successful JavaScript-to-Rust-text freshness
// check. It never launches rustc, a native planner or an injected observation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync, spawnSync} from 'node:child_process';
import {existsSync, readFileSync, writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData, readProjectData, writeProjectData} from '../../js/scripts/project-data.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FILE, ORIGINAL_LINK_RAW_SCHEMA_PRODUCER, ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES,
  ORIGINAL_LINK_RAW_SCHEMA_ADDITIONS, readOriginalLinkRawHistoricalSources, verifyOriginalLinkRawSchemaSourceClosure} from './raw-schema-source-closure.mjs';
import {verifyOriginalLinkRawSchemaExtensionClosure} from './raw-schema-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaSecondExtensionClosure} from './raw-schema-second-extension-closure.mjs';
import {verifyOriginalLinkRawNativeModel} from './raw-native-observations.mjs';

const root = new URL('../../', import.meta.url), receiptFile = new URL('./data/raw-schema-source-closure.lino', import.meta.url);
const oracleFile = new URL('./raw-native-observations.mjs', import.meta.url);
const extensionFile = new URL('./data/raw-schema-extension-closure.lino', import.meta.url);
const secondExtensionFile = new URL('./data/raw-schema-second-extension-closure.lino', import.meta.url);
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');
const command = 'node experiments/original-link-complete-table-projection/generate-raw.mjs --native --check';
const gitSource = (commit, file) => execFileSync('git', ['show', commit + ':' + file], {cwd: root, maxBuffer: 1024 * 1024});

/** Keep the complete producer closure and independently checked schema transition. */
export function generateOriginalLinkRawSchemaSourceClosure({check = false, schemaCommit} = {}) {
  const actualBytes = readFileSync(new URL('../../' + ORIGINAL_LINK_RAW_SCHEMA_FILE, import.meta.url));
  const historicalSources = readOriginalLinkRawHistoricalSources(readFileSync(oracleFile, 'utf8'));
  if (check) {
    const receipt = readProjectData(receiptFile);
    if (existsSync(secondExtensionFile)) {
      verifyOriginalLinkRawSchemaSecondExtensionClosure(receipt, readProjectData(extensionFile), actualBytes, historicalSources, readProjectData(secondExtensionFile));
      checkProjectData(extensionFile, readProjectData(extensionFile));
      checkProjectData(secondExtensionFile, readProjectData(secondExtensionFile));
    } else if (existsSync(extensionFile)) {
      verifyOriginalLinkRawSchemaExtensionClosure(receipt, actualBytes, historicalSources, readProjectData(extensionFile));
      checkProjectData(extensionFile, readProjectData(extensionFile));
    } else verifyOriginalLinkRawSchemaSourceClosure(receipt, actualBytes, historicalSources);
    // Every original source remains authenticated, including the independent
    // runner closure and this schema's separately pinned append-only closure.
    assert.equal(verifyOriginalLinkRawNativeModel().length, 47);
    checkProjectData(receiptFile, receipt);
    const pin = /^const schemaClosureReceiptSha256 = '([0-9a-f]{64})';$/mu.exec(readFileSync(oracleFile, 'utf8'))?.[1];
    assert.equal(pin, sha256(readFileSync(receiptFile)), 'the source-closure receipt pin matches its canonical bytes');
    return receipt;
  }
  assert.equal(existsSync(extensionFile), false, 'the original receipt is immutable after a separate extension was recorded');
  assert.match(schemaCommit ?? '', /^[0-9a-f]{40}$/u, 'a committed immutable schema revision is required');
  const baseline = gitSource(ORIGINAL_LINK_RAW_SCHEMA_PRODUCER, ORIGINAL_LINK_RAW_SCHEMA_FILE);
  const current = gitSource(schemaCommit, ORIGINAL_LINK_RAW_SCHEMA_FILE);
  assert.deepEqual(current, actualBytes, 'the captured committed schema is the actual current file');
  const historicalOracle = gitSource(schemaCommit, 'experiments/original-link-complete-table-projection/raw-native-observations.mjs');
  assert.deepEqual(Object.entries(readOriginalLinkRawHistoricalSources(historicalOracle.toString('utf8'))), Object.entries(historicalSources));
  for (const [file, expected] of Object.entries(historicalSources)) {
    if (file !== '../../' + ORIGINAL_LINK_RAW_SCHEMA_FILE) assert.equal(sha256(readFileSync(new URL(file, import.meta.url))), expected, file);
  }
  const receipt = {
    version: 1, producerCommit: ORIGINAL_LINK_RAW_SCHEMA_PRODUCER,
    historicalClosure: {commit: schemaCommit, file: 'experiments/original-link-complete-table-projection/raw-native-observations.mjs',
      bytes: historicalOracle.length, sha256: sha256(historicalOracle)},
    historicalSources,
    baseline: {commit: ORIGINAL_LINK_RAW_SCHEMA_PRODUCER, file: ORIGINAL_LINK_RAW_SCHEMA_FILE,
      bytes: baseline.length, sha256: sha256(baseline), source: baseline.toString('utf8')},
    current: {commit: schemaCommit, file: ORIGINAL_LINK_RAW_SCHEMA_FILE, bytes: current.length, sha256: sha256(current)},
    consumedSchemaProperties: [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES], addedUnusedSchemaProperties: [...ORIGINAL_LINK_RAW_SCHEMA_ADDITIONS],
    generation: {command, exitCode: 0, sourceSha256: historicalSources['raw-rules.mjs'], definitionCount: 106, completeRustBytes: 351746,
      completeRustSha256: '24db774177e047de0c0ea22adb2464e4e8481d03547f5a810b2d9fe317fb3046',
      artifactSha256: historicalSources['data/raw-native-definitions.lino']},
    scope: 'Authenticated additive host schema source closure only; no native execution or replacement of the immutable 9123 producer artifact.',
  };
  // Validate all immutable and additive premises before doing the one approved
  // complete generation check; no receipt is written until that command passes.
  verifyOriginalLinkRawSchemaSourceClosure(receipt, actualBytes, historicalSources);
  const result = spawnSync(process.execPath, ['--max-old-space-size=1024',
    fileURLToPath(new URL('./generate-raw.mjs', import.meta.url)), '--native', '--check'],
  {cwd: root, encoding: 'utf8', maxBuffer: 64 * 1024, env: process.env});
  if (result.error) throw result.error;
  assert.equal(result.signal, null, result.stderr);
  assert.equal(result.status, 0, result.stdout + result.stderr);
  receipt.generation.stdout = result.stdout;receipt.generation.stderr = result.stderr;
  assert.equal(sha256(readFileSync(new URL('./data/raw-native-definitions.lino', import.meta.url))), receipt.generation.artifactSha256);
  const artifact = readProjectData(new URL('./data/raw-native-definitions.lino', import.meta.url));
  receipt.generation.sourceSha256 = artifact.sourceSha256;
  receipt.generation.definitionCount = artifact.declarations.length;
  receipt.generation.completeRustBytes = Buffer.byteLength(artifact.completeRust);
  receipt.generation.completeRustSha256 = sha256(artifact.completeRust);
  assert.deepEqual(readFileSync(new URL('../../' + ORIGINAL_LINK_RAW_SCHEMA_FILE, import.meta.url)), actualBytes);
  verifyOriginalLinkRawSchemaSourceClosure(receipt, actualBytes, historicalSources);
  writeProjectData(receiptFile, receipt);
  const before = readFileSync(oracleFile, 'utf8'), pinPattern = /^const schemaClosureReceiptSha256 = '(?:PENDING|[0-9a-f]{64})';$/mu;
  assert.ok(pinPattern.test(before), 'one exact receipt pin is required');
  writeFileSync(oracleFile, before.replace(pinPattern, "const schemaClosureReceiptSha256 = '" + sha256(readFileSync(receiptFile)) + "';"));
  return receipt;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2), check = args[0] === '--check';
  assert.ok(check ? args.length === 1 : args.length === 2 && args[0] === '--schema-commit', 'use --check or --schema-commit COMMIT');
  const receipt = generateOriginalLinkRawSchemaSourceClosure({check, schemaCommit: check ? undefined : args[1]});
  console.log('Raw schema source closure: ' + receipt.current.commit + '; immutable native producer ' + receipt.producerCommit);
}
