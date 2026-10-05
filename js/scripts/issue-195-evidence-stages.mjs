// The issue #195 evidence run, split into stages. CI runs every stage as its
// own job and uploads the stage's record, execution records, logs and
// artifacts; the Full Requirements Aggregate only merges the stage outputs,
// derives the groups that combine them, and writes the verification cells.
// A local run without --stage runs the same stages one after another.
//
// Each stage writes `work/stage-<name>.json`:
//
//   { schemaVersion: 1, issue: 195, stage, commit, checkpoint,
//     toolchainVersions, groups: { <group>: { commands, artifacts, failureLogs } },
//     error: null | string }
//
// and appends its execution records to `work/stage-<name>.jsonl`. A stage that
// fails, or never reports, becomes one gate error naming it instead of
// stopping the run. A stage that runs `after` another (each Rust stage after
// its JavaScript counterpart) and never reported because that stage failed
// adds no error of its own: the blocking failure's one error names it.

export const RUNTIMES = Object.freeze(['javascript', 'rust']);
export const NATIVE_TARGETS = Object.freeze(['JavaScript', 'Rust', 'Lean', 'Rocq']);

/** The toolchain versions the issue #195 evidence declares. */
export const DECLARED_TOOLCHAINS = Object.freeze({ rustc: '1.99.0', lean: '4.34.1', rocq: '9.3' });

const NATIVE_TOOLS = Object.freeze({ JavaScript: 'node', Rust: 'rustc', Lean: 'lean', Rocq: 'rocq' });
// The suites' tests run the native toolchains on emitted translations.
const SUITE_TOOLS = Object.freeze(['node', 'npm', 'rustc', 'cargo', 'lean', 'rocq']);
const CARGO_TOOLS = Object.freeze(['node', 'npm', 'rustc', 'cargo']);

/** Bounds for every build and test command the evidence run spawns, unless the caller set them. */
export const BOUNDED_BUILD_ENVIRONMENT = Object.freeze({
  CARGO_BUILD_JOBS: '2',
  RUST_TEST_THREADS: '2',
  CARGO_INCREMENTAL: '0',
});

function nativeStage(target) {
  return {
    name: `native-${target.toLowerCase()}`,
    target,
    // Each Rust job runs after its JavaScript counterpart passes.
    ...(target === 'Rust' ? { after: 'native-javascript' } : {}),
    groups: RUNTIMES.map((runtime) => `native:${target}:${runtime}`),
    tools: [...new Set(['node', NATIVE_TOOLS[target]])],
  };
}

/** The checkpoints, in delivery order; only pre-merge runs on pull requests. */
export const CHECKPOINTS = Object.freeze(['pre-merge', 'post-merge', 'release-delivery']);

/** The stages of each checkpoint, in the order a local run executes them. */
export const EVIDENCE_STAGES = Object.freeze({
  'pre-merge': Object.freeze([
    { name: 'javascript-suite', groups: ['suite:javascript'], tools: SUITE_TOOLS },
    { name: 'rust-suite', groups: ['suite:rust'], tools: SUITE_TOOLS, after: 'javascript-suite' },
    { name: 'runtime-parity', groups: ['runtime-parity'], tools: CARGO_TOOLS },
    // The native stages validate the runtime-parity stage's emitted translations.
    ...NATIVE_TARGETS.map(nativeStage),
    { name: 'delivery', groups: ['delivery:npm', 'delivery:crate', 'delivery:rml'], tools: CARGO_TOOLS },
  ]),
  // Live default-branch rule inspection runs on main after merge, as a
  // non-blocking report: no pull request check may depend on it.
  'post-merge': Object.freeze([
    { name: 'merge-enforcement', groups: ['merge-enforcement'], tools: ['node'] },
  ]),
  'release-delivery': Object.freeze([
    {
      name: 'delivery',
      groups: ['delivery:npm', 'delivery:crate', 'delivery:rml', 'delivery:formal-ai'],
      tools: CARGO_TOOLS,
    },
  ]),
});

/** Groups the aggregate derives itself: it measures the cleanup and joins each translation's evidence. */
export function aggregateGroups(checkpoint) {
  if (checkpoint !== 'pre-merge') return [];
  return [
    'cache-cleanup:measured',
    ...RUNTIMES.flatMap((runtime) => NATIVE_TARGETS.map((target) => `translation:${target}:${runtime}`)),
  ];
}

export function evidenceStages(checkpoint) {
  const stages = EVIDENCE_STAGES[checkpoint];
  if (!stages) throw new Error(`unknown issue-195 evidence checkpoint ${checkpoint}`);
  return stages;
}

export function evidenceStage(checkpoint, name) {
  const stage = evidenceStages(checkpoint).find((candidate) => candidate.name === name);
  if (!stage) {
    const names = evidenceStages(checkpoint).map((candidate) => candidate.name).join(', ');
    throw new Error(`unknown ${checkpoint} evidence stage ${name} (stages: ${names})`);
  }
  return stage;
}

/** Relative paths of a stage's record and execution records inside the results directory. */
export function stageFiles(name) {
  return { record: `work/stage-${name}.json`, observations: `work/stage-${name}.jsonl` };
}

