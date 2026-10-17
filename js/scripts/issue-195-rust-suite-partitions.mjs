// Physical native executions preserve the original logical Rust evidence stage.
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { physicalShardCount, readPhysicalShardGroups, readPhysicalShardPolicy, verifyPhysicalShardUnion } from './rust-test-shards.mjs';

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

export function rustSuitePartition(shard, policy = readPhysicalShardPolicy()) {
  const count = physicalShardCount(policy, 'evidence', 'all');
  const match = /^([1-9][0-9]*)\/([1-9][0-9]*)$/u.exec(shard ?? '');
  if (!match || Number(match[2]) !== count || Number(match[1]) > count) {
    throw new Error(`Rust evidence shard must select one of 1/${count} through ${count}/${count}`);
  }
  const index = Number(match[1]);
  return { name: `rust-suite-${index}-of-${count}`, shard: `${index}/${count}`, index, count };
}

export function rustSuiteSelection(argv, stage, aggregate, policy) {
  const positions = argv.flatMap((value, index) => value === '--rust-shard' ? [index] : []);
  if (positions.length > 1) throw new Error('--rust-shard must be supplied at most once');
  if (!positions.length) return null;
  if (stage !== 'rust-suite' || aggregate) throw new Error('--rust-shard requires --stage rust-suite without --aggregate');
  return rustSuitePartition(argv[positions[0] + 1], policy);
}

export function assertRustSuitePartitionEnvironment(selection, environment) {
  const expected = { META_LANGUAGE_SHARD_PROFILE: 'evidence', META_LANGUAGE_SHARD_PLATFORM: 'ubuntu-latest',
    META_LANGUAGE_SHARD_SUITE: 'all', META_LANGUAGE_SHARD_INDEX: String(selection.index) };
  for (const [name, value] of Object.entries(expected)) {
    if (environment[name] !== value) throw new Error(`Rust evidence partition ${selection.shard}: ${name} must be ${value}`);
  }
}

export function rustSuiteCommandArguments(root) {
  return ['test', '--no-fail-fast', '--locked', '--manifest-path', path.join(root, 'rust/Cargo.toml'), '--tests',
    '--config', 'profile.test.package.meta-language.opt-level=3',
    '--config', 'profile.test.package.meta-language.debug-assertions=true',
    '--config', 'profile.test.package.meta-language.overflow-checks=true', '--', '--nocapture'];
}

function rustSuiteCommandText(root) {
  return ['cargo', ...rustSuiteCommandArguments(root)].map(value => /^[A-Za-z0-9_./:=@+-]+$/u.test(value)
    ? value : `'${value.replaceAll("'", "'\\''")}'`).join(' ');
}

export function isRustSuitePartitionName(name) {
  return typeof name === 'string' && name.startsWith('rust-suite-');
}

export function verifyRustSuitePartitionUnion({ commit, directory = path.join(repository, 'issue-195-artifacts/rust-physical-partitions'),
  environment = process.env, policy = readPhysicalShardPolicy() }) {
  const groups = readPhysicalShardGroups(directory).filter(group => group.seal.identity.profile === 'evidence');
  return verifyPhysicalShardUnion(groups, policy, { commit, run: environment.GITHUB_RUN_ID,
    attempt: environment.GITHUB_RUN_ATTEMPT, profiles: ['evidence'] });
}

/** A logical suite requires every actual native inventory and outcome once. */
export function mergeRustSuitePartitions({ checkpoint, commit, records, root = repository,
  policy = readPhysicalShardPolicy(), verifyPhysicalUnion = verifyRustSuitePartitionUnion }) {
  const names = [...records.keys()].filter(isRustSuitePartitionName);
  if (checkpoint !== 'pre-merge' || !names.length) return records;
  const merged = new Map(records), problems = [], indices = [];
  const commands = [], artifacts = [], failureLogs = [];
  let versions;
  if (records.has('rust-suite')) problems.push('whole-suite and partition records cannot be combined');
  for (const name of names.sort()) {
    merged.delete(name);
    const record = records.get(name);
    if (record instanceof Error || !record || typeof record !== 'object') {
      problems.push(`${name}: the partition record cannot be read`);
      continue;
    }
    const mismatch = [['schemaVersion', 1], ['issue', 195], ['stage', name], ['commit', commit], ['checkpoint', checkpoint]]
      .find(([field, expected]) => record[field] !== expected);
    if (mismatch) { problems.push(`${name}: ${mismatch[0]} differs from the evaluated stage`); continue; }
    let selection;
    try {
      selection = rustSuitePartition(record.rustPartition?.shard, policy);
      if (selection.name !== name || JSON.stringify(record.rustPartition) !== JSON.stringify(selection)) {
        throw new Error('the recorded native partition scope differs');
      }
    } catch (error) { problems.push(`${name}: ${error.message}`); continue; }
    indices.push(selection.index);
    if (record.error !== null && typeof record.error !== 'string') problems.push(`${name}: malformed partition error`);
    else if (record.error) problems.push(`${name}: ${record.error}`);
    if (!record.groups || Object.keys(record.groups).length !== 1 || !Object.hasOwn(record.groups, 'suite:rust')) {
      problems.push(`${name}: the partition must report only suite:rust`);
      continue;
    }
    const bundle = record.groups['suite:rust'];
    if (!bundle || !['commands', 'artifacts', 'failureLogs'].every(field => Array.isArray(bundle[field])
      && bundle[field].every(value => typeof value === 'string'))) {
      problems.push(`${name}: malformed partition evidence bundle`);
      continue;
    }
    if (bundle.commands.length !== 1 || bundle.commands[0] !== rustSuiteCommandText(root)
      || !bundle.artifacts.includes(`issue-195-results/logs/suite-${name}.log`)) {
      problems.push(`${name}: the original complete native command or distinct output log differs`);
    }
    if (!record.toolchainVersions || typeof record.toolchainVersions !== 'object' || Array.isArray(record.toolchainVersions)
      || Object.values(record.toolchainVersions).some(value => typeof value !== 'string')) {
      problems.push(`${name}: toolchain versions are missing`);
      continue;
    }
    const selectedVersions = Object.entries(record.toolchainVersions).sort(([a], [b]) => a.localeCompare(b));
    if (versions && JSON.stringify(selectedVersions) !== JSON.stringify(Object.entries(versions).sort(([a], [b]) => a.localeCompare(b)))) {
      problems.push(`${name}: toolchain versions differ between native partitions`);
    } else versions ??= record.toolchainVersions;
    commands.push(...bundle.commands); artifacts.push(...bundle.artifacts); failureLogs.push(...bundle.failureLogs);
  }
  const count = physicalShardCount(policy, 'evidence', 'all');
  if (JSON.stringify([...indices].sort((a, b) => a - b)) !== JSON.stringify(Array.from({ length: count }, (_, index) => index + 1))) {
    problems.push('duplicate, missing or mixed native evidence partitions');
  }
  try { verifyPhysicalUnion({ commit, policy }); }
  catch (error) { problems.push(`the authenticated complete native test union failed: ${error.message}`); }
  merged.set('rust-suite', {
    schemaVersion: 1, issue: 195, stage: 'rust-suite', commit, checkpoint, toolchainVersions: versions ?? {},
    groups: problems.length ? {} : { 'suite:rust': { commands: [...new Set(commands)], artifacts: [...new Set(artifacts)], failureLogs: [...new Set(failureLogs)] } },
    error: problems.length ? [...new Set(problems)].join('; ') : null,
  });
  return merged;
}
