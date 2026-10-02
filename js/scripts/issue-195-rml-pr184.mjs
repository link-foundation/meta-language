#!/usr/bin/env node
// The audit of the current relative-meta-logic pull request 184
// (docs/downstream-consumers.md, "Current RML pull request 184"). The fixture
// parity/fixtures/rml-pr184-workloads.json pins the pull request head and
// inventories the files of that revision that reach meta-language: the
// sources that import the package or a source that does, and the tests that
// import such a source. `validateRmlPr184Audit` checks that the audit section
// pins the same head, links every inventoried file at that head, and maps
// every workload to existing ledger rows with an implementation and
// executable acceptance cells. `readRmlPr184` re-derives the inventory from
// GitHub, so the fixture is checked against the live pull request rather
// than against itself.
//
// Usage:
//   node js/scripts/issue-195-rml-pr184.mjs            check the fixture and the audit section
//   node js/scripts/issue-195-rml-pr184.mjs --online   also re-derive the inventory from GitHub
//   node js/scripts/issue-195-rml-pr184.mjs --write <checkout>
//                                                      rewrite the fixture from a checkout of the head
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const RML_PR184_FIXTURE = 'parity/fixtures/rml-pr184-workloads.json';
export const RML_PR184_SECTION = 'Current RML pull request 184';
export const RML_REPOSITORY = 'link-foundation/relative-meta-logic';
export const RML_PULL_REQUEST = 184;

const CONSUMER_MATRIX = 'docs/downstream-consumers.md';
// The directories whose files can reach meta-language, and the name the Rust tests import the crate by.
const SCANNED = /^(?:js\/(?:src|tests)\/[^/]+\.m?js|rust\/(?:src|tests)\/.+\.rs)$/u;
const RUST_CRATE = 'rml';

/** The git blob id of `bytes`, as `git hash-object` computes it. */
export function gitBlob(bytes) {
  const buffer = Buffer.from(bytes);
  return createHash('sha1').update(`blob ${buffer.length}\0`).update(buffer).digest('hex');
}

