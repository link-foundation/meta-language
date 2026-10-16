#!/usr/bin/env node
// Cargo's host runner partitions actual libtest inventories. Cargo commands,
// logical filters, native status codes and complete output remain observable.
import {createHash, randomUUID} from 'node:crypto';
import {spawn, spawnSync} from 'node:child_process';
import {appendFileSync, closeSync, mkdirSync, openSync, readFileSync, readdirSync, writeFileSync, writeSync} from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {readStructuredDataLino, structuredDataToLino} from '../src/structured-data-links.js';

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const script = fileURLToPath(import.meta.url);
const policyPath = path.join(repository, 'parity/rust-test-physical-shards.lino');
const digest = value => createHash('sha256').update(value).digest('hex');
const requireValue = (condition, message) => {if (!condition) throw new Error(`Rust physical shards: ${message}`);};
const encoded = value => structuredDataToLino(value);
const read = file => readStructuredDataLino(readFileSync(file, 'utf8'));
export function readPhysicalShardPolicy() {return read(policyPath);}

export function physicalShardCount(policy, profile, suite) {
  if (profile === 'fresh-merge') return policy.freshMergeShards;
  requireValue(['test', 'coverage'].includes(profile), 'unknown execution profile');
  const row = policy.suites.find(row => row.name === suite);
  requireValue(row, 'unknown logical suite');
  return row.shards;
}

export function readHarnessInventory(output) {
  const names = output.split(/\r?\n/u).filter(line => line && !/^\d+ tests?, \d+ benchmarks?$/u.test(line)).map(line => {
    const match = /^(.*): (test|benchmark)$/u.exec(line);
    requireValue(match && match[1] && !/[\r\n\0]/u.test(match[1]), 'invalid native test inventory');
    return {name: match[1], kind: match[2]};
  });
  requireValue(new Set(names.map(row => row.name)).size === names.length, 'duplicate native test identity');
  return names.sort((left, right) => left.name < right.name ? -1 : left.name > right.name ? 1 : 0);
}

// Identity hashing spreads independently expensive tests across cells without
// dropping new names or binding scheduling to host-specific binary hashes.
export function selectPhysicalTests(inventory, shard, count) {
  requireValue(Number.isSafeInteger(shard) && Number.isSafeInteger(count) && shard >= 1 && shard <= count, 'invalid physical shard');
  return inventory.filter(row => Number.parseInt(digest(row.name).slice(0, 8), 16) % count === shard - 1);
}

export function exactHarnessArguments(original, selected) {
  const controls = [];
  const withValue = new Set(['--skip', '--test-threads', '--color', '--format', '--logfile', '--shuffle-seed']);
  for (let index = 0; index < original.length; index++) {
    const argument = original[index];
    if (withValue.has(argument)) {
      requireValue(index + 1 < original.length, 'missing harness option value');
      controls.push(argument, original[++index]);
    } else if (argument.startsWith('-')) controls.push(argument);
    // Positive filters have already selected the actual native inventory.
  }
  requireValue(!controls.includes('--list'), 'execution cannot be an inventory listing');
  const names = selected.map(row => row.name);
  return [...controls, '--exact', ...(names.length ? names : ['__meta_language_empty_physical_partition__'])];
}

export function readHarnessOutcomes(output) {
  return output.split(/\r?\n/u).flatMap(line => {
    const match = /^test (.+) \.\.\. (ok|FAILED|ignored)(?:,.*)?$/u.exec(line);
    return match ? [{name: match[1], outcome: match[2]}] : [];
  });
}

export function matchesLogicalFilters(name, arguments_) {
  const positive = [], skipped = [];
  for (let index = 0; index < arguments_.length; index++) {
    if (arguments_[index] === '--skip') skipped.push(arguments_[++index]);
    else {requireValue(!arguments_[index].startsWith('-'), 'unsupported logical filter option');positive.push(arguments_[index]);}
  }
  return (!positive.length || positive.some(filter => name.includes(filter))) && !skipped.some(filter => name.includes(filter));
}

