// Projects concrete syntax trees to the rows of
// parity/fixtures/default-cst-expected.json, `[depth, field, kind, named,
// startByte, endByte, flags]`, so a native grammar's trees can be compared
// with its tree-sitter oracle, the pinned grammar the language's default parse
// used before the native grammar replaced it. The oracle rows come from
// tree-sitter directly, not from the default parse, which now runs the native
// grammar. rust/tests/unit/issue_195_native_grammar_rows.rs projects the Rust
// trees the same way.
//
// The native tree is projected as tree-sitter places nodes: whitespace trivia
// is not a row, named trivia (comments) is an extra row (flag X), leading
// trivia belongs before the node it precedes, a node spans its first to last
// non-trivia leaf, and the root starts at its first leaf that is not
// whitespace and ends at the end of the input, unless the fixture's root
// includes leading trivia and retains the native tree's input start.
// Kinds in `hidden` are leaves
// the native tree keeps and the oracle drops, such as a byte order mark; they
// are projected like whitespace. Kinds in `anonymous` are leaves the oracle
// keeps inside a node without a row of their own, as tree-sitter keeps a
// regular expression token such as a line break; they are not rows but count
// in the spans. Node kinds in `extras` are rows with flag X, as the oracle
// marks a comment node it parses as an extra. A rule renamed from its
// tree-sitter name is a row of that name (`oracleKinds`). A kind `'TEXT`, an
// imported anonymous alias, is an anonymous row TEXT. An ERROR leaf is a named
// `ERROR` row with flag E and a MISSING leaf an empty row with flag M, named
// unless it stands for a literal.
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { gunzipSync } from 'node:zlib';
import { Parser } from 'web-tree-sitter';

import { languageEntry } from '../src/index.js';
import { loadGrammarLanguage } from '../src/grammar-tiering.js';
import { treeSitterNodeKind } from '../src/tree-sitter-node-kind.js';
import { grammarFile } from './grammar-files.mjs';

