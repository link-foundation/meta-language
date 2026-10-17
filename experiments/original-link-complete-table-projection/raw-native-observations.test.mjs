// Heavy contracts deliberately require CI, including when the native flag is 0.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, mkdirSync, readFileSync, writeFileSync} from 'node:fs';
import {dirname, join} from 'node:path';
import test from 'node:test';
import {createSourceNativeObservationRunner} from '../source-native-observations.mjs';
import {createOriginalLinkRawConsumerPacket, decodeOriginalLinkRawCarrierData, requireOriginalLinkRawPlanning, originalLinkRawNativeMode, planOriginalLinkRawNative, runOriginalLinkRawNative, runOriginalLinkRawPlanningPhases, verifyOriginalLinkRawNativeModel} from './raw-native-observations.mjs';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {parseLinoStatements} from '../../js/src/lino-serialization.js';
import {formatLinoReadings} from '../../js/src/lino-semantics.js';
import {readOriginalLinkRawHistoricalSources} from './raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES, readOriginalLinkRawSchemaFifthContext, verifyOriginalLinkRawSchemaFifthExtensionClosure} from './raw-schema-fifth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_EMISSION_BASELINE} from './raw-emission-profiles.mjs';
import './raw-emission-profiles.test.mjs';

const verifyOriginalLinkRawSchemaSourceClosure = (receipt, bytes, historicalSources) => verifyOriginalLinkRawSchemaFifthExtensionClosure(
  {...readOriginalLinkRawSchemaFifthContext(), receipt, historicalSources}, bytes,
  readStructuredDataLino(readFileSync(new URL('./data/raw-schema-fifth-extension-closure.lino', import.meta.url), 'utf8')),
  Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES.map(file => [file, readFileSync(new URL('../../' + file, import.meta.url))])));

const schemaClosureFixture = () => ({
  receipt: readStructuredDataLino(readFileSync(new URL('./data/raw-schema-source-closure.lino', import.meta.url), 'utf8')),
  bytes: readFileSync(new URL('../../js/src/data/typed-program-links-schema.lino', import.meta.url)),
  historicalSources: readOriginalLinkRawHistoricalSources(readFileSync(new URL('./raw-native-observations.mjs', import.meta.url), 'utf8')),
});

function rawConsumerDataFixture() {
  const carrier = {words: ['0000000000000000', '8000000000000000', '7ff0000000000000', 'fff0000000000000', '7ff8000000000001', 'ffffffffffffffff'],
    blocks: [[0, 1, 4], [2, 3, 5], []], vectors: [[0, 1], [2], [1, 0]], invocations: [{columns: [0, 1], result: 2}, {columns: [2, 0], result: 1}]};
  const vectorBits = index => carrier.vectors[index].flatMap(block => carrier.blocks[block].map(word => carrier.words[word]));
  const roots = [['0000000000000000', '8000000000000000'], ['7ff8000000000001', '7ff0000000000000']];
  const observations = carrier.invocations.map((row, index) => ({inputBits: [...roots[index], row.columns.map(vectorBits)], resultBits: vectorBits(row.result)}));
  return {carrier, observations, header: {functionName: 'check_original_link_raw_shape'}};
}

