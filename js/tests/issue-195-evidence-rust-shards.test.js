import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {appendFileSync, cpSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import path from 'node:path';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {exactHarnessArguments, executePhysicalHarness, isRootHarness, matchesLogicalFilters, physicalShardCount,
  readHarnessInventory, readHarnessOutcomes, readPhysicalShardPolicy, readShardAdmission, renderCargoRunnerConfiguration, selectPhysicalTests,
  verifyHarnessExecution, verifyPhysicalShardDirectory, verifyPhysicalShardUnion} from '../scripts/rust-test-shards.mjs';
import {readStructuredDataLino, structuredDataToLino} from '../src/structured-data-links.js';

const policy = readPhysicalShardPolicy();
// Keep the original scheduling inputs alongside their platform expansions.
const originalPolicy = structuredClone(policy);
for (const row of originalPolicy.suites) delete row.platformShards;
delete originalPolicy.freshMergeExpansion;
const sha = value => createHash('sha256').update(value).digest('hex');
const commit = 'a'.repeat(40), tree = 'b'.repeat(40), base = 'c'.repeat(40);
const expected = {commit, run: '123', attempt: '2', profiles: ['test', 'coverage', 'fresh-merge']};
const inventory = readHarnessInventory(['grammar_case', 'inference_case', 'issue_195_generative_case', 'translation_case', 'ordinary_case']
  .flatMap(prefix => Array.from({length: 8}, (_, index) => `${prefix}_${index}: test`)).join('\n'));
const listing = rows => rows.map(row => `${row.name}: ${row.kind}\n`).join('');

function fakeNative(all = inventory, failure = null, calls = []) {
  return async (binary, arguments_, listingMode) => {
    calls.push({binary, arguments: arguments_, listing: listingMode});
    if (listingMode) {
      const filters = listingMode === 'all' ? [] : arguments_.slice(0, -3);
      return {stdout: listing(all.filter(row => matchesLogicalFilters(row.name, filters))), stderr: '', exitCode: 0, signal: null};
    }
    assert.ok(arguments_.includes('--exact'));
    const selected = all.filter(row => arguments_.includes(row.name));
    return {stdout: selected.map(row => `test ${row.name} ... ${row.name === failure ? 'FAILED' : 'ok'}\n`).join('')
      + (selected.some(row => row.name === failure) ? '\nfailures:\nfull native panic details\n' : ''),
    stderr: 'unaltered native stderr\n', exitCode: selected.some(row => row.name === failure) ? 101 : 0, signal: null};
  };
}

async function groups(configuration = policy) {
  const result = [];
  for (const profile of expected.profiles) for (const platform of profile === 'test' ? configuration.platforms : ['ubuntu-latest']) {
    const suites = profile === 'fresh-merge' ? [{name: 'all', filters: []}] : configuration.suites;
    for (const row of suites) for (let shard = 1, count = physicalShardCount(configuration, profile, row.name, platform); shard <= count; shard++) {
      const identity = {commit, run: expected.run, attempt: expected.attempt, profile, platform, suite: row.name, tree};
      if (profile === 'fresh-merge') Object.assign(identity, {base, head: commit});
      const execution = await executePhysicalHarness({binary: '/project/rust/target/debug/deps/unit-abcdef01',
        arguments: row.filters, identity, shard, count, invoke: fakeNative()});
      result.push({seal: {identity, shard, count, result: 'success'}, executions: [execution], expectedBinaries: ['unit']});
    }
  }
  return result;
}

test('the physical policy retains all five original disjoint filters and unchanged small suites', () => {
  assert.deepEqual(policy.suites.map(row => [row.name, row.shards]), [['grammar', 4], ['inference', 1], ['generative', 1], ['translation', 4], ['remaining', 4]]);
  for (const row of inventory) assert.equal(policy.suites.filter(suite => matchesLogicalFilters(row.name, suite.filters)).length, 1, row.name);
  for (const count of [1, 4, 8]) {
    const selected = Array.from({length: count}, (_, index) => selectPhysicalTests(inventory, index + 1, count)).flat();
    assert.deepEqual(selected.map(row => row.name).sort(), inventory.map(row => row.name));
  }
  for (const shard of [0, 5, NaN, 1.5]) assert.throws(() => selectPhysicalTests([], shard, 4));
});

test('actual native inventories supply exact execution filters and preserve native failures and full diagnostics', async () => {
  const calls = [], filters = policy.suites[0].filters;
  const receipt = await executePhysicalHarness({binary: '/project/unit-abcdef01', arguments: filters,
    identity: {}, shard: 1, count: 4, invoke: fakeNative(inventory, null, calls)});
  assert.equal(calls.length, 3);
  assert.deepEqual(calls[0].arguments, ['grammar_', '--list', '--format', 'terse']);
  assert.deepEqual(calls[1].arguments, ['--list', '--format', 'terse']);
  assert.deepEqual(calls[2].arguments, exactHarnessArguments(filters, receipt.selected));
  verifyHarnessExecution(receipt);
  const failure = receipt.selected[0]?.name ?? inventory[0].name;
  const failed = await executePhysicalHarness({binary: '/project/unit-abcdef01', arguments: [], identity: {},
    shard: 1, count: 1, invoke: fakeNative(inventory, failure)});
  assert.equal(failed.exitCode, 101);
  assert.equal(failed.stdoutDigest, sha(inventory.map(row => `test ${row.name} ... ${row.name === failure ? 'FAILED' : 'ok'}\n`).join('') + '\nfailures:\nfull native panic details\n'));
  assert.equal(failed.stderrDigest, sha('unaltered native stderr\n'));
  assert.throws(() => verifyHarnessExecution(failed), /native test execution failed/u);

  // Exact successful libtest output retained from macOS grammar CI. The
  // expected-panic annotation is display metadata, not an inventory name.
  const name = 'issue_195_native_grammar_rows::recovery_projection_tests::accepted_reconstruction_remains_strict_about_missing_leaves';
  const all = readHarnessInventory(`${name}: test\n`);
  const stdout = `test ${name} - should panic ... ok\n`;
  const annotated = await executePhysicalHarness({binary: '/project/unit-abcdef01', arguments: [], identity: {}, shard: 1, count: 1,
    invoke: async (binary, arguments_, listingMode) => listingMode ? fakeNative(all)(binary, arguments_, listingMode)
      : {stdout, stderr: 'unaltered native stderr\n', exitCode: 0, signal: null}});
  assert.deepEqual(annotated.outcomes, [{name, outcome: 'ok'}]);
  assert.equal(annotated.stdoutDigest, sha(stdout), 'normalization preserves complete original native output authentication');
  verifyHarnessExecution(annotated);
  for (const output of [stdout + stdout, stdout + `test ${name} ... ok\n`, stdout.replace(name, 'unselected'), stdout.replace(' - should panic', ' - should panic - should panic')]) {
    const changed = {...annotated, outcomes: readHarnessOutcomes(output, annotated.selected)};
    assert.throws(() => verifyHarnessExecution(changed), /missing or duplicate native test outcomes/u);
  }
  assert.throws(() => verifyHarnessExecution({...annotated, exitCode: 101,
    outcomes: readHarnessOutcomes(stdout.replace('... ok', '... FAILED'), annotated.selected)}), /native test execution failed/u);
});

test('empty selections execute zero tests and exact names cannot admit longer neighboring names', async () => {
  const all = readHarnessInventory('short: test\nshort_long: test\n');
  const arguments_ = exactHarnessArguments([], [all[0]]);
  assert.deepEqual(arguments_, ['--exact', 'short']);
  const empty = exactHarnessArguments([], []);
  assert.deepEqual(empty, ['--exact', '__meta_language_empty_physical_partition__']);
  const receipt = await executePhysicalHarness({binary: '/project/unit-abcdef01', arguments: ['absent'], identity: {},
    shard: 1, count: 4, invoke: fakeNative(all)});
  assert.deepEqual(receipt.selected, []);assert.deepEqual(receipt.outcomes, []);verifyHarnessExecution(receipt);
  assert.throws(() => readHarnessInventory('duplicate: test\nduplicate: test\n'), /duplicate/u);
  assert.throws(() => readHarnessInventory('native crash diagnostics\n'), /invalid/u);
  const selected = [{name: 'owned', kind: 'test'}];
  for (const outcome of ['ok', 'FAILED', 'ignored']) {
    assert.deepEqual(readHarnessOutcomes(`test owned - should panic ... ${outcome}\r\n`, selected), [{name: 'owned', outcome}]);
  }
  for (const name of ['unselected - should panic', 'owned - should panic - should panic', 'owned - should panicked']) {
    assert.deepEqual(readHarnessOutcomes(`test ${name} ... ok\n`, selected), [{name, outcome: 'ok'}]);
  }
  const literal = [{name: 'owned - should panic', kind: 'test'}];
  assert.deepEqual(readHarnessOutcomes('test owned - should panic ... ok\n', literal), [{name: literal[0].name, outcome: 'ok'}]);
  assert.deepEqual(readHarnessOutcomes('test owned - should panic - should panic ... ok\n', literal), [{name: literal[0].name, outcome: 'ok'}]);
  assert.deepEqual(readHarnessOutcomes('test owned - should panic ... ok\n', [...selected, ...literal]), [{name: literal[0].name, outcome: 'ok'}]);
  assert.deepEqual(readHarnessOutcomes('test owned - should panic ... ok\n'), [{name: literal[0].name, outcome: 'ok'}]);
});

test('the full physical union authenticates every platform, source identity, test outcome and native inventory', async () => {
  const original = await groups(originalPolicy);
  assert.deepEqual(verifyPhysicalShardUnion(original, originalPolicy, expected), {cells: 64, inventories: 21});
  const valid = await groups();
  assert.deepEqual(verifyPhysicalShardUnion(valid, policy, expected), {cells: 96, inventories: 21});
  const mutations = [
    rows => rows.pop(),
    rows => rows.push(structuredClone(rows[0])),
    rows => {rows[0].seal.identity.commit = 'd'.repeat(40);},
    rows => {rows[0].seal.identity.attempt = '1';},
    rows => {rows[0].seal.identity.tree = 'd'.repeat(40);},
    rows => {rows[0].seal.result = 'failure';},
    rows => {rows[0].executions = [];},
    rows => {rows[0].executions.push(structuredClone(rows[0].executions[0]));},
    rows => {rows[0].executions[0].outcomes.push({name: 'invented', outcome: 'ok'});},
    rows => {rows[0].executions[0].inventory.pop();},
    rows => {rows[0].executions[0].allInventory.pop();},
    rows => {rows[0].executions[0].arguments = [];},
    rows => {rows.at(-1).seal.identity.base = 'd'.repeat(40);rows.at(-1).executions[0].identity.base = 'd'.repeat(40);},
  ];
  for (const mutate of mutations) {
    const rows = structuredClone(valid);mutate(rows);
    assert.throws(() => verifyPhysicalShardUnion(rows, policy, expected));
  }
  const testOnly = valid.filter(row => row.seal.identity.profile === 'test');
  assert.equal(verifyPhysicalShardUnion(original.filter(row => row.seal.identity.profile === 'test'), originalPolicy, {...expected, profiles: ['test']}).cells, 42);
  assert.equal(verifyPhysicalShardUnion(testOnly, policy, {...expected, profiles: ['test']}).cells, 58);
  assert.throws(() => verifyPhysicalShardUnion(testOnly, policy, expected), /missing/u);
});

test('Cargo runner recognition includes Windows and instrumented host test paths while preserving CLI consumers', () => {
  assert.equal(isRootHarness('/project/rust/target/debug/deps/unit-abcdef01', '/project'), true);
  assert.equal(isRootHarness('/project/rust/target/llvm-cov-target/debug/deps/unit-abcdef01', '/project'), true);
  assert.equal(isRootHarness('C:\\project\\rust\\target\\debug\\deps\\unit-abcdef01.exe', 'C:\\project'), true);
  assert.equal(isRootHarness('/project/rust/target/debug/meta-language', '/project'), false);
  assert.equal(isRootHarness('/tmp/consumer/target/debug/deps/unit-abcdef01', '/project'), false);
});

test('Cargo environment configuration keeps literal argument tokens and Windows paths without JSON or shell quoting', () => {
  for (const [host, executable, runner] of [
    ['x86_64-unknown-linux-gnu', '/opt/hostedtoolcache/node/24/x64/bin/node', '/home/runner/work/meta-language/meta-language/js/scripts/rust-test-shards.mjs'],
    ['aarch64-apple-darwin', '/Users/runner/hostedtoolcache/node/24/arm64/bin/node', '/Users/runner/work/meta-language/meta-language/js/scripts/rust-test-shards.mjs'],
    ['x86_64-pc-windows-msvc', 'C:\\hostedtoolcache\\windows\\node\\24\\x64\\node.exe', 'D:\\a\\meta-language\\meta-language\\js\\scripts\\rust-test-shards.mjs'],
  ]) {
    const configuration = renderCargoRunnerConfiguration(host, executable, runner);
    assert.deepEqual(configuration.value.split(/\s+/u), [executable, runner, '--runner']);
    assert.equal(configuration.value, `${executable} ${runner} --runner`);
    assert.equal(configuration.environmentLine, `CARGO_TARGET_${host.toUpperCase().replaceAll('-', '_')}_RUNNER=${configuration.value}\n`);
    assert.equal(configuration.shellLine, `export ${configuration.name}='${configuration.value}'`);
    assert.ok(!configuration.value.includes('"'));
  }
  for (const invalid of ['', '/some path/node', 'C:\\Program Files\\nodejs\\node.exe', '/project\tname/runner', '/project\nname/runner', '/project\u0085name/runner', '/project\u00a0name/runner', '/project\0name/runner', '"/quoted/node"']) {
    assert.throws(() => renderCargoRunnerConfiguration('x86_64-pc-windows-msvc', invalid, '/runner.mjs'), /Cargo runner environment paths/u);
    assert.throws(() => renderCargoRunnerConfiguration('x86_64-pc-windows-msvc', '/node', invalid), /Cargo runner environment paths/u);
  }
  assert.throws(() => renderCargoRunnerConfiguration('invalid\nhost', '/node', '/runner.mjs'), /invalid Rust host/u);
});

test('retained complete stdout, listings, configuration and receipt seals reject filesystem tampering', async () => {
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-rust-shard-control-'));
  try {
    const rows = (await groups()).filter(row => row.seal.identity.profile === 'test');
    const policyDigest = sha(readFileSync(new URL('../../parity/rust-test-physical-shards.lino', import.meta.url)));
    for (const [index, row] of rows.entries()) {
      const cell = path.join(directory, String(index));mkdirSync(cell);
      const execution = row.executions[0];
      const files = {'identity.lino': {identity: row.seal.identity, shard: row.seal.shard, count: row.seal.count, policyDigest, expectedBinaries: ['unit']}, 'execution.lino': execution};
      const receipts = [];
      for (const [file, value] of Object.entries(files)) {
        const source = structuredDataToLino(value);writeFileSync(path.join(cell, file), source);receipts.push({file, sha256: sha(source)});
      }
      writeFileSync(path.join(cell, 'seal.lino'), structuredDataToLino({...row.seal, receipts}));
      writeFileSync(path.join(cell, 'execution.stdout.log'), execution.outcomes.map(outcome => `test ${outcome.name} ... ${outcome.outcome}\n`).join(''));
      writeFileSync(path.join(cell, 'execution.stderr.log'), 'unaltered native stderr\n');
      writeFileSync(path.join(cell, 'execution.inventory.log'), listing(execution.inventory));
      writeFileSync(path.join(cell, 'execution.all-inventory.log'), listing(execution.allInventory));
    }
    const options = {...expected, profiles: ['test']};
    assert.deepEqual(verifyPhysicalShardDirectory(directory, options), {cells: 58, inventories: 15});
    // The artifact reader must apply the same inventory-bound normalization as
    // the live producer without modifying or discarding authenticated stdout.
    const annotatedIndex = rows.findIndex(row => row.executions[0].outcomes.length > 0);
    assert.ok(annotatedIndex >= 0);
    const annotatedCell = path.join(directory, String(annotatedIndex));
    const originals = ['execution.stdout.log', 'execution.lino', 'seal.lino'].map(file => [file, readFileSync(path.join(annotatedCell, file))]);
    try {
      const execution = readStructuredDataLino(originals[1][1].toString());
      const name = execution.outcomes[0].name;
      const stdout = originals[0][1].toString().replace(`test ${name} ...`, `test ${name} - should panic ...`);
      assert.notEqual(stdout, originals[0][1].toString());
      execution.stdoutDigest = sha(stdout);
      const source = structuredDataToLino(execution);
      const seal = readStructuredDataLino(originals[2][1].toString());
      seal.receipts.find(row => row.file === 'execution.lino').sha256 = sha(source);
      writeFileSync(path.join(annotatedCell, 'execution.stdout.log'), stdout);
      writeFileSync(path.join(annotatedCell, 'execution.lino'), source);
      writeFileSync(path.join(annotatedCell, 'seal.lino'), structuredDataToLino(seal));
      assert.deepEqual(verifyPhysicalShardDirectory(directory, options), {cells: 58, inventories: 15});
    } finally {
      for (const [file, bytes] of originals) writeFileSync(path.join(annotatedCell, file), bytes);
    }
    const stdout = path.join(directory, '0/execution.stdout.log'), original = readFileSync(stdout);
    appendFileSync(stdout, 'discarded failure details');
    assert.throws(() => verifyPhysicalShardDirectory(directory, options), /complete native output/u);
    writeFileSync(stdout, original);
    cpSync(path.join(directory, '0'), path.join(directory, 'duplicate'), {recursive: true});
    assert.throws(() => verifyPhysicalShardDirectory(directory, options), /duplicate physical cell/u);
    rmSync(path.join(directory, 'duplicate'), {recursive: true});
    appendFileSync(path.join(directory, '0/execution.lino'), '\n');
    assert.throws(() => verifyPhysicalShardDirectory(directory, options), /changed execution receipt/u);
  } finally {rmSync(directory, {recursive: true});}
});

test('exact-attempt admission requires every physical job and retains legitimate original conditional scopes', () => {
  const originalJobs = [];
  for (const platform of originalPolicy.platforms) for (const row of originalPolicy.suites) for (let shard = 1; shard <= row.shards; shard++) originalJobs.push({name: `Rust / Test (${platform}, ${row.name})`, status: 'completed', conclusion: 'success'});
  assert.deepEqual(readShardAdmission(originalJobs, originalPolicy), ['test']);
  const jobs = [];
  for (const platform of policy.platforms) for (const row of policy.suites) for (let shard = 1; shard <= physicalShardCount(policy, 'test', row.name, platform); shard++) jobs.push({name: `Rust / Test (${platform}, ${row.name})`, status: 'completed', conclusion: 'success'});
  assert.deepEqual(readShardAdmission(jobs, policy), ['test']);
  assert.deepEqual(readShardAdmission([{name: 'Rust / Fresh Merge Simulation', status: 'completed', conclusion: 'skipped'}], policy), []);
  assert.throws(() => readShardAdmission(jobs.slice(1), policy), /scheduled physical/u);
  const changed = structuredClone(jobs);changed[0].name = changed.at(-1).name;
  assert.throws(() => readShardAdmission(changed, policy), /logical scopes/u);
  const originalFresh = Array.from({length: policy.freshMergeShards}, () => ({name: 'Rust / Fresh Merge Simulation', status: 'completed', conclusion: 'success'}));
  assert.deepEqual(readShardAdmission(originalFresh, originalPolicy), ['fresh-merge']);
  const fresh = Array.from({length: physicalShardCount(policy, 'fresh-merge', 'all')}, () => ({name: 'Rust / Fresh Merge Simulation', status: 'completed', conclusion: 'success'}));
  assert.deepEqual(readShardAdmission(fresh, policy), ['fresh-merge']);
  const indexed = fresh.map((job, index) => ({...job, name: `${job.name} (${index + 1})`}));
  assert.deepEqual(readShardAdmission(indexed, policy), ['fresh-merge']);
  for (const mutate of [
    rows => {rows[0].name = rows[1].name;},
    rows => {rows[0].name = 'Rust / Fresh Merge Simulation (9)';},
    rows => {rows[0].name = 'Rust / Fresh Merge Simulation (17)';},
    rows => {rows[0].name = 'Rust / Fresh Merge Simulation';},
  ]) {const rows = structuredClone(indexed);mutate(rows);assert.throws(() => readShardAdmission(rows, policy), /physical identities/u);}
});

test('supplemental physical cells preserve original native oracle, doctest and example owners and all deadlines', () => {
  const source = readFileSync(new URL('../../.github/workflows/rust.yml', import.meta.url), 'utf8');
  const jobs = new Map(parseWorkflow(source).map(job => [job.id, job]));
  assert.equal(originalPolicy.platforms.length * originalPolicy.suites.reduce((sum, row) => sum + row.shards - 1, 0), 27);
  assert.equal(originalPolicy.suites.reduce((sum, row) => sum + row.shards - 1, 0), 9);
  for (const [id, cells] of [['test', 43], ['coverage', 17]]) {
    const job = jobs.get(id);
    assert.equal([...job.header.matchAll(/- suite: physical\n/gu)].length, cells);
    assert.match(job.header, /suite: \[grammar, inference, generative, translation, remaining\]/u);
    assert.match(job.header, /fail-fast: false/u);
    assert.match(job.header, /META_LANGUAGE_SHARD_SUITE: \$\{\{ matrix.logical_suite \|\| matrix.suite \}\}/u);
  }
  const native = jobs.get('test').steps.filter(step => /^Run complete /u.test(step.name));
  assert.equal(native.length, 9);
  for (const step of native) assert.match(step.text, /matrix.suite == 'translation' && matrix.os == 'ubuntu-latest'/u);
  for (const name of ['Run doc tests', 'Check examples']) assert.match(jobs.get('test').steps.find(step => step.name === name).text, /matrix.suite == 'remaining'/u);
  assert.equal(originalPolicy.freshMergeShards, 8);
  assert.match(jobs.get('fresh-merge').header, /physical_shard: \[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16\]/u);
  const caller = readFileSync(new URL('../../.github/workflows/ci.yml', import.meta.url), 'utf8');
  assert.match(caller, /acceptance:[\s\S]*?needs: \[js, rust,/u);
  assert.match(caller, /attempts\/\$GITHUB_RUN_ATTEMPT\/jobs\?per_page=100/u);
  assert.match(caller, /rust-test-shards.mjs --verify/u);
  const fresh = readFileSync(new URL('../../rust/scripts/simulate-fresh-merge.sh', import.meta.url), 'utf8');
  assert.match(fresh, /if \[ "\$\{META_LANGUAGE_SHARD_INDEX:-1\}" = 1 \]; then[\s\S]*?--doc[\s\S]*?--examples --no-run/u);
  assert.equal([...fresh.matchAll(/--config 'profile\.test\.package\.meta-language\.[^']+'/gu)].length, 3);
});
