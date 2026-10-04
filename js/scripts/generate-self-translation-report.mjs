#!/usr/bin/env node
// Measures, per module, how far the self-translation of each JavaScript module
// of js/src into Rust is from its hand-written Rust counterpart
// (rust/src/<path in snake case>.rs or .../mod.rs), and writes the report as
// Markdown and JSON. CI runs it on every push and publishes both; it reads the
// whole source tree, so it is not meant for a local run (pass --modules to
// measure a few modules).
//
//   node js/scripts/generate-self-translation-report.mjs --out-dir <dir> [--modules a.js,b.js]
//
// For each module the report lists its top-level items by status, the Rust
// functions the translation writes, how many of them the hand-written Rust
// defines under the same name and how many of those are identical up to
// whitespace, and the code lines of the translation (no comments, preludes or
// blank lines) the hand-written Rust holds too, counted as a multiset.
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { parseProgrammingLanguage } from '../src/programming-language-parser.js';
import { selfTranslate } from '../src/self-translation.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const STATUSES = ['translated', 'carried', 'comment'];

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

function measure(module) {
  const rust = counterpart(module);
  const source = readFileSync(join(root, module), 'utf8');
  const started = performance.now();
  const translation = selfTranslate(source, 'JavaScript', 'Rust');
  const items = Object.fromEntries(STATUSES.map((status) => [status, translation.items.filter((item) => item.status === status).length]));
  const row = { module, rust, items, milliseconds: Math.round(performance.now() - started) };
  const code = translatedCode(translation.code);
  const written = functions(code);
  row.functions = written.size;
  if (!rust) return { ...row, matched: 0, identical: 0, codeLines: codeLines(code).length, sharedLines: 0, handWrittenLines: 0 };
  const handWritten = readFileSync(join(root, rust), 'utf8');
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
  const lines = [
    '# Self-translation of JavaScript modules against hand-written Rust',
    '',
    `${rows.length} modules; ${items('translated')} items translated, ${items('carried')} carried, ${items('comment')} comment groups copied.`,
    `${total('functions')} Rust functions written, ${total('matched')} named as in the hand-written Rust, ${total('identical')} identical to it up to whitespace;`,
    `${total('sharedLines')} of ${total('codeLines')} translated code lines appear in the hand-written Rust (${total('handWrittenLines')} code lines).`,
    '',
    '| JavaScript module | Rust module | translated | carried | functions | same name | identical | shared / translated lines | hand-written lines |',
    '| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |',
    ...rows.map((row) => `| ${row.module} | ${row.rust ?? '—'} | ${row.items.translated} | ${row.items.carried} | ${row.functions} | ${row.matched} | ${row.identical} | ${row.sharedLines} / ${row.codeLines} | ${row.handWrittenLines} |`),
  ];
  return `${lines.join('\n')}\n`;
}

const outDir = option('--out-dir');
if (!outDir) {
  console.error('usage: generate-self-translation-report.mjs --out-dir <dir> [--modules a.js,b.js]');
  process.exit(2);
}
const rows = [];
const failures = [];
for (const module of modules()) {
  try {
    rows.push(measure(module));
  } catch (error) {
    failures.push({ module, error: String(error?.message ?? error) });
  }
}
mkdirSync(outDir, { recursive: true });
const report = markdown(rows) + (failures.length ? `\n## Modules the self-translation refused\n\n${failures.map(({ module, error }) => `- ${module}: ${error}`).join('\n')}\n` : '');
writeFileSync(join(outDir, 'self-translation-report.md'), report);
writeFileSync(join(outDir, 'self-translation-report.json'), `${JSON.stringify({ modules: rows, failures }, null, 2)}\n`);
// Refused modules are listed in the report; the tests hold self-translation to its contract.
console.log(report);