// The repository files a JavaScript file imports, and whether it imports the package itself.
function javascriptImports(file, text) {
  const specifiers = [...text.matchAll(/(?:\bfrom\s*|\bimport\s*\(?\s*)['"]([^'"]+)['"]/gu)].map(([, specifier]) => specifier);
  return {
    direct: specifiers.includes('meta-language'),
    local: specifiers.filter((specifier) => specifier.startsWith('.'))
      .map((specifier) => path.posix.normalize(path.posix.join(path.posix.dirname(file), specifier))),
  };
}

// The crate modules a Rust file names, and whether it names the meta_language crate.
function rustImports(file, text) {
  const prefix = file.startsWith('rust/tests/') ? RUST_CRATE : 'crate';
  const modules = [...text.matchAll(new RegExp(`\\b${prefix}::(\\w+)`, 'gu'))].map(([, name]) => name);
  return {
    direct: /\bmeta_language::/u.test(text),
    local: modules.flatMap((name) => [`rust/src/${name}.rs`, `rust/src/${name}/mod.rs`]),
  };
}

/**
 * The files of `files` (a map from repository path to its bytes or text) that reach
 * meta-language: a source reaches it when it imports the package or a source
 * that reaches it; a test, when it imports such a source.
 */
export function inventoryWorkloads(files) {
  const imports = new Map([...files].filter(([file]) => SCANNED.test(file))
    .map(([file, bytes]) => [file, (file.endsWith('.rs') ? rustImports : javascriptImports)(file, String(bytes))]));
  const reaching = new Set([...imports].filter(([, { direct }]) => direct).map(([file]) => file));
  for (let grown = true; grown;) {
    grown = false;
    for (const [file, { local }] of imports) {
      if (!reaching.has(file) && local.some((target) => reaching.has(target))) {
        reaching.add(file);
        grown = true;
      }
    }
  }
  return [...reaching].sort().map((file) => ({
    file,
    kind: /^(?:js|rust)\/tests\//u.test(file) ? 'test' : 'source',
    blob: gitBlob(files.get(file)),
  }));
}

// The cells of a Markdown table row, with `|` inside backticks kept.
function cells(line) {
  const result = [];
  let current = '';
  let code = false;
  for (const character of line.trim().replace(/^\||\|$/gu, '')) {
    if (character === '`') code = !code;
    if (character === '|' && !code) {
      result.push(current.trim());
      current = '';
    } else {
      current += character;
    }
  }
  result.push(current.trim());
  return result;
}

const codeSpans = (text) => [...text.matchAll(/`([^`]+)`/gu)].map(([, span]) => span);
const blobUrl = /^https:\/\/github\.com\/([^/]+\/[^/]+)\/blob\/([0-9a-f]{40})\/(.+)$/u;

/** The audit section of the consumer matrix: the head it pins, its link definitions and its workload table. */
export function parseRmlPr184Audit(markdown) {
  const heading = `### ${RML_PR184_SECTION}\n`;
  const start = markdown.indexOf(heading);
  if (start < 0) return null;
  const rest = markdown.slice(start + heading.length);
  const end = rest.search(/^#{2,3} /mu);
  const section = end < 0 ? rest : rest.slice(0, end);
  const scope = markdown.split(/^## /mu).find((part) => part.startsWith('Audit scope')) ?? '';
  const scoped = scope.split('\n- ').find((item) => item.includes(`/${RML_REPOSITORY}/pull/${RML_PULL_REQUEST})`));
  const links = new Map([...section.matchAll(/^\[([^\]]+)\]: (\S+)$/gmu)].map(([, label, url]) => [label, url]));
  const rows = section.split('\n').filter((line) => line.startsWith('|')).slice(2);
  return {
    text: section,
    revision: section.match(/`([0-9a-f]{40})`/u)?.[1] ?? null,
    scopeRevision: scoped?.match(/head `([0-9a-f]{40})`/u)?.[1] ?? null,
    links,
    mappings: rows.map((line) => {
      const [usage, capability, ledger, tests, status] = cells(line);
      return {
        usage,
        labels: [...`${usage} ${capability} ${tests}`.matchAll(/\]\[([^\]]+)\]/gu)].map(([, label]) => label),
        rows: codeSpans(ledger).filter((span) => span.startsWith('I195-')),
        tests: codeSpans(tests).filter((span) => span.includes('/')),
        status,
      };
    }),
  };
}

/**
 * The problems of the audit against the fixture, by assertion:
 * `currentPullRequestRevisionPinned`, `actualWorkloadsInventoried` and
 * `eachWorkloadMappedToExecutableAcceptance`. `requirements` maps a ledger
 * row id to its manifest entry.
 */