export function verifyHarnessExecution(receipt) {
  requireValue(receipt.allInventoryDigest === digest(JSON.stringify(receipt.allInventory)), 'changed unfiltered inventory');
  requireValue(JSON.stringify(receipt.allInventory.filter(row => matchesLogicalFilters(row.name, receipt.arguments))) === JSON.stringify(receipt.inventory), 'logical inventory differs from the complete native inventory');
  const selected = selectPhysicalTests(receipt.inventory, receipt.shard, receipt.count);
  requireValue(JSON.stringify(selected) === JSON.stringify(receipt.selected), 'changed selected inventory');
  requireValue(receipt.inventoryDigest === digest(JSON.stringify(receipt.inventory)), 'changed complete inventory');
  const observed = receipt.outcomes.map(row => row.name).sort();
  const expected = selected.map(row => row.name).sort();
  requireValue(JSON.stringify(observed) === JSON.stringify(expected), 'missing or duplicate native test outcomes');
  requireValue(receipt.exitCode === 0 && receipt.signal === null && receipt.outcomes.every(row => row.outcome !== 'FAILED'), 'native test execution failed');
}

export async function executePhysicalHarness({binary, arguments: original, identity, shard, count, invoke}) {
  const listed = await invoke(binary, [...original, '--list', '--format', 'terse'], true);
  requireValue(listed.exitCode === 0 && listed.signal === null, 'native test inventory failed');
  const inventory = readHarnessInventory(listed.stdout);
  const allListed = await invoke(binary, ['--list', '--format', 'terse'], 'all');
  requireValue(allListed.exitCode === 0 && allListed.signal === null, 'complete native inventory failed');
  const allInventory = readHarnessInventory(allListed.stdout);
  requireValue(!inventory.some(row => row.name === '__meta_language_empty_physical_partition__'), 'reserved partition identity');
  const selected = selectPhysicalTests(inventory, shard, count);
  // Empty harnesses remain actual executions, with an explicitly empty cell.
  const actual = await invoke(binary, exactHarnessArguments(original, selected), false);
  return {schemaVersion: 1, identity, binary: path.basename(binary), arguments: original,
    inventory, inventoryDigest: digest(JSON.stringify(inventory)), allInventory, allInventoryDigest: digest(JSON.stringify(allInventory)), selected, shard, count,
    listingDigest: digest(listed.stdout),
    allListingDigest: digest(allListed.stdout),
    outcomes: readHarnessOutcomes(actual.stdout), exitCode: actual.exitCode, signal: actual.signal,
    stdoutDigest: digest(actual.stdout), stderrDigest: digest(actual.stderr)};
}

function nativeText(command, arguments_) {
  const result = spawnSync(command, arguments_, {cwd: repository, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024});
  requireValue(result.status === 0 && !result.signal && !result.error, `cannot read ${command} execution identity`);
  return result.stdout.trim();
}

export function shardIdentity(environment = process.env) {
  const commit = environment.GITHUB_SHA;
  requireValue(/^[0-9a-f]{40}$/u.test(commit ?? ''), 'exact GitHub commit required');
  const identity = {commit, run: environment.GITHUB_RUN_ID, attempt: environment.GITHUB_RUN_ATTEMPT,
    profile: environment.META_LANGUAGE_SHARD_PROFILE, platform: environment.META_LANGUAGE_SHARD_PLATFORM,
    suite: environment.META_LANGUAGE_SHARD_SUITE, tree: nativeText('git', ['rev-parse', 'HEAD^{tree}'])};
  requireValue(identity.run && identity.attempt && identity.platform, 'missing run identity');
  if (identity.profile === 'fresh-merge') {
    identity.base = environment.META_LANGUAGE_MERGE_BASE;
    identity.head = environment.META_LANGUAGE_MERGE_HEAD;
    requireValue([identity.base, identity.head].every(value => /^[0-9a-f]{40}$/u.test(value ?? '')), 'missing fresh merge identities');
  } else requireValue(nativeText('git', ['rev-parse', 'HEAD']) === commit, 'checkout differs from recorded commit');
  return identity;
}