const encoder = new TextEncoder();
const ORACLES = new Map();
const LOCK = JSON.parse(readFileSync(new URL('../src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));
const ISOLATED_ORACLES = new Set(JSON.parse(readFileSync(new URL('../../parity/grammars/sources.json', import.meta.url), 'utf8')).sources
  .filter(source => source.oracleIsolation?.mode === 'process').map(source => source.oracle));

function isolatedOracle(source, language) {
  const entry = languageEntry(language);
  const id = (entry.oracleGrammars ?? entry.grammars)[0].id;
  if (!ISOLATED_ORACLES.has(id) || process.env.META_LANGUAGE_ORACLE_WORKER === '1') return null;
  const run = spawnSync(process.execPath, [fileURLToPath(new URL('./isolated-grammar-oracle.mjs', import.meta.url))], {
    input: JSON.stringify({ source, language }), encoding: 'utf8',
    env: { ...process.env, META_LANGUAGE_ORACLE_WORKER: '1' },
    maxBuffer: 64 * 1024 * 1024, timeout: 30_000,
  });
  if (run.error || run.status !== 0) throw new Error(`isolated ${id} oracle failed: ${run.error?.message ?? run.stderr}`);
  const result = JSON.parse(run.stdout);
  if (!Array.isArray(result.rows) || typeof result.recovers !== 'boolean') throw new Error(`isolated ${id} oracle returned an invalid snapshot`);
  return result;
}


/** The pinned tree-sitter grammar that is the oracle of `language`. */
function oracleLanguage(language) {
  const entry = languageEntry(language);
  const id = (entry.oracleGrammars ?? entry.grammars)[0].id;
  let grammar = ORACLES.get(id);
  if (!grammar) {
    const file = new URL(`../../${grammarFile(LOCK.grammars[id], `${id}.wasm.gz`)}`, import.meta.url);
    grammar = loadGrammarLanguage(gunzipSync(readFileSync(file)));
    ORACLES.set(id, grammar);
  }
  return grammar;
}

/** Calls `use` with the oracle's tree of `source`, and frees the tree. */
function withOracleTree(source, language, use) {
  const parser = new Parser();
  parser.setLanguage(oracleLanguage(language));
  const bytes = encoder.encode(source);
  const tree = parser.parse((index) => {
    let end = Math.min(bytes.length, index + 4096);
    while (end < bytes.length && (bytes[end] & 0xc0) === 0x80) end -= 1;
    return String.fromCharCode(...bytes.subarray(index, end));
  });
  parser.delete();
  try {
    return use(tree.rootNode);
  } finally {
    tree.delete();
  }
}

const flagsOf = (node) => `${node.isError ? 'E' : ''}${node.isMissing ? 'M' : ''}${node.isExtra ? 'X' : ''}`;

function oracleTreeRows(root) {
  const rows = [];
  const walk = (node, depth, field) => {
    rows.push([depth, field, treeSitterNodeKind(node), node.isNamed ? 1 : 0, node.startIndex, node.endIndex, flagsOf(node)]);
    node.children.forEach((child, index) => walk(child, depth + 1, node.fieldNameForChild(index)));
  };
  walk(root, 0, null);
  return rows;
}

/** One independent parse provides both the rows and recovery diagnostics. */
export function oracleSnapshot(source, language) {
  const isolated = isolatedOracle(source, language);
  if (isolated) return isolated;
  return withOracleTree(source, language, root => ({ rows: oracleTreeRows(root), recovers: root.hasError }));
}

/** The rows of the oracle's tree of `source` as `language`. */
export function oracleRows(source, language) {
  const isolated = isolatedOracle(source, language);
  return isolated ? isolated.rows : withOracleTree(source, language, oracleTreeRows);
}

/** The rows of a native `SyntaxTree` of `source`. */
export function nativeRows(tree, source, { hidden = [], anonymous = [], extras = [], oracleKinds = {}, rootIncludesLeadingTrivia = false } = {}) {
  const oracleNames = new Map(Object.entries(oracleKinds));
  const oracleKind = (kind) => oracleNames.get(kind) ?? kind;
  const anonymousAlias = (kind) => typeof kind === 'string' && kind.startsWith("'");
  const term = (kind) => (anonymousAlias(kind) ? kind.slice(1) : oracleKind(kind));
  const hiddenKinds = new Set(hidden);
  const anonymousKinds = new Set(anonymous);
  const extraKinds = new Set(extras);
  const invisible = (node) => node.type === 'token' && ((node.trivia && node.kind === null) || hiddenKinds.has(node.kind));
  const trivia = (node) => invisible(node) || (node.type === 'token' && node.trivia) || (node.type === 'node' && extraKinds.has(node.kind));
  const hoist = (node) => {
    if (node.type !== 'node') return [node];
    const children = node.children.flatMap(hoist);
    let first = 0;
    while (first < children.length && trivia(children[first])) first += 1;
    return [...children.slice(0, first), { ...node, children: children.slice(first) }];
  };
  const span = (node) => {
    if (node.type !== 'node') return [node.start, node.end];
    const inner = node.children.filter((child) => !trivia(child)).map(span);
    return inner.length === 0 ? [node.start, node.start] : [inner[0][0], inner.at(-1)[1]];
  };
  // A MISSING leaf of such a kind is a token tree-sitter hides, so no row.
  const hiddenMissing = (node) => node.type === 'missing' && (hiddenKinds.has(node.kind) || anonymousKinds.has(node.kind));
  const rows = [];
  const visit = (node, depth) => {
    if (invisible(node) || (node.type === 'token' && anonymousKinds.has(node.kind)) || hiddenMissing(node)) return;
    const [start, end] = span(node);
    if (node.type === 'node') {
      rows.push([depth, node.field ?? null, term(node.kind), anonymousAlias(node.kind) ? 0 : 1, start, end, extraKinds.has(node.kind) ? 'X' : '']);
      for (const child of node.children.flatMap(hoist)) visit(child, depth + 1);
    } else if (node.type === 'error') {
      rows.push([depth, null, 'ERROR', 1, start, end, 'E']);
    } else if (node.type === 'missing') {
      rows.push([depth, null, term(node.kind), node.literal || anonymousAlias(node.kind) ? 0 : 1, start, end, 'M']);
    } else {
      const named = Boolean(node.kind) && !anonymousAlias(node.kind);
      rows.push([depth, node.field ?? null, node.kind ? term(node.kind) : node.text, named ? 1 : 0, start, end, node.trivia ? 'X' : '']);
    }
  };
  const length = Buffer.byteLength(source);
  const leaves = [];
  const collect = (node) => (node.type === 'node' ? node.children.forEach(collect) : leaves.push(node));
  collect(tree);
  const first = leaves.find((leaf) => !invisible(leaf));
  rows.push([0, null, oracleKind(tree.kind), 1, rootIncludesLeadingTrivia ? tree.start : first ? first.start : length, length, '']);
  for (const child of tree.children.flatMap(hoist)) visit(child, 1);
  return rows;
}

/** Whether any row carries an error or missing flag. */
export const hasRecovery = (rows) => rows.some((row) => /[EM]/u.test(row[6]));

/**
 * Whether the oracle recovers from an error in `source`: a row with an error
 * or missing flag, or a missing token the rows do not show, such as a line
 * break tree-sitter inserts at the end of the input, which only the root's
 * has-error flag records.
 */
export function oracleRecovers(source, language) {
  const isolated = isolatedOracle(source, language);
  if (isolated) return isolated.recovers;
  return withOracleTree(source, language, (root) => root.hasError);
}

/** Compact assertion diagnostics retain the first differing row for every corpus failure. */
export function nativeCorpusFailure(label, error) {
  const { actual, expected } = error;
  if (Array.isArray(actual) && Array.isArray(expected)) {
    let index = 0;
    while (index < Math.max(actual.length, expected.length) && JSON.stringify(actual[index]) === JSON.stringify(expected[index])) index += 1;
    return `${label}: first difference ${index}: ${JSON.stringify({ actual: actual[index] ?? null, expected: expected[index] ?? null, actualLength: actual.length, expectedLength: expected.length })}`;
  }
  const compact = (value) => typeof value === 'string' && value.length > 120 ? { prefix: value.slice(0, 120), length: value.length } : value;
  return `${label}: ${JSON.stringify({ actual: compact(actual), expected: compact(expected), message: error.message.slice(0, 250) })}`;
}