export function validateRmlPr184Audit(audit, fixture, { requirements, fileExists }) {
  const problems = {
    currentPullRequestRevisionPinned: [],
    actualWorkloadsInventoried: [],
    eachWorkloadMappedToExecutableAcceptance: [],
  };
  const pinned = problems.currentPullRequestRevisionPinned;
  if (audit === null) {
    for (const list of Object.values(problems)) list.push(`${CONSUMER_MATRIX} has no "${RML_PR184_SECTION}" section`);
    return problems;
  }
  if (fixture.repository !== RML_REPOSITORY || fixture.pullRequest !== RML_PULL_REQUEST) {
    pinned.push(`${RML_PR184_FIXTURE} records ${fixture.repository}#${fixture.pullRequest}`);
  }
  if (!/^[0-9a-f]{40}$/u.test(fixture.headRevision ?? '')) pinned.push(`${RML_PR184_FIXTURE} pins no head revision`);
  if (audit.revision !== fixture.headRevision) {
    pinned.push(`the audit section inspects ${audit.revision}, not the pinned head ${fixture.headRevision}`);
  }
  if (audit.scopeRevision !== fixture.headRevision) {
    pinned.push(`the audit scope names the head ${audit.scopeRevision}, not ${fixture.headRevision}`);
  }
  for (const [runtime, pin] of Object.entries(fixture.dependencies ?? {})) {
    if (!pin || !audit.text.includes(pin)) pinned.push(`the audit section does not state the ${runtime} meta-language pin ${pin}`);
  }
  const linked = new Map();
  for (const [label, url] of audit.links) {
    const [, repository, revision, file] = url.match(blobUrl) ?? [];
    if (repository !== RML_REPOSITORY) continue;
    if (revision !== fixture.headRevision) pinned.push(`[${label}] points at ${revision}, not the pinned head`);
    else linked.set(label, file);
  }

  const inventoried = problems.actualWorkloadsInventoried;
  const workloads = fixture.workloads ?? [];
  for (const kind of ['source', 'test']) {
    for (const runtime of ['js/', 'rust/']) {
      if (!workloads.some((workload) => workload.kind === kind && workload.file.startsWith(runtime))) {
        inventoried.push(`${RML_PR184_FIXTURE} inventories no ${runtime.slice(0, -1)} ${kind}`);
      }
    }
  }
  const mapped = new Set(audit.mappings.flatMap(({ labels }) => labels.map((label) => linked.get(label))));
  for (const { file } of workloads) {
    if (!mapped.has(file)) inventoried.push(`no workload of the audit links ${file} at the pinned head`);
  }
  for (const { usage, labels } of audit.mappings) {
    for (const label of labels) {
      if (!audit.links.has(label)) inventoried.push(`${usage.slice(0, 60)} uses the undefined link [${label}]`);
    }
  }

  const executable = problems.eachWorkloadMappedToExecutableAcceptance;
  if (audit.mappings.length === 0) executable.push('the audit section maps no workload');
  for (const { usage, rows, tests } of audit.mappings) {
    const label = usage.slice(0, 60);
    const entries = rows.map((id) => requirements.get(id));
    rows.forEach((id, index) => {
      const entry = entries[index];
      if (entry === undefined) {
        executable.push(`${label} maps the missing ledger row ${id}`);
        return;
      }
      // A row is executable acceptance when every runtime it requires has a verification with an evidence artifact.
      const verified = new Set((entry.verifications ?? []).filter(({ evidenceArtifact }) => evidenceArtifact)
        .map(({ runtime }) => runtime));
      const missing = (entry.requiredRuntimes ?? []).filter((runtime) => !verified.has(runtime));
      if (verified.size === 0 || missing.length > 0) {
        executable.push(`${label} maps ${id}, which has no executable acceptance cell for ${missing.join(', ') || 'any runtime'}`);
      }
      if (!('implementationEntryPoints' in entry)) executable.push(`${label} maps ${id}, which declares no implementation entry points`);
    });
    if (rows.length === 0) executable.push(`${label} maps no ledger row`);
    for (const test of tests) if (!fileExists(test)) executable.push(`${label} maps the missing test ${test}`);
  }
  return problems;
}

