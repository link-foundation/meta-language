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
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { DecoratorSet } from '../src/decorators.js';
import { parseProgrammingLanguage } from '../src/programming-language-parser.js';
import { selfTranslate } from '../src/self-translation.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const STATUSES = ['translated', 'carried', 'comment'];
const DEFAULT_DECORATORS = 'parity/self-translation/decorators.lino';

function option(name) {
  const index = process.argv.indexOf(name);
  return index === -1 ? null : process.argv[index + 1];
}

function modules() {
  const listed = option('--modules');
  if (listed) return listed.split(',').map((file) => join('js/src', file));
  const walk = (directory) => readdirSync(join(root, directory), { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
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
  const path = relative('js/src', module).replace(/\.js$/u, '').replaceAll('-', '_');
  return [`rust/src/${path}.rs`, `rust/src/${path}/mod.rs`].find((candidate) => existsSync(join(root, candidate))) ?? null;
}

/** The top-level Rust functions of `text`, by name, with their text. */
function functions(text) {
  const tree = parseProgrammingLanguage(text, 'Rust')?.tree;
  const found = new Map();
  for (const child of tree?.children ?? []) {
    if (child.term !== 'function_item') continue;
    const code = Buffer.from(text, 'utf8').subarray(child.start, child.end).toString('utf8');
    const name = /\bfn\s+([A-Za-z_][A-Za-z0-9_]*)/u.exec(code)?.[1];
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

function measure(module, decorators) {
  const rust = counterpart(module);
  const source = readFileSync(join(root, module), 'utf8');
  const started = performance.now();
  const translation = selfTranslate(source, 'JavaScript', 'Rust');
  const items = Object.fromEntries(STATUSES.map((status) => [status, translation.items.filter((item) => item.status === status).length]));
  const milliseconds = Math.round(performance.now() - started);
  const handWritten = rust ? readFileSync(join(root, rust), 'utf8') : null;
  const generic = compare(translation.code, handWritten);
  const decorated = decorators.size > 0 ? compare(selfTranslate(source, 'JavaScript', 'Rust', { decorators }).code, handWritten) : generic;
  return { module, rust, items, milliseconds, ...generic, decorated };
}

/** How the Rust a translation writes compares with the hand-written Rust, or with none. */
function compare(translated, handWritten) {
  const code = translatedCode(translated);
  const written = functions(code);
  const row = { functions: written.size };
  if (handWritten === null) return { ...row, matched: 0, identical: 0, codeLines: codeLines(code).length, sharedLines: 0, handWrittenLines: 0 };
  const existing = functions(handWritten);
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
    `${total('functions')} Rust functions written, ${total('matched')} named as in the hand-written Rust, ${total('identical')} identical to it up to whitespace;`,
    `${total('sharedLines')} of ${total('codeLines')} translated code lines appear in the hand-written Rust (${total('handWrittenLines')} code lines).`,
    `With the shared decorators: ${decorated('identical')} functions identical, ${decorated('sharedLines')} of ${decorated('codeLines')} translated code lines shared.`,
    '',
    '| JavaScript module | Rust module | translated | carried | functions | same name | identical | identical, decorated | shared / translated lines | shared, decorated | hand-written lines |',
    '| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |',
    ...rows.map((row) => `| ${row.module} | ${row.rust ?? '—'} | ${row.items.translated} | ${row.items.carried} | ${row.functions} | ${row.matched} | ${row.identical} | ${row.decorated.identical} | ${row.sharedLines} / ${row.codeLines} | ${row.decorated.sharedLines} / ${row.decorated.codeLines} | ${row.handWrittenLines} |`),
  ];
  return `${lines.join('\n')}\n`;
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
for (const module of selectedModules()) {
  try {
    rows.push(measure(module, decorators));
  } catch (error) {
    failures.push({ module, error: String(error?.message ?? error) });
  }
}
mkdirSync(outDir, { recursive: true });
const report = markdown(rows) + (failures.length ? `\n## Modules the self-translation refused\n\n${failures.map(({ module, error }) => `- ${module}: ${error}`).join('\n')}\n` : '');
writeFileSync(join(outDir, 'self-translation-report.md'), report);
writeFileSync(join(outDir, 'self-translation-report.json'), `${JSON.stringify({ decorators: decorators.ids(), modules: rows, failures }, null, 2)}\n`);
// Refused modules are listed in the report; the tests hold self-translation to its contract.
console.log(report);
