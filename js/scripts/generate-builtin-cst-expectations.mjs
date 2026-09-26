#!/usr/bin/env node
// Generates parity/fixtures/builtin-cst-expected.json: for every language in
// parity/language-grammar-inventory.json that parses with a built-in grammar
// (`builtinGrammar`), the concrete syntax tree rows of its positive and
// recovery sources, derived without the meta-language parsers:
//
// - plain text and natural languages: from the ICU word and sentence
//   segmentation of Intl.Segmenter, grouped by the grammar's lexical classes
//   (parity/grammars/plain-text.ebnf, parity/grammars/natural-language.ebnf);
// - LiNo and PDF: hand-written trees for the grammars in
//   parity/grammars/links-notation-0.13.0.pegjs and parity/grammars/pdf-cos.ebnf,
//   whose tokens must tile the source exactly and whose structure must project
//   to what the official links-notation parser and pdf-lib read from the
//   positive source, while both oracles reject the recovery source.
//
//   node js/scripts/generate-builtin-cst-expectations.mjs          # write
//   node js/scripts/generate-builtin-cst-expectations.mjs --check  # verify
//
// Rows are those of parity/fixtures/default-cst-expected.json:
// [depth, field, kind, named, startByte, endByte, flags] in depth-first order
// with UTF-8 byte offsets and flags drawn from E (error), M (missing) and
// X (extra). Built-in grammars own their whitespace as extra nodes.
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { parse as parseLinks } from 'links-notation/src/parser-generated.js';
import { PDFDict, PDFName, PDFParser, PDFRef } from 'pdf-lib';

