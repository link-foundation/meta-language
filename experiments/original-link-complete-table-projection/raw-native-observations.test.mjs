// Heavy contracts deliberately require CI, including when the native flag is 0.
import assert from 'node:assert/strict';
import {existsSync, mkdirSync, readFileSync, writeFileSync} from 'node:fs';
import {dirname, join} from 'node:path';
import test from 'node:test';
import {requireOriginalLinkRawPlanning, originalLinkRawNativeMode, planOriginalLinkRawNative, runOriginalLinkRawNative, verifyOriginalLinkRawNativeModel} from './raw-native-observations.mjs';

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
