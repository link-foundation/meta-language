#!/usr/bin/env node
// Generates the merge quality evidence of the native grammars (requirement
// I195-MERGE-QUALITY-EVIDENCE): for every native grammar in the
// `nativeGrammars` of parity/language-grammar-inventory.json, a comparison
// with the pinned tree-sitter grammar it was merged from and replaced as the
// default parse, on the corpus of that grammar in
// parity/fixtures/native-grammars/<language>.json: the upstream test corpus of
// the source grammar at its pinned revision for the grammars the importer
// merges, and the cases written from the merged sources for the others.
//
// - coverage: the rules by kind, the visible rules the corpus exercises, the
//   node kinds and fields the oracle rows check, and the merge report counts;
// - preserved features: the scanner, conflict, precedence, extra and kind
//   links the merge kept from the source grammar;
// - correctness: the sources whose native rows equal the oracle rows, the
//   divergences and the rejected invalid sources;
// - recovery: the rejections the native executor repairs into lossless trees
//   and the categories of parity/fixtures/native-recovery.json;
// - shared reuse: the rules that name a concept another native grammar names;
// - time and memory of the native executor and the oracle on the same corpus
//   in both runtimes, in parity/fixtures/merge-quality-measurements.json.
//
// The first five are deterministic; both runtimes compute them and compare
// with parity/fixtures/merge-quality-evidence.json. The measurements depend on
// the machine; `--measure` writes them, and the tests check that every native
// grammar has them and measure their own runtime again.
//
//   node js/scripts/build-merge-quality-evidence.mjs            # write the report and the document
//   node js/scripts/build-merge-quality-evidence.mjs --measure  # measure both runtimes, then write
//   node js/scripts/build-merge-quality-evidence.mjs --check    # fail on drift
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { arch, cpus, platform } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { performance } from 'node:perf_hooks';
import { fileURLToPath } from 'node:url';
import { gunzipSync } from 'node:zlib';

import { Parser } from 'web-tree-sitter';

import { compileGrammar, languageEntry, parseGrammarLinks } from '../src/index.js';
import { loadGrammarLanguage } from '../src/grammar-tiering.js';
import { grammarFile } from './grammar-files.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
export const MERGE_QUALITY_FIXTURE = 'parity/fixtures/merge-quality-evidence.json';
export const MERGE_QUALITY_MEASUREMENTS = 'parity/fixtures/merge-quality-measurements.json';
export const MERGE_QUALITY_DOCUMENT = 'docs/grammar/merge-quality-evidence.md';
const INVENTORY = 'parity/language-grammar-inventory.json';
const RECOVERY = 'parity/fixtures/native-recovery.json';
const REUSE = 'parity/fixtures/native-grammar-concept-reuse.json';
const FEATURE_LINKS = ['scanner', 'conflict', 'precedences', 'extra', 'kind'];
const RULE_KINDS = ['normal', 'atomic', 'silent', 'token'];
// The source that is the upstream test corpus at the pinned revision.
const isUpstreamCorpus = (source) => source.includes('/tree/') && source.endsWith('corpus');

const read = (path) => readFileSync(join(root, path), 'utf8');
const readJson = (path) => JSON.parse(read(path));

/** The native grammars of the inventory: `[id, entry]` by id. */
export function nativeGrammarEntries() {
  return Object.entries(readJson(INVENTORY).nativeGrammars).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0));
}

/** The corpus fixture of the native grammar whose links are at `grammar`. */
export function nativeFixtureFile(grammar) {
  return `parity/fixtures/native-grammars/${grammar.replace(/^.*\//u, '')}`.replace(/\.lino$/u, '.json');
}

/** The merge report of the native grammar whose links are at `grammar`, when the importer wrote one. */
function mergeReportFile(grammar) {
  const file = `parity/grammars/merge-reports/${grammar.replace(/^.*\//u, '').replace(/\.lino$/u, '.json')}`;
  try {
    read(file);
    return file;
  } catch {
    return null;
  }
}

const utf8Length = (text) => Buffer.byteLength(text, 'utf8');

