// Source register of issue #195: the issue and every requirement-bearing
// comment of pull request #196, each with the revision (updatedAt, byte count
// and sha256 of the exact body the GitHub REST API returns) that the atomic
// ledger in parity/issue-195-requirements.json was reconciled against.
//
// Editing a registered comment changes its hash, and a comment that is neither
// registered nor recognized as automation output is reported as unregistered,
// so the ledger cannot silently keep a stale or incomplete requirement snapshot.
// The offline check (the default) validates the committed register against the
// ledger; `--online` also compares it with the live discussion.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';

export const ISSUE_195_SOURCE_REGISTER_PATH = 'parity/issue-195-sources.json';
export const ISSUE_195_SOURCE_REGISTER_SCHEMA_VERSION = 1;

const REPOSITORY = 'link-foundation/meta-language';

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
    const expectedUrl = entry.kind === 'issue'
      ? `https://github.com/${REPOSITORY}/issues/${entry.id}`
      : `https://github.com/${REPOSITORY}/pull/196#issuecomment-${entry.id}`;
    if (entry.url !== expectedUrl) errors.push(`source ${label} url must be ${expectedUrl}`);
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
 * Fetches the issue with its comments and every conversation comment, inline
 * review comment and review of pull request #196.
 */
export async function fetchLiveDiscussion() {
  const token = githubToken();
  const [issue, issueComments, conversation, reviewComments, reviews, remainingScope] = await Promise.all([
    githubGet(`repos/${REPOSITORY}/issues/195`, token),
    githubGet(`repos/${REPOSITORY}/issues/195/comments?per_page=100`, token),
    githubGet(`repos/${REPOSITORY}/issues/196/comments?per_page=100`, token),
    githubGet(`repos/${REPOSITORY}/pulls/196/comments?per_page=100`, token),
    githubGet(`repos/${REPOSITORY}/pulls/196/reviews?per_page=100`, token),
    githubGet(`repos/${REPOSITORY}/issues/199`, token),
  ]);
  const reviewBodies = reviews
    .filter((review) => (review.body ?? '').trim() !== '')
    .map((review) => ({ ...review, created_at: review.submitted_at, updated_at: review.submitted_at }));
  return { issue, issues: [remainingScope], comments: [...issueComments, ...conversation, ...reviewComments, ...reviewBodies] };
}
