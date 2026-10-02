// The link-assistant/formal-ai workloads run against the clean installed
// meta-language artifacts (requirement I195-DOWNSTREAM-FORMAL-AI-WORKLOADS).
// js/scripts/run-formal-ai-workloads.mjs checks out the pinned revision of
// parity/fixtures/formal-ai-workloads.json and verifies every inventoried blob.
// The Rust side patches formal-ai's rust/ to the unpacked crate and runs
// formal-ai's own meta-language test groups plus the probe of
// js/scripts/issue-195-formal-ai-rust-probe.mjs. formal-ai's npm package has no
// meta-language dependency, so the JavaScript side installs the npm tarball
// into a clean consumer and runs formal-ai's LiNo data through the public API
// (js/scripts/issue-195-formal-ai-workload-probes.mjs). Both runtimes write the
// same outputs, and sharedConceptsReused holds only when they agree.
// `validateFormalAiWorkloadReport` is the pure check of the runner's report for
// one runtime, shared by the runner and the tests; the Rust suite ports it.
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const FORMAL_AI_WORKLOAD_REQUIREMENT = 'I195-DOWNSTREAM-FORMAL-AI-WORKLOADS';
export const FORMAL_AI_WORKLOAD_FIXTURE_ID = 'planned:repository-directive:i195-downstream-formal-ai-workloads';
export const FORMAL_AI_WORKLOAD_FIXTURE = 'parity/fixtures/formal-ai-workloads.json';
export const FORMAL_AI_WORKLOAD_REPORT_VARIABLE = 'ISSUE_195_FORMAL_AI_WORKLOAD_REPORT';
export const FORMAL_AI_WORKLOAD_SCHEMA_VERSION = 1;
export const FORMAL_AI_WORKLOAD_ASSERTIONS = Object.freeze([
  'workloadsRunOnInstalledArtifacts',
  'sharedConceptsReused',
  'distinctionsPreserved',
]);
// The consumer-probe assertions; `workloadsRunOnInstalledArtifacts` is read from the report itself.
export const FORMAL_AI_PROBE_ASSERTIONS = Object.freeze(FORMAL_AI_WORKLOAD_ASSERTIONS.slice(1));
// The report part of each runtime, and the inventoried workload kind and directory it runs:
// the Rust side runs formal-ai's test files, the JavaScript side formal-ai's data files.
export const FORMAL_AI_WORKLOAD_RUNTIMES = Object.freeze({
  javascript: Object.freeze({ part: 'npm', kind: 'data', directory: 'data/' }),
  rust: Object.freeze({ part: 'crate', kind: 'test', directory: 'rust/' }),
});

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

/** The pinned formal-ai fixture. */
export function loadFormalAiWorkloadFixture(directory = root) {
  return JSON.parse(readFileSync(path.join(directory, FORMAL_AI_WORKLOAD_FIXTURE), 'utf8'));
}

/** The inventoried workload files `runtime` runs, as formal-ai repository paths. */
export function workloadFiles(fixture, runtime) {
  const { kind, directory } = FORMAL_AI_WORKLOAD_RUNTIMES[runtime];
  return fixture.workloads
    .filter((entry) => entry.kind === kind && entry.file.startsWith(directory))
    .map(({ file }) => file);
}

/** Counts of test outcomes, as the runner writes them. */
export function countOutcomes(tests) {
  const count = (outcome) => tests.filter((entry) => entry.outcome === outcome).length;
  return { passed: count('passed'), failed: count('failed'), skipped: count('skipped'), total: tests.length };
}

/** formal-ai's committed census of its own sources as meta-language parses them. */
export const SELF_AST_CENSUS_DIRECTORY = 'data/meta/self-ast';
/** formal-ai's example that rewrites the census documents whose rendering changed. */
export const SELF_AST_CENSUS_EXAMPLE = 'regenerate_self_ast_census';
// The census line that counts every link of the lossless network. It is the
// only census field outside the abstract-syntax projection, the symbol scan
// and the source itself.
const LINK_TOTAL_LINE = /^ {4}total_link_count (\d+)$/u;