test('raw consumer data restores every exact original carrier cell and keeps compiled source independent of data size', () => {
  const {carrier, observations, header} = rawConsumerDataFixture(), packet = createOriginalLinkRawConsumerPacket(observations, carrier, header);
  const data = packet.additionalSources[0], restored = decodeOriginalLinkRawCarrierData(data.source);
  assert.deepEqual(restored, {...carrier, invocations: carrier.invocations.map((row, index) => ({rootBits: observations[index].inputBits.slice(0, 2), ...row}))});
  assert.equal(data.relativePath, 'carrier.lino');assert.equal(data.sourceBytes, Buffer.byteLength(data.source));
  assert.equal(data.sourceSha256, createHash('sha256').update(data.source).digest('hex'));
  const statements = parseLinoStatements(data.source);
  assert.equal(statements[0].id, 'carrier');assert.equal(statements[0].values[0].id, 'schema');
  assert.deepEqual(statements[0].values.slice(1).map(row => row.id), ['words', 'blocks', 'vectors', 'invocations']);
  const reading = row => row.values.length ? [row.id, ...row.values.map(reading)] : row.id;
  assert.equal(formatLinoReadings(statements.map(reading)), data.source, 'the actual Links parser and existing Links writer restore the complete packet text');
  assert.equal(packet.consumerBytes, Buffer.byteLength(packet.consumerSource) + data.sourceBytes);
  assert.ok(packet.consumerBytes <= 4 * 1024 * 1024);assert.ok(packet.consumerSourceBytes < 8192);
  assert.match(packet.consumerSource, /std::fs::read_to_string\("carrier\.lino"\)/u);
  assert.match(packet.consumerSource, /for index in 0\.\.invocation_count/u);
  assert.match(packet.consumerSource, /oracle_source::check_original_link_raw_shape\(root_high,root_low,columns\.clone\(\)\)/u);
  assert.doesNotMatch(packet.consumerSource, /vec!\[/u);
  const larger = structuredClone(carrier);larger.blocks.push(Array.from({length: 4096}, (_, index) => index % carrier.words.length));
  const expanded = createOriginalLinkRawConsumerPacket(observations, larger, header);
  assert.equal(expanded.consumerSource, packet.consumerSource, 'complete data growth cannot duplicate compiled Rust expressions');
  assert.ok(expanded.consumerDataBytes > packet.consumerDataBytes);
  assert.deepEqual(decodeOriginalLinkRawCarrierData(expanded.additionalSources[0].source).blocks, larger.blocks);
  for (const invalid of [
    '', data.source.trimEnd().slice(0, -1), data.source + '(extra: 0)\n',
    data.source.replace('(words: 6 ', '(words: 06 '), data.source.replace('(words: 6 ', '(words: 9007199254740992 '),
    data.source.replace('7ff8000000000001', '7ff800000000000x'),
    data.source.replace('(schema: original-link-raw-carrier 1)', '(schema: original-link-raw-carrier 2)'),
    data.source.replace('block-1', 'block-0'), data.source.replace('(vectors:', '(foreign:'),
  ]) assert.throws(() => decodeOriginalLinkRawCarrierData(invalid), assert.AssertionError);
  const foreign = structuredClone(carrier);foreign.blocks[0][0] = 99;
  assert.throws(() => createOriginalLinkRawConsumerPacket(observations, foreign, header), /word indices must exist/u);
});

test('raw consumer data uses the existing complete packet preflight and detects edits around every injected phase', () => {
  const {carrier, observations, header} = rawConsumerDataFixture(), packet = createOriginalLinkRawConsumerPacket(observations, carrier, header);
  const source = 'pub fn check_original_link_raw_shape()->Vec<f64>{Vec::new()}\n', sha256 = value => createHash('sha256').update(value).digest('hex');
  for (const mutation of ['none', 'preflight', 'library', 'consumer', 'execute']) {
    const entry = {id: 'injected raw carrier transport only', kind: 'injected transport contract', source, sourceBytes: Buffer.byteLength(source), sourceSha256: sha256(source),
      consumerSource: packet.consumerSource, consumerSha256: sha256(packet.consumerSource), additionalSources: structuredClone(packet.additionalSources), observations: []};
    if (mutation === 'preflight') entry.additionalSources[0].source += 'changed';
    const plan = {entries: [entry]}, calls = [], environment = {CI: 'true', ORIGINAL_LINK_RAW_NATIVE: '1'};
    const run = createSourceNativeObservationRunner({nativeMode: env => env === environment, checkedPlan: candidate => candidate === plan,
      textBytes: value => Buffer.from(value), parseObservations: output => {assert.equal(output, 'injected transport only\n');return [];}, entryReceipt: () => ({}),
      temporaryPrefix: 'raw-data-contract-', failureLabel: 'injected raw data contract', compilerProfile: 'checkedOptimized'});
    const execute = () => run(plan, {environment, spawn(command, args, options) {
      const phase = args.includes('--crate-type=rlib') ? 'library' : args.includes('--extern') ? 'consumer' : 'execute';calls.push({phase, options});
      const file = join(options.cwd, 'carrier.lino');assert.equal(readFileSync(file, 'utf8'), packet.additionalSources[0].source);
      assert.equal(options.timeout, phase === 'execute' ? 10000 : 30000);assert.equal(options.maxBuffer, 1024 * 1024);
      if (phase !== 'execute') {
        assert.deepEqual(args.slice(1, 5), ['--error-format=short', '-Copt-level=2', '-Cdebug-assertions=on', '-Coverflow-checks=on']);
        writeFileSync(args[args.indexOf('-o') + 1], 'injected artifact only');
      }
      if (phase === mutation) writeFileSync(file, 'changed during injected phase');
      return {status: 0, signal: null, stdout: phase === 'execute' ? 'injected transport only\n' : '', stderr: ''};
    }});
    if (mutation === 'none') {
      const receipt = execute();assert.equal(receipt.processMode, 'injected process contract');
      assert.deepEqual(receipt.processCounts, {libraryCompilations: 1, consumerCompilations: 1, executions: 1});
      assert.deepEqual(receipt.observations[0].additionalSources, packet.additionalSources.map(({relativePath, sourceBytes, sourceSha256}) => ({relativePath, sourceBytes, sourceSha256})));
    } else {
      assert.throws(execute, error => {assert.ok(error instanceof AggregateError);assert.match(error.message, /byte count differs|additional source bytes changed/u);return true;});
      assert.equal(calls.length, mutation === 'preflight' ? 0 : ['library', 'consumer', 'execute'].indexOf(mutation) + 1);
    }
    assert.ok(calls.every(call => !existsSync(call.options.cwd)), 'all injected packet scratch is removed');
  }
});

test('raw planning phases finish exact source replay before allocating the observation matrix', () => {
  const source = 'pub fn example()->&\'static str{"whole 🌍 source"}\r\nfn main(){}\r\n';
  const prepared = Object.freeze({source, javascript: 'function example(){return "whole 🌍 source";}'});
  const receipt = Object.freeze({scope: 'injected lifecycle receipt only'}), result = Object.freeze({scope: 'injected matrix only'}), order = [];
  const actual = runOriginalLinkRawPlanningPhases({
    prepareSource() {order.push('prepare');return prepared;},
    replaySource(original) {
      assert.equal(original, source);assert.deepEqual(Buffer.from(original), Buffer.from(source));
      assert.deepEqual(order, ['prepare']);order.push('text replay', 'binary replay');return receipt;
    },
    buildMatrix(original, replayed) {
      assert.strictEqual(original, prepared);assert.strictEqual(replayed, receipt);
      assert.deepEqual(order, ['prepare', 'text replay', 'binary replay']);order.push('matrix');return result;
    },
  });
  assert.strictEqual(actual, result);assert.deepEqual(order, ['prepare', 'text replay', 'binary replay', 'matrix']);
});

test('raw planning phases preserve failure identity and never allocate a matrix before successful replay', () => {
  for (const failedPhase of ['prepare', 'text replay', 'binary replay', 'matrix']) {
    const failure = new Error(failedPhase), order = [];
    const record = phase => {order.push(phase);if (phase === failedPhase) throw failure;};
    assert.throws(() => runOriginalLinkRawPlanningPhases({
      prepareSource() {record('prepare');return Object.freeze({source: 'complete original bytes'});},
      replaySource() {record('text replay');record('binary replay');return Object.freeze({scope: 'injected replay only'});},
      buildMatrix() {record('matrix');},
    }), error => error === failure);
    assert.deepEqual(order, ['prepare', 'text replay', 'binary replay', 'matrix'].slice(0, ['prepare', 'text replay', 'binary replay', 'matrix'].indexOf(failedPhase) + 1));
  }
});

test('raw planning phases cannot authorize a fabricated matrix for native or injected execution', () => {
  const plan = runOriginalLinkRawPlanningPhases({
    prepareSource: () => Object.freeze({source: 'injected bytes'}),
    replaySource: () => Object.freeze({scope: 'injected replay only'}),
    buildMatrix: () => Object.freeze({entries: []}),
  });
  let processes = 0;
  assert.throws(() => runOriginalLinkRawNative(plan, {
    environment: {CI: 'true', ORIGINAL_LINK_RAW_NATIVE: '1'},
    spawn() {processes++;throw new Error('a fabricated lifecycle result cannot launch a process');},
  }), /exact checked raw plan/u);
  assert.equal(processes, 0);
});

test('the additive schema closure retains exact original source bytes and every historical producer binding', () => {
  const {receipt, bytes, historicalSources} = schemaClosureFixture();
  const closure = verifyOriginalLinkRawSchemaSourceClosure(receipt, bytes, historicalSources);
  assert.equal(closure.producerCommit, '9123dd5d19fc26c95a9d31eff47dd9655c33da53');
  assert.equal(closure.baselineSha256, '377101200a12b4ada15e0a34a583900e8b6d62adb1fb685f9b2d6f161cacafdc');
  assert.equal(receipt.baseline.bytes, Buffer.byteLength(receipt.baseline.source));
  const sources = verifyOriginalLinkRawNativeModel();assert.equal(sources.length, 47);
  const row = sources.find(row => row.file === '../../js/src/data/typed-program-links-schema.lino');
  assert.equal(row.producerSha256, closure.baselineSha256);assert.equal(row.sha256, closure.sha256);
  assert.deepEqual(row.schemaSourceClosure, closure);
  assert.equal(receipt.generation.definitionCount, 106);
  assert.equal(receipt.generation.completeRustBytes, 351746);
  assert.match(receipt.scope, /no native execution/u);
});

test('coherent new schema hashes cannot excuse changed, removed, reordered or unexpected old data', () => {
  for (const mutate of [
    schema => {schema.fields.k = 'counterfeit construct kind';},
    schema => {delete schema.root;},
    schema => {delete schema.fields.k;},
    schema => {schema.fields = Object.fromEntries(Object.entries(schema.fields).reverse());},
    schema => {const entries = Object.entries(schema);[entries[0], entries[1]] = [entries[1], entries[0]];return Object.fromEntries(entries);},
    schema => {schema.unexpectedUnusedPlan = {};},
    schema => {delete schema.propositionOperands;},
  ]) {
    const {receipt, bytes, historicalSources} = schemaClosureFixture();
    const schema = readStructuredDataLino(bytes.toString('utf8')), changed = mutate(schema) ?? schema;
    const counterfeit = Buffer.from(structuredDataToLino(changed));
    receipt.current.bytes = counterfeit.length;
    receipt.current.sha256 = createHash('sha256').update(counterfeit).digest('hex');
    assert.throws(() => verifyOriginalLinkRawSchemaSourceClosure(receipt, counterfeit, historicalSources), assert.AssertionError);
  }
});

test('schema closure receipts cannot forge source hashes, byte counts, producer bindings or generation results', () => {
  for (const mutate of [
    receipt => {receipt.baseline.sha256 = '0'.repeat(64);},
    receipt => {receipt.current.sha256 = '0'.repeat(64);},
    receipt => {receipt.baseline.source += '\n';},
    receipt => {receipt.baseline.bytes--;},
    receipt => {receipt.current.bytes--;},
    receipt => {receipt.producerCommit = '0'.repeat(40);},
    receipt => {receipt.consumedSchemaProperties.reverse();},
    receipt => {receipt.consumedSchemaProperties.pop();},
    receipt => {receipt.addedUnusedSchemaProperties.push('unexpected');},
    receipt => {receipt.historicalSources['policy.mjs'] = '0'.repeat(64);},
    receipt => {receipt.historicalSources = Object.fromEntries(Object.entries(receipt.historicalSources).reverse());},
    receipt => {receipt.generation.exitCode = 1;},
    receipt => {receipt.generation.definitionCount = 105;},
    receipt => {receipt.generation.completeRustSha256 = '0'.repeat(64);},
  ]) {
    const {receipt, bytes, historicalSources} = schemaClosureFixture();mutate(receipt);
    assert.throws(() => verifyOriginalLinkRawSchemaSourceClosure(receipt, bytes, historicalSources), assert.AssertionError);
  }
});

test('heavy raw planning refuses before fixture imports outside CI', async () => {
  for (const environment of [{}, {CI: 'false'}]) {
    assert.throws(() => requireOriginalLinkRawPlanning(environment), /require CI=true/u);
    await assert.rejects(planOriginalLinkRawNative(environment), /require CI=true/u);
  }
  assert.doesNotThrow(() => requireOriginalLinkRawPlanning({CI: 'true', ORIGINAL_LINK_RAW_NATIVE: '0'}));
  assert.equal(originalLinkRawNativeMode({CI: 'true', ORIGINAL_LINK_RAW_NATIVE: '0'}), false);
  assert.equal(originalLinkRawNativeMode({CI: 'true', ORIGINAL_LINK_RAW_NATIVE: '1'}), true);
  assert.throws(() => originalLinkRawNativeMode({ORIGINAL_LINK_RAW_NATIVE: '1'}), /require CI=true/u);
});

if (process.env.CI === 'true') {
  const plan = await planOriginalLinkRawNative(), entry = plan.entries[0];
  const environment = {CI: 'true', ORIGINAL_LINK_RAW_NATIVE: '1', GITHUB_SHA: 'injected process contracts only'};
  const stream = entry.observations.map((row, index) => [index, row.resultBits.length, ...entry.carrier.vectors[entry.carrier.invocations[index].result].map(value => value.toString(16))].join(':')).join('\n') + '\n';
  function fakeProcess(change) {
    const calls = [];
    const spawn = (command, args, options) => {
      const phase = args.includes('--crate-type=rlib') ? 'library' : args.includes('--extern') ? 'consumer' : 'execute'; calls.push({command, args, options, phase});
      const replacement = change?.({command, args, options, phase}); if (replacement) return replacement;
      if (phase !== 'execute') writeFileSync(args[args.indexOf('-o') + 1], 'injected contract artifact; never native evidence');
      return {status: 0, signal: null, stdout: phase === 'execute' ? stream : '', stderr: ''};
    };
    return {spawn, calls};
  }
  const run = fake => runOriginalLinkRawNative(plan, {environment, spawn: fake.spawn});
  const fails = (operation, pattern) => assert.throws(operation, error => { assert.ok(error instanceof AggregateError); assert.match(error.message, pattern); return true; });

  test('all original archive premises and complete numeric columns survive exact-bit carrier reuse', () => {
    assert.equal(plan.originalCount, 1); assert.deepEqual(entry.inventoryCounts, {fixtures: 78, formats: 195, numericPremises: 190, serializationRefusals: 5});
    assert.equal(entry.fixtureInventory.length, 78); assert.equal(new Set(entry.fixtureInventory).size, 78);
    assert.equal(entry.genuineFormatPremiseCount, 190); assert.equal(entry.serializationRefusals.length, 5);
    assert.equal(entry.premises.filter(row => row.format).length, entry.genuineFormatPremiseCount);
    assert.equal(entry.premises.length, 211);
    assert.deepEqual(entry.observations.flatMap(row => row.premiseIndices).sort((left, right) => left - right), entry.premises.map((_, index) => index), 'all genuine and synthetic premises survive exact-bit deduplication exactly once');
    assert.ok(entry.serializationRefusals.length > 0); for (const row of entry.serializationRefusals) { assert.equal(row.format, 'binary'); assert.match(row.scope, /no fabricated native transport/u); }
    assert.ok(entry.premises.some(row => row.kind === 'accepted')); assert.ok(entry.premises.some(row => row.kind === 'refused')); assert.ok(entry.premises.some(row => row.kind === 'unselected'));
    for (const row of entry.observations) assert.equal(row.inputBits[2].length, 16);
    assert.equal(entry.hostChecks.length, 32);
    assert.equal(plan.emissionProfile.profile, 'current');
    assert.deepEqual(entry.emissionProfile, plan.emissionProfile);
    assert.equal(entry.sourceSha256, ORIGINAL_LINK_RAW_EMISSION_BASELINE.currentSha256);
    assert.equal(entry.emissionProfile.historicalSourceSha256, ORIGINAL_LINK_RAW_EMISSION_BASELINE.historicalSha256);
    assert.equal(entry.emissionProfile.currentSourceSha256, ORIGINAL_LINK_RAW_EMISSION_BASELINE.currentSha256);
    for (const check of entry.hostChecks) assert.match(check.scope, /JavaScript host check only/u);
    assert.ok(entry.sizes.consumerBytes <= 4 * 1024 * 1024); assert.ok(entry.sizes.protocolBytes <= 1024 * 1024);
    assert.equal(entry.sizes.consumerBytes, entry.sizes.consumerSourceBytes + entry.sizes.consumerDataBytes);
    assert.equal(entry.additionalSources.length, 1);assert.equal(entry.additionalSources[0].relativePath, 'carrier.lino');
    const restored = decodeOriginalLinkRawCarrierData(entry.additionalSources[0].source);
    assert.deepEqual(restored, {...entry.carrier, invocations: entry.carrier.invocations.map((row, index) => ({rootBits: entry.observations[index].inputBits.slice(0, 2), result: row.result, columns: row.columns}))});
    assert.match(entry.observationScope, /No native LinkNetwork capture/u);
    assert.equal(entry.independentJavaScriptControls.testFile, 'experiments/original-link-complete-table-projection/raw-projection.test.mjs');
    assert.equal(entry.independentJavaScriptControls.controls.length, 7);
    assert.match(entry.independentJavaScriptControls.scope, /55 groups independently/u);
    assert.match(entry.independentJavaScriptControls.scope, /not native observations/u);
    assert.throws(() => { entry.consumerSource += 'changed'; }, TypeError);
  });
  test('every frozen source is authenticated and source substitution fails before any process', () => {
    assert.equal(plan.modelSources.length, 47);
    assert.deepEqual(verifyOriginalLinkRawNativeModel(), plan.modelSources);
    let reads = 0;
    assert.throws(() => verifyOriginalLinkRawNativeModel(file => { reads++; return Buffer.concat([readFileSync(file), Buffer.from('changed')]); }), /frozen raw component changed/u);
    assert.equal(reads, 1);
  });
  test('CI flag zero still permits injected contracts without producing native evidence', () => {
    const fake = fakeProcess(), receipt = run(fake);
    assert.equal(receipt.processMode, 'injected process contract'); assert.deepEqual(receipt.processCounts, {libraryCompilations: 1, consumerCompilations: 1, executions: 1});
    assert.equal(receipt.observations[0].observations.length, entry.observations.length);
    for (const call of fake.calls) {
      assert.equal(call.options.shell, false); assert.equal(existsSync(call.options.cwd), false);
      assert.equal(call.options.maxBuffer, 1024 * 1024); assert.equal(call.options.timeout, call.phase === 'execute' ? 10000 : 30000);
      if(call.phase!=='execute')assert.deepEqual(call.args.slice(1,5),['--error-format=short','-Copt-level=2','-Cdebug-assertions=on','-Coverflow-checks=on']);
      assert.equal(call.options.env.CI, 'true'); assert.equal(call.options.env.ORIGINAL_LINK_RAW_NATIVE, '1'); assert.equal(Object.isFrozen(call.options.env), true);
    }
    assert.deepEqual(receipt.observations[0].inventoryCounts, entry.inventoryCounts);
    assert.deepEqual(receipt.observations[0].fixtureInventory, entry.fixtureInventory);
    assert.deepEqual(receipt.observations[0].independentJavaScriptControls, entry.independentJavaScriptControls);
    assert.deepEqual(receipt.emissionProfile, plan.emissionProfile);
    assert.deepEqual(receipt.observations[0].emissionProfile, entry.emissionProfile);
  });
  test('omitted, reordered, extra, noncanonical or corrupt output blocks refuse', () => {
    const lines = stream.trimEnd().split('\n');
    for (const stdout of [stream.slice(0, -1), stream + '0:0\n', lines.slice(1).join('\n') + '\n', [lines[1], lines[0], ...lines.slice(2)].join('\n') + '\n', stream.replace(':', ':z'), stream.replace(/:[0-9a-f]+(?=\n)/u, ':ffffffff')]) {
      const fake = fakeProcess(call => call.phase === 'execute' ? {status: 0, signal: null, stdout, stderr: ''} : null); fails(() => run(fake), /observations failed/u);
    }
  });
  test('compiler, consumer, execution and missing-artifact failures cannot become observations', () => {
    for (const phase of ['library', 'consumer', 'execute']) for (const failure of [{status: 1, stderr: 'failure', stdout: ''}, {status: null, signal: 'SIGTERM', stdout: ''}, {error: {code: 'ETIMEDOUT'}, stdout: ''}]) {
      const fake = fakeProcess(call => call.phase === phase ? failure : null); fails(() => run(fake), /exit 1|SIGTERM|timed out/u);
    }
    for (const phase of ['library', 'consumer']) { const fake = fakeProcess(call => call.phase === phase ? {status: 0, signal: null, stdout: '', stderr: ''} : null); fails(() => run(fake), /compiler did not create/u); }
  });
  test('source and consumer bytes remain authenticated around injected processes', () => {
    for (const name of ['original.rs', 'consumer.rs', 'carrier.lino']) for (const phase of ['library', 'consumer', 'execute']) {
      const fake = fakeProcess(call => { if (call.phase === phase) { const file = join(call.options.cwd, name); writeFileSync(file, readFileSync(file, 'utf8') + '\n// changed\n'); } }); fails(() => run(fake), /bytes changed|consumer changed/u);
    }
  });
  test('a detached plan cannot launch even injected processes', () => {
    const fake = fakeProcess(); assert.throws(() => runOriginalLinkRawNative({...plan}, {environment, spawn: fake.spawn}), /exact checked raw plan/u); assert.equal(fake.calls.length, 0);
  });
  // Historical title: 'CI executes the unchanged complete raw artifact and persists only real observations'.
  // That byte expectation belongs to the historical producer profile; the same
  // complete fixture/format assertions now execute the authenticated current profile.
  if (originalLinkRawNativeMode(process.env)) test('CI executes the selected complete raw emission profile and persists only real observations', async () => {
    const receipt = runOriginalLinkRawNative(plan); assert.equal(receipt.processMode, 'native subprocesses');
    assert.equal(receipt.emissionProfile.profile, 'current');
    assert.equal(receipt.emissionProfile.sourceSha256, ORIGINAL_LINK_RAW_EMISSION_BASELINE.currentSha256);
    const [{writeProjectData}, {fileURLToPath}] = await Promise.all([import('../../js/scripts/project-data.mjs'), import('node:url')]);
    const file = fileURLToPath(new URL('../../issue-195-artifacts/original-link-raw-native-observations.lino', import.meta.url)); mkdirSync(dirname(file), {recursive: true}); writeProjectData(file, receipt);
  });
}
