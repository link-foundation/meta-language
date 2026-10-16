import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, unlinkSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { readProjectData, writeProjectData, serializeProjectData } from '../../js/scripts/project-data.mjs';
import { readStructuredDataLino } from '../../js/src/index.js';
import { planRustTextCertificateNative } from '../rust-text-certificate/native-observations.mjs';
import {
  RUST_TEXT_CERTIFICATE_LIBRARY_MODEL_COMMIT, verifyRustTextCertificateLibraryModel, rustTextCertificateLibraryNativeMode,
  planRustTextCertificateLibraryNative, runRustTextCertificateLibraryNative,
} from './native-observations.mjs';

// A real native request fails locally before registering or running any test.
const nativeEnabled = rustTextCertificateLibraryNativeMode(process.env);
const plan = planRustTextCertificateLibraryNative(), entry = plan.entries[0];
const original = planRustTextCertificateNative().entries[0];
const packet = readProjectData(new URL('./data/library.lino', import.meta.url));
const environment = { CI: 'true', RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE: '1', GITHUB_SHA: 'injected-process-contract-only' };
const stream = entry.observations.map((row, index) => index + ':' + row.resultBits.join(':') + '\n').join('');
const sha256 = value => createHash('sha256').update(value).digest('hex');
function fakeProcess(override) {
  const calls = [];
  const spawn = (command, args, options) => {
    const phase = args.includes('--crate-type=rlib') ? 'library' : args.includes('--extern') ? 'consumer' : 'execute';
    const call = { command, args, options, phase }; calls.push(call);
    const replacement = override?.(call); if (replacement) return replacement;
    if (phase !== 'execute') writeFileSync(args[args.indexOf('-o') + 1], 'injected artifact; not native evidence');
    return { status: 0, signal: null, stdout: phase === 'execute' ? stream : '', stderr: '' };
  };
  return { calls, spawn };
}
const injected = fake => runRustTextCertificateLibraryNative(plan, { environment, spawn: fake.spawn });
function failure(action) {
  assert.throws(action, error => {
    assert.ok(error instanceof AggregateError); assert.equal(error.errors.length, 1);
    assert.match(error.message, /Complete recorded Rust library observations failed/u); return true;
  });
}