/**
 * Compares formal-ai's census documents before and after its own regeneration
 * with a candidate meta-language (two maps of relative path to document text).
 *
 * A candidate that adds links to the lossless network, such as a grammar
 * provenance link or hidden-text tokens, changes `total_link_count` and nothing
 * else. `linkTotals` lists those changes; `otherChanges` lists every other
 * changed line and every added or removed document, so a candidate that
 * changes a named node, a node kind, the text preservation or the clean
 * verification of any formal-ai source is reported.
 */
export function compareSelfAstCensus(before, after) {
  const linkTotals = [];
  const otherChanges = [];
  for (const file of [...new Set([...before.keys(), ...after.keys()])].sort()) {
    if (!before.has(file) || !after.has(file)) {
      otherChanges.push({ file, line: null, before: before.has(file) ? 'present' : 'absent', after: after.has(file) ? 'present' : 'absent' });
      continue;
    }
    const previous = before.get(file).split('\n');
    const current = after.get(file).split('\n');
    for (let index = 0; index < Math.max(previous.length, current.length); index += 1) {
      if (previous[index] === current[index]) continue;
      const previousTotal = LINK_TOTAL_LINE.exec(previous[index] ?? '');
      const currentTotal = LINK_TOTAL_LINE.exec(current[index] ?? '');
      if (previousTotal && currentTotal) {
        linkTotals.push({ file, before: Number(previousTotal[1]), after: Number(currentTotal[1]) });
      } else {
        otherChanges.push({ file, line: index + 1, before: previous[index] ?? null, after: current[index] ?? null });
      }
    }
  }
  return { documents: after.size, linkTotals, otherChanges };
}

function sameCounts(left, right) {
  return ['passed', 'failed', 'skipped', 'total'].every((key) => left?.[key] === right?.[key]);
}

// Problems of the pinned checkout: the revision and every inventoried blob.
function checkoutProblems(report, fixture) {
  const problems = [];
  const checkout = report?.formalAi;
  if (checkout?.repository !== fixture.repository) {
    problems.push(`the report checks out ${checkout?.repository}, not ${fixture.repository}`);
  }
  for (const key of ['revision', 'checkedOutRevision']) {
    if (checkout?.[key] !== fixture.revision) {
      problems.push(`the formal-ai ${key} is ${checkout?.[key]}, not the pinned ${fixture.revision}`);
    }
  }
  const verified = new Map((checkout?.verifiedBlobs ?? []).map((entry) => [entry.file, entry]));
  for (const { file, blob } of fixture.workloads) {
    const entry = verified.get(file);
    if (!entry) problems.push(`the workload blob of ${file} was not verified`);
    else if (entry.actual !== blob || entry.expected !== blob) {
      problems.push(`${file} has blob ${entry.actual}, the fixture pins ${blob}`);
    }
  }
  return problems;
}

// Problems of the artifact: its checksum and that the consumer resolved meta-language to it.
function artifactProblems(runtime, part) {
  const problems = [];
  const { checksum = {}, resolution = {} } = part;
  if (!/^[0-9a-f]{64}$/u.test(checksum.sha256 ?? '') || checksum.sha256 !== checksum.expectedSha256) {
    problems.push(`the ${runtime} artifact sha256 ${checksum.sha256} does not match its published checksum ${checksum.expectedSha256}`);
  }
  if (runtime === 'javascript') {
    if (typeof resolution.lockResolved !== 'string' || !resolution.lockResolved.startsWith('file:')) {
      problems.push(`meta-language resolves from ${resolution.lockResolved}, not from the installed artifact`);
    }
    if (!checksum.installedIntegrity || checksum.installedIntegrity !== checksum.expectedIntegrity) {
      problems.push(`the installed integrity ${checksum.installedIntegrity} is not the tarball's ${checksum.expectedIntegrity}`);
    }
    if (resolution.freshCache !== true || resolution.consumerHadNoModules !== true) {
      problems.push('the npm install did not start from an empty node_modules and a fresh cache');
    }
  } else {
    const unpacked = typeof resolution.unpackedDirectory === 'string' ? resolution.unpackedDirectory : null;
    if (resolution.metadataSource !== null || !unpacked ||
        typeof resolution.manifestPath !== 'string' || !resolution.manifestPath.startsWith(unpacked)) {
      problems.push(`meta-language resolves to ${resolution.manifestPath} (source ${resolution.metadataSource}), not to the unpacked crate`);
    }
    if (resolution.lockSource !== null) {
      problems.push(`Cargo.lock resolves meta-language from ${resolution.lockSource}, not from a path`);
    }
    if (resolution.metaLanguagePackages !== 1) {
      problems.push(`the formal-ai build resolves ${resolution.metaLanguagePackages} meta-language packages, not one`);
    }
  }
  if (!part.version || resolution.installedVersion !== part.version) {
    problems.push(`the consumer resolved meta-language ${resolution.installedVersion}, the artifact is ${part.version}`);
  }
  return problems;
}

