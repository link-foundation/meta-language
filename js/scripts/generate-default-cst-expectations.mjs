#!/usr/bin/env node
// Generates parity/fixtures/default-cst-expected.json: for every grammar
// language in parity/language-grammar-inventory.json, the concrete syntax tree
// its vendored tree-sitter grammar produces for the inventory's positive and
// recovery sources, plus the embedded-language regions the host grammar
// delimits. The trees come straight from the grammar (no meta-language adapter)
// and, with --cli, are cross-checked against the native tree-sitter CLI built
// from the same pinned grammar sources that rust/Cargo.lock compiles.
//
//   node js/scripts/generate-default-cst-expectations.mjs          # write
//   node js/scripts/generate-default-cst-expectations.mjs --check  # verify
//   TREE_SITTER_CLI=tree-sitter node js/scripts/generate-default-cst-expectations.mjs --cli
//
// A row is [depth, field, kind, named, startByte, endByte, flags] in
// depth-first order, with UTF-8 byte offsets and flags drawn from
// E (error node), M (missing node) and X (extra node, such as a comment).
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { cp, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from 'web-tree-sitter';

import {
  GRAMMAR_SOURCES,
  cargoLockVersions,
  ensureGrammarConfig,
  grammarSource,
} from './build-vendored-grammars.mjs';

const run = promisify(execFile);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = join(root, 'parity/language-grammar-inventory.json');
const expectedPath = join(root, 'parity/fixtures/default-cst-expected.json');
const grammarDir = join(root, 'js/src/vendor/grammars');
const encoder = new TextEncoder();
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

/**
 * Where a meta-language network deliberately differs from the raw grammar
 * tree; the conformance tests undo exactly these before comparing rows.
 */
export const PUBLIC_PROJECTIONS = Object.freeze({
  Lean: 'The public root is a `file` node spanning the whole source whose only grammar child is the grammar root `module`.',
  Rocq: 'Every `ident` leaf keeps its grammar node and gains one child token named `identifier`, or `primitive_type` for bool, nat, Prop, Set, SProp, Type and Z.',
  Markdown: 'Each block `inline` and `pipe_table_cell` node holds the children of the inline grammar tree parsed over its range minus its named children after the first (upstream MarkdownParser included ranges); those excluded block children sit under the deepest inline node containing them, otherwise among the children by start byte, block nodes first on ties.',
  'CSS style attribute': 'A declaration list without a final `;` or `}` is parsed with `;` appended; nodes after the attribute value are dropped and ends are clipped to it.',
  'Zero-width nodes': 'MISSING nodes and other zero-width grammar nodes are Syntax links without a source token.',
});

const MARKDOWN_INLINE_CONTAINERS = new Set(['inline', 'pipe_table_cell']);

await Parser.init();
const languages = new Map();
async function loadGrammar(id) {
  if (!languages.has(id)) {
    languages.set(id, await Language.load(gunzipSync(await readFile(join(grammarDir, `${id}.wasm.gz`)))));
  }
  return languages.get(id);
}

/** UTF-16 index -> UTF-8 byte offset for every index of `text`. */
function byteOffsets(text) {
  const offsets = new Uint32Array(text.length + 1);
  let byte = 0;
  for (let index = 0; index < text.length; index += 1) {
    offsets[index] = byte;
    const unit = text.charCodeAt(index);
    if (unit >= 0xd800 && unit <= 0xdbff && index + 1 < text.length) {
      offsets[index + 1] = byte;
      byte += 4;
      index += 1;
    } else {
      byte += unit < 0x80 ? 1 : unit < 0x800 ? 2 : 3;
    }
  }
  offsets[text.length] = byte;
  return offsets;
}

function flagsOf(node) {
  return `${node.isError ? 'E' : ''}${node.isMissing ? 'M' : ''}${node.isExtra ? 'X' : ''}`;
}

async function parse(grammar, text, options) {
  const parser = new Parser();
  parser.setLanguage(await loadGrammar(grammar));
  const tree = parser.parse(text, null, options);
  parser.delete();
  if (!tree) throw new Error(`${grammar} returned no tree`);
  return tree;
}

/** Rows of a plain grammar tree. */
function grammarRows(tree, text) {
  const bytes = byteOffsets(text);
  const rows = [];
  const walk = (node, depth, field) => {
    rows.push([depth, field, node.type, node.isNamed ? 1 : 0, bytes[node.startIndex], bytes[node.endIndex], flagsOf(node)]);
    node.children.forEach((child, index) => walk(child, depth + 1, node.fieldNameForChild(index)));
  };
  walk(tree.rootNode, 0, null);
  return rows;
}

// Markdown: upstream tree-sitter-md's MarkdownParser parses every block
// `inline`/`pipe_table_cell` node again with the inline grammar, over the
// node's range minus its named children after the first.
async function markdownRows(text) {
  const bytes = byteOffsets(text);
  const block = await parse('markdown', text);
  const inlineTrees = [];
  const rows = [];
  const contains = (outer, inner) =>
    outer.startIndex <= inner.startIndex && inner.endIndex <= outer.endIndex &&
    (inner.startIndex < inner.endIndex || (outer.startIndex < inner.startIndex && inner.startIndex < outer.endIndex));
  // `extra` holds block nodes (excluded from an inline parse) to place among
  // this node's children.
  const walk = async (node, depth, field, extra) => {
    rows.push([depth, field, node.type, node.isNamed ? 1 : 0, bytes[node.startIndex], bytes[node.endIndex], flagsOf(node)]);
    let children = node.children.map((child, index) => ({ node: child, field: node.fieldNameForChild(index), extra: [] }));
    let injected = extra;
    if (MARKDOWN_INLINE_CONTAINERS.has(node.type) && node.tree === block) {
      const excluded = node.children.slice(1).filter((child) => child.isNamed);
      const ranges = [];
      let start = { index: node.startIndex, position: node.startPosition };
      for (const child of excluded) {
        ranges.push({ startIndex: start.index, startPosition: start.position, endIndex: child.startIndex, endPosition: child.startPosition });
        start = { index: child.endIndex, position: child.endPosition };
      }
      ranges.push({ startIndex: start.index, startPosition: start.position, endIndex: node.endIndex, endPosition: node.endPosition });
      const inline = await parse('markdown_inline', text, { includedRanges: ranges });
      inlineTrees.push(inline);
      const inlineRoot = inline.rootNode;
      children = inlineRoot.children.map((child, index) => ({ node: child, field: inlineRoot.fieldNameForChild(index), extra: [] }));
      injected = [...extra, ...excluded];
    }
    const top = [];
    for (const item of injected) {
      const owner = children.find((child) => contains(child.node, item));
      if (owner) owner.extra.push(item);
      else top.push({ node: item, field: null, extra: [] });
    }
    if (top.length) children = [...top, ...children].sort((left, right) => left.node.startIndex - right.node.startIndex);
    for (const child of children) await walk(child.node, depth + 1, child.field, child.extra);
  };
  await walk(block.rootNode, 0, null, []);
  for (const tree of [block, ...inlineTrees]) tree.delete();
  return rows;
}

async function languageRows(grammars, text) {
  if (grammars.includes('markdown_inline')) return markdownRows(text);
  const tree = await parse(grammars[0], text);
  try {
    return grammarRows(tree, text);
  } finally {
    tree.delete();
  }
}

// Embedded regions the host grammar delimits. CSS in a style attribute is a
// declaration list, so a missing final `;` is supplied and clipped away.
function cssNeedsTerminator(text) {
  const trimmed = text.trimEnd();
  return trimmed.length > 0 && !trimmed.endsWith(';') && !trimmed.endsWith('}') && !trimmed.includes('{');
}

async function embeddedRows(language, grammarIds, text, clipToText) {
  if (!clipToText || !cssNeedsTerminator(text)) return languageRows(grammarIds, text);
  const byteEnd = encoder.encode(text).length;
  const rows = await languageRows(grammarIds, `${text};`);
  const kept = [];
  let droppedDepth = null;
  for (const row of rows) {
    if (droppedDepth !== null && row[0] > droppedDepth) continue;
    droppedDepth = null;
    if (row[4] >= byteEnd && row[5] > byteEnd) {
      droppedDepth = row[0];
      continue;
    }
    kept.push(row[5] > byteEnd ? [...row.slice(0, 5), byteEnd, row[6]] : row);
  }
  return kept;
}

function canonicalLanguage(inventory, name) {
  const wanted = name.trim().toLowerCase();
  return inventory.languages.find(
    (language) => language.name.toLowerCase() === wanted || language.aliases.some((alias) => alias.toLowerCase() === wanted),
  )?.name;
}

async function embeddedRegions(inventory, host, text) {
  const hostLanguage = inventory.languages.find((language) => language.name === host);
  if (!['HTML', 'Markdown'].includes(host)) return [];
  const bytes = byteOffsets(text);
  const regions = [];
  const found = [];
  const tree = await parse(hostLanguage.grammars[0], text);
  const visit = (node) => {
    if (host === 'HTML') {
      if ((node.type === 'script_element' || node.type === 'style_element')) {
        const raw = node.children.find((child) => child.type === 'raw_text');
        if (raw) found.push({ path: node.type === 'script_element' ? 'HTML script element' : 'HTML style element', language: node.type === 'script_element' ? 'JavaScript' : 'CSS', node: raw });
      }
      if (node.type === 'attribute') {
        const name = node.children.find((child) => child.type === 'attribute_name');
        const value = node.descendantsOfType('attribute_value')[0];
        if (name?.text.toLowerCase() === 'style' && value) found.push({ path: 'CSS style attribute', language: 'CSS', node: value, clip: true });
      }
    } else if (node.type === 'fenced_code_block') {
      const info = node.descendantsOfType('language')[0];
      const content = node.children.find((child) => child.type === 'code_fence_content');
      const language = info && canonicalLanguage(inventory, info.text);
      if (content && language) found.push({ path: 'Markdown fenced code', language, node: content });
    } else if (node.type === 'html_block') {
      found.push({ path: 'Markdown HTML block', language: 'HTML', node });
    }
    for (const child of node.children) visit(child);
  };
  visit(tree.rootNode);
  for (const { path, language, node, clip } of found) {
    const target = inventory.languages.find((candidate) => candidate.name === language);
    const regionText = text.slice(node.startIndex, node.endIndex);
    regions.push({
      path,
      language,
      startByte: bytes[node.startIndex],
      endByte: bytes[node.endIndex],
      rows: await embeddedRows(language, target.grammars, regionText, clip === true),
    });
  }
  tree.delete();
  return regions;
}

// ---- tree-sitter CLI cross-check ------------------------------------------

async function cliGrammarDirectory(id, versions, scratch) {
  const source = await grammarSource(id, versions);
  const copy = join(scratch, id);
  await cp(source.crateDir, copy, { recursive: true, filter: (path) => !path.includes('/target/') });
  // A vendored grammar keeps only its generated parser, at the root.
  const dir = GRAMMAR_SOURCES[id].vendored ? copy : join(copy, source.dir);
  if (GRAMMAR_SOURCES[id].vendored) {
    // Only the generated parser is vendored; the CLI reads the grammar name
    // from grammar.json.
    await writeFile(join(dir, 'src/parser.c'), source.parser);
    await writeFile(join(dir, 'src/grammar.json'), JSON.stringify({ name: id, rules: {} }));
  }
  await ensureGrammarConfig(dir);
  return dir;
}

// `tree-sitter parse --cst` output for a web-tree-sitter tree, reproducing
// the CLI 0.25.10 renderer (crates/cli/src/parse.rs: cst_render_node,
// write_node_text, render_node_range) so the native parse must match byte for
// byte: kinds, UTF-8 row/column ranges, named fields, error and missing nodes
// and leaf text.
const INVISIBLE = { '\n': '\\n', '\r': '\\r', '\t': '\\t', '\0': '\\0', '\\': '\\\\', '\v': '\\v', '\f': '\\f' };
const escapeInvisible = (text) => [...text].map((character) => INVISIBLE[character] ?? character).join('');
const log10 = (value) => (value > 0 ? Math.floor(Math.log10(value)) : 0);

export function renderCliCst(tree, text) {
  const utf8 = encoder.encode(text);
  const bytes = byteOffsets(text);
  const lineStarts = [0];
  utf8.forEach((byte, index) => { if (byte === 0x0a) lineStarts.push(index + 1); });
  const point = (byte) => {
    let row = lineStarts.length - 1;
    while (lineStarts[row] > byte) row -= 1;
    return { row, column: byte - lineStarts[row] };
  };
  const lines = text.split('\n');
  if (lines.at(-1) === '') lines.pop();
  const totalWidth = Math.max(1, ...lines.map((line, row) => log10(row) + log10(encoder.encode(line.replace(/\r$/u, '')).length) + 1));
  const range = (start, end) => {
    const startWidth = Math.max(1, totalWidth - log10(start.row) - log10(start.column));
    const endWidth = Math.max(1, totalWidth - log10(end.row) - log10(end.column));
    return `${start.row}:${start.column}${' '.repeat(startWidth)}- ${end.row}:${end.column}${' '.repeat(endWidth)}`;
  };
  let out = '';
  let indent = 1;
  let inError = false;
  const render = (node, field) => {
    const startByte = bytes[node.startIndex];
    const endByte = bytes[node.endIndex];
    const start = point(startByte);
    out += `${range(start, point(endByte))}${'  '.repeat(indent)}${inError && !node.hasError ? ' ' : ''}`;
    if (node.isNamed) {
      if (field) out += `${field}: `;
      if (node.hasError || node.isError) out += '•';
      out += `${node.type} `;
      if (node.childCount === 0) {
        const source = new TextDecoder().decode(utf8.subarray(startByte, endByte));
        const pieces = source.match(/[^\n]*\n|[^\n]+$/gu) ?? [];
        const multiline = source.includes('\n');
        pieces.forEach((piece, index) => {
          if (multiline) {
            const row = start.row + index;
            const pieceBytes = encoder.encode(piece).length;
            const column = pieceBytes + (index === 0 ? start.column : 0);
            out += `\n${range({ row, column: start.column }, { row, column })}${'  '.repeat(indent + 1)}`;
          }
          out += `\`${escapeInvisible(piece)}\``;
        });
      }
    } else if (node.isMissing) {
      out += `MISSING: "${node.type}"`;
    } else {
      out += `"${escapeInvisible(node.type)}"`;
    }
    out += '\n';
    if (node.childCount > 0) {
      indent += 1;
      if (node.child(0).hasError) inError = true;
      node.children.forEach((child, index) => render(child, node.fieldNameForChild(index)));
      indent -= 1;
      if (!node.hasError) inError = false;
    }
  };
  render(tree.rootNode, null);
  return out;
}

async function cliOutput(treeSitter, dir, text, scratch, name) {
  const file = join(scratch, `${name.replace(/[^A-Za-z0-9]+/gu, '_')}.src`);
  await writeFile(file, text);
  const options = { cwd: dir, maxBuffer: 64 << 20, env: { ...process.env, NO_COLOR: '1' } };
  let stdout;
  try {
    ({ stdout } = await run(treeSitter, ['parse', '--cst', file], options));
  } catch (error) {
    if (!error.stdout) throw error;
    stdout = error.stdout;
  }
  // The tree ends at the blank line before the parse summary.
  return stdout.replace(/\x1b\[[0-9;]*m/gu, '').split('\n\n')[0].concat('\n');
}

function compareOutput(label, expected, actual) {
  if (expected === actual) return null;
  const left = expected.split('\n');
  const right = actual.split('\n');
  const index = left.findIndex((line, position) => line !== right[position]);
  return `${label}: line ${index + 1}\n  expected ${JSON.stringify(left[index])}\n  CLI      ${JSON.stringify(right[index])}`;
}

// ---- output ------------------------------------------------------------------

function formatExpected(value) {
  const rowsBlock = (rows, indent) => `[\n${rows.map((row) => `${indent}  ${JSON.stringify(row)}`).join(',\n')}\n${indent}]`;
  const render = (item, indent) => {
    if (Array.isArray(item) && item.length && Array.isArray(item[0])) return rowsBlock(item, indent);
    if (Array.isArray(item)) {
      if (item.every((entry) => entry === null || typeof entry !== 'object')) return JSON.stringify(item);
      return `[\n${item.map((entry) => `${indent}  ${render(entry, `${indent}  `)}`).join(',\n')}\n${indent}]`;
    }
    if (item && typeof item === 'object') {
      const entries = Object.entries(item);
      if (!entries.length) return '{}';
      return `{\n${entries.map(([key, entry]) => `${indent}  ${JSON.stringify(key)}: ${render(entry, `${indent}  `)}`).join(',\n')}\n${indent}}`;
    }
    return JSON.stringify(item);
  };
  return `${render(value, '')}\n`;
}

async function generate() {
  const inventory = JSON.parse(await readFile(inventoryPath, 'utf8'));
  const lock = JSON.parse(await readFile(join(grammarDir, 'grammar-lock.json'), 'utf8'));
  const webTreeSitter = JSON.parse(await readFile(join(root, 'js/node_modules/web-tree-sitter/package.json'), 'utf8')).version;
  const result = {};
  for (const language of inventory.languages) {
    if (!language.grammars) continue;
    result[language.name] = {
      grammars: Object.fromEntries(language.grammars.map((id) => [id, {
        version: lock.grammars[id].version,
        parserSha256: lock.grammars[id].parserSha256,
      }])),
      sourceSha256: sha256(language.source),
      recoverySourceSha256: sha256(language.recoverySource),
      positive: await languageRows(language.grammars, language.source),
      recovery: await languageRows(language.grammars, language.recoverySource),
      embedded: await embeddedRegions(inventory, language.name, language.source),
    };
  }
  return {
    description:
      'Grammar concrete syntax trees for the inventory sources, produced directly by the pinned tree-sitter grammars (no meta-language adapter) and cross-checked with the native tree-sitter CLI. Row: [depth, field, kind, named, startByte, endByte, flags]; flags: E error, M missing, X extra.',
    generator: {
      script: 'js/scripts/generate-default-cst-expectations.mjs',
      webTreeSitter,
      treeSitterCli: lock.treeSitterCli,
    },
    publicProjections: PUBLIC_PROJECTIONS,
    languages: result,
  };
}

async function crossCheck(expected, treeSitter) {
  const inventory = JSON.parse(await readFile(inventoryPath, 'utf8'));
  const versions = await cargoLockVersions();
  const scratch = await mkdtemp(join(tmpdir(), 'default-cst-cli-'));
  const directories = new Map();
  const problems = [];
  const checked = [];
  try {
    for (const language of inventory.languages) {
      if (!language.grammars) continue;
      const id = language.grammars[0];
      if (!directories.has(id)) directories.set(id, await cliGrammarDirectory(id, versions, scratch));
      for (const kind of ['positive', 'recovery']) {
        const text = kind === 'positive' ? language.source : language.recoverySource;
        const tree = await parse(id, text);
        const expectedOutput = renderCliCst(tree, text);
        tree.delete();
        const actual = await cliOutput(treeSitter, directories.get(id), text, scratch, `${language.name}-${kind}`);
        const problem = compareOutput(`${language.name} ${kind}`, expectedOutput, actual);
        if (problem) problems.push(problem);
        else checked.push(`${language.name} ${kind}`);
      }
    }
  } finally {
    await rm(scratch, { recursive: true, force: true });
  }
  return { problems, checked };
}

async function main() {
  const args = process.argv.slice(2);
  const expected = await generate();
  const text = formatExpected(expected);
  if (args.includes('--cli')) {
    const treeSitter = process.env.TREE_SITTER_CLI ?? 'tree-sitter';
    const { problems, checked } = await crossCheck(expected, treeSitter);
    if (problems.length) {
      console.error(problems.join('\n'));
      process.exit(1);
    }
    console.log(`tree-sitter CLI agrees on ${checked.length} trees (Markdown: block grammar)`);
  }
  if (args.includes('--check')) {
    if ((await readFile(expectedPath, 'utf8').catch(() => '')) !== text) {
      console.error('parity/fixtures/default-cst-expected.json is stale; run node js/scripts/generate-default-cst-expectations.mjs');
      process.exit(1);
    }
    console.log(`default CST expectations match for ${Object.keys(expected.languages).length} languages`);
    return;
  }
  if (!args.includes('--cli')) {
    await writeFile(expectedPath, text);
    console.log(`wrote ${Object.keys(expected.languages).length} languages to parity/fixtures/default-cst-expected.json`);
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
