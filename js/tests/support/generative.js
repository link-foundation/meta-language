// Reproducible generation of issue 195 generative cases: a seeded PRNG, source
// mutations that keep UTF-8 boundaries, compositions, edit sequences and the
// metamorphic relations, plus the oracle-free properties every parse must keep
// (round trip, Unicode spans, diagnostics). js/scripts/generate-issue-195-generative.mjs
// uses them to build the fixtures the native tree-sitter CLI then parses, and the
// suites use them to generate more inputs at run time. The PRNG and the property
// checks are mirrored by rust/tests/unit/generative_support.rs.
import { LinkType } from '../../src/index.js';
import { readFileSync } from 'node:fs';

import { parseCstLines } from './cst-lines.js';
import { parseCorpus } from './cst-sexpression.js';

const conformanceRoot = new URL('../../../parity/fixtures/issue-195-conformance/', import.meta.url);
const readConformance = (path) => readFileSync(new URL(path, conformanceRoot), 'utf8');

/** FNV-1a (32-bit) of a UTF-8 string: the numeric seed of a textual seed. */
export function seedNumber(text) {
  let hash = 0x811c9dc5;
  for (const byte of Buffer.from(text, 'utf8')) {
    hash ^= byte;
    hash = Math.imul(hash, 0x01000193) >>> 0;
  }
  return hash;
}

/** mulberry32: a small 32-bit PRNG, identical in both suites. */
export function createRandom(seed) {
  let state = typeof seed === 'number' ? seed >>> 0 : seedNumber(seed);
  const next = () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let value = state;
    value = Math.imul(value ^ (value >>> 15), value | 1);
    value ^= value + Math.imul(value ^ (value >>> 7), value | 61);
    return (value ^ (value >>> 14)) >>> 0;
  };
  const int = (bound) => (bound <= 0 ? 0 : next() % bound);
  return { next, int, pick: (items) => items[int(items.length)] };
}

/** Inserted text that stresses spans: astral, combining, joined, invisible and line-ending characters. */
export const UNICODE_POOL = [
  '𝒳', '👩\u200D👩\u200D👧', 'e\u0301', '日本', '🦀', 'λ', '\u00A0', '\u200B', '\u2060', '\uFEFF', '\r\n', '\r', '\t', '\n', ' ',
];

/** Tokens of each language that open, close or split constructs. */
export const TOKEN_POOLS = {
  JavaScript: ['(', ')', '{', '}', '[', ']', ';', ',', '=>', '"', '`', '${', '/*', '//', 'function', 'let', '=', '.', 'class', '<div>'],
  Lean: ['(', ')', '⟨', '⟩', ':=', ':', 'def', 'theorem', 'by', 'fun', '=>', '|', '--', '/-', '-/', 'do', '←', '"', '.', 'where'],
  Rocq: ['(', ')', '.', ':=', ':', 'Definition', 'Lemma', 'Proof.', 'Qed.', '(*', '*)', 'fun', '=>', '|', 'match', 'end', '"', '%', ';', 'forall'],
  Rust: ['(', ')', '{', '}', '[', ']', ';', ',', '::', '->', '=>', '"', "'a", 'fn', 'let', '#[', '/*', '//', 'r#"', '<', '>'],
};

const codePoints = (text) => [...text];
const utf8Length = (text) => Buffer.byteLength(text, 'utf8');
const byteOffset = (points, index) => utf8Length(points.slice(0, index).join(''));

/** One random edit of `source` as UTF-8 byte offsets on code point boundaries: `{ start, end, replacement }`. */
export function randomEdit(random, source, language) {
  const points = codePoints(source);
  const pool = random.int(3) === 0 ? UNICODE_POOL : TOKEN_POOLS[language];
  const at = random.int(points.length + 1);
  const length = Math.min(points.length - at, 1 + random.int(8));
  let edit;
  switch (random.int(5)) {
    case 0: // delete
      edit = { from: at, to: at + length, replacement: '' };
      break;
    case 1: // duplicate
      edit = { from: at, to: at, replacement: points.slice(at, at + length).join('') };
      break;
    case 2: // replace
      edit = { from: at, to: at + length, replacement: random.pick(pool) };
      break;
    case 3: // truncate
      edit = { from: at, to: points.length, replacement: '' };
      break;
    default: // insert
      edit = { from: at, to: at, replacement: random.pick(pool) };
  }
  return { start: byteOffset(points, edit.from), end: byteOffset(points, edit.to), replacement: edit.replacement };
}