const require = createRequire(import.meta.url);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = join(root, 'parity/language-grammar-inventory.json');
const expectedPath = join(root, 'parity/fixtures/builtin-cst-expected.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const byteLength = (text) => Buffer.byteLength(text, 'utf8');

// ---- plain text and natural languages: ICU segmentation -----------------

// The control characters the lexical grammar rejects: Cc minus White_Space.
const isControl = (text) => [...text].every((character) => /\p{Cc}/u.test(character) && !/\s/u.test(character));
const isWhitespace = (text) => /^\p{White_Space}+$/u.test(text);

// The lexical class of an ICU word segment; adjacent segments of one class
// form one node, as the grammar matches maximal runs.
function lexicalClass({ segment, isWordLike }) {
  if (isWhitespace(segment)) return 'whitespace';
  if (isControl(segment)) return 'ERROR';
  if (isWordLike) return 'word';
  return 'punctuation';
}

// Rows of the lexical nodes of text[start..end] (UTF-16), which begins at
// byte `offset`.
function lexicalRows(text, start, end, depth, offset) {
  const rows = [];
  for (const segment of new Intl.Segmenter('und', { granularity: 'word' }).segment(text.slice(start, end))) {
    const kind = lexicalClass(segment);
    const bytes = byteLength(segment.segment);
    const last = rows.at(-1);
    if (last?.[2] === kind) {
      last[5] += bytes;
    } else {
      const flags = kind === 'whitespace' ? 'X' : kind === 'ERROR' ? 'E' : '';
      rows.push([depth, null, kind, kind === 'whitespace' ? 0 : 1, offset, offset + bytes, flags]);
    }
    offset += bytes;
  }
  return rows;
}

/** UTF-16 [start, end) ranges of the lines of `text`, each ending after its LF. */
function lineRanges(text) {
  const ranges = [];
  let start = 0;
  for (let index = text.indexOf('\n'); index !== -1; index = text.indexOf('\n', start)) {
    ranges.push([start, index + 1]);
    start = index + 1;
  }
  if (start < text.length || text === '') ranges.push([start, text.length]);
  return ranges;
}

function balancedParentheses(text) {
  let depth = 0;
  for (const character of text) {
    if (character === '(') depth += 1;
    if (character === ')' && (depth -= 1) < 0) return false;
  }
  return depth === 0;
}

// Rows of a document of `unit` nodes over UTF-16 `ranges`, with byte offsets.
function segmentedRows(text, documentKind, unit, ranges, documentFlags = '') {
  const rows = [[0, null, documentKind, 1, 0, byteLength(text), documentFlags]];
  for (const [start, end] of ranges) {
    const startByte = byteLength(text.slice(0, start));
    rows.push([1, null, unit, 1, startByte, startByte + byteLength(text.slice(start, end)), '']);
    rows.push(...lexicalRows(text, start, end, 2, startByte));
  }
  return rows;
}

function plainTextRows(text) {
  return segmentedRows(text, 'text_document', 'line', lineRanges(text), balancedParentheses(text) ? '' : 'E');
}

function sentenceRanges(text) {
  const ranges = [];
  for (const { index, segment } of new Intl.Segmenter('und', { granularity: 'sentence' }).segment(text)) {
    ranges.push([index, index + segment.length]);
  }
  return ranges;
}

function naturalLanguageRows(text) {
  return segmentedRows(text, 'natural_language_document', 'sentence', sentenceRanges(text));
}

// ---- LiNo and PDF: hand-written trees checked against the oracles ---------

// A tree is [kind, children] for a named node, or a token: 'text' (an
// anonymous token whose kind is its text), ['kind', 'text'] (a named token),
// ['field:kind', ...] (a node in a field), {ws: 'text'} (a whitespace extra),
// {comment: 'text'} (a named comment extra), {error: [children]} (an ERROR
// node) and {missing: 'kind'} / {missingNamed: 'kind'} (a missing node).
// Positions come from tiling the source with the tokens in order.
const ws = (text) => ({ ws: text });

const LINO_TREES = {
  'papa (lovesMama: loves mama)\n  child: "quoted value"\n(1 2)\n': ['lino_document', [
    ['link', [
      ['value:reference', 'papa'], ws(' '),
      ['value:link', ['(', ['id:reference', 'lovesMama'], ':', ws(' '), ['value:reference', 'loves'], ws(' '),
        ['value:reference', 'mama'], ')']],
      ws('\n  '),
      ['child:link', [['id:reference', 'child'], ':', ws(' '), ['value:quoted_reference', '"quoted value"']]],
    ]],
    ws('\n'),
    ['link', ['(', ['value:reference', '1'], ws(' '), ['value:reference', '2'], ')']],
    ws('\n'),
  ]],
  '(unclosed\n': ['lino_document', [{ error: ['(', ['reference', 'unclosed']] }, ws('\n')]],
};

const indirectReference = (number, generation) => [
  ['object_number:integer', number], ws(' '), ['generation:integer', generation], ws(' '), 'R',
];

const PDF_TREES = {
  '%PDF-1.7\n% note\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\ntrailer\n<< /Root 1 0 R >>\n%%EOF\n': ['pdf_file', [
    ['header', '%PDF-1.7'], ws('\n'), { comment: '% note' }, ws('\n'),
    ['indirect_object', [
      ['object_number:integer', '1'], ws(' '), ['generation:integer', '0'], ws(' '), 'obj', ws('\n'),
      ['value:dictionary', ['<<', ws(' '),
        ['dictionary_entry', [['key:name', '/Type'], ws(' '), ['value:name', '/Catalog']]], ws(' '),
        ['dictionary_entry', [['key:name', '/Pages'], ws(' '), ['value:indirect_reference', indirectReference('2', '0')]]],
        ws(' '), '>>']],
      ws('\n'), 'endobj',
    ]],
    ws('\n'),
    ['trailer', ['trailer', ws('\n'),
      ['dictionary:dictionary', ['<<', ws(' '),
        ['dictionary_entry', [['key:name', '/Root'], ws(' '), ['value:indirect_reference', indirectReference('1', '0')]]],
        ws(' '), '>>']]]],
    ws('\n'), ['end_of_file', '%%EOF'], ws('\n'),
  ]],
  '%PDF-1.7\n1 0 obj\n<< /Type /Catalog\nendobj\n': ['pdf_file', [
    ['header', '%PDF-1.7'], ws('\n'),
    ['indirect_object', [
      ['object_number:integer', '1'], ws(' '), ['generation:integer', '0'], ws(' '), 'obj', ws('\n'),
      ['value:dictionary', ['<<', ws(' '),
        ['dictionary_entry', [['key:name', '/Type'], ws(' '), ['value:name', '/Catalog']]], { missing: '>>' }]],
      ws('\n'), 'endobj',
    ]],
    { missingNamed: 'end_of_file' }, ws('\n'),
  ]],
};

/** Rows of a hand-written tree, tiling `text` from its start. */
function treeRows(text, tree) {
  const bytes = Buffer.from(text, 'utf8');
  let offset = 0;
  const rows = [];
  const take = (token) => {
    const length = byteLength(token);
    const actual = bytes.subarray(offset, offset + length).toString('utf8');
    if (actual !== token) throw new Error(`expected ${JSON.stringify(token)} at byte ${offset}, found ${JSON.stringify(actual)}`);
    offset += length;
    return [offset - length, offset];
  };
  const walk = (node, depth) => {
    if (typeof node === 'string') {
      rows.push([depth, null, node, 0, ...take(node), '']);
    } else if (node.ws !== undefined) {
      rows.push([depth, null, 'whitespace', 0, ...take(node.ws), 'X']);
    } else if (node.comment !== undefined) {
      rows.push([depth, null, 'comment', 1, ...take(node.comment), 'X']);
    } else if (node.missing !== undefined || node.missingNamed !== undefined) {
      rows.push([depth, null, node.missing ?? node.missingNamed, node.missing === undefined ? 1 : 0, offset, offset, 'M']);
    } else {
      const [label, content] = node.error ? ['ERROR', node.error] : node;
      const [field, kind] = label.includes(':') ? label.split(':') : [null, label];
      const row = [depth, field, kind, 1, offset, offset, node.error ? 'E' : ''];
      rows.push(row);
      if (typeof content === 'string') [, row[5]] = take(content);
      else {
        for (const child of content) walk(child, depth + 1);
        row[5] = offset;
      }
    }
  };
  walk(tree, 0);
  // The root spans the whole source, so the tokens must tile it.
  if (offset !== bytes.length) throw new Error(`tree covers ${offset} of ${bytes.length} bytes`);
  return rows;
}

// Row trees for the oracle projections: children of each row by index.
function rowTree(rows, text) {
  const bytes = Buffer.from(text, 'utf8');
  const nodes = rows.map(([depth, field, kind, , start, end, flags]) => ({
    depth, field, kind, flags, text: bytes.subarray(start, end).toString('utf8'), children: [],
  }));
  const stack = [];
  for (const node of nodes) {
    stack.length = node.depth;
    stack.at(-1)?.children.push(node);
    stack.push(node);
  }
  return nodes[0];
}

const structural = (node) => !node.flags.includes('X') && node.children !== undefined;
const fieldChildren = (node, field) => node.children.filter((child) => child.field === field);

// A LiNo link as the official parser returns it: {id, values, children}.
function linoLink(node) {
  if (node.kind === 'reference') return { id: node.text, values: [], children: [] };
  if (node.kind === 'quoted_reference') return { id: node.text.slice(1, -1), values: [], children: [] };
  if (node.kind !== 'link') throw new Error(`no LiNo projection for ${node.kind}`);
  const [id] = fieldChildren(node, 'id');
  return {
    id: id ? linoLink(id).id : null,
    values: fieldChildren(node, 'value').map(linoLink),
    children: fieldChildren(node, 'child').map(linoLink),
  };
}

const normalizeLink = (link) => ({
  id: link.id ?? null,
  values: (link.values ?? []).map(normalizeLink),
  children: (link.children ?? []).map(normalizeLink),
});

function checkLino(source, rows, positive) {
  let official = null;
  try {
    official = parseLinks(source).map(normalizeLink);
  } catch {
    official = null;
  }
  const hasError = rows.some((row) => row[6].includes('E') || row[6].includes('M'));
  if (!positive) {
    if (official !== null || !hasError) throw new Error(`LiNo recovery ${JSON.stringify(source)} must be rejected by links-notation and have error rows`);
    return;
  }
  const projected = rowTree(rows, source).children.filter((child) => structural(child) && child.kind === 'link').map(linoLink);
  if (hasError || JSON.stringify(projected) !== JSON.stringify(official)) {
    throw new Error(`LiNo ${JSON.stringify(source)}: rows project to ${JSON.stringify(projected)}, links-notation reads ${JSON.stringify(official)}`);
  }
}

// A PDF value as the pdf-grammar-cases projection writes it.
function pdfValue(node) {
  if (node.kind === 'name') return { name: node.text.slice(1) };
  if (node.kind === 'integer') return { number: Number(node.text) };
  if (node.kind === 'indirect_reference') {
    return { ref: [Number(fieldChildren(node, 'object_number')[0].text), Number(fieldChildren(node, 'generation')[0].text)] };
  }
  if (node.kind === 'dictionary') {
    return {
      dictionary: node.children.filter((child) => child.kind === 'dictionary_entry').map((entry) => [
        pdfValue(fieldChildren(entry, 'key')[0]).name,
        pdfValue(fieldChildren(entry, 'value')[0]),
      ]),
    };
  }
  throw new Error(`no PDF projection for ${node.kind}`);
}

function pdfOracleValue(object) {
  if (object instanceof PDFName) return { name: object.decodeText() };
  if (object instanceof PDFRef) return { ref: [object.objectNumber, object.generationNumber] };
  if (object instanceof PDFDict) {
    return { dictionary: object.entries().map(([key, value]) => [key.decodeText(), pdfOracleValue(value)]) };
  }
  if (typeof object?.asNumber === 'function') return { number: object.asNumber() };
  throw new Error(`no projection for pdf-lib ${object?.constructor?.name}`);
}

async function checkPdf(source, rows, positive) {
  const warn = console.warn;
  console.warn = () => {};
  let official = null;
  try {
    const context = await PDFParser.forBytesWithOptions(Buffer.from(source, 'utf8'), Infinity, true).parseDocument();
    official = context.enumerateIndirectObjects().map(([ref, object]) => ({
      ref: [ref.objectNumber, ref.generationNumber],
      value: pdfOracleValue(object),
    }));
  } catch {
    official = null;
  } finally {
    console.warn = warn;
  }
  const hasError = rows.some((row) => row[6].includes('E') || row[6].includes('M'));
  if (!positive) {
    if (official !== null || !hasError) throw new Error(`PDF recovery ${JSON.stringify(source)} must be rejected by pdf-lib and have error rows`);
    return;
  }
  const projected = rowTree(rows, source).children.filter((child) => child.kind === 'indirect_object').map((object) => ({
    ref: [Number(fieldChildren(object, 'object_number')[0].text), Number(fieldChildren(object, 'generation')[0].text)],
    value: pdfValue(fieldChildren(object, 'value')[0]),
  }));
  if (hasError || JSON.stringify(projected) !== JSON.stringify(official)) {
    throw new Error(`PDF ${JSON.stringify(source)}: rows project to ${JSON.stringify(projected)}, pdf-lib reads ${JSON.stringify(official)}`);
  }
}

async function builtinRows(language, source, positive) {
  if (language.builtinGrammar === 'plain-text') return plainTextRows(source);
  if (language.builtinGrammar === 'natural-language') return naturalLanguageRows(source);
  const [trees, check] = language.builtinGrammar === 'links-notation' ? [LINO_TREES, checkLino] : [PDF_TREES, checkPdf];
  if (!trees[source]) throw new Error(`no hand-written ${language.name} tree for ${JSON.stringify(source)}`);
  const rows = treeRows(source, trees[source]);
  await check(source, rows, positive);
  return rows;
}

// ---- output ------------------------------------------------------------------

function formatExpected(value) {
  const render = (item, indent) => {
    if (Array.isArray(item) && item.length && Array.isArray(item[0])) {
      return `[\n${item.map((row) => `${indent}  ${JSON.stringify(row)}`).join(',\n')}\n${indent}]`;
    }
    if (Array.isArray(item)) return JSON.stringify(item);
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
  const result = {};
  for (const language of inventory.languages) {
    if (!language.builtinGrammar) continue;
    const grammar = inventory.builtinGrammars[language.builtinGrammar];
    result[language.name] = {
      grammars: {
        [language.builtinGrammar]: {
          version: grammar.version,
          parserSha256: sha256(await readFile(join(root, grammar.specification))),
        },
      },
      sourceSha256: sha256(language.source),
      recoverySourceSha256: sha256(language.recoverySource),
      positive: await builtinRows(language, language.source, true),
      recovery: await builtinRows(language, language.recoverySource, false),
      embedded: [],
    };
  }
  return {
    description:
      'Built-in grammar concrete syntax trees for the inventory sources, derived without the meta-language parsers: plain text and natural languages from ICU word and sentence segmentation (Intl.Segmenter) grouped by the lexical classes of the grammar, LiNo and PDF from hand-written trees that tile the source and project to what links-notation and pdf-lib read (both reject the recovery sources). Row: [depth, field, kind, named, startByte, endByte, flags]; flags: E error, M missing, X extra.',
    generator: {
      script: 'js/scripts/generate-builtin-cst-expectations.mjs',
      linksNotation: require('links-notation/package.json').version,
      pdfLib: require('pdf-lib/package.json').version,
    },
    languages: result,
  };
}

const expected = formatExpected(await generate());
if (process.argv.includes('--check')) {
  if ((await readFile(expectedPath, 'utf8').catch(() => '')) !== expected) {
    console.error('parity/fixtures/builtin-cst-expected.json is stale; run node js/scripts/generate-builtin-cst-expectations.mjs');
    process.exit(1);
  }
  console.log('builtin CST expectations match');
} else {
  await writeFile(expectedPath, expected);
  console.log('wrote parity/fixtures/builtin-cst-expected.json');
}
