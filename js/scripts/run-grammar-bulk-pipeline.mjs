#!/usr/bin/env node
// The bulk grammar pipeline (I195-GRAMMAR-BULK-PIPELINE): for every language
// of parity/language-grammar-inventory.json with a grammar, it imports the
// pinned tree-sitter grammar.json natively and the pinned antlr/grammars-v4
// grammar of parity/grammars-v4-sources.json, compiles both, parses the
// inventory's sample source with each, compares the native rows with the
// oracle rows of parity/fixtures/default-cst-expected.json, merges the
// grammars that imported, and lists every feature still missing: unsupported
// constructs, undefined rules, compile errors and rejected samples.
//
//   node js/scripts/run-grammar-bulk-pipeline.mjs --out-dir DIR [--only A,B] [--jobs N] [--timeout SECONDS]
//
// It writes DIR/grammar-bulk-matrix.json and DIR/grammar-bulk-matrix.md. Each
// language runs in its own process with a memory limit and a timeout, and
// reports after every stage, so one slow grammar keeps the stages it finished
// and never stops the others. Registry grammar.json files need `cargo fetch`
// in rust/; upstream grammars (vendored tree-sitter parsers and grammars-v4)
// are fetched at their pinned revisions into GRAMMAR_BULK_CACHE (default: a
// directory under the system temporary directory).
import { spawn } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { Grammar } from '../src/grammar.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const script = fileURLToPath(import.meta.url);
export const GRAMMARS_V4_SOURCES = 'parity/grammars-v4-sources.json';
const readJson = (file) => JSON.parse(readFileSync(join(root, file), 'utf8'));
const cacheDir = process.env.GRAMMAR_BULK_CACHE ?? join(tmpdir(), 'meta-language-grammar-bulk');

/** The inventory languages with a grammar, in inventory order. */
export function bulkLanguages() {
  return readJson('parity/language-grammar-inventory.json').languages.filter(({ grammars }) => grammars?.length > 0);
}