// Problems of the executed workload tests: every inventoried workload file had a passing test and nothing failed.
function testProblems(runtime, part, fixture) {
  const problems = [];
  const tests = Array.isArray(part.tests) ? part.tests : [];
  const counts = countOutcomes(tests);
  if (!sameCounts(counts, part.counts)) {
    problems.push(`the ${runtime} counts ${JSON.stringify(part.counts)} do not match the executed tests ${JSON.stringify(counts)}`);
  }
  for (const entry of tests.filter(({ outcome }) => outcome === 'failed')) {
    problems.push(`the ${runtime} workload ${entry.file} › ${entry.name} failed`);
  }
  for (const file of workloadFiles(fixture, runtime)) {
    if (!tests.some((entry) => entry.file === file && entry.outcome === 'passed')) {
      problems.push(`no ${runtime} test of ${file} passed`);
    }
  }
  if (part.exit !== 0) problems.push(`the ${runtime} workload run exited with ${part.exit}`);
  return problems;
}

// Problems of one probe assertion: it holds, every check holds, and there is at least one check.
function probeProblems(runtime, part, assertion) {
  const observed = part.probe?.[assertion];
  const checks = Array.isArray(observed?.checks) ? observed.checks : [];
  if (checks.length === 0) return [`the ${runtime} probe made no ${assertion} check`];
  const problems = checks.filter(({ holds }) => holds !== true).map(({ name, detail }) =>
    `the ${runtime} ${assertion} check "${name}" failed: ${typeof detail === 'string' ? detail : JSON.stringify(detail)}`);
  if (problems.length === 0 && observed.holds !== true) {
    problems.push(`the ${runtime} probe reports ${assertion} as not holding`);
  }
  return problems;
}

/**
 * The problems of `report` for `runtime` (`javascript` reads the npm part,
 * `rust` the crate part), keyed by assertion, and the assertions without a
 * problem. A probe observation counts only when the probe ran against the
 * verified checkout and the installed artifact.
 */
export function validateFormalAiWorkloadReport(report, fixture, runtime) {
  const problems = Object.fromEntries(FORMAL_AI_WORKLOAD_ASSERTIONS.map((assertion) => [assertion, []]));
  const layout = FORMAL_AI_WORKLOAD_RUNTIMES[runtime];
  if (!layout) throw new Error(`unknown runtime ${runtime}`);
  const part = report?.[layout.part];
  if (report?.schemaVersion !== FORMAL_AI_WORKLOAD_SCHEMA_VERSION) {
    problems.workloadsRunOnInstalledArtifacts.push(
      `the report schema version is ${report?.schemaVersion}, not ${FORMAL_AI_WORKLOAD_SCHEMA_VERSION}`);
  }
  if (!part || part.skipped) {
    const message = `the report has no ${runtime} run${part?.reason ? `: ${part.reason}` : ''}`;
    for (const assertion of FORMAL_AI_WORKLOAD_ASSERTIONS) problems[assertion].push(message);
    return { problems, observed: [] };
  }
  const grounding = [...checkoutProblems(report, fixture), ...artifactProblems(runtime, part)];
  problems.workloadsRunOnInstalledArtifacts.push(...grounding, ...testProblems(runtime, part, fixture));
  for (const assertion of FORMAL_AI_PROBE_ASSERTIONS) {
    problems[assertion].push(...grounding.map((problem) => `not observed on the installed artifact: ${problem}`));
    problems[assertion].push(...probeProblems(runtime, part, assertion));
  }
  return {
    problems,
    observed: FORMAL_AI_WORKLOAD_ASSERTIONS.filter((assertion) => problems[assertion].length === 0),
  };
}
