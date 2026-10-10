#!/usr/bin/env node
// Measures, per module, how far the self-translation of each JavaScript module
// of js/src into Rust is from its hand-written Rust counterpart
// (rust/src/<path in snake case>.rs or .../mod.rs), and writes the report as
// Markdown and JSON. CI runs it on every push and publishes both; it reads the
// whole source tree, so it is not meant for a local run (pass --modules to
// measure a few modules).
//
//   node js/scripts/generate-self-translation-report.mjs --out-dir <dir> [--modules a.js,b.js]
//     [--decorators parity/self-translation/decorators.lino] [--shard K/N] [--list]
//
// The whole tree takes over an hour on one runner, so CI splits it: `--shard
// K/N` measures the K-th of N shards, which hold about the same number of
// source bytes each. `--list` prints the modules it would measure and stops.
//
// For each module the report lists its top-level items by status, the Rust
// functions the translation writes, how many of them the hand-written Rust
// defines under the same name and how many of those are identical up to
// whitespace, and the code lines of the translation (no comments, preludes or
// blank lines) the hand-written Rust holds too, counted as a multiset. Each
// module is measured twice, by the generic translation and by the translation
// with the shared emitter decorators (docs/decorators.md), so the report shows
// how much of the difference the decorators close.
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { dirname, join, posix, resolve } from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

import { DecoratorSet } from '../src/decorators.js';
import { parseProgrammingLanguage } from '../src/programming-language-parser.js';
import { selfTranslate, selfTranslationSignatures } from '../src/self-translation.js';
import { tokenize } from '../src/translation/lexer.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const STATUSES = ['translated', 'carried', 'comment'];
const DEFAULT_DECORATORS = 'parity/self-translation/decorators.lino';

function option(name) {
  const index = process.argv.indexOf(name);
  return index === -1 ? null : process.argv[index + 1];
}

function modules() {
  const listed = option('--modules');
  if (listed) return listed.split(',').map((file) => posix.join('js/src', file));
  return listSourceModules();
}

function listSourceModules() {
  const walk = (directory) => readdirSync(join(root, directory), { withFileTypes: true }).flatMap((entry) => {
    const path = posix.join(directory, entry.name);
    if (entry.isDirectory()) return walk(path);
    return entry.name.endsWith('.js') ? [path] : [];
  });
  return walk('js/src').sort();
}

/**
 * The K-th of `count` shards of `files`: each file goes, largest first, to the
 * shard with the fewest bytes so far, so the shards take about as long.
 */
function shardOf(files, index, count) {
  const loads = Array(count).fill(0);
  const shards = Array.from({ length: count }, () => []);
  const sized = files.map((file) => ({ file, size: readFileSync(join(root, file)).length }));
  sized.sort((a, b) => b.size - a.size || a.file.localeCompare(b.file));
  for (const { file, size } of sized) {
    const target = loads.indexOf(Math.min(...loads));
    loads[target] += size;
    shards[target].push(file);
  }
  return shards[index - 1].sort();
}

function selectedModules() {
  const shard = option('--shard');
  if (!shard) return modules();
  const match = /^([1-9][0-9]*)\/([1-9][0-9]*)$/u.exec(shard);
  if (!match || Number(match[1]) > Number(match[2])) {
    console.error(`--shard takes K/N with 1 <= K <= N, not ${shard}`);
    process.exit(2);
  }
  return shardOf(modules(), Number(match[1]), Number(match[2]));
}

/** The hand-written Rust module `module` corresponds to, or null. */
function counterpart(module) {
  const path = posix.relative('js/src', module).replace(/\.js$/u, '').replaceAll('-', '_');
  return [`rust/src/${path}.rs`, `rust/src/${path}/mod.rs`].find((candidate) => existsSync(join(root, candidate))) ?? null;
}

/** The top-level Rust functions of `text`, by name, with their text. */
export function rustFunctionDefinitions(text) {
  const tree = parseProgrammingLanguage(text, 'Rust')?.tree;
  const bytes = Buffer.from(text, 'utf8');
  const found = new Map();
  for (const child of tree?.children ?? []) {
    if (child.term !== 'function_item') continue;
    const { start, end } = child.span.byteRange;
    const code = bytes.subarray(start, end).toString('utf8');
    const identifier = child.children.find((node) => node.field === 'name');
    const name = identifier && bytes.subarray(identifier.span.byteRange.start, identifier.span.byteRange.end).toString('utf8');
    if (name && !found.has(name)) found.set(name, code);
  }
  return found;
}

/** The code lines of `text`: trimmed, without blank lines and line comments. */
const codeLines = (text) => text.split('\n').map((line) => line.trim()).filter((line) => line && !line.startsWith('//'));
const normalized = (text) => text.replace(/\s+/gu, ' ').trim();