async function githubJson(pathname, fetchImpl) {
  const token = process.env.GITHUB_TOKEN ?? process.env.GH_TOKEN ?? (() => {
    try {
      return execFileSync('gh', ['auth', 'token'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim();
    } catch {
      return null;
    }
  })();
  const response = await fetchImpl(`https://api.github.com/${pathname}`, {
    headers: {
      Accept: 'application/vnd.github+json',
      'User-Agent': 'meta-language-issue-195-rml-pr184-audit',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
  });
  if (!response.ok) throw new Error(`GET ${pathname} failed with ${response.status}`);
  return response.json();
}

/**
 * The live pull request: its current head and the workloads of `revision`,
 * re-derived from the files of that revision on GitHub.
 */
export async function readRmlPr184(revision, { fetch: fetchImpl = fetch } = {}) {
  const pull = await githubJson(`repos/${RML_REPOSITORY}/pulls/${RML_PULL_REQUEST}`, fetchImpl);
  const tree = await githubJson(`repos/${RML_REPOSITORY}/git/trees/${revision}?recursive=1`, fetchImpl);
  if (tree.truncated) throw new Error(`the tree of ${revision} is truncated`);
  const scanned = tree.tree.filter(({ type, path: file }) => type === 'blob' && SCANNED.test(file));
  const files = new Map();
  for (let index = 0; index < scanned.length; index += 8) {
    await Promise.all(scanned.slice(index, index + 8).map(async ({ path: file, sha }) => {
      const response = await fetchImpl(`https://raw.githubusercontent.com/${RML_REPOSITORY}/${revision}/${file}`);
      if (!response.ok) throw new Error(`GET ${file} at ${revision} failed with ${response.status}`);
      const bytes = Buffer.from(await response.arrayBuffer());
      if (gitBlob(bytes) !== sha) throw new Error(`${file} at ${revision} does not match its tree blob ${sha}`);
      files.set(file, bytes);
    }));
  }
  return { head: pull.head.sha, state: pull.state, workloads: inventoryWorkloads(files) };
}

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

/** The fixture, the audit section and the ledger rows of this repository. */
export function loadRmlPr184Audit(directory = root) {
  const manifest = JSON.parse(readFileSync(path.join(directory, 'parity/issue-195-requirements.json'), 'utf8'));
  return {
    fixture: JSON.parse(readFileSync(path.join(directory, RML_PR184_FIXTURE), 'utf8')),
    audit: parseRmlPr184Audit(readFileSync(path.join(directory, CONSUMER_MATRIX), 'utf8')),
    context: {
      requirements: new Map(manifest.atomicRequirements.map((row) => [row.id, row])),
      fileExists: (relative) => existsSync(path.join(directory, relative)),
    },
  };
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const argv = process.argv.slice(2);
  if (argv[0] === '--write') {
    const checkout = path.resolve(argv[1] ?? '.');
    const git = (...args) => execFileSync('git', ['-C', checkout, ...args], { encoding: 'utf8' }).trim();
    const files = new Map(git('ls-files').split('\n').filter((file) => SCANNED.test(file))
      .map((file) => [file, readFileSync(path.join(checkout, file))]));
    const pin = (file, pattern) => readFileSync(path.join(checkout, file), 'utf8').match(pattern)?.[1] ?? null;
    const fixture = {
      description: 'The files of the current relative-meta-logic pull request 184 head that reach meta-language: sources that import the package or a source that does, and the tests that import them. Regenerate with node js/scripts/issue-195-rml-pr184.mjs --write <checkout of the head>.',
      repository: RML_REPOSITORY,
      pullRequest: RML_PULL_REQUEST,
      headRevision: git('rev-parse', 'HEAD'),
      dependencies: {
        javascript: pin('js/package.json', /"meta-language":\s*"([^"]+)"/u),
        rust: pin('rust/Cargo.toml', /^meta-language\s*=\s*"([^"]+)"/mu),
      },
      workloads: inventoryWorkloads(files),
    };
    writeFileSync(path.join(root, RML_PR184_FIXTURE), `${JSON.stringify(fixture, null, 2)}\n`);
    console.log(`wrote ${fixture.workloads.length} workloads of ${fixture.headRevision} to ${RML_PR184_FIXTURE}`);
  } else {
    const { fixture, audit, context } = loadRmlPr184Audit();
    const errors = Object.values(validateRmlPr184Audit(audit, fixture, context)).flat();
    if (argv.includes('--online')) {
      const live = await readRmlPr184(fixture.headRevision);
      if (live.head !== fixture.headRevision) errors.push(`pull request ${RML_PULL_REQUEST} is now at ${live.head}`);
      if (JSON.stringify(live.workloads) !== JSON.stringify(fixture.workloads)) {
        errors.push(`the workloads of ${fixture.headRevision} on GitHub differ from ${RML_PR184_FIXTURE}`);
      }
    }
    if (errors.length > 0) {
      console.error(`RML pull request ${RML_PULL_REQUEST} audit check failed:\n- ${errors.join('\n- ')}`);
      process.exit(1);
    }
    console.log(`RML pull request ${RML_PULL_REQUEST} audit OK: ${fixture.workloads.length} workloads of ${fixture.headRevision}`);
  }
}
