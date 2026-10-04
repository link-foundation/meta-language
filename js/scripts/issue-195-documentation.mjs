// Documentation consistency (docs/vision.md#documentation-consistency).
//
// Audits every documentation file, package README, changelog fragment, case
// study, the generated ledger, the pull request description and the release
// reports against the requirement ledger:
// - no text presents future emitters, approximate round trips, external
//   production parsers or an incomplete scope as the finished contract while
//   the ledger rows that would deliver it are unimplemented, failing or not
//   yet recorded passing by an acceptance report;
// - no text states or implies that issue #195 is complete, or reports every
//   requirement as passing, before every requirement is verified and delivered;
// - no text carries an issue-closing directive for #195 before then;
// - every case study carries the historical label.
//
// A statement is audited sentence by sentence. A sentence that names a gap, a
// target, a condition or a negation is not a claim; fenced code is not read.
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import path from 'node:path';

export const REPOSITORY = 'link-foundation/meta-language';
export const ISSUE = 195;
export const PULL_REQUEST = 196;

/** The label every case study README carries near its top. */
export const HISTORICAL_LABEL = '> **Historical case study.**';
export const CASE_STUDIES = 'docs/case-studies';
/** Accepted and rejected statements the check is tested against. */
export const DOCUMENTATION_FIXTURES = 'parity/documentation/claim-fixtures.json';
const LABEL_WINDOW = 5;

