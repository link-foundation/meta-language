// Physical JavaScript suite jobs retain the existing file partition as their
// sole membership source. A logical suite exists only after the whole union
// of those jobs is present for the evaluated checkout.
import { partitionGroupFiles, partitionProblems, partitionTests, testFiles, TEST_GROUPS } from './test-groups.mjs';

export function javascriptSuiteTestGroups() {
  const partition = partitionTests(testFiles());
  const problems = partitionProblems(partition);
  if (problems.length) throw new Error(`JavaScript test groups: ${problems.join('; ')}`);
  return partition.groups;
}

export function javascriptSuitePartition(group, shard = '1/1', groups = javascriptSuiteTestGroups()) {
  if (!Object.hasOwn(TEST_GROUPS, group) || !Object.hasOwn(groups, group)) {
    throw new Error(`unknown JavaScript evidence group ${group}`);
  }
  const files = partitionGroupFiles(groups[group], shard);
  const [index, count] = shard.split('/');
  return { name: `javascript-suite-${group}-${index}-of-${count}`, group, shard, files };
}

export function javascriptSuiteSelection(argv, stage, aggregate, groups) {
  const read = name => {
    const positions = argv.flatMap((value, index) => value === name ? [index] : []);
    if (positions.length > 1) throw new Error(`${name} must be supplied at most once`);
    if (!positions.length) return null;
    const value = argv[positions[0] + 1];
    if (!value || value.startsWith('--')) throw new Error(`${name} needs a value`);
    return value;
  };
  const group = read('--javascript-group');
  const shard = read('--javascript-shard');
  if (shard && !group) throw new Error('--javascript-shard requires --javascript-group');
  if (!group) return null;
  if (stage !== 'javascript-suite' || aggregate) {
    throw new Error('--javascript-group requires --stage javascript-suite and cannot accompany --aggregate');
  }
  return javascriptSuitePartition(group, shard ?? '1/1', groups);
}

export function javascriptSuiteGroupEntries(selection) {
  if (selection) return [{ group: selection.group, shard: selection.shard, label: `suite-${selection.name}` }];
  return Object.keys(TEST_GROUPS).map(group => ({ group, shard: null, label: `suite-javascript-${group}` }));
}

export function javascriptSuiteCommandArguments(group, shard) {
  const args = ['scripts/test-groups.mjs', '--run', group];
  if (shard) args.push('--shard', shard);
  return args;
}

export function isJavaScriptSuitePartitionName(name) {
  return typeof name === 'string' && name.startsWith('javascript-suite-');
}