// The text of `url`, cached by its path under the cache directory.
async function fetchCached(url) {
  const file = join(cacheDir, url.replace(/^https:\/\//u, '').replace(/[^\w./-]/gu, '_'));
  if (existsSync(file)) return readFileSync(file, 'utf8');
  const response = await fetch(url);
  if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`);
  const text = await response.text();
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, text);
  return text;
}

/**
 * The tree-sitter grammar.json text of grammar `id` and where it came from: a
 * source pinned in parity/grammars/sources.json (with its entry, `pinned`),
 * the crate rust/Cargo.lock pins, or the upstream revision of a vendored
 * parser.
 */
export async function treeSitterGrammarJson(id) {
  const { sourceText } = await import('./import-native-grammars.mjs');
  const pinned = readJson('parity/grammars/sources.json').sources.find(({ language }) => language === id);
  if (pinned) return { origin: `${pinned.repository}@${pinned.revision}`, text: sourceText(pinned), pinned };
  const { GRAMMAR_SOURCES, cargoLockVersions, grammarSource } = await import('./build-vendored-grammars.mjs');
  const source = GRAMMAR_SOURCES[id];
  if (!source) throw new Error(`no pinned tree-sitter grammar ${id}`);
  if (source.vendored) {
    const path = [source.dir, 'src/grammar.json'].filter((part) => part && part !== '.').join('/');
    const text = await fetchCached(`https://raw.githubusercontent.com/${source.upstream}/${source.revision}/${path}`);
    return { origin: `https://github.com/${source.upstream}@${source.revision} (without ${source.patch ?? 'a local patch'})`.replace(' (without a local patch)', ''), text };
  }
  const { crateDir, version } = await grammarSource(id, await cargoLockVersions());
  return { origin: `crates.io ${source.crate} ${version}`, text: readFileSync(join(crateDir, source.dir, 'src/grammar.json'), 'utf8') };
}

/**
 * The concatenated pinned grammars-v4 text of `language` and its entry point,
 * or null. ANTLR has no start rule: grammars-v4 names the rule its tests
 * parse from in the `desc.xml` beside the grammar. Without one, the first
 * parser rule is the start rule.
 */
export async function grammarsV4Text(language) {
  const { revision, languages } = readJson(GRAMMARS_V4_SOURCES);
  const files = languages[language];
  if (!files) return null;
  const url = (file) => `https://raw.githubusercontent.com/antlr/grammars-v4/${revision}/${file}`;
  const texts = await Promise.all(files.map((file) => fetchCached(url(file))));
  const desc = await fetchCached(url(`${dirname(files[0])}/desc.xml`)).catch(() => null);
  return { files, text: texts.join('\n'), entryPoint: desc === null ? null : descEntryPoint(desc) };
}

/** The first `<entry-point>` of a grammars-v4 `desc.xml`, or null. */
export function descEntryPoint(xml) {
  return /<entry-point>\s*([A-Za-z_][A-Za-z0-9_]*)\s*<\/entry-point>/u.exec(xml)?.[1] ?? null;
}

/** `grammar` started at `name`, or `grammar` itself when it has no such rule. */
export function startingAt(grammar, name) {
  if (name === null || name === grammar.startRule()?.name || !grammar.rule(name)) return grammar;
  return new Grammar(name, grammar.rules, grammar.sourceFormat, grammar.declarations);
}

const message = (error) => String(error?.message ?? error).split('\n')[0].slice(0, 300);
const elapsed = (start) => Math.round(performance.now() - start);

// A rejection as `syntax at 3:1, expected "\n" or "\r"`.
function rejectionText(rejection) {
  if (!rejection?.reason) return message(rejection?.message ?? JSON.stringify(rejection));
  const expected = rejection.expected?.length ? `, expected ${rejection.expected.join(' or ')}` : '';
  return message(`${rejection.reason} at ${rejection.line}:${rejection.column}${expected}`);
}

// Compiles `grammar` and parses `source`, filling `row`.
function compileAndParse(row, grammar, source, compileGrammar) {
  let start = performance.now();
  const compiled = compileGrammar(grammar);
  row.compiled = true;
  row.compileMs = elapsed(start);
  start = performance.now();
  const result = compiled.parseTree(source);
  row.parseMs = elapsed(start);
  row.sample = result.tree ? 'accepted' : 'rejected';
  if (!result.tree) row.rejection = rejectionText(result.rejection);
  return result;
}

// The native import of a source pinned in parity/grammars/sources.json, as
// js/scripts/import-native-grammars.mjs makes it, and the row projection of
// its native grammar fixture.
async function importPinned(pinned) {
  const { NAME_EXPANSIONS, importSource } = await import('./import-native-grammars.mjs');
  const { NATIVE_GRAMMARS } = await import('./generate-native-grammar-fixtures.mjs');
  const naming = readJson(NAME_EXPANSIONS);
  const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
  const { imported, text } = importSource(pinned, words, naming.grammars[pinned.language]);
  const { hidden, anonymous, extras, oracleKinds } = NATIVE_GRAMMARS.find(({ id }) => id === pinned.language);
  return { imported, listing: text, projection: { hidden, anonymous, extras, oracleKinds } };
}

/** Runs every stage of one language, calling `report(row)` after each. */
export async function bulkLanguageRow(entry, report = () => {}) {
  const { compileGrammar, importAntlr, mergeGrammars, parseGrammarLinks, sharedRuleDecisions } = await import('../src/index.js');
  const { importTreeSitterNative, renderTreeSitterNative } = await import('../src/grammar-importers/tree-sitter-native.js');
  const { nativeRows } = await import('./native-grammar-rows.mjs');
  const oracle = readJson('parity/fixtures/default-cst-expected.json').languages[entry.name]?.positive ?? null;
  const row = { language: entry.name, grammars: entry.grammars, treeSitter: { stage: 'source' }, grammarsV4: null, merge: null };
  const grammars = [];

  const tree = row.treeSitter;
  try {
    const { origin, text, pinned } = await treeSitterGrammarJson(entry.grammars[0]);
    tree.origin = origin;
    tree.stage = 'import';
    const json = JSON.parse(text);
    // A pinned source is imported as the shipped native grammar is, with its
    // reviewed names and native scanner, and projected as its fixture is.
    const { imported, listing, projection } = pinned ? await importPinned(pinned) : (() => {
      const result = importTreeSitterNative(json, { wordRule: 'word_characters' });
      const extras = (json.extras ?? []).filter(({ type }) => type === 'SYMBOL').map(({ name }) => name);
      return { imported: result, listing: renderTreeSitterNative(result), projection: { extras, anonymous: ['unnamed_token'] } };
    })();
    tree.rules = imported.rules.length;
    tree.approximations = imported.report.approximations.length;
    tree.unsupported = imported.report.unsupported.map((item) => (typeof item === 'string' ? item : JSON.stringify(item)));
    tree.stage = 'compile';
    report(row);
    const grammar = parseGrammarLinks(listing);
    grammars.push({ id: 'tree-sitter', grammar });
    const result = compileAndParse(tree, grammar, entry.source, compileGrammar);
    tree.stage = 'done';
    if (result.tree && oracle) tree.rowsMatch = JSON.stringify(nativeRows(result.tree, entry.source, projection)) === JSON.stringify(oracle);
  } catch (error) {
    tree.error = message(error);
  }
  report(row);

  try {
    const v4 = await grammarsV4Text(entry.name);
    if (v4) {
      row.grammarsV4 = { files: v4.files, stage: 'import' };
      report(row);
      const grammar = startingAt(importAntlr(v4.text), v4.entryPoint);
      row.grammarsV4.start = grammar.startRule()?.name;
      row.grammarsV4.rules = grammar.rules.size;
      row.grammarsV4.undefinedRules = grammar.undefinedNonterminals();
      row.grammarsV4.stage = 'compile';
      report(row);
      grammars.push({ id: 'grammars-v4', grammar });
      compileAndParse(row.grammarsV4, grammar, entry.source, compileGrammar);
      row.grammarsV4.stage = 'done';
    }
  } catch (error) {
    row.grammarsV4.error = message(error);
  }
  report(row);

  if (grammars.length > 0) {
    try {
      row.merge = { sources: grammars.map(({ id }) => id), stage: 'merge' };
      report(row);
      const start = performance.now();
      const merged = mergeGrammars(
        grammars.map(({ id, grammar }, precedence) => ({ id, language: entry.name, precedence, grammar })),
        { reconcile: true },
      );
      const shared = merged.groups.flatMap(sharedRuleDecisions);
      row.merge = {
        ...row.merge,
        stage: 'done',
        status: merged.status,
        rules: merged.groups.reduce((sum, group) => sum + group.grammar.rules.size, 0),
        sharedRules: shared.length,
        // The shared rules by how they were matched: proven equivalent,
        // corresponding up to lexical roles, or corresponding by name.
        sharedBy: Object.fromEntries(['merged', 'structural', 'name'].map((basis) => [basis, shared.filter(({ kind, basis: how }) =>
          (basis === 'merged' ? kind === 'merged' : kind === 'reconciled' && how.startsWith(basis === 'name' ? 'name' : 'structural'))).length])),
        mergeMs: elapsed(start),
      };
    } catch (error) {
      row.merge.error = message(error);
    }
  }
  report(row);
  return row;
}

/** The features a row shows missing, as `{ stage, feature }`. */
export function missingFeatures(row) {
  const missing = [];
  const add = (stage, feature) => missing.push({ stage, feature });
  const { treeSitter: tree, grammarsV4: v4, merge } = row;
  for (const item of tree.unsupported ?? []) add('tree-sitter import', item);
  if (tree.error) add(`tree-sitter ${tree.stage}`, tree.error);
  if (tree.timedOut) add(`tree-sitter ${tree.stage}`, 'timed out');
  if (tree.sample === 'rejected') add('tree-sitter sample', tree.rejection);
  if (tree.sample === 'accepted' && tree.rowsMatch === false) add('tree-sitter rows', 'the native rows differ from the oracle rows');
  if (v4) {
    if (v4.undefinedRules?.length) add('grammars-v4 import', `undefined rules: ${v4.undefinedRules.join(' ')}`);
    if (v4.error) add(`grammars-v4 ${v4.stage}`, v4.error);
    if (v4.timedOut) add(`grammars-v4 ${v4.stage}`, 'timed out');
    if (v4.sample === 'rejected') add('grammars-v4 sample', v4.rejection);
  }
  if (merge?.error) add('merge', merge.error);
  if (merge?.timedOut) add('merge', 'timed out');
  return missing;
}

// Runs one language in a child process; resolves with its last reported row.
function runWorker(entry, { timeout, memory }) {
  return new Promise((resolveRow) => {
    const child = spawn(process.execPath, [`--max-old-space-size=${memory}`, script, '--worker', entry.name], { stdio: ['ignore', 'pipe', 'pipe'] });
    let last = { language: entry.name, grammars: entry.grammars, treeSitter: { stage: 'start' }, grammarsV4: null, merge: null };
    let buffer = '';
    let stderr = '';
    let timedOut = false;
    child.stdout.on('data', (chunk) => {
      buffer += chunk;
      const lines = buffer.split('\n');
      buffer = lines.pop();
      for (const line of lines) if (line.startsWith('{')) last = JSON.parse(line);
    });
    child.stderr.on('data', (chunk) => { stderr = (stderr + chunk).slice(-2000); });
    const timer = setTimeout(() => { timedOut = true; child.kill('SIGKILL'); }, timeout * 1000);
    child.on('close', (code) => {
      clearTimeout(timer);
      // The stage still running when the process ended is the one that failed.
      const running = [last.merge, last.grammarsV4, last.treeSitter].find((stage) => stage && stage.stage !== 'done' && !stage.error);
      if (running && (timedOut || code !== 0)) {
        if (timedOut) running.timedOut = true;
        else running.error = message(stderr.trim().split('\n').filter(Boolean).pop() ?? `exit ${code}`);
      }
      resolveRow(last);
    });
  });
}

const yes = (value) => (value === undefined ? '' : value ? 'yes' : 'no');

function stageCells(stage) {
  if (!stage) return ['—', '', '', ''];
  const failed = stage.timedOut ? `timed out in ${stage.stage}` : stage.error ? `fails in ${stage.stage}` : '';
  return [String(stage.rules ?? ''), failed || yes(stage.compiled), stage.sample ?? '', stage.parseMs === undefined ? '' : String(stage.parseMs)];
}

/** The Markdown matrix of `matrix`. */
export function renderMatrix(matrix) {
  const lines = [
    '# Bulk grammar pipeline matrix',
    '',
    `Generated by \`js/scripts/run-grammar-bulk-pipeline.mjs\` for ${matrix.languages.length} inventory languages with a grammar; grammars-v4 at \`${matrix.grammarsV4Revision}\`.`,
    '',
    `Summary: ${matrix.summary.treeSitterImported} tree-sitter grammars import natively, ${matrix.summary.treeSitterCompiled} compile, ${matrix.summary.treeSitterAccepted} accept the sample and ${matrix.summary.rowsMatch} give the oracle rows; ${matrix.summary.grammarsV4Imported} of ${matrix.summary.grammarsV4Mapped} grammars-v4 grammars import, ${matrix.summary.grammarsV4Compiled} compile and ${matrix.summary.grammarsV4Accepted} accept the sample; ${matrix.summary.merged} languages merge.`,
    '',
    '| Language | tree-sitter rules | unsupported | compiles | sample | ms | oracle rows | grammars-v4 rules | compiles | sample | ms | merged rules | shared | missing features |',
    '| --- | ---: | ---: | --- | --- | ---: | --- | ---: | --- | --- | ---: | ---: | ---: | ---: |',
  ];
  for (const row of matrix.languages) {
    const tree = stageCells(row.treeSitter);
    const v4 = stageCells(row.grammarsV4);
    const merge = row.merge?.stage === 'done' ? [String(row.merge.rules), String(row.merge.sharedRules)] : [row.merge ? `fails in ${row.merge.stage}` : '', ''];
    lines.push(`| ${row.language} | ${tree[0]} | ${row.treeSitter.unsupported?.length ?? ''} | ${tree[1]} | ${tree[2]} | ${tree[3]} | ${yes(row.treeSitter.rowsMatch)} | ${v4.join(' | ')} | ${merge.join(' | ')} | ${row.missing.length} |`);
  }
  lines.push('', '## Missing features', '');
  for (const row of matrix.languages) {
    if (row.missing.length === 0) continue;
    lines.push(`### ${row.language}`, '');
    for (const { stage, feature } of row.missing) lines.push(`- ${stage}: ${feature.replace(/\|/gu, '\\|')}`);
    lines.push('');
  }
  const notes = Object.entries(matrix.grammarsV4Notes);
  if (notes.length > 0) {
    lines.push('## grammars-v4 notes', '');
    for (const [language, note] of notes) lines.push(`- ${language}: ${note}`);
    lines.push('');
  }
  return `${lines.join('\n')}\n`;
}

function summarize(rows, mapped) {
  const count = (predicate) => rows.filter(predicate).length;
  return {
    languages: rows.length,
    treeSitterImported: count(({ treeSitter }) => treeSitter.rules !== undefined),
    treeSitterCompiled: count(({ treeSitter }) => treeSitter.compiled),
    treeSitterAccepted: count(({ treeSitter }) => treeSitter.sample === 'accepted'),
    rowsMatch: count(({ treeSitter }) => treeSitter.rowsMatch === true),
    grammarsV4Mapped: mapped,
    grammarsV4Imported: count(({ grammarsV4 }) => grammarsV4?.rules !== undefined),
    grammarsV4Compiled: count(({ grammarsV4 }) => grammarsV4?.compiled),
    grammarsV4Accepted: count(({ grammarsV4 }) => grammarsV4?.sample === 'accepted'),
    merged: count(({ merge }) => merge?.stage === 'done'),
    missingFeatures: rows.reduce((sum, row) => sum + row.missing.length, 0),
  };
}

/** Runs the pipeline over `only` (default: every language) and writes the matrix. */
export async function runBulkPipeline({ outDir, only = null, jobs = 2, timeout = 300, memory = 3072 }) {
  const languages = bulkLanguages().filter(({ name }) => !only || only.includes(name));
  if (only) for (const name of only) if (!languages.some((entry) => entry.name === name)) throw new Error(`${name} is not an inventory language with a grammar`);
  const rows = new Array(languages.length);
  let next = 0;
  const worker = async () => {
    while (next < languages.length) {
      const index = next++;
      const row = await runWorker(languages[index], { timeout, memory });
      row.missing = missingFeatures(row);
      rows[index] = row;
      process.stderr.write(`${row.language}: ${row.missing.length} missing features\n`);
    }
  };
  await Promise.all(Array.from({ length: Math.max(1, jobs) }, worker));
  const v4 = readJson(GRAMMARS_V4_SOURCES);
  const mapped = languages.filter(({ name }) => v4.languages[name]).length;
  const matrix = { grammarsV4Revision: v4.revision, summary: summarize(rows, mapped), grammarsV4Notes: v4.notes, languages: rows };
  mkdirSync(outDir, { recursive: true });
  writeFileSync(join(outDir, 'grammar-bulk-matrix.json'), `${JSON.stringify(matrix, null, 2)}\n`);
  writeFileSync(join(outDir, 'grammar-bulk-matrix.md'), renderMatrix(matrix));
  return matrix;
}

function option(args, name, fallback) {
  const index = args.indexOf(name);
  return index === -1 ? fallback : args[index + 1];
}

async function main(args) {
  const worker = option(args, '--worker', null);
  if (worker) {
    const entry = bulkLanguages().find(({ name }) => name === worker);
    await bulkLanguageRow(entry, (row) => process.stdout.write(`${JSON.stringify(row)}\n`));
    return;
  }
  const outDir = option(args, '--out-dir', null);
  if (!outDir) throw new Error('usage: run-grammar-bulk-pipeline.mjs --out-dir DIR [--only A,B] [--jobs N] [--timeout SECONDS]');
  const only = option(args, '--only', null)?.split(',');
  const matrix = await runBulkPipeline({
    outDir: resolve(outDir),
    only,
    jobs: Number(option(args, '--jobs', 2)),
    timeout: Number(option(args, '--timeout', 300)),
    memory: Number(option(args, '--memory', 3072)),
  });
  process.stdout.write(`${JSON.stringify(matrix.summary)}\n`);
}

if (process.argv[1] === script) {
  main(process.argv.slice(2)).catch((error) => {
    process.stderr.write(`${error.stack ?? error}\n`);
    process.exit(1);
  });
}