/** Third-party files that are not this repository's documentation. */
const THIRD_PARTY = [/(^|\/)vendor\//u, /(^|\/)node_modules\//u, /^rust\/benches\/corpora\//u];

/** Words that make a sentence a gap, a target, a condition or a negation instead of a claim. */
const QUALIFIERS = new RegExp(
  `\\b(?:${[
    'not', 'no longer', 'never', 'yet', 'until', 'unless', 'once', 'when', 'if', 'only', 'will', 'would',
    'should', 'must', 'target', 'targets', 'planned', 'plan', 'plans', 'future', 'remaining', 'remain',
    'remains', 'gap', 'gaps', 'open', 'missing', 'required', 'requires', 'require', 'goal', 'pending',
    'incomplete', 'unfinished', 'later', 'instead', 'forbids', 'forbidden', 'rejects', 'rejected', 'fails',
  ].join('|')})\\b|n't\\b`,
  'iu',
);

const COMPLETE = '(?:complete|completed|done|finished|delivered|implemented|met|satisfied|resolved|verified)';

/**
 * Claims the audit recognizes. `requirements` are the ledger rows that must
 * all pass before the claim is true (`*` is every row); `always` claims
 * contradict the specification whatever the ledger says.
 */
export const CLAIMS = Object.freeze([
  {
    id: 'overall-completion',
    title: 'states that issue #195 or its whole scope is complete',
    requirements: '*',
    pattern: new RegExp(
      `\\b(?:issue\\s*#?${ISSUE}|the (?:full |whole |complete )?(?:vision|scope|specification)|every requirement|all (?:the |atomic |of the )?requirements)\\b[^.;:]*?\\b(?:is|are|has been|have been|was|were)\\s+(?:now\\s+)?(?:fully\\s+|completely\\s+|all\\s+)?${COMPLETE}\\b`,
      'iu',
    ),
  },
  {
    id: 'fully-implemented',
    title: 'states that the specification is fully implemented',
    requirements: '*',
    pattern: new RegExp(
      `\\b(?:fully|completely)\\s+(?:implements|implemented|delivers|delivered|completes|completed|satisfies|satisfied)\\s+(?:issue\\s*#?${ISSUE}|the (?:vision|specification|scope|contract))\\b`,
      'iu',
    ),
  },
  {
    id: 'external-production-parsers',
    title: 'presents native parsing without external production parsers as delivered',
    requirements: ['I195-DEPENDENCY-PRODUCTION-PARSERS-REMOVED', 'I195-GRAMMAR-NATIVE-MERGED'],
    pattern: /\b(?:no|without|free of|removes? (?:all|every))\s+(?:external|third-party|tree-sitter|vendored)\s+(?:production\s+)?(?:parsers?|grammars?|parser dependencies)\b|\b(?:every|all)\s+(?:catalog\s+)?languages?\s+(?:is|are)\s+parsed\s+(?:by|with)\s+native\b|\bnative (?:merged )?(?:links notation )?grammars? (?:for|cover|covers) (?:every|all) (?:catalog )?languages?\b/iu,
  },
  {
    id: 'future-emitters',
    title: 'presents reverse conversion to every grammar format as delivered',
    requirements: ['I195-INTERCHANGE-REVERSE-CONVERSION', 'I195-INTERCHANGE-FORMAT-FEATURES'],
    pattern: /\b(?:emits?|emitters? for|exports?|converts? back)\b[^.;:]*?\b(?:to|into|in|for) (?:every|all|each) (?:supported |imported )?(?:grammar |source )?formats?\b|\breverse conversion (?:is|works) (?:complete|for every)\b/iu,
  },
  {
    id: 'approximate-round-trips',
    title: 'presents an approximate or lossy round trip as the contract',
    requirements: ['I195-GRAMMAR-LOSSLESS-TREES'],
    always: true,
    pattern: /\b(?:approximate(?:ly)?|lossy|best[- ]effort|near[- ]lossless)\s+round[- ]trips?\b/iu,
  },
  {
    id: 'automatic-merge',
    title: 'presents the automatic multi-source grammar merge as delivered',
    requirements: ['I195-MERGE-AUTOMATIC-PIPELINE', 'I195-MERGE-SHIPPED-GRAMMARS-ARE-MERGED'],
    pattern: /\bautomatically merges? (?:every|all|the) (?:source |upstream )?grammars?\b|\bshipped grammars are (?:all )?merged\b|\bautomatic (?:multi-source )?(?:grammar )?merge (?:is complete|runs for every)\b/iu,
  },
  {
    id: 'proof-preservation',
    title: 'presents proof preservation as delivered',
    requirements: ['I195-SEMANTICS-PROOF-PRESERVATION'],
    pattern: /\bproofs? (?:are|is) (?:always |fully )?preserved\b|\bpreserves (?:every |all )proofs?\b/iu,
  },
  {
    id: 'full-language-translation',
    title: 'presents translation of the full languages as delivered',
    requirements: ['I195-SEMANTICS-CONSTRUCT-INVENTORY', 'I195-SEMANTICS-FAITHFUL-BEHAVIOR'],
    pattern: /\btranslates? (?:every|all|any) (?:construct|program|language feature)s?\b|\btranslat(?:es|ion of) the (?:full|complete|whole) (?:languages?|construct inventor(?:y|ies))\b/iu,
  },
  {
    id: 'incomplete-scope-as-finished',
    title: 'presents a subset as the finished contract',
    requirements: '*',
    pattern: /\b(?:portable core|subset)\b[^.;:]*?\b(?:is|are) (?:the )?(?:complete|full|finished|final) (?:contract|scope|implementation)\b/iu,
  },
]);

/** A report of `N/N requirements` passing. */
const PASS_COUNT = /\b(\d+)\s*\/\s*(\d+)\s+(?:atomic\s+)?requirements?\b[^.;]*?\bpass(?:ed|ing|es)?\b/giu;
const CLOSING = new RegExp(
  `\\b(?:close[sd]?|fix(?:e[sd])?|resolve[sd]?)\\s*:?\\s+(?:${REPOSITORY.replace('/', '\\/')})?#${ISSUE}\\b|` +
    `\\b(?:close[sd]?|fix(?:e[sd])?|resolve[sd]?)\\s*:?\\s+https:\\/\\/github\\.com\\/${REPOSITORY.replace('/', '\\/')}\\/issues\\/${ISSUE}\\b`,
  'iu',
);
const LEDGER_RESULT = /Acceptance result:\s*\*\*PASS\*\*/u;

/**
 * The completion state of the ledger: the rows with no implementation entry
 * point, and, when an acceptance report is given, the rows it failed. A row
 * stays open until an acceptance report records it passing: a declared entry
 * point does not stand in for the observed behavior. The work is complete only
 * when an acceptance report over every checkpoint passed every row of this
 * manifest.
 */
export function completionState(manifest, report = null) {
  const rows = manifest.atomicRequirements;
  const unimplemented = rows
    .filter((row) => Object.values(row.implementationEntryPoints ?? {}).some((entry) => entry === null))
    .map(({ id }) => id);
  const failing = report ? (report.requirements ?? []).filter((entry) => !entry.passed).map(({ id }) => id) : [];
  const passed = new Set((report?.requirements ?? []).filter((entry) => entry.passed).map(({ id }) => id));
  const unverified = rows.filter(({ id }) => !passed.has(id)).map(({ id }) => id);
  const verified = Boolean(
    report?.passed && report.checkpoint === 'all' && report.summary?.requirements === rows.length,
  );
  return {
    requirements: rows.length,
    ids: new Set(rows.map(({ id }) => id)),
    open: new Set([...unimplemented, ...failing, ...unverified]),
    unimplemented,
    failing,
    verified,
    complete: verified && unimplemented.length === 0 && failing.length === 0,
  };
}

/** Markdown prose as sentences with the line each starts on; fenced code, HTML comments and tables are skipped. */
export function sentences(text) {
  const result = [];
  let fenced = false;
  let paragraph = [];
  const flush = () => {
    if (paragraph.length === 0) return;
    const [{ line }] = paragraph;
    const prose = paragraph.map(({ text: part }) => part).join(' ').replace(/`/gu, '');
    for (const sentence of prose.split(/(?<=[.!?])\s+(?=[A-Z*_[(`"'])/u)) {
      if (sentence.trim()) result.push({ line, text: sentence.trim() });
    }
    paragraph = [];
  };
  text.split(/\r?\n/u).forEach((raw, index) => {
    const line = index + 1;
    if (/^\s*(?:```|~~~)/u.test(raw)) {
      flush();
      fenced = !fenced;
      return;
    }
    if (fenced) return;
    const trimmed = raw.trim();
    if (trimmed === '' || /^#{1,6}\s/u.test(trimmed) || /^<!--.*-->$/u.test(trimmed)) {
      flush();
      if (/^#{1,6}\s/u.test(trimmed)) result.push({ line, text: trimmed.replace(/^#+\s*/u, '') });
      return;
    }
    // A list item or table row starts its own statement.
    if (/^(?:[-*+]|\d+[.)])\s/u.test(trimmed) || trimmed.startsWith('|')) flush();
    paragraph.push({ line, text: trimmed.replace(/^(?:[-*+]|\d+[.)]|>)\s*/u, '') });
    if (trimmed.startsWith('|')) flush();
  });
  flush();
  return result;
}

const qualified = (sentence) => QUALIFIERS.test(sentence);

/**
 * The problems of one text. `kind` is `contract` for docs/vision.md (the
 * target, audited only for completion claims and closing directives),
 * `case-study` for historical case studies (audited only for closing
 * directives), `ledger` for the generated ledger, and `document` otherwise.
 */
export function auditText(text, { file, kind = 'document', state }) {
  const problems = [];
  const report = (line, claim, message) => problems.push({ file, line, claim, message });
  if (kind === 'ledger') {
    const line = text.split('\n').findIndex((row) => LEDGER_RESULT.test(row)) + 1;
    if (line > 0 && !state.complete) report(line, 'ledger-pass', 'reports PASS while a requirement is unimplemented, failing or unverified');
    return problems;
  }
  for (const sentence of sentences(text)) {
    if (CLOSING.test(sentence.text) && !state.complete) {
      report(sentence.line, 'closing-directive', `carries an issue-closing directive for #${ISSUE} before every requirement is verified and delivered`);
    }
    if (kind === 'case-study') continue;
    for (const [whole, passed, total] of sentence.text.matchAll(PASS_COUNT)) {
      if (passed !== total || qualified(sentence.text)) continue;
      if (!state.complete || Number(total) !== state.requirements) {
        report(sentence.line, 'full-pass-count', `reports ${passed}/${total} requirements passing while the ledger has ${state.requirements} rows and ${state.open.size || 'unverified'} open`);
      }
    }
    for (const claim of CLAIMS) {
      if (kind === 'contract' && claim.requirements !== '*') continue;
      const match = claim.pattern.exec(sentence.text);
      if (!match || qualified(sentence.text)) continue;
      const open = claim.requirements === '*'
        ? (state.complete ? [] : [...state.open].slice(0, 3))
        : claim.requirements.filter((id) => state.open.has(id) || !state.ids.has(id));
      if (claim.always || open.length > 0 || (claim.requirements === '*' && !state.complete)) {
        const reason = claim.always ? 'the specification requires lossless round trips' : `open: ${open.join(', ') || 'unverified'}`;
        report(sentence.line, claim.id, `${claim.title} ("${match[0]}"); ${reason}`);
      }
    }
  }
  return problems;
}

/** Every case study directory whose README lacks the historical label near its top. */
export function unlabeledCaseStudies(root, files) {
  const directories = new Set(files
    .filter((file) => file.startsWith(`${CASE_STUDIES}/`))
    .map((file) => file.split('/').slice(0, 3).join('/')));
  return [...directories].sort().filter((directory) => {
    const readme = path.join(directory, 'README.md');
    if (!files.includes(readme)) return true;
    const head = readFileSync(path.join(root, readme), 'utf8').split('\n').slice(0, LABEL_WINDOW);
    return !head.some((line) => line.startsWith(HISTORICAL_LABEL));
  });
}

/** The kind of a repository document for auditText. */
export function documentKind(file) {
  if (file === 'docs/vision.md') return 'contract';
  if (file === 'docs/issue-195-requirement-ledger.md') return 'ledger';
  if (file.startsWith(`${CASE_STUDIES}/`)) return 'case-study';
  return 'document';
}

/** Every tracked Markdown file this repository writes: documentation, READMEs, changelogs, case studies and logs. */
export function documentationFiles(root) {
  return execFileSync('git', ['ls-files', '-z', '--', '*.md', '*.markdown'], { cwd: root, encoding: 'utf8' })
    .split('\0')
    .filter((file) => file && !THIRD_PARTY.some((pattern) => pattern.test(file)))
    .sort();
}

/**
 * Audits the repository documents and the given external texts (the pull
 * request title and description, the release reports) against `state`.
 */
export function auditDocumentation(root, state, { external = [], files = documentationFiles(root) } = {}) {
  const problems = [];
  for (const file of files) {
    problems.push(...auditText(readFileSync(path.join(root, file), 'utf8'), { file, kind: documentKind(file), state }));
  }
  for (const directory of unlabeledCaseStudies(root, files)) {
    problems.push({ file: `${directory}/README.md`, line: 1, claim: 'historical-label', message: `lacks the "${HISTORICAL_LABEL}" label in its first ${LABEL_WINDOW} lines` });
  }
  for (const { name, text } of external) problems.push(...auditText(text, { file: name, state }));
  return { files, problems };
}

async function githubGet(pathname) {
  const token = process.env.GITHUB_TOKEN ?? process.env.GH_TOKEN ?? (() => {
    try {
      return execFileSync('gh', ['auth', 'token'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim();
    } catch {
      return null;
    }
  })();
  const response = await fetch(`https://api.github.com/${pathname}`, {
    headers: {
      Accept: 'application/vnd.github+json',
      'User-Agent': 'meta-language-documentation-check',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
  });
  if (!response.ok) throw new Error(`GET ${pathname} failed with ${response.status}`);
  return response.json();
}

/** The live pull request title and description and the published release reports. */
export async function fetchExternalTexts() {
  const [pull, releases] = await Promise.all([
    githubGet(`repos/${REPOSITORY}/pulls/${PULL_REQUEST}`),
    githubGet(`repos/${REPOSITORY}/releases?per_page=100`),
  ]);
  return [
    { name: `pull request #${PULL_REQUEST} title`, text: pull.title ?? '' },
    { name: `pull request #${PULL_REQUEST} description`, text: pull.body ?? '' },
    ...releases.map((release) => ({ name: `release ${release.tag_name}`, text: `${release.name ?? ''}\n\n${release.body ?? ''}` })),
  ];
}
