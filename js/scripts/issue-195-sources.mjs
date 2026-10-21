// Source register of issue #195: the issue and every requirement-bearing
// comment of the original and remaining-scope discussions, each with the revision (updatedAt, byte count
// and sha256 of the exact body the GitHub REST API returns) that the atomic
// ledger in parity/issue-195-requirements.json was reconciled against.
//
// Editing a registered comment changes its hash, and a comment that is neither
// registered nor recognized as automation output is reported as unregistered,
// so the ledger cannot silently keep a stale or incomplete requirement snapshot.
// The offline check (the default) validates the committed register against the
// ledger and captured direct instructions; `--online` also compares it with
// the live issue #195/#199 and pull request #196/#201 discussions.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const ISSUE_195_SOURCE_REGISTER_PATH = 'parity/issue-195-sources.json';
export const ISSUE_195_SOURCE_REGISTER_SCHEMA_VERSION = 1;

const REPOSITORY = 'link-foundation/meta-language';
export const TRACKED_ISSUES = Object.freeze([195, 199, 224]);
export const TRACKED_PULL_REQUESTS = Object.freeze([196, 201, 226]);
const REPOSITORY_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

// Actual direct conversation captures. Appending these declarations never
// rewrites an earlier live source, capture, timestamp or ledger mapping.
export const ASSERTION_POLICY_INSTRUCTION_SOURCES = Object.freeze([
  {
    id: 'modern-javascript-parity-expectations-2026-10-16',
    key: 'modernJavaScriptParityRequest',
    title: 'Use newer JavaScript behavior and correct obsolete native expectations',
    role: 'Explicit user resolution of the retained Rust versus JavaScript string length and array bounds expectation conflict, with the exact question and reply.',
    ledgerCoverage: ['I195-PARITY-MODERN-JAVASCRIPT-ARRAY-BEHAVIOR', 'I195-PARITY-OBSOLETE-NATIVE-EXPECTATIONS'],
  },
  {
    id: 'compact-contributor-instructions-and-assertion-corrections-2026-10-16',
    key: 'compactContributorInstructionsRequest',
    title: 'Minimize contributor instructions and permit evidence based assertion corrections',
    role: 'Direct user request to shorten AGENTS.md and CLAUDE.md without losing obligations and to allow genuinely wrong assertions to be fixed.',
    ledgerCoverage: ['I195-CONTRIBUTOR-INSTRUCTIONS-CONCISE', 'I195-ASSERTION-EVIDENCE-BASED-CORRECTION'],
  },
  {
    id: 'version-bounded-assertions-2026-10-16',
    key: 'versionBoundedAssertionsRequest',
    title: 'Bound version dependent assertions to the actual active language or dependency version',
    role: 'Direct user requirement for explicit applicable version ranges on assertions that depend on language or dependency versions.',
    ledgerCoverage: ['I195-ASSERTION-EXPLICIT-VERSION-RANGES', 'I195-ASSERTION-VERSION-COVERAGE', 'I195-ASSERTION-LANGUAGE-EDITION-ORDER'],
  },
  {
    id: 'solo-continuation-2026-10-19',
    key: 'soloContinuationRequest',
    title: 'Finish existing agent work and continue only as coordinator',
    role: 'Latest user direction to let active subagents finish slowly, then continue implementation and CI follow-up alone without starting or reactivating agents.',
    ledgerCoverage: ['I195-WORKFLOW-GRADUAL-AGENT-REDUCTION'],
  },
  {
    id: 'continuous-bulk-completion-2026-10-19',
    key: 'continuousBulkCompletionRequest',
    title: 'Fully draft and deliver the authorized scope while CI executes',
    role: 'Direct correction that opening a continuation PR does not deliver the task; keep drafting all outstanding changes during CI and carry the complete scope through implementation and verification.',
    ledgerCoverage: ['I195-WORKFLOW-FAST-CONTINUOUS-PROGRESS', 'I195-WORKFLOW-BULK-EXACT-SCOPE'],
  },
  {
    id: 'continue-unmet-requirements-during-ci-2026-10-19',
    key: 'continueUnmetRequirementsDuringCiRequest',
    title: 'Implement unmet requirements and repair CI during the next run',
    role: 'Direct user continuation instruction to implement all unmet requirements and CI repairs while awaiting the next CI result; retain the complete scope and continue working during verification.',
    ledgerCoverage: ['I195-WORKFLOW-FAST-CONTINUOUS-PROGRESS', 'I195-WORKFLOW-BULK-EXACT-SCOPE'],
  },
  {
    id: 'automation-first-own-codebase-baseline-2026-10-21',
    key: 'automationFirstOwnCodebaseBaselineRequest',
    title: 'Automate high coverage work and prioritize complete JavaScript Rust translation of this codebase',
    role: 'Direct user priority for automation and twenty percent resource investment toward eighty percent results, up to three subagents in the current phase, and full JavaScript and Rust parsing, emission and bidirectional own-codebase translation as the baseline before the remaining capabilities.',
    ledgerCoverage: ['I195-WORKFLOW-AUTOMATION-FIRST-COVERAGE', 'I195-WORKFLOW-THREE-SUBAGENT-CURRENT-PHASE', 'I195-TRANSLATION-OWN-CODEBASE-BASELINE'],
  },
  {
    id: 'solo-continuation-2026-10-21',
    key: 'latestSoloContinuationRequest',
    title: 'Finish all existing agents and continue the current phase alone',
    role: 'Latest direct user instruction supersedes the earlier three-agent allowance for this task phase: allow existing agents to finish and continue as coordinator without spawning or reactivating agents.',
    ledgerCoverage: ['I195-WORKFLOW-GRADUAL-AGENT-REDUCTION'],
  },
].map(source => Object.freeze({ ...source, ledgerCoverage: Object.freeze(source.ledgerCoverage) })));