/** The deterministic merge quality record of the native grammar `id`. */
export function mergeQualityRecord(id, entry, { recovery = readJson(RECOVERY), reuse = readJson(REUSE) } = {}) {
  const links = read(entry.grammar);
  const grammar = parseGrammarLinks(links);
  const fixtureFile = nativeFixtureFile(entry.grammar);
  const fixture = readJson(fixtureFile);
  const reportFile = mergeReportFile(entry.grammar);
  const report = reportFile ? readJson(reportFile) : null;

  const rules = Object.fromEntries(RULE_KINDS.map((kind) => [kind, 0]));
  for (const rule of grammar.rules.values()) rules[rule.kind] += 1;
  const oracleName = (name) => fixture.oracleKinds[name] ?? name;
  const unseen = new Set([...fixture.hidden, ...fixture.anonymous]);
  const visible = [...grammar.rules.values()]
    .filter((rule) => rule.kind === 'normal' && !unseen.has(rule.name) && !oracleName(rule.name).startsWith('_'))
    .map((rule) => oracleName(rule.name));
  const kinds = new Set();
  const fields = new Set();
  let rows = 0;
  for (const match of fixture.matches) {
    rows += match.rows.length;
    for (const [, field, kind, named] of match.rows) {
      if (named === 1) kinds.add(kind);
      if (field !== null) fields.add(field);
    }
  }
  const heads = Object.fromEntries(FEATURE_LINKS.map((head) => [head, 0]));
  for (const line of links.split('\n')) {
    const head = /^\(([a-z]+) /u.exec(line)?.[1];
    if (head in heads) heads[head] += 1;
  }
  const categories = {};
  for (const { language, category } of recovery.cases) {
    if (language === fixture.language) categories[category] = (categories[category] ?? 0) + 1;
  }
  const count = (pattern) => fixture.rejections.reduce((sum, { recovered }) => sum + ((recovered ?? '').match(pattern)?.length ?? 0), 0);
  const reused = reuse.grammars.find((candidate) => candidate.grammar === id);
  return {
    grammar: id,
    language: fixture.language,
    links: entry.grammar,
    fixture: fixtureFile,
    oracle: fixture.oracle,
    sources: fixture.sources,
    corpus: fixture.sources.find(isUpstreamCorpus) ?? null,
    mergeReport: reportFile,
    coverage: {
      rules,
      visibleRules: visible.length,
      exercisedRules: visible.filter((name) => kinds.has(name)).length,
      checkedKinds: kinds.size,
      checkedFields: fields.size,
      renamed: report ? report.renamed.length : null,
      expandedWords: report ? report.expandedWords.length : null,
      approximations: report ? report.approximations.length : null,
      unsupported: report ? report.unsupported.length : null,
    },
    features: heads,
    correctness: {
      matches: fixture.matches.length,
      rows,
      bytes: fixture.matches.reduce((sum, { source }) => sum + utf8Length(source), 0),
      divergences: fixture.divergences.length,
      rejections: fixture.rejections.length,
    },
    recovery: {
      repaired: fixture.rejections.filter(({ recovered }) => /\((?:ERROR|MISSING)@/u.test(recovered ?? '')).length,
      errorNodes: count(/\(ERROR@/gu),
      missingNodes: count(/\(MISSING@/gu),
      categories: Object.fromEntries(Object.entries(categories).sort(([a], [b]) => (a < b ? -1 : 1))),
    },
    reuse: { shared: reused.shared.length, specific: reused.specific.length },
  };
}

/** The deterministic merge quality report of every native grammar. */
export function mergeQualityReport() {
  const recovery = readJson(RECOVERY);
  const reuse = readJson(REUSE);
  return { grammars: nativeGrammarEntries().map(([id, entry]) => mergeQualityRecord(id, entry, { recovery, reuse })) };
}

const inline = (value) => JSON.stringify(value).replaceAll('":', '": ').replaceAll(',"', ', "');

export function formatMergeQuality(report) {
  const grammars = report.grammars.map((record) => [
    '    {',
    Object.entries(record).map(([key, value]) => (
      key === 'sources'
        ? `      "sources": [\n${value.map((source) => `        ${JSON.stringify(source)}`).join(',\n')}\n      ]`
        : `      ${JSON.stringify(key)}: ${inline(value)}`
    )).join(',\n'),
    '    }',
  ].join('\n'));
  return [
    '{',
    '  "generatedBy": "js/scripts/build-merge-quality-evidence.mjs",',
    '  "grammars": [',
    grammars.join(',\n'),
    '  ]',
    '}',
    '',
  ].join('\n');
}

export function formatMeasurements(measurements) {
  return [
    '{',
    '  "generatedBy": "js/scripts/build-merge-quality-evidence.mjs --measure",',
    `  "measuredOn": ${inline(measurements.measuredOn)},`,
    '  "grammars": [',
    measurements.grammars.map((entry) => `    ${inline(entry)}`).join(',\n'),
    '  ]',
    '}',
    '',
  ].join('\n');
}

const percent = (part, whole) => `${whole === 0 ? 0 : Math.round((100 * part) / whole)}%`;
const code = (text) => `\`${text}\``;
const milliseconds = (value) => (value === null || value === undefined ? 'n/a' : value.toFixed(1));
const mebibytes = (kibibytes) => (kibibytes === null || kibibytes === undefined ? 'n/a' : (kibibytes / 1024).toFixed(1));
const ratio = (native, oracle) => (native === null || oracle === null || oracle === 0 ? 'n/a' : `${(native / oracle).toFixed(1)}×`);

export function renderMergeQualityDocument(report, measurements) {
  const total = (pick) => report.grammars.reduce((sum, record) => sum + pick(record), 0);
  const lines = [
    '# Merge quality evidence of the native grammars',
    '',
    'Generated by `node js/scripts/build-merge-quality-evidence.mjs` from the native',
    'grammars in `parity/grammars/native/`, their corpus fixtures in',
    '`parity/fixtures/native-grammars/`, the merge reports in',
    '`parity/grammars/merge-reports/`, the recovery records in',
    `${code(RECOVERY)} and the concept reuse report in ${code(REUSE)};`,
    'do not edit it by hand. It is the published comparison of requirement',
    '`I195-MERGE-QUALITY-EVIDENCE`: every native grammar against the pinned',
    'tree-sitter grammar it was merged from and replaced as the default parse,',
    'which stays a development oracle only.',
    '',
    `The deterministic part is ${code(MERGE_QUALITY_FIXTURE)}; the JavaScript test`,
    '`js/tests/issue-195-merge-quality-evidence.test.js` and the Rust test',
    '`rust/tests/unit/issue_195_merge_quality_evidence.rs` recompute it in their',
    `runtimes and compare. The measurements are ${code(MERGE_QUALITY_MEASUREMENTS)},`,
    'written by `--measure`; `rust/tests/merge_quality.rs` measures the Rust',
    'executor with a counting allocator.',
    '',
    '## Corpora',
    '',
    'The corpus of a grammar the importer merges is the upstream test corpus of',
    'its source grammar at the pinned revision, with a few sources more. The',
    'corpus of a grammar written from the language specification is **curated**:',
    'the cases `js/scripts/generate-native-grammar-fixtures.mjs` lists from its',
    'merged sources. No corpus is generated from the native grammar it checks,',
    'and the oracle rows of every source are the tree the pinned tree-sitter',
    'grammar builds. The sources are the specifications and grammars merged.',
    '',
    '| Grammar | Oracle | Corpus | Sources |',
    '| --- | --- | --- | --- |',
  ];
  for (const record of report.grammars) {
    const corpus = record.corpus === null ? 'curated' : `[upstream](${record.corpus})`;
    const sources = record.sources.map((source, index) => `[${index + 1}](${/^https:/u.test(source) ? source : `../../${source}`})`).join(' ');
    lines.push(`| ${code(record.grammar)} | ${record.oracle} | ${corpus} | ${sources} |`);
  }
  lines.push(
    '',
    '## Correctness',
    '',
    'A **match** is a valid source whose native tree has the oracle rows: every',
    'node kind, field, byte range and named flag in the same order. A',
    '**divergence** is a source the language specification accepts and the oracle',
    'recovers from; the native grammar accepts it. A **rejection** is an invalid',
    'source the oracle recovers from; the native grammar rejects it by default.',
    '',
    '| Grammar | Matches | Rows | Bytes | Divergences | Rejections |',
    '| --- | ---: | ---: | ---: | ---: | ---: |',
  );
  for (const { grammar, correctness: c } of report.grammars) {
    lines.push(`| ${code(grammar)} | ${c.matches} | ${c.rows} | ${c.bytes} | ${c.divergences} | ${c.rejections} |`);
  }
  lines.push(
    `| all | ${total((r) => r.correctness.matches)} | ${total((r) => r.correctness.rows)} | ${total((r) => r.correctness.bytes)} | ${total((r) => r.correctness.divergences)} | ${total((r) => r.correctness.rejections)} |`,
    '',
    '## Coverage',
    '',
    'A **visible rule** is a normal rule whose node the oracle shows; it is',
    '**exercised** when a matched source has its node. **Checked kinds** and',
    '**checked fields** are the named node kinds and the fields the oracle rows',
    'of the matches compare. The renamed, expanded, approximated and unsupported',
    'counts come from the merge report of the grammars the importer merges from a',
    'tree-sitter `grammar.json`; the other native grammars were written from the',
    'language specification and checked against the oracle.',
    '',
    '| Grammar | Normal | Atomic | Silent | Token | Exercised visible rules | Checked kinds | Checked fields | Renamed | Expanded words | Approximations | Unsupported |',
    '| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |',
  );
  const optional = (value) => (value === null ? 'n/a' : value);
  for (const { grammar, coverage: c } of report.grammars) {
    lines.push(`| ${code(grammar)} | ${c.rules.normal} | ${c.rules.atomic} | ${c.rules.silent} | ${c.rules.token} | ${c.exercisedRules} of ${c.visibleRules} (${percent(c.exercisedRules, c.visibleRules)}) | ${c.checkedKinds} | ${c.checkedFields} | ${optional(c.renamed)} | ${optional(c.expandedWords)} | ${optional(c.approximations)} | ${optional(c.unsupported)} |`);
  }
  lines.push(
    '',
    '## Preserved features and shared reuse',
    '',
    'The links the native grammar keeps from the features of its source grammar:',
    'native **scanners** ported from the external scanner, declared **conflicts**,',
    '**precedence** tables, **extras** and node **kinds** no rule defines. **Shared**',
    'rules name a concept another native grammar names',
    '(`docs/grammar/native-grammar-concept-reuse.md`).',
    '',
    '| Grammar | Scanners | Conflicts | Precedences | Extras | Kinds | Shared rules | Language-specific rules |',
    '| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |',
  );
  for (const { grammar, features: f, reuse } of report.grammars) {
    lines.push(`| ${code(grammar)} | ${f.scanner} | ${f.conflict} | ${f.precedences} | ${f.extra} | ${f.kind} | ${reuse.shared} | ${reuse.specific} |`);
  }
  lines.push(
    '',
    '## Recovery',
    '',
    'With `errorRecovery` the native executor repairs every rejection into a',
    'lossless tree of ERROR and MISSING leaves, kept in the corpus fixture.',
    `${code(RECOVERY)} compares the native repairs of the conformance and`,
    'generative cases with the oracle and gives each difference a category.',
    '',
    '| Grammar | Rejections | Repaired | ERROR nodes | MISSING nodes | Recovery records by category |',
    '| --- | ---: | ---: | ---: | ---: | --- |',
  );
  for (const { grammar, correctness, recovery } of report.grammars) {
    const categories = Object.entries(recovery.categories).map(([category, count]) => `${category} ${count}`).join(', ') || 'none';
    lines.push(`| ${code(grammar)} | ${correctness.rejections} | ${recovery.repaired} | ${recovery.errorNodes} | ${recovery.missingNodes} | ${categories} |`);
  }
  lines.push('', '## Time and memory', '');
  if (!measurements) {
    lines.push('No measurements are recorded; run `node js/scripts/build-merge-quality-evidence.mjs --measure`.', '');
    return lines.join('\n');
  }
  const { measuredOn } = measurements;
  lines.push(
    `Measured on ${measuredOn.platform} ${measuredOn.arch}, ${measuredOn.cpu}, with Node.js ${measuredOn.node}`,
    `and the ${measuredOn.rustProfile}. Times are milliseconds of one warm pass over`,
    'the matches (parse) and over the rejections (recover: the native executor',
    'with `errorRecovery`, the oracle with its own recovery), after one pass to',
    'warm up. Machines differ; the ratios matter more than the values.',
    '',
    'JavaScript runs each side in its own process: the native executor of',
    '`js/src/` and the oracle compiled to WebAssembly in `web-tree-sitter`. Memory',
    'is the peak resident set of the process above the peak of a process that',
    'loads the same modules and parses nothing.',
    '',
    '| Grammar | Native parse | Oracle parse | Ratio | Native recover | Oracle recover | Native peak MiB | Oracle peak MiB |',
    '| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |',
  );
  for (const { grammar, javascript: { native, oracle } } of measurements.grammars) {
    lines.push(`| ${code(grammar)} | ${milliseconds(native.parseMs)} | ${milliseconds(oracle.parseMs)} | ${ratio(native.parseMs, oracle.parseMs)} | ${milliseconds(native.recoverMs)} | ${milliseconds(oracle.recoverMs)} | ${mebibytes(native.peakKiB)} | ${mebibytes(oracle.peakKiB)} |`);
  }
  lines.push(
    '',
    'Rust measures the native executor of `rust/src/` and, where the pinned',
    'grammar crate is a development dependency, the oracle through the',
    '`tree-sitter` crate. Native memory is the peak of live heap bytes the',
    'counting allocator sees while the grammar compiles and parses; the oracle',
    'allocates in C, outside the Rust allocator, so its memory is not measured',
    'here. The test profile builds the native executor and the C of the oracle',
    'without optimization, as the tests run them.',
    '',
    '| Grammar | Native parse | Oracle parse | Ratio | Native recover | Oracle recover | Native peak heap MiB |',
    '| --- | ---: | ---: | ---: | ---: | ---: | ---: |',
  );
  for (const { grammar, rust: { native, oracle } } of measurements.grammars) {
    lines.push(`| ${code(grammar)} | ${milliseconds(native.parseMs)} | ${milliseconds(oracle?.parseMs)} | ${ratio(native.parseMs, oracle?.parseMs ?? null)} | ${milliseconds(native.recoverMs)} | ${milliseconds(oracle?.recoverMs)} | ${mebibytes(native.peakHeapBytes / 1024)} |`);
  }
  lines.push('');
  return lines.join('\n');
}

// --- Measurement ---------------------------------------------------------

const LOCK = JSON.parse(readFileSync(new URL('../src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));

/** The pinned tree-sitter grammar that is the oracle of `language`. */
function oracleLanguage(language) {
  const entry = languageEntry(language);
  const id = (entry.oracleGrammars ?? entry.grammars)[0].id;
  const file = new URL(`../../${grammarFile(LOCK.grammars[id], `${id}.wasm.gz`)}`, import.meta.url);
  return loadGrammarLanguage(gunzipSync(readFileSync(file)));
}

/** Milliseconds of a pass of `parse` over `sources`, after a pass to warm up. */
function timed(sources, parse) {
  for (const source of sources) parse(source);
  const start = performance.now();
  for (const source of sources) parse(source);
  return Number((performance.now() - start).toFixed(3));
}

/**
 * Measures one side of the native grammar `id` in this process: `native`,
 * `oracle`, or `baseline`, which loads the same modules and parses nothing.
 */
export function measureSide(id, side) {
  const [, entry] = nativeGrammarEntries().find(([candidate]) => candidate === id);
  const fixture = readJson(nativeFixtureFile(entry.grammar));
  const matches = fixture.matches.map(({ source }) => source);
  const rejections = fixture.rejections.map(({ source }) => source);
  const result = {};
  if (side === 'native') {
    const parser = compileGrammar(parseGrammarLinks(read(entry.grammar)));
    result.parseMs = timed(matches, (source) => {
      if (!parser.parseTree(source).ok) throw new Error(`the native ${id} grammar rejects ${JSON.stringify(source)}`);
    });
    result.recoverMs = timed(rejections, (source) => parser.parseTree(source, { errorRecovery: true }));
  } else if (side === 'oracle') {
    const parser = new Parser();
    parser.setLanguage(oracleLanguage(fixture.language));
    const parse = (source) => parser.parse(source).delete();
    result.parseMs = timed(matches, parse);
    result.recoverMs = timed(rejections, parse);
    parser.delete();
  } else if (side !== 'baseline') {
    throw new Error(`no side is named ${side}`);
  }
  result.peakKiB = process.resourceUsage().maxRSS;
  return result;
}

function probe(id, side) {
  const run = spawnSync(process.execPath, [fileURLToPath(import.meta.url), '--probe', id, side], { encoding: 'utf8' });
  if (run.status !== 0) throw new Error(`measuring ${side} ${id} failed:\n${run.stderr}`);
  return JSON.parse(run.stdout);
}

/** The JavaScript measurements of the native grammar `id`, each side in its own process. */
export function measureJavaScript(id) {
  const baseline = probe(id, 'baseline');
  const above = ({ peakKiB, ...rest }) => ({ ...rest, peakKiB: Math.max(0, peakKiB - baseline.peakKiB) });
  return { native: above(probe(id, 'native')), oracle: above(probe(id, 'oracle')) };
}

/** The Rust measurements of every native grammar, from `rust/tests/merge_quality.rs`. */
function measureRust() {
  const run = spawnSync('cargo', ['test', '--test', 'merge_quality', '--', '--nocapture', '--test-threads=1'], {
    cwd: join(root, 'rust'),
    encoding: 'utf8',
    env: { ...process.env, MERGE_QUALITY_PRINT: '1' },
    maxBuffer: 1 << 26,
  });
  if (run.status !== 0) throw new Error(`the Rust measurement failed:\n${run.stderr}`);
  const measured = new Map();
  for (const line of run.stdout.split('\n')) {
    // libtest prints the test name on the line of the first result.
    const json = /merge-quality (\{.*\})$/u.exec(line)?.[1];
    if (json) {
      const { grammar, ...rest } = JSON.parse(json);
      measured.set(grammar, rest);
    }
  }
  return measured;
}

function rustVersion() {
  const run = spawnSync('rustc', ['--version'], { encoding: 'utf8' });
  return run.status === 0 ? run.stdout.trim().split(' ').slice(0, 2).join(' ') : 'rustc';
}

export function measure() {
  const rust = measureRust();
  const grammars = nativeGrammarEntries().map(([id]) => {
    if (!rust.has(id)) throw new Error(`the Rust measurement has no ${id}`);
    return { grammar: id, javascript: measureJavaScript(id), rust: rust.get(id) };
  });
  return {
    measuredOn: {
      platform: platform(),
      arch: arch(),
      cpu: cpus()[0]?.model.trim() ?? 'unknown',
      node: process.versions.node,
      rustProfile: `unoptimized test profile of ${rustVersion()}`,
    },
    grammars,
  };
}

async function main() {
  const probeAt = process.argv.indexOf('--probe');
  if (probeAt >= 0) {
    process.stdout.write(`${JSON.stringify(measureSide(process.argv[probeAt + 1], process.argv[probeAt + 2]))}\n`);
    return;
  }
  const report = mergeQualityReport();
  let measurements = null;
  if (process.argv.includes('--measure')) {
    measurements = measure();
    writeFileSync(join(root, MERGE_QUALITY_MEASUREMENTS), formatMeasurements(measurements));
  } else {
    try {
      measurements = readJson(MERGE_QUALITY_MEASUREMENTS);
    } catch {
      measurements = null;
    }
  }
  const outputs = [
    [MERGE_QUALITY_FIXTURE, formatMergeQuality(report)],
    [MERGE_QUALITY_DOCUMENT, renderMergeQualityDocument(report, measurements)],
  ];
  if (process.argv.includes('--check')) {
    const stale = outputs.filter(([path, text]) => {
      try {
        return read(path) !== text;
      } catch {
        return true;
      }
    }).map(([path]) => path);
    if (stale.length > 0) {
      console.error(`the merge quality evidence is stale: ${stale.join(', ')}`);
      console.error('run: node js/scripts/build-merge-quality-evidence.mjs');
      process.exit(1);
    }
    console.log(`the merge quality evidence matches ${report.grammars.length} native grammars`);
    return;
  }
  for (const [path, text] of outputs) writeFileSync(join(root, path), text);
  console.log(`wrote the merge quality evidence of ${report.grammars.length} native grammars`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) await main();