/** Applies a byte-offset edit to a string. */
export function applyEdit(source, { start, end, replacement }) {
  const bytes = Buffer.from(source, 'utf8');
  return Buffer.concat([bytes.subarray(0, start), Buffer.from(replacement, 'utf8'), bytes.subarray(end)]).toString('utf8');
}

export const DEFAULT_SEED = 'issue-195-generative-v1';
/** Cases per language and family. */
export const COUNTS = { property: 16, fuzz: 32, metamorphic: 8, edit: 8, editSteps: 4 };
const MAX_SEED_BYTES = 600;

/** Seed sources of one language: its small conformance inputs, in a stable order. */
export function seedSources(language) {
  const details = JSON.parse(readConformance('manifest.json')).languages[language];
  const oracle = JSON.parse(readConformance(details.oracle));
  const sources = [];
  for (const file of Object.keys(details.corpus.files)) {
    const text = readConformance(`${details.corpus.directory}/${file}`);
    parseCorpus(text).forEach((test, index) => sources.push({ from: `corpus/${file}/${index}`, source: test.source }));
  }
  for (const entry of oracle.cases.filter((item) => item.source !== undefined)) {
    sources.push({ from: entry.id, source: entry.source });
  }
  return sources.filter(({ source }) => source.trim() && Buffer.byteLength(source, 'utf8') <= MAX_SEED_BYTES);
}

/** The generated inputs of one language, before the oracle parses them. */
export function generateInputs(language, seeds, seed) {
  const random = createRandom(`${seed}:${language}`);
  const pick = () => random.pick(seeds);
  const cases = [];
  for (let index = 0; index < COUNTS.property; index += 1) {
    // Compositions: two or three seed programs joined by a separator from the Unicode or token pool.
    const parts = Array.from({ length: 2 + random.int(2) }, pick);
    const separators = ['\n', '\n\n', '\r\n', ' ', '\u00A0\n', '\n\u200B', '\t\n'];
    const source = parts.map((part) => part.source).reduce((text, part) => `${text}${random.pick(separators)}${part}`);
    cases.push({ id: `property/${index}`, kind: 'property', from: parts.map((part) => part.from), source });
  }
  for (let index = 0; index < COUNTS.fuzz; index += 1) {
    const base = pick();
    let source = base.source;
    const edits = [];
    for (let step = 0, total = 1 + random.int(4); step < total; step += 1) {
      const edit = randomEdit(random, source, language);
      edits.push(edit);
      source = applyEdit(source, edit);
    }
    cases.push({ id: `fuzz/${index}`, kind: 'fuzz', from: [base.from], edits, source });
  }
  for (let index = 0; index < COUNTS.metamorphic; index += 1) {
    const base = pick();
    for (const relation of Object.keys(RELATIONS)) {
      cases.push({
        id: `metamorphic/${index}/${relation}`,
        kind: 'metamorphic',
        relation,
        from: [base.from],
        source: base.source,
        variant: RELATIONS[relation].transform(base.source),
      });
    }
  }
  for (let index = 0; index < COUNTS.edit; index += 1) {
    const base = pick();
    let source = base.source;
    const steps = [];
    for (let step = 0; step < COUNTS.editSteps; step += 1) {
      const edit = randomEdit(random, source, language);
      source = applyEdit(source, edit);
      steps.push({ ...edit, source });
    }
    cases.push({ id: `edit/${index}`, kind: 'edit', from: [base.from], source: base.source, steps });
  }
  return cases;
}

/**
 * The metamorphic relations: each maps a source to a variant whose tree the relation
 * predicts from the tree of the source.
 */