/** Convert complete physical jobs to the original logical stage contract. */
export function mergeJavaScriptSuitePartitions({ checkpoint, commit, records, testGroups }) {
  const names = [...records.keys()].filter(isJavaScriptSuitePartitionName);
  if (checkpoint !== 'pre-merge' || !names.length) return records;
  const groups = testGroups ?? javascriptSuiteTestGroups();
  const merged = new Map(records);
  const problems = [];
  const partitions = new Map(Object.keys(TEST_GROUPS).map(group => [group, []]));
  if (records.has('javascript-suite')) problems.push('whole-suite and partition records cannot be combined');
  const commands = [], artifacts = [], failureLogs = [];
  let versions;
  for (const name of names.sort()) {
    merged.delete(name);
    const record = records.get(name);
    if (record instanceof Error || !record || typeof record !== 'object') {
      problems.push(`${name}: the partition record cannot be read${record instanceof Error ? `: ${record.message}` : ''}`);
      continue;
    }
    const mismatch = [['schemaVersion', 1], ['issue', 195], ['stage', name], ['commit', commit], ['checkpoint', checkpoint]]
      .find(([field, expected]) => record[field] !== expected);
    if (mismatch) {
      problems.push(`${name}: ${mismatch[0]} ${record[mismatch[0]]}, expected ${mismatch[1]}`);
      continue;
    }
    let selection;
    try {
      const scope = record.javascriptPartition;
      if (!scope || typeof scope !== 'object' || Array.isArray(scope)) throw new Error('the executed partition scope is missing');
      if (typeof scope.group !== 'string' || typeof scope.shard !== 'string') throw new Error('the executed group and shard are required');
      selection = javascriptSuitePartition(scope.group, scope.shard, groups);
      if (selection.name !== name || scope.name !== name) throw new Error('the stage identity does not match its group and shard');
      if (!Array.isArray(scope.files) || JSON.stringify(scope.files) !== JSON.stringify(selection.files)) {
        throw new Error('executed files do not match the exact current group/shard partition');
      }
    } catch (error) {
      problems.push(`${name}: ${error.message}`);
      continue;
    }
    partitions.get(selection.group).push(selection);
    if (record.error !== null && typeof record.error !== 'string') {
      problems.push(`${name}: the partition error must be null or a command failure`);
    } else if (record.error) problems.push(`${name}: ${record.error}`);
    const produced = record.groups;
    if (!produced || typeof produced !== 'object' || Array.isArray(produced)
        || Object.keys(produced).length !== 1 || !Object.hasOwn(produced, 'suite:javascript')) {
      problems.push(`${name}: the partition must report only suite:javascript`);
      continue;
    }
    const bundle = produced['suite:javascript'];
    if (!bundle || !['commands', 'artifacts', 'failureLogs'].every(field =>
      Array.isArray(bundle[field]) && bundle[field].every(value => typeof value === 'string'))) {
      problems.push(`${name}: the partition evidence bundle is malformed`);
      continue;
    }
    const expectedCommand = ['node', ...javascriptSuiteCommandArguments(selection.group, selection.shard)].join(' ');
    if (bundle.commands.length !== 1 || bundle.commands[0] !== expectedCommand || bundle.artifacts.length < 1) {
      problems.push(`${name}: the recorded command does not execute exactly its declared group and shard`);
    }
    if (!record.toolchainVersions || typeof record.toolchainVersions !== 'object' || Array.isArray(record.toolchainVersions)
        || Object.values(record.toolchainVersions).some(value => typeof value !== 'string')) {
      problems.push(`${name}: toolchain versions are missing`);
      continue;
    }
    const entries = Object.entries(record.toolchainVersions).sort(([a], [b]) => a.localeCompare(b));
    if (versions && JSON.stringify(entries) !== JSON.stringify(Object.entries(versions).sort(([a], [b]) => a.localeCompare(b)))) {
      problems.push(`${name}: toolchain versions differ between suite partitions`);
    } else versions ??= record.toolchainVersions;
    commands.push(...bundle.commands);
    artifacts.push(...bundle.artifacts);
    failureLogs.push(...bundle.failureLogs);
  }
  for (const [group, selections] of partitions) {
    if (!selections.length) {
      problems.push(`JavaScript test group ${group} produced no partition record`);
      continue;
    }
    const counts = new Set(selections.map(selection => Number(selection.shard.split('/')[1])));
    if (counts.size !== 1) {
      problems.push(`JavaScript test group ${group} has mixed shard counts`);
      continue;
    }
    const count = [...counts][0];
    const indices = selections.map(selection => Number(selection.shard.split('/')[0]));
    const missing = Array.from({ length: count }, (_, index) => index + 1).filter(index => !indices.includes(index));
    if (new Set(indices).size !== indices.length || missing.length) {
      problems.push(`JavaScript test group ${group} has duplicate or missing shards${missing.length ? `: ${missing.join(', ')}` : ''}`);
    }
    const executed = selections.flatMap(selection => selection.files);
    if (new Set(executed).size !== executed.length || JSON.stringify([...executed].sort()) !== JSON.stringify([...groups[group]].sort())) {
      problems.push(`JavaScript test group ${group} does not cover every current file exactly once`);
    }
  }
  merged.set('javascript-suite', {
    schemaVersion: 1, issue: 195, stage: 'javascript-suite', commit, checkpoint,
    toolchainVersions: versions ?? {},
    groups: problems.length ? {} : { 'suite:javascript': {
      commands: [...new Set(commands)], artifacts: [...new Set(artifacts)], failureLogs: [...new Set(failureLogs)],
    } },
    error: problems.length ? [...new Set(problems)].join('; ') : null,
  });
  return merged;
}