function context(environment = process.env) {
  const policy = readPhysicalShardPolicy();
  const profile = environment.META_LANGUAGE_SHARD_PROFILE;
  const suite = environment.META_LANGUAGE_SHARD_SUITE;
  const shard = Number(environment.META_LANGUAGE_SHARD_INDEX);
  const count = physicalShardCount(policy, profile, suite);
  selectPhysicalTests([], shard, count);
  const directory = path.join(repository, 'issue-195-artifacts/rust-physical-shards', `${profile}-${environment.META_LANGUAGE_SHARD_PLATFORM}-${suite}-${shard}`);
  return {policy, profile, suite, shard, count, directory, identity: shardIdentity(environment)};
}

function writeReceipt(directory, name, value) {
  mkdirSync(directory, {recursive: true});
  writeFileSync(path.join(directory, `${name}.lino`), encoded(value), {flag: 'wx'});
}

// Only Cargo's own root test executables are partitioned. A CLI or a nested
// consumer executable retains its original invocation and native result.
export function isRootHarness(binary, root = repository) {
  const paths = /^[A-Za-z]:[\\/]/u.test(binary) ? path.win32 : path;
  const relative = paths.relative(paths.join(root, 'rust/target'), paths.resolve(binary));
  return !relative.startsWith('..') && !paths.isAbsolute(relative)
    && relative.split(paths.sep).includes('deps')
    && /-[0-9a-f]+(?:\.exe)?$/u.test(paths.basename(binary));
}