test('one checked plan retains the complete five-file candidate, all 92 owners and six source hashes', () => {
  assert.equal(plan.modelCommit, RUST_TEXT_CERTIFICATE_LIBRARY_MODEL_COMMIT); assert.equal(plan.originalCount, 1);
  assert.equal(entry.packagedModuleCount, 5); assert.equal(entry.additionalSources.length, 5); assert.equal(entry.sourceFiles.length, 6);
  assert.equal(entry.definitionOwners.length, 92); assert.equal(entry.itemCount, 96);
  assert.deepEqual(entry.definitionOwners, packet.definitions); assert.deepEqual(entry.declarations, original.declarations);
  assert.equal(entry.source, '#[path = "library/mod.rs"]\npub mod text_certificate_library;\npub use text_certificate_library::check_rust_text_certificate;\n');
  assert.doesNotMatch(entry.source, /fn |allow\(|check_rust_text_certificate\(/u, 'the wrapper contains no substitute algorithm or function');
  for (const [index, module] of packet.modules.entries()) {
    const source = entry.additionalSources[index];
    assert.equal(source.relativePath, 'library/' + module.file); assert.equal(source.source, module.source);
    assert.equal(source.sourceBytes, Buffer.byteLength(module.source)); assert.equal(source.sourceSha256, module.sha256);
    assert.equal(sha256(source.source), source.sourceSha256);
  }
  for (const row of entry.sourceFiles) assert.match(row.sourceSha256, /^[0-9a-f]{64}$/u);
  assert.deepEqual(entry.originalArtifactReceipt, original.originalReceipt);
  assert.equal(entry.replaySha256, entry.sourceSha256);
  assert.throws(() => { entry.additionalSources[0].source = 'fn main(){}'; }, TypeError);
  assert.equal(verifyRustTextCertificateLibraryModel().length, 6);
  assert.throws(() => verifyRustTextCertificateLibraryModel(() => Buffer.from('counterfeit packaging input')), /frozen Rust library candidate changed/u);
  const originalOracle = new URL('../rust-text-certificate/native-observations.mjs', import.meta.url).href;
  assert.throws(() => verifyRustTextCertificateLibraryModel(file => file.href === originalOracle
    ? Buffer.from('counterfeit forty-input oracle') : readFileSync(file)), /frozen Rust library candidate changed/u);
});

test('the original complete consumer, 17-vector ABI and every input and result bit are reused unchanged', () => {
  assert.equal(entry.consumerSource, original.consumerSource); assert.equal(entry.consumerSha256, original.consumerSha256);
  assert.equal(sha256(entry.consumerSource), entry.consumerSha256);
  assert.deepEqual(entry.tableOrder, original.tableOrder); assert.equal(entry.tableOrder.length, 17);
  assert.deepEqual(entry.parameterModes, Array(17).fill('Vec<f64>')); assert.equal(entry.resultMode, 'Vec<f64>');
  assert.equal(entry.functionName, 'check_rust_text_certificate'); assert.deepEqual(entry.outputRoles, original.outputRoles);
  assert.equal(entry.invocationCount, 40); assert.equal(entry.originalGraphCount, 13); assert.equal(entry.corruptionCount, 27);
  assert.deepEqual(entry.observations, original.observations); assert.equal(entry.observations.length, 40);
  assert.equal((entry.consumerSource.match(/oracle_source::check_rust_text_certificate\(/gu) ?? []).length, 40);
  assert.ok(entry.validationBoundary.includes('Native compilation and strict lint validation have not been executed for this library packet.'));
});

test('only this oracle explicit CI flag and the exact checked plan authorize any process', () => {
  const fake = fakeProcess();
  for (const missing of [{}, { CI: 'true' }, { CI: 'true', RUST_TEXT_CERTIFICATE_NATIVE: '1' }]) {
    assert.equal(rustTextCertificateLibraryNativeMode(missing), false);
    assert.throws(() => runRustTextCertificateLibraryNative(plan, { environment: missing, spawn: fake.spawn }), /explicit CI-only switch/u);
  }
  for (const local of [{ RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE: '1' }, { CI: 'false', RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE: '1' }]) {
    assert.throws(() => runRustTextCertificateLibraryNative(plan, { environment: local, spawn: fake.spawn }), /requires CI=true/u);
  }
  assert.throws(() => runRustTextCertificateLibraryNative(structuredClone(plan), { environment, spawn: fake.spawn }), /exact checked in-memory/u);
  assert.equal(fake.calls.length, 0);
});

test('injected process contracts read every unchanged source before all three process phases', () => {
  const fake = fakeProcess(({ args, options, phase }) => {
    assert.equal(options.shell, false); assert.equal(options.encoding, 'utf8'); assert.equal(options.env, environment);
    assert.equal(options.maxBuffer, 1048576); assert.equal(options.timeout, phase === 'execute' ? 10000 : 30000);
    if (phase !== 'execute') assert.equal(args[1], '--error-format=short');
    assert.equal(readFileSync(join(options.cwd, 'original.rs'), 'utf8'), entry.source);
    assert.equal(readFileSync(join(options.cwd, 'consumer.rs'), 'utf8'), original.consumerSource);
    for (const source of entry.additionalSources) assert.deepEqual(readFileSync(join(options.cwd, source.relativePath)), Buffer.from(source.source));
    if (phase === 'library') assert.equal(args[args.indexOf('--crate-name') + 2], join(options.cwd, 'original.rs'));
    if (phase === 'consumer') assert.ok(args.includes('oracle_source=' + join(options.cwd, 'liboracle_source.rlib')));
    if (phase === 'execute') assert.deepEqual(args, []);
  });
  const receipt = injected(fake);
  assert.equal(receipt.processMode, 'injected process contract');
  assert.deepEqual(receipt.processCounts, { libraryCompilations: 1, consumerCompilations: 1, executions: 1 }); assert.equal(fake.calls.length, 3);
  assert.equal(receipt.observations.length, 1); const observed = receipt.observations[0];
  assert.equal(observed.sourceFileCount, 6); assert.equal(observed.packagedModuleCount, 5); assert.equal(observed.itemCount, 96);
  assert.deepEqual(observed.sourceFiles, entry.sourceFiles);
  assert.deepEqual(observed.additionalSources, entry.additionalSources.map(({ source: _source, ...row }) => row));
  assert.deepEqual(observed.observations.map(row => row.resultBits), original.observations.map(row => row.resultBits));
  assert.equal(observed.observations.length, 40); assert.equal(observed.definitionOwners.length, 92);
  assert.equal(existsSync(fake.calls[0].options.cwd), false);
  assert.deepEqual(readStructuredDataLino(serializeProjectData(receipt, 'observations.lino')), receipt);
});

test('editing any recorded module during any process phase cannot produce an observation', () => {
  for (const source of entry.additionalSources) for (const phase of ['library', 'consumer', 'execute']) {
    const fake = fakeProcess(call => {
      if (call.phase === phase) writeFileSync(join(call.options.cwd, source.relativePath), source.source + '\n// process-side source change\n');
    });
    failure(() => injected(fake)); assert.equal(fake.calls.at(-1).phase, phase);
    assert.equal(existsSync(fake.calls[0].options.cwd), false);
  }
  const removed = fakeProcess(call => { if (call.phase === 'library') unlinkSync(join(call.options.cwd, entry.additionalSources[0].relativePath)); });
  failure(() => injected(removed)); assert.equal(removed.calls.length, 1); assert.equal(existsSync(removed.calls[0].options.cwd), false);
});

test('wrong bits, missing or reordered calls and compiler failures remain failed observations', () => {
  const lines = stream.trimEnd().split('\n'), changed = entry.observations[0].resultBits.slice();
  assert.equal(changed.at(-1), '0000000000000000'); changed[changed.length - 1] = '8000000000000000';
  for (const output of [
    ['0:' + changed.join(':'), ...lines.slice(1)].join('\n') + '\n',
    stream.replace('3ff0000000000000', '0000000000000000'),
    [lines[1], lines[0], ...lines.slice(2)].join('\n') + '\n',
    lines.slice(1).join('\n') + '\n', stream.slice(0, -1), stream + lines[0] + '\n',
  ]) {
    const fake = fakeProcess(call => call.phase === 'execute' ? { status: 0, stdout: output, stderr: '' } : null);
    failure(() => injected(fake)); assert.equal(fake.calls.length, 3);
  }
  for (const phase of ['library', 'consumer', 'execute']) {
    const fake = fakeProcess(call => call.phase === phase ? { status: 1, stdout: '', stderr: 'actual subprocess rejection' } : null);
    failure(() => injected(fake)); assert.equal(fake.calls.at(-1).phase, phase);
  }
});

// A real compiler is reachable only in CI under this oracle's own explicit flag.
if (nativeEnabled) test('CI executes all five recorded library files over the original forty complete oracle inputs', () => {
  const receipt = runRustTextCertificateLibraryNative(plan);
  assert.equal(receipt.processMode, 'native subprocesses'); assert.equal(receipt.originalCount, 1);
  assert.deepEqual(receipt.processCounts, { libraryCompilations: 1, consumerCompilations: 1, executions: 1 });
  assert.equal(receipt.observations.length, 1); const observed = receipt.observations[0];
  assert.equal(observed.sourceFileCount, 6); assert.equal(observed.packagedModuleCount, 5);
  assert.equal(observed.observations.length, 40); assert.equal(observed.invocationCount, 40);
  assert.equal(observed.originalGraphCount, 13); assert.equal(observed.corruptionCount, 27);
  assert.deepEqual(observed.sourceFiles, entry.sourceFiles); assert.deepEqual(observed.definitionOwners, packet.definitions);
  const file = fileURLToPath(new URL('../../issue-195-artifacts/rust-text-certificate-library-native-observations.lino', import.meta.url));
  mkdirSync(dirname(file), { recursive: true }); writeProjectData(file, receipt);
  process.stdout.write('Complete recorded Rust library observations: 5 unchanged candidate files, 40 actual calls; Links Notation receipts: ' + file + '\n');
});