/** Prepare append-only registrations from the actual captured UTF-8 bytes. */
export async function appendAssertionPolicyInstructionSources(register, { root = REPOSITORY_ROOT, capturedAt = new Date().toISOString(), read = readFile } = {}) {
  if (!Array.isArray(register?.requirementSources)) throw new TypeError('a requirement source register is required');
  if (typeof capturedAt !== 'string' || !Number.isFinite(Date.parse(capturedAt))) {
    throw new TypeError('an actual capture time is required; the conversation exposes no source timestamp');
  }
  const requirementSources = [...register.requirementSources];
  for (const declaration of ASSERTION_POLICY_INSTRUCTION_SOURCES) {
    const url = `parity/sources/${declaration.id}.txt`;
    const body = await read(path.join(root, url), 'utf8');
    const captured = { ...declaration, kind: 'direct-instruction', url, author: 'user',
      createdAt: capturedAt, updatedAt: capturedAt,
      revisionOrigin: 'capture time; the conversation does not expose a source timestamp',
      bytes: byteLength(body), sha256: contentHash(body) };
    const matches = registeredEntries(register).filter(entry => entry.id === declaration.id || entry.url === url || entry.key === declaration.key);
    if (matches.length) {
      if (matches.length !== 1 || matches[0].classification !== 'requirement') throw new Error(`conflicting captured source ${declaration.key}`);
      const current = matches[0];
      for (const field of ['id', 'key', 'kind', 'url', 'title', 'role', 'revisionOrigin', 'bytes', 'sha256']) {
        if (current[field] !== captured[field]) throw new Error(`captured source ${declaration.key} changed ${field}`);
      }
      if (JSON.stringify(current.ledgerCoverage) !== JSON.stringify(captured.ledgerCoverage)) {
        throw new Error(`captured source ${declaration.key} changed its original ledger coverage`);
      }
      continue;
    }
    requirementSources.push(captured);
  }
  return { ...register, requirementSources };
}