/** The translated definitions of a self-translation into Rust, without its header and prelude. */
function translatedCode(code) {
  const end = code.indexOf('// meta-language:prelude end');
  return end === -1 ? code : code.slice(end);
}

/** Account for every UTF-8 source byte independently of the emitted code. */
export function sourceCoverage(source, items) {
  const bytes = Buffer.from(source, 'utf8');
  const coverage = { sourceBytes: bytes.length, itemBytes: 0, layoutBytes: 0, unrepresentedBytes: 0, unrepresentedRanges: [], bytesByStatus: {} };
  let cursor = 0;
  const gap = (end) => {
    if (end === cursor) return;
    const text = bytes.subarray(cursor, end).toString('utf8');
    if (/^\s*$/u.test(text)) coverage.layoutBytes += end - cursor;
    else {
      coverage.unrepresentedBytes += end - cursor;
      coverage.unrepresentedRanges.push({ start: cursor, end });
    }
  };
  const boundary = (offset) => offset === bytes.length || (bytes[offset] & 0xc0) !== 0x80;
  for (const { start, end, status } of items) {
    if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end) || start < cursor || end <= start || end > bytes.length || !boundary(start) || !boundary(end)) {
      throw new Error(`invalid self-translation source range ${start}..${end} after ${cursor} of ${bytes.length} bytes`);
    }
    if (!['translated', 'carried', 'comment', 'kept', 'restored', 'provenance'].includes(status)) {
      throw new Error(`unknown self-translation item status ${status}`);
    }
    gap(start);
    coverage.itemBytes += end - start;
    coverage.bytesByStatus[status] = (coverage.bytesByStatus[status] ?? 0) + end - start;
    cursor = end;
  }
  gap(bytes.length);
  return coverage;
}

function checkedCoverage(source, translation) {
  const coverage = sourceCoverage(source, translation.items);
  if (coverage.unrepresentedBytes !== 0) {
    throw new Error(`self-translation omitted ${coverage.unrepresentedBytes} source bytes at ${coverage.unrepresentedRanges.map(({ start, end }) => `${start}..${end}`).join(', ')}`);
  }
  return coverage;
}

/**
 * Resolve actual translated exports before measuring their importers. A
 * signature is supplied only when the provider's translation checked it;
 * missing exports and cycles keep the ordinary carried-item diagnostic.
 * Source paths are relative to js/src, independent of the report shard.
 */