// Cargo's StringList deserializer splits strings on Unicode whitespace. It
// does not interpret shell quotes or JSON escapes; keep Windows '\\' literal.
// https://github.com/rust-lang/cargo/blob/0892e1e1c21e3f68ee7c542a2347ad30690ef363/src/context/de.rs#L180
// Paths containing whitespace require Cargo's array configuration form. This
// environment adapter rejects those paths before writing any configuration.
export function renderCargoRunnerConfiguration(host, executable, runnerScript) {
  requireValue(typeof host === 'string' && /^[A-Za-z0-9_-]+$/u.test(host), 'invalid Rust host target');
  for (const value of [executable, runnerScript]) {
    requireValue(typeof value === 'string' && value.length > 0
      && !/[\s\u0000-\u0020\u007f-\u009f]/u.test(value)
      && !/^['"]|['"]$/u.test(value), 'Cargo runner environment paths must be unquoted and contain no whitespace or controls; use array configuration for paths with spaces');
  }
  const name = `CARGO_TARGET_${host.toUpperCase().replaceAll('-', '_')}_RUNNER`;
  const value = [executable, runnerScript, '--runner'].join(' ');
  return {name, value, environmentLine: `${name}=${value}\n`, shellLine: `export ${name}='${value.replaceAll("'", "'\\''")}'`};
}

async function invokeNative(binary, arguments_, listing, logPrefix) {
  if (listing) {
    const result = spawnSync(binary, arguments_, {encoding: 'utf8', maxBuffer: 4 * 1024 * 1024});
    writeFileSync(`${logPrefix}.${listing === 'all' ? 'all-inventory' : 'inventory'}.log`, result.stdout ?? '', {flag: 'wx'});
    writeFileSync(`${logPrefix}.${listing === 'all' ? 'all-inventory' : 'inventory'}.stderr.log`, result.stderr ?? '', {flag: 'wx'});
    if (result.status !== 0 || result.signal || result.error) {
      process.stdout.write(result.stdout ?? '');process.stderr.write(result.stderr ?? '');
      console.error(`native inventory exited ${result.status}, signal=${result.signal}, error=${result.error?.message ?? 'none'}`);
    }
    return {stdout: result.stdout ?? '', stderr: result.stderr ?? '', exitCode: result.status, signal: result.signal};
  }
  const stdout = [], stderr = [];
  const files = ['stdout', 'stderr'].map(channel => openSync(`${logPrefix}.${channel}.log`, 'wx'));
  try {
    return await new Promise((resolve, reject) => {
      const child = spawn(binary, arguments_, {stdio: ['inherit', 'pipe', 'pipe']});
      child.stdout.on('data', data => {writeSync(files[0], data);process.stdout.write(data);stdout.push(data);});
      child.stderr.on('data', data => {writeSync(files[1], data);process.stderr.write(data);stderr.push(data);});
      child.on('error', reject);
      child.on('close', (exitCode, signal) => resolve({stdout: Buffer.concat(stdout).toString('utf8'), stderr: Buffer.concat(stderr).toString('utf8'), exitCode, signal}));
      for (const signal of ['SIGTERM', 'SIGINT']) process.once(signal, () => child.kill(signal));
    });
  } finally {for (const file of files) closeSync(file);}
}

async function runHarness(binary, arguments_) {
  const current = context();
  mkdirSync(current.directory, {recursive: true});
  const name = digest(JSON.stringify([path.basename(binary), arguments_, randomUUID()])).slice(0, 24);
  const logPrefix = path.join(current.directory, name);
  if (!isRootHarness(binary)) {
    const actual = await invokeNative(binary, arguments_, false, logPrefix);
    writeReceipt(current.directory, name, {schemaVersion: 1, identity: current.identity, auxiliary: true,
      binary: path.basename(binary), arguments: arguments_, shard: current.shard, count: current.count,
      exitCode: actual.exitCode, signal: actual.signal, stdoutDigest: digest(actual.stdout), stderrDigest: digest(actual.stderr)});
    return actual.exitCode ?? 1;
  }
  const receipt = await executePhysicalHarness({binary, arguments: arguments_, identity: current.identity,
    shard: current.shard, count: current.count,
    invoke: (binary, arguments_, listing) => invokeNative(binary, arguments_, listing, logPrefix)});
  writeReceipt(current.directory, name, receipt);
  if (receipt.exitCode !== 0 || receipt.signal !== null) return receipt.exitCode ?? 1;
  verifyHarnessExecution(receipt);
  return receipt.exitCode;
}

function configure(shell = false) {
  const current = context();
  const host = /^host: (.+)$/mu.exec(nativeText('rustc', ['-vV']))?.[1];
  requireValue(host, 'missing Rust host target');
  const runner = renderCargoRunnerConfiguration(host, process.execPath, script);
  const metadata = JSON.parse(nativeText('cargo', ['metadata', '--locked', '--no-deps', '--format-version', '1', '--manifest-path', path.join(repository, 'rust/Cargo.toml')]));
  const target = metadata.packages.find(row => path.resolve(row.manifest_path) === path.join(repository, 'rust/Cargo.toml'));
  requireValue(target, 'missing root Cargo package');
  const expectedBinaries = target.targets.filter(row => row.test && row.kind.some(kind => ['lib', 'bin', 'test'].includes(kind)))
    .map(row => row.name.replaceAll('-', '_')).sort();
  requireValue(expectedBinaries.length > 0, 'no root Cargo test targets');
  requireValue(process.env.GITHUB_ENV, 'GitHub environment file required');
  appendFileSync(process.env.GITHUB_ENV, runner.environmentLine);
  writeReceipt(current.directory, 'identity', {identity: current.identity, shard: current.shard, count: current.count,
    policyDigest: digest(readFileSync(policyPath)), host, expectedBinaries});
  if (shell) console.log(runner.shellLine);
}

function finish() {
  const current = context();
  const files = readdirSync(current.directory).filter(file => file.endsWith('.lino') && file !== 'seal.lino').sort();
  requireValue(files.some(file => file !== 'identity.lino'), 'no actual native executions recorded');
  const receipts = files.map(file => ({file, sha256: digest(readFileSync(path.join(current.directory, file)))}));
  writeReceipt(current.directory, 'seal', {identity: current.identity, shard: current.shard, count: current.count,
    result: process.env.META_LANGUAGE_SHARD_RESULT, receipts});
  requireValue(process.env.META_LANGUAGE_SHARD_RESULT === 'success', 'physical job failed');
}

export function readShardAdmission(jobs, policy) {
  const definitions = [['test', /^Rust \/ Test \(/u], ['coverage', /^Rust \/ Code Coverage \(/u], ['fresh-merge', /^Rust \/ Fresh Merge Simulation(?: \([1-9][0-9]*\))?$/u]];
  const profiles = [];
  for (const [profile, pattern] of definitions) {
    const active = jobs.filter(job => pattern.test(job.name) && job.conclusion !== 'skipped');
    if (!active.length) continue;
    const expected = profile === 'fresh-merge' ? policy.freshMergeShards
      : policy.suites.reduce((sum, row) => sum + row.shards, 0) * (profile === 'test' ? policy.platforms.length : 1);
    requireValue(active.length === expected, 'missing or duplicate scheduled physical jobs');
    const expectedNames = profile === 'fresh-merge' ? Array(policy.freshMergeShards).fill('Rust / Fresh Merge Simulation')
      : (profile === 'test' ? policy.platforms : [null]).flatMap(platform => policy.suites.flatMap(row =>
        Array(row.shards).fill(profile === 'test' ? `Rust / Test (${platform}, ${row.name})` : `Rust / Code Coverage (${row.name})`)));
    let actualNames = active.map(job => job.name);
    if (profile === 'fresh-merge' && actualNames.some(name => name !== 'Rust / Fresh Merge Simulation')) {
      const indexed = Array.from({length: policy.freshMergeShards}, (_, index) => `Rust / Fresh Merge Simulation (${index + 1})`);
      requireValue(JSON.stringify([...actualNames].sort()) === JSON.stringify(indexed.sort()), 'changed scheduled fresh merge physical identities');
      actualNames = actualNames.map(() => 'Rust / Fresh Merge Simulation');
    }
    requireValue(JSON.stringify(actualNames.sort()) === JSON.stringify(expectedNames.sort()), 'changed scheduled logical scopes');
    requireValue(active.every(job => job.status === 'completed'), 'physical job inventory is unfinished');
    profiles.push(profile);
  }
  return profiles;
}

export function verifyPhysicalShardUnion(groups, policy, {commit, run, attempt, profiles = ['test', 'coverage', 'fresh-merge']}) {
  const expected = [];
  for (const profile of ['test', 'coverage'].filter(profile => profiles.includes(profile))) for (const platform of profile === 'test' ? policy.platforms : ['ubuntu-latest']) {
    for (const row of policy.suites) for (let shard = 1; shard <= row.shards; shard++) expected.push([profile, platform, row.name, shard]);
  }
  if (profiles.includes('fresh-merge')) for (let shard = 1; shard <= policy.freshMergeShards; shard++) expected.push(['fresh-merge', 'ubuntu-latest', 'all', shard]);
  const keys = new Set();
  const inventories = new Map();
  const completeInventories = new Map();
  const completeOutcomes = new Map();
  const trees = new Map();
  const binaryKeys = new Map();
  for (const group of groups) {
    const seal = group.seal, identity = seal.identity;
    requireValue(identity.commit === commit && identity.run === run && identity.attempt === attempt, 'stale run or commit identity');
    const key = JSON.stringify([identity.profile, identity.platform, identity.suite, seal.shard]);
    requireValue(!keys.has(key), 'duplicate physical cell');keys.add(key);
    requireValue(seal.result === 'success', 'failed physical cell');
    const count = physicalShardCount(policy, identity.profile, identity.suite);
    requireValue(seal.count === count, 'changed physical cell count');
    const scope = JSON.stringify([identity.profile, identity.platform, identity.suite]);
    const source = JSON.stringify({tree: identity.tree, base: identity.base ?? null, head: identity.head ?? null});
    const sourceScope = identity.profile === 'fresh-merge' ? 'fresh-merge' : 'commit';
    if (identity.profile === 'fresh-merge') requireValue(identity.head === commit && /^[0-9a-f]{40}$/u.test(identity.base ?? ''), 'invalid fresh merge base or head');
    if (trees.has(sourceScope)) requireValue(trees.get(sourceScope) === source, 'different source or fresh merge identities');
    trees.set(sourceScope, source);
    requireValue(group.executions.length > 0, 'missing native executions');
    const names = [];
    const binaries = [];
    for (const receipt of group.executions) {
      requireValue(JSON.stringify(receipt.identity) === JSON.stringify(identity), 'execution identity differs from cell');
      requireValue(receipt.shard === seal.shard && receipt.count === count, 'execution physical identity differs');
      // A tested CLI rejection may intentionally return nonzero. Its owning
      // libtest assertion and the cell seal establish whether that was expected.
      if (receipt.auxiliary) {requireValue(Number.isInteger(receipt.exitCode) || typeof receipt.signal === 'string', 'missing auxiliary native result');continue;}
      const filters = identity.profile === 'fresh-merge' ? [] : policy.suites.find(row => row.name === identity.suite).filters;
      requireValue(JSON.stringify(receipt.arguments) === JSON.stringify(filters), 'changed original logical filters');
      verifyHarnessExecution(receipt);
      binaries.push(receipt.binary.replace(/-[0-9a-f]+(?:\.exe)?$/u, ''));
      const completeKey = JSON.stringify([identity.profile, identity.platform, receipt.binary]);
      const complete = JSON.stringify(receipt.allInventory);
      if (completeInventories.has(completeKey)) requireValue(completeInventories.get(completeKey) === complete, 'different unfiltered native inventories');
      completeInventories.set(completeKey, complete);
      const outcomes = completeOutcomes.get(completeKey) ?? [];
      completeOutcomes.set(completeKey, [...outcomes, ...receipt.outcomes.map(row => row.name)]);
      const binary = JSON.stringify([scope, receipt.binary, receipt.arguments]);
      const inventory = JSON.stringify(receipt.inventory);
      if (inventories.has(binary)) requireValue(inventories.get(binary) === inventory, 'different complete native inventories');
      inventories.set(binary, inventory);names.push(binary);
    }
    requireValue(new Set(names).size === names.length, 'duplicate native harness invocation');
    requireValue(JSON.stringify(binaries.sort()) === JSON.stringify(group.expectedBinaries), 'missing native Cargo test target');
    binaryKeys.set(key, names.sort());
  }
  requireValue(JSON.stringify([...keys].sort()) === JSON.stringify(expected.map(row => JSON.stringify(row)).sort()), 'missing or unregistered physical cells');
  for (const [key, inventory] of completeInventories) {
    const names = JSON.parse(inventory).map(row => row.name).sort();
    requireValue(JSON.stringify(completeOutcomes.get(key).sort()) === JSON.stringify(names), 'complete native union has missing or duplicate tests');
  }
  for (const row of expected.filter(row => row[3] === 1)) {
    const peers = groups.filter(group => [group.seal.identity.profile, group.seal.identity.platform, group.seal.identity.suite].every((value, index) => value === row[index]));
    const keysOf = peer => binaryKeys.get(JSON.stringify([peer.seal.identity.profile, peer.seal.identity.platform, peer.seal.identity.suite, peer.seal.shard]));
    for (const peer of peers) requireValue(JSON.stringify(keysOf(peer)) === JSON.stringify(keysOf(peers[0])), 'missing native harness in a physical cell');
  }
  return {cells: keys.size, inventories: inventories.size};
}

export function verifyPhysicalShardDirectory(directory, expected) {
  const groups = [];
  function visit(directory) {
    const files = readdirSync(directory, {withFileTypes: true});
    if (files.some(file => file.name === 'seal.lino')) {
      requireValue(!files.some(file => file.isDirectory()), 'unexpected nested physical cell');
      const seal = read(path.join(directory, 'seal.lino'));
      const executions = [];
      let expectedBinaries = null;
      const actual = files.filter(file => file.isFile() && file.name.endsWith('.lino') && file.name !== 'seal.lino').map(file => file.name).sort();
      requireValue(JSON.stringify(actual) === JSON.stringify(seal.receipts.map(row => row.file).sort()), 'missing or extra receipt files');
      for (const row of seal.receipts) {
        requireValue(path.basename(row.file) === row.file, 'invalid receipt path');
        const file = path.join(directory, row.file);
        requireValue(digest(readFileSync(file)) === row.sha256, 'changed execution receipt');
        const receipt = read(file);
        if (row.file === 'identity.lino') {
          requireValue(JSON.stringify(receipt.identity) === JSON.stringify(seal.identity), 'changed configured source identity');
          requireValue(receipt.shard === seal.shard && receipt.count === seal.count, 'changed configured physical identity');
          requireValue(receipt.policyDigest === digest(readFileSync(policyPath)), 'changed scheduling policy');
          expectedBinaries = receipt.expectedBinaries;
        } else {
          for (const channel of ['stdout', 'stderr']) {
            const log = path.join(directory, row.file.replace(/\.lino$/u, `.${channel}.log`));
            requireValue(digest(readFileSync(log)) === receipt[`${channel}Digest`], 'changed complete native output');
          }
          if (!receipt.auxiliary) {
            const listing = readFileSync(path.join(directory, row.file.replace(/\.lino$/u, '.inventory.log')), 'utf8');
            requireValue(digest(listing) === receipt.listingDigest, 'changed complete native inventory output');
            requireValue(JSON.stringify(readHarnessInventory(listing)) === JSON.stringify(receipt.inventory), 'native listing differs from declared inventory');
            const allListing = readFileSync(path.join(directory, row.file.replace(/\.lino$/u, '.all-inventory.log')), 'utf8');
            requireValue(digest(allListing) === receipt.allListingDigest, 'changed unfiltered native inventory output');
            requireValue(JSON.stringify(readHarnessInventory(allListing)) === JSON.stringify(receipt.allInventory), 'unfiltered native listing differs from declared inventory');
            const stdout = readFileSync(path.join(directory, row.file.replace(/\.lino$/u, '.stdout.log')), 'utf8');
            requireValue(JSON.stringify(readHarnessOutcomes(stdout)) === JSON.stringify(receipt.outcomes), 'native output differs from declared outcomes');
          }
          executions.push(receipt);
        }
      }
      requireValue(expectedBinaries !== null, 'missing configured Cargo target inventory');
      groups.push({seal, executions, expectedBinaries});
    } else {
      requireValue(!files.some(file => file.name.endsWith('.lino')), 'unsealed physical execution receipts');
      for (const entry of files) if (entry.isDirectory()) visit(path.join(directory, entry.name));
    }
  }
  visit(directory);
  return verifyPhysicalShardUnion(groups, readPhysicalShardPolicy(), expected);
}

if (process.argv[1] && path.resolve(process.argv[1]) === script) {
  try {
    const [mode, ...arguments_] = process.argv.slice(2);
    if (mode === '--configure') configure(arguments_[0] === '--shell');
    else if (mode === '--finish') finish();
    else if (mode === '--runner') process.exitCode = await runHarness(arguments_[0], arguments_.slice(1));
    else if (mode === '--admission') {
      const pages = JSON.parse(readFileSync(arguments_[0], 'utf8'));
      const profiles = readShardAdmission(pages.flatMap(page => page.jobs), readPhysicalShardPolicy());
      appendFileSync(process.env.GITHUB_OUTPUT, `enabled=${profiles.length > 0}\nprofiles=${profiles.join(',')}\n`);
    }
    else if (mode === '--verify') console.log(verifyPhysicalShardDirectory(arguments_[0], {commit: process.env.GITHUB_SHA, run: process.env.GITHUB_RUN_ID,
      attempt: process.env.GITHUB_RUN_ATTEMPT, profiles: (process.env.META_LANGUAGE_SHARD_EXPECTED_PROFILES ?? '').split(',').filter(Boolean)}));
    else throw new Error('expected --configure, --runner, --finish or --verify');
  } catch (error) {console.error(error.stack);process.exitCode = 1;}
}