/** Problems with the declared toolchain versions among `versions`. */
export function toolchainProblems(versions) {
  return Object.entries(DECLARED_TOOLCHAINS)
    .filter(([tool, wanted]) => tool in versions && !String(versions[tool]).includes(wanted))
    .map(([tool, wanted]) => `${tool} version does not match declared toolchain ${wanted}: ${versions[tool]}`);
}

/** `process.env` with the build bounds filled in where the caller left them unset. */
export function boundedEnvironment(env) {
  const bounded = { ...env };
  for (const [name, value] of Object.entries(BOUNDED_BUILD_ENVIRONMENT)) {
    if (!bounded[name]) bounded[name] = value;
  }
  return bounded;
}

export function bundleRecords(...records) {
  const present = records.filter(Boolean);
  return {
    commands: [...new Set(present.flatMap((record) => record.commands ?? []))],
    artifacts: [...new Set(present.flatMap((record) => record.artifacts ?? []))],
    failureLogs: [...new Set(present.flatMap((record) => record.failureLogs ?? []))],
    toolchainVersions: Object.assign({}, ...present.map((record) => record.toolchainVersions ?? {})),
  };
}

/**
 * Merges the stage records of `checkpoint` for `commit`. `records` maps a
 * stage name to its parsed record (or an Error when it could not be read).
 * Returns the groups, each carrying the toolchain versions of the stage that
 * produced it, and one error per stage that failed or did not report; a stage
 * skipped because the stage it runs after failed joins that stage's error.
 */
export function mergeStageRecords({ checkpoint, commit, records }) {
  const groups = new Map();
  const stageErrors = [];
  const fail = (stage, error) => stageErrors.push({ stage, error });
  const blocked = new Map();
  for (const stage of evidenceStages(checkpoint)) {
    const record = records.get(stage.name);
    if (record === undefined) {
      const blocking = stage.after && stageErrors.find((stageError) => stageError.stage === stage.after);
      if (blocking) {
        blocked.set(blocking, [...(blocked.get(blocking) ?? []), stage.name]);
        continue;
      }
      fail(stage.name, 'the stage produced no record; see its job log');
      continue;
    }
    if (record instanceof Error) {
      fail(stage.name, `the stage record cannot be read: ${record.message}`);
      continue;
    }
    const mismatch = [
      ['schemaVersion', 1], ['issue', 195], ['stage', stage.name], ['commit', commit], ['checkpoint', checkpoint],
    ].find(([field, expected]) => record[field] !== expected);
    if (mismatch) {
      fail(stage.name, `the stage record has ${mismatch[0]} ${record[mismatch[0]]}, expected ${mismatch[1]}`);
      continue;
    }
    if (record.error) fail(stage.name, record.error);
    const produced = record.groups && typeof record.groups === 'object' ? record.groups : {};
    for (const [group, groupRecord] of Object.entries(produced)) {
      if (!stage.groups.includes(group)) {
        fail(stage.name, `the stage reported group ${group}, which belongs to another stage`);
        continue;
      }
      groups.set(group, { ...bundleRecords(groupRecord), toolchainVersions: { ...record.toolchainVersions } });
    }
    const absent = stage.groups.filter((group) => !(group in produced));
    if (!record.error && absent.length > 0) {
      fail(stage.name, `the stage reported no record for ${absent.join(', ')}`);
    }
  }
  for (const [blocking, skipped] of blocked) {
    blocking.error += `; ${skipped.join(', ')} did not run, as ${skipped.length === 1 ? 'it runs' : 'they run'} after ${blocking.stage}`;
  }
  for (const [name] of records) {
    if (!evidenceStages(checkpoint).some((stage) => stage.name === name)) {
      fail(name, `${name} is not a ${checkpoint} evidence stage`);
    }
  }
  return { groups, stageErrors };
}

/**
 * The translation groups: each joins its runtime's suite, the runtime parity
 * observation and the target's native validation. A translation whose inputs
 * are not all present has no group, so its cells stay missing. The native
 * negative control is not the translation's failure.
 */
export function translationGroups(groups) {
  const result = new Map();
  for (const runtime of RUNTIMES) {
    for (const target of NATIVE_TARGETS) {
      const inputs = [`suite:${runtime}`, 'runtime-parity', `native:${target}:${runtime}`].map((group) => groups.get(group));
      if (inputs.some((input) => !input)) continue;
      result.set(`translation:${target}:${runtime}`, { ...bundleRecords(...inputs), failureLogs: [] });
    }
  }
  return result;
}

/** One gate error per failed stage, as the evaluator reports it. */
export function stageGateError({ stage, error }) {
  return `evidence stage ${stage} failed: ${error}`;
}

/**
 * Files in a native validation directory the cells do not cite (compiler
 * outputs such as .rmeta, .vo, .vok, .vos and .glob), which the stage deletes
 * once the target's cell is recorded.
 */
export function uncitedFiles(names, cited) {
  const keep = new Set(cited);
  return names.filter((name) => !keep.has(name)).sort();
}