export const RELATIONS = {
  // Two leading blank lines move every node down two rows and change nothing else, for a clean
  // tree (they can change error recovery) that does not start with a byte-order mark (the
  // lexer skips U+FEFF only at byte 0).
  'prepend-blank-lines': {
    applies: (source, clean) => clean && !source.startsWith('\uFEFF'),
    transform: (source) => `\n\n${source}`,
    project: (cst) => cst,
    expect: (cst) => parseCstLines(cst).map((node) => ({ ...node, start: { ...node.start, row: node.start.row + 2 }, end: { ...node.end, row: node.end.row + 2 } })),
  },
  // CRLF line ends keep every kind, field, nesting and start point; ends may take the `\r`.
  crlf: {
    applies: () => true,
    transform: (source) => source.replace(/\r?\n/gu, '\r\n'),
    project: (cst) => parseCstLines(cst).map(({ end, ...node }) => node),
    expect: (cst) => parseCstLines(cst).map(({ end, ...node }) => node),
  },
};

/** Whether the variant tree is what the relation predicts from the base tree. */
export function relationHolds(relation, baseCst, variantCst) {
  const { project, expect } = RELATIONS[relation];
  const predicted = expect(baseCst);
  const actual = relation === 'crlf' ? project(variantCst) : parseCstLines(variantCst);
  return JSON.stringify(actual) === JSON.stringify(predicted);
}

/**
 * Oracle-free problems of a parsed network: exact reconstruction; every span inside the
 * source, on code point boundaries, with points that name its byte offsets; every Syntax
 * child inside its parent; and a verification report that is clean exactly when no link
 * carries an error, missing or has-error flag. A node may have an error with no flagged
 * link inside it: tree-sitter hides a MISSING leaf of a hidden kind, as the native
 * projection does, but the node it is missing from still has an error.
 */
export function propertyProblems(network, source) {
  const problems = [];
  if (network.reconstructText() !== source) problems.push('reconstruction differs from the source');
  const bytes = Buffer.from(source, 'utf8');
  const rows = [0];
  bytes.forEach((byte, index) => { if (byte === 0x0a) rows.push(index + 1); });
  const pointOf = (offset) => {
    let row = rows.length - 1;
    while (rows[row] > offset) row -= 1;
    return { row, column: offset - rows[row] };
  };
  const boundary = (offset) => offset === bytes.length || (offset < bytes.length && (bytes[offset] & 0xc0) !== 0x80);
  const links = [...network.links()];
  const byId = new Map(links.map((link) => [link.id().value, link]));
  let flagged = false;
  for (const link of links) {
    const { span, flags, linkType } = link.metadata();
    if (flags?.isError || flags?.isMissing || flags?.hasError) flagged = true;
    if (!span) continue;
    const { start, end } = span.byteRange;
    const where = `${linkType} ${link.metadata().term ?? ''} ${start}..${end}`;
    if (!(start <= end && end <= bytes.length)) {
      problems.push(`${where} is outside the source`);
      continue;
    }
    if (!boundary(start) || !boundary(end)) problems.push(`${where} splits a UTF-8 code point`);
    const [startPoint, endPoint] = [pointOf(start), pointOf(end)];
    if (span.start.row !== startPoint.row || span.start.column !== startPoint.column ||
      span.end.row !== endPoint.row || span.end.column !== endPoint.column) {
      problems.push(`${where} has points ${span.start.row}:${span.start.column}-${span.end.row}:${span.end.column}`);
    }
    if (linkType === LinkType.Syntax) {
      for (const reference of link.references()) {
        const child = byId.get(reference.value)?.metadata();
        if (child?.linkType === LinkType.Syntax && child.span &&
          (child.span.byteRange.start < start || child.span.byteRange.end > end)) {
          problems.push(`${where} does not contain its child ${child.term} ${child.span.byteRange.start}..${child.span.byteRange.end}`);
        }
      }
    }
    if (problems.length > 8) break;
  }
  if (network.verifyFullMatch().isClean() === flagged) {
    problems.push(`verifyFullMatch().isClean() is ${!flagged} while ${flagged ? 'a' : 'no'} link is flagged`);
  }
  return problems;
}