export function createModuleContext(readSource) {
  const sources = new Map();
  const read = (module) => {
    if (!sources.has(module)) sources.set(module, readSource(module));
    return sources.get(module);
  };
  const contexts = new Map();
  const signatures = new Map();
  const visiting = new Set();
  const context = (module) => {
    if (contexts.has(module)) return contexts.get(module);
    const directory = posix.dirname(module);
    const moduleDirectory = directory === '.' ? [] : directory.split('/');
    const imports = {};
    const result = { moduleDirectory, imports };
    const source = read(module);
    if (source === null || visiting.has(module)) return result;
    visiting.add(module);
    try {
      const bytes = Buffer.from(source, 'utf8');
      const tree = parseProgrammingLanguage(source, 'JavaScript')?.tree;
      for (const item of tree?.children ?? []) {
        if (item.term !== 'import_statement') continue;
        const name = item.children.find((child) => child.field === 'source');
        if (!name) continue;
        const text = bytes.subarray(name.span.byteRange.start, name.span.byteRange.end).toString('utf8');
        const specifier = tokenize(text, 'JavaScript').tokens[0]?.value;
        if (!specifier || !/^\.\.?\//u.test(specifier)) continue;
        const target = posix.normalize(posix.join(directory, specifier));
        if (target === '..' || target.startsWith('../') || visiting.has(target)) continue;
        if (!signatures.has(target)) {
          const provider = read(target);
          if (provider === null) continue;
          signatures.set(target, selfTranslationSignatures(provider, 'JavaScript', context(target)));
        }
        imports[specifier] = signatures.get(target);
      }
      contexts.set(module, result);
      return result;
    } finally {
      visiting.delete(module);
    }
  };
  return context;
}

function measure(module, decorators, context) {
  const rust = counterpart(module);
  const source = readFileSync(join(root, module), 'utf8');
  const started = performance.now();
  const translation = selfTranslate(source, 'JavaScript', 'Rust', context);
  const coverage = checkedCoverage(source, translation);
  const items = Object.fromEntries(STATUSES.map((status) => [status, translation.items.filter((item) => item.status === status).length]));
  const milliseconds = Math.round(performance.now() - started);
  const handWritten = rust ? readFileSync(join(root, rust), 'utf8') : null;
  const generic = compareRustDefinitions(translation.code, handWritten);
  let decorated = { ...generic, coverage };
  if (decorators.size > 0) {
    const translation = selfTranslate(source, 'JavaScript', 'Rust', { ...context, decorators });
    decorated = { ...compareRustDefinitions(translation.code, handWritten), coverage: checkedCoverage(source, translation) };
  }
  const sha256 = (file) => createHash('sha256').update(readFileSync(join(root, file))).digest('hex');
  return { module, rust, sourceSha256: sha256(module), rustSha256: rust ? sha256(rust) : null, items, coverage, milliseconds, ...generic, decorated };
}

/** How the Rust a translation writes compares with the hand-written Rust, or with none. */
export function compareRustDefinitions(translated, handWritten) {
  const code = translatedCode(translated);
  const written = rustFunctionDefinitions(code);
  const row = { functions: written.size };
  if (handWritten === null) return { ...row, matched: 0, identical: 0, codeLines: codeLines(code).length, sharedLines: 0, handWrittenLines: 0 };
  const existing = rustFunctionDefinitions(handWritten);
  let matched = 0;
  let identical = 0;
  for (const [name, text] of written) {
    if (!existing.has(name)) continue;
    matched += 1;
    if (normalized(existing.get(name)) === normalized(text)) identical += 1;
  }
  const available = new Map();
  for (const line of codeLines(handWritten)) available.set(line, (available.get(line) ?? 0) + 1);
  let sharedLines = 0;
  const lines = codeLines(code);
  for (const line of lines) {
    if ((available.get(line) ?? 0) === 0) continue;
    available.set(line, available.get(line) - 1);
    sharedLines += 1;
  }
  return { ...row, matched, identical, codeLines: lines.length, sharedLines, handWrittenLines: codeLines(handWritten).length };
}

function markdown(rows) {
  const total = (field) => rows.reduce((sum, row) => sum + row[field], 0);
  const items = (status) => rows.reduce((sum, row) => sum + row.items[status], 0);
  const decorated = (field) => rows.reduce((sum, row) => sum + row.decorated[field], 0);
  const lines = [
    '# Self-translation of JavaScript modules against hand-written Rust',
    '',
    `${rows.length} modules; ${items('translated')} items translated, ${items('carried')} carried, ${items('comment')} comment groups copied.`,
    `${rows.reduce((sum, row) => sum + row.coverage.sourceBytes, 0)} source bytes accounted for by item ranges and whitespace layout; modules with omitted code are refused.`,
    `${total('functions')} Rust functions written, ${total('matched')} named as in the hand-written Rust, ${total('identical')} identical to it up to whitespace;`,
    `${total('sharedLines')} of ${total('codeLines')} translated code lines appear in the hand-written Rust (${total('handWrittenLines')} code lines).`,
    `With the shared decorators: ${decorated('identical')} functions identical, ${decorated('sharedLines')} of ${decorated('codeLines')} translated code lines shared.`,
    '',
    '| JavaScript module | Rust module | translated | carried | functions | same name | identical | identical, decorated | shared / translated lines | shared, decorated | hand-written lines |',
    '| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |',
    ...rows.map((row) => `| ${row.module} | ${row.rust ?? '—'} | ${row.items.translated} | ${row.items.carried} | ${row.functions} | ${row.matched} | ${row.identical} | ${row.decorated.identical} | ${row.sharedLines} / ${row.codeLines} | ${row.decorated.sharedLines} / ${row.decorated.codeLines} | ${row.handWrittenLines} |`),
    '',
    '| JavaScript module | source bytes | translated bytes | carried bytes | comment bytes | other item bytes | layout bytes |',
    '| --- | ---: | ---: | ---: | ---: | ---: | ---: |',
    ...rows.map(({ module, coverage: c }) => `| ${module} | ${c.sourceBytes} | ${c.bytesByStatus.translated ?? 0} | ${c.bytesByStatus.carried ?? 0} | ${c.bytesByStatus.comment ?? 0} | ${c.itemBytes - (c.bytesByStatus.translated ?? 0) - (c.bytesByStatus.carried ?? 0) - (c.bytesByStatus.comment ?? 0)} | ${c.layoutBytes} |`),
  ];
  return `${lines.join('\n')}\n`;
}


const hash = (text) => createHash('sha256').update(text).digest('hex');

// Acceptance reads the complete reports published by CI rather than rerunning
// the whole corpus inside either test suite. Hashes bind each measurement to
// the JavaScript and Rust sources of the observed commit.
function verifyPublishedReports(directory, commit) {
  assert.match(commit ?? '', /^[0-9a-f]{40}$/u, 'the observed commit is required');
  const collect = (location) => readdirSync(location, { withFileTypes: true }).flatMap((entry) => {
    const file = join(location, entry.name);
    return entry.isDirectory() ? collect(file) : entry.name === 'self-translation-report.json' ? [file] : [];
  });
  const expected = new Set(listSourceModules());
  const seen = new Set();
  const decorators = DecoratorSet.fromLino(readFileSync(join(root, DEFAULT_DECORATORS), 'utf8')).ids();
  const validateMeasurements = (row, module) => {
    for (const field of ['functions', 'matched', 'identical', 'codeLines', 'sharedLines', 'handWrittenLines']) {
      assert.ok(Number.isSafeInteger(row?.[field]) && row[field] >= 0, `${module}: invalid measurements (${field})`);
    }
    assert.ok(row.identical <= row.matched && row.matched <= row.functions && row.sharedLines <= row.codeLines && row.sharedLines <= row.handWrittenLines,
      `${module}: inconsistent measurements`);
  };
  const files = collect(directory);
  assert.ok(files.length > 0, 'no published self-translation reports');
  for (const file of files) {
    const report = JSON.parse(readFileSync(file, 'utf8'));
    assert.equal(report.schemaVersion, 1, `${file}: report schema`);
    assert.equal(report.commit, commit, `${file}: report commit`);
    assert.deepEqual(report.failures, [], `${file}: failed modules`);
    assert.deepEqual(report.decorators, decorators, `${file}: decorators`);
    assert.ok(Array.isArray(report.modules), `${file}: module rows`);
    const markdown = readFileSync(join(dirname(file), 'self-translation-report.md'), 'utf8');
    for (const row of report.modules) {
      assert.ok(expected.has(row.module), `${file}: unknown module ${row.module}`);
      assert.ok(!seen.has(row.module), `${file}: duplicate module ${row.module}`);
      seen.add(row.module);
      assert.equal(row.rust, counterpart(row.module), `${row.module}: Rust counterpart`);
      assert.equal(row.sourceSha256, hash(readFileSync(join(root, row.module))), `${row.module}: source hash`);
      assert.equal(row.rustSha256, row.rust ? hash(readFileSync(join(root, row.rust))) : null, `${row.module}: Rust hash`);
      validateMeasurements(row, row.module);
      validateMeasurements(row.decorated, row.module);
      assert.equal(row.decorated.handWrittenLines, row.handWrittenLines, `${row.module}: hand-written measurements`);
      for (const status of STATUSES) assert.ok(Number.isSafeInteger(row.items?.[status]) && row.items[status] >= 0, `${row.module}: item measurements`);
      assert.ok(markdown.includes(`| ${row.module} |`), `${row.module}: missing Markdown row`);
    }
  }
  for (const module of expected) assert.ok(seen.has(module), `missing module ${module}`);
  console.log(`verified ${seen.size} modules at ${commit}`);
}


function main() {
  if (process.argv.includes('--verify-reports')) {
    try {
      verifyPublishedReports(option('--verify-reports'), option('--commit'));
    } catch (error) {
      console.error(String(error.message ?? error));
      process.exitCode = 1;
    }
    return;
  }
  if (process.argv.includes('--list')) {
    console.log(selectedModules().join('\n'));
    process.exit(0);
  }
  const outDir = option('--out-dir');
  if (!outDir) {
    console.error('usage: generate-self-translation-report.mjs --out-dir <dir> [--modules a.js,b.js] [--decorators file.lino] [--shard K/N] [--list]');
    process.exit(2);
  }
  const decorators = DecoratorSet.fromLino(readFileSync(join(root, option('--decorators') ?? DEFAULT_DECORATORS), 'utf8'));
  const rows = [];
  const failures = [];
  const context = createModuleContext((module) => {
    const file = join(root, 'js/src', module);
    return existsSync(file) && module.endsWith('.js') ? readFileSync(file, 'utf8') : null;
  });
  for (const module of selectedModules()) {
    try {
      rows.push(measure(module, decorators, context(posix.relative('js/src', module))));
    } catch (error) {
      failures.push({ module, error: String(error?.message ?? error) });
    }
  }
  mkdirSync(outDir, { recursive: true });
  const report = markdown(rows) + (failures.length ? `\n## Modules the self-translation refused\n\n${failures.map(({ module, error }) => `- ${module}: ${error}`).join('\n')}\n` : '');
  writeFileSync(join(outDir, 'self-translation-report.md'), report);
  writeFileSync(join(outDir, 'self-translation-report.json'), `${JSON.stringify({ schemaVersion: 1, commit: process.env.GITHUB_SHA ?? execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim(), decorators: decorators.ids(), modules: rows, failures }, null, 2)}\n`);
  // Refused modules are listed in the report; the tests hold self-translation to its contract.
  console.log(report);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main();
