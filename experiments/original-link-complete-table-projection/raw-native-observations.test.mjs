// Heavy contracts deliberately require CI, including when the native flag is 0.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, mkdirSync, readFileSync, writeFileSync} from 'node:fs';
import {dirname, join} from 'node:path';
import test from 'node:test';
import {requireOriginalLinkRawPlanning, originalLinkRawNativeMode, planOriginalLinkRawNative, runOriginalLinkRawNative, runOriginalLinkRawPlanningPhases, verifyOriginalLinkRawNativeModel} from './raw-native-observations.mjs';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {readOriginalLinkRawHistoricalSources, verifyOriginalLinkRawSchemaSourceClosure} from './raw-schema-source-closure.mjs';

const schemaClosureFixture = () => ({
  receipt: readStructuredDataLino(readFileSync(new URL('./data/raw-schema-source-closure.lino', import.meta.url), 'utf8')),
  bytes: readFileSync(new URL('../../js/src/data/typed-program-links-schema.lino', import.meta.url)),
  historicalSources: readOriginalLinkRawHistoricalSources(readFileSync(new URL('./raw-native-observations.mjs', import.meta.url), 'utf8')),
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
    for (const check of entry.hostChecks) assert.match(check.scope, /JavaScript host check only/u);
    assert.ok(entry.sizes.consumerBytes <= 4 * 1024 * 1024); assert.ok(entry.sizes.protocolBytes <= 1024 * 1024);
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
    for (const name of ['original.rs', 'consumer.rs']) for (const phase of ['library', 'consumer', 'execute']) {
      const fake = fakeProcess(call => { if (call.phase === phase) { const file = join(call.options.cwd, name); writeFileSync(file, readFileSync(file, 'utf8') + '\n// changed\n'); } }); fails(() => run(fake), /bytes changed|consumer changed/u);
    }
  });
  test('a detached plan cannot launch even injected processes', () => {
    const fake = fakeProcess(); assert.throws(() => runOriginalLinkRawNative({...plan}, {environment, spawn: fake.spawn}), /exact checked raw plan/u); assert.equal(fake.calls.length, 0);
  });
  if (originalLinkRawNativeMode(process.env)) test('CI executes the unchanged complete raw artifact and persists only real observations', async () => {
    const receipt = runOriginalLinkRawNative(plan); assert.equal(receipt.processMode, 'native subprocesses');
    const [{writeProjectData}, {fileURLToPath}] = await Promise.all([import('../../js/scripts/project-data.mjs'), import('node:url')]);
    const file = fileURLToPath(new URL('../../issue-195-artifacts/original-link-raw-native-observations.lino', import.meta.url)); mkdirSync(dirname(file), {recursive: true}); writeProjectData(file, receipt);
  });
}