// Openings of the comments that the solver automation posts (session logs,
// restart, usage-limit and resume notices and readiness markers); they carry
// no requirements.
export const AUTOMATION_MARKERS = Object.freeze([
  '<!-- hive-mind',
  '## 🤖',
  '🤖 **AI Work Session',
  '## 🔄 Auto-restart',
  '## 🔄 Solution Draft Log',
  '## ❌ Auto-restart',
  '## ⏳ Usage Limit Reached',
  '⏰ **Auto Resume',
  '## ⏰ Auto Resume',
  '## 🚨 Solution Draft Failed',
  '## ✅ Ready to merge',
  '## 📎 Intermediate working-session log',
  '## 🛑 Automation stopped',
]);

export function contentHash(body) {
  return createHash('sha256').update(body ?? '', 'utf8').digest('hex');
}

export function byteLength(body) {
  return Buffer.byteLength(body ?? '', 'utf8');
}

/** The first heading of a body, or its first line. */
export function sourceTitle(body) {
  const lines = (body ?? '').split(/\r?\n/).map((line) => line.trim()).filter(Boolean);
  const heading = lines.find((line) => line.startsWith('#'));
  return (heading ?? lines[0] ?? '').replace(/^#+\s*/, '').slice(0, 160);
}

export function isAutomationComment(body) {
  const opening = (body ?? '').trimStart();
  return AUTOMATION_MARKERS.some((marker) => opening.startsWith(marker));
}

/** The revision fields of a live issue or comment. */
export function revisionOf(live) {
  return {
    author: live.user?.login ?? null,
    createdAt: live.created_at,
    updatedAt: live.updated_at,
    bytes: byteLength(live.body),
    sha256: contentHash(live.body),
  };
}

function registeredEntries(register) {
  return [
    ...(register.requirementSources ?? []).map((entry) => ({ ...entry, classification: 'requirement' })),
    ...(register.statusReports ?? []).map((entry) => ({ ...entry, classification: 'status-report' })),
  ];
}

/**
 * Checks the committed register against the ledger without network access:
 * every ledger source and ISSUE_195_SOURCES permalink is registered, every
 * registered requirement source covers existing ledger rows, and every row is
 * covered by the source it names.
 */
export function validateSourceRegister(register, manifest, sources) {
  const errors = [];
  if (register.schemaVersion !== ISSUE_195_SOURCE_REGISTER_SCHEMA_VERSION) {
    errors.push(`schemaVersion must be ${ISSUE_195_SOURCE_REGISTER_SCHEMA_VERSION}`);
  }
  if (register.repository !== REPOSITORY) errors.push(`repository must be ${REPOSITORY}`);
  const byUrl = new Map();
  const ids = new Set();
  for (const entry of registeredEntries(register)) {
    const label = entry.key ?? entry.id;
    if (ids.has(entry.id)) errors.push(`source ${entry.id} is registered twice`);
    ids.add(entry.id);
    byUrl.set(entry.url, entry);
    for (const field of ['id', 'kind', 'url', 'createdAt', 'updatedAt', 'bytes', 'sha256', 'title']) {
      if (entry[field] === undefined || entry[field] === null || entry[field] === '') {
        errors.push(`source ${label} is missing ${field}`);
      }
    }
    if (!/^[0-9a-f]{64}$/.test(entry.sha256 ?? '')) errors.push(`source ${label} has no sha256 content hash`);
    if (entry.kind === 'issue') {
      const expectedUrl = `https://github.com/${REPOSITORY}/issues/${entry.id}`;
      if (entry.url !== expectedUrl) errors.push(`source ${label} url must be ${expectedUrl}`);
    } else if (entry.kind === 'comment') {
      const discussion = /^https:\/\/github\.com\/link-foundation\/meta-language\/(pull|issues)\/(\d+)#issuecomment-(\d+)$/u.exec(entry.url ?? '');
      const tracked = discussion?.[1] === 'pull' ? TRACKED_PULL_REQUESTS : TRACKED_ISSUES;
      if (!discussion || !tracked.includes(Number(discussion[2])) || discussion[3] !== String(entry.id)) {
        errors.push(`source ${label} must cite its exact comment in a tracked issue or pull request`);
      }
    } else if (entry.kind === 'direct-instruction') {
      if (entry.url !== `parity/sources/${entry.id}.txt` || !/^[a-z0-9-]+$/u.test(entry.id)) {
        errors.push(`source ${label} must cite its captured direct instruction in parity/sources`);
      }
      if (entry.revisionOrigin !== 'capture time; the conversation does not expose a source timestamp') {
        errors.push(`source ${label} must identify its timestamps as capture metadata`);
      }
    } else {
      errors.push(`source ${label} has unknown kind ${entry.kind}`);
    }
  }
  const rows = manifest.atomicRequirements ?? [];
  for (const entry of register.requirementSources ?? []) {
    if (!entry.key || !entry.role) errors.push(`requirement source ${entry.id} needs a key and a role`);
    if (sources[entry.key] !== entry.url) {
      errors.push(`requirement source ${entry.key} is not the ledger source ISSUE_195_SOURCES.${entry.key}`);
    }
    if (!Array.isArray(entry.ledgerCoverage) || entry.ledgerCoverage.length === 0) {
      errors.push(`requirement source ${entry.key} maps to no ledger rows`);
      continue;
    }
    for (const prefix of entry.ledgerCoverage) {
      if (!rows.some((row) => row.id.startsWith(prefix))) {
        errors.push(`requirement source ${entry.key} maps to ${prefix}, which matches no ledger row`);
      }
    }
  }
  for (const [key, url] of Object.entries(sources)) {
    const entry = byUrl.get(url);
    if (!entry || entry.classification !== 'requirement') {
      errors.push(`ledger source ISSUE_195_SOURCES.${key} is not a registered requirement source`);
    }
  }
  for (const row of rows) {
    const entry = byUrl.get(row.source);
    if (!entry || entry.classification !== 'requirement') {
      errors.push(`${row.id} cites unregistered source ${row.source}`);
      continue;
    }
    if (!entry.ledgerCoverage?.some((prefix) => row.id.startsWith(prefix))) {
      errors.push(`${row.id} is not covered by the ledgerCoverage of its source ${entry.key}`);
    }
  }
  return errors;
}

/**
 * Compares the register with the live issue and pull request comments: an
 * edited or deleted registered source and an unregistered, non-automation
 * comment are errors.
 */
export function compareWithLiveDiscussion(register, { issue, issues = [], comments }) {
  const errors = [];
  const live = new Map([[String(issue.number ?? 195), { ...issue, kind: 'issue' }]]);
  for (const additional of issues) live.set(String(additional.number), { ...additional, kind: 'issue' });
  for (const comment of comments) live.set(String(comment.id), { ...comment, kind: 'comment' });
  const registered = new Map(registeredEntries(register).map((entry) => [String(entry.id), entry]));
  for (const [id, entry] of registered) {
    if (entry.kind === 'direct-instruction') continue;
    const current = live.get(id);
    if (!current) {
      errors.push(`registered ${entry.classification} ${entry.url} no longer exists`);
      continue;
    }
    const revision = revisionOf(current);
    if (revision.sha256 !== entry.sha256 || revision.updatedAt !== entry.updatedAt) {
      errors.push(
        `${entry.classification} ${entry.url} was edited at ${revision.updatedAt} ` +
          `(sha256 ${revision.sha256.slice(0, 12)}, registered ${entry.sha256.slice(0, 12)} at ${entry.updatedAt}); ` +
          're-read it, reconcile the ledger and refresh the register',
      );
    }
  }
  for (const comment of comments) {
    if (registered.has(String(comment.id)) || isAutomationComment(comment.body)) continue;
    errors.push(
      `comment ${comment.html_url} (${sourceTitle(comment.body)}) is not registered; ` +
        'classify it as a requirement source or a status report',
    );
  }
  return errors;
}

/** Rewrites the revision fields of the registered entries from the live discussion. */
export function refreshRegister(register, { issue, issues = [], comments }) {
  const live = new Map([[String(issue.number ?? 195), issue]]);
  for (const additional of issues) live.set(String(additional.number), additional);
  for (const comment of comments) live.set(String(comment.id), comment);
  const refresh = (entry) => {
    const current = live.get(String(entry.id));
    if (!current) return entry;
    return { ...entry, ...revisionOf(current), title: entry.title || sourceTitle(current.body) };
  };
  return {
    ...register,
    requirementSources: register.requirementSources.map(refresh),
    statusReports: register.statusReports.map(refresh),
  };
}

/** Captured conversation instructions are checked from their committed bytes. */
export async function validateDirectInstructions(register, root = REPOSITORY_ROOT) {
  const errors = [];
  for (const entry of registeredEntries(register)) {
    if (entry.kind !== 'direct-instruction') continue;
    if (entry.url !== `parity/sources/${entry.id}.txt` || !/^[a-z0-9-]+$/u.test(entry.id)) continue;
    try {
      const body = await readFile(path.join(root, entry.url), 'utf8');
      if (contentHash(body) !== entry.sha256 || byteLength(body) !== entry.bytes) {
        errors.push(`direct instruction ${entry.url} differs from its registered content hash or byte count`);
      }
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
      errors.push(`registered direct instruction ${entry.url} no longer exists`);
    }
  }
  return errors;
}

function githubToken() {
  if (process.env.GITHUB_TOKEN) return process.env.GITHUB_TOKEN;
  if (process.env.GH_TOKEN) return process.env.GH_TOKEN;
  try {
    return execFileSync('gh', ['auth', 'token'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim();
  } catch {
    return null;
  }
}

async function githubGet(pathname, token) {
  const results = [];
  let url = `https://api.github.com/${pathname}`;
  while (url) {
    const response = await fetch(url, {
      headers: {
        Accept: 'application/vnd.github+json',
        'User-Agent': 'meta-language-issue-195-sources',
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
      },
    });
    if (!response.ok) throw new Error(`GET ${url} failed with ${response.status}`);
    const page = await response.json();
    if (!Array.isArray(page)) return page;
    results.push(...page);
    url = /<([^>]+)>;\s*rel="next"/.exec(response.headers.get('link') ?? '')?.[1] ?? null;
  }
  return results;
}

/**
 * Fetches every tracked issue and its comments, and every conversation comment,
 * inline review comment and review of the original and follow-up pull requests.
 * The injectable request keeps the endpoint coverage test independent of GitHub.
 */
export async function fetchLiveDiscussion({ request = githubGet } = {}) {
  const token = githubToken();
  const [issues, issueComments, pullRequestComments] = await Promise.all([
    Promise.all(TRACKED_ISSUES.map((number) => request(`repos/${REPOSITORY}/issues/${number}`, token))),
    Promise.all(TRACKED_ISSUES.map((number) => request(`repos/${REPOSITORY}/issues/${number}/comments?per_page=100`, token))),
    Promise.all(TRACKED_PULL_REQUESTS.map(async (number) => {
      const [conversation, reviewComments, reviews] = await Promise.all([
        request(`repos/${REPOSITORY}/issues/${number}/comments?per_page=100`, token),
        request(`repos/${REPOSITORY}/pulls/${number}/comments?per_page=100`, token),
        request(`repos/${REPOSITORY}/pulls/${number}/reviews?per_page=100`, token),
      ]);
      const reviewBodies = reviews.filter((review) => (review.body ?? '').trim() !== '')
        .map((review) => ({ ...review, created_at: review.submitted_at, updated_at: review.submitted_at }));
      return [...conversation, ...reviewComments, ...reviewBodies];
    })),
  ]);
  return { issue: issues[0], issues: issues.slice(1), comments: [...issueComments.flat(), ...pullRequestComments.flat()] };
}
